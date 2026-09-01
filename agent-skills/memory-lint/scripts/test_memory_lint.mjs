// test_memory_lint.mjs — node mirror of test_memory_lint.py.
// Same fixtures, same expectations: this is the cross-runtime contract that
// keeps memory-lint.mjs at parity with memory-lint.py. Run: node --test <file>
import { afterEach, test } from "node:test";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync, readdirSync, readFileSync } from "node:fs";
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
  check_secret_material,
  closed_narrative_lines,
  check_closed_thread_bloat,
  load_windows,
  check_thread_files,
  check_duplicate_ids,
  check_duplicate_state_keys,
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

test("closed-thread bloat counts only closed blocks", () => {
  // (11) counting rule: non-empty lines inside `- [x]` blocks (checkbox through
  // footer), a block ending at the next open thread or heading. Open threads and
  // headings never count (the bloat class is completed ship narratives —
  // mercury-composable field report, 64% of continuity).
  const cont_text =
    "## Open Threads\n\n" +
    "- [x] **Shipped X.** line two of the record\n" +
    "  more narrative\n" +
    "  <!-- id: shipped-x | created: 2026-01-01 | last_used: 2026-01-01 " +
    "| uses: 1 | tier: active -->\n" +
    "\n" +
    "- [ ] **Open thing.** must not count\n" +
    "  narrative of the open thread\n";
  assert.equal(closed_narrative_lines(cont_text), 3);
  assert.deepEqual(check_closed_thread_bloat(cont_text, 150), []);
  const w = check_closed_thread_bloat(cont_text, 2);
  assert.equal(w.length, 1);
  assert.ok(w[0].includes("[closed-thread-bloat] 3 line(s)"));
  assert.ok(w[0].includes("condense them to 3-6-line stubs"));
  assert.ok(w[0].includes("origin session log"));
});

