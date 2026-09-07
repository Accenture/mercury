#!/usr/bin/env python3
"""Claims-fixture gate for documentation BEHAVIOR claims (the Rust edition of the
mechanism the Java engine ships as check 8 of its doc-canon script; see the Java repo's
ADR-0023 and this repo's docs/guides/claims-registry.json).

Every registry entry pins a high-value prose claim on both sides:
  - the claim's normative sentence (`quote`) must still appear on one of its `pages`
    (whitespace-normalized, case-insensitive containment — removal or rewording fails,
    re-casing passes);
  - the claim's engine test pin (`test`: crates/<crate-dir>::<test-file-stem>::<fn>)
    must still exist as a function DEFINITION in that crate's test tree (a renamed or
    deleted pin cannot silently orphan a claim). The pinned tests themselves run in the
    normal `cargo test --workspace` build.

Rationale: the AI grammar coverage study found all documentation drift in ungated prose —
this gate extends drift testing from shape to behavior.

Exit 0 = clean; exit 1 = drift (with details). Run from anywhere:
    python3 scripts/check-doc-claims.py [--root PATH]
"""
import argparse
import json
import re
import sys
from pathlib import Path

REGISTRY_REL = "docs/guides/claims-registry.json"


def norm(s: str) -> str:
    return re.sub(r"\s+", " ", s).strip().lower()


def rel(p: Path, root: Path) -> str:
    try:
        return str(p.relative_to(root))
    except ValueError:
        return str(p)


def read_text(p: Path) -> str | None:
    try:
        return p.read_text(encoding="utf-8")
    except OSError:
        return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=str(Path(__file__).resolve().parent.parent),
                    help="repo root (default: parent of scripts/)")
    args = ap.parse_args()
    root = Path(args.root)
    registry = root / REGISTRY_REL
    errors: list[str] = []

    if not registry.exists():
        print(f"claims registry not found at {REGISTRY_REL}; nothing to check.")
        return 0

    try:
        claims = json.loads(registry.read_text(encoding="utf-8")).get("claims", [])
    except json.JSONDecodeError as e:
        print(f"[claims] registry is not valid JSON: {e}")
        return 1

    for c in claims:
        if not isinstance(c, dict):
            errors.append(f"non-object entry in claims list: {str(c)[:60]}")
            continue
        cid = c.get("id", "?")
        pages_raw = c.get("pages", [])
        if not isinstance(pages_raw, list) or not all(
                isinstance(p, str) for p in pages_raw):
            errors.append(f"{cid}: 'pages' must be a list of path strings")
            continue
        page_texts: list[str] = []
        for p in (root / page for page in pages_raw):
            if not p.is_file():
                errors.append(f"{cid}: page not found: {rel(p, root)}")
                continue
            text = read_text(p)
            if text is None:
                errors.append(f"{cid}: page not readable: {rel(p, root)}")
            else:
                page_texts.append(text)
        quote = norm(c.get("quote", "") or "")
        if not quote:
            errors.append(f"{cid}: empty quote")
        elif not any(quote in norm(t) for t in page_texts):
            errors.append(f"{cid}: pinned sentence not found on any of its pages: "
                          f"\"{c.get('quote', '')[:80]}\"")
        test = c.get("test", "") or ""
        if test:
            ref = re.fullmatch(r"([\w./-]+)::([\w-]+)::(\w+)", test)
            if not ref:
                errors.append(f"{cid}: malformed test ref "
                              f"(want crates/<crate>::<test-file-stem>::<fn>): {test}")
                continue
            crate_dir, stem, fn = ref.groups()
            crate = root / crate_dir
            candidates = [crate / "tests" / f"{stem}.rs"]
            if not candidates[0].is_file() and crate.exists():
                candidates = list(crate.rglob(f"{stem}.rs"))
            # match a fn DEFINITION (any visibility, incl. pub(crate)), not a
            # call site or comment; /* */ blocks are stripped first so a
            # commented-out pin cannot keep a claim green
            def_rx = re.compile(
                rf"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+{re.escape(fn)}\s*[(<]",
                re.MULTILINE)
            sources = [read_text(p) for p in candidates if p.is_file()]
            sources = [re.sub(r"/\*.*?\*/", "", s, flags=re.DOTALL)
                       for s in sources if s is not None]
            if not sources:
                errors.append(f"{cid}: test file not found: {test}")
            elif not any(def_rx.search(s) for s in sources):
                errors.append(f"{cid}: test fn '{fn}' not defined in {stem}.rs")

    if errors:
        print("Documentation behavior-claim drift detected:\n")
        for e in errors:
            print("  - " + e)
        print(f"\n{len(errors)} issue(s). See docs/guides/claims-registry.json.")
        return 1
    print(f"docs/guides/claims-registry.json: OK ({len(claims)} claims pinned on both sides).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
