use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "flashcards",
    version,
    about = "TUI flashcards + Obsidian KB for AI/ML/DS interview prep"
)]
pub struct Cli {
    /// Path to the vault root (Obsidian-compatible Markdown).
    /// Defaults to ./vault relative to the binary's working dir.
    #[arg(long, global = true)]
    pub vault: Option<PathBuf>,

    /// Path to the SQLite scheduling DB.
    /// If set, bypasses the profile system entirely.
    #[arg(long, global = true)]
    pub db: Option<PathBuf>,

    /// Profile slug. Defaults to FLASHCARDS_PROFILE env var, then the last-used
    /// profile. If none of those resolve, an interactive picker launches.
    #[arg(long, global = true, env = "FLASHCARDS_PROFILE")]
    pub profile: Option<String>,

    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Review due cards (default if no subcommand is given).
    Review,
    /// Browse all concepts/cards.
    Browse,
    /// Sync the vault into the DB without launching the TUI.
    Sync,
    /// Stamp missing UUIDs in vault notes; flag duplicate-front cards.
    Lint {
        /// Don't modify files; just report what would change.
        #[arg(long)]
        dry_run: bool,
    },
    /// Show retention/leech stats, daily streak, and last 30 days of reviews.
    Stats,
    /// Reset scheduling state. By default keeps the reviews log; use `--scope all` to wipe it too.
    Reset {
        /// `schedule` (default): zero out card_state. `all`: also delete the reviews log.
        #[arg(long, default_value = "schedule")]
        scope: ResetScope,
        /// Only reset cards in this track (e.g. `ml-engineer`).
        #[arg(long)]
        track: Option<String>,
        /// Only reset cards in this topic (e.g. `embeddings-retrieval`).
        #[arg(long)]
        topic: Option<String>,
        /// Skip the confirmation prompt.
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum ResetScope {
    /// Zero out card_state but keep the reviews log (recommended).
    Schedule,
    /// Zero out card_state AND delete matching rows from reviews.
    All,
}

impl Cli {
    pub fn run(mut self) -> Result<()> {
        let cmd = self.cmd.take();
        match cmd {
            None => ui::run_menu(&self),
            Some(Cmd::Review) => ui::run_review(&self),
            Some(Cmd::Browse) => ui::run_browse(&self),
            Some(Cmd::Sync) => {
                let _ = vault::sync(&self)?;
                Ok(())
            }
            Some(Cmd::Lint { dry_run }) => vault::lint_cli(&self, dry_run),
            Some(Cmd::Stats) => ui::run_stats(&self),
            Some(Cmd::Reset {
                scope,
                track,
                topic,
                yes,
            }) => reset::run(&self, scope, track.as_deref(), topic.as_deref(), yes),
        }
    }
}

use crate::{reset, ui, vault};
