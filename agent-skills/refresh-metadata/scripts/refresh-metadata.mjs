#!/usr/bin/env node
// refresh-metadata.mjs — recompute continuity.md fact footers from the session log.
//
// Node port of refresh-metadata.py, at output parity (shared fixtures are the contract).
// Executes REVIEW.md steps 2–3 (apply events + re-tier) deterministically: recompute
// last_used / uses / tier for every fact from `## Memory References` across sessions, and
// write footers back. The "full rebuild" path — pure arithmetic, no judgment, safe to
// mechanize. Does NOT decide what to archive (that's archive-fact + agent judgment),
// touch core/superseded, or fabricate fields; clamps tier at 'archive-candidate'.
// Safe by construction: reads into memory, writes once.
//
// Usage: node refresh-metadata.mjs [--root PATH] [--dry-run]
// Exit:  0 = refreshed (or dry-run), 2 = could not locate memory/. Run memory-lint after.

import { readFileSync, writeFileSync, existsSync, statSync, readdirSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ID_RE = /[a-z][a-z0-9]*(?:-[a-z0-9]+)+/g;
const FOOTER_RE = /<!--\s*id:\s*([a-z0-9-]+)\s*\|([^\n]*?)-->/g;

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

const read_text = (p) => readFileSync(p, "utf-8");

function parse_fields(blob) {
  const fields = {};
  for (const part of blob.split("|")) {
    const i = part.indexOf(":");
    if (i !== -1) fields[part.slice(0, i).trim()] = part.slice(i + 1).trim();
  }
  return fields;
}

function memref_ids(text) {
  const m = text.match(/^## +Memory References[ \t]*$/m);
  if (m === null) return new Set();
  let block = text.slice(m.index + m[0].length);
  const nxt = block.match(/^## +\S/m);
  if (nxt !== null) block = block.slice(0, nxt.index);
  return new Set([...block.matchAll(ID_RE)].map((x) => x[0]));
}

function pinned_open_threads(text) {
  const pinned = new Set();
  let state = null, indent = 0;
  for (const ln of text.split("\n")) {
    const st = ln.trimStart();
    if (!st) continue;
    const cur = ln.length - st.length;
    if (st.startsWith("- [ ]")) { state = "open"; indent = cur; }
    else if (st.startsWith("- [x]") || st.startsWith("- [X]")) { state = "done"; indent = cur; }
    else if ((st.startsWith("- ") || st.startsWith("* ")) && state !== null && cur <= indent) state = null;
    const m = ln.match(/<!--\s*id:\s*([a-z0-9-]+)/);
    if (m && state === "open") pinned.add(m[1]);
  }
  return pinned;
}

function load_sessions(root) {
  const sdir = join(root, "memory", "sessions");
  const stems = existsSync(sdir)
    ? readdirSync(sdir).filter((f) => f.endsWith(".md")).map((f) => f.slice(0, -3)).sort()
    : [];
  const refs = stems.map((s) => memref_ids(read_text(join(sdir, s + ".md"))));
  return { stems, refs };
}

function load_windows(root) {
  const w = { working_window: 3, active_window: 8, archive_window: 20 };
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

export function expected_tier(fields, fid, sslu_val, uses_val, created_ago, pinned, ww, acw) {
  if (fields["superseded-by"] || fields.tier === "superseded") return "superseded";
  if (fields.tier === "core") return "core";
  if (pinned.has(fid)) return fields.tier ?? null;  // pinned: never decays; leave the tier label as-is
  if (sslu_val === null) return fields.tier ?? null;
  if (created_ago !== null && created_ago <= ww && uses_val <= 1) return "working";
  if (sslu_val <= acw) return "active";
  return "archive-candidate";
}

export function refresh(root, dry_run) {
  const cont_path = join(root, "memory", "continuity.md");
  const text = read_text(cont_path);
  const { stems, refs } = load_sessions(root);
  const pinned = pinned_open_threads(text);
  const w = load_windows(root);
  const ww = w.working_window, acw = w.active_window;

  const changes = [];
  const new_text = text.replace(FOOTER_RE, (full, fid, blob) => {
    const fields = parse_fields(blob);
    if (fields.tier === "core" || fields.tier === "superseded" || fields["superseded-by"]) return full;
    const hits = [];
    refs.forEach((ids, i) => { if (ids.has(fid)) hits.push(i); });
    if (hits.length === 0) return full;
    const uses_val = hits.length;
    const last = hits[hits.length - 1];
    const last_used = stems[last].slice(0, 10);
    const sslu_val = stems.length - 1 - last;
    const created_ago = fields.created ? stems.filter((s) => s.slice(0, 10) > fields.created).length : null;
    const tier = expected_tier(fields, fid, sslu_val, uses_val, created_ago, pinned, ww, acw);
    let next = full;
    if ("last_used" in fields) next = next.replace(/(\blast_used:\s*)[0-9-]+/, `$1${last_used}`);
    if ("uses" in fields) next = next.replace(/(\buses:\s*)\d+/, `$1${uses_val}`);
    if ("tier" in fields && tier) next = next.replace(/(\btier:\s*)[a-z-]+/, `$1${tier}`);
    if (next !== full) changes.push([fid, fields.tier, tier, fields.uses, String(uses_val)]);
    return next;
  });

  if (changes.length === 0) return { code: 0, msg: "all fact footers already match the reference log — nothing to refresh" };

  const lines = changes.map(([fid, ot, nt, ou, nu]) => {
    const bits = [];
    if (ot !== nt) bits.push(`tier ${ot}→${nt}`);
    if (ou !== nu) bits.push(`uses ${ou}→${nu}`);
    return bits.length ? `  ${fid}: ${bits.join(", ")}` : `  ${fid}: last_used`;
  });
  const summary = lines.join("\n");
  if (dry_run) return { code: 0, msg: `DRY-RUN — would refresh ${changes.length} footer(s):\n${summary}\n(no files changed)` };

  writeFileSync(cont_path, new_text, "utf-8"); // safe: new_text already in memory
  return { code: 0, msg: `refreshed ${changes.length} footer(s) from the reference log:\n${summary}\nNow run memory-lint to confirm.` };
}

function parse_args(argv) {
  const out = { root: null, dry_run: false };
  for (let i = 0; i < argv.length; i++) {
    if (argv[i] === "--root") out.root = argv[++i];
    else if (argv[i] === "--dry-run") out.dry_run = true;
  }
  return out;
}

export function main(argv) {
  const args = parse_args(argv ?? process.argv.slice(2));
  const root = find_root(args.root || process.cwd());
  if (!root) {
    console.error("refresh-metadata: could not find memory/continuity.md");
    return 2;
  }
  const { code, msg } = refresh(root, args.dry_run);
  console.log("refresh-metadata: " + msg);
  return code;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  process.exit(main());
}
