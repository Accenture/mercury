#!/usr/bin/env python3
"""Link-integrity check for docs/llms.txt — the AI-agent documentation map.

llms.txt is the map an AI agent follows to find the right page instead of reading
`crates/`. A path that does not resolve sends the agent nowhere, and the failure is
silent: nothing renders it, so no site build catches it. This is that guard.

Two path conventions live in the file, and this enforces both as they are actually
written (the header once claimed the wrong one, which broke every link at once):

  1. markdown links  - `[text](guides/foo.md)` resolve from **docs/**, the file's own
                       directory. External http(s) links are skipped (no network in CI).
  2. backticked paths - `` `docs/INCREMENTS.md` ``, `` `draft-design-specs/` `` resolve
                       from the **repo root**. Only tokens containing "/", ending in
                       ".md" or "/", and free of ":" are treated as paths — so identifiers
                       (`graph.task`), a bare filename (`ai-companion-sync.md`) and URI
                       schemes (`classpath:/`, `file:/`, `flow://`) are never guessed at.

Deliberately NOT a coverage check: this repo's llms.txt is a *curated* agent map, not a
full site map (maintainer ruling, 2026-09-04), so "every guide must be listed" would be
wrong here. The Java repo, whose llms.txt is a full site map, enforces coverage instead.

Exit 0 = clean; exit 1 = broken links. Run from anywhere:
    python3 scripts/check-llms-links.py [--root PATH]
"""
import argparse
import re
import sys
from pathlib import Path

MD_LINK = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
BACKTICKED = re.compile(r"`([^`]+)`")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=str(Path(__file__).resolve().parent.parent),
                    help="repo root (default: parent of scripts/)")
    args = ap.parse_args()
    root = Path(args.root)
    llms = root / "docs" / "llms.txt"
    if not llms.exists():
        print(f"docs/llms.txt not found under {root}")
        return 1

    text = llms.read_text(encoding="utf-8")
    errors: list[str] = []
    checked = 0

    # 1. markdown links, relative to docs/
    for target in MD_LINK.findall(text):
        if target.startswith(("http://", "https://", "mailto:")):
            continue
        path = target.split("#", 1)[0].strip()
        if not path:
            continue
        checked += 1
        if not (llms.parent / path).exists():
            errors.append(f"[link] does not resolve from docs/: {target}")

    # 2. backticked paths, relative to the repo root
    for token in BACKTICKED.findall(text):
        if "/" not in token or not (token.endswith(".md") or token.endswith("/")):
            continue
        if ":" in token or token.startswith("${"):
            continue        # a URI scheme (classpath:/, file:/, flow://), not a repo path
        checked += 1
        if not (root / token).exists():
            errors.append(f"[path] does not resolve from the repo root: `{token}`")

    if errors:
        print("docs/llms.txt link integrity — broken references:\n")
        for e in errors:
            print("  - " + e)
        print(f"\n{len(errors)} of {checked} reference(s) broken.")
        return 1
    print(f"docs/llms.txt link integrity: OK ({checked} references resolve).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
