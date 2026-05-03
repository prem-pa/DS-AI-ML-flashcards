use anyhow::{Context, Result};
use directories::BaseDirs;
use rusqlite::Connection;
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
    if let Some(slug_in) = &cli.profile {
        let slug = crate::profile::slugify(slug_in);
        if !slug.is_empty() {
            return crate::profile::db_path_for(&slug);
        }
    }
    if let Ok(Some(last)) = crate::profile::read_last_used() {
        let p = crate::profile::db_path_for(&last)?;
        if p.exists() {
            return Ok(p);
        }
    }
    // Legacy single-DB fallback for non-interactive subcommands (sync, lint, reset).
    let bd = BaseDirs::new().context("could not determine home directory")?;
    let dir = bd.data_local_dir().join("flashcards");
    std::fs::create_dir_all(&dir).context("creating app data dir")?;
    Ok(dir.join("flashcards.db"))
}

/// Open a connection that respects `--db`, `--profile`, $FLASHCARDS_PROFILE,
/// and the last-used profile. When a profile is involved, this also writes
/// the metadata rows so the profile shows up in the picker.
pub fn open_db(cli: &Cli) -> Result<Connection> {
    if let Some(path) = &cli.db {
        return crate::db::open(path);
    }
    if let Some(slug_in) = &cli.profile {
        let slug = crate::profile::slugify(slug_in);
        if !slug.is_empty() {
            return crate::profile::open_or_create(&slug, slug_in);
        }
    }
    if let Ok(Some(last)) = crate::profile::read_last_used() {
        if crate::profile::db_path_for(&last)?.exists() {
            return crate::profile::open_or_create(&last, &last);
        }
    }
    crate::db::open(&db_path(cli)?)
}
