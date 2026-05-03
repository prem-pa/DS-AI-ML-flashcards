-- 0002_profiles_sessions.sql
-- Per-profile metadata + review-session tracking.
-- Each profile is its own SQLite file under
--   ~/Library/Application Support/flashcards/profiles/<slug>.db
-- so all rows here are scoped to *this* profile.

CREATE TABLE IF NOT EXISTS profile_meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
-- canonical keys: slug, display_name, created_at, last_active_at

CREATE TABLE IF NOT EXISTS sessions (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at       INTEGER NOT NULL,
    ended_at         INTEGER,                       -- NULL while open
    scope_track      TEXT,                          -- NULL = all tracks
    scope_topic      TEXT,                          -- NULL = all topics
    scope_difficulty TEXT,                          -- NULL = all; else beginner|intermediate|advanced
    cards_reviewed   INTEGER NOT NULL DEFAULT 0,
    last_card_id     TEXT,                          -- where we stopped, NULL if not yet reviewed any
    resumed_from     INTEGER REFERENCES sessions(id) -- if this session resumed another
);

CREATE INDEX IF NOT EXISTS idx_sessions_open      ON sessions(ended_at) WHERE ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_sessions_started   ON sessions(started_at DESC);
