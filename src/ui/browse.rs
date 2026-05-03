use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use rusqlite::Connection;

use crate::cli::Cli;
use crate::db::{self, CardView};
use crate::render;

use super::term::TermGuard;

pub fn run(conn: &Connection, cli: &Cli) -> Result<()> {
    run_with_term(conn, cli, None)
}

pub fn run_with_term(
    conn: &Connection,
    cli: &Cli,
    held_term: Option<&mut TermGuard>,
) -> Result<()> {
    let prefs = db::load_prefs(conn)?;
    let mut scope = db::Scope::default();
    db::apply_prefs(&mut scope, &prefs);
    let cards = db::fetch_all_scoped(conn, &scope, 10_000)?;
    if cards.is_empty() {
        println!("No cards in DB. Run `flashcards sync` first.");
        return Ok(());
    }
    match held_term {
        Some(tg) => run_loop(tg, conn, cli, cards),
        None => {
            let mut tg = TermGuard::enter()?;
            let r = run_loop(&mut tg, conn, cli, cards);
            drop(tg);
            r
        }
    }
}

#[derive(Default)]
struct State {
    query: String,
    filter_mode: bool,
    visible: Vec<usize>, // indices into the master `cards` slice
    list_state: ListState,
}

fn run_loop(
    tg: &mut TermGuard,
    conn: &Connection,
    _cli: &Cli,
    cards: Vec<CardView>,
) -> Result<()> {
    let mut state = State::default();
    state.visible = (0..cards.len()).collect();
    state.list_state.select(Some(0));
    let mut matcher = Matcher::new(Config::DEFAULT);

    loop {
        tg.term.draw(|f| draw(f, &cards, &mut state))?;
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        if state.filter_mode {
            match key.code {
                KeyCode::Esc => {
                    state.filter_mode = false;
                    state.query.clear();
                    state.visible = (0..cards.len()).collect();
                    state.list_state.select(Some(0));
                }
                KeyCode::Enter => {
                    state.filter_mode = false;
                }
                KeyCode::Backspace => {
                    state.query.pop();
                    refilter(&cards, &mut state, &mut matcher);
                }
                KeyCode::Char(c) => {
                    state.query.push(c);
                    refilter(&cards, &mut state, &mut matcher);
                }
                _ => {}
            }
            continue;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
            KeyCode::Char('/') => {
                state.filter_mode = true;
            }
            KeyCode::Char('e') => {
                if let Some(_card) = current(&cards, &state) {
                    // edit-in-vault is wired in the review screen (where it makes
                    // most sense). Browse stays read-only for now to avoid the
                    // cursor-state dance.
                }
            }
            KeyCode::Down | KeyCode::Char('j') => move_sel(&mut state, 1),
            KeyCode::Up | KeyCode::Char('k') => move_sel(&mut state, -1),
            KeyCode::PageDown | KeyCode::Char('d') => move_sel(&mut state, 20),
            KeyCode::PageUp | KeyCode::Char('u') => move_sel(&mut state, -20),
            KeyCode::Home | KeyCode::Char('g') => state.list_state.select(Some(0)),
            KeyCode::End | KeyCode::Char('G') => {
                if !state.visible.is_empty() {
                    state.list_state.select(Some(state.visible.len() - 1));
                }
            }
            _ => {
                let _ = conn; // currently unused outside review
            }
        }
    }
}

fn move_sel(state: &mut State, delta: i64) {
    let n = state.visible.len();
    if n == 0 {
        return;
    }
    let cur = state.list_state.selected().unwrap_or(0) as i64;
    let next = (cur + delta).clamp(0, n as i64 - 1) as usize;
    state.list_state.select(Some(next));
}

fn current<'a>(cards: &'a [CardView], state: &State) -> Option<&'a CardView> {
    let idx = state.list_state.selected()?;
    let card_idx = *state.visible.get(idx)?;
    cards.get(card_idx)
}

fn refilter(cards: &[CardView], state: &mut State, matcher: &mut Matcher) {
    if state.query.is_empty() {
        state.visible = (0..cards.len()).collect();
    } else {
        let pat = Pattern::parse(
            &state.query,
            CaseMatching::Smart,
            Normalization::Smart,
        );
        let haystacks: Vec<String> = cards
            .iter()
            .map(|c| {
                format!(
                    "{} {} {} {} {}",
                    c.concept_title, c.front, c.track, c.topic, c.concept_slug
                )
            })
            .collect();
        let mut scored: Vec<(usize, u32)> = Vec::new();
        for (i, h) in haystacks.iter().enumerate() {
            let utf32 = nucleo_matcher::Utf32String::from(h.as_str());
            if let Some(score) = pat.score(utf32.slice(..), matcher) {
                scored.push((i, score));
            }
        }
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        state.visible = scored.into_iter().map(|(i, _)| i).collect();
    }
    if state.visible.is_empty() {
        state.list_state.select(None);
    } else {
        state.list_state.select(Some(0));
    }
}

fn draw(f: &mut ratatui::Frame, cards: &[CardView], state: &mut State) {
    let area = f.area();
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(area);
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(rows[0]);

    let items: Vec<ListItem> = state
        .visible
        .iter()
        .filter_map(|i| cards.get(*i))
        .map(|c| {
            let prefix = if c.kind == "mcq" { "M" } else { "F" };
            let label = format!(
                "{} {} :: {}",
                prefix,
                c.concept_title,
                truncate(&c.front, 80)
            );
            ListItem::new(label)
        })
        .collect();
    let title = format!("Cards ({} / {})", state.visible.len(), cards.len());
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, cols[0], &mut state.list_state);

    let preview = current(cards, state)
        .map(render_preview)
        .unwrap_or_else(|| Text::raw(""));
    let preview_widget = Paragraph::new(preview)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title("Preview"));
    f.render_widget(preview_widget, cols[1]);

    let footer_text = if state.filter_mode {
        format!("/ {}_   [enter] apply  [esc] clear", state.query)
    } else {
        "[/] filter   [j/k] move   [g/G] top/bottom   [q] quit".to_string()
    };
    let footer_style = if state.filter_mode {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let footer = Paragraph::new(Span::styled(footer_text, footer_style))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, rows[1]);
}

fn render_preview(c: &CardView) -> Text<'static> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    lines.push(Line::from(Span::styled(
        format!("{} · {}", c.track, c.topic),
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        c.concept_title.clone(),
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        "Front:",
        Style::default().fg(Color::DarkGray),
    )));
    for l in render::render(&c.front).lines {
        lines.push(l);
    }
    if c.kind == "mcq" {
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Choices:",
            Style::default().fg(Color::DarkGray),
        )));
        for ch in &c.choices {
            let style = if ch.correct {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            lines.push(Line::from(vec![
                Span::styled(
                    format!(" {}) ", ch.key),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(render::prettify_math(&ch.text), style),
            ]));
        }
    }
    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        "Back:",
        Style::default().fg(Color::DarkGray),
    )));
    for l in render::render(&c.back).lines {
        lines.push(l);
    }
    Text::from(lines)
}

fn truncate(s: &str, n: usize) -> String {
    let single = s.replace('\n', " ");
    if single.chars().count() <= n {
        single
    } else {
        let mut out: String = single.chars().take(n - 1).collect();
        out.push('…');
        out
    }
}
