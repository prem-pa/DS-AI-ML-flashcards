use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use rusqlite::Connection;

use super::term::TermGuard;

pub fn run(conn: &Connection) -> Result<()> {
    run_with_term(conn, None)
}

pub fn run_with_term(conn: &Connection, held_term: Option<&mut TermGuard>) -> Result<()> {
    let snapshot = compute(conn)?;
    match held_term {
        Some(tg) => run_loop(tg, &snapshot),
        None => {
            let mut tg = TermGuard::enter()?;
            let r = run_loop(&mut tg, &snapshot);
            drop(tg);
            r
        }
    }
}

fn run_loop(tg: &mut TermGuard, snap: &Snapshot) -> Result<()> {
    loop {
        tg.term.draw(|f| draw(f, snap))?;
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if matches!(
                key.code,
                KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter
            ) {
                return Ok(());
            }
        }
    }
}

#[derive(Debug)]
struct Snapshot {
    total_cards: i64,
    suspended: i64,
    state_counts: [i64; 4], // new, learning, review, relearning
    due_now: i64,
    due_today: i64,
    due_week: i64,
    streak_days: i64,
    last_30_daily: Vec<(NaiveDate, i64)>, // 30 entries oldest..newest
    leeches: Vec<Leech>,
    weakest_topics: Vec<TopicScore>,
    review_count_total: i64,
    grade_dist: [i64; 4], // again/hard/good/easy
}

#[derive(Debug)]
struct Leech {
    title: String,
    front: String,
    lapses: i64,
}

#[derive(Debug)]
struct TopicScore {
    track: String,
    topic: String,
    n_cards: i64,
    avg_lapses: f64,
}

fn compute(conn: &Connection) -> Result<Snapshot> {
    let now = Utc::now();
    let now_ts = now.timestamp();

    let total_cards: i64 = conn.query_row(
        "SELECT COUNT(*) FROM cards WHERE suspended = 0",
        [],
        |r| r.get(0),
    )?;
    let suspended: i64 = conn.query_row(
        "SELECT COUNT(*) FROM cards WHERE suspended = 1",
        [],
        |r| r.get(0),
    )?;

    let mut state_counts = [0i64; 4];
    {
        let mut stmt = conn.prepare(
            "SELECT s.state, COUNT(*)
             FROM card_state s JOIN cards c ON c.id = s.card_id
             WHERE c.suspended = 0
             GROUP BY s.state",
        )?;
        let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?;
        for r in rows {
            let (st, n) = r?;
            if (0..4).contains(&st) {
                state_counts[st as usize] = n;
            }
        }
    }

    let due_now = count_due_in_range(conn, 0, now_ts)?;
    let end_today = end_of_day_ts(now);
    let due_today = count_due_in_range(conn, 0, end_today)?;
    let due_week = count_due_in_range(conn, 0, end_today + 6 * 86400)?;

    let last_30_daily = daily_review_counts(conn, now, 30)?;
    let streak_days = compute_streak(conn, now)?;

    let leeches = top_leeches(conn, 10)?;
    let weakest_topics = weakest_topics(conn, 10)?;

    let review_count_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM reviews",
        [],
        |r| r.get(0),
    )?;
    let mut grade_dist = [0i64; 4];
    {
        let mut stmt = conn.prepare(
            "SELECT rating, COUNT(*) FROM reviews GROUP BY rating",
        )?;
        let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?;
        for r in rows {
            let (g, n) = r?;
            if (1..=4).contains(&g) {
                grade_dist[(g - 1) as usize] = n;
            }
        }
    }

    Ok(Snapshot {
        total_cards,
        suspended,
        state_counts,
        due_now,
        due_today,
        due_week,
        streak_days,
        last_30_daily,
        leeches,
        weakest_topics,
        review_count_total,
        grade_dist,
    })
}

fn end_of_day_ts(now: DateTime<Utc>) -> i64 {
    let date = now.date_naive();
    let next = date + Duration::days(1);
    Utc.from_utc_datetime(&next.and_hms_opt(0, 0, 0).unwrap())
        .timestamp()
        - 1
}

fn count_due_in_range(conn: &Connection, lo: i64, hi: i64) -> Result<i64> {
    let n: i64 = conn.query_row(
        "SELECT COUNT(*) FROM card_state s
         JOIN cards c ON c.id = s.card_id
         JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
         WHERE c.suspended = 0 AND s.due BETWEEN ?1 AND ?2",
        rusqlite::params![lo, hi],
        |r| r.get(0),
    )?;
    Ok(n)
}

