//! `flashcards lint` — stamp missing UUIDs and flag near-duplicate card fronts.

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use super::parse::{parse, split_frontmatter, Frontmatter};
use super::scan::scan;

#[derive(Debug, Default)]
pub struct LintReport {
    pub concept_stamps: Vec<PathBuf>,
    pub card_stamps: Vec<(PathBuf, usize)>,
    pub dup_groups: Vec<DupGroup>,
}

#[derive(Debug)]
pub struct DupGroup {
    pub similarity: f32,
    pub a: CardLoc,
    pub b: CardLoc,
}

#[derive(Debug, Clone)]
pub struct CardLoc {
    pub path: PathBuf,
    pub card_index: usize,
    pub front: String,
}

pub fn run(vault_root: &Path, dry_run: bool) -> Result<LintReport> {
    let scanned = scan(vault_root)?;
    let mut report = LintReport::default();
    let mut all_cards: Vec<CardLoc> = Vec::new();

    for note in &scanned {
        let parsed = match &note.parsed {
            Ok(p) => p,
            Err(e) => {
                eprintln!("parse err: {}: {}", note.rel_path.display(), e);
                continue;
            }
        };

        if !valid_uuid(parsed.frontmatter.id.as_deref()) {
            report.concept_stamps.push(note.path.clone());
        }
        for (i, card) in parsed.frontmatter.cards.iter().enumerate() {
            all_cards.push(CardLoc {
                path: note.path.clone(),
                card_index: i,
                front: card.front.clone(),
            });
            if !valid_uuid(card.id.as_deref()) {
                report.card_stamps.push((note.path.clone(), i));
            }
        }
    }

    if !dry_run {
        for path in &report.concept_stamps {
            stamp_concept_id(path).with_context(|| {
                format!("stamping concept id into {}", path.display())
            })?;
        }
        // Card stamps require a YAML roundtrip; group by file to avoid
        // rewriting the same file multiple times.
        let mut by_path: std::collections::BTreeMap<PathBuf, Vec<usize>> = Default::default();
        for (p, i) in &report.card_stamps {
            by_path.entry(p.clone()).or_default().push(*i);
        }
        for (path, indices) in &by_path {
            stamp_card_ids(path, indices)
                .with_context(|| format!("stamping card ids into {}", path.display()))?;
        }
    }

    report.dup_groups = find_near_dups(&all_cards, 0.85);
    Ok(report)
}

fn valid_uuid(s: Option<&str>) -> bool {
    s.map(|v| Uuid::parse_str(v).is_ok()).unwrap_or(false)
}

fn stamp_concept_id(path: &Path) -> Result<()> {
    let raw = std::fs::read_to_string(path)?;
    let (yaml, _body) = match split_frontmatter(&raw) {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };
    if yaml.lines().any(|l| l.trim_start().starts_with("id:")) {
        return Ok(());
    }
    let id = Uuid::new_v4().to_string();
    let needle = "---\n";
    let pos = raw.find(needle).map(|p| p + needle.len()).unwrap_or(0);
    let mut out = String::with_capacity(raw.len() + 64);
    out.push_str(&raw[..pos]);
    out.push_str(&format!("id: {id}\n"));
    out.push_str(&raw[pos..]);
    std::fs::write(path, out)?;
    Ok(())
}

fn stamp_card_ids(path: &Path, indices: &[usize]) -> Result<()> {
    let raw = std::fs::read_to_string(path)?;
    let parsed = parse(&raw)?;
    let mut fm: Frontmatter = parsed.frontmatter.clone();
    let body = parsed.body.clone();
    for &i in indices {
        if let Some(card) = fm.cards.get_mut(i) {
            if !valid_uuid(card.id.as_deref()) {
                card.id = Some(Uuid::new_v4().to_string());
            }
        }
    }
    // Re-render via roundtrip. Drops trailing whitespace differences but
    // is fine for machine-generated notes.
    let yaml = serde_yml::to_string(&fm).context("re-serializing frontmatter")?;
    let new_text = format!("---\n{yaml}---\n{body}");
    std::fs::write(path, new_text)?;
    Ok(())
}

fn tokens(s: &str) -> HashSet<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 1)
        .map(String::from)
        .collect()
}

fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f32 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    let inter = a.intersection(b).count() as f32;
    let uni = a.union(b).count() as f32;
    if uni == 0.0 { 0.0 } else { inter / uni }
}

fn find_near_dups(cards: &[CardLoc], threshold: f32) -> Vec<DupGroup> {
    let toks: Vec<HashSet<String>> = cards.iter().map(|c| tokens(&c.front)).collect();
    let lens: Vec<usize> = toks.iter().map(|t| t.len()).collect();
    let mut out = Vec::new();
    for i in 0..cards.len() {
        if lens[i] == 0 {
            continue;
        }
        for j in (i + 1)..cards.len() {
            if lens[j] == 0 {
                continue;
            }
            // length-based prefilter: jaccard >= t requires |A∩B| >= t*|A∪B|
            // and |A∪B| >= max(|A|,|B|), so a fast min/max ratio bound applies.
            let (lo, hi) = if lens[i] < lens[j] {
                (lens[i], lens[j])
            } else {
                (lens[j], lens[i])
            };
            if (lo as f32) < threshold * (hi as f32) {
                continue;
            }
            let s = jaccard(&toks[i], &toks[j]);
            if s >= threshold {
                out.push(DupGroup {
                    similarity: s,
                    a: cards[i].clone(),
                    b: cards[j].clone(),
                });
            }
        }
    }
    out
}

pub fn print_report(r: &LintReport, dry_run: bool, vault_root: &Path) {
    let action = if dry_run { "would stamp" } else { "stamped" };
    println!("Concepts {}: {}", action, r.concept_stamps.len());
    for p in &r.concept_stamps {
        println!("  {}", display_rel(p, vault_root));
    }
    println!("Card UUIDs {}: {}", action, r.card_stamps.len());
    for (p, i) in &r.card_stamps {
        println!("  {} #{}", display_rel(p, vault_root), i);
    }
    println!("Near-duplicate front pairs (jaccard >= 0.85): {}", r.dup_groups.len());
    for g in r.dup_groups.iter().take(50) {
        println!(
            "  {:.2}  {} #{}\n        {} #{}\n        \"{}\" <-> \"{}\"",
            g.similarity,
            display_rel(&g.a.path, vault_root),
            g.a.card_index,
            display_rel(&g.b.path, vault_root),
            g.b.card_index,
            truncate(&g.a.front, 60),
            truncate(&g.b.front, 60),
        );
    }
    if r.dup_groups.len() > 50 {
        println!("  ...and {} more", r.dup_groups.len() - 50);
    }
}

fn display_rel(p: &Path, root: &Path) -> String {
    p.strip_prefix(root)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| p.display().to_string())
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n).collect();
        out.push('…');
        out
    }
}