test("closed_narrative_max_lines: default and policy parse", () => {
  const root = mkdtempSync(join(tmpdir(), "lint-knob-"));
  try {
    assert.equal(load_windows(root).closed_narrative_max_lines, 150);
    mkdirSync(join(root, "memory"), { recursive: true });
    writeFileSync(join(root, "memory", "decay-policy.md"),
      "- closed_narrative_max_lines: 99\n");
    assert.equal(load_windows(root).closed_narrative_max_lines, 99);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
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

// (10) [secret-material]: committed memory surfaces must not carry credentials/PII
// (field incident: a rendered kafka JAAS secret pasted into a session log, caught by a
// client-side DLP scanner). Advisory; must NEVER echo the matched value into the report.
function secretSetup(files) {
  const root = mkdtempSync(join(tmpdir(), "memlint-"));
  const all = { "continuity.md": "# c\nclean\n", ...files };
  for (const [rel, body] of Object.entries(all)) {
    const full = join(root, "memory", rel);
    mkdirSync(join(full, ".."), { recursive: true });
    writeFileSync(full, body);
  }
  return root;
}

const fixtureEnvValues = new Map();

function fixtureEnv(name, value) {
  if (!fixtureEnvValues.has(name)) fixtureEnvValues.set(name, process.env[name]);
  process.env[name] = value;
  return process.env[name];
}

function fixtureSecret(name, { length = 24, prefix = "", uppercase = false } = {}) {
  const digest = createHash("sha256").update(name).digest("hex");
  let material = [...digest].map((ch, i) => i % 2 ? ch.toUpperCase() : ch).join("");
  if (uppercase) material = material.toUpperCase();
  return fixtureEnv(name, prefix + material.slice(0, length));
}

afterEach(() => {
  for (const [name, previous] of fixtureEnvValues) {
    if (previous === undefined) delete process.env[name];
    else process.env[name] = previous;
  }
  fixtureEnvValues.clear();
});

test("check_secret_material flags a credential assignment and never echoes the value", () => {
  const secret = fixtureSecret("AGENT_MEMORY_TEST_ASSIGNMENT");
  const root = secretSetup({
    "sessions/2026-08-06-120000.md": `# Session\n\`\`\`\nbearer.auth.client.secret=${secret}\n\`\`\`\n`,
  });
  try {
    const w = check_secret_material(root);
    assert.equal(w.length, 1);
    assert.ok(w[0].includes("[secret-material]"));
    assert.ok(w[0].includes("memory/sessions/2026-08-06-120000.md:3"));
    assert.ok(w[0].includes("credential-assignment"));
    assert.ok(w[0].includes("key 'bearer.auth.client.secret'"));
    assert.ok(!w[0].includes(secret)); // the report must never amplify the secret
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material: placeholders are safe but nonempty defaults flag", () => {
  const placeholder = fixtureEnv("AGENT_MEMORY_TEST_PLACEHOLDER", "change" + "me-please");
  const fallback = fixtureSecret("AGENT_MEMORY_TEST_TEMPLATE_FALLBACK");
  const root = secretSetup({
    "sessions/2026-08-06-120000.md": [
      "# Session",
      "clientSecret='${KAFKA_CLIENT_SECRET}'",
      "password: (REDACTED)",
      "api_key: <your-key-here>",
      "client_secret: {{VAULT_REF}}",
      "client_secret: placeholder-value",
      "access_token: 2026-08-06-153509",
      "max_tokens_password: 128000000",
      `password=${placeholder}`,
      // env-var references with default-value / dotted forms are placeholders too
      // (field FP, mercury-composable 2026-08-13 — line quoted VERBATIM below):
      "  (`redis.host`/`redis.port`/`redis.password=${REDIS_PASSWORD:}`/`redis.ssl`/`redis.database`/`redis.timeout.ms`)",
      "client_secret: ${vault.paths.kafka}",
      `password=\${REDIS_URL:-${fallback}}`,
    ].join("\n") + "\n",
  });
  try {
    const w = check_secret_material(root);
    assert.equal(w.length, 1);
    assert.ok(w[0].includes("credential-assignment"));
    assert.ok(w[0].includes("(1 hit(s)"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material flags known token shapes", () => {
  const githubToken = fixtureSecret(
    "AGENT_MEMORY_TEST_GITHUB_TOKEN", { length: 40, prefix: "ghp_" }
  );
  const awsKey = fixtureSecret(
    "AGENT_MEMORY_TEST_AWS_KEY", { length: 16, prefix: "AKIA", uppercase: true }
  );
  const privateKeyHeader = fixtureEnv(
    "AGENT_MEMORY_TEST_PRIVATE_KEY_HEADER",
    "-".repeat(5) + "BEGIN " + "RSA " + "PRIVATE " + "KEY" + "-".repeat(5)
  );
  const root = secretSetup({
    "sessions/2026-08-06-120000.md": [
      "# Session",
      `pushed with ${githubToken}`,
      `aws key ${awsKey}`,
      privateKeyHeader,
    ].join("\n") + "\n",
  });
  try {
    const cats = check_secret_material(root).join("\n");
    assert.ok(cats.includes("github-token"));
    assert.ok(cats.includes("aws-access-key-id"));
    assert.ok(cats.includes("private-key-block"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material flags email PII, excludes public forms", () => {
  const privateEmail = fixtureEnv(
    "AGENT_MEMORY_TEST_PRIVATE_EMAIL",
    "fixture.person" + "@" + "some-client-corp" + "." + "com"
  );
  const root = secretSetup({
    "sessions/2026-08-06-120000.md": [
      "# Session",
      `contact ${privateEmail} about rotation`,
      "Co-Authored-By: Claude Code <noreply@anthropic.com>",
      "tagger 12345+acn-user@users.noreply.github.com",
      "remote git@github.com:acn-ericlaw/agent-memory.git",
      "docs use alice@example.com",
    ].join("\n") + "\n",
  });
  try {
    const w = check_secret_material(root);
    assert.equal(w.length, 1);
    assert.ok(w[0].includes("email"));
    assert.ok(w[0].includes("(1 hit(s)"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material: ssn + Luhn-valid card flagged; dates and Luhn-fails are not", () => {
  const ssn = fixtureEnv("AGENT_MEMORY_TEST_SSN", ["123", "45", "6789"].join("-"));
  const card = fixtureEnv(
    "AGENT_MEMORY_TEST_CARD", ["4539", "1488", "0343", "6467"].join(" ")
  );
  const invalidCard = fixtureEnv(
    "AGENT_MEMORY_TEST_INVALID_CARD", ["1234", "5678", "9012", "3456"].join(" ")
  );
  const root = secretSetup({
    "sessions/2026-08-06-120000.md": [
      "# Session",
      `ssn ${ssn} leaked`,
      `card ${card} on file`,
      "dated 2026-08-06, stem 2026-08-06-153509, v4.33.0", // none of these
      `not a card: ${invalidCard}`,
    ].join("\n") + "\n",
  });
  try {
    const all = check_secret_material(root);
    const cats = all.join("\n");
    assert.ok(cats.includes("ssn"));
    assert.ok(cats.includes("payment-card"));
    assert.ok(all.find((x) => x.includes("payment-card")).includes("(1 hit(s)"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material flags absolute home paths, excludes CI users", () => {
  const privateHome = fixtureEnv(
    "AGENT_MEMORY_TEST_HOME_PATH",
    "/" + "Users" + "/" + "fixture-user" + "/projects/foo"
  );
  const root = secretSetup({
    "continuity.md": `# c\n- repo: ${privateHome}\n`,
    "sessions/2026-08-06-120000.md": "# Session\nCI ran in /home/runner/work and ~/sandbox/foo\n",
  });
  try {
    const w = check_secret_material(root);
    assert.equal(w.length, 1);
    assert.ok(w[0].includes("home-path"));
    assert.ok(w[0].includes("memory/continuity.md:2"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material scans archive/ and aggregates counts per file+category", () => {
  const password = fixtureSecret("AGENT_MEMORY_TEST_ARCHIVE_PASSWORD");
  const apiKey = fixtureSecret("AGENT_MEMORY_TEST_ARCHIVE_API_KEY");
  const clientSecret = fixtureSecret("AGENT_MEMORY_TEST_ARCHIVE_CLIENT_SECRET");
  const root = secretSetup({
    "archive/2026-Q2.md":
      `# a\npassword=${password}\napi_key=${apiKey}\nclient_secret=${clientSecret}\n`,
  });
  try {
    const w = check_secret_material(root);
    assert.equal(w.length, 1); // one report per file per category
    assert.ok(w[0].includes("credential-assignment"));
    assert.ok(w[0].includes("(3 hit(s), first at line 2)"));
    assert.ok(w[0].includes("memory/archive/2026-Q2.md:2"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material: waiver line and placeholder home paths are not flagged", () => {
  // A log DOCUMENTING a leak cleanup legitimately quotes the patterns — the explicit
  // line waiver keeps the advisory signal, not noise; `/Users/...` is a placeholder.
  const waivedSecret = fixtureSecret("AGENT_MEMORY_TEST_WAIVED_SECRET");
  const root = secretSetup({
    "sessions/2026-08-06-120000.md": [
      "# Session",
      `the leaked line was password=${waivedSecret} <!-- lint:allow-secret-material -->`,
      "docs quote `/Users/...` as the placeholder form",
    ].join("\n") + "\n",
  });
  try {
    assert.deepEqual(check_secret_material(root), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material: quoted assignments, Authorization, and embedded placeholders flag", () => {
  const quotedSecret = fixtureSecret("AGENT_MEMORY_TEST_QUOTED_SECRET");
  const authorizationSecret = fixtureSecret("AGENT_MEMORY_TEST_AUTHORIZATION");
  const embeddedSecret = fixtureSecret("AGENT_MEMORY_TEST_EMBEDDED_PLACEHOLDER");
  const fallbackSecret = fixtureSecret("AGENT_MEMORY_TEST_NONEMPTY_FALLBACK");
  const root = secretSetup({
    "sessions/2026-08-13-120000.md": [
      "# Session",
      `{"client_secret": "${quotedSecret}"}`,
      `Authorization: Bearer ${authorizationSecret}`,
      `client_secret=dummy${embeddedSecret}`,
      `client_secret=$${embeddedSecret}`,
      `client_secret=\${CLIENT_SECRET:-${fallbackSecret}}`,
    ].join("\n") + "\n",
  });
  try {
    const w = check_secret_material(root);
    assert.equal(w.length, 2);
    const joined = w.join("\n");
    assert.ok(joined.includes("credential-assignment"));
    assert.ok(joined.includes("(4 hit(s)"));
    assert.ok(joined.includes("authorization-header"));
    assert.ok(!joined.includes(quotedSecret));
    assert.ok(!joined.includes(authorizationSecret));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material: all-caps enum constants are key-scoped", () => {
  // First field FP (mercury-composable, 2026-08-13): config docs quoted in a session log —
  // a credential-keyed property set to an ALL-CAPS enum constant is a source TYPE, not a
  // credential — including the markdown inline-code form (`key=VALUE`), where the closing
  // backtick must not ride into the value (v4.33.2, the form the real field line used).
  // Mixed-case values on the same key class must still flag, backticked or bare.
  const mixedSecret = fixtureSecret("AGENT_MEMORY_TEST_MIXED_SECRET");
  const backtickedSecret = fixtureSecret("AGENT_MEMORY_TEST_BACKTICKED_SECRET");
  const uppercaseSecret = fixtureSecret(
    "AGENT_MEMORY_TEST_UPPERCASE_SECRET", { length: 24, uppercase: true }
  );
  const root = secretSetup({
    "sessions/2026-08-13-120000.md": [
      "# Session",
      "bearer.auth.credentials.source: OAUTHBEARER",
      "sasl.password.mode=STATIC_TOKEN",
      "markdown form: `bearer.auth.credentials.source=OAUTHBEARER` + `bearer.auth.issuer.endpoint.url` /",
      `still real: client_secret=${mixedSecret}`,
      `backticked real: \`api_key=${backtickedSecret}\``,
      `uppercase real: client_secret=${uppercaseSecret}`,
    ].join("\n") + "\n",
  });
  try {
    const w = check_secret_material(root);
    assert.equal(w.length, 1);
    assert.ok(w[0].includes("key 'client_secret'"));
    assert.ok(w[0].includes("(3 hit(s)"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("check_secret_material: the tool's own opt-down knob settings are not credentials", () => {
  // Field FP (mercury-composable, 2026-08-19): the pre-commit guard's own blocking message
  // prints "AGENT_MEMORY_SECRET_GUARD=advisory", so a session log documenting that guidance
  // self-flagged (the key contains SECRET; "advisory" meets the value floor). The knob's
  // documented settings are exempt — but ONLY those values: an arbitrary value under the
  // same key must still flag (no smuggling envelope). Fixture lines quote the guard's
  // guidance line and the field repro line VERBATIM (v4.33.2 lesson) — the guidance line's
  // closing paren rides into the captured value, which the exemption must tolerate.
  const opaque = fixtureSecret("AGENT_MEMORY_TEST_KNOB_OPAQUE");
  const cleanRoot = secretSetup({
    "sessions/2026-08-19-120000.md": [
      "# Session",
      "  git commit --no-verify    (or opt down: AGENT_MEMORY_SECRET_GUARD=advisory)",
      "Opt down with AGENT_MEMORY_SECRET_GUARD=advisory if needed.",
      "The default is AGENT_MEMORY_SECRET_GUARD=enforcing.",
      "inline form: `AGENT_MEMORY_SECRET_GUARD=advisory`",
      "git-config spelling: `agent-memory.secretguard=advisory`",
    ].join("\n") + "\n",
  });
  try {
    assert.deepEqual(check_secret_material(cleanRoot), []);
  } finally {
    rmSync(cleanRoot, { recursive: true, force: true });
  }
  const flaggedRoot = secretSetup({
    "sessions/2026-08-19-130000.md": `# Session\nAGENT_MEMORY_SECRET_GUARD=${opaque}\n`,
  });
  try {
    const w = check_secret_material(flaggedRoot);
    assert.equal(w.length, 1);
    assert.ok(w[0].includes("key 'AGENT_MEMORY_SECRET_GUARD'"));
    assert.ok(!w[0].includes(opaque));
  } finally {
    rmSync(flaggedRoot, { recursive: true, force: true });
  }
});

// (v4.34.0) `--scan-files`: credential-class scan of arbitrary config files — the
// pre-commit hook / CI-wrapper surface behind the field incident (a Postman JSON and an
// OpenShift YAML with live credentials, committed outside memory/).
import { scan_secret_files } from "./memory-lint.mjs";

test("scan_secret_files is credential-class only", () => {
  const secret = fixtureSecret("AGENT_MEMORY_TEST_SCANFILES");
  const root = mkdtempSync(join(tmpdir(), "memlint-"));
  try {
    mkdirSync(join(root, "src"), { recursive: true });
    const props = join(root, "src", "app.properties");
    writeFileSync(props, `spring.datasource.password=${secret}\n`);
    const pj = join(root, "package.json");
    writeFileSync(pj, '{"author": "Dev One <dev.one@some-client-corp.com>"}\n');
    const w = scan_secret_files([props, pj]);
    assert.equal(w.length, 1); // the email is a memory-layer check, not a config one
    assert.ok(w[0].includes("credential-assignment"));
    assert.ok(!w[0].includes(secret)); // never echo the value
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("scan_secret_files: config placeholder forms are not flagged", () => {
  // The four FP classes from the 661-file field probe (2026-08-14) must stay quiet:
  // single-brace JAAS template, template-with-placeholder-default, test-affixed fixture,
  // dotted route reference in an authorization value — plus a GH-Actions expression.
  const root = mkdtempSync(join(tmpdir(), "memlint-"));
  try {
    const cfg = join(root, "conf.yaml");
    writeFileSync(cfg, [
      "#sasl.jaas.config=…PlainLoginModule required username={CHANGE_THIS} password={CHANGE_THIS};",
      "authorization: '${DEMO_PEER_TOKEN:demo}'",
      "bearer.auth.client.secret=test-secret",
      '- "authorization: v1.basic.auth"',
      "api_key: ${{secrets.SR_KEY}}",
    ].join("\n") + "\n");
    assert.deepEqual(scan_secret_files([cfg]), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("scan_secret_files: Postman split key/value form", () => {
  // Postman's `"key": "client_secret", "value": "…"` convention — the literal incident
  // artifact class; a `{{variable}}` reference stays a placeholder.
  const secret = fixtureSecret("AGENT_MEMORY_TEST_POSTMAN");
  const root = mkdtempSync(join(tmpdir(), "memlint-"));
  try {
    const col = join(root, "collection.json");
    writeFileSync(col,
      '{"key": "client_secret", "value": "' + secret + '"},\n' +
      '{"key": "client_secret", "value": "{{client_secret}}"}\n');
    const w = scan_secret_files([col]);
    assert.equal(w.length, 1);
    assert.ok(w[0].includes("key 'client_secret'"));
    assert.ok(w[0].includes("(1 hit(s)"));
    assert.ok(!w[0].includes(secret));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("shipped scripts: prose identifiers stay scanner-neutral", () => {
  // Field regression (Snyk, enterprise deployment, 2026-08-25): hardcoded-secret
  // detectors key on identifier-contains-trigger-word + string-literal assignment,
  // and the prose guidance constant's old name rejected downstream builds. Prose
  // constants in the shipped scripts must stay scanner-neutral. Trigger words are
  // assembled at runtime so this test never commits the flagged shape itself.
  const words = ["SEC" + "RET", "TOK" + "EN", "PASS" + "WORD", "PASS" + "WD",
                 "CREDEN" + "TIAL", "API" + "KEY", "API_" + "KEY"];
  const assignRx = /^[ \t]*(?:const |let |var )?([A-Za-z_][A-Za-z0-9_]*)\s*=\s*\(?\s*["'`]/gm;
  const here = new URL(".", import.meta.url);
  const offenders = [];
  for (const name of readdirSync(here).sort()) {
    if (!name.endsWith(".py") && !name.endsWith(".mjs")) continue;
    for (const m of readFileSync(new URL(name, here), "utf-8").matchAll(assignRx)) {
      const ident = m[1].toUpperCase();
      if (words.some((w) => ident.includes(w))) offenders.push(`${name}: ${m[1]}`);
    }
  }
  assert.deepEqual(offenders, []);
});

// (12) the thread-file contract (v4.39.0): one Open Thread per file, named after its
// footer id. Filename = identity is what makes concurrent thread work merge-free.
const VALID_THREAD = `- [ ] **Ship it.** The plan.
  <!-- id: ship-it | created: 2026-09-01 | last_used: 2026-09-01 | uses: 1 | tier: working -->
`;

test("thread-file: valid file passes", () => {
  assert.deepEqual(check_thread_files([["thread-ship-it.md", VALID_THREAD]]), []);
});

test("thread-file: misnamed file flagged", () => {
  const out = check_thread_files([["thread-wrong-name.md", VALID_THREAD]]);
  assert.equal(out.length, 1);
  assert.ok(out[0].includes("[thread-file]"));
  assert.ok(out[0].includes("should be named thread-ship-it.md"));
});

test("thread-file: missing footer flagged", () => {
  const out = check_thread_files([["thread-x.md", "- [ ] no footer here\n"]]);
  assert.equal(out.length, 1);
  assert.ok(out[0].includes("no fact footer"));
});

test("thread-file: two footers flagged", () => {
  const two =
    VALID_THREAD +
    "- [ ] second\n  <!-- id: other | created: 2026-09-01 | last_used: 2026-09-01 | uses: 1 | tier: working -->\n";
  const out = check_thread_files([["thread-ship-it.md", two]]);
  assert.equal(out.length, 1);
  assert.ok(out[0].includes("holds 2 footers"));
});

test("thread-file: non-bullet start flagged", () => {
  const out = check_thread_files([["thread-ship-it.md", "# A heading instead of the block\n" + VALID_THREAD]]);
  assert.equal(out.length, 1);
  assert.ok(out[0].includes("does not start with"));
});

// (13) an id exists exactly once across the live layer — the backstop for a same-id
// creation collision on parallel branches (the silent-fork shape).
test("duplicate-id: unique ids pass", () => {
  const cont = "- fact\n  <!-- id: a-fact | tier: active -->\n";
  const threads = [["thread-b-thread.md", "- [ ] t\n  <!-- id: b-thread | tier: working -->\n"]];
  assert.deepEqual(check_duplicate_ids(cont, threads), []);
});

test("duplicate-id: continuity + thread file flagged", () => {
  const cont = "- fact\n  <!-- id: same-id | tier: active -->\n";
  const threads = [["thread-same-id.md", "- [ ] t\n  <!-- id: same-id | tier: working -->\n"]];
  const out = check_duplicate_ids(cont, threads);
  assert.equal(out.length, 1);
  assert.ok(out[0].includes("[duplicate-id] same-id has 2 footers"));
  assert.ok(out[0].includes("memory/open-threads/thread-same-id.md"));
});

test("duplicate-id: two thread files flagged", () => {
  const threads = [
    ["thread-same-id.md", "- [ ] t\n  <!-- id: same-id | tier: working -->\n"],
    ["thread-other.md", "- [ ] t2\n  <!-- id: same-id | tier: working -->\n"],
  ];
  const out = check_duplicate_ids("# Continuity\n", threads);
  assert.equal(out.length, 1);
  assert.ok(out[0].includes("same-id"));
});

// (14) Project State fields are scalars — absorbed from PR #27 (Roland Heusser):
// the backstop for a union-style hand merge that kept both sides of a bumped scalar.
function stateRoot(cont_text) {
  const root = mkdtempSync(join(tmpdir(), "memlint-state-"));
  mkdirSync(join(root, "memory"), { recursive: true });
  writeFileSync(join(root, "memory", "continuity.md"), cont_text);
  return root;
}

test("duplicate-state-key: duplicate scalar flagged with both lines", () => {
  const root = stateRoot(
    "# C\n\n## Project State\n\n- **project:** x\n- **last_review:** 2026-08-01\n" +
      "- **last_review:** 2026-08-20\n\n## Key Decisions\n"
  );
  try {
    const out = check_duplicate_state_keys(root);
    assert.equal(out.length, 1);
    assert.ok(out[0].includes("[duplicate-state-key]"));
    assert.ok(out[0].includes("'last_review' is set twice"));
    assert.ok(out[0].includes("also line 6"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("duplicate-state-key: repeated key outside Project State ok", () => {
  const root = stateRoot(
    "# C\n\n## Project State\n\n- **project:** x\n\n## Key Decisions\n\n" +
      "- **project:** mention one\n- **project:** mention two\n"
  );
  try {
    assert.deepEqual(check_duplicate_state_keys(root), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("duplicate-state-key: unique scalars ok", () => {
  const root = stateRoot("# C\n\n## Project State\n\n- **project:** x\n- **status:** y\n");
  try {
    assert.deepEqual(check_duplicate_state_keys(root), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

// Thread files are live continuity-domain facts: load_repo merges their footers and
// checkbox pinning; conflict markers and closed-thread bloat see them too.
function threadLayer() {
  const root = mkdtempSync(join(tmpdir(), "memlint-threads-"));
  mkdirSync(join(root, "memory", "sessions"), { recursive: true });
  mkdirSync(join(root, "memory", "open-threads"), { recursive: true });
  writeFileSync(join(root, "memory", "continuity.md"), "# Continuity\n\n## Project State\n\n- **project:** t\n");
  return root;
}

test("thread layer: load_repo merges thread facts and pins", () => {
  const root = threadLayer();
  try {
    writeFileSync(
      join(root, "memory", "open-threads", "thread-live-gap.md"),
      "- [ ] **Gap.** open work\n  <!-- id: live-gap | created: 2026-09-01 | last_used: 2026-09-01 | uses: 1 | tier: working -->\n"
    );
    writeFileSync(
      join(root, "memory", "open-threads", "thread-done-gap.md"),
      "- [x] **Done.** closed work\n  <!-- id: done-gap | created: 2026-09-01 | last_used: 2026-09-01 | uses: 1 | tier: working -->\n"
    );
    const { cont, pinned, threads } = load_repo(root);
    assert.ok(cont.has("live-gap"));
    assert.ok(cont.has("done-gap"));
    assert.ok(pinned.has("live-gap"));    // unchecked -> pinned, never decays
    assert.ok(!pinned.has("done-gap"));   // checked -> decay-eligible for the sweep
    assert.equal(threads.length, 2);
    assert.deepEqual(check_thread_files(threads), []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("thread layer: conflict marker in a thread file is an error", () => {
  const root = threadLayer();
  try {
    writeFileSync(
      join(root, "memory", "open-threads", "thread-t.md"),
      "- [ ] t\n<<<<<<< HEAD\n  <!-- id: t | tier: working -->\n"
    );
    const out = check_conflict_markers(root);
    assert.equal(out.length, 1);
    assert.ok(out[0].includes("memory/open-threads/thread-t.md:2"));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("thread layer: closed bloat counts thread files", () => {
  // 4 closed-record lines in continuity + 4 in a thread file > cap 6 -> flagged once.
  const cont_text = "- [x] closed A\n  line\n  line\n  <!-- id: a | tier: working -->\n";
  const threads = [["thread-b.md", "- [x] closed B\n  line\n  line\n  <!-- id: b | tier: working -->\n"]];
  const out = check_closed_thread_bloat(cont_text, 6, threads);
  assert.equal(out.length, 1);
  assert.ok(out[0].includes("8 line(s)"));
  assert.deepEqual(check_closed_thread_bloat(cont_text, 8, threads), []);
});
