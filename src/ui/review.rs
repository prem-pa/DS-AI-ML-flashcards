use anyhow::Result;
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rusqlite::Connection;

use crate::db::{self, CardView, ConceptMeta, Scope};
use crate::llm;
use crate::render;
use crate::sched;
use crate::util;
use std::time::Duration;

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
        let msg = format!(
            "Nothing due in this scope ({}).\n\n{} cards total, {} due now.",
            scope.label(),
            total,
            due_now
        );
        match held_term {
            Some(tg) => show_notice(tg, "no due cards", &msg)?,
            None => println!("{msg}"),
        }
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

fn show_notice(tg: &mut TermGuard, title: &str, body: &str) -> Result<()> {
    use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
    loop {
        tg.term.draw(|f| {
            let area = f.area();
            // center a small modal-ish region
            let text = ratatui::text::Text::from(format!("{body}\n\npress any key to go back."));
            let widget = Paragraph::new(text)
                .wrap(Wrap { trim: false })
                .block(Block::default().borders(Borders::ALL).title(title.to_string()));
            f.render_widget(widget, area);
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(());
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LlmMode {
    Explain,
    Hint,
}

#[derive(Debug, Clone)]
enum FetchMode {
    Explain(Option<String>), // None = pre-pick agnostic, Some = post-pick aware
    Hint,
}

struct LlmState {
    card_id: String,
    picked_key: String, // PRE_PICK_KEY / HINT_KEY / actual choice key
    mode: LlmMode,
    body: String,
    status: super::llm_panel::Status,
    error: Option<String>,
    handle: Option<llm::StreamHandle>,
    model: String,
    content_hash: String,
}

struct State {
    cards: Vec<CardView>,
    idx: usize,
    revealed: bool,
    mcq_pick: Option<usize>,
    /// True when `?` revealed the answer pre-pick — grades the card as Again
    /// and locks any further pick on this card.
    answered_via_llm: bool,
    last_grade: Option<u8>,
    reviewed: usize,
    session_id: i64,
    meta_for: Option<String>,
    meta: ConceptMeta,
    llm: Option<LlmState>,
    llm_enabled: bool,
    llm_model: String,
    llm_endpoint: String,
    vault_root: std::path::PathBuf,
}

impl State {
    fn current(&self) -> Option<&CardView> {
        self.cards.get(self.idx)
    }
    fn advance(&mut self) {
        self.idx += 1;
        self.reset_card_state();
    }
    fn go_back(&mut self) {
        if self.idx > 0 {
            self.idx -= 1;
        }
        self.reset_card_state();
    }
    fn reset_card_state(&mut self) {
        self.revealed = false;
        self.mcq_pick = None;
        self.answered_via_llm = false;
        self.last_grade = None;
        self.llm = None;
    }
}

fn run_loop(
    tg: &mut TermGuard,
    conn: &mut Connection,
    cli: &crate::cli::Cli,
    cards: Vec<CardView>,
    session_id: i64,
) -> Result<()> {
    let prefs = db::load_prefs(conn)?;
    let vault_root = util::vault_path(cli)?;
    let mut state = State {
        cards,
        idx: 0,
        revealed: false,
        mcq_pick: None,
        answered_via_llm: false,
        last_grade: None,
        reviewed: 0,
        session_id,
        meta_for: None,
        meta: ConceptMeta::default(),
        llm: None,
        llm_enabled: prefs.llm_enabled,
        llm_model: db::effective_model(&prefs),
        llm_endpoint: db::effective_endpoint(&prefs),
        vault_root,
    };
    refresh_meta(&mut state, conn);

    loop {
        refresh_meta(&mut state, conn);
        // pump streaming tokens (non-blocking)
        drain_llm(&mut state, conn);

        if state.idx >= state.cards.len() {
            tg.term.draw(|f| draw_done(f, &state))?;
        } else {
            tg.term.draw(|f| draw(f, &state))?;
        }

        // Poll for input briefly so streamed tokens can flow between keypresses.
        if !event::poll(Duration::from_millis(80))? {
            continue;
        }
        let evt = event::read()?;
        let Event::Key(key) = evt else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        if state.idx >= state.cards.len() {
            if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter) {
                return Ok(());
            }
            continue;
        }
        // 'e' = edit-in-vault, but a handful of MCQs use 'e' as a choice key.
        // Only intercept when the current card isn't an MCQ awaiting a pick.
        if matches!(key.code, KeyCode::Char('e')) {
            let should_edit = match state.current() {
                Some(c) if c.kind == "mcq" && state.mcq_pick.is_none() => false,
                Some(_) => true,
                None => true,
            };
            if should_edit {
                if let Some(card) = state.current().cloned() {
                    super::edit::edit_concept(tg, conn, cli, &card.concept_id)?;
                    if let Some(updated) = db::fetch_one(conn, &card.id)? {
                        state.cards[state.idx] = updated;
                    }
                    state.llm = None;
                }
                continue;
            }
        }
        // '?' = on-demand LLM explanation for the current MCQ.
        //   - pre-pick: counts as "I gave up". Grade the card as Again (1) and
        //     lock any further pick. Then fire the (option-agnostic) LLM call.
        //   - post-pick: pure on-demand re-fetch with the existing pick context.
        if matches!(key.code, KeyCode::Char('?')) {
            if state.llm_enabled {
                if let Some(card) = state.current().cloned() {
                    if card.kind == "mcq" {
                        let picked = state
                            .mcq_pick
                            .and_then(|i| card.choices.get(i).map(|c| c.key.clone()));
                        let pre_pick = state.mcq_pick.is_none() && !state.answered_via_llm;
                        if pre_pick {
                            sched::review(conn, &card.id, 1)?;
                            state.reviewed += 1;
                            state.last_grade = Some(1);
                            state.answered_via_llm = true;
                            state.revealed = true;
                            let _ = db::touch_session(
                                conn,
                                state.session_id,
                                &card.id,
                                state.reviewed as i64,
                            );
                        }
                        trigger_llm(&mut state, conn, FetchMode::Explain(picked));
                    }
                }
            }
            continue;
        }
        // 'n' / 'p' = global navigation, no grading. Useful for skimming.
        if matches!(key.code, KeyCode::Char('n')) {
            state.advance();
            continue;
        }
        if matches!(key.code, KeyCode::Char('p')) {
            state.go_back();
            continue;
        }
        // 'h' = hint (no spoilers). Works on flip cards and on MCQs.
        if matches!(key.code, KeyCode::Char('h')) && state.llm_enabled {
            trigger_llm(&mut state, conn, FetchMode::Hint);
            continue;
        }
        // 'c' = chat. Always available on flip cards. On MCQs it conflicts
        // with picking option C, so we only intercept *after* the user has
        // picked.
        if matches!(key.code, KeyCode::Char('c')) && state.llm_enabled {
            if let Some(card) = state.current().cloned() {
                let allow_chat = card.kind == "flip"
                    || (card.kind == "mcq" && state.mcq_pick.is_some());
                if allow_chat {
                    let pick = build_pick_context(&card, state.mcq_pick);
                    super::chat::run(
                        tg,
                        conn,
                        &card,
                        &state.vault_root,
                        &state.llm_model,
                        &state.llm_endpoint,
                        pick,
                    )?;
                    let _ = tg.term.clear();
                    continue;
                }
            }
        }
        match handle(key, &mut state, conn)? {
            Loop::Continue => {}
            Loop::Quit => return Ok(()),
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

/// Build a pick-context snapshot for chat seeding. Returns None when the
/// card isn't an MCQ or no pick has been made yet.
fn build_pick_context(
    card: &CardView,
    pick_idx: Option<usize>,
) -> Option<llm::prompt::PickContext> {
    if card.kind != "mcq" {
        return None;
    }
    let i = pick_idx?;
    let picked = card.choices.get(i)?;
    let correct = card.choices.iter().find(|c| c.correct)?;
    Some(llm::prompt::PickContext {
        picked_key: picked.key.clone(),
        picked_text: picked.text.clone(),
        correct_key: correct.key.clone(),
        correct_text: correct.text.clone(),
        was_correct: picked.correct,
    })
}

/// Trigger an LLM call for the current card. `mode` chooses between an
/// explanation (post-pick or pre-pick) and a hint (no spoilers). Hints work
/// on flip cards too; explanations are MCQ-only since flip cards already have
/// chat for follow-ups.
fn trigger_llm(state: &mut State, conn: &rusqlite::Connection, mode: FetchMode) {
    if !state.llm_enabled {
        return;
    }
    let card = match state.current() {
        Some(c) => c.clone(),
        None => return,
    };
    let panel_mode = match &mode {
        FetchMode::Explain(_) => LlmMode::Explain,
        FetchMode::Hint => LlmMode::Hint,
    };
    if matches!(mode, FetchMode::Explain(_)) && card.kind != "mcq" {
        return; // explanations only for MCQs
    }
    let pick_slot = match &mode {
        FetchMode::Explain(None) => db::PRE_PICK_KEY.to_string(),
        FetchMode::Explain(Some(k)) => k.to_ascii_lowercase(),
        FetchMode::Hint => db::HINT_KEY.to_string(),
    };
    let model = state.llm_model.clone();
    let content_hash = db::card_content_hash(&card);

    if let Ok(Some(cached)) =
        db::fetch_cached_explanation(conn, &card.id, &pick_slot, &model, &content_hash)
    {
        state.llm = Some(LlmState {
            card_id: card.id.clone(),
            picked_key: pick_slot,
            mode: panel_mode,
            body: cached.body,
            status: super::llm_panel::Status::Cached,
            error: None,
            handle: None,
            model,
            content_hash,
        });
        return;
    }

    let bundle = match llm::context::build(conn, &card, &state.vault_root) {
        Ok(b) => b,
        Err(e) => {
            state.llm = Some(LlmState {
                card_id: card.id.clone(),
                picked_key: pick_slot,
                mode: panel_mode,
                body: String::new(),
                status: super::llm_panel::Status::Error,
                error: Some(format!("context: {e:#}")),
                handle: None,
                model,
                content_hash,
            });
            return;
        }
    };
    let prompt = match &mode {
        FetchMode::Explain(picked) => llm::prompt::build_mcq_prompt(&bundle, picked.as_deref()),
        FetchMode::Hint => llm::prompt::build_hint_prompt(&bundle),
    };
    let handle = llm::client::generate_stream(&state.llm_endpoint, &model, prompt);
    state.llm = Some(LlmState {
        card_id: card.id.clone(),
        picked_key: pick_slot,
        mode: panel_mode,
        body: String::new(),
        status: super::llm_panel::Status::Streaming,
        error: None,
        handle: Some(handle),
        model,
        content_hash,
    });
}

/// Drain pending streamed tokens into the buffer, persisting to cache on Done.
fn drain_llm(state: &mut State, conn: &rusqlite::Connection) {
    let Some(s) = state.llm.as_mut() else { return };
    let Some(h) = s.handle.as_mut() else { return };
    let (tokens, terminal) = h.poll();
    for t in tokens {
        s.body.push_str(&t);
    }
    if let Some(ev) = terminal {
        match ev {
            llm::Event::Done(final_body) => {
                if !final_body.is_empty() {
                    s.body = final_body;
                }
                s.status = super::llm_panel::Status::Done;
                s.handle = None;
                let _ = db::upsert_explanation(
                    conn,
                    &s.card_id,
                    &s.picked_key,
                    &s.model,
                    &s.content_hash,
                    &s.body,
                );
            }
            llm::Event::Error(msg) => {
                s.status = super::llm_panel::Status::Error;
                s.error = Some(msg);
                s.handle = None;
            }
            _ => {}
        }
    }
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
    // Pick stage. Locked once the user picks OR uses `?` to reveal pre-pick.
    if state.mcq_pick.is_none() && !state.answered_via_llm {
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
            // Pick-aware LLM explanation fires now (not on card open) so the
            // model sees the user's choice and tailors the response.
            let picked_key = card.choices[idx].key.clone();
            trigger_llm(state, conn, FetchMode::Explain(Some(picked_key)));
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

    // If LLM is in flight or cached for this card, split the screen.
    // (Hints and explanations both render in the side panel.)
    let panel_active = state.llm.is_some();
    let (main_area, side_area) = if panel_active && area.width > 80 {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);
        (cols[0], Some(cols[1]))
    } else {
        (area, None)
    };

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
        .split(main_area);

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
            // Reveal correctness markers when the user has picked OR has used
            // `?` to give up — both states have committed to a grade.
            let reveal_correctness = state.mcq_pick.is_some() || state.answered_via_llm;
            for (i, ch) in card.choices.iter().enumerate() {
                let prefix = format!(" {})  ", ch.key);
                let mut style = Style::default();
                let mut marker = "";
                if reveal_correctness {
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
                    } else if ch.correct {
                        // answered via `?` with no actual pick
                        style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
                        marker = "  ← correct";
                    }
                }
                lines.push(Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::DarkGray)),
                    Span::styled(render::prettify_math(&ch.text), style),
                    Span::styled(marker.to_string(), Style::default().fg(Color::DarkGray)),
                ]));
            }
            if reveal_correctness && !card.back.trim().is_empty() {
                lines.push(Line::raw(""));
                lines.push(Line::from(Span::styled(
                    "Why:",
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
                )));
                for l in render::render(&card.back).lines {
                    lines.push(l);
                }
            }
            if state.answered_via_llm && state.mcq_pick.is_none() {
                lines.push(Line::raw(""));
                lines.push(Line::from(Span::styled(
                    "(asked LLM — graded as Again)",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC),
                )));
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

    let llm_on = state.llm_enabled;
    let h_hint = if llm_on { "  [h] hint" } else { "" };
    let ask_pre = if llm_on { "  [?] reveal (=Again)" } else { "" };
    let ask_post = if llm_on { "  [?] explain" } else { "" };
    let chat_hint = if llm_on { "  [c] chat" } else { "" };
    let nav = "  [n/p] next/prev";
    let mcq_locked = state.answered_via_llm && state.mcq_pick.is_none();
    let hint = match card.kind.as_str() {
        "flip" if !state.revealed => format!(
            "[space] reveal{nav}   [e] edit{h_hint}   [q] quit"
        ),
        "flip" => format!(
            "[1] again  [2] hard  [3] good  [4] easy{nav}   [e] edit{chat_hint}{h_hint}   [q] quit"
        ),
        "mcq" if state.mcq_pick.is_none() && !mcq_locked => format!(
            "[a-e] pick{nav}   [e] edit{ask_pre}{h_hint}   [q] quit"
        ),
        "mcq" => format!(
            "[space] next{nav}   [e] edit{ask_post}{chat_hint}{h_hint}   [q] quit"
        ),
        _ => "[q] quit".to_string(),
    };
    let footer = Paragraph::new(Span::styled(
        hint.clone(),
        Style::default().fg(Color::DarkGray),
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[footer_idx]);

    if let (Some(area), Some(s)) = (side_area, state.llm.as_ref()) {
        let (picked_key, correct_key) = if s.mode == LlmMode::Hint {
            // Hint mode: don't pre-tag any option as user's pick / correct so
            // the panel doesn't accidentally spoil things via highlighting.
            (None, None)
        } else {
            (
                state
                    .mcq_pick
                    .and_then(|i| card.choices.get(i))
                    .map(|c| c.key.as_str()),
                card.choices.iter().find(|c| c.correct).map(|c| c.key.as_str()),
            )
        };
        let label = match s.mode {
            LlmMode::Explain => "explain",
            LlmMode::Hint => "hint",
        };
        let panel = super::llm_panel::render_panel(super::llm_panel::PanelView {
            model: &s.model,
            status: s.status,
            body: &s.body,
            picked_key,
            correct_key,
            error: s.error.as_deref(),
            label,
        });
        f.render_widget(panel, area);
    }
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
