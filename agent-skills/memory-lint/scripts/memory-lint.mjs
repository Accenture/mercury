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
//         node memory-lint.mjs --scan-files FILE...   (credential-class [secret-material]
//         scan of arbitrary config files; exit 1 on findings)
// Exit:   0 = clean (no errors), 1 = integrity error(s) (or warnings under
//         --strict), 2 = could not locate the memory/ layer.

import { readFileSync, readdirSync, existsSync, statSync } from "node:fs";
import { resolve, dirname, join, basename, sep } from "node:path";
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
    closed_narrative_max_lines: 150,
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
  let scan_files = null;
  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--root" && i + 1 < args.length) root_arg = args[i + 1];
    if (args[i] === "--scan-files") {
      scan_files = args.slice(i + 1); // everything after the flag is a path
      break;
    }
  }
  return { strict, root_arg, scan_files };
}

export function load_thread_files(mem) {
  // memory/open-threads/*.md — one Open Thread per file (v4.39.0). Returns
  // [[basename, text]]; an absent directory (pre-4.39.0 layout) is an empty list.
  // Thread files are live continuity-domain facts: their footers merge into `cont`
  // and their checkbox state feeds the pinned set, so every decay/reference rule
  // applies to them unchanged — only the storage location moved (merge-scale).
  const tdir = join(mem, "open-threads");
  if (!existsSync(tdir)) return [];
  return readdirSync(tdir)
    .filter((x) => x.endsWith(".md"))
    .sort(byCodePoint)
    .map((n) => [n, read_text(join(tdir, n))]);
}

export function load_repo(root) {
  // Read the memory/ layer. Returns { cont, pinned, arch, extra, sessions, refs, threads }.
  const mem = join(root, "memory");
  const cont_text = read_text(join(mem, "continuity.md"));
  const cont = parse_footers(cont_text);
  const pinned = pinned_open_threads(cont_text);

  const threads = load_thread_files(mem);
  for (const [, ttext] of threads) {
    for (const [k, v] of parse_footers(ttext)) cont.set(k, v);
    for (const fid of pinned_open_threads(ttext)) pinned.add(fid);
  }

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
  return { cont, pinned, arch, extra, sessions, refs, threads };
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
  const live = readdirSync(mem).filter((n) => n.endsWith(".md")).sort(byCodePoint)
    .map((n) => [join(mem, n), `memory/${n}`]);
  const tdir = join(mem, "open-threads"); // thread files are live truth too (v4.39.0)
  if (existsSync(tdir)) {
    for (const n of readdirSync(tdir).filter((x) => x.endsWith(".md")).sort(byCodePoint)) {
      live.push([join(tdir, n), `memory/open-threads/${n}`]);
    }
  }
  for (const [path, rel] of live) {
    const lines = read_text(path).split("\n");
    for (let i = 0; i < lines.length; i++) {
      if (marker.test(lines[i])) {
        out.push(
          `[conflict-marker] ${rel}:${i + 1} unresolved merge-conflict marker ` +
            "— resolve it before committing"
        );
        break; // one report per file is enough
      }
    }
  }
  return out;
}

export function check_thread_files(threads) {
  // (12) the thread-file contract (v4.39.0): memory/open-threads/ holds ONE Open Thread
  // per file, named thread-<id>.md after its footer id. Filename = identity is what makes
  // concurrent thread work merge-free (parallel branches touch different files), so drift
  // here is an ERROR, not style: a wrong name or a second block re-creates the shared-file
  // conflict surface this layout exists to remove.
  const out = [];
  for (const [name, text] of threads) {
    const rel = `memory/open-threads/${name}`;
    const footers = [...text.matchAll(FOOTER_RE)];
    if (footers.length === 0) {
      out.push(`[thread-file] ${rel} has no fact footer — a thread file carries exactly one \`<!-- id: … -->\``);
      continue;
    }
    if (footers.length > 1) {
      out.push(`[thread-file] ${rel} holds ${footers.length} footers — one thread per file; split it`);
      continue;
    }
    const fid = footers[0][1];
    const expect = `thread-${fid}.md`;
    if (name !== expect) {
      out.push(`[thread-file] ${rel} should be named ${expect} (filename = the footer id)`);
    }
    const first = text.split(/\r?\n/).find((ln) => ln.trim()) ?? "";
    if (!first.startsWith("- [ ]") && !first.startsWith("- [x]") && !first.startsWith("- [X]")) {
      out.push(`[thread-file] ${rel} does not start with a \`- [ ]\`/\`- [x]\` bullet — file content is exactly the thread block`);
    }
  }
  return out;
}

