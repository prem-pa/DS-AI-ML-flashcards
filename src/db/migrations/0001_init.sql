-- 0001_init.sql
-- Schema for flashcards. The Markdown vault is the source of truth for content;
-- this DB only holds scheduling state and indexes for fast queries.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS _meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS concepts (
    id            TEXT PRIMARY KEY,             -- uuid
    slug          TEXT NOT NULL,
    title         TEXT NOT NULL,
    track         TEXT NOT NULL,
    topic         TEXT NOT NULL,
    difficulty    INTEGER NOT NULL DEFAULT 3,    -- 1..5
    path          TEXT NOT NULL UNIQUE,          -- vault-relative path
    content_hash  TEXT NOT NULL,                 -- blake3 hex of file content
    created_at    INTEGER NOT NULL,              -- unix epoch seconds
    updated_at    INTEGER NOT NULL,
    deleted_at    INTEGER                        -- soft-delete marker
);

CREATE INDEX IF NOT EXISTS idx_concepts_track_topic ON concepts(track, topic);
CREATE INDEX IF NOT EXISTS idx_concepts_slug ON concepts(slug);

CREATE TABLE IF NOT EXISTS concept_tags (
    concept_id  TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    tag         TEXT NOT NULL,
    PRIMARY KEY (concept_id, tag)
);

CREATE INDEX IF NOT EXISTS idx_concept_tags_tag ON concept_tags(tag);

CREATE TABLE IF NOT EXISTS concept_aliases (
    concept_id  TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    alias       TEXT NOT NULL,
    PRIMARY KEY (concept_id, alias)
);

CREATE TABLE IF NOT EXISTS concept_sources (
    concept_id  TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    url         TEXT NOT NULL,
    label       TEXT,
    PRIMARY KEY (concept_id, url)
);

-- Resolved wikilinks: src -> dst by concept_id. Unresolved links are dropped here
-- and instead reported by `flashcards lint`.
CREATE TABLE IF NOT EXISTS concept_links (
    src_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    dst_id TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    PRIMARY KEY (src_id, dst_id)
);

CREATE TABLE IF NOT EXISTS cards (
    id          TEXT PRIMARY KEY,             -- uuid; stable join key to card_state
    concept_id  TEXT NOT NULL REFERENCES concepts(id) ON DELETE CASCADE,
    type        TEXT NOT NULL CHECK(type IN ('flip','mcq')),
    front       TEXT NOT NULL,
    back        TEXT NOT NULL,
    choices_json TEXT,                         -- JSON array of {key,text,correct} for mcq
    position    INTEGER NOT NULL,              -- order within concept
    suspended   INTEGER NOT NULL DEFAULT 0,    -- 1 if vacated from disk; preserves history
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cards_concept ON cards(concept_id);
CREATE INDEX IF NOT EXISTS idx_cards_suspended ON cards(suspended);

-- FSRS state, one row per card. INSERT ... ON CONFLICT DO NOTHING during sync
-- so that re-imports never overwrite live scheduling state.
CREATE TABLE IF NOT EXISTS card_state (
    card_id          TEXT PRIMARY KEY REFERENCES cards(id) ON DELETE CASCADE,
    due              INTEGER NOT NULL,        -- unix epoch seconds
    stability        REAL NOT NULL DEFAULT 0,
    difficulty       REAL NOT NULL DEFAULT 0,
    elapsed_days     REAL NOT NULL DEFAULT 0,
    scheduled_days   REAL NOT NULL DEFAULT 0,
    reps             INTEGER NOT NULL DEFAULT 0,
    lapses           INTEGER NOT NULL DEFAULT 0,
    state            INTEGER NOT NULL DEFAULT 0,  -- 0=new 1=learning 2=review 3=relearning
    last_review      INTEGER                    -- unix epoch seconds, NULL if never reviewed
);

CREATE INDEX IF NOT EXISTS idx_card_state_due ON card_state(due);

-- Append-only review log. Survives card edits and concept renames.
CREATE TABLE IF NOT EXISTS reviews (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    card_id       TEXT NOT NULL,           -- not a FK: keep history if card row is deleted
    rating        INTEGER NOT NULL,        -- 1=again, 2=hard, 3=good, 4=easy
    reviewed_at   INTEGER NOT NULL,
    elapsed_days  REAL,
    scheduled_days REAL,
    state_before  INTEGER,
    state_after   INTEGER
);

CREATE INDEX IF NOT EXISTS idx_reviews_card ON reviews(card_id);
CREATE INDEX IF NOT EXISTS idx_reviews_reviewed_at ON reviews(reviewed_at);
