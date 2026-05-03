use anyhow::Result;
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rusqlite::Connection;

use crate::db::{self, CardView, ConceptMeta, Scope};
use crate::render;
use crate::sched;

use super::term::TermGuard;

const QUEUE_LIMIT: usize = 200;

pub fn run(conn: &mut Connection, cli: &crate::cli::Cli) -> Result<()> {
    run_with_scope(conn, cli, Scope::default(), None)
}

/// Run a review pass with an optional scope filter. If a `TermGuard` is already
/// held by the caller (i.e. we were launched from the main menu), they should
/// pass it via `run_in_term`; otherwise this entrypoint creates its own.
pub fn run_with_scope(
    conn: &mut Connection,
    cli: &crate::cli::Cli,
    scope: Scope,
    held_term: Option<&mut TermGuard>,
) -> Result<()> {
    let now = Utc::now().timestamp();
    let queue = db::fetch_due_scoped(conn, now, &scope, QUEUE_LIMIT)?;
    if queue.is_empty() {
        let total = db::count_total_scoped(conn, &scope)?;
        let due_now = db::count_due_scoped(conn, now, &scope)?;
        println!(
            "Nothing due in this scope ({}). {} cards total, {} due now.",
            scope.label(),
            total,
            due_now
        );
        return Ok(());
    }

    let session_id = db::open_session(conn, &scope)?;

    let res = match held_term {
        Some(tg) => run_loop(tg, conn, cli, queue, session_id),
        None => {
            let mut tg = TermGuard::enter()?;
            let r = run_loop(&mut tg, conn, cli, queue, session_id);
            drop(tg);
            r
        }
    };

    let _ = db::close_session(conn, session_id);
    res
}

#[derive(Debug)]
struct State {
    cards: Vec<CardView>,
    idx: usize,
    revealed: bool,
    mcq_pick: Option<usize>,
    last_grade: Option<u8>,
    reviewed: usize,
    session_id: i64,
    meta_for: Option<String>, // concept_id whose meta is currently cached
    meta: ConceptMeta,
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
    session_id: i64,
) -> Result<()> {
    let mut state = State {
        cards,
        idx: 0,
        revealed: false,
        mcq_pick: None,
        last_grade: None,
        reviewed: 0,
        session_id,
        meta_for: None,
        meta: ConceptMeta::default(),
    };
    refresh_meta(&mut state, conn);

    loop {
        refresh_meta(&mut state, conn);
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

fn refresh_meta(state: &mut State, conn: &rusqlite::Connection) {
    let cur = match state.current() {
        Some(c) => c,
        None => return,
    };
    if state.meta_for.as_deref() == Some(cur.concept_id.as_str()) {
        return;
    }
    let cid = cur.concept_id.clone();
    state.meta = db::fetch_concept_meta(conn, &cid).unwrap_or_default();
    state.meta_for = Some(cid);
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
    let _ = db::touch_session(conn, state.session_id, &card.id, state.reviewed as i64);
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
            let _ = db::touch_session(conn, state.session_id, &card.id, state.reviewed as i64);
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
    let show_meta = state.revealed || state.mcq_pick.is_some();
    let meta_h: u16 = if show_meta { meta_panel_height(card, &state.meta) } else { 0 };
    let mut constraints = vec![Constraint::Length(3), Constraint::Min(5)];
    if show_meta {
        constraints.push(Constraint::Length(meta_h));
    }
    constraints.push(Constraint::Length(3));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
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

    let footer_idx = if show_meta {
        let meta_widget = render_meta_panel(card, &state.meta);
        f.render_widget(meta_widget, chunks[2]);
        3
    } else {
        2
    };

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
    f.render_widget(footer, chunks[footer_idx]);
}

fn meta_panel_height(_card: &CardView, meta: &ConceptMeta) -> u16 {
    // 1 stat line + sources block + see-also block + tags block, each
    // with a header + content + spacer; bound 4..=14.
    let mut h = 3u16; // borders + stats line
    if !meta.sources.is_empty() {
        h += (meta.sources.len() as u16).min(4) + 1;
    }
    if !meta.see_also.is_empty() {
        h += 2;
    }
    if !meta.tags.is_empty() {
        h += 1;
    }
    h.clamp(4, 14)
}

fn render_meta_panel<'a>(card: &'a CardView, meta: &'a ConceptMeta) -> Paragraph<'a> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    // top stat line
    let mut top: Vec<Span<'static>> = Vec::new();
    top.push(Span::styled(
        format!("difficulty {}", card.difficulty),
        Style::default().fg(Color::DarkGray),
    ));
    top.push(Span::raw("    "));
    top.push(Span::styled(
        state_label(card.state),
        Style::default().fg(state_color(card.state)),
    ));
    top.push(Span::raw("    "));
    top.push(Span::styled(
        format!("reps {}  lapses {}", card.reps, card.lapses),
        Style::default().fg(Color::DarkGray),
    ));
    if let Some(ts) = card.last_review {
        top.push(Span::raw("    "));
        top.push(Span::styled(
            format!("last review {}", relative_time(ts)),
            Style::default().fg(Color::DarkGray),
        ));
    }
    lines.push(Line::from(top));

    if !meta.sources.is_empty() {
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "sources",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )));
        for (i, (url, label)) in meta.sources.iter().take(4).enumerate() {
            let label_text = label.as_deref().unwrap_or("");
            lines.push(Line::from(vec![
                Span::styled(format!("  {}) ", i + 1), Style::default().fg(Color::DarkGray)),
                Span::styled(
                    label_text.to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw("  "),
                Span::styled(
                    url.clone(),
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::UNDERLINED),
                ),
            ]));
        }
    }

    if !meta.see_also.is_empty() {
        lines.push(Line::from(vec![
            Span::styled(
                "see also: ",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                meta.see_also.join(" · "),
                Style::default().fg(Color::Cyan),
            ),
        ]));
    }

    if !meta.tags.is_empty() {
        lines.push(Line::from(vec![
            Span::styled(
                "tags: ",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                meta.tags.join(", "),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title("context"))
}

fn state_label(state: i64) -> String {
    match state {
        0 => "new".into(),
        1 => "learning".into(),
        2 => "review".into(),
        3 => "relearning".into(),
        _ => format!("state {}", state),
    }
}

fn state_color(state: i64) -> Color {
    match state {
        0 => Color::Blue,
        1 => Color::Yellow,
        2 => Color::Green,
        3 => Color::Red,
        _ => Color::Gray,
    }
}

fn relative_time(ts: i64) -> String {
    use chrono::Utc;
    let now = Utc::now().timestamp();
    let d = (now - ts).max(0);
    if d < 60 {
        "just now".into()
    } else if d < 3600 {
        format!("{}m ago", d / 60)
    } else if d < 86400 {
        format!("{}h ago", d / 3600)
    } else if d < 86400 * 30 {
        format!("{}d ago", d / 86400)
    } else {
        format!("{}mo ago", d / (86400 * 30))
    }
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
