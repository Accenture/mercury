// test_memory_lint.mjs — node mirror of test_memory_lint.py.
// Same fixtures, same expectations: this is the cross-runtime contract that
// keeps memory-lint.mjs at parity with memory-lint.py. Run: node --test <file>
import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  pinned_open_threads,
  memref_ids,
  load_repo,
  check_dangling,
  check_session_filenames,
  check_version_manifest,
  check_conflict_markers,
  check_continuity_health,
  sessions_since_review,
  check_stale_metadata,
} from "./memory-lint.mjs";

// (8) advisory cadence/size triggers (v4.24.0). cont is a Map; cont.size is the fact count.
const facts = (n) => new Map(Array.from({ length: n }, (_, i) => [`fact-${i}`, {}]));

function byCodePoint(a, b) {
  if (a < b) return -1;
  if (a > b) return 1;
  return 0;
}
const sortedArr = (s) => [...s].sort(byCodePoint);
const assertPins = (text, expected) =>
  assert.deepEqual(sortedArr(pinned_open_threads(text)), [...expected].sort(byCodePoint));

test("pinned_open_threads flat", () => {
  assertPins(
    `
- [ ] Parent task
  <!-- id: t1 -->
- [x] Done task
  <!-- id: t2 -->
`,
    ["t1"]
  );
});

test("pinned_open_threads nested", () => {
  // Nested list inside an open thread
  assertPins(
    `
- [ ] Parent task
  - Subtask 1
  - Subtask 2
  <!-- id: t3 -->
`,
    ["t3"]
  );
});

test("pinned_open_threads nested open", () => {
  assertPins(
    `
- [ ] Parent task
  - [ ] Nested open
    <!-- id: t4 -->
`,
    ["t4"]
  );
});

test("pinned_open_threads sibling reset", () => {
  assertPins(
    `
- [ ] Parent task
  <!-- id: t5 -->
- Regular bullet that should reset
  <!-- id: t6 -->
`,
    ["t5"]
  );
});

test("pinned_open_threads mixed", () => {
  assertPins(
    `
- [ ] Open task 1
  - Subtask
  <!-- id: mix-1 -->
- [x] Done task
  <!-- id: mix-2 -->
- [ ] Open task 2
  <!-- id: mix-3 -->
- Regular sub-bullet
  <!-- id: mix-4 -->
`,
    ["mix-1", "mix-3"]
  );
});

test("memref_ids ignores prose and review-summary mentions (ot-review-step6-prose)", () => {
  // A fact named only in prose / a '## Memory Review' summary is NOT a use —
  // only '## Memory References' counts.
  const text = `# Session
## What happened
Archiving \`foo-fact\` because it is overdue.
## Memory Review (2026-06-19)
- Archived: 1 (\`foo-fact\` -> archive, faded)
- Tier changes: foo-fact archive-candidate->archived
## Memory References
- Created: bar-fact
- Referenced: baz-fact
`;
  const ids = memref_ids(text);
  assert.ok(ids.has("bar-fact"));
  assert.ok(ids.has("baz-fact"));
  assert.ok(!ids.has("foo-fact")); // prose / review-summary mention is not a reference
});

test("memref_ids is bounded by the next heading", () => {
  const text = `## Memory References
- Referenced: in-block-id
## Next Section
- not-a-ref-id mentioned here
`;
  const ids = memref_ids(text);
  assert.ok(ids.has("in-block-id"));
  assert.ok(!ids.has("not-a-ref-id"));
});

test("check_session_filenames flags date-only names", () => {
  const sessions = ["2026-06-12.md", "2026-06-23.md"];
  const warns = check_session_filenames(sessions);
  assert.equal(warns.length, 2);
  assert.ok(warns.every((w) => w.includes("[date-only-session]")));
});

test("check_session_filenames passes timestamped names", () => {
  const sessions = ["2026-06-23-153401.md", "2026-06-13-011149.md"];
  const warns = check_session_filenames(sessions);
  assert.equal(warns.length, 0);
});

test("check_session_filenames mixed", () => {
  const sessions = ["2026-06-12.md", "2026-06-23-153401.md"];
  const warns = check_session_filenames(sessions);
  assert.equal(warns.length, 1);
  assert.ok(warns[0].includes("2026-06-12.md"));
});

