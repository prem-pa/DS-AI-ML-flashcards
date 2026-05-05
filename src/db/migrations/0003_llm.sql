-- 0003_llm.sql
-- LLM-assisted explanation cache and per-card chat history.
-- Profile-level prefs (llm_enabled, llm_model, llm_endpoint) live in
-- profile_meta and don't need their own table.

CREATE TABLE IF NOT EXISTS card_explanations (
    card_id      TEXT PRIMARY KEY REFERENCES cards(id) ON DELETE CASCADE,
    model        TEXT NOT NULL,
    content_hash TEXT NOT NULL,    -- blake3(front + back + choices_json) at gen time
    body         TEXT NOT NULL,
    generated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS chat_messages (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    card_id   TEXT NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    role      TEXT NOT NULL CHECK(role IN ('system','user','assistant')),
    content   TEXT NOT NULL,
    sent_at   INTEGER NOT NULL,
    model     TEXT
);

CREATE INDEX IF NOT EXISTS idx_chat_messages_card ON chat_messages(card_id, id);
