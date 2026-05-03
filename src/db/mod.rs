use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;

const MIGRATIONS: &[(i32, &str)] = &[
    (1, include_str!("migrations/0001_init.sql")),
    (2, include_str!("migrations/0002_profiles_sessions.sql")),
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
    pub difficulty: i64,
    pub kind: String,
    pub front: String,
    pub back: String,
    pub choices: Vec<CardChoice>,
    pub due: i64,
    pub reps: i64,
    pub lapses: i64,
    pub state: i64,
    pub last_review: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct ConceptMeta {
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub sources: Vec<(String, Option<String>)>,
    /// Resolved [[wikilinks]] from this concept → destination titles.
    pub see_also: Vec<String>,
}

const CARD_SELECT: &str = "
    SELECT c.id, c.concept_id, k.title, k.slug, k.track, k.topic, k.difficulty,
           c.type, c.front, c.back, c.choices_json,
           s.due, s.reps, s.lapses, s.state, s.last_review
    FROM cards c
    JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
    JOIN card_state s ON s.card_id = c.id
    WHERE c.suspended = 0
";

fn map_card_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<CardView> {
    let choices_json: Option<String> = row.get(10)?;
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
        difficulty: row.get(6)?,
        kind: row.get(7)?,
        front: row.get(8)?,
        back: row.get(9)?,
        choices,
        due: row.get(11)?,
        reps: row.get(12)?,
        lapses: row.get(13)?,
        state: row.get(14)?,
        last_review: row.get(15)?,
    })
}

