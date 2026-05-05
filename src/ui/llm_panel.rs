//! Renders the streaming LLM body in a side panel, with the picked MCQ
//! option highlighted (green if right, red if wrong).

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::render;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Streaming,
    Cached,
    Done,
    Error,
}

pub struct PanelView<'a> {
    pub model: &'a str,
    pub status: Status,
    pub body: &'a str,
    pub picked_key: Option<&'a str>,
    pub correct_key: Option<&'a str>,
    pub error: Option<&'a str>,
    pub label: &'a str, // "explain" or "hint"
}

pub fn render_panel<'a>(view: PanelView<'a>) -> Paragraph<'a> {
    let title = format!(
        "llm · {} · {} · {}",
        view.model,
        view.label,
        match view.status {
            Status::Streaming => "streaming",
            Status::Cached => "cached",
            Status::Done => "done",
            Status::Error => "error",
        }
    );
    let mut lines: Vec<Line<'static>> = Vec::new();

    if let Some(err) = view.error {
        lines.push(Line::from(Span::styled(
            err.to_string(),
            Style::default().fg(Color::Red),
        )));
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Tip: ensure ollama is running and the model is pulled.",
            Style::default().fg(Color::DarkGray),
        )));
        return Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title(title));
    }

    if view.body.trim().is_empty() {
        lines.push(Line::from(Span::styled(
            "thinking…",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )));
    } else {
        // Render the body through markdown so **bold** etc. show, then apply
        // option-aware highlighting line by line.
        let rendered = render::render(view.body);
        for line in rendered.lines {
            lines.push(restyle_for_pick(line, view.picked_key, view.correct_key));
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title(title))
}

fn restyle_for_pick<'a>(
    line: Line<'a>,
    picked: Option<&str>,
    correct: Option<&str>,
) -> Line<'a> {
    let raw: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
    if let Some(letter) = leading_option_letter(&raw) {
        let pl = picked.map(|s| s.to_ascii_uppercase());
        let cl = correct.map(|s| s.to_ascii_uppercase());
        let is_pick = pl.as_deref() == Some(letter.as_str());
        let is_correct = cl.as_deref() == Some(letter.as_str());
        let style = if is_pick && is_correct {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else if is_pick {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else if is_correct {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };
        // Re-emit the whole line in a single styled span so the highlight is
        // visible even where original spans had nested styling.
        return Line::from(Span::styled(raw, style));
    }
    if raw.trim_start().to_ascii_lowercase().starts_with("**correct") {
        return Line::from(Span::styled(
            raw,
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ));
    }
    line
}

/// Look for "- A:" or "- A )" or "A)" at line start and return "A" if found.
fn leading_option_letter(s: &str) -> Option<String> {
    let trimmed = s.trim_start_matches(|c: char| c == '-' || c == '•' || c.is_whitespace());
    let mut chars = trimmed.chars();
    let first = chars.next()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }
    let next = chars.next()?;
    if next == ':' || next == ')' || next == '.' || next == ' ' {
        // also require an actual reason after, not just "A. "
        let after = trimmed.split_at(2).1.trim_start();
        if after.is_empty() {
            return None;
        }
        return Some(first.to_ascii_uppercase().to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_option_letter_at_start() {
        assert_eq!(leading_option_letter("- A: short reason"), Some("A".into()));
        assert_eq!(leading_option_letter("  • B: reason"), Some("B".into()));
        assert_eq!(leading_option_letter("C) reason here"), Some("C".into()));
        assert_eq!(leading_option_letter("normal sentence"), None);
        assert_eq!(leading_option_letter("- A:"), None); // empty reason is not a bullet match
    }
}
