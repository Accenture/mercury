#!/usr/bin/env python3
"""memory-lint — deterministic integrity checks for an agent-memory repo.

Removes the LLM from the decay arithmetic. It counts session files and verifies
archival / tiers / supersession against *observable evidence* — the agent judges
meaning, this script does the counting. Pure Python 3 stdlib; no dependencies.

Usage:
    python3 memory-lint.py [--root PATH] [--strict]

Exit: 0 = clean (no errors), 1 = integrity error(s) (or warnings under --strict),
2 = could not locate the memory/ layer.
"""
import glob
import os
import re
import sys

ID_RE = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+")
# Footers are single-line HTML comments. Bind the field span to one line
# ([^\n], and no re.S) so an *unclosed* footer (a stray "<!-- id: foo | ..." with
# no closing "-->") can't let .*? swallow the rest of the file up to a "-->"
# elsewhere — that would silently misparse fields (wrong tier/superseded ⇒ wrong
# decay counts) with no error raised. The verifier must not be fooled by malformed input.
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
    return open(path, encoding="utf-8").read()


def parse_footers(text):
    out = {}
    for m in FOOTER_RE.finditer(text):
        fields = {}
        for part in m.group(2).split("|"):
            if ":" in part:
                k, _, v = part.partition(":")
                fields[k.strip()] = v.strip()
        out[m.group(1)] = fields
    return out


def pinned_open_threads(text):
    """ids whose nearest preceding list bullet is an unchecked '- [ ]' (never decay)."""
    pinned = set()
    state = None  # None, "open", "done"
    indent_level = 0

    for ln in text.split("\n"):
        st = ln.lstrip()
        if not st:
            continue

        current_indent = len(ln) - len(st)

        if st.startswith("- [ ]"):
            state = "open"
            indent_level = current_indent
        elif st.startswith(("- [x]", "- [X]")):
            state = "done"
            indent_level = current_indent
        elif st.startswith(("- ", "* ")):
            # Only reset state if this bullet is at the same or higher level than the parent open thread
            if state is not None and current_indent <= indent_level:
                state = None

        m = re.search(r"<!--\s*id:\s*([a-z0-9-]+)", ln)
        if m and state == "open":
            pinned.add(m.group(1))
    return pinned


def memref_ids(text):
    # Anchor the heading to the start of a line. A session log may *quote* the
    # string "## Memory References" inline in prose (e.g. while describing this
    # very check); an un-anchored find() would match that mention and scoop a
    # neighbouring section's ids into the references set — a false "over-archived"
    # positive. Match only a real heading line, and bound at the next one.
    m = re.search(r"(?m)^## +Memory References[ \t]*$", text)
    if m is None:
        return set()
    block = text[m.end():]
    nxt = re.search(r"(?m)^## +\S", block)
    if nxt is not None:
        block = block[: nxt.start()]
    return set(ID_RE.findall(block))


def load_windows(root):
    # Defaults track the shipped templates/memory/decay-policy.md (v4.24.0): a repo
    # whose policy omits a field falls back to these. continuity_max_facts is the
    # primary lean signal (count > lines — verbosity/velocity-independent).
    w = {
        "working_window": 3,
        "active_window": 8,
        "archive_window": 20,
        "review_every": 10,
        "continuity_max_facts": 30,
        "continuity_max_lines": 600,
    }
    p = os.path.join(root, "memory", "decay-policy.md")
    if os.path.isfile(p):
        t = read_text(p)
        for k in w:
            m = re.search(rf"{k}\s*:\s*(\d+)", t)
            if m:
                w[k] = int(m.group(1))
    return w


def parse_args(args):
    strict = "--strict" in args
    root_arg = None
    for i, a in enumerate(args):
        if a == "--root" and i + 1 < len(args):
            root_arg = args[i + 1]
    return strict, root_arg


