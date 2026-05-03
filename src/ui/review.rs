use anyhow::Result;
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rusqlite::Connection;

use crate::db::{self, CardView};
use crate::render;
use crate::sched;

use super::term::TermGuard;

const QUEUE_LIMIT: usize = 200;

pub fn run(conn: &mut Connection, cli: &crate::cli::Cli) -> Result<()> {
    let now = Utc::now().timestamp();
    let queue = db::fetch_due(conn, now, QUEUE_LIMIT)?;
    if queue.is_empty() {
        println!("Nothing due. ({} cards in the future queue)", count_total(conn)?);
        return Ok(());
    }
    let mut tg = TermGuard::enter()?;
    let res = run_loop(&mut tg, conn, cli, queue);
    drop(tg);
    res
}

fn count_total(conn: &Connection) -> Result<i64> {
    let n: i64 = conn.query_row(
        "SELECT COUNT(*) FROM cards WHERE suspended = 0",
        [],
        |r| r.get(0),
    )?;
    Ok(n)
}

#[derive(Debug)]
struct State {
    cards: Vec<CardView>,
    idx: usize,
    revealed: bool,
    mcq_pick: Option<usize>,
    last_grade: Option<u8>,
    reviewed: usize,
}

impl State {
    fn current(&self) -> Option<&CardView> {
        self.cards.get(self.idx)
    }
    fn advance(&mut self) {
        self.idx += 1;
        self.revealed = false;
        self.mcq_pick = None;
        self.last_grade = None;
    }
}

fn run_loop(
    tg: &mut TermGuard,
    conn: &mut Connection,
    cli: &crate::cli::Cli,
    cards: Vec<CardView>,
) -> Result<()> {
    let mut state = State {
        cards,
        idx: 0,
        revealed: false,
        mcq_pick: None,
        last_grade: None,
        reviewed: 0,
    };

    loop {
        if state.idx >= state.cards.len() {
            tg.term.draw(|f| draw_done(f, &state))?;
        } else {
            tg.term.draw(|f| draw(f, &state))?;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if state.idx >= state.cards.len() {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter) {
                    return Ok(());
                }
                continue;
            }
            // 'e' = edit current concept's .md in $EDITOR, then re-sync.
            if matches!(key.code, KeyCode::Char('e')) {
                if let Some(card) = state.current().cloned() {
                    super::edit::edit_concept(tg, conn, cli, &card.concept_id)?;
                    // Refresh current card view so renames/edits show after returning.
                    if let Some(updated) = db::fetch_one(conn, &card.id)? {
                        state.cards[state.idx] = updated;
                    }
                }
                continue;
            }
            match handle(key, &mut state, conn)? {
                Loop::Continue => {}
                Loop::Quit => return Ok(()),
            }
        }
    }
}

enum Loop {
    Continue,
    Quit,
}

fn handle(key: KeyEvent, state: &mut State, conn: &mut Connection) -> Result<Loop> {
    let Some(card) = state.current().cloned() else {
        return Ok(Loop::Quit);
    };

    if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
        return Ok(Loop::Quit);
    }

    match card.kind.as_str() {
        "flip" => handle_flip(key, state, conn, &card),
        "mcq" => handle_mcq(key, state, conn, &card),
        _ => {
            state.advance();
            Ok(Loop::Continue)
        }
    }
}

fn handle_flip(
    key: KeyEvent,
    state: &mut State,
    conn: &mut Connection,
    card: &CardView,
) -> Result<Loop> {
    if !state.revealed {
        if matches!(
            key.code,
            KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Char('f')
        ) {
            state.revealed = true;
        }
        return Ok(Loop::Continue);
    }
    let grade = match key.code {
        KeyCode::Char('1') => 1,
        KeyCode::Char('2') => 2,
        KeyCode::Char('3') | KeyCode::Char(' ') | KeyCode::Enter => 3,
        KeyCode::Char('4') => 4,
        _ => return Ok(Loop::Continue),
    };
    sched::review(conn, &card.id, grade)?;
    state.reviewed += 1;
    state.last_grade = Some(grade);
    state.advance();
    Ok(Loop::Continue)
}

