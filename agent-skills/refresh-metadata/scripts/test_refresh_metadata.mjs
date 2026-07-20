// test_refresh_metadata.mjs — node mirror of test_refresh_metadata.py.
// Same fixture, same expectations: the cross-runtime parity contract. Run: node --test <file>
import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { refresh } from "./refresh-metadata.mjs";

const CONT = `# Continuity

## Open Threads
- [x] promoted
  <!-- id: promoted-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->
- [x] aged
  <!-- id: aged-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->
- [x] fresh
  <!-- id: fresh-fact | created: 2026-06-06 | last_used: 2026-06-06 | uses: 1 | tier: working -->
- [ ] open work
  <!-- id: open-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->
- [x] core thing
  <!-- id: core-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 5 | tier: core -->
- [x] retired
  <!-- id: super-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: superseded | superseded-by: promoted-fact -->
- [x] legacy never referenced
  <!-- id: legacy-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->
`;
const POLICY = "## Lifecycle windows\n- working_window: 3\n- active_window: 2\n- archive_window: 4\n";
const SESSIONS = {
  "2026-06-01-000000": ["aged-fact"],
  "2026-06-02-000000": [],
  "2026-06-03-000000": [],
  "2026-06-04-000000": [],
  "2026-06-05-000000": ["promoted-fact"],
  "2026-06-06-000000": ["promoted-fact", "fresh-fact", "open-fact"],
};

function setup() {
  const root = mkdtempSync(join(tmpdir(), "rmtest-"));
  mkdirSync(join(root, "memory", "sessions"), { recursive: true });
  writeFileSync(join(root, "memory", "continuity.md"), CONT);
  writeFileSync(join(root, "memory", "decay-policy.md"), POLICY);
  for (const [stem, ids] of Object.entries(SESSIONS)) {
    const body = "# Session\n\n## Memory References\n" + ids.map((i) => `- Referenced: ${i}\n`).join("");
    writeFileSync(join(root, "memory", "sessions", stem + ".md"), body);
  }
  return root;
}
function fields(root, fid) {
  const text = readFileSync(join(root, "memory", "continuity.md"), "utf-8");
  const esc = fid.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const m = text.match(new RegExp(`<!--\\s*id:\\s*${esc}\\s*\\|([^\\n]*?)-->`));
  const out = {};
  for (const p of m[1].split("|")) { const i = p.indexOf(":"); if (i !== -1) out[p.slice(0, i).trim()] = p.slice(i + 1).trim(); }
  return out;
}

test("dry-run changes nothing", () => {
  const root = setup();
  try {
    const before = readFileSync(join(root, "memory", "continuity.md"), "utf-8");
    const { code, msg } = refresh(root, true);
    assert.equal(code, 0);
    assert.ok(msg.includes("DRY-RUN"));
    assert.equal(readFileSync(join(root, "memory", "continuity.md"), "utf-8"), before);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("re-tier and re-count from references", () => {
  const root = setup();
  try {
    refresh(root, false);
    assert.equal(fields(root, "promoted-fact").tier, "active");
    assert.equal(fields(root, "promoted-fact").uses, "2");
    assert.equal(fields(root, "aged-fact").tier, "archive-candidate");
    assert.equal(fields(root, "fresh-fact").tier, "working");
    assert.equal(fields(root, "open-fact").tier, "working");      // pinned: tier label left as-is
    assert.equal(fields(root, "open-fact").last_used, "2026-06-06"); // …but factual fields still refresh
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("core and superseded untouched", () => {
  const root = setup();
  try {
    refresh(root, false);
    assert.equal(fields(root, "core-fact").tier, "core");
    assert.equal(fields(root, "core-fact").uses, "5");
    assert.equal(fields(root, "super-fact").tier, "superseded");
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("never-referenced fact preserved", () => {
  const root = setup();
  try {
    refresh(root, false);
    assert.equal(fields(root, "legacy-fact").tier, "working");
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("idempotent — second run is a no-op", () => {
  const root = setup();
  try {
    refresh(root, false);
    const { msg } = refresh(root, false);
    assert.ok(msg.includes("nothing to refresh"));
  } finally { rmSync(root, { recursive: true, force: true }); }
});
