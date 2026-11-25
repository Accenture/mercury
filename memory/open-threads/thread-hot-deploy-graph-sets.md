- [ ] **Hot deploy and undeploy of graph sets - the twin of the Java engine's RFC-0017, built as Increment 214 on
  2026-11-24 (branch `feat/hot-deploy-graph-sets`, for a pull request on the maintainer's word).** A certified graph
  set is deployed to a running engine without a restart, registered on every instance on demand through the set's own
  record `<id>.deployed.yaml`, and removed by name; a task node names its function's provider with `server.url` and
  `server.header[]`. The design and its rulings live in the Java repository (`draft-design-specs/hot-deploy-undeploy-graph-sets.md`,
  RFC-0017 in its register); this repository carries the engine twin (`graphs.rs`, `hot_deployment.rs`,
  `task_server.rs`, the three functions behind their switches), the bundle with the Deploy view, the two scripts and
  the guide text. One difference the port surfaced: the state lock is a plain `Mutex`, so no hook-running registry read
  under it. **Next:** the pull request; then the Java sprint's WP5 - the live drive with both engines on one folder,
  negative controls first - and WP7, the promotion to an ADR there, whose rulings this engine follows.
  → serves: vision-mercury
  <!-- id: hot-deploy-graph-sets | created: 2026-11-24 | last_used: 2026-11-24 | uses: 1 | tier: working | origin: 2026-11-25-023528 -->
