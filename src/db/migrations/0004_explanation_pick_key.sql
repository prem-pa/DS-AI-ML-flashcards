-- 0004_explanation_pick_key.sql
-- Re-key card_explanations on (card_id, picked_key) so we can cache one
-- response per (card, user's pick). The original v3 schema cached one
-- option-agnostic response per card; that prompt design changed (the LLM now
-- gets the picked option and tailors its answer), so the old rows aren't
-- reusable. Drop and recreate.

DROP TABLE IF EXISTS card_explanations;

CREATE TABLE card_explanations (
    card_id      TEXT NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    picked_key   TEXT NOT NULL,        -- 'a'|'b'|'c'|'d', or '_' for pre-pick / on-demand
    model        TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    body         TEXT NOT NULL,
    generated_at INTEGER NOT NULL,
    PRIMARY KEY(card_id, picked_key)
);
