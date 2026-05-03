//! Per-profile DB management.
//!
//! Each profile is a self-contained SQLite file under
//!   `<data_local_dir>/flashcards/profiles/<slug>.db`
//! plus a tiny `<data_local_dir>/flashcards/last_profile.txt` that remembers
//! the most-recently used profile slug for "open at default profile" UX.

use anyhow::{Context, Result};
use chrono::Utc;
use directories::BaseDirs;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;

use crate::db;

pub mod names;

#[derive(Debug, Clone)]
pub struct ProfileSummary {
    pub slug: String,
    pub display_name: String,
    pub created_at: i64,
    pub last_active_at: i64,
    pub path: PathBuf,
}

pub fn profiles_dir() -> Result<PathBuf> {
    let bd = BaseDirs::new().context("could not determine home directory")?;
    let dir = bd.data_local_dir().join("flashcards").join("profiles");
    std::fs::create_dir_all(&dir).context("creating profiles dir")?;
    Ok(dir)
}

pub fn last_profile_path() -> Result<PathBuf> {
    let bd = BaseDirs::new().context("could not determine home directory")?;
    Ok(bd.data_local_dir().join("flashcards").join("last_profile.txt"))
}

pub fn db_path_for(slug: &str) -> Result<PathBuf> {
    Ok(profiles_dir()?.join(format!("{slug}.db")))
}

/// Lowercase alnum + hyphens. Trims leading/trailing hyphens.
pub fn slugify(s: &str) -> String {
    let lower = s.trim().to_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut prev_dash = false;
    for ch in lower.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_dash = false;
        } else if ch == '-' || ch == '_' || ch.is_whitespace() {
            if !prev_dash && !out.is_empty() {
                out.push('-');
                prev_dash = true;
            }
        }
    }
    out.trim_matches('-').to_string()
}

pub fn list() -> Result<Vec<ProfileSummary>> {
    let dir = profiles_dir()?;
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).context("reading profiles dir")? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("db") {
            continue;
        }
        let conn = match Connection::open(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // Don't run migrations here — we just want metadata; skip files that
        // don't have profile_meta yet (legacy single-profile DB).
        let meta_exists: bool = conn
            .query_row(
                "SELECT 1 FROM sqlite_master WHERE type='table' AND name='profile_meta'",
                [],
                |_| Ok(true),
            )
            .optional()?
            .unwrap_or(false);
        if !meta_exists {
            continue;
        }
        let slug = read_meta(&conn, "slug")?
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string()
            });
        let display_name = read_meta(&conn, "display_name")?.unwrap_or_else(|| slug.clone());
        let created_at = read_meta(&conn, "created_at")?
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0);
        let last_active_at = read_meta(&conn, "last_active_at")?
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0);
        out.push(ProfileSummary {
            slug,
            display_name,
            created_at,
            last_active_at,
            path,
        });
    }
    out.sort_by(|a, b| b.last_active_at.cmp(&a.last_active_at));
    Ok(out)
}

fn read_meta(conn: &Connection, key: &str) -> Result<Option<String>> {
    Ok(conn
        .query_row(
            "SELECT value FROM profile_meta WHERE key = ?1",
            params![key],
            |r| r.get::<_, String>(0),
        )
        .optional()?)
}

/// Open (and migrate) a profile's DB; create the meta rows if absent.
pub fn open_or_create(slug: &str, display_name: &str) -> Result<Connection> {
    let path = db_path_for(slug)?;
    let conn = db::open(&path)?;
    let now = Utc::now().timestamp();
    // INSERT OR IGNORE so existing values stick; then bump last_active_at always.
    conn.execute(
        "INSERT OR IGNORE INTO profile_meta(key, value) VALUES ('slug', ?1)",
        params![slug],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO profile_meta(key, value) VALUES ('display_name', ?1)",
        params![display_name],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO profile_meta(key, value) VALUES ('created_at', ?1)",
        params![now.to_string()],
    )?;
    conn.execute(
        "INSERT INTO profile_meta(key, value) VALUES ('last_active_at', ?1)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        params![now.to_string()],
    )?;
    save_last_used(slug)?;
    Ok(conn)
}

pub fn touch_last_active(conn: &Connection) -> Result<()> {
    let now = Utc::now().timestamp();
    conn.execute(
        "INSERT INTO profile_meta(key, value) VALUES ('last_active_at', ?1)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        params![now.to_string()],
    )?;
    Ok(())
}

pub fn save_last_used(slug: &str) -> Result<()> {
    let p = last_profile_path()?;
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(&p, slug).ok();
    Ok(())
}

pub fn read_last_used() -> Result<Option<String>> {
    let p = last_profile_path()?;
    if !p.exists() {
        return Ok(None);
    }
    let s = std::fs::read_to_string(&p).context("reading last_profile")?;
    let s = s.trim().to_string();
    Ok(if s.is_empty() { None } else { Some(s) })
}

pub fn delete_profile(slug: &str) -> Result<()> {
    let p = db_path_for(slug)?;
    if p.exists() {
        std::fs::remove_file(&p).ok();
    }
    // Drop accompanying WAL/SHM sidecars too.
    for ext in &["db-wal", "db-shm"] {
        let side = profiles_dir()?.join(format!("{slug}.{ext}"));
        if side.exists() {
            let _ = std::fs::remove_file(side);
        }
    }
    if let Ok(Some(last)) = read_last_used() {
        if last == slug {
            let _ = std::fs::remove_file(last_profile_path()?);
        }
    }
    Ok(())
}

/// Rename a profile. If `new_slug` differs from `old_slug`, the underlying
/// `<old>.db` file is moved to `<new>.db` (along with WAL/SHM sidecars), and
/// `last_profile.txt` is updated if it pointed at the old slug. Either way,
/// the profile_meta `slug` and `display_name` rows are updated.
pub fn rename_profile(old_slug: &str, new_slug: &str, new_display: &str) -> Result<()> {
    let old_path = db_path_for(old_slug)?;
    if !old_path.exists() {
        return Err(anyhow::anyhow!("profile '{old_slug}' has no DB at {}", old_path.display()));
    }
    if new_slug == old_slug {
        // display-name-only rename
        let conn = Connection::open(&old_path)?;
        write_meta(&conn, "display_name", new_display)?;
        return Ok(());
    }
    // slug change: ensure target is free, then move all sidecars
    let new_path = db_path_for(new_slug)?;
    if new_path.exists() {
        return Err(anyhow::anyhow!("profile '{new_slug}' already exists"));
    }
    std::fs::rename(&old_path, &new_path).context("renaming profile DB file")?;
    let dir = profiles_dir()?;
    for ext in &["db-wal", "db-shm"] {
        let from = dir.join(format!("{old_slug}.{ext}"));
        if from.exists() {
            let to = dir.join(format!("{new_slug}.{ext}"));
            let _ = std::fs::rename(&from, &to);
        }
    }
    {
        let conn = Connection::open(&new_path)?;
        write_meta(&conn, "slug", new_slug)?;
        write_meta(&conn, "display_name", new_display)?;
    }
    if let Ok(Some(last)) = read_last_used() {
        if last == old_slug {
            save_last_used(new_slug)?;
        }
    }
    Ok(())
}

fn write_meta(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO profile_meta(key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        params![key, value],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_punctuation_and_space() {
        assert_eq!(slugify("Drowsy Otter 42"), "drowsy-otter-42");
        assert_eq!(slugify("  hello---world  "), "hello-world");
        assert_eq!(slugify("@@@!"), "");
        assert_eq!(slugify("WIBBLY_wobbly"), "wibbly-wobbly");
    }
}
