//! Open the current concept's `.md` in `$EDITOR`, then re-sync.

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use rusqlite::Connection;
use std::io::stdout;
use std::process::Command;

use crate::cli::Cli;
use crate::{db, util, vault};

use super::term::TermGuard;

pub fn edit_concept(
    tg: &mut TermGuard,
    conn: &mut Connection,
    cli: &Cli,
    concept_id: &str,
) -> Result<()> {
    let rel = match db::concept_path(conn, concept_id)? {
        Some(p) => p,
        None => return Ok(()),
    };
    let vault_root = util::vault_path(cli)?;
    let abs = vault_root.join(&rel);

    // Drop our terminal hold so the editor can take over the TTY cleanly.
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = Command::new(&editor).arg(&abs).status();

    // Re-enter raw mode + alt screen.
    let _ = enable_raw_mode();
    let _ = execute!(stdout(), EnterAlternateScreen);
    let _ = tg.term.clear();

    match status {
        Ok(s) if s.success() => {
            // Re-sync so DB reflects the edit.
            let _ = vault::sync::sync_vault(conn, &vault_root)?;
        }
        Ok(_) => { /* editor exited non-zero — leave DB alone */ }
        Err(_) => { /* editor not found — silently ignore for now */ }
    }
    Ok(())
}