export function check_duplicate_ids(cont_text, threads) {
  // (13) an id exists exactly ONCE across the live layer (continuity + thread files).
  // Two live footers with one id is the silent-fork shape a same-id creation collision
  // on parallel branches (or a bad hand-merge) produces — [both] covers live-vs-archive,
  // this covers live-vs-live. Without it, parse_footers' id-keyed map hides the twin.
  const where = new Map();
  const surfaces = [["memory/continuity.md", cont_text]];
  for (const [n, txt] of threads) surfaces.push([`memory/open-threads/${n}`, txt]);
  for (const [src, text] of surfaces) {
    for (const m of text.matchAll(FOOTER_RE)) {
      if (!where.has(m[1])) where.set(m[1], []);
      where.get(m[1]).push(src);
    }
  }
  const out = [];
  for (const fid of [...where.keys()].sort(byCodePoint)) {
    const srcs = where.get(fid);
    if (srcs.length > 1) {
      out.push(
        `[duplicate-id] ${fid} has ${srcs.length} footers across the live layer ` +
          `(${srcs.join(", ")}) — an id exists exactly once; merge the copies or re-id one`
      );
    }
  }
  return out;
}

export function check_duplicate_state_keys(root) {
  // (14) `## Project State` holds SCALARS — one value each, latest wins. This is the
  // backstop for a union-style hand merge that kept both sides of a bumped scalar, or a
  // hand-edited header. Deliberately scoped to `## Project State`: a repeated key anywhere
  // else is a bullet, not a scalar, and repetition there is legitimate.
  // (Absorbed from PR #27 — credit: Roland Heusser.)
  const out = [];
  const p = join(root, "memory", "continuity.md");
  if (!existsSync(p) || !statSync(p).isFile()) return out;
  const keyRe = /^-\s+\*\*([a-z_]+):\*\*/;
  const lines = read_text(p).split(/\r?\n/);
  const seen = new Map();
  let inState = false;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (line.startsWith("## ")) {
      if (inState) break;
      inState = line.trim() === "## Project State";
      continue;
    }
    if (!inState) continue;
    const m = keyRe.exec(line);
    if (!m) continue;
    const key = m[1];
    if (seen.has(key)) {
      out.push(
        `[duplicate-state-key] memory/continuity.md:${i + 1} '${key}' is set twice ` +
          `(also line ${seen.get(key)}) — Project State fields are scalars. Usually a union ` +
          `merge keeping both sides: delete the stale line, keeping the later value.`
      );
    } else {
      seen.set(key, i + 1);
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

export function closed_narrative_lines(cont_text) {
  // Non-empty lines belonging to completed `- [x]` thread records (checkbox line
  // through footer), the block ending at the next open thread or heading. This is
  // the measured bloat class (mercury-composable field report, 2026-08-21: 64% of
  // continuity was closed-thread narrative whose canonical home is the origin log).
  let in_block = false;
  let count = 0;
  for (const line of cont_text.split(/\r?\n/)) {
    if (/^- \[x\]/.test(line)) in_block = true;
    else if (/^- \[ \]/.test(line) || line.startsWith("#")) in_block = false;
    if (in_block && line.trim()) count += 1;
  }
  return count;
}

export function check_closed_thread_bloat(cont_text, cap, threads = []) {
  // (11) advisory: completed threads should wait out archive_window as terse
  // stubs (3–6 lines), not full ship narratives — REVIEW.md condenses them.
  // Measured across every live surface: continuity + the thread files (v4.39.0).
  let n = closed_narrative_lines(cont_text);
  for (const [, txt] of threads) n += closed_narrative_lines(txt);
  if (n <= cap) return [];
  return [
    `[closed-thread-bloat] ${n} line(s) of completed [x] thread records > ` +
      `closed_narrative_max_lines ${cap} — condense them to 3-6-line stubs at the next ` +
      `review (REVIEW.md; the full narrative lives in each thread's origin session log), ` +
      `or raise closed_narrative_max_lines in decay-policy.md.`,
  ];
}

// (10) [secret-material] — committed memory surfaces must not carry credentials or PII.
// Field incident (reported 2026-08-13, a client repo's DLP scanner): smoke-test output pasted into a
// session log leaked a live OAuth client secret — session logs are committed & shared, so
// anything pasted into them ships to every clone. This check is the deterministic backstop
// behind the memory/PROTOCOL.md redaction rule. Advisory (WARN): the script detects *shapes*; whether
// a hit is a real secret stays human/agent judgment. Unlike check 7 it DOES scan sessions/
// and archive/ — that's where pasted output lives — and it never echoes the matched value
// (a lint line quoting the secret would just amplify the leak into terminals and CI logs).
const SECRET_VALUE_PATTERNS = [
  ["aws-access-key-id", /\bAKIA[0-9A-Z]{16}\b/],
  ["github-token", /\b(?:gh[pousr]_[A-Za-z0-9]{36,}|github_pat_[A-Za-z0-9_]{22,})\b/],
  ["gitlab-token", /\bglpat-[A-Za-z0-9_-]{20,}\b/],
  ["slack-token", /\bxox[baprs]-[A-Za-z0-9-]{10,}\b/],
  ["google-api-key", /\bAIza[0-9A-Za-z_-]{35}\b/],
  ["private-key-block", /-----BEGIN [A-Z ]*PRIVATE KEY-----/],
  ["jwt", /\beyJ[A-Za-z0-9_-]{8,}\.eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,}\b/],
];
// Credential-KEY assignment with a literal value (clientSecret='…', password: …, api_key=…).
// Keyed on the *name*, not the value shape — this is what catches a rendered JAAS line.
const ASSIGNMENT_RE = new RegExp(
  String.raw`\b([A-Za-z0-9_.\-]*(?:secret|password|passwd|credential|api[_.\-]?key|apikey` +
    String.raw`|access[_.\-]?token|auth[_.\-]?token|bearer[_.\-]?token)[A-Za-z0-9_.\-]*)` +
    // Permit a closing quote around a JSON/YAML key (`"client_secret": "…"`) without
    // making the quote part of the reported key.
    String.raw`['"\`]?` +
    // Backtick is a value delimiter alongside quotes: every scanned surface is markdown, where
    // assignments are typically quoted as inline code (`key=VALUE`) — without this, the closing
    // backtick rides into the captured value and defeats the enum-constant exclusion (v4.33.2).
    // Semicolon is a value delimiter too: JAAS/properties lines terminate with `;`
    // (`password={CHANGE_THIS};`) and it otherwise rides into the value, defeating the
    // placeholder rules (field probe 2026-08-14). A real secret containing `;` still flags on
    // its captured prefix.
    String.raw`\s*[=:]\s*(['"\`]?)([^\s'"\`;]{8,})\2`,
  "gi"
);
const AUTHORIZATION_RE =
  /\b((?:proxy[_.\-]?)?authorization)\s*:\s*(?:(?:bearer|basic)\s+)?(['"`]?)([^\s'"`]{8,})\2/gi;
// An authorization VALUE that is a dotted lowercase identifier (`v1.basic.auth`) is a
// service/route/handler reference, never a token — real credentials carry uppercase, digit
// runs, or symbols beyond dots (field probe 2026-08-14: mercury REST configs). Case-sensitive
// on purpose: any uppercase keeps it flagged.
const ROUTE_REF_RE = /^[a-z][a-z0-9_-]*(?:\.[a-z0-9_-]+)+$/;
// Postman collections split the pair: `"key": "client_secret", "value": "…"` — the credential
// key is itself the VALUE of a "key" field (the field-incident artifact class, 2026-08-14).
const POSTMAN_KV_RE = new RegExp(
  String.raw`"key"\s*:\s*"([^"]*(?:secret|password|passwd|credential|api[_.\-]?key|apikey` +
    String.raw`|access[_.\-]?token|auth[_.\-]?token|bearer[_.\-]?token|authorization)[^"]*)"` +
    String.raw`\s*,\s*"(?:value|src)"\s*:\s*"([^"]{8,})"`,
  "gi"
);
const PLACEHOLDER_VALUE_RE =
  /^(?:redacted|changeme|change-me|placeholder|example|sample|dummy|demo|test|todo|x{4,}|your[-_][A-Za-z0-9_.\-]+|(?:changeme|change-me|example|sample|dummy|demo|test|placeholder)[-_][A-Za-z0-9_.\-]+|[A-Za-z0-9_.\-]+[-_](?:changeme|change-me|example|sample|dummy|demo|test|placeholder))$/i;
// Accept a bare or dotted reference with no fallback (`${VAR}`, `${VAR:}`, `${a.b}`).
// A non-empty default may itself be a rendered secret, so `${VAR:-secret}` must flag —
// except when the fallback is provably a placeholder (see TEMPLATE_DEFAULT_RE below).
// Also: GitHub-Actions expressions (`${{ secrets.X }}`) and single-brace placeholders
// (`{CHANGE_THIS}` — the commented-JAAS-template form, field probe 2026-08-14).
const TEMPLATE_VALUE_RE =
  /^(?:\$\{[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*:?\}|\$\{\{[^{}]+\}\}|\{[^{}\s]+\}|\$\([^)]+\)|\{\{[^{}]+\}\}|<[A-Za-z0-9_.:\-]+>|%\([A-Za-z_][A-Za-z0-9_]*\)s|\(REDACTED\)|\*+)$/i;
// A template reference WITH a non-empty fallback: safe only when the fallback itself is
// provably a placeholder — under the 8-char value floor, or passing the placeholder word
// rules (`${DEMO_PEER_TOKEN:demo}`, field probe 2026-08-14). `${CLIENT_SECRET:-Real…}` with a
// credential-shaped fallback keeps flagging (the v4.33.4 rule).
const TEMPLATE_DEFAULT_RE = /^\$\{[A-Za-z_][A-Za-z0-9_.]*:-?([^{}]+)\}$/;
const ENUM_KEY_RE = /(?:^|[_.\-])(?:source|type|mode|mechanism|strategy)$/i;
const EMAIL_RE = /\b([A-Za-z0-9._%+-]+)@([A-Za-z0-9.-]+\.[A-Za-z]{2,})\b/g;
const SSN_RE = /\b\d{3}-\d{2}-\d{4}\b/;
const E164_RE = /\+\d{10,15}\b/;
const CARD_RE = /\b(?:\d{4}[ -]){3}\d{4}\b|\b\d{13,19}\b/g;
const HOME_PATH_RE = /(?:\/Users\/|\/home\/|[A-Za-z]:\\Users\\)([A-Za-z0-9._-]{2,})/g;
const HOME_OK = new Set(["runner", "user", "username", "vsts_azpcontainer"]); // well-known CI users, not PII

function is_placeholder_value(key, v) {
  // Values that are templates, redactions, or number/date/version shapes — not secrets.
  // The tool's own opt-down knob is knob vocabulary, not a credential: the pre-commit guard's
  // blocking message itself prints "AGENT_MEMORY_SECRET_GUARD=advisory", so a memory file
  // documenting that guidance would otherwise self-flag (field report, 2026-08-19).
  // Value-constrained on purpose — an arbitrary value under this key still flags, so the
  // exemption cannot be used as a smuggling envelope. Trailing ).,  punctuation is tolerated
  // because prose/parenthesized guidance rides it into the captured value — the guard's own
  // line ends "…=advisory)" (same capture behavior v4.33.2 fixed for backticks).
  if (["AGENT_MEMORY_SECRET_GUARD", "AGENT-MEMORY.SECRETGUARD"].includes(key.toUpperCase())
      && /^(?:advisory|enforcing)[).,]*$/i.test(v)) {
    return true; // the git-config spelling (agent-memory.secretguard) is the same knob
  }
  if (TEMPLATE_VALUE_RE.test(v)) return true;
  const m = TEMPLATE_DEFAULT_RE.exec(v);
  if (m && (m[1].length < 8 || is_placeholder_value(key, m[1]))) {
    return true; // template whose non-empty fallback is itself provably a placeholder
  }
  if (/^[\d.\-:/T]+$/.test(v)) return true; // timestamps, dates, versions, counts (max_tokens: 128000, …)
  // ALL-CAPS is safe only on keys that explicitly describe an enum dimension. Treating every
  // uppercase value as an enum lets ordinary uppercase passwords and opaque secrets bypass the
  // assignment detector. (The motivating field line is credentials.source=OAUTHBEARER.)
  if (ENUM_KEY_RE.test(key) && /^[A-Z][A-Z0-9_]{2,}$/.test(v)) return true;
  return PLACEHOLDER_VALUE_RE.test(v);
}

function is_public_email(local, domain) {
  const l = local.toLowerCase();
  const d = domain.toLowerCase();
  return (
    l === "git" || l === "noreply" || l === "no-reply" || l.endsWith("+noreply") ||
    d.includes("noreply") || d.startsWith("example.") || d.includes(".example") ||
    d.endsWith(".invalid") || d.endsWith(".test") || d.endsWith(".local") || d.endsWith(".localhost")
  );
}

function luhn_ok(digits) {
  let total = 0;
  const rev = [...digits].reverse();
  for (let i = 0; i < rev.length; i++) {
    let d = Number.parseInt(rev[i], 10);
    if (i % 2 === 1) d = d * 2 > 9 ? d * 2 - 9 : d * 2;
    total += d;
  }
  return total % 10 === 0;
}

export function check_secret_material(root) {
  const mem = join(root, "memory");
  const files = [];
  const addDir = (dir, prefix) => {
    if (!existsSync(dir)) return;
    for (const n of readdirSync(dir).filter((x) => x.endsWith(".md")).sort(byCodePoint)) {
      const fp = join(dir, n);
      if (statSync(fp).isFile()) files.push([fp, `${prefix}${n}`]);
    }
  };
  addDir(mem, "memory/");
  addDir(join(mem, "open-threads"), "memory/open-threads/");
  addDir(join(mem, "sessions"), "memory/sessions/");
  addDir(join(mem, "archive"), "memory/archive/");

  const out = [];
  for (const [path, rel] of files) {
    out.push(...scan_lines(path, rel, false));
  }
  return out;
}

// One consolidated guidance line accompanies [secret-material] findings — printed ONCE per
// run by the consumer (report() for a full lint, the --scan-files CLI branch, the pre-commit
// hook's footer), never repeated per finding (field feedback, 2026-08-14 regression test).
// Scanner-neutral name: enterprise secret scanners flag trigger-word identifiers assigned
// string literals (Snyk field FP, 2026-08-25) — the suites' hygiene test enforces this.
const GUIDANCE =
  "  -> committed files are shared: redact to (REDACTED) or move the value out; a live " +
  "credential is EXPOSED — rotate it (git history keeps the original; see the memory/PROTOCOL.md " +
  "redaction rule)";

function scan_lines(path, rel, credential_only) {
  // One file's [secret-material] scan. credential_only=true is the config-file profile:
  // token shapes, assignments, Authorization headers, private keys — NOT the PII classes
  // (email/SSN/card/phone/home-path), which are memory-layer checks: config files
  // legitimately carry contact emails and paths; credential material is never legitimate.
  const found = new Map(); // category -> [first_line, count, detail]
  const tally = (cat, line_no, detail = "") => {
    if (found.has(cat)) found.get(cat)[1] += 1;
    else found.set(cat, [line_no, 1, detail]);
  };

  const lines = read_text(path).split(/\r\n|\r|\n/);
  for (let idx = 0; idx < lines.length; idx++) {
    const line = lines[idx];
    const i = idx + 1;
    // Explicit waiver for deliberately-quoted examples (a log *documenting* a leak
    // cleanup legitimately quotes the patterns). Tag the line, all categories skip it:
    if (line.includes("lint:allow-secret-material")) continue;
    for (const [cat, rx] of SECRET_VALUE_PATTERNS) {
      if (rx.test(line)) tally(cat, i);
    }
    for (const m of line.matchAll(ASSIGNMENT_RE)) {
      if (!is_placeholder_value(m[1], m[3])) tally("credential-assignment", i, ` key '${m[1]}'`);
    }
    for (const m of line.matchAll(POSTMAN_KV_RE)) {
      if (!is_placeholder_value(m[1], m[2])) tally("credential-assignment", i, ` key '${m[1]}'`);
    }
    for (const m of line.matchAll(AUTHORIZATION_RE)) {
      if (ROUTE_REF_RE.test(m[3])) continue; // dotted lowercase service/route reference, not a token
      if (!is_placeholder_value(m[1], m[3])) tally("authorization-header", i);
    }
    if (credential_only) continue;
    for (const m of line.matchAll(EMAIL_RE)) {
      if (!is_public_email(m[1], m[2])) tally("email", i);
    }
    if (SSN_RE.test(line)) tally("ssn", i);
    if (E164_RE.test(line)) tally("phone-e164", i);
    for (const m of line.matchAll(CARD_RE)) {
      const digits = m[0].replaceAll(/[ -]/g, "");
      if (digits.length >= 13 && digits.length <= 19 && luhn_ok(digits)) tally("payment-card", i);
    }
    for (const m of line.matchAll(HOME_PATH_RE)) {
      // need a letter/digit in the username — `/Users/...` is a placeholder, not a path
      if (!HOME_OK.has(m[1].toLowerCase()) && /[A-Za-z0-9]/.test(m[1])) tally("home-path", i);
    }
  }

  const out = [];
  for (const cat of [...found.keys()].sort(byCodePoint)) {
    const [line_no, count, detail] = found.get(cat);
    out.push(`[secret-material] ${rel}:${line_no} ${cat}${detail} (${count} hit(s), first at line ${line_no})`);
  }
  return out;
}

export function scan_secret_files(paths) {
  // `--scan-files` mode (v4.34.0): credential-class scan of arbitrary config files —
  // used by the pre-commit hook on staged .json/.yml/.yaml/.properties/.env/.toml/.ini
  // blobs and by the forge CI wrappers on changed files. Paths are reported as given;
  // missing paths are skipped (a staged blob mirror owns existence).
  const out = [];
  for (const p of paths) {
    if (!existsSync(p) || !statSync(p).isFile()) continue;
    out.push(...scan_lines(p, p.split(sep).join("/"), true));
  }
  return out;
}

function report({ cont, arch, sessions, acw, aw, warns, errors, strict }) {
  console.log(
    `memory-lint: ${cont.size} live facts (continuity + open-threads), ${arch.size} archived, ` +
      `${sessions.length} sessions; windows active=${acw} archive=${aw}`
  );
  for (const line of warns) console.log("WARN  " + line);
  if (warns.some((w) => w.includes("[secret-material]"))) {
    console.log(GUIDANCE); // once per run, not per finding
  }
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
  const { strict, root_arg, scan_files } = parse_args(args);
  if (scan_files !== null) {
    // --scan-files mode: credential-class scan of the given paths, nothing else.
    // Exit 1 when findings exist (the calling wrapper owns advisory-vs-block semantics).
    const findings = scan_secret_files(scan_files);
    for (const line of findings) console.log("WARN  " + line);
    if (findings.length) console.log(GUIDANCE); // once per run, not per finding
    return findings.length ? 1 : 0;
  }
  const root = find_root(root_arg || process.cwd());
  if (!root) {
    console.error("memory-lint: could not find memory/continuity.md");
    return 2;
  }

  const { cont, pinned, arch, extra, sessions, refs, threads } = load_repo(root);
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
    ...check_thread_files(threads),
    ...check_duplicate_ids(cont_text, threads),
    ...check_duplicate_state_keys(root),
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
    ...check_closed_thread_bloat(cont_text, w.closed_narrative_max_lines, threads),
    ...check_stale_metadata(cont, pinned, refs, stems, w.working_window, acw, aw),
    ...check_secret_material(root),
  ];

  return report({ cont, arch, sessions, acw, aw, warns, errors, strict });
}

// Run only when executed directly, not when imported by the test suite.
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  process.exit(main());
}
