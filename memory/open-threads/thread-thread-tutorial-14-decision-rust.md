- [x] (lock-step — **MERGED 2026-08-07 as PR #193, merge `bea95c80` carrying commit
  `8162b733`, CI green; same-day mirror of Java PR #263; rides the next release via
  INCREMENTS 81**) **tutorial-14's manager approval became a real three-outcome
  decision** (approved → checkpoint; explicit rejected → terminal manager-reject with
  the reason; anything else → re-suspend through await-decision looping back to
  check-approval). Model byte-identical to Java; decide-before-you-suspend + the
  suspensible capability envelope + the wait-loop RESET pattern stated across guide,
  tutorial help, skill help, and AI catalog (suspend entries byte-identical
  cross-engine); the suspend-on-routing-skill error TEACHES at both Rust enforcement
  sites (validator + traveler). **Durable engine facts (mirror of the Java lesson):**
  seen marks survive suspension and a seen node never re-executes — a wait loop across
  suspensions must `RESET:` its own nodes before the IFs; the Playground Tutorials tab
  bakes resources/help/*.md into the webapp bundle at build time — help edits need
  `npm run release` (bundle now index-DK_iWtSl.js). E2E: rejection + wait-loop
  sections in suspend_resume_tutorial.rs; knowledge-graph 9 suites, fmt, clippy 0.
  <!-- id: thread-tutorial-14-decision-rust | created: 2026-08-07 | last_used: 2026-08-07 | uses: 1 | tier: working | origin: 2026-08-07-150018 -->
