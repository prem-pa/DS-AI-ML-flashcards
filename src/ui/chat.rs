//! Multi-turn chat overlay for flip cards. Persists exchanges in
//! `chat_messages` so threads continue across launches.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rusqlite::Connection;
use std::path::Path;
use std::time::Duration;

use crate::db::{self, CardView, ChatMessage, ChatRole};
use crate::llm;
use crate::render;

use super::term::TermGuard;

pub fn run(
    tg: &mut TermGuard,
    conn: &Connection,
    card: &CardView,
    vault_root: &Path,
    model: &str,
    endpoint: &str,
    pick: Option<llm::prompt::PickContext>,
) -> Result<()> {
    let mut messages = db::fetch_chat(conn, &card.id)?;
    if messages.is_empty() {
        // First time on this card: build context bundle, then seed the thread
        // with a hidden system message + a visible "primer" assistant turn so
        // the user immediately sees what's loaded and is invited to pick a
        // direction.
        let bundle = llm::context::build(conn, card, vault_root)?;
        let sys = llm::prompt::build_chat_system_with_pick(&bundle, pick.as_ref());
        let sys_id = db::append_chat(conn, &card.id, ChatRole::System, &sys, Some(model))?;
        messages.push(ChatMessage {
            id: sys_id,
            role: ChatRole::System,
            content: sys,
            sent_at: chrono::Utc::now().timestamp(),
        });
        let primer = llm::prompt::build_chat_primer(&bundle, pick.as_ref());
        let pid = db::append_chat(conn, &card.id, ChatRole::Assistant, &primer, Some(model))?;
        messages.push(ChatMessage {
            id: pid,
            role: ChatRole::Assistant,
            content: primer,
            sent_at: chrono::Utc::now().timestamp(),
        });
    }
    let mut input = String::new();
    let mut error: Option<String> = None;
    let mut handle: Option<llm::StreamHandle> = None;
    let mut partial = String::new();

    loop {
        // pump streaming tokens
        if let Some(h) = handle.as_mut() {
            let (tokens, terminal) = h.poll();
            for t in tokens {
                partial.push_str(&t);
            }
            if let Some(ev) = terminal {
                match ev {
                    llm::Event::Done(final_body) => {
                        let body = if !final_body.is_empty() {
                            final_body
                        } else {
                            partial.clone()
                        };
                        if !body.trim().is_empty() {
                            let id = db::append_chat(
                                conn,
                                &card.id,
                                ChatRole::Assistant,
                                &body,
                                Some(model),
                            )?;
                            messages.push(ChatMessage {
                                id,
                                role: ChatRole::Assistant,
                                content: body,
                                sent_at: chrono::Utc::now().timestamp(),
                            });
                        }
                        handle = None;
                        partial.clear();
                    }
                    llm::Event::Error(msg) => {
                        error = Some(msg);
                        handle = None;
                        partial.clear();
                    }
                    _ => {}
                }
            }
        }

        tg.term.draw(|f| draw(f, card, model, &messages, &partial, &input, error.as_deref()))?;

        if !event::poll(Duration::from_millis(80))? {
            continue;
        }
        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Esc => return Ok(()),
            KeyCode::Enter => {
                let q = input.trim().to_string();
                input.clear();
                if q.is_empty() {
                    continue;
                }
                if q == ":clear" {
                    db::clear_chat(conn, &card.id)?;
                    messages.clear();
                    continue;
                }
                if q == ":q" || q == ":quit" {
                    return Ok(());
                }
                if handle.is_some() {
                    error = Some("a previous request is still streaming; please wait".into());
                    continue;
                }
                error = None;
                let id = db::append_chat(conn, &card.id, ChatRole::User, &q, None)?;
                messages.push(ChatMessage {
                    id,
                    role: ChatRole::User,
                    content: q,
                    sent_at: chrono::Utc::now().timestamp(),
                });
                let to_send: Vec<llm::client::OwnedChatMessage> = messages
                    .iter()
                    .map(|m| llm::client::OwnedChatMessage {
                        role: m.role.as_str().to_string(),
                        content: m.content.clone(),
                    })
                    .collect();
                handle = Some(llm::client::chat_stream(endpoint, model, to_send));
                partial.clear();
            }
            KeyCode::Backspace => {
                input.pop();
            }
            KeyCode::Char(c) => {
                input.push(c);
            }
            _ => {}
        }
    }
}

fn draw(
    f: &mut ratatui::Frame,
    card: &CardView,
    model: &str,
    messages: &[ChatMessage],
    partial: &str,
    input: &str,
    error: Option<&str>,
) {
    let area = f.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                "chat",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled(
                card.concept_title.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("   model: "),
            Span::styled(
                model.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]))
        .block(Block::default().borders(Borders::ALL)),
        layout[0],
    );

    let mut lines: Vec<Line<'static>> = Vec::new();
    for m in messages {
        if m.role == ChatRole::System {
            continue; // hide system context from the user-visible scrollback
        }
        let prefix = match m.role {
            ChatRole::User => "you: ",
            ChatRole::Assistant => "ai:  ",
            ChatRole::System => "sys: ",
        };
        let prefix_style = match m.role {
            ChatRole::User => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ChatRole::Assistant => {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            }
            ChatRole::System => Style::default().fg(Color::DarkGray),
        };
        let body_lines = render::render(&m.content).lines;
        let mut first = true;
        for ln in body_lines {
            if first {
                let mut spans = vec![Span::styled(prefix.to_string(), prefix_style)];
                spans.extend(ln.spans);
                lines.push(Line::from(spans));
                first = false;
            } else {
                let mut spans = vec![Span::styled("     ".to_string(), Style::default())];
                spans.extend(ln.spans);
                lines.push(Line::from(spans));
            }
        }
        lines.push(Line::raw(""));
    }
    if !partial.is_empty() {
        let body_lines = render::render(partial).lines;
        let mut first = true;
        for ln in body_lines {
            if first {
                let mut spans = vec![Span::styled(
                    "ai:  ".to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD | Modifier::ITALIC),
                )];
                spans.extend(ln.spans);
                lines.push(Line::from(spans));
                first = false;
            } else {
                let mut spans = vec![Span::raw("     ".to_string())];
                spans.extend(ln.spans);
                lines.push(Line::from(spans));
            }
        }
    }

    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title("scrollback")),
        layout[1],
    );

    let input_para = Paragraph::new(Line::from(vec![
        Span::styled(
            "› ",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{input}_"),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title("input"));
    f.render_widget(input_para, layout[2]);

    let footer_text = if let Some(e) = error {
        e.to_string()
    } else {
        "[enter] send   :clear erases history   [esc] back".to_string()
    };
    let footer_style = if error.is_some() {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    f.render_widget(
        Paragraph::new(Span::styled(footer_text, footer_style))
            .block(Block::default().borders(Borders::ALL)),
        layout[3],
    );
}