def load_repo(root):
    """Read the memory/ layer. Returns (cont, pinned, arch, extra, sessions, refs)."""
    mem = os.path.join(root, "memory")
    cont_text = read_text(os.path.join(mem, "continuity.md"))
    cont = parse_footers(cont_text)
    pinned = pinned_open_threads(cont_text)

    archive_text = ""
    for f in glob.glob(os.path.join(mem, "archive", "*.md")):
        if os.path.basename(f).upper().startswith("INDEX"):
            continue
        archive_text += read_text(f) + "\n"
    arch = parse_footers(archive_text)

    # Extra footers from other memory/*.md files (e.g. vision.md) — used only for
    # supersession link resolution in check_dangling; not counted as cont/arch facts.
    _skip = {"continuity.md", "decay-policy.md"}
    extra_text = ""
    for name in sorted(os.listdir(mem)):
        if not name.endswith(".md") or name in _skip:
            continue
        fp = os.path.join(mem, name)
        if os.path.isfile(fp):
            extra_text += read_text(fp) + "\n"
    extra = parse_footers(extra_text)

    sessions = sorted(glob.glob(os.path.join(mem, "sessions", "*.md")))
    refs = [memref_ids(read_text(s)) for s in sessions]
    return cont, pinned, arch, extra, sessions, refs


def make_sslu(refs):
    """sessions_since_last_used: how many sessions back a fact was last referenced."""
    def sslu(fid):
        last = -1
        for i, ids in enumerate(refs):
            if fid in ids:
                last = i
        return None if last == -1 else len(refs) - 1 - last
    return sslu


def check_duplicates(cont, arch):
    # (1) a fact must live in exactly one place
    return [
        f"[both] {fid} is in BOTH continuity.md and the archive"
        for fid in sorted(set(cont) & set(arch))
    ]


def check_over_archived(arch, sslu, aw):
    # (2) the decay miscount guard: archived-as-faded but still referenced in-window
    out = []
    for fid, fields in arch.items():
        if "superseded-by" in fields or fields.get("tier") == "superseded":
            continue  # superseded archives on truth-state, not recency
        s = sslu(fid)
        if s is not None and s <= aw:
            out.append(
                f"[over-archived] {fid} archived as faded but last referenced {s} "
                f"session(s) ago (<= archive_window {aw}) — reactivate it"
            )
    return out


def check_overdue(cont, pinned, sslu, aw):
    # (3) advisory: continuity fact overdue for archival
    #     (core, superseded, and pinned unchecked open threads never decay)
    out = []
    for fid, fields in cont.items():
        if fields.get("tier") in ("core", "superseded") or fid in pinned:
            continue
        s = sslu(fid)
        if s is not None and s > aw:
            out.append(f"[overdue] {fid} sslu {s} > archive_window {aw} — review may archive it")
    return out


def check_session_filenames(sessions):
    # (5) session filenames must carry a time component (YYYY-MM-DD-HHmmss.md).
    # A date-only name means the agent used the injected context date instead of
    # running `date -u +%Y-%m-%d-%H%M%S` — it breaks same-day lexicographic ordering.
    DATE_ONLY = re.compile(r"^\d{4}-\d{2}-\d{2}$")
    out = []
    for s in sessions:
        stem = os.path.basename(s)[:-3]
        if DATE_ONLY.match(stem):
            out.append(
                f"[date-only-session] {os.path.basename(s)} — missing time component; "
                "run `date -u +%Y-%m-%d-%H%M%S` at persist time (not the context date)"
            )
    return out


def check_version_manifest(root):
    # (6) .agent/version.md, IF present, must carry a parseable semver `version:` line.
    # An empty/malformed manifest breaks Mode B upgrade detection — and was a real bug
    # (a truncating stamp one-liner emptied it). A MISSING file is valid (pre-versioning
    # baseline, handled by ENABLE/UPGRADE) and is NOT flagged.
    p = os.path.join(root, ".agent", "version.md")
    if not os.path.isfile(p):
        return []
    m = re.search(r"(?m)^- \*\*version:\*\*\s*(\d+\.\d+\.\d+)", read_text(p))
    if m is None:
        return [
            "[version-manifest] .agent/version.md exists but has no parseable "
            "`- **version:** X.Y.Z` line (empty or malformed) — breaks Mode B upgrade detection"
        ]
    return []


