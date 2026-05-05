//! Build a context bundle for the LLM from the existing KB:
//! - the card itself (front/back/choices)
//! - top-N wikilink-resolved neighbor concepts → just the `## Intuition` paragraph
//! - the concept's tags
//!
//! Stays in pure SQL + a tiny markdown extractor; no embeddings, no vector store.

use anyhow::Result;
use rusqlite::{params, Connection};

use crate::db::CardView;

const NEIGHBOR_LIMIT: usize = 5;
const INTUITION_MAX_CHARS: usize = 360;

#[derive(Debug, Clone)]
pub struct Neighbor {
    pub title: String,
    pub intuition: String, // possibly empty
}

#[derive(Debug, Clone)]
pub struct Bundle {
    pub card: CardView,
    pub neighbors: Vec<Neighbor>,
    pub tags: Vec<String>,
}

pub fn build(conn: &Connection, card: &CardView, vault_root: &std::path::Path) -> Result<Bundle> {
    let tags = fetch_tags(conn, &card.concept_id)?;
    let neighbors = fetch_neighbors(conn, &card.concept_id, vault_root, NEIGHBOR_LIMIT)?;
    Ok(Bundle {
        card: card.clone(),
        neighbors,
        tags,
    })
}

fn fetch_tags(conn: &Connection, concept_id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT tag FROM concept_tags WHERE concept_id = ?1 ORDER BY tag",
    )?;
    let rows = stmt.query_map(params![concept_id], |r| r.get::<_, String>(0))?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

fn fetch_neighbors(
    conn: &Connection,
    src_id: &str,
    vault_root: &std::path::Path,
    limit: usize,
) -> Result<Vec<Neighbor>> {
    let mut stmt = conn.prepare(
        "SELECT k.title, k.path FROM concept_links cl
         JOIN concepts k ON k.id = cl.dst_id AND k.deleted_at IS NULL
         WHERE cl.src_id = ?1
         ORDER BY k.title
         LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![src_id, limit as i64], |r| {
        Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
    })?;
    let mut out = Vec::new();
    for r in rows {
        let (title, rel_path) = r?;
        let intuition = extract_intuition(&vault_root.join(&rel_path)).unwrap_or_default();
        out.push(Neighbor { title, intuition });
    }
    Ok(out)
}

/// Pull the prose immediately under `## Intuition` (until the next `## ` header
/// or end of file), trim, and truncate to ~one short paragraph.
fn extract_intuition(path: &std::path::Path) -> Option<String> {
    let raw = std::fs::read_to_string(path).ok()?;
    let lines: Vec<&str> = raw.lines().collect();
    let mut start = None;
    for (i, line) in lines.iter().enumerate() {
        let l = line.trim_start();
        if l.starts_with("## ")
            && l[3..]
                .trim()
                .to_ascii_lowercase()
                .starts_with("intuition")
        {
            start = Some(i + 1);
            break;
        }
    }
    let s = start?;
    let mut buf = String::new();
    for line in &lines[s..] {
        let l = line.trim_start();
        if l.starts_with("## ") {
            break;
        }
        buf.push_str(line);
        buf.push('\n');
    }
    let t = buf.trim();
    if t.is_empty() {
        return None;
    }
    Some(truncate_chars(t, INTUITION_MAX_CHARS))
}

fn truncate_chars(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}
