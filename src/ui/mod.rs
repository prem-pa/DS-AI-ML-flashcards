use anyhow::{anyhow, Result};
use rusqlite::Connection;

use crate::cli::Cli;
use crate::db::Scope;
use crate::{db, profile, util, vault};

mod browse;
mod edit;
mod menu;
mod picker;
mod review;
mod stats;
mod term;
mod topic_pick;

use term::TermGuard;

/// `flashcards` (no subcommand): show the main menu and stay there until quit.
pub fn run_menu(cli: &Cli) -> Result<()> {
    let (mut conn, mut display) = resolve_and_open(cli, /*force_picker=*/ false)?;
    sync_quietly(&mut conn, cli)?;
    let mut tg = TermGuard::enter()?;
    loop {
        let action = menu::run(&mut tg, &conn, &display)?;
        match action {
            menu::Action::Quit => return Ok(()),
            menu::Action::ReviewAllDue => {
                review::run_with_scope(&mut conn, cli, Scope::default(), Some(&mut tg))?;
            }
            menu::Action::ContinueLast(s) => {
                review::run_with_scope(&mut conn, cli, s.scope.clone(), Some(&mut tg))?;
            }
            menu::Action::PickTopic => {
                if let Some(scope) = topic_pick::run(&mut tg, &conn)? {
                    review::run_with_scope(&mut conn, cli, scope, Some(&mut tg))?;
                }
            }
            menu::Action::Browse => {
                browse::run_with_term(&conn, cli, Some(&mut tg))?;
            }
            menu::Action::Stats => {
                stats::run_with_term(&conn, Some(&mut tg))?;
            }
            menu::Action::SwitchProfile => {
                drop(tg);
                let (new_conn, new_display) = resolve_and_open(cli, /*force_picker=*/ true)?;
                conn = new_conn;
                display = new_display;
                sync_quietly(&mut conn, cli)?;
                tg = TermGuard::enter()?;
            }
        }
    }
}

pub fn run_review(cli: &Cli) -> Result<()> {
    let (mut conn, _) = resolve_and_open(cli, false)?;
    sync_quietly(&mut conn, cli)?;
    review::run(&mut conn, cli)
}

pub fn run_browse(cli: &Cli) -> Result<()> {
    let (mut conn, _) = resolve_and_open(cli, false)?;
    sync_quietly(&mut conn, cli)?;
    browse::run(&conn, cli)
}

pub fn run_stats(cli: &Cli) -> Result<()> {
    let (mut conn, _) = resolve_and_open(cli, false)?;
    sync_quietly(&mut conn, cli)?;
    stats::run(&conn)
}

/// Resolution order for the active profile / DB:
///   1. `--db <PATH>`        — legacy escape hatch, no profile system
///   2. `--profile <slug>`   — explicit; create if missing
///   3. FLASHCARDS_PROFILE   — wired through clap into `cli.profile`
///   4. last_profile.txt     — most-recently-used profile
///   5. force_picker=true OR no resolution → interactive picker
fn resolve_and_open(cli: &Cli, force_picker: bool) -> Result<(Connection, String)> {
    if !force_picker {
        if let Some(path) = &cli.db {
            let conn = db::open(path)?;
            return Ok((conn, path.display().to_string()));
        }
        if let Some(slug_in) = &cli.profile {
            let slug = profile::slugify(slug_in);
            if slug.is_empty() {
                return Err(anyhow!("--profile slug is empty after slugifying"));
            }
            let conn = profile::open_or_create(&slug, slug_in)?;
            let display = display_name_or_slug(&conn, &slug);
            return Ok((conn, display));
        }
        if let Some(last) = profile::read_last_used()? {
            if profile::db_path_for(&last)?.exists() {
                let conn = profile::open_or_create(&last, &last)?;
                let display = display_name_or_slug(&conn, &last);
                return Ok((conn, display));
            }
        }
    }

    // Picker path
    let mut tg = TermGuard::enter()?;
    let outcome = picker::run(&mut tg)?;
    drop(tg);
    match outcome {
        picker::PickerOutcome::Quit => Err(anyhow!("aborted at profile picker")),
        picker::PickerOutcome::Picked { slug, display_name } => {
            let conn = profile::open_or_create(&slug, &display_name)?;
            let display = display_name_or_slug(&conn, &slug);
            Ok((conn, display))
        }
    }
}

fn display_name_or_slug(conn: &Connection, slug: &str) -> String {
    let n: Option<String> = conn
        .query_row(
            "SELECT value FROM profile_meta WHERE key = 'display_name'",
            [],
            |r| r.get(0),
        )
        .optional()
        .ok()
        .flatten();
    n.unwrap_or_else(|| slug.to_string())
}

fn sync_quietly(conn: &mut Connection, cli: &Cli) -> Result<()> {
    let vault_root = util::vault_path(cli)?;
    let _ = vault::sync::sync_vault(conn, &vault_root)?;
    Ok(())
}

use rusqlite::OptionalExtension;