fn daily_review_counts(
    conn: &Connection,
    now: DateTime<Utc>,
    days: i64,
) -> Result<Vec<(NaiveDate, i64)>> {
    let today = now.date_naive();
    let oldest = today - Duration::days(days - 1);
    let oldest_ts = Utc
        .from_utc_datetime(&oldest.and_hms_opt(0, 0, 0).unwrap())
        .timestamp();
    let mut counts: std::collections::HashMap<NaiveDate, i64> = Default::default();
    let mut stmt = conn.prepare(
        "SELECT reviewed_at FROM reviews WHERE reviewed_at >= ?1",
    )?;
    let rows = stmt.query_map([oldest_ts], |r| r.get::<_, i64>(0))?;
    for r in rows {
        let ts = r?;
        if let Some(dt) = Utc.timestamp_opt(ts, 0).single() {
            *counts.entry(dt.date_naive()).or_default() += 1;
        }
    }
    let mut out = Vec::with_capacity(days as usize);
    for i in 0..days {
        let d = oldest + Duration::days(i);
        out.push((d, *counts.get(&d).unwrap_or(&0)));
    }
    Ok(out)
}

fn compute_streak(conn: &Connection, now: DateTime<Utc>) -> Result<i64> {
    let today = now.date_naive();
    let mut streak = 0i64;
    let mut day = today;
    loop {
        let start = Utc
            .from_utc_datetime(&day.and_hms_opt(0, 0, 0).unwrap())
            .timestamp();
        let end = start + 86400;
        let n: i64 = conn.query_row(
            "SELECT COUNT(*) FROM reviews WHERE reviewed_at >= ?1 AND reviewed_at < ?2",
            rusqlite::params![start, end],
            |r| r.get(0),
        )?;
        if n == 0 {
            // allow today to be empty (haven't reviewed yet today) without breaking streak.
            if streak == 0 && day == today {
                day -= Duration::days(1);
                continue;
            }
            break;
        }
        streak += 1;
        day -= Duration::days(1);
    }
    Ok(streak)
}

fn top_leeches(conn: &Connection, limit: usize) -> Result<Vec<Leech>> {
    let mut out = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT k.title, c.front, s.lapses
         FROM card_state s
         JOIN cards c ON c.id = s.card_id
         JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
         WHERE c.suspended = 0 AND s.lapses > 0
         ORDER BY s.lapses DESC
         LIMIT ?1",
    )?;
    let rows = stmt.query_map([limit as i64], |r| {
        Ok(Leech {
            title: r.get(0)?,
            front: r.get(1)?,
            lapses: r.get(2)?,
        })
    })?;
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

fn weakest_topics(conn: &Connection, limit: usize) -> Result<Vec<TopicScore>> {
    let mut out = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT k.track, k.topic, COUNT(*), AVG(s.lapses)
         FROM card_state s
         JOIN cards c ON c.id = s.card_id
         JOIN concepts k ON k.id = c.concept_id AND k.deleted_at IS NULL
         WHERE c.suspended = 0
         GROUP BY k.track, k.topic
         HAVING AVG(s.lapses) > 0
         ORDER BY AVG(s.lapses) DESC
         LIMIT ?1",
    )?;
    let rows = stmt.query_map([limit as i64], |r| {
        Ok(TopicScore {
            track: r.get(0)?,
            topic: r.get(1)?,
            n_cards: r.get(2)?,
            avg_lapses: r.get(3)?,
        })
    })?;
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

// ---------- drawing ----------

fn draw(f: &mut ratatui::Frame, snap: &Snapshot) {
    let area = f.area();
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(10), Constraint::Length(3)])
        .split(area);

    draw_top(f, outer[0], snap);
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[1]);
    draw_history(f, body[0], snap);
    draw_lists(f, body[1], snap);
    draw_footer(f, outer[2]);
}

fn draw_top(f: &mut ratatui::Frame, area: Rect, snap: &Snapshot) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(area);

    let summary = vec![
        Line::from(vec![
            Span::raw("active cards: "),
            Span::styled(
                snap.total_cards.to_string(),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("   suspended: {}", snap.suspended)),
        ]),
        Line::from(vec![
            Span::raw("new "),
            colored(snap.state_counts[0], Color::Blue),
            Span::raw("  learn "),
            colored(snap.state_counts[1], Color::Yellow),
            Span::raw("  review "),
            colored(snap.state_counts[2], Color::Green),
            Span::raw("  relearn "),
            colored(snap.state_counts[3], Color::Red),
        ]),
        Line::from(vec![
            Span::raw("total reviews: "),
            colored(snap.review_count_total, Color::Magenta),
        ]),
        Line::from(vec![
            Span::raw("grade dist: "),
            Span::styled(
                format!("again {} ", snap.grade_dist[0]),
                Style::default().fg(Color::Red),
            ),
            Span::styled(
                format!("hard {} ", snap.grade_dist[1]),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                format!("good {} ", snap.grade_dist[2]),
                Style::default().fg(Color::Green),
            ),
            Span::styled(
                format!("easy {}", snap.grade_dist[3]),
                Style::default().fg(Color::Cyan),
            ),
        ]),
    ];
    f.render_widget(
        Paragraph::new(Text::from(summary))
            .block(Block::default().borders(Borders::ALL).title("overview")),
        cols[0],
    );

    let due = vec![
        Line::from(vec![
            Span::raw("due now:    "),
            colored(snap.due_now, Color::Cyan),
        ]),
        Line::from(vec![
            Span::raw("due today:  "),
            colored(snap.due_today, Color::Cyan),
        ]),
        Line::from(vec![
            Span::raw("due 7d:     "),
            colored(snap.due_week, Color::Cyan),
        ]),
    ];
    f.render_widget(
        Paragraph::new(Text::from(due))
            .block(Block::default().borders(Borders::ALL).title("queue")),
        cols[1],
    );

    let streak = vec![
        Line::from(vec![Span::raw("streak: "), {
            Span::styled(
                format!("{} day{}", snap.streak_days, if snap.streak_days == 1 { "" } else { "s" }),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )
        }]),
        Line::raw(""),
        Line::raw(if snap.streak_days >= 7 { "🔥" } else { "" }.to_string()),
    ];
    f.render_widget(
        Paragraph::new(Text::from(streak))
            .block(Block::default().borders(Borders::ALL).title("streak")),
        cols[2],
    );
}

