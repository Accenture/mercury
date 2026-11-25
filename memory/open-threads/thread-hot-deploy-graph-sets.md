- [ ] **Hot deploy and undeploy of graph sets - the twin of the Java engine's RFC-0017, built as Increment 214 on
  2026-11-24 and MERGED 2026-11-25 as PR #424 (commit `838060f3`, merge `8d5580f2`).** A certified graph
  set is deployed to a running engine without a restart, registered on every instance on demand through the set's own
  record `<id>.deployed.yaml`, and removed by name; a task node names its function's provider with `server.url` and
  `server.header[]`. The design and its rulings live in the Java repository (`draft-design-specs/hot-deploy-undeploy-graph-sets.md`,
  RFC-0017 in its register); this repository carries the engine twin (`graphs.rs`, `hot_deployment.rs`,
  `task_server.rs`, the three functions behind their switches), the bundle with the Deploy view, the two scripts and
  the guide text. One difference the port surfaced: the state lock is a plain `Mutex`, so no hook-running registry read
  under it. **Next:** the Java sprint's WP5 - the live drive with both engines on one folder,
  negative controls first - and WP7, the promotion to an ADR there, whose rulings this engine follows.
  - **The live drive DONE 2026-11-25 (Increment 215, with the Java engine on one hot folder; the record is the Java
    repository's `docs/test-reports/hot-deploy-live-drive.md`):** this engine registered a set the other engine deployed
    at its next lookup, served three versions deployed here, answered 403 `is private` to a `server.url` call for a
    function it does not publish (the engines publish none; a pack's extension functions are public and worked), 404
    with the switches off and 503 with the folder refused. Three wording corrections mirrored from the findings (404 for
    off, the digest header as the check on altered bytes, the provider's function public), the scripts and `help deploy`
    byte for byte from Java, the bundle with the Tools menu's `Graph set deployment`. Follow-ups declared in the record:
    an INFO line on the on-demand reconcile of another instance's deploy, the list entry under off-for-this-run, a
    stray `.json` without a WARN, the deploy line's missing caller address. Next: the ADR in the Java repository.
  → serves: vision-mercury
  <!-- id: hot-deploy-graph-sets | created: 2026-11-24 | last_used: 2026-11-24 | uses: 1 | tier: working | origin: 2026-11-25-023528 -->
