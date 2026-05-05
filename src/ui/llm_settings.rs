//! Settings screen for LLM assist (toggle, model, endpoint).

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use rusqlite::Connection;

use crate::db;
use crate::llm;

use super::term::TermGuard;

const PRESETS: &[(&str, &str, &str)] = &[
    ("phi4-mini",     "3.8B  ~3.5 GB",  "default — strongest reasoning per GB"),
    ("llama3.2",      "3B    ~2.0 GB",  "fastest; lighter on hardware"),
    ("gemma3",        "4B    ~4.2 GB",  "well-rounded; 128k context"),
    ("llama3.2:1b",   "1B    ~1.3 GB",  "weakest; only for very thin hardware"),
];

pub fn run(tg: &mut TermGuard, conn: &Connection) -> Result<()> {
    let mut prefs = db::load_prefs(conn)?;
    if prefs.llm_model.is_empty() {
        prefs.llm_model = db::DEFAULT_LLM_MODEL.to_string();
    }
    if prefs.llm_endpoint.is_empty() {
        prefs.llm_endpoint = db::DEFAULT_LLM_ENDPOINT.to_string();
    }

    let mut list_state = ListState::default();
    list_state.select(Some(initial_row(&prefs.llm_model)));
    let mut endpoint_buf = prefs.llm_endpoint.clone();
    let mut editing_endpoint = false;
    let mut editing_custom_model = false;
    let mut custom_model_buf = String::new();
    let mut last_status: Option<String> = None;
    let mut last_action: Option<String> = None;

    loop {
        let row_count = PRESETS.len() + 1; // +1 for "(custom)"
        // Probe Ollama lazily — only on demand or after enabling.
        tg.term.draw(|f| {
            draw(
                f,
                &prefs,
                &mut list_state,
                &endpoint_buf,
                editing_endpoint,
                editing_custom_model,
                &custom_model_buf,
                last_status.as_deref(),
                last_action.as_deref(),
            )
        })?;
        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        if editing_endpoint {
            match key.code {
                KeyCode::Esc => {
                    endpoint_buf = prefs.llm_endpoint.clone();
                    editing_endpoint = false;
                }
                KeyCode::Enter => {
                    let v = endpoint_buf.trim().to_string();
                    if !v.is_empty() {
                        prefs.llm_endpoint = v.clone();
                        db::set_llm_endpoint(conn, &v)?;
                        last_action = Some(format!("saved endpoint: {v}"));
                    }
                    editing_endpoint = false;
                }
                KeyCode::Backspace => {
                    endpoint_buf.pop();
                }
                KeyCode::Char(c) => endpoint_buf.push(c),
                _ => {}
            }
            continue;
        }

        if editing_custom_model {
            match key.code {
                KeyCode::Esc => {
                    custom_model_buf.clear();
                    editing_custom_model = false;
                }
                KeyCode::Enter => {
                    let v = custom_model_buf.trim().to_string();
                    if !v.is_empty() {
                        prefs.llm_model = v.clone();
                        db::set_llm_model(conn, &v)?;
                        last_action = Some(format!("saved model: {v} (custom)"));
                    }
                    custom_model_buf.clear();
                    editing_custom_model = false;
                }
                KeyCode::Backspace => {
                    custom_model_buf.pop();
                }
                KeyCode::Char(c) => custom_model_buf.push(c),
                _ => {}
            }
            continue;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
            KeyCode::Char('t') => {
                let new_val = !prefs.llm_enabled;
                db::set_llm_enabled(conn, new_val)?;
                prefs.llm_enabled = new_val;
                last_action = Some(format!(
                    "llm assist: {}",
                    if new_val { "ON" } else { "off" }
                ));
            }
            KeyCode::Char('e') => {
                editing_endpoint = true;
                last_action = None;
            }
            KeyCode::Char('p') => {
                last_status = Some(probe_status(&prefs));
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let i = list_state.selected().unwrap_or(0);
                if i > 0 {
                    list_state.select(Some(i - 1));
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let i = list_state.selected().unwrap_or(0);
                if i + 1 < row_count {
                    list_state.select(Some(i + 1));
                }
            }
            KeyCode::Enter => {
                let i = list_state.selected().unwrap_or(0);
                if i < PRESETS.len() {
                    let m = PRESETS[i].0.to_string();
                    db::set_llm_model(conn, &m)?;
                    prefs.llm_model = m.clone();
                    last_action = Some(format!("saved model: {m}"));
                } else {
                    editing_custom_model = true;
                    custom_model_buf.clear();
                    last_action = None;
                }
            }
            _ => {}
        }
    }
}

fn initial_row(model: &str) -> usize {
    PRESETS
        .iter()
        .position(|(m, _, _)| *m == model)
        .unwrap_or(PRESETS.len()) // last row = custom if not in presets
}

fn probe_status(prefs: &db::Prefs) -> String {
    let endpoint = if prefs.llm_endpoint.is_empty() {
        db::DEFAULT_LLM_ENDPOINT.to_string()
    } else {
        prefs.llm_endpoint.clone()
    };
    match llm::ping(&endpoint) {
        Ok(true) => match llm::client::list_models(&endpoint) {
            Ok(models) => {
                let target = if prefs.llm_model.is_empty() {
                    db::DEFAULT_LLM_MODEL.to_string()
                } else {
                    prefs.llm_model.clone()
                };
                let installed = models.iter().any(|n| n == &target || n.starts_with(&format!("{target}:")));
                if installed {
                    format!("✓ ollama up; '{target}' is installed")
                } else {
                    format!(
                        "✓ ollama up but '{target}' is NOT installed. Run:  ollama pull {target}"
                    )
                }
            }
            Err(_) => "✓ ollama up but couldn't list models".to_string(),
        },
        Ok(false) => "x reachable but /api/tags returned an error".to_string(),
        Err(e) => format!("x ollama unreachable at {endpoint} ({})", short_err(&e)),
    }
}

fn short_err(e: &anyhow::Error) -> String {
    let s = e.to_string();
    if s.len() > 80 { format!("{}…", &s[..80]) } else { s }
}

fn draw(
    f: &mut ratatui::Frame,
    prefs: &db::Prefs,
    list_state: &mut ListState,
    endpoint_buf: &str,
    editing_endpoint: bool,
    editing_custom_model: bool,
    custom_model_buf: &str,
    last_status: Option<&str>,
    last_action: Option<&str>,
) {
    let area = f.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),                          // status / endpoint
            Constraint::Length(PRESETS.len() as u16 + 4),   // model list
            Constraint::Min(1),                             // probe / hints
            Constraint::Length(3),                          // footer
        ])
        .split(area);

    // ---- status block ----
    let enabled_label = if prefs.llm_enabled { "ON" } else { "off" };
    let enabled_style = if prefs.llm_enabled {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let mut status_lines = vec![
        Line::from(vec![
            Span::raw("status:   "),
            Span::styled(format!("{}   [t] toggle", enabled_label), enabled_style),
        ]),
        Line::from(vec![
            Span::raw("endpoint: "),
            if editing_endpoint {
                Span::styled(
                    format!("{endpoint_buf}_"),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(endpoint_buf.to_string(), Style::default().fg(Color::Yellow))
            },
            Span::styled("   [e] edit", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    if !prefs.llm_model.is_empty() {
        status_lines.push(Line::from(vec![
            Span::raw("model:    "),
            Span::styled(
                prefs.llm_model.clone(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]));
    }
    f.render_widget(
        Paragraph::new(Text::from(status_lines))
            .block(Block::default().borders(Borders::ALL).title("llm assist")),
        layout[0],
    );

    // ---- model list ----
    // We prefix each row with a static "active" marker so the user can tell
    // which model is currently saved, separately from the cursor highlight.
    let custom_active = !PRESETS.iter().any(|(m, _, _)| *m == prefs.llm_model)
        && !prefs.llm_model.is_empty();
    let mut items: Vec<ListItem> = PRESETS
        .iter()
        .map(|(name, size, note)| {
            let active = *name == prefs.llm_model;
            let marker = if active { "● " } else { "  " };
            let marker_style = if active {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let active_tag = if active { "  (active)" } else { "" };
            ListItem::new(Line::from(vec![
                Span::styled(marker.to_string(), marker_style),
                Span::styled(
                    format!("{:<14}", name),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  {:<14}", size),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(note.to_string(), Style::default().fg(Color::DarkGray)),
                Span::styled(
                    active_tag.to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]))
        })
        .collect();
    let (custom_marker, custom_marker_style) = if custom_active {
        ("● ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    } else {
        ("  ", Style::default().fg(Color::DarkGray))
    };
    let custom_label = if editing_custom_model {
        format!("(custom): {custom_model_buf}_")
    } else if custom_active {
        format!("(custom): {}", prefs.llm_model)
    } else {
        "(custom): type a model name".to_string()
    };
    let custom_text_style = if editing_custom_model {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let mut custom_spans = vec![
        Span::styled(custom_marker.to_string(), custom_marker_style),
        Span::styled(custom_label, custom_text_style),
    ];
    if custom_active {
        custom_spans.push(Span::styled(
            "  (active)".to_string(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ));
    }
    items.push(ListItem::new(Line::from(custom_spans)));
    f.render_stateful_widget(
        List::new(items)
            .block(Block::default().borders(Borders::ALL).title("model"))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ "),
        layout[1],
        list_state,
    );

    // ---- probe / hints ----
    let mut probe_lines: Vec<Line> = Vec::new();
    if let Some(act) = last_action {
        probe_lines.push(Line::from(Span::styled(
            format!("✓ {act}"),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )));
    }
    match last_status {
        Some(s) => {
            let style = if s.starts_with('✓') {
                Style::default().fg(Color::Green)
            } else if s.starts_with('x') {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Yellow)
            };
            probe_lines.push(Line::from(Span::styled(s.to_string(), style)));
        }
        None => {
            if last_action.is_none() {
                probe_lines.push(Line::from(Span::styled(
                    "press [p] to probe ollama at the endpoint above",
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }
    }
    f.render_widget(
        Paragraph::new(Text::from(probe_lines))
            .block(Block::default().borders(Borders::ALL).title("probe")),
        layout[2],
    );

    // ---- footer ----
    let footer = if editing_endpoint || editing_custom_model {
        "[enter] save   [esc] cancel"
    } else {
        "[j/k] move   [enter] save model   [t] toggle   [e] endpoint   [p] probe   [q/esc] back"
    };
    f.render_widget(
        Paragraph::new(Span::styled(
            footer,
            Style::default().fg(Color::DarkGray),
        ))
        .block(Block::default().borders(Borders::ALL)),
        layout[3],
    );
}
