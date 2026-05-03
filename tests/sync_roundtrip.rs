//! End-to-end test: editing a card's front text in the vault must NOT reset
//! its FSRS scheduling state. The card UUID is the join key.

use rusqlite::Connection;
use std::fs;
use std::path::Path;

use flashcards::db;
use flashcards::vault::sync::sync_vault;

const NOTE_BEFORE: &str = "---
id: 11111111-1111-1111-1111-111111111111
title: Bias-Variance Tradeoff
track: data-scientist
topic: foundational-statistics-probability
difficulty: 2
tags:
- stats
aliases: []
sources: []
cards:
- id: 22222222-2222-2222-2222-222222222222
  type: flip
  front: What is bias?
  back: Systematic error.
- id: 33333333-3333-3333-3333-333333333333
  type: mcq
  front: Pick the best.
  back: Because reasons.
  choices:
  - key: a
    text: First
    correct: false
  - key: b
    text: Second
    correct: true
---

# Body

See [[Variance]].
";

const NOTE_AFTER: &str = "---
id: 11111111-1111-1111-1111-111111111111
title: Bias-Variance Tradeoff
track: data-scientist
topic: foundational-statistics-probability
difficulty: 2
tags:
- stats
aliases: []
sources: []
cards:
- id: 22222222-2222-2222-2222-222222222222
  type: flip
  front: WHAT IS BIAS (rephrased)?
  back: Systematic error in the estimator.
- id: 33333333-3333-3333-3333-333333333333
  type: mcq
  front: Pick the best.
  back: Because reasons.
  choices:
  - key: a
    text: First
    correct: false
  - key: b
    text: Second
    correct: true
---

# Body

See [[Variance]].
";

fn write_note(vault: &Path, content: &str) {
    let dir = vault.join("data-scientist/foundational-statistics-probability");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("bias-variance.md"), content).unwrap();
}

fn count(conn: &Connection, sql: &str) -> i64 {
    conn.query_row(sql, [], |r| r.get(0)).unwrap()
}

#[test]
fn sync_roundtrip_preserves_card_state() {
    let tmp = tempfile::tempdir().unwrap();
    let vault = tmp.path().join("vault");
    let db_path = tmp.path().join("flashcards.db");
    write_note(&vault, NOTE_BEFORE);

    // First sync: 1 concept, 2 cards, 2 card_state rows.
    let mut conn = db::open(&db_path).unwrap();
    let r = sync_vault(&mut conn, &vault).unwrap();
    assert_eq!(r.new_concepts, 1);
    assert_eq!(count(&conn, "SELECT COUNT(*) FROM concepts"), 1);
    assert_eq!(count(&conn, "SELECT COUNT(*) FROM cards"), 2);
    assert_eq!(count(&conn, "SELECT COUNT(*) FROM card_state"), 2);

    // Mutate scheduling state directly.
    conn.execute(
        "UPDATE card_state SET due=999999, reps=42 WHERE card_id=?1",
        rusqlite::params!["22222222-2222-2222-2222-222222222222"],
    )
    .unwrap();

    // Edit the note: change front + back text.
    write_note(&vault, NOTE_AFTER);

    // Second sync: must update card text but preserve FSRS state.
    let r2 = sync_vault(&mut conn, &vault).unwrap();
    assert_eq!(r2.new_concepts, 0);
    assert_eq!(r2.changed, 1);

    // Card text reflects the edit.
    let new_front: String = conn
        .query_row(
            "SELECT front FROM cards WHERE id=?1",
            rusqlite::params!["22222222-2222-2222-2222-222222222222"],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(new_front, "WHAT IS BIAS (rephrased)?");

    // Critically: card_state survived.
    let (due, reps): (i64, i64) = conn
        .query_row(
            "SELECT due, reps FROM card_state WHERE card_id=?1",
            rusqlite::params!["22222222-2222-2222-2222-222222222222"],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(due, 999999, "due was reset by sync — bug!");
    assert_eq!(reps, 42, "reps was reset by sync — bug!");
}

const NOTE_DROPPED_MCQ: &str = "---
id: 11111111-1111-1111-1111-111111111111
title: Bias-Variance Tradeoff
track: data-scientist
topic: foundational-statistics-probability
difficulty: 2
tags:
- stats
aliases: []
sources: []
cards:
- id: 22222222-2222-2222-2222-222222222222
  type: flip
  front: What is bias?
  back: Systematic error.
---

# Body
";

#[test]
fn sync_suspends_cards_removed_from_a_note() {
    let tmp = tempfile::tempdir().unwrap();
    let vault = tmp.path().join("vault");
    let db_path = tmp.path().join("flashcards.db");
    write_note(&vault, NOTE_BEFORE);
    let mut conn = db::open(&db_path).unwrap();
    sync_vault(&mut conn, &vault).unwrap();
    assert_eq!(count(&conn, "SELECT COUNT(*) FROM cards"), 2);

    // Drop the mcq card from the note.
    write_note(&vault, NOTE_DROPPED_MCQ);
    let r = sync_vault(&mut conn, &vault).unwrap();
    assert_eq!(r.suspended_cards, 1);

    let suspended: i64 = conn
        .query_row(
            "SELECT suspended FROM cards WHERE id=?1",
            rusqlite::params!["33333333-3333-3333-3333-333333333333"],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(suspended, 1);
}
