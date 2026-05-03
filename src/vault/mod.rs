use anyhow::Result;

use crate::cli::Cli;
use crate::util;

pub mod lint;
pub mod parse;
pub mod scan;
pub mod sync;

pub struct SyncReport {
    pub changed: usize,
    pub new_concepts: usize,
    pub suspended_cards: usize,
}

pub fn sync(cli: &Cli) -> Result<SyncReport> {
    let vault = util::vault_path(cli)?;
    let mut conn = util::open_db(cli)?;
    eprintln!("vault root: {}", vault.display());

    let report = sync::sync_vault(&mut conn, &vault)?;
    eprintln!(
        "sync: {} new, {} changed, {} suspended cards",
        report.new_concepts, report.changed, report.suspended_cards
    );
    Ok(report)
}

pub fn lint_cli(cli: &Cli, dry_run: bool) -> Result<()> {
    let vault = util::vault_path(cli)?;
    let report = lint::run(&vault, dry_run)?;
    lint::print_report(&report, dry_run, &vault);
    Ok(())
}
