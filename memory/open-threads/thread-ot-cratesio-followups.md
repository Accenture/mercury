- [ ] (follow-ups — crates.io publication round, 2026-09-01) **Registry polish for the
  next release:** (1) `minigraph-state-redis` `readme` pointer → its own `README.md`
  (cargo warns; the local copy correctly ships meanwhile); (2) consider `include_dir!`
  embedding for the engine's bundled resources — today `CARGO_MANIFEST_DIR/resources`
  resolves into the cargo registry cache for crates.io consumers, which breaks
  bare-binary redeploys to other machines (works for build-and-run and containerized
  builds); revisit on field demand; (3) add crate co-owners beyond the maintainer's
  personal account (`cargo owner --add`, per crate — governance for a corporate OSS
  project). origin: 2026-09-01-224359.
  <!-- id: ot-cratesio-followups | created: 2026-09-01 | last_used: 2026-09-01 | uses: 1 | tier: working | origin: 2026-09-01-224359 -->
