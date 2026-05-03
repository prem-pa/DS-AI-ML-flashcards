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
cargo build --release
```

The binary lands at `~/.cache/flashcards-target/release/flashcards` (the `.cargo/config.toml` redirects `target/` outside the Drive-synced project dir to avoid sync corruption).

Optionally symlink it onto your PATH:

```sh
ln -s ~/.cache/flashcards-target/release/flashcards ~/.local/bin/flashcards
```

## Usage

```sh
flashcards            # default: review due cards
flashcards review     # same
flashcards browse     # split-pane card browser with fuzzy search
flashcards stats      # streak, grade distribution, leeches, weakest topics
flashcards sync       # re-index the vault into the DB without launching the TUI
flashcards lint       # stamp missing UUIDs, flag near-duplicate front text
flashcards reset      # zero out scheduling state (with confirmation)
```

Both `--vault <PATH>` and `--db <PATH>` are global overrides; defaults are `./vault` and `~/Library/Application Support/flashcards/flashcards.db` (macOS) / `$XDG_DATA_HOME/flashcards/flashcards.db` (Linux).

The DB intentionally lives **outside** the Google-Drive-synced project dir — SQLite + cloud sync corrupts incremental writes.

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

Open it in Obsidian for graph view + backlinks, or run `flashcards` for the review queue.