fn handle_mcq(
    key: KeyEvent,
    state: &mut State,
    conn: &mut Connection,
    card: &CardView,
) -> Result<Loop> {
    // Pick stage
    if state.mcq_pick.is_none() {
        let pick = match key.code {
            KeyCode::Char(c) if c.is_ascii_alphabetic() => {
                let lc = c.to_ascii_lowercase();
                card.choices.iter().position(|ch| ch.key.to_ascii_lowercase() == lc.to_string())
            }
            _ => None,
        };
        if let Some(idx) = pick {
            state.mcq_pick = Some(idx);
            state.revealed = true;
            // Auto-grade: correct => 3 (Good), wrong => 1 (Again).
            let correct = card.choices.get(idx).map(|c| c.correct).unwrap_or(false);
            let grade = if correct { 3 } else { 1 };
            sched::review(conn, &card.id, grade)?;
            state.reviewed += 1;
            state.last_grade = Some(grade);
        }
        return Ok(Loop::Continue);
    }

    // Reveal stage — any forward key advances.
    if matches!(
        key.code,
        KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Char('n')
    ) {
        state.advance();
    }
    Ok(Loop::Continue)
}

// ---------- drawing ----------

fn draw(f: &mut ratatui::Frame, state: &State) {
    let card = match state.current() {
        Some(c) => c,
        None => return,
    };
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    let header_left = format!("{} / {}", state.idx + 1, state.cards.len());
    let header_mid = format!("{} · {}", card.track, card.topic);
    let header_right = format!("✓ {}", state.reviewed);
    let header = Paragraph::new(Line::from(vec![
        Span::styled(header_left, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("    "),
        Span::styled(header_mid, Style::default().fg(Color::DarkGray)),
        Span::raw("    "),
        Span::styled(header_right, Style::default().fg(Color::Green)),
    ]))
    .block(Block::default().borders(Borders::ALL).title(card.concept_title.clone()));
    f.render_widget(header, chunks[0]);

    let body: Text = match card.kind.as_str() {
        "flip" => {
            let mut lines: Vec<Line<'static>> = Vec::new();
            if state.revealed {
                lines.push(Line::from(Span::styled(
                    "Q:",
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
                )));
            }
            for l in render::render(&card.front).lines {
                lines.push(l);
            }
            if state.revealed {
                lines.push(Line::raw(""));
                lines.push(Line::from(Span::styled(
                    "A:",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                )));
                for l in render::render(&card.back).lines {
                    lines.push(l);
                }
            }
            Text::from(lines)
        }
        "mcq" => {
            let mut lines: Vec<Line<'static>> = Vec::new();
            for l in render::render(&card.front).lines {
                lines.push(l);
            }
            lines.push(Line::raw(""));
            for (i, ch) in card.choices.iter().enumerate() {
                let prefix = format!(" {})  ", ch.key);
                let mut style = Style::default();
                let mut marker = "";
                if let Some(picked) = state.mcq_pick {
                    if picked == i {
                        if ch.correct {
                            style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
                            marker = "  ✔ correct";
                        } else {
                            style = style.fg(Color::Red).add_modifier(Modifier::BOLD);
                            marker = "  ✗ your pick";
                        }
                    } else if ch.correct {
                        style = style.fg(Color::Green);
                        marker = "  ← correct";
                    }
                }
                lines.push(Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::DarkGray)),
                    Span::styled(render::prettify_math(&ch.text), style),
                    Span::styled(marker.to_string(), Style::default().fg(Color::DarkGray)),
                ]));
            }
            if state.mcq_pick.is_some() && !card.back.trim().is_empty() {
                lines.push(Line::raw(""));
                lines.push(Line::from(Span::styled(
                    "Why:",
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
                )));
                for l in render::render(&card.back).lines {
                    lines.push(l);
                }
            }
            Text::from(lines)
        }
        _ => Text::raw("(unknown card type)"),
    };
    let body_widget = Paragraph::new(body)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(body_widget, chunks[1]);

    let hint = match card.kind.as_str() {
        "flip" if !state.revealed => "[space] reveal   [e] edit in $EDITOR   [q] quit",
        "flip" => "[1] again  [2] hard  [3] good  [4] easy   [e] edit   [q] quit",
        "mcq" if state.mcq_pick.is_none() => "[a/b/c/d] pick   [e] edit   [q] quit",
        "mcq" => "[space] next   [e] edit   [q] quit",
        _ => "[q] quit",
    };
    let footer = Paragraph::new(Span::styled(
        hint,
        Style::default().fg(Color::DarkGray),
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn draw_done(f: &mut ratatui::Frame, state: &State) {
    let area = f.area();
    let msg = format!(
        "Done. Reviewed {} card{}.\n\nPress any key to exit.",
        state.reviewed,
        if state.reviewed == 1 { "" } else { "s" }
    );
    let widget = Paragraph::new(msg)
        .block(Block::default().borders(Borders::ALL).title("review"));
    f.render_widget(widget, area);
}
