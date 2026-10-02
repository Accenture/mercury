- [ ] **(drift) Rust docs and help still name `v1.hello.task`, the demo function Increment 83 retired.** `docs/guides/knowledge-graph/composing-the-layers.md` calls it "a shipped demo function" in
  the `graph.task` example, the dev-mock row of `docs/guides/reserved-names-and-headers.md` lists it, and the bundled `help update.md` example names it (the help is compiled into the Playground
  bundle, so that one needs a webapp rebuild, `npm run release` in `crates/knowledge-graph/webapp`). A first-time developer who pastes the guide's example gets a missing route. Fix by pointing the
  examples at a function that exists (the dev-gated `mock.mdm.profile`) or at `async.http.request`. While in those docs, add the #344 `async-trait` bump to the CHANGELOG's Unreleased section, which
  lists every change merged after v4.12.20. Eric decides whether to do it now or leave it.
  → serves: vision-mercury
  <!-- id: hello-task-doc-references | created: 2026-10-01 | last_used: 2026-10-01 | uses: 1 | tier: working | origin: 2026-10-02-001532 -->
