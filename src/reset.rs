//! `flashcards reset` — zero out FSRS state, optionally wipe the reviews log,
//! optionally scoped to a track / topic.

use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use std::io::{self, Write};

use crate::cli::{Cli, ResetScope};
use crate::{db, util};

pub fn run(
    cli: &Cli,
    scope: ResetScope,
    track: Option<&str>,
    topic: Option<&str>,
    yes: bool,
) -> Result<()> {
    let db_path = util::db_path(cli)?;
    let mut conn = db::open(&db_path)?;

    // Filter clause + params reused by both COUNT and the writes.
    let (where_clause, scope_label) = build_filter_label(track, topic);

    // Tally what will change.
    let card_count = count_matching_cards(&conn, &where_clause, track, topic)?;
    let review_count = count_matching_reviews(&conn, &where_clause, track, topic)?;
    let already_reviewed = count_already_reviewed_cards(&conn, &where_clause, track, topic)?;

    if card_count == 0 {
        println!("No cards match the filter ({}). Nothing to do.", scope_label);
        return Ok(());
    }

    println!("Reset target: {}", scope_label);
    println!(
        "  cards in scope:           {}",
        card_count
    );
    println!(
        "  cards with prior reviews: {}",
        already_reviewed
    );
    if scope == ResetScope::All {
        println!("  reviews log rows:         {}  (will be deleted)", review_count);
    } else {
        println!(
            "  reviews log rows:         {}  (kept; only card_state is reset)",
            review_count
        );
    }
    println!();

    if !yes && !confirm()? {
        println!("aborted.");
        return Ok(());
    }

    let now = Utc::now().timestamp();
    let tx = conn.transaction()?;

    // Reset card_state.
    let updated = match (track, topic) {
        (None, None) => tx.execute(
            "UPDATE card_state
             SET due=?1, stability=0, difficulty=0, elapsed_days=0, scheduled_days=0,
                 reps=0, lapses=0, state=0, last_review=NULL",
            params![now],
        )?,
        (Some(t), None) => tx.execute(
            "UPDATE card_state
             SET due=?1, stability=0, difficulty=0, elapsed_days=0, scheduled_days=0,
                 reps=0, lapses=0, state=0, last_review=NULL
             WHERE card_id IN (
                SELECT c.id FROM cards c
                JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
                WHERE k.track = ?2
             )",
            params![now, t],
        )?,
        (None, Some(tp)) => tx.execute(
            "UPDATE card_state
             SET due=?1, stability=0, difficulty=0, elapsed_days=0, scheduled_days=0,
                 reps=0, lapses=0, state=0, last_review=NULL
             WHERE card_id IN (
                SELECT c.id FROM cards c
                JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
                WHERE k.topic = ?2
             )",
            params![now, tp],
        )?,
        (Some(t), Some(tp)) => tx.execute(
            "UPDATE card_state
             SET due=?1, stability=0, difficulty=0, elapsed_days=0, scheduled_days=0,
                 reps=0, lapses=0, state=0, last_review=NULL
             WHERE card_id IN (
                SELECT c.id FROM cards c
                JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
                WHERE k.track = ?2 AND k.topic = ?3
             )",
            params![now, t, tp],
        )?,
    };

    let deleted_reviews = if scope == ResetScope::All {
        match (track, topic) {
            (None, None) => tx.execute("DELETE FROM reviews", [])?,
            (Some(t), None) => tx.execute(
                "DELETE FROM reviews WHERE card_id IN (
                    SELECT c.id FROM cards c
                    JOIN concepts k ON k.id = c.concept_id
                    WHERE k.track = ?1
                 )",
                params![t],
            )?,
            (None, Some(tp)) => tx.execute(
                "DELETE FROM reviews WHERE card_id IN (
                    SELECT c.id FROM cards c
                    JOIN concepts k ON k.id = c.concept_id
                    WHERE k.topic = ?1
                 )",
                params![tp],
            )?,
            (Some(t), Some(tp)) => tx.execute(
                "DELETE FROM reviews WHERE card_id IN (
                    SELECT c.id FROM cards c
                    JOIN concepts k ON k.id = c.concept_id
                    WHERE k.track = ?1 AND k.topic = ?2
                 )",
                params![t, tp],
            )?,
        }
    } else {
        0
    };

    tx.commit()?;

    println!(
        "OK. Reset {} card_state row{}{}.",
        updated,
        if updated == 1 { "" } else { "s" },
        if scope == ResetScope::All {
            format!(" and deleted {} review log entries", deleted_reviews)
        } else {
            String::new()
        }
    );
    Ok(())
}

