# Flash Cards

Terminal flashcard app + Obsidian knowledge base for AI / ML / Data Science interview prep.

- **Vault** (`vault/`) — Obsidian-compatible Markdown KB. One note per concept, with `[[wikilinks]]`, citations, and long-form prose. Source of truth for all card content.
- **App** (`src/`) — Rust TUI built on `ratatui`. FSRS spaced repetition. Flip cards (self-graded 1-4) + MCQ (auto-graded) drawn from the vault on every launch.

Coverage: Data Scientist, ML Engineer, AI/LLM Engineer, Research-leaning, plus verticals (CV, recsys, time-series, speech). Beginner → advanced. Currently **406 concepts / 1786 cards** indexed across 37 topic-areas.

## Install

Requires Rust 1.85+. If you don't have it:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
```

Then from the project root:

```sh
cargo build --release      # binary lands at target/release/flashcards
```

If your project lives on a cloud-synced drive (Google Drive, iCloud, Dropbox), point the build out of the synced dir to avoid corrupted incremental artifacts:

```sh
export CARGO_TARGET_DIR="$HOME/.cache/flashcards-target"
cargo build --release      # now at $CARGO_TARGET_DIR/release/flashcards
```

(or write the same `target-dir` line into a local `.cargo/config.toml`; it's gitignored.)

Optionally symlink the binary onto your PATH:

```sh
ln -s "$CARGO_TARGET_DIR/release/flashcards" ~/.local/bin/flashcards
```

## Usage

```sh
flashcards            # main menu (continue / review / pick topic / browse / stats / switch profile)
flashcards review     # skip the menu; jump straight into the due queue
flashcards browse     # split-pane card browser with fuzzy search
flashcards stats      # streak, grade distribution, leeches, weakest topics
flashcards sync       # re-index the vault into the DB without launching the TUI
flashcards lint       # stamp missing UUIDs, flag near-duplicate front text
flashcards reset      # zero out scheduling state (with confirmation)
```

Global flags:

- `--vault <PATH>` — override the vault root (default `./vault`)
- `--profile <SLUG>` — pick a specific profile; creates one with that slug if it doesn't exist
- `--db <PATH>` — bypass the profile system entirely and use a single SQLite file
- `FLASHCARDS_PROFILE=<slug>` — same as `--profile` via env var

If you don't pass any of the above, the app resolves a profile in this order: env var → last-used profile → interactive picker.

Per-profile DBs live at `~/Library/Application Support/flashcards/profiles/<slug>.db` (macOS) / `$XDG_DATA_HOME/flashcards/profiles/<slug>.db` (Linux). They are **outside** the Google-Drive-synced project dir on purpose — SQLite + cloud sync corrupts incremental writes.

### Profiles

Each profile gets its own DB file, so two people sharing the same Obsidian vault can keep their FSRS state isolated. The first time you run `flashcards`, you land in the picker:

- existing profiles are listed by `display_name` + slug + last-active time
- press `n` to create a new one — the create screen shows 8 quirky username suggestions (`drowsy-otter-42`, `zesty-axolotl-19`, ...); press `r` to reroll, `1`-`8` to use a suggestion, or just type a custom name
- press `d` on a profile to delete its DB
- the most-recently-used profile is remembered in `~/Library/Application Support/flashcards/last_profile.txt`, so subsequent launches go straight to the menu

The pool of generated names is huge (~80 adjectives × ~80 nouns × 100 numeric suffixes ≈ 700k+ combinations), so collisions across profiles are unlikely.

### Main menu

Launching `flashcards` (no subcommand) opens a menu with:

| Key | Action                                                                 |
|-----|------------------------------------------------------------------------|
| `c` | Continue last session — reuses the previous session's track/topic/difficulty scope |
| `d` | Review all due — global queue, no scope                                |
| `t` | Pick topic / difficulty — three-stage picker (track → topic → difficulty) |
| `b` | Browse cards                                                            |
| `s` | Stats                                                                   |
| `p` | Switch profile (relaunches the picker)                                  |
| `q` | Quit                                                                    |

### Topic / difficulty picker

Selecting `t` from the menu opens a breadcrumb-style picker:

1. **Track** — `data-scientist`, `ml-engineer`, `ai-llm-engineer`, `research-leaning`, `verticals`, or "all tracks"
2. **Topic** — topics within the chosen track, with card counts
3. **Difficulty** — `all`, `beginner (1-2)`, `intermediate (3)`, or `advanced (4-5)`. Maps to the `difficulty:` field in each note's frontmatter.

`Esc` walks back up a stage; `q` exits to the menu.

### Review key bindings

| Card type | State    | Keys                                          |
|-----------|----------|-----------------------------------------------|
| flip      | hidden   | `space` / `enter` / `f` reveal                |
| flip      | revealed | `1` again, `2` hard, `3` good, `4` easy       |
| mcq       | unpicked | `a`–`d` (or whichever keys the choices use)   |
| mcq       | picked   | `space` / `enter` / `n` advance               |
| any       | any      | `e` open concept's `.md` in `$EDITOR` then re-sync |
| any       | any      | `q` / `Esc` quit                              |

### Browse key bindings

| Key                | Action                                          |
|--------------------|-------------------------------------------------|
| `j` / `k` or arrows| move selection                                  |
| `d` / `u`          | half-page down/up                               |
| `g` / `G`          | top / bottom                                    |
| `/`                | enter fuzzy filter mode (nucleo-matcher)        |
| (in filter mode)   | type to filter live; `enter` apply, `esc` clear |
| `q` / `Esc`        | quit                                            |

### Resetting progress

```sh
flashcards reset                                       # all cards; keeps reviews log
flashcards reset --scope all                           # also wipes the reviews log
flashcards reset --track ml-engineer                   # one track only
flashcards reset --topic embeddings-retrieval          # one topic only
flashcards reset --track ai-llm-engineer --topic agents-tool-use -y
```

Prints a pre-flight summary (cards in scope, cards with prior reviews, reviews log rows) and prompts for `yes` before touching anything. `-y` / `--yes` skips the prompt.

`--scope schedule` (default) zeros every FSRS field on `card_state` so cards re-enter the `New` state, due now. `--scope all` additionally deletes matching rows from `reviews`, so the stats screen forgets that history too.

### Stats screen

`flashcards stats` shows, in one screen:

- active / suspended cards, FSRS state breakdown (new / learning / review / relearning)
- total reviews + grade distribution (again / hard / good / easy)
- queue counts: due now / due today / due in 7 days
- daily streak
- last 30 days as a horizontal bar chart
- top leeches (cards with the most lapses)
- weakest topics (highest mean lapses per card)

## Layout

```
vault/                          # Obsidian KB — open this directory in Obsidian
  data-scientist/
  ml-engineer/
  ai-llm-engineer/
  research-leaning/
  verticals/
  _templates/concept.md         # Templater stub for new concepts