def check_conflict_markers(root):
    # (7) No leftover VCS merge-conflict markers in the LIVE top-level memory files —
    # the ones every teammate concurrently edits and the agent reads as truth
    # (continuity.md, instructions.md, vision.md, decay-policy.md, smoke-test.md). We scan
    # `memory/*.md` only (non-recursive): `sessions/` and `archive/` are deliberately
    # EXCLUDED — they are immutable/append narrative that legitimately *quotes* conflict
    # markers (a session log pasting terminal output or a real diff to document it), so
    # scanning them would false-positive. Match git's `<<<<<<<` / `>>>>>>>` and the diff3
    # `|||||||` line markers; deliberately do NOT match a bare `=======` line (a valid
    # Markdown setext heading underline).
    out = []
    mem = os.path.join(root, "memory")
    marker = re.compile(r"^(<{7}|>{7}|\|{7})(\s|$)")
    for path in sorted(glob.glob(os.path.join(mem, "*.md"))):
        for i, line in enumerate(read_text(path).splitlines(), 1):
            if marker.match(line):
                rel = os.path.relpath(path, root)
                out.append(
                    f"[conflict-marker] {rel}:{i} unresolved merge-conflict marker "
                    "— resolve it before committing"
                )
                break  # one report per file is enough
    return out


def check_dangling(allf):
    # (4) supersession links resolve
    out = []
    for fid, fields in allf.items():
        for key in ("superseded-by", "supersedes"):
            tgt = fields.get(key)
            if tgt and tgt not in allf:
                out.append(f"[dangling] {fid} {key} {tgt}, which has no footer anywhere")
    return out


LAST_REVIEW_RE = re.compile(
    r"(?m)^- \*\*last_review:\*\*\s*([0-9-]+)(?:\s*\|\s*through\s+([0-9][0-9-]*))?"
)


def sessions_since_review(sessions, cont_text):
    """How many session files were written after the last_review 'through' stamp.
    No last_review recorded (never reviewed) ⇒ every session counts as 'since'."""
    stems = [os.path.basename(s)[:-3] for s in sessions]
    m = LAST_REVIEW_RE.search(cont_text)
    if not m:
        return len(stems)
    through = m.group(2) or m.group(1)  # prefer the 'through <session-file>' token
    return sum(1 for s in stems if s > through)


def created_sessions_ago(created, stems):
    """How many session files are dated strictly after `created` (YYYY-MM-DD).
    Approximate (counts by date, undercounts same-day) — only used for the
    working-window check, where a small bias toward 'working' is the safe side."""
    if not created:
        return None
    return sum(1 for s in stems if s[:10] > created)


def expected_tier(fields, fid, sslu_val, uses_val, created_ago, pinned, ww, acw, aw):
    """Tier a fact *should* carry, per DECAY.md §5 (first match wins). Clamps at
    'archive-candidate' — a fact still in continuity is never 'archived' (that tier
    means *moved*; the [overdue] check + archive-fact handle the actual move)."""
    if fields.get("superseded-by") or fields.get("tier") == "superseded":
        return "superseded"
    if fields.get("tier") == "core":
        return "core"
    if fid in pinned:                      # unchecked Open Thread → never decays; its pinned-ness
        return fields.get("tier")          # protects it, not the tier label — so leave the label as-is
    if sslu_val is None:                   # never referenced — can't recompute; don't flag
        return fields.get("tier")
    if created_ago is not None and created_ago <= ww and uses_val <= 1:
        return "working"
    if sslu_val <= acw:
        return "active"
    return "archive-candidate"             # acw < sslu (incl. > aw, which [overdue] flags for the move)


def check_stale_metadata(cont, pinned, refs, stems, ww, acw, aw):
    # (9) advisory: a fact's stored `tier` disagrees with the tier recomputed from the
    # session reference log — i.e. review steps 2–3 (apply events / re-tier) were skipped.
    # Catches the "did the archive but not the metadata pass" gap. core/superseded exempt.
    out = []
    sslu = make_sslu(refs)
    for fid, fields in cont.items():
        if fields.get("tier") in ("core", "superseded") or fields.get("superseded-by"):
            continue
        uses_val = sum(1 for ids in refs if fid in ids)
        et = expected_tier(fields, fid, sslu(fid), uses_val, created_sessions_ago(fields.get("created"), stems), pinned, ww, acw, aw)
        stored = fields.get("tier")
        if et is not None and et != stored:
            out.append(
                f"[stale-metadata] {fid} tier '{stored}' should be '{et}' (sslu {sslu(fid)}) "
                "— review steps 2–3 (re-tier) skipped; run refresh-metadata or a review"
            )
    return out


