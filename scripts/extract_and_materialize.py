#!/usr/bin/env python3
"""
Extract concept JSON arrays from agent .output transcripts (JSONL),
normalize schema variants, and feed them to json_to_vault.py.

Reads every .output file under TASKS_DIR, finds the last assistant message,
extracts the first ```json ... ``` block, parses, normalizes, and writes
one materialized JSON per (track, topic) at data/waves/<track>__<topic>.json.
Then runs json_to_vault.py to render Markdown into vault/.

Schema normalizations handled:
  - cards[].question  -> cards[].front
  - cards[].options as list[str]   -> cards[].choices [{key,text,correct?}]
  - cards[].options as {A:..,B:..} -> cards[].choices [{key,text,correct?}]
  - cards[].answer (key str / int) -> mark matching choice correct
  - cards[].correct (int index)    -> mark matching choice correct
  - cards[].explanation            -> appended to back
  - cards[].side_a / side_b        -> front / back
  - wikilinks "[[Title]]"          -> "title"  (slugified, brackets stripped)
"""

import json
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TASKS_DIR = Path(
    "/private/tmp/claude-502/-Users-prempa-Library-CloudStorage-GoogleDrive-"
    "patelprem96-gmail-com-My-Drive-Projects-flash-cards/"
    "d3771441-2a68-4d7f-bf06-2d82d4dfa4fa/tasks"
)
OUT_DIR = ROOT / "data" / "waves"
OUT_DIR.mkdir(parents=True, exist_ok=True)

JSON_FENCE_RE = re.compile(r"```(?:json)?\s*\n([\s\S]*?)\n```", re.MULTILINE)


def slugify(s: str) -> str:
    s = s.strip().strip("[]").strip()
    s = re.sub(r"[^\w\s-]", "", s).strip().lower()
    s = re.sub(r"[-\s]+", "-", s)
    return s


def extract_last_assistant_text(jsonl_path: Path):
    """Return the concatenated text content of the last assistant message in the JSONL."""
    last_text = None
    try:
        with jsonl_path.open() as f:
            for line in f:
                if not line.strip():
                    continue
                try:
                    rec = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if rec.get("type") != "assistant":
                    continue
                msg = rec.get("message") or {}
                content = msg.get("content") or []
                if isinstance(content, list):
                    parts = [c.get("text", "") for c in content if isinstance(c, dict) and c.get("type") == "text"]
                    txt = "\n".join(p for p in parts if p)
                    if txt.strip():
                        last_text = txt
                elif isinstance(content, str):
                    last_text = content
    except FileNotFoundError:
        return None
    return last_text


def find_json_array(text: str):
    """Find the first JSON array (concept list) in the text."""
    # Try fenced code blocks first
    for m in JSON_FENCE_RE.finditer(text):
        body = m.group(1).strip()
        if body.startswith("["):
            try:
                return json.loads(body)
            except json.JSONDecodeError:
                continue
    # Fallback: try to find a top-level '[' ... ']' span and brute-parse
    depth = 0
    start = None
    for i, ch in enumerate(text):
        if ch == "[":
            if depth == 0:
                start = i
            depth += 1
        elif ch == "]":
            depth -= 1
            if depth == 0 and start is not None:
                candidate = text[start : i + 1]
                try:
                    parsed = json.loads(candidate)
                    if isinstance(parsed, list) and parsed and isinstance(parsed[0], dict):
                        return parsed
                except json.JSONDecodeError:
                    pass
                start = None
    return None


def normalize_wikilinks(wl):
    if not wl:
        return []
    out = []
    for w in wl:
        if not isinstance(w, str):
            continue
        out.append(slugify(w))
    return [w for w in out if w]


