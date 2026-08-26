#!/usr/bin/env python3
"""memory-lint — deterministic integrity checks for an agent-memory repo.

Removes the LLM from the decay arithmetic. It counts session files and verifies
archival / tiers / supersession against *observable evidence* — the agent judges
meaning, this script does the counting. Pure Python 3 stdlib; no dependencies.

Usage:
    python3 memory-lint.py [--root PATH] [--strict]
    python3 memory-lint.py --scan-files FILE...   # credential-class [secret-material] scan
                                                  # of arbitrary (config) files; exit 1 on findings

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
        "closed_narrative_max_lines": 150,
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
    scan_files = None
    for i, a in enumerate(args):
        if a == "--root" and i + 1 < len(args):
            root_arg = args[i + 1]
        if a == "--scan-files":
            scan_files = args[i + 1:]  # everything after the flag is a path
            break
    return strict, root_arg, scan_files


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


def closed_narrative_lines(cont_text):
    # Non-empty lines belonging to completed `- [x]` thread records (checkbox line
    # through footer), the block ending at the next open thread or heading. This is
    # the measured bloat class (mercury-composable field report, 2026-08-21: 64% of
    # continuity was closed-thread narrative whose canonical home is the origin log).
    in_block, count = False, 0
    for line in cont_text.splitlines():
        if re.match(r"- \[x\]", line):
            in_block = True
        elif re.match(r"- \[ \]", line) or line.startswith("#"):
            in_block = False
        if in_block and line.strip():
            count += 1
    return count


def check_closed_thread_bloat(cont_text, cap):
    # (11) advisory: completed threads should wait out archive_window as terse
    # stubs (3–6 lines), not full ship narratives — REVIEW.md condenses them.
    n = closed_narrative_lines(cont_text)
    if n <= cap:
        return []
    return [
        f"[closed-thread-bloat] {n} line(s) of completed [x] thread records > "
        f"closed_narrative_max_lines {cap} — condense them to 3-6-line stubs at the next "
        f"review (REVIEW.md; the full narrative lives in each thread's origin session log), "
        f"or raise closed_narrative_max_lines in decay-policy.md."
    ]


# (10) [secret-material] — committed memory surfaces must not carry credentials or PII.
# Field incident (reported 2026-08-13, a client repo's DLP scanner): smoke-test output pasted into a
# session log leaked a live OAuth client secret — session logs are committed & shared, so
# anything pasted into them ships to every clone. This check is the deterministic backstop
# behind the memory/PROTOCOL.md redaction rule. Advisory (WARN): the script detects *shapes*; whether
# a hit is a real secret stays human/agent judgment. Unlike check 7 it DOES scan sessions/
# and archive/ — that's where pasted output lives — and it never echoes the matched value
# (a lint line quoting the secret would just amplify the leak into terminals and CI logs).
SECRET_VALUE_PATTERNS = [
    ("aws-access-key-id", re.compile(r"\bAKIA[0-9A-Z]{16}\b")),
    ("github-token", re.compile(r"\b(?:gh[pousr]_[A-Za-z0-9]{36,}|github_pat_[A-Za-z0-9_]{22,})\b")),
    ("gitlab-token", re.compile(r"\bglpat-[A-Za-z0-9_-]{20,}\b")),
    ("slack-token", re.compile(r"\bxox[baprs]-[A-Za-z0-9-]{10,}\b")),
    ("google-api-key", re.compile(r"\bAIza[0-9A-Za-z_-]{35}\b")),
    ("private-key-block", re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----")),
    ("jwt", re.compile(r"\beyJ[A-Za-z0-9_-]{8,}\.eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\b")),
]
# Credential-KEY assignment with a literal value (clientSecret='…', password: …, api_key=…).
# Keyed on the *name*, not the value shape — this is what catches a rendered JAAS line.
ASSIGNMENT_RE = re.compile(
    r"(?i)\b([A-Za-z0-9_.\-]*(?:secret|password|passwd|credential|api[_.\-]?key|apikey"
    r"|access[_.\-]?token|auth[_.\-]?token|bearer[_.\-]?token)[A-Za-z0-9_.\-]*)"
    # Permit a closing quote around a JSON/YAML key (`"client_secret": "…"`) without
    # making the quote part of the reported key.
    r"['\"`]?"
    # Backtick is a value delimiter alongside quotes: every scanned surface is markdown, where
    # assignments are typically quoted as inline code (`key=VALUE`) — without this, the closing
    # backtick rides into the captured value and defeats the enum-constant exclusion (v4.33.2).
    # Semicolon is a value delimiter too: JAAS/properties lines terminate with `;`
    # (`password={CHANGE_THIS};`) and it otherwise rides into the value, defeating the
    # placeholder rules (field probe 2026-08-14). A real secret containing `;` still flags on
    # its captured prefix.
    r"\s*[=:]\s*(['\"`]?)([^\s'\"`;]{8,})\2"
)
AUTHORIZATION_RE = re.compile(
    r"(?i)\b((?:proxy[_.\-]?)?authorization)\s*:\s*"
    r"(?:(?:bearer|basic)\s+)?(['\"`]?)([^\s'\"`]{8,})\2"
)
# An authorization VALUE that is a dotted lowercase identifier (`v1.basic.auth`) is a
# service/route/handler reference, never a token — real credentials carry uppercase, digits
# runs, or symbols beyond dots (field probe 2026-08-14: mercury REST configs). Case-sensitive
# on purpose: any uppercase keeps it flagged.
ROUTE_REF_RE = re.compile(r"[a-z][a-z0-9_-]*(?:\.[a-z0-9_-]+)+")
# Postman collections split the pair: `"key": "client_secret", "value": "…"` — the credential
# key is itself the VALUE of a "key" field (the field-incident artifact class, 2026-08-14).
POSTMAN_KV_RE = re.compile(
    r"(?i)\"key\"\s*:\s*\"([^\"]*(?:secret|password|passwd|credential|api[_.\-]?key|apikey"
    r"|access[_.\-]?token|auth[_.\-]?token|bearer[_.\-]?token|authorization)[^\"]*)\""
    r"\s*,\s*\"(?:value|src)\"\s*:\s*\"([^\"]{8,})\""
)
PLACEHOLDER_VALUE_RE = re.compile(
    r"(?i)(?:redacted|changeme|change-me|placeholder|example|sample|dummy|demo|test|todo|x{4,}"
    r"|your[-_][A-Za-z0-9_.\-]+"
    r"|(?:changeme|change-me|example|sample|dummy|demo|test|placeholder)[-_][A-Za-z0-9_.\-]+"
    r"|[A-Za-z0-9_.\-]+[-_](?:changeme|change-me|example|sample|dummy|demo|test|placeholder))"
)
TEMPLATE_VALUE_RE = re.compile(
    # Accept a bare or dotted reference with no fallback (`${VAR}`, `${VAR:}`, `${a.b}`).
    # A non-empty default may itself be a rendered secret, so `${VAR:-secret}` must flag —
    # except when the fallback is provably a placeholder (see _TEMPLATE_DEFAULT_RE below).
    r"(?:\$\{[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*:?\}"
    # GitHub-Actions expressions (`${{ secrets.X }}`) and single-brace placeholders
    # (`{CHANGE_THIS}` — the commented-JAAS-template form, field probe 2026-08-14):
    r"|\$\{\{[^{}]+\}\}|\{[^{}\s]+\}"
    r"|\$\([^)]+\)|\{\{[^{}]+\}\}"
    r"|<[A-Za-z0-9_.:\-]+>|%\([A-Za-z_][A-Za-z0-9_]*\)s|\(REDACTED\)|\*+)",
    re.IGNORECASE,
)
# A template reference WITH a non-empty fallback: safe only when the fallback itself is
# provably a placeholder — under the 8-char value floor, or passing the placeholder word
# rules (`${DEMO_PEER_TOKEN:demo}`, field probe 2026-08-14). `${CLIENT_SECRET:-Real…}` with a
# credential-shaped fallback keeps flagging (the v4.33.4 rule).
_TEMPLATE_DEFAULT_RE = re.compile(r"\$\{[A-Za-z_][A-Za-z0-9_.]*:-?([^{}]+)\}")
ENUM_KEY_RE = re.compile(r"(?i)(?:^|[_.\-])(?:source|type|mode|mechanism|strategy)$")
EMAIL_RE = re.compile(r"\b([A-Za-z0-9._%+-]+)@([A-Za-z0-9.-]+\.[A-Za-z]{2,})\b")
SSN_RE = re.compile(r"\b\d{3}-\d{2}-\d{4}\b")
E164_RE = re.compile(r"\+\d{10,15}\b")
CARD_RE = re.compile(r"\b(?:\d{4}[ -]){3}\d{4}\b|\b\d{13,19}\b")
HOME_PATH_RE = re.compile(r"(?:/Users/|/home/|[A-Za-z]:\\Users\\)([A-Za-z0-9._-]{2,})")
_HOME_OK = {"runner", "user", "username", "vsts_azpcontainer"}  # well-known CI users, not PII


def _is_placeholder_value(key, v):
    """Values that are templates, redactions, or number/date/version shapes — not secrets."""
    # The tool's own opt-down knob is knob vocabulary, not a credential: the pre-commit guard's
    # blocking message itself prints "AGENT_MEMORY_SECRET_GUARD=advisory", so a memory file
    # documenting that guidance would otherwise self-flag (field report, 2026-08-19).
    # Value-constrained on purpose — an arbitrary value under this key still flags, so the
    # exemption cannot be used as a smuggling envelope. Trailing ).,  punctuation is tolerated
    # because prose/parenthesized guidance rides it into the captured value — the guard's own
    # line ends "…=advisory)" (same capture behavior v4.33.2 fixed for backticks).
    if key.upper() in ("AGENT_MEMORY_SECRET_GUARD", "AGENT-MEMORY.SECRETGUARD") and re.fullmatch(
        r"(?:advisory|enforcing)[).,]*", v, re.IGNORECASE
    ):
        return True  # the git-config spelling (agent-memory.secretguard) is the same knob
    if TEMPLATE_VALUE_RE.fullmatch(v):
        return True
    m = _TEMPLATE_DEFAULT_RE.fullmatch(v)
    if m and (len(m.group(1)) < 8 or _is_placeholder_value(key, m.group(1))):
        return True  # template whose non-empty fallback is itself provably a placeholder
    if re.fullmatch(r"[\d.\-:/T]+", v):
        return True  # timestamps, dates, versions, counts (max_tokens: 128000, …)
    # ALL-CAPS is safe only on keys that explicitly describe an enum dimension. Treating every
    # uppercase value as an enum lets ordinary uppercase passwords and opaque secrets bypass the
    # assignment detector. (The motivating field line is credentials.source=OAUTHBEARER.)
    if ENUM_KEY_RE.search(key) and re.fullmatch(r"[A-Z][A-Z0-9_]{2,}", v):
        return True
    return bool(PLACEHOLDER_VALUE_RE.fullmatch(v))


def _is_public_email(local, domain):
    l, d = local.lower(), domain.lower()
    return (
        l in ("git", "noreply", "no-reply") or l.endswith("+noreply")
        or "noreply" in d or d.startswith("example.") or ".example" in d
        or d.endswith((".invalid", ".test", ".local", ".localhost"))
    )


def _luhn_ok(digits):
    total = 0
    for i, ch in enumerate(reversed(digits)):
        d = int(ch)
        if i % 2 == 1:
            d = d * 2 - 9 if d * 2 > 9 else d * 2
        total += d
    return total % 10 == 0


# One consolidated guidance line accompanies [secret-material] findings — printed ONCE per
# run by the consumer (report() for a full lint, the --scan-files CLI branch, the pre-commit
# hook's footer), never repeated per finding (field feedback, 2026-08-14 regression test).
# Scanner-neutral name: enterprise secret scanners flag trigger-word identifiers assigned
# string literals (Snyk field FP, 2026-08-25) — the suites' hygiene test enforces this.
GUIDANCE = (
    "  -> committed files are shared: redact to (REDACTED) or move the value out; a live "
    "credential is EXPOSED — rotate it (git history keeps the original; see the memory/PROTOCOL.md "
    "redaction rule)"
)


def _scan_lines(path, rel, credential_only):
    """One file's [secret-material] scan. credential_only=True is the config-file profile:
    token shapes, assignments, Authorization headers, private keys — NOT the PII classes
    (email/SSN/card/phone/home-path), which are memory-layer checks: config files
    legitimately carry contact emails and paths; credential material is never legitimate."""
    found = {}  # category -> [first_line, count, detail]

    def tally(cat, line_no, detail=""):
        if cat in found:
            found[cat][1] += 1
        else:
            found[cat] = [line_no, 1, detail]

    for i, line in enumerate(read_text(path).splitlines(), 1):
        # Explicit waiver for deliberately-quoted examples (a log *documenting* a leak
        # cleanup legitimately quotes the patterns). Tag the line, all categories skip it:
        if "lint:allow-secret-material" in line:
            continue
        for cat, rx in SECRET_VALUE_PATTERNS:
            if rx.search(line):
                tally(cat, i)
        for m in ASSIGNMENT_RE.finditer(line):
            if not _is_placeholder_value(m.group(1), m.group(3)):
                tally("credential-assignment", i, f" key '{m.group(1)}'")
        for m in POSTMAN_KV_RE.finditer(line):
            if not _is_placeholder_value(m.group(1), m.group(2)):
                tally("credential-assignment", i, f" key '{m.group(1)}'")
        for m in AUTHORIZATION_RE.finditer(line):
            if ROUTE_REF_RE.fullmatch(m.group(3)):
                continue  # dotted lowercase service/route reference, not a token
            if not _is_placeholder_value(m.group(1), m.group(3)):
                tally("authorization-header", i)
        if credential_only:
            continue
        for m in EMAIL_RE.finditer(line):
            if not _is_public_email(m.group(1), m.group(2)):
                tally("email", i)
        if SSN_RE.search(line):
            tally("ssn", i)
        if E164_RE.search(line):
            tally("phone-e164", i)
        for m in CARD_RE.finditer(line):
            digits = re.sub(r"[ -]", "", m.group(0))
            if 13 <= len(digits) <= 19 and _luhn_ok(digits):
                tally("payment-card", i)
        for m in HOME_PATH_RE.finditer(line):
            # need a letter/digit in the username — `/Users/...` is a placeholder, not a path
            if m.group(1).lower() not in _HOME_OK and re.search(r"[A-Za-z0-9]", m.group(1)):
                tally("home-path", i)

    out = []
    for cat in sorted(found):
        line_no, count, detail = found[cat]
        hits = f"{count} hit(s), first at line {line_no}"
        out.append(f"[secret-material] {rel}:{line_no} {cat}{detail} ({hits})")
    return out


def check_secret_material(root):
    mem = os.path.join(root, "memory")
    files = (
        sorted(glob.glob(os.path.join(mem, "*.md")))
        + sorted(glob.glob(os.path.join(mem, "sessions", "*.md")))
        + sorted(glob.glob(os.path.join(mem, "archive", "*.md")))
    )
    out = []
    for path in files:
        rel = os.path.relpath(path, root).replace(os.sep, "/")
        out.extend(_scan_lines(path, rel, False))
    return out


def scan_secret_files(paths):
    """`--scan-files` mode (v4.34.0): credential-class scan of arbitrary config files —
    used by the pre-commit hook on staged .json/.yml/.yaml/.properties/.env/.toml/.ini
    blobs and by the forge CI wrappers on changed files. Paths are reported as given;
    missing paths are skipped (a staged blob mirror owns existence)."""
    out = []
    for p in paths:
        if not os.path.isfile(p):
            continue
        out.extend(_scan_lines(p, p.replace(os.sep, "/"), True))
    return out


def report(cont, arch, sessions, acw, aw, warns, errors, strict):
    print(
        f"memory-lint: {len(cont)} continuity facts, {len(arch)} archived, "
        f"{len(sessions)} sessions; windows active={acw} archive={aw}"
    )
    for line in warns:
        print("WARN  " + line)
    if any("[secret-material]" in w for w in warns):
        print(GUIDANCE)  # once per run, not per finding
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
    strict, root_arg, scan_files = parse_args(sys.argv[1:])
    if scan_files is not None:
        # --scan-files mode: credential-class scan of the given paths, nothing else.
        # Exit 1 when findings exist (the calling wrapper owns advisory-vs-block semantics).
        findings = scan_secret_files(scan_files)
        for line in findings:
            print("WARN  " + line)
        if findings:
            print(GUIDANCE)  # once per run, not per finding
        return 1 if findings else 0
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
        + check_closed_thread_bloat(cont_text, w["closed_narrative_max_lines"])
        + check_stale_metadata(cont, pinned, refs, stems, w["working_window"], acw, aw)
        + check_secret_material(root)
    )

    return report(cont, arch, sessions, acw, aw, warns, errors, strict)


if __name__ == "__main__":
    sys.exit(main())