src/
  cli.rs                        # clap CLI / subcommand dispatch
  db/                           # SQLite + migrations + queries
    migrations/0001_init.sql
  vault/
    parse.rs                    # frontmatter + wikilink parser
    scan.rs                     # walkdir + blake3 content hashing
    sync.rs                     # idempotent upsert; preserves FSRS state
    lint.rs                     # UUID stamper + Jaccard dup detection
  sched/                        # rs-fsrs adapter
  render/                       # math-unicode + syntect code blocks
    math.rs                     # LaTeX -> Unicode pretty-printer
  ui/
    review.rs / browse.rs / stats.rs
    edit.rs                     # spawns $EDITOR, then re-syncs
    term.rs                     # raw-mode lifecycle guard
  util.rs                       # path resolution
data/
  taxonomy.json                 # 450-concept taxonomy from research phase
  waves/                        # per-topic JSON output from research agents
scripts/
  json_to_vault.py              # research-agent JSON -> vault Markdown
  extract_and_materialize.py    # walks agent transcripts, normalizes, materializes
tests/
  sync_roundtrip.rs             # editing card text must NOT reset FSRS state
examples/
  math_demo.rs                  # cargo run --example math_demo
```

## Adding new content

The vault is the source of truth, so you can either:

1. **Edit in Obsidian.** Open any `.md` and edit. The `cards:` array in YAML frontmatter holds card text + ids. On the next `flashcards review` (or `flashcards sync`), changes flow into the DB. **Card UUIDs are the join key to FSRS state — preserve them across edits.** A roundtrip test (`tests/sync_roundtrip.rs`) guards this invariant.

2. **Add a fresh note.** Use the Templater stub at `vault/_templates/concept.md` (or copy any existing note). Run `flashcards lint` to stamp any missing UUIDs.

3. **Bulk import from research output.** Drop a per-topic JSON list at `data/waves/<track>__<topic>.json` (schema in `scripts/json_to_vault.py` docstring) and run:
   ```sh
   python3 scripts/json_to_vault.py data/waves/<file>.json
   ```
   The materializer reuses any UUIDs already in target notes, so re-running is idempotent.

## How sync stays safe

`vault::sync::sync_vault` runs in a single SQLite transaction:

1. `walkdir` the vault, blake3-hash each `.md`.
2. Diff hashes against `concepts.content_hash`.
3. For changed/new files: upsert concept + cards (UPSERT on uuid, never `INSERT OR REPLACE` which would cascade-delete `card_state`).
4. `INSERT INTO card_state ON CONFLICT DO NOTHING` — initial scheduler row created once, never overwritten.
5. Cards present in DB but missing from frontmatter → `suspended = 1` (preserves the append-only `reviews` log for stats).
6. Concepts whose files vanished from disk → soft-deleted (`deleted_at` set), their cards suspended.
7. Resolve `[[wikilinks]]` → `concept_links(src_id, dst_id)` for the graph.

## Dev

```sh
cargo build              # debug
cargo build --release    # 5.2 MB optimized binary
cargo test               # 9 unit + 2 integration tests
cargo run --example math_demo   # demo the LaTeX -> Unicode pass
```

Tech stack:

- `ratatui` 0.30 + `crossterm` 0.29 — TUI
- `rusqlite` 0.39 (`bundled` feature) — SQLite, no system libs
- `rs-fsrs` 1 — FSRS scheduler
- `serde_yml` + hand-rolled split for frontmatter
- `walkdir` + `blake3` — content-hash-based diff
- `nucleo-matcher` 0.3 — Helix-grade fuzzy search
- `syntect` 5.3 (`default-fancy`) — code-block highlighting
- `regex` — wikilinks + math token table
- `clap` 4 derive — CLI

## Status

- Phase 1 (KB content) — done. 406 notes / 1786 cards across 37 topic-areas.
- Phase 2 (Rust TUI v1) — done. review, browse, sync, lint, all green tests.
- Phase 3 (polish) — done. stats screen, fuzzy search, math-unicode, syntect code blocks, edit-in-`$EDITOR`.
- Phase 4 (multi-profile) — done. per-profile DBs, quirky-name suggester, main menu, sessions table, "continue last session", topic/difficulty picker.

Open it in Obsidian for graph view + backlinks, or run `flashcards` for the menu.
