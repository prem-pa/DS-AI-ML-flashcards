//! Lightweight markdown → `Vec<Line>` for the TUI.
//!
//! Handles inline `**bold**`, `*em*`, `` `code` ``, `[text](url)`, headings,
//! lists, paragraphs, and `[[wikilinks]]`. Block-level fenced code is handled
//! one level up in `render::render` (because we want syntect for those);
//! anything else falls through to text with math-unicode pretty-printing.

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use super::math::prettify_math;

#[derive(Default)]
struct Renderer {
    out: Vec<Line<'static>>,
    line: Vec<Span<'static>>,
    style: Style,
    bold: u32,
    italic: u32,
    code_inline: u32,
    in_link: Option<String>, // current link URL
    list_stack: Vec<ListKind>,
    heading: u32,
}

#[derive(Clone, Copy)]
enum ListKind {
    Bullet,
    Ordered(u64),
}

impl Renderer {
    fn flush_line(&mut self) {
        let line = std::mem::take(&mut self.line);
        self.out.push(Line::from(line));
    }

    fn push_text(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        let pretty = if self.code_inline > 0 {
            // don't math-substitute inside `code`
            s.to_string()
        } else {
            prettify_math(s)
        };
        // split on '\n' so newlines flush lines
        let mut first = true;
        for piece in pretty.split('\n') {
            if !first {
                self.flush_line();
            }
            if !piece.is_empty() {
                self.line
                    .push(Span::styled(piece.to_string(), self.current_style()));
            }
            first = false;
        }
    }

    fn current_style(&self) -> Style {
        let mut s = self.style;
        if self.bold > 0 {
            s = s.add_modifier(Modifier::BOLD);
        }
        if self.italic > 0 {
            s = s.add_modifier(Modifier::ITALIC);
        }
        if self.code_inline > 0 {
            s = s.fg(Color::Magenta).add_modifier(Modifier::DIM);
        }
        if self.heading > 0 {
            s = s
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);
        }
        if self.in_link.is_some() {
            s = s
                .fg(Color::Blue)
                .add_modifier(Modifier::UNDERLINED);
        }
        s
    }

    fn list_prefix(&self) -> String {
        if self.list_stack.is_empty() {
            return String::new();
        }
        let depth = self.list_stack.len() - 1;
        let indent = "  ".repeat(depth);
        match self.list_stack.last() {
            Some(ListKind::Bullet) => format!("{}• ", indent),
            Some(ListKind::Ordered(n)) => format!("{}{}. ", indent, n),
            None => String::new(),
        }
    }
}

/// Convert a markdown block (text between `\`\`\`` fences in the outer renderer)
/// into ratatui Lines. Inline-only nesting; nested ordered lists not deep.
pub fn render(md: &str) -> Vec<Line<'static>> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md, opts);
    let mut r = Renderer::default();

    for ev in parser {
        match ev {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {}
                Tag::Heading { level, .. } => {
                    r.heading = match level {
                        HeadingLevel::H1 => 3,
                        HeadingLevel::H2 => 2,
                        _ => 1,
                    };
                }
                Tag::Strong => r.bold += 1,
                Tag::Emphasis => r.italic += 1,
                Tag::Strikethrough => {
                    r.style = r.style.add_modifier(Modifier::CROSSED_OUT);
                }
                Tag::List(start) => {
                    r.list_stack.push(match start {
                        Some(n) => ListKind::Ordered(n),
                        None => ListKind::Bullet,
                    });
                }
                Tag::Item => {
                    if !r.line.is_empty() {
                        r.flush_line();
                    }
                    let prefix = r.list_prefix();
                    if !prefix.is_empty() {
                        r.line.push(Span::styled(
                            prefix,
                            Style::default().fg(Color::DarkGray),
                        ));
                    }
                }
                Tag::Link { dest_url, .. } => {
                    r.in_link = Some(dest_url.to_string());
                }
                Tag::CodeBlock(_) => { /* outer renderer extracts ``` blocks */ }
                _ => {}
            },
            Event::End(end) => match end {
                TagEnd::Paragraph => {
                    r.flush_line();
                    r.out.push(Line::raw(""));
                }
                TagEnd::Heading(_) => {
                    r.heading = 0;
                    r.flush_line();
                    r.out.push(Line::raw(""));
                }
                TagEnd::Strong => r.bold = r.bold.saturating_sub(1),
                TagEnd::Emphasis => r.italic = r.italic.saturating_sub(1),
                TagEnd::Strikethrough => {
                    r.style = r.style.remove_modifier(Modifier::CROSSED_OUT);
                }
                TagEnd::List(_) => {
                    r.list_stack.pop();
                    if r.list_stack.is_empty() && !r.line.is_empty() {
                        r.flush_line();
                    }
                }
                TagEnd::Item => {
                    if let Some(ListKind::Ordered(n)) = r.list_stack.last_mut() {
                        *n += 1;
                    }
                    r.flush_line();
                }
                TagEnd::Link => {
                    if let Some(url) = r.in_link.take() {
                        // append url in dim text after the link label
                        r.line.push(Span::styled(
                            format!(" ({})", url),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ));
                    }
                }
                _ => {}
            },
            Event::Text(t) => r.push_text(&t),
            Event::Code(c) => {
                r.code_inline += 1;
                r.push_text(&c);
                r.code_inline = r.code_inline.saturating_sub(1);
            }
            Event::SoftBreak => r.line.push(Span::raw(" ")),
            Event::HardBreak => r.flush_line(),
            Event::Rule => {
                r.flush_line();
                r.out.push(Line::from(Span::styled(
                    "─".repeat(40),
                    Style::default().fg(Color::DarkGray),
                )));
            }
            _ => {}
        }
    }
    if !r.line.is_empty() {
        r.flush_line();
    }
    // trim trailing empty lines
    while r.out.last().map(|l| l.spans.is_empty()).unwrap_or(false) {
        r.out.pop();
    }
    r.out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat(lines: &[Line<'static>]) -> String {
        lines
            .iter()
            .map(|l| {
                l.spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn bold_strips_stars() {
        let out = render("**Bi-encoder**: encodes once.");
        assert_eq!(flat(&out), "Bi-encoder: encodes once.");
    }

    #[test]
    fn list_renders_bullets() {
        let out = render("- alpha\n- beta\n- gamma");
        let s = flat(&out);
        assert!(s.contains("• alpha"), "{s}");
        assert!(s.contains("• beta"), "{s}");
    }

    #[test]
    fn link_text_with_url() {
        let out = render("see [the paper](https://arxiv.org/abs/1234.5678)");
        let s = flat(&out);
        assert!(s.contains("the paper"));
        assert!(s.contains("https://arxiv.org/abs/1234.5678"));
    }

    #[test]
    fn math_inside_paragraph() {
        let out = render(r"loss is $\sum_i x_i^2$");
        let s = flat(&out);
        assert!(s.contains('∑'), "{s}");
        assert!(s.contains('²'), "{s}");
    }
}
