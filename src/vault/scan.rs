use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::parse::{parse, ConceptNote};

pub struct ScannedNote {
    pub path: PathBuf,
    pub rel_path: PathBuf,
    pub content_hash: String,
    pub raw: String,
    pub parsed: Result<ConceptNote>,
}

/// Walk a vault root, returning every `.md` note (excluding the Obsidian config
/// directory and the templates folder).
pub fn scan(vault_root: &Path) -> Result<Vec<ScannedNote>> {
    let mut out = Vec::new();
    for entry in WalkDir::new(vault_root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // skip dotdirs (.obsidian, .git, .DS_Store etc.) and templates folder
            !(name.starts_with('.') || name == "_templates")
        })
    {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("skip (walk err): {err}");
                continue;
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let path = entry.path().to_path_buf();
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let content_hash = blake3::hash(raw.as_bytes()).to_hex().to_string();
        let rel_path = path
            .strip_prefix(vault_root)
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| path.clone());
        let parsed = parse(&raw);
        out.push(ScannedNote {
            path,
            rel_path,
            content_hash,
            raw,
            parsed,
        });
    }
    Ok(out)
}
