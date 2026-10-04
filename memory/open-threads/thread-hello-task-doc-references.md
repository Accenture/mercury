- [x] (drift) **CLOSED 2026-10-02 00:59Z — the Playground's `help update` example runs `no.op`, and the webapp bundle is regenerated.** PR #345 (merge `2f766281`) fixed the two guides; PR #346 (merge `259bca9b`,
  Increment 151) fixed `help update.md` (`task=no.op`) and rebuilt the hashed bundle (`npm ci`, `npm run release`). The committed bundle had not been regenerated since Increment 136 while Increments 141, 143 and 145
  edited the `graph.math` help, so the same PR also brought `CONDITION`, the dialect and `DECIMAL` into the Playground ([[webapp-bundle-follows-help-edits]]).
  Lesson: a help edit is not done until the bundle is rebuilt and committed; `no.op`, not `mock.mdm.profile`, is the always-registered echo. origin: 2026-10-02-001532; close 2026-10-02-010629.
  → served: vision-mercury
  <!-- id: hello-task-doc-references | created: 2026-10-01 | last_used: 2026-10-02 | uses: 3 | tier: active | origin: 2026-10-02-001532 -->
