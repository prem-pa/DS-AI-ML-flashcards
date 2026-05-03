use anyhow::{anyhow, Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardChoice {
    pub key: String,
    pub text: String,
    #[serde(default)]
    pub correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardNote {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub kind: String, // "flip" | "mcq"
    pub front: String,
    pub back: String,
    #[serde(default)]
    pub choices: Option<Vec<CardChoice>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub url: String,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    #[serde(default)]
    pub id: Option<String>,
    pub title: String,
    pub track: String,
    pub topic: String,
    #[serde(default = "default_difficulty")]
    pub difficulty: i64,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub sources: Vec<Source>,
    #[serde(default)]
    pub cards: Vec<CardNote>,
}

fn default_difficulty() -> i64 {
    3
}

#[derive(Debug, Clone)]
pub struct ConceptNote {
    pub frontmatter: Frontmatter,
    pub body: String,
    /// Wikilink slugs extracted from the body. `[[Foo Bar]]` -> "foo-bar".
    pub wikilinks: Vec<String>,
}

/// Extract the leading YAML frontmatter block. Returns (frontmatter_yaml, body).
pub fn split_frontmatter(text: &str) -> Result<(&str, &str)> {
    let trimmed = text.trim_start_matches('\u{feff}');
    let rest = trimmed
        .strip_prefix("---\n")
        .or_else(|| trimmed.strip_prefix("---\r\n"))
        .ok_or_else(|| anyhow!("note is missing leading `---` frontmatter"))?;
    // Find the closing fence on its own line.
    let close_idx = find_closing_fence(rest)
        .ok_or_else(|| anyhow!("note is missing closing `---` frontmatter fence"))?;
    let yaml = &rest[..close_idx];
    let after = &rest[close_idx..];
    let after = after
        .strip_prefix("---\n")
        .or_else(|| after.strip_prefix("---\r\n"))
        .or_else(|| after.strip_prefix("---"))
        .unwrap_or(after);
    Ok((yaml, after))
}

fn find_closing_fence(s: &str) -> Option<usize> {
    let mut idx = 0;
    while idx < s.len() {
        // Find next newline; the frontmatter close must be at a line start.
        let line_start = idx;
        let nl = s[idx..].find('\n').map(|n| idx + n);
        let line_end = nl.unwrap_or(s.len());
        let line = &s[line_start..line_end];
        let line_trimmed_right = line.trim_end_matches('\r');
        if line_trimmed_right == "---" {
            return Some(line_start);
        }
        match nl {
            Some(p) => idx = p + 1,
            None => break,
        }
    }
    None
}

pub fn parse(text: &str) -> Result<ConceptNote> {
    let (yaml, body) = split_frontmatter(text)?;
    let frontmatter: Frontmatter =
        serde_yml::from_str(yaml).context("parsing YAML frontmatter")?;
    let wikilinks = extract_wikilinks(body);
    Ok(ConceptNote {
        frontmatter,
        body: body.to_string(),
        wikilinks,
    })
}

static WIKILINK_RE: OnceLock<Regex> = OnceLock::new();

/// `[[Some Title]]` or `[[some-slug|display]]` -> slug strings ("some-title", "some-slug").
pub fn extract_wikilinks(text: &str) -> Vec<String> {
    let re = WIKILINK_RE.get_or_init(|| Regex::new(r"\[\[([^\[\]]+?)\]\]").unwrap());
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for cap in re.captures_iter(text) {
        let inner = cap.get(1).unwrap().as_str();
        // Drop optional display text after `|`.
        let target = inner.split('|').next().unwrap_or(inner).trim();
        // Drop optional fragment (#section).
        let target = target.split('#').next().unwrap_or(target).trim();
        if target.is_empty() {
            continue;
        }
        let slug = slugify(target);
        if slug.is_empty() {
            continue;
        }
        if seen.insert(slug.clone()) {
            out.push(slug);
        }
    }
    out
}

pub fn slugify(s: &str) -> String {
    let lower = s.trim().to_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut prev_dash = false;
    for ch in lower.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_dash = false;
        } else if ch == '-' || ch.is_whitespace() || ch == '_' {
            if !prev_dash && !out.is_empty() {
                out.push('-');
                prev_dash = true;
            }
        }
        // drop everything else
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "---
id: 11111111-1111-1111-1111-111111111111
title: Bias-Variance Tradeoff
track: data-scientist
topic: foundational-statistics-probability
difficulty: 2
tags: [stats, generalization]
aliases: [BV tradeoff]
sources:
  - url: https://example.com
    label: Example
cards:
  - id: 22222222-2222-2222-2222-222222222222
    type: flip
    front: What is bias?
    back: Systematic error.
  - type: mcq
    front: Pick the right one.
    back: Because reasons.
    choices:
      - key: a
        text: First
      - key: b
        text: Second
        correct: true
---

# Body

See [[Variance]] and [[bias|the bias note]].
";

    #[test]
    fn parses_full_note() {
        let note = parse(FIXTURE).unwrap();
        assert_eq!(note.frontmatter.title, "Bias-Variance Tradeoff");
        assert_eq!(note.frontmatter.track, "data-scientist");
        assert_eq!(note.frontmatter.cards.len(), 2);
        assert_eq!(note.frontmatter.cards[0].kind, "flip");
        assert_eq!(note.frontmatter.cards[1].kind, "mcq");
        let mcq_choices = note.frontmatter.cards[1].choices.as_ref().unwrap();
        assert_eq!(mcq_choices.len(), 2);
        assert!(mcq_choices[1].correct);
        assert_eq!(note.wikilinks, vec!["variance".to_string(), "bias".to_string()]);
    }

    #[test]
    fn slugify_basic() {
        assert_eq!(slugify("Bias-Variance Tradeoff"), "bias-variance-tradeoff");
        assert_eq!(slugify("  HNSW vs IVF  "), "hnsw-vs-ivf");
        assert_eq!(slugify("foo / bar (baz)"), "foo-bar-baz");
    }

    #[test]
    fn missing_frontmatter_errors() {
        let err = parse("no frontmatter here").unwrap_err();
        assert!(format!("{err}").contains("frontmatter"));
    }
}
