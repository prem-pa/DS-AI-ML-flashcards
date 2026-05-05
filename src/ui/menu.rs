//! Main menu shown after a profile is resolved. Maps choices to actions
//! that the launcher executes (review with scope, browse, stats, switch profile, etc.).

use anyhow::Result;
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use rusqlite::Connection;

use crate::db::{self, Prefs, Scope, SessionRow};

use super::term::TermGuard;

#[derive(Debug)]
pub enum Action {
    ReviewAllDue,
    ContinueLast(SessionRow),
    PickTopic,
    Browse,
    Stats,
    LlmSettings,
    SwitchProfile,
    Quit,
}

pub fn run(tg: &mut TermGuard, conn: &Connection, profile_display: &str) -> Result<Action> {
    loop {
        let now = Utc::now().timestamp();
        let prefs = db::load_prefs(conn)?;
        let mut scope = Scope::default();
        db::apply_prefs(&mut scope, &prefs);
        let due_total = db::count_due_scoped(conn, now, &scope)?;
        let active_total = db::count_total_scoped(conn, &scope)?;
        let last_session = db::most_recent_session(conn)?;

        tg.term.draw(|f| {
            draw(
                f,
                profile_display,
                &prefs,
                due_total,
                active_total,
                last_session.as_ref(),
            )
        })?;
        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(Action::Quit),
            KeyCode::Char('c') | KeyCode::Char('1') => {
                if let Some(s) = &last_session {
                    return Ok(Action::ContinueLast(s.clone()));
                }
            }
            KeyCode::Char('d') | KeyCode::Char('2') => return Ok(Action::ReviewAllDue),
            KeyCode::Char('t') | KeyCode::Char('3') => return Ok(Action::PickTopic),
            KeyCode::Char('b') | KeyCode::Char('4') => return Ok(Action::Browse),
            KeyCode::Char('s') | KeyCode::Char('5') => return Ok(Action::Stats),
            KeyCode::Char('p') | KeyCode::Char('6') => return Ok(Action::SwitchProfile),
            KeyCode::Char('l') => return Ok(Action::LlmSettings),
            KeyCode::Char('m') => {
                db::set_mcq_only(conn, !prefs.mcq_only)?;
                // re-loop: redraw with updated counts
                continue;
            }
            _ => {}
        }
    }
}

fn draw(
    f: &mut ratatui::Frame,
    profile_display: &str,
    prefs: &Prefs,
    due_total: i64,
    active_total: i64,
    last_session: Option<&SessionRow>,
) {
    let area = f.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                "flashcards",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::raw("   profile: "),
            Span::styled(
                profile_display.to_string(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled(
                format!("mcq-only: {}", if prefs.mcq_only { "ON" } else { "off" }),
                if prefs.mcq_only {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                },
            ),
        ]))
        .block(Block::default().borders(Borders::ALL)),
        layout[0],
    );

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::raw(""));

    // Continue last session
    let cont_line = match last_session {
        Some(s) => {
            let mut ago = relative(s.started_at);
            if let Some(end) = s.ended_at {
                ago = format!("ended {}", relative(end));
            } else {
                ago = format!("started {}", ago);
            }
            Line::from(vec![
                Span::styled(
                    "  c) ",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "continue last session",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("        ({}, scope: {}, {} card{})", ago, s.scope.label(), s.cards_reviewed, if s.cards_reviewed == 1 { "" } else { "s" }),
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        }
        None => Line::from(vec![
            Span::styled(
                "  c) ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                "continue last session",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                "        (no sessions yet)",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    };
    lines.push(cont_line);

    lines.push(Line::from(vec![
        Span::styled(
            "  d) ",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "review all due",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("              ({} due, {} active total)", due_total, active_total),
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    lines.push(Line::from(vec![
        Span::styled(
            "  t) ",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "pick topic / difficulty",
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::raw(""));

    lines.push(Line::from(vec![
        Span::styled("  b) ", Style::default().fg(Color::DarkGray)),
        Span::raw("browse cards"),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  s) ", Style::default().fg(Color::DarkGray)),
        Span::raw("stats"),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  p) ", Style::default().fg(Color::DarkGray)),
        Span::raw("switch profile"),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  m) ", Style::default().fg(Color::DarkGray)),
        Span::raw("toggle "),
        Span::styled(
            "mcq-only",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  (currently {})", if prefs.mcq_only { "on" } else { "off" }),
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    let llm_status = if prefs.llm_enabled {
        let m = if prefs.llm_model.is_empty() {
            db::DEFAULT_LLM_MODEL.to_string()
        } else {
            prefs.llm_model.clone()
        };
        format!("on · {}", m)
    } else {
        "off".to_string()
    };
    lines.push(Line::from(vec![
        Span::styled("  l) ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "llm assist",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("            ({})", llm_status),
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  q) ", Style::default().fg(Color::DarkGray)),
        Span::raw("quit"),
    ]));

    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("menu")),
        layout[1],
    );

    let footer = Paragraph::new(Span::styled(
        "press the highlighted letter (or its number) to choose",
        Style::default().fg(Color::DarkGray),
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, layout[2]);
}

fn relative(ts: i64) -> String {
    let now = Utc::now().timestamp();
    let diff = (now - ts).max(0);
    if diff < 60 {
        "just now".into()
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else {
        format!("{}d ago", diff / 86400)
    }
}
