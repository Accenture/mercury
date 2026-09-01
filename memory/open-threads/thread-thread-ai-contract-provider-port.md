- [x] (feature — lock-step port of the Java engine's AI discovery app, implemented and
  **MERGED 2026-08-22 as mercury PR #209, merge `c0f5245e` carrying `30497961` (tree
  verified identical), CI green (test 2m18s + recheck), branches deleted both ends; rides
  the next release via CHANGELOG Unreleased**) **`system/ai-contract-provider` — version-matched operational contract for AI
  discovery.** (Moved from examples/ to system/ 2026-08-21 at Eric's release review —
  same path as the Java module, completing the cross-repo `system/` convention;
  pure relocation, INCREMENTS 89.) Six REST endpoints on 8999 + `--export` offline skill; the seven flow YAMLs
  are BYTE-IDENTICAL to the Java app's and ran unchanged (portability proven on a whole
  app's orchestration). Adaptations: compile-time anchor verification (Class.forName
  analog; knowledge-graph dev-only), build.rs-embedded snapshot from files.list (missing
  doc = build failure), workspace-pinned version (mixed assembly structurally impossible),
  packaged references/llms.txt (self-contained, replaces Java's link rewrite). Contract ids
  identical to Java. Also: `system/AGENTS.md` consumer guide (same path convention as the
  Java repo, Eric's ruling) + root AGENTS.md fork with the role-resolution ladder +
  llms.txt/getting-started discovery entries. 11 tests across 4 binaries; CLI export
  proven live (43 files, hashes independently re-verified). Gate: 63 suites / 317 tests,
  clippy 0, fmt clean. CHANGELOG Unreleased; INCREMENTS 88. Origin log has the
  `#[path]`-inclusion super::-imports gotcha.
  <!-- id: thread-ai-contract-provider-port | created: 2026-08-22 | last_used: 2026-08-22 | uses: 2 | tier: archive-candidate | origin: 2026-08-22-023108 -->
