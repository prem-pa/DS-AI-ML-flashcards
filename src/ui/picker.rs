//! Profile picker. Lists existing profiles and offers a "create new" path
//! that suggests Reddit-style usernames (`drowsy-otter-42`).

use anyhow::Result;
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use crate::profile::{self, names::Rng, ProfileSummary};

use super::term::TermGuard;

#[derive(Debug)]
pub enum PickerOutcome {
    Picked { slug: String, display_name: String },
    Quit,
}

pub fn run(tg: &mut TermGuard) -> Result<PickerOutcome> {
    let mut profiles = profile::list()?;
    let mut state = State::new(&profiles);
    let mut rng = Rng::from_clock();
    let mut form: Option<FormState> = None;

    loop {
        if let Some(c) = &mut form {
            tg.term.draw(|f| draw_form(f, c))?;
        } else {
            tg.term.draw(|f| draw_list(f, &profiles, &mut state))?;
        }
        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        if let Some(c) = form.as_mut() {
            match key.code {
                KeyCode::Esc => {
                    form = None;
                }
                KeyCode::Enter => {
                    let raw = c.input.trim().to_string();
                    let slug = profile::slugify(&raw);
                    if slug.is_empty() {
                        c.error = Some("name must contain at least one alphanumeric character".into());
                        continue;
                    }
                    match c.mode {
                        FormMode::Create => {
                            if profiles.iter().any(|p| p.slug == slug) {
                                c.error = Some(format!("profile '{slug}' already exists"));
                                continue;
                            }
                            let display = if raw.is_empty() { slug.clone() } else { raw };
                            return Ok(PickerOutcome::Picked {
                                slug,
                                display_name: display,
                            });
                        }
                        FormMode::Rename(ref old_slug) => {
                            if &slug != old_slug && profiles.iter().any(|p| p.slug == slug) {
                                c.error = Some(format!("profile '{slug}' already exists"));
                                continue;
                            }
                            let display = if raw.is_empty() { slug.clone() } else { raw };
                            if let Err(e) = profile::rename_profile(old_slug, &slug, &display) {
                                c.error = Some(format!("rename failed: {e}"));
                                continue;
                            }
                            profiles = profile::list()?;
                            // try to keep the renamed profile selected
                            let pos = profiles.iter().position(|p| p.slug == slug).unwrap_or(0);
                            state.list.select(Some(pos));
                            form = None;
                        }
                    }
                }
                KeyCode::Backspace => {
                    c.input.pop();
                    c.error = None;
                }
                KeyCode::Char('r') if c.input.is_empty() => {
                    c.suggestions = profile::names::suggestions(8, &mut rng);
                }
                KeyCode::Char(d) if d.is_ascii_digit() && c.input.is_empty() => {
                    let idx = (d as u8 - b'0') as usize;
                    if idx >= 1 && idx <= c.suggestions.len() {
                        c.input = c.suggestions[idx - 1].clone();
                        c.error = None;
                    }
                }
                KeyCode::Char(ch) => {
                    c.input.push(ch);
                    c.error = None;
                }
                _ => {}
            }
            continue;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(PickerOutcome::Quit),
            KeyCode::Char('n') => {
                form = Some(FormState {
                    mode: FormMode::Create,
                    input: String::new(),
                    suggestions: profile::names::suggestions(8, &mut rng),
                    error: None,
                });
            }
            KeyCode::Char('r') => {
                if let Some(idx) = state.list.selected() {
                    if idx < profiles.len() {
                        let p = &profiles[idx];
                        form = Some(FormState {
                            mode: FormMode::Rename(p.slug.clone()),
                            input: p.display_name.clone(),
                            suggestions: profile::names::suggestions(8, &mut rng),
                            error: None,
                        });
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') => state.up(profiles.len()),
            KeyCode::Down | KeyCode::Char('j') => state.down(profiles.len()),
            KeyCode::Char('d') => {
                if let Some(idx) = state.list.selected() {
                    if idx < profiles.len() {
                        let slug = profiles[idx].slug.clone();
                        profile::delete_profile(&slug)?;
                        profiles = profile::list()?;
                        state.list.select(Some(0));
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(idx) = state.list.selected() {
                    if idx < profiles.len() {
                        let p = &profiles[idx];
                        return Ok(PickerOutcome::Picked {
                            slug: p.slug.clone(),
                            display_name: p.display_name.clone(),
                        });
                    } else {
                        form = Some(FormState {
                            mode: FormMode::Create,
                            input: String::new(),
                            suggestions: profile::names::suggestions(8, &mut rng),
                            error: None,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

struct State {
    list: ListState,
}

impl State {
    fn new(profiles: &[ProfileSummary]) -> Self {
        let mut s = ListState::default();
        s.select(Some(if profiles.is_empty() { 0 } else { 0 }));
        Self { list: s }
    }
    fn down(&mut self, n_profiles: usize) {
        let total = n_profiles + 1; // +1 for "create new" row
        let i = self.list.selected().unwrap_or(0);
        if i + 1 < total {
            self.list.select(Some(i + 1));
        }
    }
    fn up(&mut self, _n: usize) {
        let i = self.list.selected().unwrap_or(0);
        if i > 0 {
            self.list.select(Some(i - 1));
        }
    }
}

enum FormMode {
    Create,
    Rename(String), // old slug
}

struct FormState {
    mode: FormMode,
    input: String,
    suggestions: Vec<String>,
    error: Option<String>,
}

fn draw_list(f: &mut ratatui::Frame, profiles: &[ProfileSummary], state: &mut State) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(area);

    let mut items: Vec<ListItem> = profiles
        .iter()
        .map(|p| {
            let active = relative_time(p.last_active_at);
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        p.display_name.clone(),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("   {}", p.slug),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]),
                Line::from(Span::styled(
                    format!("    last active {}", active),
                    Style::default().fg(Color::DarkGray),
                )),
            ])
        })
        .collect();
    items.push(ListItem::new(Line::from(Span::styled(
        "+ create new profile",
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    ))));

    let title = if profiles.is_empty() {
        "no profiles yet — create one".to_string()
    } else {
        format!("pick a profile ({})", profiles.len())
    };
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, chunks[0], &mut state.list);

    let footer = Paragraph::new(Span::styled(
        "[j/k] move   [enter] select   [n] new   [r] rename   [d] delete   [q] quit",
        Style::default().fg(Color::DarkGray),
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[1]);
}

fn draw_form(f: &mut ratatui::Frame, c: &FormState) {
    let area = f.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(c.suggestions.len() as u16 + 4),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    let (heading, title) = match &c.mode {
        FormMode::Create => (
            "name your profile (anything you like — slug is auto-derived)".to_string(),
            "new profile".to_string(),
        ),
        FormMode::Rename(old) => (
            format!("rename '{}' (slug auto-derived; file is renamed if slug changes)", old),
            format!("rename profile '{}'", old),
        ),
    };
    let input_lines = vec![
        Line::from(Span::styled(heading, Style::default().fg(Color::DarkGray))),
        Line::raw(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("{}_", c.input),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    f.render_widget(
        Paragraph::new(Text::from(input_lines))
            .block(Block::default().borders(Borders::ALL).title(title)),
        layout[0],
    );

    let mut sug_lines: Vec<Line> = vec![Line::from(Span::styled(
        "suggestions  (press a digit to use, or [r] to reroll)",
        Style::default().fg(Color::DarkGray),
    ))];
    sug_lines.push(Line::raw(""));
    for (i, s) in c.suggestions.iter().enumerate() {
        sug_lines.push(Line::from(vec![
            Span::styled(format!("  {}) ", i + 1), Style::default().fg(Color::DarkGray)),
            Span::styled(s.clone(), Style::default().fg(Color::Yellow)),
        ]));
    }
    f.render_widget(
        Paragraph::new(Text::from(sug_lines))
            .block(Block::default().borders(Borders::ALL).title("ideas")),
        layout[1],
    );

    let info: Vec<Line> = if let Some(err) = &c.error {
        vec![Line::from(Span::styled(
            err.clone(),
            Style::default().fg(Color::Red),
        ))]
    } else {
        vec![Line::from(Span::styled(
            format!("slug = {}", profile::slugify(&c.input)),
            Style::default().fg(Color::DarkGray),
        ))]
    };
    f.render_widget(
        Paragraph::new(Text::from(info))
            .block(Block::default().borders(Borders::ALL)),
        layout[2],
    );

    let footer = Paragraph::new(Span::styled(
        "[enter] create   [r] reroll (when input empty)   [1-8] use suggestion   [esc] back",
        Style::default().fg(Color::DarkGray),
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, layout[3]);
}

fn relative_time(ts: i64) -> String {
    if ts <= 0 {
        return "—".into();
    }
    let now = Utc::now().timestamp();
    let diff = (now - ts).max(0);
    if diff < 60 {
        "just now".into()
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else if diff < 86400 * 30 {
        format!("{}d ago", diff / 86400)
    } else {
        format!("{}mo ago", diff / (86400 * 30))
    }
}

fn _suppress_unused() -> Rect {
    Rect::default()
}
