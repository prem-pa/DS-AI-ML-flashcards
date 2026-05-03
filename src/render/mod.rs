//! Card-text renderer for the TUI.
//!
//! Pipeline per text block:
//!   1. detect ``` fenced code → keep as code segment
//!   2. for non-code text: replace common LaTeX/math constructs with Unicode
//!      so equations render as `∑_i p_i log p_i` instead of `\sum_i p_i \log p_i`
//!   3. syntax-highlight code blocks via syntect
//! Output is `ratatui::text::Text<'static>` ready to drop into a Paragraph.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use std::sync::OnceLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as SynStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

mod markdown;
mod math;

pub use math::prettify_math;

struct Engine {
    syntax_set: SyntaxSet,
    theme: syntect::highlighting::Theme,
}

fn engine() -> &'static Engine {
    static E: OnceLock<Engine> = OnceLock::new();
    E.get_or_init(|| {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        // base16-ocean.dark is dark-friendly; falls back to whatever's first if missing.
        let theme = theme_set
            .themes
            .get("base16-ocean.dark")
            .or_else(|| theme_set.themes.values().next())
            .cloned()
            .expect("syntect default themes");
        Engine { syntax_set, theme }
    })
}

/// Render a card text block (front or back) into ratatui Lines, with math
/// pretty-printing on prose and syntax highlighting on fenced code blocks.
pub fn render(text: &str) -> Text<'static> {
    let mut out: Vec<Line<'static>> = Vec::new();
    let mut in_code = false;
    let mut code_lang: Option<String> = None;
    let mut buf: Vec<String> = Vec::new();

    let flush_prose = |buf: &mut Vec<String>, out: &mut Vec<Line<'static>>| {
        if buf.is_empty() {
            return;
        }
        let joined = buf.join("\n");
        // markdown::render also runs prettify_math on each text run, so we
        // route everything through it instead of emitting raw lines.
        for line in markdown::render(&joined) {
            out.push(line);
        }
        buf.clear();
    };

    let flush_code = |buf: &mut Vec<String>,
                      lang: Option<&str>,
                      out: &mut Vec<Line<'static>>| {
        if buf.is_empty() {
            return;
        }
        let joined = buf.join("\n");
        let lines = highlight_code(&joined, lang);
        out.extend(lines);
        buf.clear();
    };

    for line in text.split('\n') {
        if let Some(rest) = line.trim_start().strip_prefix("```") {
            if !in_code {
                flush_prose(&mut buf, &mut out);
                in_code = true;
                let lang = rest.trim();
                code_lang = if lang.is_empty() { None } else { Some(lang.to_string()) };
            } else {
                flush_code(&mut buf, code_lang.as_deref(), &mut out);
                in_code = false;
                code_lang = None;
            }
            continue;
        }
        buf.push(line.to_string());
    }
    if in_code {
        flush_code(&mut buf, code_lang.as_deref(), &mut out);
    } else {
        flush_prose(&mut buf, &mut out);
    }

    Text::from(out)
}

fn highlight_code(code: &str, lang: Option<&str>) -> Vec<Line<'static>> {
    let eng = engine();
    let syntax = lang
        .and_then(|l| eng.syntax_set.find_syntax_by_token(l))
        .or_else(|| eng.syntax_set.find_syntax_by_first_line(code))
        .unwrap_or_else(|| eng.syntax_set.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, &eng.theme);
    let mut out: Vec<Line<'static>> = Vec::new();

    // visual margin so code blocks read as a unit even without bg highlighting
    out.push(Line::from(Span::styled(
        "─── code ───".to_string(),
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM),
    )));
    for raw in LinesWithEndings::from(code) {
        let ranges: Vec<(SynStyle, &str)> = h
            .highlight_line(raw, &eng.syntax_set)
            .unwrap_or_default();
        let spans: Vec<Span<'static>> = ranges
            .into_iter()
            .map(|(st, txt)| {
                let style = Style::default().fg(Color::Rgb(
                    st.foreground.r,
                    st.foreground.g,
                    st.foreground.b,
                ));
                Span::styled(txt.trim_end_matches('\n').to_string(), style)
            })
            .collect();
        out.push(Line::from(spans));
    }
    out.push(Line::from(Span::styled(
        "────────────".to_string(),
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM),
    )));
    out
}
