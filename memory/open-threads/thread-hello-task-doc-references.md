- [ ] **(drift) The Playground's `help update` example still names `v1.hello.task`, the demo function Increment 83 retired.** PR #345 (merge `2f766281`) fixed the two guides
  (*Composing the layers* now runs the built-in `no.op`, checked live as a deployed graph; the reserved-names row dropped the function) and gave the CHANGELOG the #344 line. What
  remains is `crates/knowledge-graph/resources/help/help update.md` (line 24, `task=v1.hello.task`). The help is bundled into the webapp (`import.meta.glob` over `resources/help/*.md`), so the
  fix means `npm run release` in `crates/knowledge-graph/webapp` (its dependencies are not installed: a network install) and a regenerated, hashed bundle: batch it with the next webapp
  release, or make it its own PR if Eric asks. Do not use `mock.mdm.profile` as the replacement: it expects an HTTP-request-shaped body and answers `Missing person id` to `input.body -> *`.
  → serves: vision-mercury
  <!-- id: hello-task-doc-references | created: 2026-10-01 | last_used: 2026-10-01 | uses: 1 | tier: working | origin: 2026-10-02-001532 -->
