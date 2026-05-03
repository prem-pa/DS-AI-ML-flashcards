use anyhow::{Context, Result};
use directories::BaseDirs;
use std::path::PathBuf;

use crate::cli::Cli;

/// Resolve the vault path: --vault override, else `<cwd>/vault`.
pub fn vault_path(cli: &Cli) -> Result<PathBuf> {
    if let Some(p) = &cli.vault {
        return Ok(p.clone());
    }
    let cwd = std::env::current_dir().context("getting cwd")?;
    Ok(cwd.join("vault"))
}

/// Resolve the SQLite DB path:
/// 1. --db override
/// 2. $XDG_DATA_HOME/flashcards/flashcards.db (Linux) or
///    ~/Library/Application Support/flashcards/flashcards.db (macOS)
///
/// Important: never inside the project dir if it's under Google Drive
/// — cloud sync corrupts SQLite.
pub fn db_path(cli: &Cli) -> Result<PathBuf> {
    if let Some(p) = &cli.db {
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        return Ok(p.clone());
    }
    let bd = BaseDirs::new().context("could not determine home directory")?;
    // On macOS this is ~/Library/Application Support/flashcards/
    // On Linux: $XDG_DATA_HOME/flashcards/ (defaults to ~/.local/share/flashcards/)
    let dir = bd.data_local_dir().join("flashcards");
    std::fs::create_dir_all(&dir).context("creating app data dir")?;
    Ok(dir.join("flashcards.db"))
}
