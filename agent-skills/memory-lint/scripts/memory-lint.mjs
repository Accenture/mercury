#!/usr/bin/env node
// memory-lint.mjs — deterministic integrity checks for an agent-memory repo.
//
// Node port of memory-lint.py, for machines that have node but not python3.
// Built-in modules only; no npm install. Kept at *feature + output parity* with
// the Python verifier — the shared test fixtures (test_memory_lint.*) are the
// cross-runtime contract that holds the two implementations equivalent. The
// point of this skill is to take the decay arithmetic off the LLM's hands; that
// guarantee should not depend on which runtime the machine happens to have.
//
// Usage:  node memory-lint.mjs [--root PATH] [--strict]
// Exit:   0 = clean (no errors), 1 = integrity error(s) (or warnings under
//         --strict), 2 = could not locate the memory/ layer.

import { readFileSync, readdirSync, existsSync, statSync } from "node:fs";
import { resolve, dirname, join, basename } from "node:path";
import { fileURLToPath } from "node:url";

const ID_RE = /[a-z][a-z0-9]*(?:-[a-z0-9]+)+/g;
// Footers are single-line HTML comments. Bind the field span to one line
// ([^\n], no dot-all) so an *unclosed* footer (a stray "<!-- id: foo | ..." with
// no closing "-->") can't swallow the rest of the file up to a "-->" elsewhere —
// that would silently misparse fields (wrong tier/superseded => wrong decay
// counts) with no error raised. The verifier must not be fooled by malformed input.
const FOOTER_RE = /<!--\s*id:\s*([a-z0-9-]+)\s*\|([^\n]*?)-->/g;
const PIN_ID_RE = /<!--\s*id:\s*([a-z0-9-]+)/;

// Code-point comparator: matches Python's default `sorted()` on ASCII ids, so the
// two runtimes order ids identically. (Explicit, per sonar S2871 — default Array
// .sort() coerces to string and compares by UTF-16, which is what we want here.)
function byCodePoint(a, b) {
  if (a < b) return -1;
  if (a > b) return 1;
  return 0;
}

export function find_root(start) {
  const here = dirname(fileURLToPath(import.meta.url));
  for (const cand of [start, process.cwd(), here]) {
    if (!cand) continue;
    let d = resolve(cand);
    while (true) {
      const f = join(d, "memory", "continuity.md");
      if (existsSync(f) && statSync(f).isFile()) return d;
      const parent = dirname(d);
      if (parent === d) break;
      d = parent;
    }
  }
  return null;
}

function read_text(path) {
  return readFileSync(path, "utf-8");
}

export function parse_footers(text) {
  const out = new Map();
  for (const m of text.matchAll(FOOTER_RE)) {
    const fields = {};
    for (const part of m[2].split("|")) {
      const i = part.indexOf(":");
      if (i !== -1) fields[part.slice(0, i).trim()] = part.slice(i + 1).trim();
    }
    out.set(m[1], fields);
  }
  return out;
}

export function pinned_open_threads(text) {
  // ids whose nearest preceding list bullet is an unchecked '- [ ]' (never decay).
  const pinned = new Set();
  let state = null; // null, "open", "done"
  let indent_level = 0;

  for (const ln of text.split("\n")) {
    const st = ln.trimStart();
    if (!st) continue;

    const current_indent = ln.length - st.length;

    if (st.startsWith("- [ ]")) {
      state = "open";
      indent_level = current_indent;
    } else if (st.startsWith("- [x]") || st.startsWith("- [X]")) {
      state = "done";
      indent_level = current_indent;
    } else if (st.startsWith("- ") || st.startsWith("* ")) {
      // Only reset state if this bullet is at the same or higher level than the parent open thread
      if (state !== null && current_indent <= indent_level) state = null;
    }

    const m = ln.match(PIN_ID_RE);
    if (m && state === "open") pinned.add(m[1]);
  }
  return pinned;
}

