- [ ] **`f:addMonths`, `f:monthsBetween`, `f:addYears` and `f:yearsBetween` are built as Increment 210 and open as pull
  request #420; the merge waits for the maintainer** (the twin of mercury-composable #667, merged 2026-11-24 as squash
  `df358e62`, part of its issue #664; branch `feat/date-month-year-plugins`).
  - **State:** the four plugins, the 175 shared vectors and the graph fixture `unit-test-date-arithmetic-2` (both
    byte-identical with the Java commit's), the contract types, the guides, the CHANGELOG entries (Unreleased, Added 3
    and Documentation 1), the ledger section and the bundle `index-wWr4LR2P` with the mirrored help page. Opened on the
    maintainer's go-ahead. On review the maintainer asked for the guides to match the Java pages and for a correct
    built-in plugin count: `flow-schema-reference.md` is now the Java page with its Rust differences marked,
    `command-reference.md` follows the Java page's form, and `tests/builtin_plugin_count.rs` holds the count (67).
  - **Next:** the merge is the maintainer's; record it here when it lands. The Java repository then replaces "in the
    Java engine for now" in its `skills-reference.md` and its CHANGELOG entry, and may take the four corrections its
    `flow-schema-reference.md` needs (the minimal flow, `'* -> *'`, `ext:append`, two `output` keys in the annotated
    example). A later change to the shared vectors or the three month-end rules lands in both repositories: copy the
    vector file again (`cmp`), follow the rule in `plugins_date.rs` and redeploy with `npm run release:rust`.
  - **Found on the way, not acted on:** the flow adapter passes no cookies or session values to a flow although the
    HTTP edge parses both (`crates/event-script/src/adapter.rs`, whose note predates that), and no upload stream or file
    name; `export` always writes the file where the Java engine skips an identical one; `syntax.md` lists the four
    input keys and `datatype` with no *Rust port* note.
  - **Lesson:** `BUILTIN_PLUGIN_COUNT` was one short since `rangeLookup` because the loader checks "at least"; the
    exact test now fails when a plugin is added without the constant.
  - origin: 2026-11-24-035344
  → serves: vision-mercury
  <!-- id: date-month-year-plugins | created: 2026-11-24 | last_used: 2026-11-24 | uses: 1 | tier: working | origin: 2026-11-24-035344 -->
