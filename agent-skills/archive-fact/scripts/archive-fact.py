#!/usr/bin/env python3
"""archive-fact — safely MOVE a decayed fact from continuity.md to the archive.

This executes the mechanical half of REVIEW.md step 4 deterministically, so the
move can't be botched. The agent still decides *what* to archive (meaning); this
script performs the *move* safely (mechanics) — the same split as memory-lint.

Why it exists: the read-modify-write `open(f,"w").write(open(f).read()+…)` truncates
the file to empty BEFORE the read runs. That exact antipattern wiped a version.md
stamp and then this repo's archive (50 facts → 6), twice. This helper reads the
whole file into memory FIRST and writes once, so truncation is structurally impossible.

Usage:
    python3 archive-fact.py --id ID [--id ID2 ...] [--reason REASON]
                            [--quarter YYYY-Qn] [--note TEXT] [--root PATH] [--dry-run]

  --reason   "faded" (default) or e.g. "superseded by new-id"
  --quarter  archive file memory/archive/<quarter>.md (default: derived from today UTC)
  --note     INDEX one-liner override (default: derived from the block's first line)
  --dry-run  print what would move; change nothing

Exit: 0 = moved (or dry-run ok), 1 = refused (id missing / already archived / unsafe),
2 = could not locate the memory/ layer. Run `memory-lint` afterward to confirm.
"""
import argparse
import datetime
import os
import re
import sys


def find_root(start):
    for cand in (start, os.getcwd(), os.path.dirname(os.path.abspath(__file__))):
        if not cand:
            continue
        d = os.path.abspath(cand)
        while True:
            if os.path.isfile(os.path.join(d, "memory", "continuity.md")):
                return d
            parent = os.path.dirname(d)
            if parent == d:
                break
            d = parent
    return None


def read_text(path):
    with open(path, encoding="utf-8") as f:
        return f.read()


def footer_line_index(lines, fid):
    """Index of the single-line footer for fid, or None."""
    pat = re.compile(r"<!--\s*id:\s*" + re.escape(fid) + r"\s*\|")
    for i, ln in enumerate(lines):
        if pat.search(ln):
            return i
    return None


def block_span(lines, footer_idx):
    """[top, footer] inclusive: walk back from the footer to the nearest column-0
    list bullet ('- '), which starts the fact/thread block."""
    top = footer_idx
    for i in range(footer_idx, -1, -1):
        if lines[i].startswith("- "):
            top = i
            break
    return top, footer_idx


def derive_note(block_lines):
    first = block_lines[0]
    first = re.sub(r"^- \[[ xX]\]\s*", "", first)  # drop checkbox
    first = re.sub(r"^- \s*", "", first)
    first = first.replace("**", "").strip()
    first = re.sub(r"\s+", " ", first)
    return (first[:90].rstrip() + "…") if len(first) > 90 else first


def derive_quarter(today):
    return f"{today.year}-Q{(today.month - 1) // 3 + 1}"


def archive_facts(root, ids, reason, quarter, note, dry_run):
    mem = os.path.join(root, "memory")
    cont_path = os.path.join(mem, "continuity.md")
    cont = read_text(cont_path)
    lines = cont.split("\n")

    # Archived-already guard: scan existing archive footers (any *.md but INDEX).
    archived_ids = set()
    arch_dir = os.path.join(mem, "archive")
    if os.path.isdir(arch_dir):
        for name in os.listdir(arch_dir):
            if name.endswith(".md") and not name.upper().startswith("INDEX"):
                for m in re.finditer(r"<!--\s*id:\s*([a-z0-9-]+)\s*\|", read_text(os.path.join(arch_dir, name))):
                    archived_ids.add(m.group(1))

    # Validate ALL ids before touching anything (all-or-nothing).
    plans = []
    for fid in ids:
        if fid in archived_ids:
            return 1, f"refused: '{fid}' is already in the archive — nothing moved"
        fidx = footer_line_index(lines, fid)
        if fidx is None:
            return 1, f"refused: '{fid}' has no footer in continuity.md — nothing moved"
        top, bot = block_span(lines, fidx)
        plans.append((fid, top, bot))

    # Order removals bottom-up so earlier indices stay valid.
    plans_sorted = sorted(plans, key=lambda p: p[1], reverse=True)

    today = datetime.datetime.now(datetime.timezone.utc).date()
    quarter = quarter or derive_quarter(today)
    arch_path = os.path.join(mem, "archive", f"{quarter}.md")
    index_path = os.path.join(mem, "archive", "INDEX.md")

    moved = []
    archive_chunks = []
    index_chunks = []
    kept = list(lines)
    for fid, top, bot in plans_sorted:
        block = lines[top : bot + 1]
        n = note if (note and len(ids) == 1) else derive_note(block)
        moved.append((fid, n))
        archive_chunks.append("\n".join(block))
        index_chunks.append(f"- {fid} — {n} — {reason} — {quarter}.md")
        # remove block + one trailing blank line if present
        end = bot + 1
        if end < len(kept) and kept[end].strip() == "":
            end += 1
        del kept[top:end]

    moved.reverse()
    archive_chunks.reverse()
    index_chunks.reverse()

    heading = (
        f"\n## {today.isoformat()} — archived via archive-fact ({reason})\n\n### Faded facts\n\n"
    )
    archive_block = heading + "\n\n".join(archive_chunks) + "\n"
    index_block = "\n".join(index_chunks) + "\n"
    new_cont = "\n".join(kept)

    if not new_cont.strip():
        return 1, "refused: removing those blocks would empty continuity.md — nothing moved"

    summary = "\n".join(f"  {fid} → {quarter}.md  ({n})" for fid, n in moved)
    if dry_run:
        return 0, f"DRY-RUN — would move {len(moved)} fact(s):\n{summary}\n(no files changed)"

    # WRITES — append-mode for the archive/INDEX (never truncates); single write for continuity.
    with open(arch_path, "a", encoding="utf-8") as f:
        f.write(archive_block)
    with open(index_path, "a", encoding="utf-8") as f:
        f.write(index_block)
    with open(cont_path, "w", encoding="utf-8") as f:  # safe: new_cont already in memory
        f.write(new_cont)

    return 0, f"moved {len(moved)} fact(s) to {quarter}.md + INDEX:\n{summary}\nNow run memory-lint to confirm."


def main(argv=None):
    ap = argparse.ArgumentParser(description="Safely move a decayed fact from continuity.md to the archive.")
    ap.add_argument("--id", action="append", dest="ids", required=True, help="fact id (repeatable)")
    ap.add_argument("--reason", default="faded", help='"faded" (default) or "superseded by <id>"')
    ap.add_argument("--quarter", default=None, help="archive quarter file, e.g. 2026-Q2 (default: today)")
    ap.add_argument("--note", default=None, help="INDEX one-liner (single --id only; else derived)")
    ap.add_argument("--root", default=None)
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args(argv)

    root = find_root(args.root or os.getcwd())
    if not root:
        print("archive-fact: could not find memory/continuity.md", file=sys.stderr)
        return 2
    code, msg = archive_facts(root, args.ids, args.reason, args.quarter, args.note, args.dry_run)
    print(("archive-fact: " + msg) if code == 0 else ("archive-fact: " + msg), file=(sys.stdout if code == 0 else sys.stderr))
    return code


if __name__ == "__main__":
    sys.exit(main())
