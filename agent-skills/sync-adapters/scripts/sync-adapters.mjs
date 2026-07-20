#!/usr/bin/env node
// sync-adapters.mjs — regenerate per-vendor skill adapters from the neutral agent-skills/ layer.
// Part of agent-memory (provenance: agent-memory-builtin). Node >= 18, built-in modules only.
//
// Idempotent. Writes ONLY the gitignored vendor adapter dirs (.claude/skills, .gemini/commands,
// .cursor/rules, .kiro/skills, .github/skills, .agents/skills); never edits agent-skills/ or any
// committed file. (.agents/skills is the Agent Skills standard dir read by Google Antigravity 'agy',
// the Gemini CLI successor — it ignores .gemini/commands.)
// Prunes only adapters THIS tool generated (identified by their pointer signature), so a
// hand-authored vendor skill is never deleted. Run from the repo root. `--dry-run` previews.
//
// Kept byte-for-byte equivalent to sync-adapters.py (shared behavior is the cross-runtime contract).

import {
  readFileSync, readdirSync, existsSync, mkdirSync, writeFileSync, rmSync,
} from "node:fs";
import { join } from "node:path";

const ROOT = process.cwd();
const SRC = join(ROOT, "agent-skills");
const DRY = process.argv.includes("--dry-run");

const SIG = "Maintained vendor-neutrally."; // pointer signature for claude/kiro/copilot adapters
const GEM_SIG = "Read and follow the skill at agent-skills/";
const CUR_SIG = "read and follow `agent-skills/";

function frontmatter(file) {
  const text = readFileSync(file, "utf8");
  const m = text.match(/^---\n([\s\S]*?)\n---/);
  const fm = m ? m[1] : "";
  const field = (k) => {
    const mm = fm.match(new RegExp("^" + k + ":[ \\t]*(.*)$", "m"));
    return mm ? mm[1].trim() : "";
  };
  return { name: field("name"), description: field("description") };
}

function pointer(name) {
  return "Maintained vendor-neutrally. Read and follow `agent-skills/" + name +
    "/SKILL.md` (repo root)\nand any scripts it references.\n";
}
const claudeLike = (n, d) => `---\nname: ${n}\ndescription: ${d}\n---\n` + pointer(n);
const geminiToml = (n, d) =>
  `description = "${d}"\nprompt = "Read and follow the skill at agent-skills/${n}/SKILL.md ` +
  `(repo root), including any scripts it references, then carry out: {{args}}"\n`;
const cursorMdc = (n, d) =>
  `---\ndescription: ${d}\nglobs:\nalwaysApply: false\n---\n` +
  "When this applies, read and follow `agent-skills/" + n + "/SKILL.md` (repo root) and any\n" +
  "scripts it references.\n";

// --- discover neutral skills ---
const skills = [];
if (existsSync(SRC)) {
  for (const entry of readdirSync(SRC).sort()) {
    const md = join(SRC, entry, "SKILL.md");
    if (existsSync(md)) {
      const { name, description } = frontmatter(md);
      skills.push({ dir: entry, name: name || entry, description });
    }
  }
}

const W = (p, c) => { if (DRY) { console.log("would write " + p); } else { writeFileSync(p, c); } };
const M = (p) => { if (!DRY) mkdirSync(p, { recursive: true }); };

let adapters = 0;
for (const s of skills) {
  const n = s.name, d = s.description;
  M(join(ROOT, ".claude/skills/" + n));   W(join(ROOT, ".claude/skills/" + n + "/SKILL.md"), claudeLike(n, d));
  M(join(ROOT, ".gemini/commands"));       W(join(ROOT, ".gemini/commands/" + n + ".toml"), geminiToml(n, d));
  M(join(ROOT, ".cursor/rules"));          W(join(ROOT, ".cursor/rules/" + n + ".mdc"), cursorMdc(n, d));
  M(join(ROOT, ".kiro/skills/" + n));      W(join(ROOT, ".kiro/skills/" + n + "/SKILL.md"), claudeLike(n, d));
  M(join(ROOT, ".github/skills/" + n));    W(join(ROOT, ".github/skills/" + n + "/SKILL.md"), claudeLike(n, d));
  M(join(ROOT, ".agents/skills/" + n));    W(join(ROOT, ".agents/skills/" + n + "/SKILL.md"), claudeLike(n, d));
  adapters += 6;
}

// --- prune orphaned adapters we generated (signature-guarded) ---
const live = new Set();
for (const s of skills) { live.add(s.name); live.add(s.dir); }
let pruned = 0;

function pruneDirs(base, signature) {
  const p = join(ROOT, base);
  if (!existsSync(p)) return;
  for (const e of readdirSync(p)) {
    if (live.has(e)) continue;
    const md = join(p, e, "SKILL.md");
    if (existsSync(md) && readFileSync(md, "utf8").includes(signature)) {
      if (!DRY) rmSync(join(p, e), { recursive: true, force: true });
      pruned++;
    }
  }
}
function pruneFiles(base, ext, signature) {
  const p = join(ROOT, base);
  if (!existsSync(p)) return;
  for (const e of readdirSync(p)) {
    if (!e.endsWith(ext)) continue;
    const n = e.slice(0, -ext.length);
    if (live.has(n)) continue;
    if (readFileSync(join(p, e), "utf8").includes(signature)) {
      if (!DRY) rmSync(join(p, e), { force: true });
      pruned++;
    }
  }
}
pruneDirs(".claude/skills", SIG);
pruneDirs(".kiro/skills", SIG);
pruneDirs(".github/skills", SIG);
pruneDirs(".agents/skills", SIG);
pruneFiles(".gemini/commands", ".toml", GEM_SIG);
pruneFiles(".cursor/rules", ".mdc", CUR_SIG);

console.log(
  `sync-adapters: synced ${skills.length} skill(s) → ${adapters} adapters ` +
  `(gitignored — do not commit; only agent-skills/ is shared); pruned ${pruned} orphan(s)`
);