pub fn fetch_concept_meta(conn: &Connection, concept_id: &str) -> Result<ConceptMeta> {
    let tags: Vec<String> = {
        let mut stmt = conn.prepare(
            "SELECT tag FROM concept_tags WHERE concept_id = ?1 ORDER BY tag",
        )?;
        let rows = stmt.query_map(params![concept_id], |r| r.get::<_, String>(0))?;
        rows.collect::<rusqlite::Result<Vec<_>>>()?
    };
    let aliases: Vec<String> = {
        let mut stmt = conn.prepare(
            "SELECT alias FROM concept_aliases WHERE concept_id = ?1 ORDER BY alias",
        )?;
        let rows = stmt.query_map(params![concept_id], |r| r.get::<_, String>(0))?;
        rows.collect::<rusqlite::Result<Vec<_>>>()?
    };
    let sources: Vec<(String, Option<String>)> = {
        let mut stmt = conn.prepare(
            "SELECT url, label FROM concept_sources WHERE concept_id = ?1",
        )?;
        let rows = stmt.query_map(params![concept_id], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?))
        })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()?
    };
    let see_also: Vec<String> = {
        let mut stmt = conn.prepare(
            "SELECT k.title FROM concept_links cl
             JOIN concepts k ON k.id = cl.dst_id AND k.deleted_at IS NULL
             WHERE cl.src_id = ?1
             ORDER BY k.title",
        )?;
        let rows = stmt.query_map(params![concept_id], |r| r.get::<_, String>(0))?;
        rows.collect::<rusqlite::Result<Vec<_>>>()?
    };
    Ok(ConceptMeta {
        tags,
        aliases,
        sources,
        see_also,
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
    fetch_all_scoped(conn, &Scope::default(), limit)
}

pub fn fetch_all_scoped(
    conn: &Connection,
    scope: &Scope,
    limit: usize,
) -> Result<Vec<CardView>> {
    let (where_extra, scope_params) = scope_where(scope);
    let sql = format!(
        "{CARD_SELECT} {} ORDER BY k.track, k.topic, k.title, c.position LIMIT ?",
        where_extra
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut all_params: Vec<rusqlite::types::Value> = Vec::with_capacity(scope_params.len() + 1);
    all_params.extend(scope_params);
    all_params.push((limit as i64).into());
    let rows = stmt.query_map(rusqlite::params_from_iter(all_params), map_card_row)?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    All,
    Beginner,    // frontmatter difficulty 1-2
    Intermediate, // 3
    Advanced,    // 4-5
}

impl Difficulty {
    pub fn as_db_token(self) -> Option<&'static str> {
        match self {
            Difficulty::All => None,
            Difficulty::Beginner => Some("beginner"),
            Difficulty::Intermediate => Some("intermediate"),
            Difficulty::Advanced => Some("advanced"),
        }
    }
    pub fn from_db_token(s: Option<&str>) -> Self {
        match s {
            Some("beginner") => Difficulty::Beginner,
            Some("intermediate") => Difficulty::Intermediate,
            Some("advanced") => Difficulty::Advanced,
            _ => Difficulty::All,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Difficulty::All => "all",
            Difficulty::Beginner => "beginner (1-2)",
            Difficulty::Intermediate => "intermediate (3)",
            Difficulty::Advanced => "advanced (4-5)",
        }
    }
    pub fn range(self) -> (i64, i64) {
        match self {
            Difficulty::All => (1, 5),
            Difficulty::Beginner => (1, 2),
            Difficulty::Intermediate => (3, 3),
            Difficulty::Advanced => (4, 5),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub track: Option<String>,
    pub topic: Option<String>,
    pub difficulty: Difficulty,
    /// "flip" / "mcq" / None (both). Applied via the `mcq_only` preference; not
    /// persisted in the sessions table — re-applied from current prefs each time.
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Prefs {
    pub mcq_only: bool,
}

pub fn load_prefs(conn: &Connection) -> Result<Prefs> {
    let v: Option<String> = conn
        .query_row(
            "SELECT value FROM profile_meta WHERE key='mcq_only'",
            [],
            |r| r.get(0),
        )
        .optional()?;
    Ok(Prefs {
        mcq_only: matches!(v.as_deref(), Some("1") | Some("true")),
    })
}

pub fn set_mcq_only(conn: &Connection, on: bool) -> Result<()> {
    conn.execute(
        "INSERT INTO profile_meta(key, value) VALUES ('mcq_only', ?1)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        params![if on { "1" } else { "0" }],
    )?;
    Ok(())
}

/// Apply current preferences to a scope (e.g. `mcq_only` → `kind = Some("mcq")`).
/// Idempotent if already constrained.
pub fn apply_prefs(scope: &mut Scope, prefs: &Prefs) {
    if prefs.mcq_only && scope.kind.is_none() {
        scope.kind = Some("mcq".to_string());
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::All
    }
}

impl Scope {
    pub fn label(&self) -> String {
        let mut parts = Vec::new();
        if let Some(t) = &self.track {
            parts.push(format!("track={}", t));
        }
        if let Some(t) = &self.topic {
            parts.push(format!("topic={}", t));
        }
        if self.difficulty != Difficulty::All {
            parts.push(format!("difficulty={}", self.difficulty.label()));
        }
        if parts.is_empty() {
            "all".into()
        } else {
            parts.join(", ")
        }
    }
}

fn scope_where(scope: &Scope) -> (String, Vec<rusqlite::types::Value>) {
    let mut clauses = Vec::new();
    let mut params: Vec<rusqlite::types::Value> = Vec::new();
    if let Some(t) = &scope.track {
        clauses.push("k.track = ?".to_string());
        params.push(t.clone().into());
    }
    if let Some(tp) = &scope.topic {
        clauses.push("k.topic = ?".to_string());
        params.push(tp.clone().into());
    }
    let (lo, hi) = scope.difficulty.range();
    if !(lo == 1 && hi == 5) {
        clauses.push("k.difficulty BETWEEN ? AND ?".to_string());
        params.push((lo as i64).into());
        params.push((hi as i64).into());
    }
    if let Some(k) = &scope.kind {
        clauses.push("c.type = ?".to_string());
        params.push(k.clone().into());
    }
    let sql = if clauses.is_empty() {
        String::new()
    } else {
        format!(" AND {}", clauses.join(" AND "))
    };
    (sql, params)
}

pub fn fetch_due_scoped(
    conn: &Connection,
    now: i64,
    scope: &Scope,
    limit: usize,
) -> Result<Vec<CardView>> {
    let (where_extra, scope_params) = scope_where(scope);
    let sql = format!(
        "{CARD_SELECT} AND s.due <= ? {} ORDER BY s.due ASC, k.track, k.topic, c.position LIMIT ?",
        where_extra
    );
    let mut stmt = conn.prepare(&sql)?;
    // parameter order: now (?1), then scope params, then limit
    let mut all_params: Vec<rusqlite::types::Value> = Vec::with_capacity(scope_params.len() + 2);
    all_params.push((now as i64).into());
    all_params.extend(scope_params);
    all_params.push((limit as i64).into());
    let rows = stmt.query_map(rusqlite::params_from_iter(all_params), map_card_row)?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

pub fn count_due_scoped(conn: &Connection, now: i64, scope: &Scope) -> Result<i64> {
    let (where_extra, scope_params) = scope_where(scope);
    let sql = format!(
        "SELECT COUNT(*) FROM card_state s
         JOIN cards c ON c.id = s.card_id
         JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
         WHERE c.suspended = 0 AND s.due <= ? {}",
        where_extra
    );
    let mut all_params: Vec<rusqlite::types::Value> = Vec::with_capacity(scope_params.len() + 1);
    all_params.push((now as i64).into());
    all_params.extend(scope_params);
    let n: i64 = conn.query_row(&sql, rusqlite::params_from_iter(all_params), |r| r.get(0))?;
    Ok(n)
}

pub fn count_total_scoped(conn: &Connection, scope: &Scope) -> Result<i64> {
    let (where_extra, scope_params) = scope_where(scope);
    let sql = format!(
        "SELECT COUNT(*) FROM card_state s
         JOIN cards c ON c.id = s.card_id
         JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
         WHERE c.suspended = 0 {}",
        where_extra
    );
    let n: i64 = conn.query_row(&sql, rusqlite::params_from_iter(scope_params), |r| r.get(0))?;
    Ok(n)
}

#[derive(Debug, Clone)]
pub struct TrackTopicCount {
    pub track: String,
    pub topic: String,
    pub n_cards: i64,
}

pub fn list_topics(conn: &Connection) -> Result<Vec<TrackTopicCount>> {
    list_topics_filtered(conn, &Prefs::default())
}

pub fn list_topics_filtered(conn: &Connection, prefs: &Prefs) -> Result<Vec<TrackTopicCount>> {
    let extra = if prefs.mcq_only { " AND c.type = 'mcq'" } else { "" };
    let sql = format!(
        "SELECT k.track, k.topic, COUNT(*)
         FROM cards c
         JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
         WHERE c.suspended = 0{}
         GROUP BY k.track, k.topic
         ORDER BY k.track, k.topic",
        extra
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |r| {
        Ok(TrackTopicCount {
            track: r.get(0)?,
            topic: r.get(1)?,
            n_cards: r.get(2)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

#[derive(Debug, Clone)]
pub struct SessionRow {
    pub id: i64,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub scope: Scope,
    pub cards_reviewed: i64,
    pub last_card_id: Option<String>,
}

pub fn open_session(conn: &Connection, scope: &Scope) -> Result<i64> {
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "INSERT INTO sessions(started_at, scope_track, scope_topic, scope_difficulty)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            now,
            scope.track,
            scope.topic,
            scope.difficulty.as_db_token(),
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn touch_session(
    conn: &Connection,
    session_id: i64,
    last_card_id: &str,
    cards_reviewed: i64,
) -> Result<()> {
    conn.execute(
        "UPDATE sessions SET cards_reviewed = ?2, last_card_id = ?3 WHERE id = ?1",
        params![session_id, cards_reviewed, last_card_id],
    )?;
    Ok(())
}

pub fn close_session(conn: &Connection, session_id: i64) -> Result<()> {
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "UPDATE sessions SET ended_at = ?2 WHERE id = ?1 AND ended_at IS NULL",
        params![session_id, now],
    )?;
    Ok(())
}

pub fn most_recent_session(conn: &Connection) -> Result<Option<SessionRow>> {
    let row = conn
        .query_row(
            "SELECT id, started_at, ended_at, scope_track, scope_topic, scope_difficulty,
                    cards_reviewed, last_card_id
             FROM sessions ORDER BY id DESC LIMIT 1",
            [],
            |r| {
                let scope = Scope {
                    track: r.get::<_, Option<String>>(3)?,
                    topic: r.get::<_, Option<String>>(4)?,
                    difficulty: Difficulty::from_db_token(
                        r.get::<_, Option<String>>(5)?.as_deref(),
                    ),
                    kind: None,
                };
                Ok(SessionRow {
                    id: r.get(0)?,
                    started_at: r.get(1)?,
                    ended_at: r.get(2)?,
                    scope,
                    cards_reviewed: r.get(6)?,
                    last_card_id: r.get(7)?,
                })
            },
        )
        .optional()?;
    Ok(row)
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
