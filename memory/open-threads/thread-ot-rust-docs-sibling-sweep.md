- [ ] (docs) **Rust sibling sweep of the AI-grammar coverage-study fixes — ON HOLD by Eric
  (2026-09-06) until the learnings from the Java round are incorporated** (notably whether the
  study's "claims-fixture test" recommendation lands, which would change how each fix is
  verified). The Java engine's coverage study
  (mercury-composable `draft-design-specs/ai-grammar-coverage-study.md`, its PR #326) found
  10 prose-drift items + recurrent gaps and fixed them there; known Rust siblings already
  spotted: `docs/guides/event-script/syntax.md:481` carries the same `error.status` →
  should be `error.code` drift, and `docs/guides/flow-schema-reference.md:409`'s "Rust port"
  divergence note attributes `error.status` to "the Java reference", which is now FIXED
  upstream — the note's premise is stale (keep only the no-stack-trace divergence). Sweep
  method when unblocked: re-verify each Java fix class against the RUST engine (never assume
  parity), skip Kafka items (no Kafka modules here), coordinates become crates.io
  `cargo add` lines. Trigger: Eric's go, after the Java round's learnings are folded in.
  <!-- id: ot-rust-docs-sibling-sweep | created: 2026-09-06 | last_used: 2026-09-06 | uses: 1 | tier: working | origin: 2026-09-07-014642 -->
