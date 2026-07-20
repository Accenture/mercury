// test_archive_fact.mjs — node mirror of test_archive_fact.py.
// Same fixtures, same expectations: the cross-runtime parity contract. Run: node --test <file>
import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { archive_facts } from "./archive-fact.mjs";

const CONT = `# Continuity

## Open Threads

- [x] **Shipped vX — alpha.** Did alpha.
  More alpha detail (a continuation line).
  <!-- id: alpha-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->

- [x] **Shipped vY — beta.** Did beta.
  <!-- id: beta-fact | created: 2026-01-02 | last_used: 2026-01-02 | uses: 1 | tier: working -->

- [ ] **Keep me open.**
  <!-- id: keep-fact | created: 2026-01-03 | last_used: 2026-01-03 | uses: 1 | tier: active -->
`;

function setup(cont = CONT) {
  const root = mkdtempSync(join(tmpdir(), "aftest-"));
  mkdirSync(join(root, "memory", "archive"), { recursive: true });
  writeFileSync(join(root, "memory", "continuity.md"), cont);
  writeFileSync(join(root, "memory", "archive", "INDEX.md"), "# Archive INDEX\n");
  writeFileSync(join(root, "memory", "archive", "2026-Q1.md"), "# 2026-Q1\n");
  return root;
}
const rd = (root, ...parts) => readFileSync(join(root, "memory", ...parts), "utf-8");

test("dry-run changes nothing", () => {
  const root = setup();
  try {
    const before = rd(root, "continuity.md");
    const { code, msg } = archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", null, true);
    assert.equal(code, 0);
    assert.ok(msg.includes("DRY-RUN"));
    assert.equal(rd(root, "continuity.md"), before);
    assert.equal(rd(root, "archive", "2026-Q1.md"), "# 2026-Q1\n");
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("real move removes from continuity and appends to archive + INDEX", () => {
  const root = setup();
  try {
    const { code } = archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", null, false);
    assert.equal(code, 0);
    const cont = rd(root, "continuity.md");
    assert.ok(!cont.includes("alpha-fact"));
    assert.ok(cont.includes("beta-fact"));
    assert.ok(cont.includes("keep-fact"));
    const arch = rd(root, "archive", "2026-Q1.md");
    assert.ok(arch.includes("alpha-fact"));
    assert.ok(arch.includes("More alpha detail")); // multi-line block captured whole
    assert.ok(arch.includes("archived via archive-fact (faded)"));
    assert.ok(rd(root, "archive", "INDEX.md").includes("- alpha-fact —"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("multi-id move", () => {
  const root = setup();
  try {
    const { code } = archive_facts(root, ["alpha-fact", "beta-fact"], "faded", "2026-Q1", null, false);
    assert.equal(code, 0);
    const cont = rd(root, "continuity.md");
    assert.ok(!cont.includes("alpha-fact"));
    assert.ok(!cont.includes("beta-fact"));
    assert.ok(cont.includes("keep-fact"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("missing id refused — atomic, nothing changes", () => {
  const root = setup();
  try {
    const before = rd(root, "continuity.md");
    const { code, msg } = archive_facts(root, ["alpha-fact", "nope-fact"], "faded", "2026-Q1", null, false);
    assert.equal(code, 1);
    assert.ok(msg.includes("no footer"));
    assert.equal(rd(root, "continuity.md"), before);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("already-archived refused", () => {
  const root = setup();
  try {
    archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", null, false);
    const { code, msg } = archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", null, false);
    assert.equal(code, 1);
    assert.ok(msg.includes("already in the archive"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("note override (single id)", () => {
  const root = setup();
  try {
    archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", "custom note", false);
    assert.ok(rd(root, "archive", "INDEX.md").includes("- alpha-fact — custom note — faded"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