def normalize_card(card: dict) -> dict:
    """Coerce any of the seen schema variants into the canonical
    {type, front, back, [choices]} shape."""
    c = dict(card)

    # type defaults
    ctype = (c.get("type") or "flip").lower()
    if ctype not in ("flip", "mcq"):
        ctype = "flip"
    c["type"] = ctype

    # front / back aliases
    front = c.get("front") or c.get("question") or c.get("stem") or c.get("side_a") or ""
    back = c.get("back") or c.get("answer_explanation") or c.get("side_b") or ""

    # MCQ-specific normalization
    if ctype == "mcq":
        # Build canonical choices list
        raw_options = c.get("choices") or c.get("options")
        choices = []

        if isinstance(raw_options, list):
            for idx, opt in enumerate(raw_options):
                key = chr(ord("a") + idx)
                if isinstance(opt, dict):
                    k = (opt.get("key") or key).lower()
                    text = opt.get("text") or opt.get("label") or ""
                    correct = bool(opt.get("correct"))
                    choices.append({"key": k, "text": str(text), "correct": correct})
                else:
                    choices.append({"key": key, "text": str(opt), "correct": False})
        elif isinstance(raw_options, dict):
            for k, v in raw_options.items():
                choices.append({"key": str(k).lower(), "text": str(v), "correct": False})

        # Mark correct choice from `answer` (string key or index) or `correct` (index)
        ans = c.get("answer")
        idx_key = c.get("correct") if isinstance(c.get("correct"), int) else None
        if choices:
            if isinstance(ans, int):
                if 0 <= ans < len(choices):
                    choices[ans]["correct"] = True
            elif isinstance(ans, str):
                ak = ans.strip().lower()
                # try direct key match
                matched = False
                for ch in choices:
                    if ch["key"] == ak:
                        ch["correct"] = True
                        matched = True
                        break
                if not matched:
                    # try matching by text content
                    for ch in choices:
                        if ch["text"].strip().lower() == ans.strip().lower():
                            ch["correct"] = True
                            break
            elif idx_key is not None and 0 <= idx_key < len(choices):
                choices[idx_key]["correct"] = True

        # Fold any standalone `explanation` into back
        explanation = c.get("explanation")
        if explanation and explanation not in back:
            back = (back + ("\n\n" if back else "") + explanation).strip()

        c["choices"] = choices

    out = {
        "type": ctype,
        "front": str(front).strip(),
        "back": str(back).strip(),
    }
    if ctype == "mcq":
        out["choices"] = c.get("choices") or []
    return out


def normalize_concept(concept: dict) -> dict:
    if not isinstance(concept, dict):
        return None
    if not concept.get("slug") or not concept.get("track") or not concept.get("topic"):
        return None
    out = dict(concept)
    out["wikilinks"] = normalize_wikilinks(concept.get("wikilinks") or [])
    out["cards"] = [normalize_card(c) for c in (concept.get("cards") or []) if isinstance(c, dict)]
    return out


def main():
    transcripts = sorted(TASKS_DIR.glob("*.output"))
    if not transcripts:
        print(f"No .output files in {TASKS_DIR}", file=sys.stderr)
        sys.exit(1)

    by_topic = defaultdict(list)  # (track, topic) -> [concept]
    skipped = []
    extracted_files = 0
    total_concepts = 0

    for t in transcripts:
        # Resolve symlink to actual JSONL
        try:
            actual = t.resolve()
        except OSError:
            continue
        if not actual.exists() or actual.suffix != ".jsonl":
            continue

        text = extract_last_assistant_text(actual)
        if not text:
            skipped.append((t.name, "no assistant text"))
            continue

        arr = find_json_array(text)
        if not arr:
            skipped.append((t.name, "no JSON array"))
            continue

        local_concepts = 0
        for raw in arr:
            normalized = normalize_concept(raw)
            if normalized is None:
                continue
            key = (normalized["track"], normalized["topic"])
            by_topic[key].append(normalized)
            local_concepts += 1
        if local_concepts:
            extracted_files += 1
            total_concepts += local_concepts

    print(
        f"Parsed {extracted_files}/{len(transcripts)} transcripts; {total_concepts} concepts across {len(by_topic)} (track, topic) groups.",
        file=sys.stderr,
    )
    if skipped:
        print(f"Skipped {len(skipped)} transcripts (likely the 1 still-running or non-agent files).", file=sys.stderr)

    # Dedupe within each (track, topic) by slug, keep first (waves are non-conflicting in practice)
    written = 0
    for (track, topic), concepts in sorted(by_topic.items()):
        seen = {}
        for c in concepts:
            seen.setdefault(c["slug"], c)
        merged = list(seen.values())

        out_path = OUT_DIR / f"{track}__{topic}.json"
        out_path.write_text(json.dumps(merged, indent=2, ensure_ascii=False), encoding="utf-8")
        written += 1
        print(f"  wrote {out_path.relative_to(ROOT)} ({len(merged)} concepts)", file=sys.stderr)

    print(f"\nWrote {written} per-topic JSON files to {OUT_DIR.relative_to(ROOT)}/", file=sys.stderr)

    # Run materializer for each
    print("\nMaterializing to vault...\n", file=sys.stderr)
    materializer = ROOT / "scripts" / "json_to_vault.py"
    total_notes = 0
    for json_file in sorted(OUT_DIR.glob("*.json")):
        result = subprocess.run(
            ["python3", str(materializer), str(json_file)],
            capture_output=True,
            text=True,
        )
        # Print materializer stderr (one note per line + summary)
        sys.stderr.write(result.stderr)
        if result.returncode != 0:
            print(f"  ERROR materializing {json_file.name}: {result.stdout[:200]}", file=sys.stderr)
        else:
            # Count "(new)" / "(reuse)" lines
            total_notes += sum(1 for line in result.stderr.splitlines() if "(new)" in line or "(reuse)" in line)

    print(f"\nDone. ~{total_notes} notes processed.", file=sys.stderr)


if __name__ == "__main__":
    main()