test("supersession target in vision.md is not dangling (cross-file resolution)", () => {
  // Regression for the dangling-link false positive: a supersession target whose
  // footer lives in another memory/*.md (e.g. vision.md) must resolve, not warn.
  // The bug was in load_repo (it only pooled continuity + archive footers), so this
  // exercises load_repo end-to-end against a temp memory/ layer, not check_dangling alone.
  const root = mkdtempSync(join(tmpdir(), "memlint-"));
  try {
    mkdirSync(join(root, "memory", "sessions"), { recursive: true });
    writeFileSync(
      join(root, "memory", "continuity.md"),
      `# Continuity
## Open Threads
- [x] Old vision retired
  <!-- id: old-fact | created: 2026-06-19 | last_used: 2026-06-19 | uses: 1 | tier: superseded | superseded-by: new-fact -->
- [x] Orphaned link
  <!-- id: orphan-fact | created: 2026-06-19 | last_used: 2026-06-19 | uses: 1 | tier: superseded | superseded-by: ghost-fact -->
`
    );
    writeFileSync(
      join(root, "memory", "vision.md"),
      `# Vision
<!-- id: new-fact | created: 2026-06-19 | last_used: 2026-06-19 | uses: 1 | tier: core -->
`
    );

    const { cont, arch, extra } = load_repo(root);
    // the vision fact is available for link resolution but NOT counted as a fact
    assert.ok(extra.has("new-fact"));
    assert.ok(!cont.has("new-fact"));
    assert.ok(!arch.has("new-fact"));

    const warns = check_dangling(new Map([...cont, ...arch, ...extra]));
    // superseded-by a vision.md fact resolves -> no dangling
    assert.ok(!warns.some((w) => w.includes("old-fact")), warns.join("\n"));
    // a genuinely missing target still dangles (negative control)
    assert.ok(
      warns.some((w) => w.includes("orphan-fact") && w.includes("ghost-fact")),
      warns.join("\n")
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

// Regression for the empty-version.md bug: a present-but-empty/malformed
// .agent/version.md breaks Mode B upgrade detection and must be flagged; a
// MISSING file is the valid pre-versioning baseline and must NOT be flagged.
function setupVersion(versionMd) {
  const root = mkdtempSync(join(tmpdir(), "memlint-"));
  mkdirSync(join(root, "memory"), { recursive: true });
  writeFileSync(join(root, "memory", "continuity.md"), "# c\n");
  if (versionMd !== null) {
    mkdirSync(join(root, ".agent"), { recursive: true });
    writeFileSync(join(root, ".agent", "version.md"), versionMd);
  }
  return root;
}

test("check_version_manifest flags empty version.md", () => {
  const root = setupVersion("");
  try {
    const errs = check_version_manifest(root);
    assert.equal(errs.length, 1);
    assert.ok(errs[0].includes("[version-manifest]"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_version_manifest flags malformed version.md", () => {
  const root = setupVersion("# manifest\n(no version line here)\n");
  try {
    assert.equal(check_version_manifest(root).length, 1);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_version_manifest passes a valid version.md", () => {
  const root = setupVersion("- **version:**       4.20.3\n");
  try {
    assert.deepEqual(check_version_manifest(root), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_version_manifest passes when version.md is missing", () => {
  const root = setupVersion(null);
  try {
    assert.deepEqual(check_version_manifest(root), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

// --- conflict markers (check 7) -------------------------------------------
function setupMemFiles(files) {
  const root = mkdtempSync(join(tmpdir(), "memlint-"));
  for (const [rel, body] of Object.entries(files)) {
    const full = join(root, "memory", rel);
    mkdirSync(join(full, ".."), { recursive: true });
    writeFileSync(full, body);
  }
  return root;
}

test("check_conflict_markers flags git markers", () => {
  const root = setupMemFiles({
    "continuity.md": "# c\n<<<<<<< HEAD\nmine\n=======\ntheirs\n>>>>>>> branch\n",
  });
  try {
    const errs = check_conflict_markers(root);
    assert.equal(errs.length, 1);
    assert.ok(errs[0].includes("[conflict-marker]"));
    assert.ok(errs[0].includes("memory/continuity.md:2"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_conflict_markers flags diff3 marker", () => {
  const root = setupMemFiles({ "continuity.md": "# c\n||||||| base\n" });
  try {
    assert.equal(check_conflict_markers(root).length, 1);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_conflict_markers ignores a setext heading underline", () => {
  const root = setupMemFiles({ "continuity.md": "Title\n=======\n\nbody\n" });
  try {
    assert.deepEqual(check_conflict_markers(root), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_conflict_markers passes clean memory", () => {
  const root = setupMemFiles({
    "continuity.md": "# c\nall good\n",
    "sessions/2026-06-27-120000.md": "# Session\nfine\n",
  });
  try {
    assert.deepEqual(check_conflict_markers(root), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_conflict_markers reports one per file", () => {
  const root = setupMemFiles({ "continuity.md": "<<<<<<< a\n>>>>>>> b\n<<<<<<< c\n" });
  try {
    assert.equal(check_conflict_markers(root).length, 1);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_conflict_markers ignores a marker in a session log (sessions/ excluded)", () => {
  // sessions/ legitimately quotes markers (documenting a diff/terminal output).
  const root = setupMemFiles({
    "continuity.md": "# c\nclean\n",
    "sessions/2026-06-27-120000.md": "# Session\n```\n<<<<<<< HEAD\nx\n=======\ny\n>>>>>>> b\n```\n",
  });
  try {
    assert.deepEqual(check_conflict_markers(root), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_conflict_markers ignores a marker in the archive (archive/ excluded)", () => {
  const root = setupMemFiles({
    "continuity.md": "# c\nclean\n",
    "archive/2026-Q2.md": "<<<<<<< HEAD\nx\n>>>>>>> b\n",
  });
  try {
    assert.deepEqual(check_conflict_markers(root), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_continuity_health flags an overdue review", () => {
  const cont_text = "- **last_review:** 2026-06-20 | through 2026-06-20-120000\n";
  const sessions = Array.from({ length: 7 }, (_, i) => `2026-06-2${i + 1}-120000.md`);
  const w = check_continuity_health(facts(1), sessions, cont_text, 10, 5, 30, 600);
  assert.equal(w.length, 1);
  assert.ok(w[0].includes("[review-overdue]"));
  assert.ok(w[0].includes("7 session(s) since last review >= review_every 5"));
});

test("check_continuity_health: a recent review is OK", () => {
  const cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n";
  assert.deepEqual(
    check_continuity_health(facts(1), ["2026-06-27-120000.md"], cont_text, 10, 10, 30, 600),
    []
  );
});

test("check_continuity_health: never reviewed counts all sessions", () => {
  const cont_text = "# Continuity\n(no last_review recorded yet)\n";
  const sessions = Array.from({ length: 4 }, (_, i) => `2026-06-2${i + 1}-120000.md`);
  const w = check_continuity_health(facts(1), sessions, cont_text, 10, 3, 30, 600);
  assert.ok(w.some((x) => x.includes("[review-overdue]") && x.includes("4 session(s)")));
});

test("sessions_since_review prefers the 'through' token", () => {
  const cont_text = "- **last_review:** 2026-06-20 | through 2026-06-25-120000\n";
  const sessions = ["2026-06-24-120000.md", "2026-06-26-120000.md", "2026-06-27-120000.md"];
  assert.equal(sessions_since_review(sessions, cont_text), 2);
});

test("check_continuity_health flags fact bloat", () => {
  const cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n";
  const w = check_continuity_health(facts(31), ["2026-06-27-120000.md"], cont_text, 10, 10, 30, 600);
  assert.equal(w.length, 1);
  assert.ok(w[0].includes("31 decay-eligible facts > continuity_max_facts 30"));
});

test("check_continuity_health: fact bloat excludes core and pinned", () => {
  // tier:core and pinned open threads must NOT count toward the cap — they can never be
  // archived, so counting them produces permanent noise (field report: mercury-composable).
  const cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n";
  const entries = [];
  for (let i = 0; i < 14; i++) entries.push([`core-${i}`, { tier: "core" }]);     // 14 core — never decay
  for (let i = 0; i < 11; i++) entries.push([`pinned-${i}`, { tier: "working" }]); // 11 pinned open threads
  for (let i = 0; i < 16; i++) entries.push([`working-${i}`, { tier: "working" }]); // 16 decay-eligible
  const cont = new Map(entries);
  const pinned = new Set(Array.from({ length: 11 }, (_, i) => `pinned-${i}`));
  // 41 total facts, only 16 decay-eligible — should be well under the cap of 30
  const w = check_continuity_health(cont, ["2026-06-27-120000.md"], cont_text, 10, 10, 30, 600, pinned);
  assert.deepEqual(w, [], "core + pinned facts must not trigger continuity-bloat");
});

test("check_continuity_health flags line bloat (archivable > 0 → review can lean it down)", () => {
  const cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n";
  const w = check_continuity_health(facts(1), ["2026-06-27-120000.md"], cont_text, 700, 10, 30, 600, new Set(), 3);
  assert.equal(w.length, 1);
  assert.ok(w[0].includes("continuity.md 700 lines > continuity_max_lines 600"));
  assert.ok(w[0].includes("a review is due to lean it down"));
});

test("check_continuity_health: line bloat with nothing archivable → active-verbosity message", () => {
  // archivable === 0 → a review has no honest lever (nothing faded/superseded). The message must
  // NOT claim a review will fix it (that nudges toward premature archival of active facts —
  // REVIEW.md's costliest error); it names the real lever instead (v4.28.3, mercury-composable).
  const cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n";
  const w = check_continuity_health(facts(1), ["2026-06-27-120000.md"], cont_text, 700, 10, 30, 600, new Set(), 0);
  assert.equal(w.length, 1);
  assert.ok(w[0].includes("continuity.md 700 lines > continuity_max_lines 600"));
  assert.ok(w[0].includes("nothing is archivable yet"));
  assert.ok(w[0].includes("Condense shipped decisions"));
  assert.ok(!w[0].includes("a review is due to lean it down"));
});

test("check_continuity_health: a healthy layer is OK", () => {
  const cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n";
  assert.deepEqual(
    check_continuity_health(facts(24), ["2026-06-27-120000.md"], cont_text, 490, 10, 30, 600),
    []
  );
});

const STALE_STEMS = ["2026-06-01-000000", "2026-06-02-000000", "2026-06-03-000000"];

test("check_stale_metadata flags tier drift", () => {
  const cont = new Map([["foo-fact", { tier: "working", created: "2026-01-01" }]]);
  const refs = [new Set(["foo-fact"]), new Set(["foo-fact"]), new Set()];
  const w = check_stale_metadata(cont, new Set(), refs, STALE_STEMS, 3, 2, 4);
  assert.equal(w.length, 1);
  assert.ok(w[0].includes("[stale-metadata]"));
  assert.ok(w[0].includes("should be 'active'"));
});

test("check_stale_metadata: matching tier not flagged", () => {
  const cont = new Map([["a-fact", { tier: "active", created: "2026-01-01" }]]);
  const refs = [new Set(["a-fact"]), new Set(["a-fact"]), new Set()];
  assert.deepEqual(check_stale_metadata(cont, new Set(), refs, STALE_STEMS, 3, 2, 4), []);
});

test("check_stale_metadata: core and superseded exempt", () => {
  const cont = new Map([
    ["c-fact", { tier: "core", created: "2026-01-01" }],
    ["s-fact", { tier: "superseded", created: "2026-01-01", "superseded-by": "a-fact" }],
  ]);
  const refs = [new Set(["c-fact"]), new Set(["s-fact"]), new Set()];
  assert.deepEqual(check_stale_metadata(cont, new Set(), refs, STALE_STEMS, 3, 2, 4), []);
});

test("check_stale_metadata: never-referenced not flagged", () => {
  const cont = new Map([["legacy-fact", { tier: "working", created: "2026-01-01" }]]);
  const refs = [new Set(), new Set(), new Set()];
  assert.deepEqual(check_stale_metadata(cont, new Set(), refs, STALE_STEMS, 3, 2, 4), []);
});

test("check_stale_metadata: a pinned thread's tier is not flagged (v4.26.1)", () => {
  // pinned `- [ ]` thread never decays; the tool leaves its tier label alone.
  const cont = new Map([["open-fact", { tier: "working", created: "2026-01-01" }]]);
  const refs = [new Set(["open-fact"]), new Set(["open-fact"]), new Set()];
  assert.deepEqual(check_stale_metadata(cont, new Set(["open-fact"]), refs, STALE_STEMS, 3, 2, 4), []);
});
