//! Terminal lifecycle helper. Enters raw mode + alternate screen on construction,
//! restores on drop (so panics don't leave the user's terminal in a weird state).

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{stdout, Stdout};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub struct TermGuard {
    pub term: Tui,
}

impl TermGuard {
    pub fn enter() -> Result<Self> {
        enable_raw_mode()?;
        let mut out = stdout();
        execute!(out, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(out);
        let term = Terminal::new(backend)?;
        Ok(Self { term })
    }
}

impl Drop for TermGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        let _ = self.term.show_cursor();
    }
}
