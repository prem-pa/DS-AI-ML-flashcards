#!/usr/bin/env python3
"""
Materialize research-agent JSON output into Obsidian vault Markdown notes.

Input: a JSON file (or stdin) containing a list of concept objects, each shaped:

  {
    "slug": "bias-variance-tradeoff",
    "title": "Bias-Variance Tradeoff",
    "track": "data-scientist",
    "topic": "foundational-statistics-probability",
    "difficulty": 2,                      # 1..5 OR "beginner"|"intermediate"|"advanced"
    "tags": ["stats", "generalization"],
    "aliases": ["BV tradeoff"],
    "wikilinks": ["bias", "variance"],
    "sources": [{"url": "...", "label": "..."}],
    "explainer_md": "long-form prose with [[wikilinks]] and ## subsections",
    "cards": [
      {"type": "flip", "front": "...", "back": "..."},
      {"type": "mcq",  "front": "...", "choices": [{"key":"a","text":"...","correct":true}], "back": "..."}
    ]
  }

Output: writes vault/<track>/<topic>/<slug>.md per concept, stamping UUIDs
for the concept and each card. Idempotent: if the file exists with a UUID,
that UUID is reused so scheduling state is never invalidated.
"""

import argparse
import json
import os
import re
import sys
import uuid
from pathlib import Path

import yaml  # PyYAML

DIFFICULTY_MAP = {"beginner": 1, "intermediate": 3, "advanced": 5}


def coerce_difficulty(d):
    if isinstance(d, int):
        return max(1, min(5, d))
    if isinstance(d, str):
        return DIFFICULTY_MAP.get(d.lower().strip(), 3)
    return 3


def load_existing_ids(path: Path):
    """If a note already exists, harvest its concept id and card ids so we reuse them."""
    if not path.exists():
        return None, {}
    text = path.read_text(encoding="utf-8")
    m = re.match(r"^---\n(.*?)\n---\n", text, re.DOTALL)
    if not m:
        return None, {}
    try:
        fm = yaml.safe_load(m.group(1)) or {}
    except yaml.YAMLError:
        return None, {}
    cid = fm.get("id")
    card_ids_by_front = {}
    for c in fm.get("cards") or []:
        front = (c.get("front") or "").strip()
        if c.get("id") and front:
            card_ids_by_front[front] = c["id"]
    return cid, card_ids_by_front


def render_note(concept: dict, existing_id, existing_card_ids):
    cid = existing_id or str(uuid.uuid4())
    cards = []
    for c in concept.get("cards", []):
        front = (c.get("front") or "").strip()
        card = {
            "id": existing_card_ids.get(front) or str(uuid.uuid4()),
            "type": c.get("type", "flip"),
            "front": front,
            "back": (c.get("back") or "").strip(),
        }
        if card["type"] == "mcq":
            card["choices"] = c.get("choices") or []
        cards.append(card)

    fm = {
        "id": cid,
        "title": concept["title"],
        "track": concept["track"],
        "topic": concept["topic"],
        "difficulty": coerce_difficulty(concept.get("difficulty", 3)),
        "tags": concept.get("tags") or [],
        "aliases": concept.get("aliases") or [],
        "sources": concept.get("sources") or [],
        "cards": cards,
    }

    body = (concept.get("explainer_md") or "").strip()
    if not body:
        body = f"# {concept['title']}\n\n_To be expanded._\n"
    elif not body.lstrip().startswith("#"):
        body = f"# {concept['title']}\n\n{body}"

    # Append See also from wikilinks (if not already present in body)
    wl = concept.get("wikilinks") or []
    if wl and "## See also" not in body:
        see_also = "\n\n## See also\n" + "\n".join(f"- [[{w}]]" for w in wl)
        body = body.rstrip() + see_also

    if "## Sources" not in body and fm["sources"]:
        body = body.rstrip() + "\n\n## Sources\nSee frontmatter `sources:`."

    yaml_str = yaml.safe_dump(
        fm, sort_keys=False, allow_unicode=True, default_flow_style=False, width=100
    )
    return f"---\n{yaml_str}---\n\n{body}\n"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("input", nargs="?", help="JSON file (default: stdin)")
    ap.add_argument(
        "--vault",
        default=str(Path(__file__).resolve().parent.parent / "vault"),
        help="Vault root (default: ../vault)",
    )
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    raw = sys.stdin.read() if not args.input else Path(args.input).read_text(encoding="utf-8")
    concepts = json.loads(raw)
    if isinstance(concepts, dict):
        concepts = [concepts]

    vault = Path(args.vault)
    written = 0
    reused = 0
    for c in concepts:
        slug = c["slug"]
        track = c["track"]
        topic = c["topic"]
        rel = Path(track) / topic / f"{slug}.md"
        path = vault / rel
        path.parent.mkdir(parents=True, exist_ok=True)

        existing_id, existing_cards = load_existing_ids(path)
        if existing_id:
            reused += 1
        note = render_note(c, existing_id, existing_cards)

        if args.dry_run:
            print(f"--- WOULD WRITE: {rel} ---")
            print(note[:400], "...")
        else:
            path.write_text(note, encoding="utf-8")
            written += 1
        print(f"  {'(reuse)' if existing_id else '(new)  '} {rel}", file=sys.stderr)

    print(
        f"\n{'Would write' if args.dry_run else 'Wrote'} {written} notes "
        f"({reused} reused existing UUIDs).",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