export function memref_ids(text) {
  // Anchor the heading to the start of a line. A session log may *quote* the
  // string "## Memory References" inline in prose (e.g. while describing this
  // very check); an un-anchored search would match that mention and scoop a
  // neighbouring section's ids into the references set — a false "over-archived"
  // positive. Match only a real heading line, and bound at the next one.
  const m = text.match(/^## +Memory References[ \t]*$/m);
  if (m === null) return new Set();
  let block = text.slice(m.index + m[0].length);
  const nxt = block.match(/^## +\S/m);
  if (nxt !== null) block = block.slice(0, nxt.index);
  return new Set([...block.matchAll(ID_RE)].map((x) => x[0]));
}

export function load_windows(root) {
  // Defaults track the shipped templates/memory/decay-policy.md (v4.24.0): a repo
  // whose policy omits a field falls back to these. continuity_max_facts is the
  // primary lean signal (count > lines — verbosity/velocity-independent).
  const w = {
    working_window: 3,
    active_window: 8,
    archive_window: 20,
    review_every: 10,
    continuity_max_facts: 30,
    continuity_max_lines: 600,
  };
  const p = join(root, "memory", "decay-policy.md");
  if (existsSync(p)) {
    const t = read_text(p);
    for (const k of Object.keys(w)) {
      const m = t.match(new RegExp(String.raw`${k}\s*:\s*(\d+)`));
      if (m) w[k] = Number.parseInt(m[1], 10);
    }
  }
  return w;
}

function parse_args(args) {
  const strict = args.includes("--strict");
  let root_arg = null;
  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--root" && i + 1 < args.length) root_arg = args[i + 1];
  }
  return { strict, root_arg };
}

export function load_repo(root) {
  // Read the memory/ layer. Returns { cont, pinned, arch, extra, sessions, refs }.
  const mem = join(root, "memory");
  const cont_text = read_text(join(mem, "continuity.md"));
  const cont = parse_footers(cont_text);
  const pinned = pinned_open_threads(cont_text);

  let archive_text = "";
  const archiveDir = join(mem, "archive");
  if (existsSync(archiveDir)) {
    for (const f of readdirSync(archiveDir).filter((x) => x.endsWith(".md")).sort(byCodePoint)) {
      if (basename(f).toUpperCase().startsWith("INDEX")) continue;
      archive_text += read_text(join(archiveDir, f)) + "\n";
    }
  }
  const arch = parse_footers(archive_text);

  // Extra footers from other memory/*.md files (e.g. vision.md) — used only for
  // supersession link resolution in check_dangling; not counted as cont/arch facts.
  let extra_text = "";
  const SKIP = new Set(["continuity.md", "decay-policy.md"]);
  for (const f of readdirSync(mem).filter((x) => x.endsWith(".md")).sort(byCodePoint)) {
    if (SKIP.has(f)) continue;
    const fp = join(mem, f);
    if (statSync(fp).isFile()) extra_text += read_text(fp) + "\n";
  }
  const extra = parse_footers(extra_text);

  const sessDir = join(mem, "sessions");
  const sessions = existsSync(sessDir)
    ? readdirSync(sessDir).filter((x) => x.endsWith(".md")).sort(byCodePoint)
    : [];
  const refs = sessions.map((s) => memref_ids(read_text(join(sessDir, s))));
  return { cont, pinned, arch, extra, sessions, refs };
}

function make_sslu(refs) {
  // sessions_since_last_used: how many sessions back a fact was last referenced.
  return (fid) => {
    let last = -1;
    for (let i = 0; i < refs.length; i++) if (refs[i].has(fid)) last = i;
    return last === -1 ? null : refs.length - 1 - last;
  };
}

function check_duplicates(cont, arch) {
  // (1) a fact must live in exactly one place
  return [...cont.keys()]
    .filter((k) => arch.has(k))
    .sort(byCodePoint)
    .map((fid) => `[both] ${fid} is in BOTH continuity.md and the archive`);
}

function check_over_archived(arch, sslu, aw) {
  // (2) the decay miscount guard: archived-as-faded but still referenced in-window
  const out = [];
  for (const [fid, fields] of arch) {
    if ("superseded-by" in fields || fields.tier === "superseded") continue;
    const s = sslu(fid);
    if (s !== null && s <= aw) {
      out.push(
        `[over-archived] ${fid} archived as faded but last referenced ${s} ` +
          `session(s) ago (<= archive_window ${aw}) — reactivate it`
      );
    }
  }
  return out;
}

function check_overdue(cont, pinned, sslu, aw) {
  // (3) advisory: continuity fact overdue for archival
  //     (core, superseded, and pinned unchecked open threads never decay)
  const out = [];
  for (const [fid, fields] of cont) {
    if (fields.tier === "core" || fields.tier === "superseded" || pinned.has(fid)) continue;
    const s = sslu(fid);
    if (s !== null && s > aw) {
      out.push(`[overdue] ${fid} sslu ${s} > archive_window ${aw} — review may archive it`);
    }
  }
  return out;
}

export function check_session_filenames(sessions) {
  // (5) session filenames must carry a time component (YYYY-MM-DD-HHmmss.md).
  // A date-only name means the agent used the injected context date instead of
  // running `date -u +%Y-%m-%d-%H%M%S` — it breaks same-day lexicographic ordering.
  const DATE_ONLY = /^\d{4}-\d{2}-\d{2}$/;
  return sessions
    .filter((s) => DATE_ONLY.test(s.replace(/\.md$/, "")))
    .map(
      (s) =>
        `[date-only-session] ${s} — missing time component; ` +
        "run `date -u +%Y-%m-%d-%H%M%S` at persist time (not the context date)"
    );
}

export function check_version_manifest(root) {
  // (6) .agent/version.md, IF present, must carry a parseable semver `version:` line.
  // An empty/malformed manifest breaks Mode B upgrade detection — and was a real bug
  // (a truncating stamp one-liner emptied it). A MISSING file is valid (pre-versioning
  // baseline, handled by ENABLE/UPGRADE) and is NOT flagged.
  const p = join(root, ".agent", "version.md");
  if (!existsSync(p) || !statSync(p).isFile()) return [];
  const m = read_text(p).match(/^- \*\*version:\*\*\s*(\d+\.\d+\.\d+)/m);
  if (m === null) {
    return [
      "[version-manifest] .agent/version.md exists but has no parseable " +
        "`- **version:** X.Y.Z` line (empty or malformed) — breaks Mode B upgrade detection",
    ];
  }
  return [];
}

export function check_conflict_markers(root) {
  // (7) No leftover VCS merge-conflict markers in the LIVE top-level memory files —
  // the ones every teammate concurrently edits and the agent reads as truth
  // (continuity.md, instructions.md, vision.md, decay-policy.md, smoke-test.md). We scan
  // `memory/*.md` only (non-recursive): `sessions/` and `archive/` are deliberately
  // EXCLUDED — they are immutable/append narrative that legitimately *quotes* conflict
  // markers (a session log pasting terminal output or a real diff to document it), so
  // scanning them would false-positive. Match git's `<<<<<<<` / `>>>>>>>` and the diff3
  // `|||||||` line markers; deliberately do NOT match a bare `=======` line (a valid
  // Markdown setext heading underline).
  const out = [];
  const mem = join(root, "memory");
  const marker = /^(<{7}|>{7}|\|{7})(\s|$)/;
  if (!existsSync(mem)) return out;
  const files = readdirSync(mem).filter((n) => n.endsWith(".md")).sort(byCodePoint);
  for (const name of files) {
    const lines = read_text(join(mem, name)).split("\n");
    for (let i = 0; i < lines.length; i++) {
      if (marker.test(lines[i])) {
        out.push(
          `[conflict-marker] memory/${name}:${i + 1} unresolved merge-conflict marker ` +
            "— resolve it before committing"
        );
        break; // one report per file is enough
      }
    }
  }
  return out;
}

export function check_dangling(allf) {
  // (4) supersession links resolve
  const out = [];
  for (const [fid, fields] of allf) {
    for (const key of ["superseded-by", "supersedes"]) {
      const tgt = fields[key];
      if (tgt && !allf.has(tgt)) {
        out.push(`[dangling] ${fid} ${key} ${tgt}, which has no footer anywhere`);
      }
    }
  }
  return out;
}

const LAST_REVIEW_RE = /^- \*\*last_review:\*\*\s*([0-9-]+)(?:\s*\|\s*through\s+([0-9][0-9-]*))?/m;

// Count lines the way Python's str.splitlines() does (trailing newline adds no line).
function count_lines(s) {
  if (s === "") return 0;
  const parts = s.split(/\r\n|\r|\n/);
  if (parts[parts.length - 1] === "") parts.pop();
  return parts.length;
}

export function sessions_since_review(sessions, cont_text) {
  // How many session files were written after the last_review 'through' stamp.
  // No last_review recorded (never reviewed) => every session counts as 'since'.
  const stems = sessions.map((s) => s.replace(/\.md$/, ""));
  const m = cont_text.match(LAST_REVIEW_RE);
  if (!m) return stems.length;
  const through = m[2] || m[1]; // prefer the 'through <session-file>' token
  return stems.filter((s) => s > through).length;
}

export function created_sessions_ago(created, stems) {
  // session files dated strictly after `created` (YYYY-MM-DD); approximate (by date).
  if (!created) return null;
  return stems.filter((s) => s.slice(0, 10) > created).length;
}

export function expected_tier(fields, fid, sslu_val, uses_val, created_ago, pinned, ww, acw, aw) {
  // Tier a fact *should* carry per DECAY.md §5 (first match wins). Clamps at
  // 'archive-candidate' — a fact still in continuity is never 'archived'.
  if (fields["superseded-by"] || fields.tier === "superseded") return "superseded";
  if (fields.tier === "core") return "core";
  if (pinned.has(fid)) return fields.tier ?? null;  // pinned: never decays; leave the tier label as-is
  if (sslu_val === null) return fields.tier ?? null;
  if (created_ago !== null && created_ago <= ww && uses_val <= 1) return "working";
  if (sslu_val <= acw) return "active";
  return "archive-candidate";
}

export function check_stale_metadata(cont, pinned, refs, stems, ww, acw, aw) {
  // (9) advisory: stored `tier` disagrees with the tier recomputed from references —
  // review steps 2–3 (apply events / re-tier) were skipped. core/superseded exempt.
  const out = [];
  const sslu = make_sslu(refs);
  for (const [fid, fields] of cont) {
    if (fields.tier === "core" || fields.tier === "superseded" || fields["superseded-by"]) continue;
    const uses_val = refs.reduce((n, ids) => n + (ids.has(fid) ? 1 : 0), 0);
    const et = expected_tier(fields, fid, sslu(fid), uses_val, created_sessions_ago(fields.created, stems), pinned, ww, acw, aw);
    const stored = fields.tier;
    if (et !== null && et !== stored) {
      out.push(
        `[stale-metadata] ${fid} tier '${stored}' should be '${et}' (sslu ${sslu(fid)}) ` +
        "— review steps 2–3 (re-tier) skipped; run refresh-metadata or a review"
      );
    }
  }
  return out;
}

export function check_continuity_health(cont, sessions, cont_text, cont_lines, re_every, max_facts, max_lines, pinned = new Set(), archivable = null) {
  // (8) advisory cadence/size triggers — what would have caught a real product repo
  // that ran 61 sessions and never archived (review never fired in the field).
  // All advisory (WARN): a review is a human/agent ritual, never a hard gate.
  // `archivable` (optional) = count of entries a review could archive right now (facts overdue
  // for decay + superseded facts). When it's 0, a lines-only breach can't be honestly cleared by
  // a review, so the message says so instead of nudging toward premature archival (v4.28.3).
  const out = [];
  const ssr = sessions_since_review(sessions, cont_text);
  if (ssr >= re_every) {
    out.push(
      `[review-overdue] ${ssr} session(s) since last review >= review_every ` +
        `${re_every} — run the REVIEW.md ritual`
    );
  }
  // Count only decay-eligible facts — exclude tier:core (structural invariants) and pinned
  // open threads (active workstreams). Those can never be archived, so counting them against
  // the cap produces permanent noise after a correct review (field report: mercury-composable).
  let nfacts = 0;
  for (const [fid, fields] of cont) {
    if (fields.tier !== "core" && !pinned.has(fid)) nfacts++;
  }
  if (nfacts > max_facts) {
    out.push(
      `[continuity-bloat] ${nfacts} decay-eligible facts > continuity_max_facts ` +
        `${max_facts} — a review is due to lean it down`
    );
  }
  if (cont_lines > max_lines) {
    if (archivable === 0) {
      // Lines over budget but a review has nothing to archive right now (nothing faded past
      // archive_window, nothing superseded). "A review will lean it down" would be dishonest and
      // pressures archiving an *active* fact — REVIEW.md's costliest error. Name the real lever
      // instead (field report: mercury-composable, a complex repo's dense active facts).
      out.push(
        `[continuity-bloat] continuity.md ${cont_lines} lines > continuity_max_lines ` +
          `${max_lines} — but nothing is archivable yet; the excess is active/dense facts. ` +
          `Condense shipped decisions, or raise continuity_max_lines in decay-policy.md if ` +
          `this repo is legitimately large.`
      );
    } else {
      out.push(
        `[continuity-bloat] continuity.md ${cont_lines} lines > continuity_max_lines ` +
          `${max_lines} — a review is due to lean it down`
      );
    }
  }
  return out;
}

function report({ cont, arch, sessions, acw, aw, warns, errors, strict }) {
  console.log(
    `memory-lint: ${cont.size} continuity facts, ${arch.size} archived, ` +
      `${sessions.length} sessions; windows active=${acw} archive=${aw}`
  );
  for (const line of warns) console.log("WARN  " + line);
  for (const line of errors) console.log("ERROR " + line);
  if (errors.length) {
    console.log(`FAIL: ${errors.length} error(s), ${warns.length} warning(s)`);
    return 1;
  }
  if (warns.length && strict) {
    console.log(`FAIL (strict): ${warns.length} warning(s)`);
    return 1;
  }
  console.log(`OK: 0 errors, ${warns.length} warning(s)`);
  return 0;
}

export function main(argv) {
  const args = argv ?? process.argv.slice(2);
  const { strict, root_arg } = parse_args(args);
  const root = find_root(root_arg || process.cwd());
  if (!root) {
    console.error("memory-lint: could not find memory/continuity.md");
    return 2;
  }

  const { cont, pinned, arch, extra, sessions, refs } = load_repo(root);
  const w = load_windows(root);
  const aw = w.archive_window;
  const acw = w.active_window;
  const sslu = make_sslu(refs);

  const cont_text = read_text(join(root, "memory", "continuity.md"));
  const cont_lines = count_lines(cont_text);

  const errors = [
    ...check_duplicates(cont, arch),
    ...check_over_archived(arch, sslu, aw),
    ...check_version_manifest(root),
    ...check_conflict_markers(root),
  ];
  const stems = sessions.map((s) => s.replace(/\.md$/, ""));
  const overdue = check_overdue(cont, pinned, sslu, aw);
  // What a review could archive right now: facts overdue for decay + superseded facts. When 0,
  // a lines-only bloat breach has no honest fix via archival (v4.28.3).
  let superseded_ct = 0;
  for (const fields of cont.values()) if (fields.tier === "superseded") superseded_ct++;
  const archivable = overdue.length + superseded_ct;
  const warns = [
    ...overdue,
    ...check_dangling(new Map([...cont, ...arch, ...extra])),
    ...check_session_filenames(sessions),
    ...check_continuity_health(
      cont, sessions, cont_text, cont_lines,
      w.review_every, w.continuity_max_facts, w.continuity_max_lines, pinned, archivable
    ),
    ...check_stale_metadata(cont, pinned, refs, stems, w.working_window, acw, aw),
  ];

  return report({ cont, arch, sessions, acw, aw, warns, errors, strict });
}

// Run only when executed directly, not when imported by the test suite.
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  process.exit(main());
}