def check_continuity_health(cont, sessions, cont_text, cont_lines, re_every, max_facts, max_lines, pinned=None, archivable=None):
    # (8) advisory cadence/size triggers — what would have caught a real product repo
    # that ran 61 sessions and never archived (review never fired in the field).
    # All advisory (WARN): a review is a human/agent ritual, never a hard gate.
    # `archivable` (optional) = count of entries a review could archive right now (facts overdue
    # for decay + superseded facts). When it's 0, a lines-only breach can't be honestly cleared by
    # a review, so the message says so instead of nudging toward premature archival (v4.28.3).
    if pinned is None:
        pinned = set()
    out = []
    ssr = sessions_since_review(sessions, cont_text)
    if ssr >= re_every:
        out.append(
            f"[review-overdue] {ssr} session(s) since last review >= review_every "
            f"{re_every} — run the REVIEW.md ritual"
        )
    # Count only decay-eligible facts — exclude tier:core (structural invariants) and pinned
    # open threads (active workstreams). Those can never be archived, so counting them against
    # the cap produces permanent noise after a correct review (field report: mercury-composable).
    decay_eligible = {
        fid: fields for fid, fields in cont.items()
        if fields.get("tier") != "core" and fid not in pinned
    }
    nfacts = len(decay_eligible)
    if nfacts > max_facts:
        out.append(
            f"[continuity-bloat] {nfacts} decay-eligible facts > continuity_max_facts "
            f"{max_facts} — a review is due to lean it down"
        )
    if cont_lines > max_lines:
        if archivable == 0:
            # Lines over budget but a review has nothing to archive right now (nothing faded past
            # archive_window, nothing superseded). "A review will lean it down" would be dishonest
            # and pressures archiving an *active* fact — REVIEW.md's costliest error. Name the real
            # lever instead (field report: mercury-composable, a complex repo's dense active facts).
            out.append(
                f"[continuity-bloat] continuity.md {cont_lines} lines > continuity_max_lines "
                f"{max_lines} — but nothing is archivable yet; the excess is active/dense facts. "
                f"Condense shipped decisions, or raise continuity_max_lines in decay-policy.md if "
                f"this repo is legitimately large."
            )
        else:
            out.append(
                f"[continuity-bloat] continuity.md {cont_lines} lines > continuity_max_lines "
                f"{max_lines} — a review is due to lean it down"
            )
    return out


def report(cont, arch, sessions, acw, aw, warns, errors, strict):
    print(
        f"memory-lint: {len(cont)} continuity facts, {len(arch)} archived, "
        f"{len(sessions)} sessions; windows active={acw} archive={aw}"
    )
    for line in warns:
        print("WARN  " + line)
    for line in errors:
        print("ERROR " + line)
    if errors:
        print(f"FAIL: {len(errors)} error(s), {len(warns)} warning(s)")
        return 1
    if warns and strict:
        print(f"FAIL (strict): {len(warns)} warning(s)")
        return 1
    print(f"OK: 0 errors, {len(warns)} warning(s)")
    return 0


def main():
    strict, root_arg = parse_args(sys.argv[1:])
    root = find_root(root_arg or os.getcwd())
    if not root:
        print("memory-lint: could not find memory/continuity.md", file=sys.stderr)
        return 2

    cont, pinned, arch, extra, sessions, refs = load_repo(root)
    w = load_windows(root)
    aw, acw = w["archive_window"], w["active_window"]
    sslu = make_sslu(refs)

    cont_text = read_text(os.path.join(root, "memory", "continuity.md"))
    cont_lines = len(cont_text.splitlines())

    errors = (
        check_duplicates(cont, arch)
        + check_over_archived(arch, sslu, aw)
        + check_version_manifest(root)
        + check_conflict_markers(root)
    )
    stems = [os.path.basename(s)[:-3] for s in sessions]
    overdue = check_overdue(cont, pinned, sslu, aw)
    # What a review could archive right now: facts overdue for decay + superseded facts. When 0,
    # a lines-only bloat breach has no honest fix via archival (v4.28.3).
    archivable = len(overdue) + sum(1 for f in cont.values() if f.get("tier") == "superseded")
    warns = (
        overdue
        + check_dangling({**cont, **arch, **extra})
        + check_session_filenames(sessions)
        + check_continuity_health(
            cont, sessions, cont_text, cont_lines,
            w["review_every"], w["continuity_max_facts"], w["continuity_max_lines"],
            pinned, archivable,
        )
        + check_stale_metadata(cont, pinned, refs, stems, w["working_window"], acw, aw)
    )

    return report(cont, arch, sessions, acw, aw, warns, errors, strict)


if __name__ == "__main__":
    sys.exit(main())
