use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;

const MIGRATIONS: &[(i32, &str)] = &[
    (1, include_str!("migrations/0001_init.sql")),
];

pub fn open(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let conn = Connection::open(path).with_context(|| format!("opening DB at {}", path.display()))?;
    // WAL is fine here because we put the DB outside Drive (see util::db_path).
    conn.pragma_update(None, "journal_mode", "wal")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    migrate(&conn)?;
    Ok(conn)
}

fn migrate(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
    )?;
    let current: i32 = conn
        .query_row(
            "SELECT value FROM _meta WHERE key='schema_version'",
            [],
            |row| row.get::<_, String>(0).map(|s| s.parse::<i32>().unwrap_or(0)),
        )
        .unwrap_or(0);

    for (version, sql) in MIGRATIONS {
        if *version > current {
            conn.execute_batch(sql)
                .with_context(|| format!("applying migration {}", version))?;
            conn.execute(
                "INSERT INTO _meta(key,value) VALUES('schema_version', ?1)
                 ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                params![version.to_string()],
            )?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardChoice {
    pub key: String,
    pub text: String,
    #[serde(default)]
    pub correct: bool,
}

#[derive(Debug, Clone)]
pub struct CardView {
    pub id: String,
    pub concept_id: String,
    pub concept_title: String,
    pub concept_slug: String,
    pub track: String,
    pub topic: String,
    pub kind: String,
    pub front: String,
    pub back: String,
    pub choices: Vec<CardChoice>,
    pub due: i64,
    pub reps: i64,
    pub state: i64,
}

const CARD_SELECT: &str = "
    SELECT c.id, c.concept_id, k.title, k.slug, k.track, k.topic,
           c.type, c.front, c.back, c.choices_json,
           s.due, s.reps, s.state
    FROM cards c
    JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
    JOIN card_state s ON s.card_id = c.id
    WHERE c.suspended = 0
";

fn map_card_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<CardView> {
    let choices_json: Option<String> = row.get(9)?;
    let choices = choices_json
        .as_deref()
        .and_then(|s| serde_json::from_str::<Vec<CardChoice>>(s).ok())
        .unwrap_or_default();
    Ok(CardView {
        id: row.get(0)?,
        concept_id: row.get(1)?,
        concept_title: row.get(2)?,
        concept_slug: row.get(3)?,
        track: row.get(4)?,
        topic: row.get(5)?,
        kind: row.get(6)?,
        front: row.get(7)?,
        back: row.get(8)?,
        choices,
        due: row.get(10)?,
        reps: row.get(11)?,
        state: row.get(12)?,
    })
}

/// Cards due at or before `now`, oldest-due first, then by track/topic for stable ordering.
pub fn fetch_due(conn: &Connection, now: i64, limit: usize) -> Result<Vec<CardView>> {
    let sql = format!(
        "{CARD_SELECT} AND s.due <= ?1 ORDER BY s.due ASC, k.track, k.topic, c.position LIMIT ?2"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![now, limit as i64], map_card_row)?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

/// All non-suspended cards, ordered for browsing. Cap large because the vault is bounded.
pub fn fetch_all(conn: &Connection, limit: usize) -> Result<Vec<CardView>> {
    let sql = format!(
        "{CARD_SELECT} ORDER BY k.track, k.topic, k.title, c.position LIMIT ?1"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![limit as i64], map_card_row)?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

pub fn fetch_one(conn: &Connection, card_id: &str) -> Result<Option<CardView>> {
    let sql = format!("{CARD_SELECT} AND c.id = ?1 LIMIT 1");
    let row = conn.query_row(&sql, params![card_id], map_card_row).optional()?;
    Ok(row)
}

pub fn concept_path(conn: &Connection, concept_id: &str) -> Result<Option<String>> {
    Ok(conn
        .query_row(
            "SELECT path FROM concepts WHERE id = ?1",
            params![concept_id],
            |r| r.get::<_, String>(0),
        )
        .optional()?)
}

pub fn count_due(conn: &Connection, now: i64) -> Result<i64> {
    let n: i64 = conn.query_row(
        "SELECT COUNT(*) FROM card_state s
         JOIN cards c ON c.id = s.card_id
         JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
         WHERE c.suspended = 0 AND s.due <= ?1",
        params![now],
        |r| r.get(0),
    )?;
    Ok(n)
}
