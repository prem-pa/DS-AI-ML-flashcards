use anyhow::Result;

use crate::cli::Cli;
use crate::{db, util, vault};

mod browse;
mod edit;
mod review;
mod stats;
mod term;

pub fn run_review(cli: &Cli) -> Result<()> {
    let mut conn = open_and_sync(cli)?;
    review::run(&mut conn, cli)
}

pub fn run_browse(cli: &Cli) -> Result<()> {
    let conn = open_and_sync(cli)?;
    browse::run(&conn, cli)
}

pub fn run_stats(cli: &Cli) -> Result<()> {
    let conn = open_and_sync(cli)?;
    stats::run(&conn)
}

/// Open the DB and run a vault sync so the queue reflects the latest notes.
fn open_and_sync(cli: &Cli) -> Result<rusqlite::Connection> {
    let db_path = util::db_path(cli)?;
    let mut conn = db::open(&db_path)?;
    let vault_root = util::vault_path(cli)?;
    let _ = vault::sync::sync_vault(&mut conn, &vault_root)?;
    Ok(conn)
}