fn draw_history(f: &mut ratatui::Frame, area: Rect, snap: &Snapshot) {
    let max = snap.last_30_daily.iter().map(|(_, n)| *n).max().unwrap_or(0).max(1);
    let inner_w = area.width.saturating_sub(2) as usize;
    let label_w = 11usize; // "Mon Jan 01 "
    let bar_w = inner_w.saturating_sub(label_w + 6).max(8);

    let lines: Vec<Line> = snap
        .last_30_daily
        .iter()
        .rev() // newest first for readability
        .map(|(d, n)| {
            let bars = (((*n as f64) / (max as f64)) * (bar_w as f64)) as usize;
            let dow = d.weekday();
            let label = format!("{} {} ", short_weekday(dow.num_days_from_monday()), d.format("%b %d"));
            let bar: String = "█".repeat(bars);
            Line::from(vec![
                Span::styled(label, Style::default().fg(Color::DarkGray)),
                Span::styled(bar, Style::default().fg(Color::Green)),
                Span::raw(format!(" {}", n)),
            ])
        })
        .collect();

    f.render_widget(
        Paragraph::new(Text::from(lines))
            .block(Block::default().borders(Borders::ALL).title("last 30 days")),
        area,
    );
}

fn draw_lists(f: &mut ratatui::Frame, area: Rect, snap: &Snapshot) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let leech_lines: Vec<Line> = if snap.leeches.is_empty() {
        vec![Line::from(Span::styled(
            "No lapsed cards yet — keep grading.",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        snap.leeches
            .iter()
            .map(|l| {
                Line::from(vec![
                    Span::styled(format!("{:>3}× ", l.lapses), Style::default().fg(Color::Red)),
                    Span::styled(
                        truncate(&l.title, 30),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" — {}", truncate(&l.front, 60)),
                        Style::default().fg(Color::DarkGray),
                    ),
                ])
            })
            .collect()
    };
    f.render_widget(
        Paragraph::new(Text::from(leech_lines))
            .block(Block::default().borders(Borders::ALL).title("top leeches")),
        rows[0],
    );

    let topic_lines: Vec<Line> = if snap.weakest_topics.is_empty() {
        vec![Line::from(Span::styled(
            "Not enough lapse data yet.",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        snap.weakest_topics
            .iter()
            .map(|t| {
                Line::from(vec![
                    Span::styled(
                        format!("{:>4.1}  ", t.avg_lapses),
                        Style::default().fg(Color::Red),
                    ),
                    Span::styled(
                        format!("{} · {}", t.track, t.topic),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("  ({} cards)", t.n_cards),
                        Style::default().fg(Color::DarkGray),
                    ),
                ])
            })
            .collect()
    };
    f.render_widget(
        Paragraph::new(Text::from(topic_lines))
            .block(Block::default().borders(Borders::ALL).title("weakest topics (avg lapses)")),
        rows[1],
    );
}

fn draw_footer(f: &mut ratatui::Frame, area: Rect) {
    let p = Paragraph::new(Span::styled(
        "[q/esc/enter] back",
        Style::default().fg(Color::DarkGray),
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}

fn colored(n: i64, c: Color) -> Span<'static> {
    Span::styled(
        n.to_string(),
        Style::default().fg(c).add_modifier(Modifier::BOLD),
    )
}

fn short_weekday(n: u32) -> &'static str {
    match n {
        0 => "Mon", 1 => "Tue", 2 => "Wed", 3 => "Thu",
        4 => "Fri", 5 => "Sat", _ => "Sun",
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.replace('\n', " ")
    } else {
        let mut out: String = s.replace('\n', " ").chars().take(n - 1).collect();
        out.push('…');
        out
    }
}
