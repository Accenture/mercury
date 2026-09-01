- [x] (release — 2026-07-30; **TAGGED, then RE-TAGGED pre-publication (Eric's ruling:
  release tags include the updated docs): `v4.11.0` now on merge commit `167484bd` (PR
  [#190](https://github.com/Accenture/mercury/pull/190) — the docs-parity fix: home
  footer "Explore the docs" clusters + Project block with the cross-engine "Java
  version" line + the Release Notes nav link). The original tag on `cc529071` (PR #189)
  was DELETED before publication and re-created — the commit VERIFIED both times
  (Cargo.toml 4.11.0 + the docs fix + both ancestor commits at `167484bd`); remote
  dereference confirmed → `167484bd`. The 4.10.2-round pre-publication tag-move
  precedent applied; a tag NEVER moves after publication. **PUBLISHED 2026-07-30 (Eric
  confirmed, BOTH repos in lock-step) — the suspend/resume feature release is live;
  the arc is CLOSED end to end: design → P1-P5 → consistency review → interop evidence
  → release. Next likely arcs: the minimalist-kafka port (helper servers per
  [[conv-java-helper-servers-for-rust-tests]]) or field feedback on 4.11.0.**) **v4.11.0 release prep, lock-step with the Java
  repo (Eric's plan; the suspend/resume feature release).** Contents this side: the
  complete suspend/resume arc (PR #186), the interop report + cid trim + 8085 port sync
  (PR #187), the nav consolidation (PR #188), plus the ManagedCache port + health-info
  cache + WS-dedup/up_time fixes that rode Unreleased since 4.10.6. Sweep 4.10.6→4.11.0:
  root Cargo.toml `[workspace.package]` (count-asserted single occurrence, no substring
  hazards), lockfile regenerated (11 members at 4.11.0, zero at 4.10.6), continuity
  status line; CHANGELOG Unreleased → `## Version 4.11.0, 7/30/2026`. Branch
  `chore/release-4.11.0`, NOT pushed — Eric gates push/PR/tag (verify the tag lands on
  the verified merge commit — the 4.10.2 tag-race lesson). Java side: 33 poms swept,
  skipTests hardcode removed from 26 poms. Close when tagged + published both repos.
  <!-- id: thread-release-4-11-0 | created: 2026-07-30 | last_used: 2026-07-30 | uses: 4 | tier: archive-candidate | origin: 2026-07-30-172823.md -->
