//! FSRS scheduler adapter.
//!
//! Translates between the on-disk `card_state` row and the in-memory
//! `rs_fsrs::Card`, applies a rating, and writes back. The `reviews` log is
//! append-only so that history survives card edits and concept renames.

use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use rs_fsrs::{Card as FsrsCard, Parameters, Rating, State, FSRS};
use rusqlite::{params, Connection, OptionalExtension};

#[derive(Debug, Clone, Copy)]
pub struct CardStateRow {
    pub due: i64,
    pub stability: f64,
    pub difficulty: f64,
    pub elapsed_days: f64,
    pub scheduled_days: f64,
    pub reps: i64,
    pub lapses: i64,
    pub state: i64,
    pub last_review: Option<i64>,
}

fn ts_to_dt(ts: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(ts, 0).single().unwrap_or_else(Utc::now)
}

fn state_from_int(i: i64) -> State {
    match i {
        1 => State::Learning,
        2 => State::Review,
        3 => State::Relearning,
        _ => State::New,
    }
}

fn state_to_int(s: State) -> i64 {
    match s {
        State::New => 0,
        State::Learning => 1,
        State::Review => 2,
        State::Relearning => 3,
    }
}

fn rating_from_u8(r: u8) -> Option<Rating> {
    match r {
        1 => Some(Rating::Again),
        2 => Some(Rating::Hard),
        3 => Some(Rating::Good),
        4 => Some(Rating::Easy),
        _ => None,
    }
}

fn row_to_fsrs(row: &CardStateRow) -> FsrsCard {
    FsrsCard {
        due: ts_to_dt(row.due),
        stability: row.stability,
        difficulty: row.difficulty,
        elapsed_days: row.elapsed_days as i64,
        scheduled_days: row.scheduled_days as i64,
        reps: row.reps as i32,
        lapses: row.lapses as i32,
        state: state_from_int(row.state),
        last_review: row
            .last_review
            .map(ts_to_dt)
            .unwrap_or_else(|| ts_to_dt(0)),
    }
}

pub fn fetch_state(conn: &Connection, card_id: &str) -> Result<Option<CardStateRow>> {
    let row = conn
        .query_row(
            "SELECT due, stability, difficulty, elapsed_days, scheduled_days,
                    reps, lapses, state, last_review
             FROM card_state WHERE card_id = ?1",
            params![card_id],
            |r| {
                Ok(CardStateRow {
                    due: r.get(0)?,
                    stability: r.get(1)?,
                    difficulty: r.get(2)?,
                    elapsed_days: r.get(3)?,
                    scheduled_days: r.get(4)?,
                    reps: r.get(5)?,
                    lapses: r.get(6)?,
                    state: r.get(7)?,
                    last_review: r.get(8)?,
                })
            },
        )
        .optional()?;
    Ok(row)
}

/// Apply `rating` (1..=4) to the given card; persist the new state and append
/// to `reviews`. Returns the new state for the UI to display.
pub fn review(conn: &mut Connection, card_id: &str, rating: u8) -> Result<CardStateRow> {
    let now = Utc::now();
    let now_ts = now.timestamp();

    let prev = fetch_state(conn, card_id)?
        .ok_or_else(|| anyhow::anyhow!("no card_state row for card {card_id} (run sync first)"))?;
    let rating = rating_from_u8(rating)
        .ok_or_else(|| anyhow::anyhow!("rating must be 1..=4 (got {rating})"))?;

    let fsrs = FSRS::new(Parameters::default());
    let info = fsrs.next(row_to_fsrs(&prev), now, rating);
    let card = info.card;

    let new_row = CardStateRow {
        due: card.due.timestamp(),
        stability: card.stability,
        difficulty: card.difficulty,
        elapsed_days: card.elapsed_days as f64,
        scheduled_days: card.scheduled_days as f64,
        reps: card.reps as i64,
        lapses: card.lapses as i64,
        state: state_to_int(card.state),
        last_review: Some(card.last_review.timestamp()),
    };

    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE card_state SET due=?2, stability=?3, difficulty=?4, elapsed_days=?5,
                                scheduled_days=?6, reps=?7, lapses=?8, state=?9, last_review=?10
         WHERE card_id=?1",
        params![
            card_id,
            new_row.due,
            new_row.stability,
            new_row.difficulty,
            new_row.elapsed_days,
            new_row.scheduled_days,
            new_row.reps,
            new_row.lapses,
            new_row.state,
            new_row.last_review,
        ],
    )?;
    tx.execute(
        "INSERT INTO reviews(card_id, rating, reviewed_at, elapsed_days, scheduled_days,
                              state_before, state_after)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            card_id,
            rating as i64,
            now_ts,
            new_row.elapsed_days,
            new_row.scheduled_days,
            prev.state,
            new_row.state,
        ],
    )?;
    tx.commit()?;
    Ok(new_row)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_row() -> CardStateRow {
        CardStateRow {
            due: 0,
            stability: 0.0,
            difficulty: 0.0,
            elapsed_days: 0.0,
            scheduled_days: 0.0,
            reps: 0,
            lapses: 0,
            state: 0,
            last_review: None,
        }
    }

    #[test]
    fn smoke_advances_state_through_a_good_review() {
        let now = Utc::now();
        let fsrs = FSRS::new(Parameters::default());
        let info = fsrs.next(row_to_fsrs(&fresh_row()), now, Rating::Good);
        assert!(info.card.due > now);
        assert_ne!(info.card.state, State::New); // moved off New
        assert!(info.card.stability >= 0.0);
    }

    #[test]
    fn rating_again_keeps_card_due_soon() {
        let now = Utc::now();
        let fsrs = FSRS::new(Parameters::default());
        let info = fsrs.next(row_to_fsrs(&fresh_row()), now, Rating::Again);
        // Rating::Again should schedule re-review within a couple of days.
        let delta = info.card.due.signed_duration_since(now);
        assert!(delta.num_days() <= 2);
    }
}
