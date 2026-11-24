- [ ] **`f:addMonths`, `f:monthsBetween`, `f:addYears` and `f:yearsBetween` are built as Increment 210 and open as a pull
  request; the merge waits for the maintainer** (the twin of mercury-composable #667, merged 2026-11-24 as squash
  `df358e62`, part of its issue #664; branch `feat/date-month-year-plugins`, one commit).
  - **State:** the four plugins, the 175 shared vectors and the graph fixture `unit-test-date-arithmetic-2` (both
    byte-identical with the Java commit's), the contract types, the guides, the CHANGELOG entry (Unreleased, Added 3),
    the ledger section and the bundle `index-wWr4LR2P` with the mirrored help page; fmt, clippy and the workspace tests
    were clean before the commit. Committed, pushed and opened on the maintainer's go-ahead ("Open the Rust PR").
  - **Next:** the merge is the maintainer's; record the pull request's number and the merge here when it lands. The
    Java repository then replaces "in the Java engine for now" in its `skills-reference.md` and its CHANGELOG entry. A
    later change to the shared vectors or the three month-end rules lands in both repositories: copy the vector file
    again (`cmp`), follow the rule in `plugins_date.rs` and redeploy with `npm run release:rust`.
  - **Lessons so far:** `BUILTIN_PLUGIN_COUNT` is the number of `#[simple_plugin]` declarations at the start of a line
    in `crates/event-script/src/plugins*.rs`; count them when a plugin is added (it was one short since `rangeLookup`,
    and a floor checked with "at least" fails nothing). This repository's `flow-schema-reference.md` has no plugin
    catalog, so a Java change to that page's plugin tables has no place here.
  - origin: 2026-11-24-035344
  → serves: vision-mercury
  <!-- id: date-month-year-plugins | created: 2026-11-24 | last_used: 2026-11-24 | uses: 1 | tier: working | origin: 2026-11-24-035344 -->
