//! Three-stage picker: track → topic → difficulty. Returns a `Scope`.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use rusqlite::Connection;

use crate::db::{self, Difficulty, Scope, TrackTopicCount};

use super::term::TermGuard;

pub fn run(tg: &mut TermGuard, conn: &Connection) -> Result<Option<Scope>> {
    let topics = db::list_topics(conn)?;
    if topics.is_empty() {
        return Ok(None);
    }
    let mut tracks: Vec<String> = topics.iter().map(|t| t.track.clone()).collect();
    tracks.dedup();
    let mut state = State::new(&tracks);

    loop {
        tg.term.draw(|f| draw(f, &topics, &tracks, &mut state))?;
        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => match state.stage {
                Stage::Track => return Ok(None),
                Stage::Topic => {
                    state.stage = Stage::Track;
                    state.list.select(Some(state.track_idx));
                }
                Stage::Difficulty => {
                    state.stage = Stage::Topic;
                    state.list.select(Some(state.topic_idx_local));
                }
            },
            KeyCode::Up | KeyCode::Char('k') => {
                let i = state.list.selected().unwrap_or(0);
                if i > 0 {
                    state.list.select(Some(i - 1));
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let n = match state.stage {
                    Stage::Track => tracks.len() + 1, // +1 for "all tracks"
                    Stage::Topic => state.current_topics(&topics).len() + 1,
                    Stage::Difficulty => 4,
                };
                let i = state.list.selected().unwrap_or(0);
                if i + 1 < n {
                    state.list.select(Some(i + 1));
                }
            }
            KeyCode::Enter => match state.stage {
                Stage::Track => {
                    let i = state.list.selected().unwrap_or(0);
                    if i == 0 {
                        // "all tracks" → straight to difficulty (with no topic filter)
                        state.track = None;
                        state.topic = None;
                        state.stage = Stage::Difficulty;
                        state.list.select(Some(0));
                    } else {
                        state.track_idx = i;
                        state.track = Some(tracks[i - 1].clone());
                        state.stage = Stage::Topic;
                        state.list.select(Some(0));
                    }
                }
                Stage::Topic => {
                    let i = state.list.selected().unwrap_or(0);
                    let local = state.current_topics(&topics);
                    if i == 0 {
                        state.topic = None; // all topics in track
                    } else {
                        state.topic_idx_local = i;
                        state.topic = Some(local[i - 1].topic.clone());
                    }
                    state.stage = Stage::Difficulty;
                    state.list.select(Some(0));
                }
                Stage::Difficulty => {
                    let diff = match state.list.selected().unwrap_or(0) {
                        1 => Difficulty::Beginner,
                        2 => Difficulty::Intermediate,
                        3 => Difficulty::Advanced,
                        _ => Difficulty::All,
                    };
                    return Ok(Some(Scope {
                        track: state.track.clone(),
                        topic: state.topic.clone(),
                        difficulty: diff,
                    }));
                }
            },
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Track,
    Topic,
    Difficulty,
}

struct State {
    stage: Stage,
    list: ListState,
    track_idx: usize,        // index into tracks list (1-based; 0 = "all tracks")
    topic_idx_local: usize,
    track: Option<String>,
    topic: Option<String>,
}

impl State {
    fn new(_tracks: &[String]) -> Self {
        let mut list = ListState::default();
        list.select(Some(0));
        Self {
            stage: Stage::Track,
            list,
            track_idx: 0,
            topic_idx_local: 0,
            track: None,
            topic: None,
        }
    }
    fn current_topics<'a>(&self, topics: &'a [TrackTopicCount]) -> Vec<&'a TrackTopicCount> {
        let track = match &self.track {
            Some(t) => t,
            None => return topics.iter().collect(),
        };
        topics.iter().filter(|t| &t.track == track).collect()
    }
}

fn draw(
    f: &mut ratatui::Frame,
    topics: &[TrackTopicCount],
    tracks: &[String],
    state: &mut State,
) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
        .split(area);

    let crumb = build_breadcrumb(state);
    f.render_widget(
        Paragraph::new(Span::styled(
            crumb,
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ))
        .block(Block::default().borders(Borders::ALL)),
        chunks[0],
    );

    let (title, items) = match state.stage {
        Stage::Track => {
            let mut items: Vec<ListItem> = vec![ListItem::new(Line::from(Span::styled(
                "all tracks",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )))];
            for t in tracks {
                let n: i64 = topics.iter().filter(|x| &x.track == t).map(|x| x.n_cards).sum();
                let n_topics = topics.iter().filter(|x| &x.track == t).count();
                items.push(ListItem::new(Line::from(vec![
                    Span::styled(t.clone(), Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("   {} topics, {} cards", n_topics, n),
                        Style::default().fg(Color::DarkGray),
                    ),
                ])));
            }
            ("track", items)
        }
        Stage::Topic => {
            let local = state.current_topics(topics);
            let total: i64 = local.iter().map(|t| t.n_cards).sum();
            let mut items: Vec<ListItem> = vec![ListItem::new(Line::from(vec![
                Span::styled(
                    "all topics in this track",
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("   {} cards", total),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))];
            for t in &local {
                items.push(ListItem::new(Line::from(vec![
                    Span::styled(t.topic.clone(), Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("   {} cards", t.n_cards),
                        Style::default().fg(Color::DarkGray),
                    ),
                ])));
            }
            ("topic", items)
        }
        Stage::Difficulty => {
            let labels = [
                ("all difficulties", Difficulty::All),
                ("beginner (1-2)", Difficulty::Beginner),
                ("intermediate (3)", Difficulty::Intermediate),
                ("advanced (4-5)", Difficulty::Advanced),
            ];
            let items: Vec<ListItem> = labels
                .iter()
                .map(|(label, _)| {
                    ListItem::new(Line::from(Span::styled(
                        label.to_string(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )))
                })
                .collect();
            ("difficulty", items)
        }
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, chunks[1], &mut state.list);

    let footer_text = match state.stage {
        Stage::Track => "[j/k] move   [enter] drill in   [q/esc] back",
        Stage::Topic => "[j/k] move   [enter] drill in   [esc] back to tracks",
        Stage::Difficulty => "[j/k] move   [enter] start review   [esc] back to topics",
    };
    f.render_widget(
        Paragraph::new(Span::styled(
            footer_text,
            Style::default().fg(Color::DarkGray),
        ))
        .block(Block::default().borders(Borders::ALL)),
        chunks[2],
    );
}

fn build_breadcrumb(state: &State) -> String {
    let track = state.track.as_deref().unwrap_or("(all tracks)");
    let topic = state.topic.as_deref().unwrap_or("(all topics)");
    match state.stage {
        Stage::Track => "pick a track".into(),
        Stage::Topic => format!("{}  ›  pick a topic", track),
        Stage::Difficulty => format!("{}  ›  {}  ›  pick difficulty", track, topic),
    }
}
