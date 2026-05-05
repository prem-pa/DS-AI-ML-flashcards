use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use super::parse::{split_frontmatter, ConceptNote};
use super::scan::scan;
use super::SyncReport;

fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[derive(Debug)]
struct ExistingConcept {
    rel_path: String,
    content_hash: String,
}

pub fn sync_vault(conn: &mut Connection, vault_root: &Path) -> Result<SyncReport> {
    let scanned = scan(vault_root)?;

    let now = now_ts();
    let tx = conn.transaction()?;

    // Load existing concepts (not soft-deleted).
    let mut by_id: HashMap<String, ExistingConcept> = HashMap::new();
    let mut by_path: HashMap<String, String> = HashMap::new(); // rel_path -> id
    {
        let mut stmt = tx.prepare(
            "SELECT id, path, content_hash FROM concepts WHERE deleted_at IS NULL",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        for row in rows {
            let (id, path, hash) = row?;
            by_path.insert(path.clone(), id.clone());
            by_id.insert(
                id,
                ExistingConcept {
                    rel_path: path,
                    content_hash: hash,
                },
            );
        }
    }

    let mut seen_ids: HashSet<String> = HashSet::new();
    let mut new_concepts = 0usize;
    let mut changed = 0usize;
    let mut suspended_cards = 0usize;
    // slug -> concept_id, for wikilink resolution after the upsert pass.
    let mut slug_to_id: HashMap<String, String> = HashMap::new();
    // (path, [(card_uuid, parent_id)]) for concept_links pass. We collect
    // (concept_id, [wikilink_slugs]) so we can resolve once everything is upserted.
    let mut links_pending: Vec<(String, Vec<String>)> = Vec::new();

    for note in &scanned {
        let parsed = match &note.parsed {
            Ok(p) => p,
            Err(e) => {
                eprintln!("sync: skipping {} ({})", note.rel_path.display(), e);
                continue;
            }
        };
        let rel_path_str = note.rel_path.to_string_lossy().to_string();

        // Resolve concept identity:
        //  1. id in frontmatter wins (rename-safe)
        //  2. else previously-seen rel_path
        //  3. else mint a fresh one and stamp it back to disk
        let (concept_id, mint_new_id) = if let Some(id) = parsed.frontmatter.id.as_deref() {
            (id.to_string(), false)
        } else if let Some(id) = by_path.get(&rel_path_str) {
            (id.clone(), false)
        } else {
            (Uuid::new_v4().to_string(), true)
        };

        if mint_new_id {
            // Stamp the new id into the file's frontmatter so future syncs are stable.
            stamp_concept_id(&note.path, &concept_id)
                .with_context(|| format!("stamping id into {}", note.path.display()))?;
            new_concepts += 1;
        }

        seen_ids.insert(concept_id.clone());
        slug_to_id.insert(slug_for_concept(parsed), concept_id.clone());

        let was_existing = by_id.contains_key(&concept_id);
        let unchanged = by_id
            .get(&concept_id)
            .map(|e| e.content_hash == note.content_hash && e.rel_path == rel_path_str)
            .unwrap_or(false);

        if !unchanged {
            upsert_concept(&tx, &concept_id, &rel_path_str, &note.content_hash, parsed, now)?;
            suspended_cards += upsert_children(&tx, &concept_id, parsed, now)?;
            if was_existing {
                changed += 1;
            } else if !mint_new_id {
                // First time we've seen this id but it's already in frontmatter — count as new too.
                new_concepts += 1;
            }
        }

        // Always re-record wikilinks for this concept; cheap and keeps graph fresh.
        links_pending.push((concept_id.clone(), parsed.wikilinks.clone()));
    }

    // Soft-delete concepts whose files vanished from disk; suspend their cards.
    let to_remove: Vec<String> = by_id
        .keys()
        .filter(|k| !seen_ids.contains(*k))
        .cloned()
        .collect();
    for id in &to_remove {
        tx.execute(
            "UPDATE concepts SET deleted_at = ?1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        let n = tx.execute(
            "UPDATE cards SET suspended = 1, updated_at = ?1 WHERE concept_id = ?2 AND suspended = 0",
            params![now, id],
        )?;
        suspended_cards += n;
    }

    // Auto-suspend cards we know are unreviewable, regardless of content_hash:
    //  - empty front: no question at all (143 cards from early agent waves)
    //  - flip + empty back: space-reveal shows nothing (66 cards)
    //  - mcq + no choice marked correct: nothing to auto-grade against (64 cards)
    // Idempotent across syncs; never un-suspends.
    let bad_suspended = tx.execute(
        "UPDATE cards SET suspended = 1, updated_at = ?1
         WHERE suspended = 0
           AND (
                TRIM(front) = ''
             OR (type = 'flip' AND TRIM(back) = '')
             OR (type = 'mcq'
                 AND (choices_json IS NULL
                      OR choices_json NOT LIKE '%\"correct\":true%'))
           )",
        params![now],
    )?;
    suspended_cards += bad_suspended;

    // Resolve wikilinks: write concept_links rows (idempotent: clear + reinsert per concept).
    {
        let mut clear = tx.prepare("DELETE FROM concept_links WHERE src_id = ?1")?;
        let mut ins = tx.prepare(
            "INSERT OR IGNORE INTO concept_links(src_id, dst_id) VALUES (?1, ?2)",
        )?;
        for (src_id, slugs) in &links_pending {
            clear.execute(params![src_id])?;
            for slug in slugs {
                if let Some(dst_id) = slug_to_id.get(slug) {
                    if dst_id != src_id {
                        ins.execute(params![src_id, dst_id])?;
                    }
                }
            }
        }
    }

    tx.commit()?;

    Ok(SyncReport {
        changed,
        new_concepts,
        suspended_cards,
    })
}

fn slug_for_concept(c: &ConceptNote) -> String {
    // Prefer slug from path/title; we don't store the slug in frontmatter, so derive from title.
    super::parse::slugify(&c.frontmatter.title)
}

fn upsert_concept(
    tx: &rusqlite::Transaction,
    id: &str,
    rel_path: &str,
    content_hash: &str,
    note: &ConceptNote,
    now: i64,
) -> Result<()> {
    let fm = &note.frontmatter;
    let difficulty = fm.difficulty.clamp(1, 5);

    tx.execute(
        "INSERT INTO concepts(id, slug, title, track, topic, difficulty, path, content_hash, created_at, updated_at, deleted_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9, NULL)
         ON CONFLICT(id) DO UPDATE SET
           slug=excluded.slug,
           title=excluded.title,
           track=excluded.track,
           topic=excluded.topic,
           difficulty=excluded.difficulty,
           path=excluded.path,
           content_hash=excluded.content_hash,
           updated_at=excluded.updated_at,
           deleted_at=NULL",
        params![
            id,
            slug_for_concept(note),
            fm.title,
            fm.track,
            fm.topic,
            difficulty,
            rel_path,
            content_hash,
            now,
        ],
    )?;

    // Replace tags / aliases / sources (small, idempotent).
    tx.execute("DELETE FROM concept_tags WHERE concept_id = ?1", params![id])?;
    {
        let mut s = tx.prepare("INSERT OR IGNORE INTO concept_tags(concept_id, tag) VALUES (?1, ?2)")?;
        for tag in &fm.tags {
            s.execute(params![id, tag])?;
        }
    }
    tx.execute("DELETE FROM concept_aliases WHERE concept_id = ?1", params![id])?;
    {
        let mut s = tx.prepare("INSERT OR IGNORE INTO concept_aliases(concept_id, alias) VALUES (?1, ?2)")?;
        for a in &fm.aliases {
            s.execute(params![id, a])?;
        }
    }
    tx.execute("DELETE FROM concept_sources WHERE concept_id = ?1", params![id])?;
    {
        let mut s = tx.prepare(
            "INSERT OR IGNORE INTO concept_sources(concept_id, url, label) VALUES (?1, ?2, ?3)",
        )?;
        for src in &fm.sources {
            s.execute(params![id, src.url, src.label])?;
        }
    }
    Ok(())
}

fn upsert_children(
    tx: &rusqlite::Transaction,
    concept_id: &str,
    note: &ConceptNote,
    now: i64,
) -> Result<usize> {
    // Cards: UPSERT by uuid. Cards present in DB but not in this file get suspended (preserves
    // review history via the append-only `reviews` table).
    let mut existing_card_ids: HashSet<String> = HashSet::new();
    {
        let mut stmt = tx.prepare("SELECT id FROM cards WHERE concept_id = ?1")?;
        let rows = stmt.query_map(params![concept_id], |row| row.get::<_, String>(0))?;
        for r in rows {
            existing_card_ids.insert(r?);
        }
    }
    let mut seen_card_ids: HashSet<String> = HashSet::new();

    for (idx, card) in note.frontmatter.cards.iter().enumerate() {
        // Cards lacking an id were stamped at parse time only if explicitly chosen.
        // Here we synthesize a stable uuid if missing and stamp back to disk.
        let card_id = card
            .id
            .clone()
            .filter(|s| Uuid::parse_str(s).is_ok())
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        seen_card_ids.insert(card_id.clone());

        let choices_json = match (card.kind.as_str(), card.choices.as_ref()) {
            ("mcq", Some(ch)) => Some(serde_json::to_string(ch)?),
            _ => None,
        };

        tx.execute(
            "INSERT INTO cards(id, concept_id, type, front, back, choices_json, position, suspended, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8, ?8)
             ON CONFLICT(id) DO UPDATE SET
               concept_id=excluded.concept_id,
               type=excluded.type,
               front=excluded.front,
               back=excluded.back,
               choices_json=excluded.choices_json,
               position=excluded.position,
               suspended=0,
               updated_at=excluded.updated_at",
            params![
                card_id,
                concept_id,
                card.kind,
                card.front,
                card.back,
                choices_json,
                idx as i64,
                now,
            ],
        )?;

        // Initial scheduler row, only inserted on first encounter — never overwritten.
        tx.execute(
            "INSERT INTO card_state(card_id, due, stability, difficulty, elapsed_days,
                                    scheduled_days, reps, lapses, state, last_review)
             VALUES (?1, ?2, 0.0, 0.0, 0.0, 0.0, 0, 0, 0, NULL)
             ON CONFLICT(card_id) DO NOTHING",
            params![card_id, now],
        )?;
    }

    // Suspend cards that vanished from this concept's frontmatter.
    let mut suspend = tx.prepare(
        "UPDATE cards SET suspended = 1, updated_at = ?1 WHERE id = ?2 AND suspended = 0",
    )?;
    let mut suspended = 0usize;
    for old in existing_card_ids.difference(&seen_card_ids) {
        suspended += suspend.execute(params![now, old])?;
    }
    Ok(suspended)
}

/// Insert `id: <uuid>` into the YAML frontmatter of a note that lacks one.
/// Idempotent: bails out silently if the frontmatter already has an id field.
fn stamp_concept_id(path: &Path, id: &str) -> Result<()> {
    let raw = std::fs::read_to_string(path)?;
    let (yaml, _body) = match split_frontmatter(&raw) {
        Ok(v) => v,
        Err(_) => return Ok(()), // unparseable; lint will surface it
    };
    if yaml.lines().any(|l| l.trim_start().starts_with("id:")) {
        return Ok(());
    }
    let needle = "---\n";
    let pos = raw.find(needle).map(|p| p + needle.len()).unwrap_or(0);
    let mut out = String::with_capacity(raw.len() + 64);
    out.push_str(&raw[..pos]);
    out.push_str(&format!("id: {id}\n"));
    out.push_str(&raw[pos..]);
    std::fs::write(path, out)?;
    Ok(())
}