fn build_filter_label(track: Option<&str>, topic: Option<&str>) -> (String, String) {
    match (track, topic) {
        (None, None) => ("".to_string(), "all cards".to_string()),
        (Some(t), None) => ("AND k.track = ?".to_string(), format!("track={}", t)),
        (None, Some(tp)) => ("AND k.topic = ?".to_string(), format!("topic={}", tp)),
        (Some(t), Some(tp)) => (
            "AND k.track = ? AND k.topic = ?".to_string(),
            format!("track={} AND topic={}", t, tp),
        ),
    }
}

fn count_matching_cards(
    conn: &Connection,
    _where_clause: &str,
    track: Option<&str>,
    topic: Option<&str>,
) -> Result<i64> {
    let n: i64 = match (track, topic) {
        (None, None) => conn.query_row(
            "SELECT COUNT(*) FROM cards c
             JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL",
            [],
            |r| r.get(0),
        )?,
        (Some(t), None) => conn.query_row(
            "SELECT COUNT(*) FROM cards c
             JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
             WHERE k.track = ?1",
            params![t],
            |r| r.get(0),
        )?,
        (None, Some(tp)) => conn.query_row(
            "SELECT COUNT(*) FROM cards c
             JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
             WHERE k.topic = ?1",
            params![tp],
            |r| r.get(0),
        )?,
        (Some(t), Some(tp)) => conn.query_row(
            "SELECT COUNT(*) FROM cards c
             JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
             WHERE k.track = ?1 AND k.topic = ?2",
            params![t, tp],
            |r| r.get(0),
        )?,
    };
    Ok(n)
}

fn count_already_reviewed_cards(
    conn: &Connection,
    _w: &str,
    track: Option<&str>,
    topic: Option<&str>,
) -> Result<i64> {
    let n: i64 = match (track, topic) {
        (None, None) => conn.query_row(
            "SELECT COUNT(*) FROM card_state s
             JOIN cards c ON c.id = s.card_id
             JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
             WHERE s.reps > 0",
            [],
            |r| r.get(0),
        )?,
        (Some(t), None) => conn.query_row(
            "SELECT COUNT(*) FROM card_state s
             JOIN cards c ON c.id = s.card_id
             JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
             WHERE s.reps > 0 AND k.track = ?1",
            params![t],
            |r| r.get(0),
        )?,
        (None, Some(tp)) => conn.query_row(
            "SELECT COUNT(*) FROM card_state s
             JOIN cards c ON c.id = s.card_id
             JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
             WHERE s.reps > 0 AND k.topic = ?1",
            params![tp],
            |r| r.get(0),
        )?,
        (Some(t), Some(tp)) => conn.query_row(
            "SELECT COUNT(*) FROM card_state s
             JOIN cards c ON c.id = s.card_id
             JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
             WHERE s.reps > 0 AND k.track = ?1 AND k.topic = ?2",
            params![t, tp],
            |r| r.get(0),
        )?,
    };
    Ok(n)
}

fn count_matching_reviews(
    conn: &Connection,
    _w: &str,
    track: Option<&str>,
    topic: Option<&str>,
) -> Result<i64> {
    let n: i64 = match (track, topic) {
        (None, None) => conn.query_row("SELECT COUNT(*) FROM reviews", [], |r| r.get(0))?,
        (Some(t), None) => conn.query_row(
            "SELECT COUNT(*) FROM reviews r
             JOIN cards c ON c.id = r.card_id
             JOIN concepts k ON k.id = c.concept_id
             WHERE k.track = ?1",
            params![t],
            |r| r.get(0),
        )?,
        (None, Some(tp)) => conn.query_row(
            "SELECT COUNT(*) FROM reviews r
             JOIN cards c ON c.id = r.card_id
             JOIN concepts k ON k.id = c.concept_id
             WHERE k.topic = ?1",
            params![tp],
            |r| r.get(0),
        )?,
        (Some(t), Some(tp)) => conn.query_row(
            "SELECT COUNT(*) FROM reviews r
             JOIN cards c ON c.id = r.card_id
             JOIN concepts k ON k.id = c.concept_id
             WHERE k.track = ?1 AND k.topic = ?2",
            params![t, tp],
            |r| r.get(0),
        )?,
    };
    Ok(n)
}

fn confirm() -> Result<bool> {
    print!("Type 'yes' to proceed (or anything else to abort): ");
    io::stdout().flush().context("flushing stdout")?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).context("reading stdin")?;
    Ok(buf.trim().eq_ignore_ascii_case("yes"))
}
