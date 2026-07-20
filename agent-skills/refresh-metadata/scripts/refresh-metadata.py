#!/usr/bin/env python3
"""refresh-metadata — recompute continuity.md fact footers from the session log.

Executes REVIEW.md steps 2–3 (apply events + re-tier) deterministically: for every
fact in continuity.md, recompute `last_used`, `uses`, and `tier` from the `## Memory
References` across `memory/sessions/`, and write the footers back. This is the
"full rebuild" path REVIEW.md calls *deterministic and reproducible by any agent* —
it's pure arithmetic (no judgment), so it's safe to mechanize. Agents routinely skip
this pass (archive but don't re-tier), leaving stale footers; this closes that gap.

What it does NOT do: decide what to *archive* (that's `archive-fact` + agent judgment),
touch `core`/`superseded` facts (terminal / human-set), or fabricate missing fields.
It clamps tier at `archive-candidate` — a fact still in continuity is never `archived`.

Safe by construction: reads continuity into memory and writes once (no truncate-before-read).

Usage:
    python3 refresh-metadata.py [--root PATH] [--dry-run]

Exit: 0 = refreshed (or dry-run), 2 = could not locate the memory/ layer.
Run `memory-lint` afterward to confirm (the [stale-metadata] advisories should clear).
"""
import argparse
import os
import re
import sys

ID_RE = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+")
FOOTER_RE = re.compile(r"<!--\s*id:\s*([a-z0-9-]+)\s*\|([^\n]*?)-->")


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


def parse_fields(blob):
    fields = {}
    for part in blob.split("|"):
        if ":" in part:
            k, _, v = part.partition(":")
            fields[k.strip()] = v.strip()
    return fields


def memref_ids(text):
    m = re.search(r"(?m)^## +Memory References[ \t]*$", text)
    if m is None:
        return set()
    block = text[m.end():]
    nxt = re.search(r"(?m)^## +\S", block)
    if nxt is not None:
        block = block[: nxt.start()]
    return set(ID_RE.findall(block))


def pinned_open_threads(text):
    pinned, state, indent = set(), None, 0
    for ln in text.split("\n"):
        st = ln.lstrip()
        if not st:
            continue
        cur = len(ln) - len(st)
        if st.startswith("- [ ]"):
            state, indent = "open", cur
        elif st.startswith(("- [x]", "- [X]")):
            state, indent = "done", cur
        elif st.startswith(("- ", "* ")) and state is not None and cur <= indent:
            state = None
        m = re.search(r"<!--\s*id:\s*([a-z0-9-]+)", ln)
        if m and state == "open":
            pinned.add(m.group(1))
    return pinned


def load_sessions(root):
    sdir = os.path.join(root, "memory", "sessions")
    stems = sorted(f[:-3] for f in os.listdir(sdir) if f.endswith(".md")) if os.path.isdir(sdir) else []
    refs = [memref_ids(read_text(os.path.join(sdir, s + ".md"))) for s in stems]
    return stems, refs


def expected_tier(fields, fid, sslu_val, uses_val, created_ago, pinned, ww, acw):
    if fields.get("superseded-by") or fields.get("tier") == "superseded":
        return "superseded"
    if fields.get("tier") == "core":
        return "core"
    if fid in pinned:                      # unchecked Open Thread → never decays; leave its tier label as-is
        return fields.get("tier")
    if sslu_val is None:
        return fields.get("tier")
    if created_ago is not None and created_ago <= ww and uses_val <= 1:
        return "working"
    if sslu_val <= acw:
        return "active"
    return "archive-candidate"


def load_windows(root):
    w = {"working_window": 3, "active_window": 8, "archive_window": 20}
    p = os.path.join(root, "memory", "decay-policy.md")
    if os.path.isfile(p):
        t = read_text(p)
        for k in w:
            m = re.search(rf"{k}\s*:\s*(\d+)", t)
            if m:
                w[k] = int(m.group(1))
    return w


def refresh(root, dry_run):
    cont_path = os.path.join(root, "memory", "continuity.md")
    text = read_text(cont_path)
    stems, refs = load_sessions(root)
    pinned = pinned_open_threads(text)
    w = load_windows(root)
    ww, acw = w["working_window"], w["active_window"]

    def footer_repl(m):
        fid, blob = m.group(1), m.group(2)
        fields = parse_fields(blob)
        # terminal / human-set: never touched
        if fields.get("tier") in ("core", "superseded") or fields.get("superseded-by"):
            return m.group(0)
        hits = [i for i, ids in enumerate(refs) if fid in ids]
        if not hits:                       # never referenced — can't recompute; preserve
            return m.group(0)
        uses_val = len(hits)
        last_used = stems[hits[-1]][:10]
        sslu_val = len(stems) - 1 - hits[-1]
        created_ago = sum(1 for s in stems if s[:10] > fields.get("created", "")) if fields.get("created") else None
        tier = expected_tier(fields, fid, sslu_val, uses_val, created_ago, pinned, ww, acw)
        new = m.group(0)
        if "last_used" in fields:
            new = re.sub(r"(\blast_used:\s*)[0-9-]+", r"\g<1>" + last_used, new)
        if "uses" in fields:
            new = re.sub(r"(\buses:\s*)\d+", r"\g<1>" + str(uses_val), new)
        if "tier" in fields and tier:
            new = re.sub(r"(\btier:\s*)[a-z-]+", r"\g<1>" + tier, new)
        if new != m.group(0):
            changes.append((fid, fields.get("tier"), tier, fields.get("uses"), str(uses_val)))
        return new

    changes = []
    new_text = FOOTER_RE.sub(footer_repl, text)

    if not changes:
        return 0, "all fact footers already match the reference log — nothing to refresh"

    lines = []
    for fid, ot, nt, ou, nu in changes:
        bits = []
        if ot != nt:
            bits.append(f"tier {ot}→{nt}")
        if ou != nu:
            bits.append(f"uses {ou}→{nu}")
        lines.append(f"  {fid}: " + ", ".join(bits) if bits else f"  {fid}: last_used")
    summary = "\n".join(lines)
    if dry_run:
        return 0, f"DRY-RUN — would refresh {len(changes)} footer(s):\n{summary}\n(no files changed)"

    with open(cont_path, "w", encoding="utf-8") as f:  # safe: new_text already in memory
        f.write(new_text)
    return 0, f"refreshed {len(changes)} footer(s) from the reference log:\n{summary}\nNow run memory-lint to confirm."


def main(argv=None):
    ap = argparse.ArgumentParser(description="Recompute continuity.md footers (last_used/uses/tier) from the session log.")
    ap.add_argument("--root", default=None)
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args(argv)
    root = find_root(args.root or os.getcwd())
    if not root:
        print("refresh-metadata: could not find memory/continuity.md", file=sys.stderr)
        return 2
    code, msg = refresh(root, args.dry_run)
    print("refresh-metadata: " + msg)
    return code


if __name__ == "__main__":
    sys.exit(main())
