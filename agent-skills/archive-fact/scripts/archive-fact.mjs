#!/usr/bin/env node
// archive-fact.mjs — safely MOVE a decayed fact from continuity.md to the archive.
//
// Node port of archive-fact.py, kept at output parity (shared test fixtures are the
// cross-runtime contract). Executes the mechanical half of REVIEW.md step 4: the agent
// decides *what* to archive (meaning); this performs the *move* safely (mechanics).
//
// Why: `open(f,"w").write(open(f).read()+…)` truncates before the read and wiped this
// repo's archive once already. This reads the whole file into memory FIRST and writes
// once, so truncation is structurally impossible.
//
// Usage: node archive-fact.mjs --id ID [--id ID2 ...] [--reason REASON]
//                              [--quarter YYYY-Qn] [--note TEXT] [--root PATH] [--dry-run]
// Exit:  0 = moved (or dry-run ok), 1 = refused, 2 = could not locate memory/.
//        Run `memory-lint` afterward to confirm.

import { readFileSync, writeFileSync, appendFileSync, existsSync, statSync, readdirSync } from "node:fs";
import { resolve, dirname, join, basename } from "node:path";
import { fileURLToPath } from "node:url";

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

function footer_line_index(lines, fid) {
  const esc = fid.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const pat = new RegExp(`<!--\\s*id:\\s*${esc}\\s*\\|`);
  for (let i = 0; i < lines.length; i++) if (pat.test(lines[i])) return i;
  return null;
}

function block_top(lines, footer_idx) {
  // walk back from the footer to the nearest column-0 list bullet ('- ')
  for (let i = footer_idx; i >= 0; i--) if (lines[i].startsWith("- ")) return i;
  return footer_idx;
}

function derive_note(block_lines) {
  let first = block_lines[0];
  first = first.replace(/^- \[[ xX]\]\s*/, "");
  first = first.replace(/^- \s*/, "");
  first = first.replaceAll("**", "").trim();
  first = first.replace(/\s+/g, " ");
  return first.length > 90 ? first.slice(0, 90).replace(/\s+$/, "") + "…" : first;
}

function derive_quarter(y, mo) {
  return `${y}-Q${Math.floor((mo - 1) / 3) + 1}`;
}

export function archive_facts(root, ids, reason, quarter, note, dry_run) {
  const mem = join(root, "memory");
  const cont_path = join(mem, "continuity.md");
  const lines = read_text(cont_path).split("\n");

  const archived_ids = new Set();
  const arch_dir = join(mem, "archive");
  if (existsSync(arch_dir)) {
    for (const name of readdirSync(arch_dir)) {
      if (name.endsWith(".md") && !name.toUpperCase().startsWith("INDEX")) {
        for (const m of read_text(join(arch_dir, name)).matchAll(/<!--\s*id:\s*([a-z0-9-]+)\s*\|/g)) {
          archived_ids.add(m[1]);
        }
      }
    }
  }

  const plans = [];
  for (const fid of ids) {
    if (archived_ids.has(fid)) return { code: 1, msg: `refused: '${fid}' is already in the archive — nothing moved` };
    const fidx = footer_line_index(lines, fid);
    if (fidx === null) return { code: 1, msg: `refused: '${fid}' has no footer in continuity.md — nothing moved` };
    plans.push([fid, block_top(lines, fidx), fidx]);
  }

  const plans_sorted = [...plans].sort((a, b) => b[1] - a[1]); // bottom-up removal

  const now = new Date();
  quarter = quarter || derive_quarter(now.getUTCFullYear(), now.getUTCMonth() + 1);
  const isoDate = now.toISOString().slice(0, 10);
  const arch_path = join(mem, "archive", `${quarter}.md`);
  const index_path = join(mem, "archive", "INDEX.md");

  const moved = [];
  const archive_chunks = [];
  const index_chunks = [];
  const kept = [...lines];
  for (const [fid, top, bot] of plans_sorted) {
    const block = lines.slice(top, bot + 1);
    const n = note && ids.length === 1 ? note : derive_note(block);
    moved.push([fid, n]);
    archive_chunks.push(block.join("\n"));
    index_chunks.push(`- ${fid} — ${n} — ${reason} — ${quarter}.md`);
    let end = bot + 1;
    if (end < kept.length && kept[end].trim() === "") end += 1;
    kept.splice(top, end - top);
  }
  moved.reverse();
  archive_chunks.reverse();
  index_chunks.reverse();

  const heading = `\n## ${isoDate} — archived via archive-fact (${reason})\n\n### Faded facts\n\n`;
  const archive_block = heading + archive_chunks.join("\n\n") + "\n";
  const index_block = index_chunks.join("\n") + "\n";
  const new_cont = kept.join("\n");

  if (new_cont.trim() === "") return { code: 1, msg: "refused: removing those blocks would empty continuity.md — nothing moved" };

  const summary = moved.map(([fid, n]) => `  ${fid} → ${quarter}.md  (${n})`).join("\n");
  if (dry_run) return { code: 0, msg: `DRY-RUN — would move ${moved.length} fact(s):\n${summary}\n(no files changed)` };

  appendFileSync(arch_path, archive_block, "utf-8");
  appendFileSync(index_path, index_block, "utf-8");
  writeFileSync(cont_path, new_cont, "utf-8"); // safe: new_cont already in memory

  return { code: 0, msg: `moved ${moved.length} fact(s) to ${quarter}.md + INDEX:\n${summary}\nNow run memory-lint to confirm.` };
}

function parse_args(argv) {
  const out = { ids: [], reason: "faded", quarter: null, note: null, root: null, dry_run: false };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--id") out.ids.push(argv[++i]);
    else if (a === "--reason") out.reason = argv[++i];
    else if (a === "--quarter") out.quarter = argv[++i];
    else if (a === "--note") out.note = argv[++i];
    else if (a === "--root") out.root = argv[++i];
    else if (a === "--dry-run") out.dry_run = true;
  }
  return out;
}

export function main(argv) {
  const args = parse_args(argv ?? process.argv.slice(2));
  if (args.ids.length === 0) {
    console.error("archive-fact: --id is required");
    return 2;
  }
  const root = find_root(args.root || process.cwd());
  if (!root) {
    console.error("archive-fact: could not find memory/continuity.md");
    return 2;
  }
  const { code, msg } = archive_facts(root, args.ids, args.reason, args.quarter, args.note, args.dry_run);
  const line = "archive-fact: " + msg;
  if (code === 0) console.log(line);
  else console.error(line);
  return code;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  process.exit(main());
}
