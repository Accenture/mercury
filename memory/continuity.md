# Continuity — mercury

> Shared ground truth for project state across all agents and sessions.
> Update at the end of every session. Never delete — only archive (see `REVIEW.md`).
>
> Each fact carries a metadata footer in an HTML comment, maintained by the review
> ritual — invisible when rendered, read/written by agents:
> `<!-- id: kebab-id | created: YYYY-MM-DD | last_used: YYYY-MM-DD | uses: N | tier: active -->`
> See `.agent/schema.md` for the fields and `memory/decay-policy.md` for the windows.

---

## Project State

- **project:** mercury
- **status:** **Rust port of `mercury-composable`** (canonical Java v4.8.6), same vision, delivered bottom-up. **All three in-scope layers are ported and milestone-closed** — platform-core (2026-07-16; benchmarked: RPC 155K ops/s @ 6µs, ~8.4× the Java record), event-script (2026-07-17; full engine validated on the canonical Java fixtures), active knowledge graph + Playground webapp (2026-07-18). Kafka service mesh + Spring out of scope. 49 increments — ledger: `docs/INCREMENTS.md`; designs: `docs/design/`; AI-companion validation sweep COMPLETE (all 13 tutorials passed, 2026-07-19; AI grammar self-sufficient — 10 consecutive zero-lookup first-attempt passes incl. two post-sweep drives). Companion surface byte-identical in both ports (Java upstream PRs #188–#199 merged). Human docs site COMPLETE (MkDocs, 20 pages, published via gh-deploy). **GRADUATED to github.com/Accenture/mercury 2026-07-20** (docs live at accenture.github.io/mercury; Rust CI gates in place) — regular PR process from here on. **Version 4.10.5**: tracks the canonical mercury-composable line (Java 4.10.5 in lock-step — one version, two languages; security patch: playground webapp on react-router 8.3.0, dependabot #16 remediated).
- **last_enabled:** 2026-07-15
- **last_session:** 2026-07-26 | agent: Claude Code (2026-07-26-024639)
- **last_review:** 2026-07-26 | through 2026-07-26-022229.md
- **last_invariant_check:** 2026-07-26 | 2026-07-26-014908.md (all five confirmed against live code; two header drifts remedied; ui-fixture carve-out RATIFIED by Eric 2026-07-26)
- **repo:** github.com/Accenture/mercury (official home; graduated 2026-07-20 from the private R&D repo acn-ericlaw/mercury)
- **vision:** `memory/vision.md` (north star, set at enable — Blueprint gaps to be derived)

## Stack & Tools

> Canonical live home for the current stack — language version, dependencies, tool
> versions. `instructions.md` keeps only a high-level descriptor and points here.

**Rust edition 2021**, toolchain 1.95.0 (latest stable at increments 1–2). Cargo **workspace**
(`Cargo.toml` root, members `crates/*`); `crates/platform-core` is the first crate.
**Deps in use:** serde 1, serde_json 1, serde_yaml 0.9 (⚠ archived upstream — works fine;
swap for a maintained fork only if it ever blocks), thiserror 1, log 0.4 (std feature),
tokio 1 (rt-multi-thread/sync/time/macros/net/signal/io-util), async-trait 0.1,
async-channel 2 (per-route MPMC queue), rmp-serde 1 + rmpv 1 (with-serde), uuid 1 (v4),
**hyper 1 (http1/server) + hyper-util 0.1 + http-body-util 0.1** (D10 — REST automation;
deliberately not a web framework: rest.yaml IS the router), **chrono 0.4 + chrono-tz 0.10 +
iana-time-zone 0.1** (event-script date/time plugins; chrono-tz = the ZoneId.of analog,
increment 53), **tokio-rustls 0.26 (ring) +
rustls-native-certs 0.8** (increment 48 — outbound HTTPS with OS-trust-store verification +
`trust_all_cert`; rcgen dev-dep for the self-signed TLS test). Stack rationale:
`platform-core-stack` + design doc D1–D10. `.gitignore` is stack-aware (Rust section:
`target/`, `**/*.rs.bk`, `*.pdb`; Cargo.lock tracked).

**Canonical source:** `mercury-composable` (Java, `com.accenture.mercury:parent-mercury`
**v4.8.6**, Java 21, Maven reactor) at `~/sandbox/mercury-composable` (added by the maintainer
2026-07-15, read-only reference). Its `docs/guides/` (architecture, event-envelope-reference,
api-overview, event-script, knowledge-graph) is the authoritative behavior spec — map, don't
mirror. Key Java deps to find Rust equivalents for: Vert.x event bus + Java 21 virtual threads
(→ async runtime), MsgPack (→ rmp-serde), Gson/JSON (→ serde_json), classgraph annotation
scanning (→ compile-time registration; no runtime scanning in Rust). platform-core alone is
~24.5K LOC / 121 files — a multi-increment port.

## Architectural Invariants

> Hard constraints that must never change. These never decay (treated as `core`).

- **Never couple functions directly** (ADR-0001) — inter-function coupling stays **route-name +
  `EventEnvelope`** only; no direct calls between user functions. This is the defining
  invariant inherited from mercury-composable (the actor-model decoupling); the whole
  three-layer design rests on it. Preserve it in the Rust port. Full ADR ledger:
  `docs/arch-decisions/ADR.md` (ADR-0001…0007 adapted from the Java repo; ADR-0008 native —
  read on demand).
  <!-- id: inv-never-couple-functions | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: core | origin: 2026-07-15-221632.md -->

- **Telemetry/log presentation parity with the Java reference implementation** — the
  trace-record topology (record count per trace, service names, parent edges,
  round_trip-vs-exec kinds, paths) and the log presentation (app-log-context gating,
  header hygiene) of this port must remain an exact structural replica of the Java
  engine's, which is THE reference. Rationale (Eric, 2026-07-23): field installations
  stay POLYGLOT for a long time — DevSecOps teams see both engines' telemetry and logs
  in one aggregation, and any presentation difference is a support burden they will
  flag. This is a standing invariant, not a one-off acceptance criterion; the Java-to-
  Java normalized signature is the acceptance instrument (see increment 64).
  <!-- id: inv-telemetry-presentation-parity | created: 2026-07-23 | last_used: 2026-07-23 | uses: 1 | tier: core | origin: 2026-07-23-152724.md -->

*(More invariants will be distilled from mercury-composable's docs/ADRs as each layer is
ported — e.g. stateless functions, HTTP-style status codes.)*

## Key Decisions

- **String plugins use Unicode scalar values, by maintainer ruling (2026-07-26).**
  `f:length` / `f:substring` count and index WHOLE CHARACTERS (Unicode scalar values /
  code points) — NOT Java's UTF-16 code units (a JVM legacy that must not propagate to
  future Python/Node/Go ports) and NOT UTF-8 bytes (你好 = 2, never 6). Supersedes the
  increment-57/F20 UTF-16 retrofit (code + ledger only — no continuity fact existed to
  supersede formally). Divergence bounded to supplementary-plane characters (emoji = 1
  here, 2 in Java; BMP text identical); out-of-bounds semantics + error messages
  unchanged; the retrofit's surrogate-split micro-divergence is retired (no lossy case
  under scalar indexing). Do NOT re-retrofit UTF-16 in the name of parity — the ruling
  is deliberate and Eric-verified. Docs: syntax-guide Rust-port note + CHANGELOG.
  <!-- id: string-plugins-unicode-scalars | created: 2026-07-26 | last_used: 2026-07-26 | uses: 1 | tier: working | origin: 2026-07-26-022229.md -->

- **Registration metadata is a cross-language contract; carriers are per-language idioms.
  (ADR-0008)** One canonical model + fixed semantics for #[preload] and family (boot-time
  env_instances resolution; optional-service OR/!/= grammar; order-free marker stacking; one
  conflict policy — explicit > declarative, duplicates WARN + last-wins; extension-point
  naming: explicit positional name or same-name derivation from idiomatic declarations;
  plugins = flow vocabulary never gated, features honor gating; discover → register →
  override → resolve → validate → route table; loud-failure discovery; misuse is a tested
  error surface). Spec: docs/guides/registration-metadata-contract.md (adapted from the Java
  reference page). Conformance: golden vectors shared verbatim
  (registration-vectors/{core,plugin,feature}.json, byte-identical to the Java copies) —
  the wire-format golden-vector method applied to the declaration surface. New ports pass
  the three vector suites before their declaration surface is done. Twin of the Java
  ledger's ADR-0009.
  <!-- id: registration-metadata-contract | created: 2026-07-26 | last_used: 2026-07-26 | uses: 1 | tier: working | origin: 2026-07-26-013851.md -->

- **Port bottom-up, faithfully to the Java original** — re-implement mercury-composable in
  Rust layer by layer, foundation → UI (platform-core, then event-script, then active
  knowledge graph), preserving the Java project's behavior. The Java repo is the canonical
  spec (map, don't mirror).
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-07-26 | uses: 92 | tier: active | origin: 2026-07-15-215538.md -->
## Conventions

> Established with the first code (increment 1, 2026-07-15); enforced from the first commit.

- **`cargo fmt` + `cargo clippy --all-targets` clean** is part of "done" for every change
  (default settings, no custom rustfmt.toml yet).
- **Apache-2.0 header** comment on every source file (ported from the Java originals'
  header style).
- Each ported module's `//!` doc names the **Java class it ports** (e.g.
  `org.platformlambda.core.util.ConfigReader`) so reviewers can diff behavior side-by-side.
- **Tests:** unit tests in-module (`#[cfg(test)]`), integration tests in `tests/` with
  fixtures under `tests/resources/` (mirrors Java's `src/test/resources`).
- **Behavior-parity notes** in doc comments wherever the Rust port deliberately mirrors a
  Java quirk (e.g. YAML-tab tolerance) or deliberately diverges — no silent divergence.
- Config-file syntax verbatim (D9): `classpath:/`, `file:/`, `${ENV:default}`, dotted routes.
- **`docs/INCREMENTS.md` is the historical ledger** (maintainer-requested, 2026-07-16):
  one overview row + one section per increment, added as part of each increment's
  definition of done (design rationale stays in `docs/design/platform-core-port.md`;
  the ledger records what shipped when).
- **Example apps are standalone `examples/<name>/` workspace crates** (increment 10,
  2026-07-16): annotated functions + `platform_core::auto_start_main!();` with the app's
  `resources/` beside its `Cargo.toml` — never cargo examples inside a library crate.
  Event-script and knowledge-graph demos land as sibling `examples/<name>/` crates.
- **`tests/ui` compile-fail FIXTURES are test resources — no license headers** (Eric,
  2026-07-26: "ok with the tests/ui without license headers"): a header shifts every
  `.stderr` line and forces TRYBUILD regeneration; treated like Java's
  `src/test/resources` files. The ui RUNNERS (`tests/ui.rs`) do carry headers.
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-07-26 | uses: 93 | tier: active | origin: 2026-07-15-224707.md -->

## Open Threads

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

- [x] **Re-verify invariants (due — 40 sessions since the last check ≥ verify_invariants_every
  40).** Raised by the 2026-07-26 review (cadence); **VERIFIED 2026-07-26 against live code,
  authorized by Eric ("proceed with the invariant re-verification when D5 is done").
  Per-fact verdicts: `inv-never-couple-functions` CONFIRMED (route-string + envelope coupling
  only — examples address peers via set_to("route"); the P1/P2 arcs changed registration
  mechanics, not the coupling model; one substance refresh: the ADR-ledger parenthetical now
  reads 0001…0007 adapted + 0008 native). `inv-telemetry-presentation-parity` CONFIRMED and
  STRENGTHENED (rpc-tag one-record-per-span gate, ENGINE_METADATA_KEYS entry scrub + exit
  sanitize, app-log-context default-on gating all live; the ce_traceparent/hygiene rounds
  ended eight-echoes-identical in v4.10.4/5). `port-bottom-up-faithful` CONFIRMED (three
  layers + macro crates + standalone examples; the whole macro arc practiced
  Java-reference-first, map-don't-mirror). `conventions-rust-baseline` CONFIRMED with two
  remedied code drifts (util/mod.rs + automation/mod.rs lacked the Apache header — headers
  added in the verification commit) and one flagged carve-out for Eric: tests/ui compile-fail
  FIXTURES carry no license header (deliberate — .stderr line numbers depend on fixture
  content; treated as test resources, like Java's src/test/resources). Vision
  (`vision-mercury`) CONFIRMED — north star unchanged, 2026-07-21 current-state context still
  accurate, the annotation-macro arc directly serves the template-for-future-ports
  trajectory. No supersessions required.**
  <!-- id: thread-reverify-invariants-2026q3 | created: 2026-07-26 | last_used: 2026-07-26 | uses: 3 | tier: active | origin: 2026-07-26-013015.md -->

- [x] (2026-07-26; **ARC COMPLETE — P2 MERGED same day: Rust PR
  [#182](https://github.com/Accenture/mercury/pull/182) (merge `fd4685d5`) in lock-step with
  Java PR #235 (squash `84c4957f`). The full ratified scope is delivered: D1/D2/D3a in P1,
  D4 (yaml.preload.override) + D5 (registration-metadata contract page + byte-identical
  golden vectors + 3 conformance suites + ADR-0008) in P2; D3b/D6 deferred by design. The
  branch also carried: the scalar-semantics ruling ([[string-plugins-unicode-scalars]] —
  f:length/f:substring on Unicode scalar values, UTF-16 retrofit superseded, with the
  anti-re-retrofit guard), the Eric-authorized invariant ceremony (all five never-decay
  facts CONFIRMED against live code, two header drifts remedied), and the docs
  housekeeping (AI-companion test log → docs/test-reports/, joined the site nav).**
  P1 was MERGED earlier the same day: Rust PR
  [#181](https://github.com/Accenture/mercury/pull/181) (merge `ecee2df6`, CI 2m26s — the
  first trybuild CI run passed, .stderr matched stable first try) in lock-step with Java
  PR #234 (squash `265f295d`, CI 6m37s). Remaining = P2: D4 yaml.preload.override port +
  D5 registration-metadata contract page + golden conformance fixture + ADR pair
  (trybuild already landed in P1). **P2/D4 IN PROGRESS 2026-07-26: yaml.preload.override
  PORTED on branch `feature/registration-metadata-contract` (1 commit, NOT pushed)** —
  new platform-core `preload_override` module (Java getPreloadOverride/overrideTasks/
  getMatchedPreload/overridePreloadInfo semantics verbatim: comma-separated locations,
  missing/malformed file logged+skipped whole-file, original/routes/instances/
  keep-original entries, multi-file merge = route-set UNION + first-file-set instances
  wins, match on ANY declared comma-split route, declared list REPLACED by sorted set,
  positive instances replaces the env-resolved count, Java log wording); applied in
  AutoStart between inventory collection and registration, after env_instances
  resolution (the resolved value is the "old" in the log). 7-scenario regression suite
  (rename+fan-out shared handler, keep-original, instances override, multi-file merge,
  missing-file chain, alias-declared original, non-matched untouched). Docs: config
  reference gains the key (removed from the not-read list), macros-reference +
  event-script/syntax.md "not ported" blocks rewritten, CHANGELOG Unreleased. Gate: 266
  (265+1) / clippy 0 / fmt. **P2/D5 DONE 2026-07-26 (2nd+3rd commits on the branch): golden vectors copied
  VERBATIM from Java 73a0d1be (diff-verified byte-identical, all three); fixtures through
  the Rust carriers (core kind incl. one marker deliberately ABOVE #[preload] — the
  conformance fixture itself exercises P1's order-freedom; plugin kind pins BOTH naming
  halves — explicit positional "vectorEcho" on an unrelated fn name + derived
  "vectorDerived" from fn vector_derived, the cross-language name-rule proof; feature kind
  gated-in + gated-out); 3 conformance suites (registration_vectors test binaries, declared
  metadata read straight from the inventory + resolved registration from the platform);
  adapted contract page docs/guides/registration-metadata-contract.md (Rust carrier
  canonical, capability N/As as the port's own note, mkdocs nav under Reference,
  macros-reference cross-linked both ways); ADR-0008 in the Rust ledger (twin of Java
  ADR-0009 — cross-ledger numbering note) formalizing the new Key Decision fact
  [[registration-metadata-contract]].**) **Annotation→macro consistency P1 (ratified arc; design:
  Java repo `draft-design-specs/annotation-macro-interop-design.md`; goal: the Rust macro
  surface reads like the Java annotation surface — the template for future Python/Node
  ports). Java's lock-step half (Platform javadoc + PlaygroundLoader WARN) rides the
  same-named Java branch.** (D1) The engine dogfoods its extension points: all 46 built-in
  mapping plugins converted to `#[simple_plugin(name = "...")]` declarations (bodies
  VERBATIM, twin tests pass unchanged; explicit name= on all 46 — the plugin_* fn prefix
  means camelCase derivation never matches, and keyword-named plugins never become fn
  idents); both built-in fetch features converted to `#[fetch_feature]`;
  builtin_registrations()/register_builtins() deleted; plugins_e8.rs is a proper module
  (include! retired); `extern crate self as ...` in both crates so the macros' absolute
  paths resolve in-crate; registry = one link-time inventory fold (OnceLock) + startup
  count assertions (>= 46 plugins at SimplePluginLoader seq 3, >= 2 features at
  GraphResources) so linker elision fails the boot loudly. (D2) One conflict policy:
  explicit register wins over declarative; duplicate name = WARN + last-wins everywhere —
  Java's exact wordings (Reloading SimplePlugin/FetchFeature {name} - please check
  duplicated ...); websocket adds the same-style WARN; features flip from skip-if-present
  to warn+replace; regressions in 2 dedicated test binaries with a capturing logger (incl.
  user #[simple_plugin] shadowing a built-in — the winner is link-order-dependent like a
  Java classpath scan, the WARN is the contract). (D3a) #[fetch_feature] accepts a stacked
  #[optional_service] (platform strip/fold pattern; FetchFeatureEntry.optional_service;
  is_required at boot; "Skip optional FetchFeature - {name}"); per Eric's ruling
  #[simple_plugin] takes NO optional_service (plugins are flow vocabulary — stated in
  macro docs). Stale docs fixed: syntax.md (#[preload] DOES take comma aliases),
  api-overview.md (public/private IS ported). Docs: macros-reference, design notes,
  CHANGELOG Unreleased, increment 70. Gate: workspace 262 (260+2) / clippy 0 / fmt.
  Two Eric-approved addenda folded in: POSITIONAL #[simple_plugin("name")] form (mirrors
  fetch_feature's grammar; all 46 built-ins flipped; name= stays as alias; no-arg keeps
  camelCase derivation — one grammar, portable to Python/Node decorators) and
  ORDER-INSENSITIVE marker stacking (#[zero_tracing]/#[event_interceptor] are real macros
  via the optional_service self-reattachment pattern — above/below/inline all equivalent,
  Java annotation semantics; no compile-fail harness in the workspace, behavioral
  regressions only at first — Eric then upgraded trybuild to THIS round: 3 tests/ui
  compile-fail suites in the runtime crates, 11 fixtures pinning every deliberate macro
  compile error, .stderr committed; no rust-toolchain pin, so a diagnostics-reshaping
  bump regenerates via TRYBUILD=overwrite). Final gate: 265 (262+3) / clippy 0 / fmt.
  P2 (same arc, gated separately): yaml.preload.override port + contract spec/ADR pair.
  P1 and P2 both merged — ARC CLOSED. Relates [[port-bottom-up-faithful]],
  [[registration-metadata-contract]], [[string-plugins-unicode-scalars]].
  <!-- id: thread-annotation-macro-consistency | created: 2026-07-26 | last_used: 2026-07-26 | uses: 3 | tier: active | origin: 2026-07-26-002157.md -->

- [x] (release — 2026-07-24; CLOSED same day) **v4.10.5 SHIPPED AND PUBLISHED in lock-step
  (both repos) — tag `v4.10.5` on merge commit `5ae307c2` (PR
  [#180](https://github.com/Accenture/mercury/pull/180), CI green), release published; the
  Java v4.10.5 published the same day (PR #230, tag on squash `4c82eae0`). Dependabot #16
  CONFIRMED closed by Eric — the security warning is gone. Sixth lock-step release of the
  4.10 arc: 4.10.0 interop → 4.10.1 presentation parity → 4.10.2 boundary demarcation →
  4.10.3 field roll-up → 4.10.4 traceparent carrier + hygiene → 4.10.5 security patch.**
  Prep detail retained: security patch in lock-step with the Java engine's v4.10.5. Content: playground webapp migrated
  react-router-dom ^7.18.1 → react-router ^8.3.0 (coordinator's commit `a2fcb26b`),
  remediating dependabot #16 (React Router RSC Mode CSRF Bypass, follow-up to CVE-2026-22030;
  affected >= 7.12.0 < 8.3.0) — the -dom package ends at 7.18.1 and pins the vulnerable
  react-router exactly; v8 consolidated into the single package. Validation: npm audit 0,
  124 webapp tests, bundle redeployed via npm run release. This closes the dependabot #16
  loop flagged in the v4.10.4 closure push response. Prep commit: sweep 4.10.4→4.10.5 (root
  Cargo.toml; lock regenerated; continuity status line), CHANGELOG
  `## Version 4.10.5, 7/24/2026` Security section. Gate: workspace 260 / clippy 0 / fmt
  (one unidentified single-test failure during the first post-bump compile-storm run; did
  not reproduce across 4 subsequent full runs incl. --no-fail-fast — noted for honesty).
  Tagged + published on both repos — CLOSED.
  <!-- id: thread-release-4-10-5 | created: 2026-07-24 | last_used: 2026-07-24 | uses: 1 | tier: active | origin: 2026-07-24-191554.md -->

- [x] (release — 2026-07-24; CLOSED same day) **v4.10.4 SHIPPED AND PUBLISHED in lock-step
  (both repos) — tag `v4.10.4` on merge commit `03424582` (PR
  [#179](https://github.com/Accenture/mercury/pull/179), CI green, 260 tests), release
  published; the Java v4.10.4 published the same day (PR #229, tag on squash `0125c17b`).
  Fifth lock-step release of the 4.10 arc: 4.10.0 interop → 4.10.1 presentation parity →
  4.10.2 boundary demarcation → 4.10.3 field roll-up → 4.10.4 traceparent carrier + interop
  hygiene.** Prep detail below retained: in lock-step with the Java engine's v4.10.4
  (Java PR #228 merge `fcd4fbc1`; Rust PR #178 merge `b58d2163`). Patch release: configurable traceparent carrier (standards-first —
  the optional name is backward-compat-only; standard traceparent wins inbound) + the
  interop hygiene round (clean delivered envelope view, /api/event wire alignment, x-ttl
  ingress parity), validated by the ce_traceparent four-way drive with all eight echoes
  identical (docs/test-reports/event-over-http-interop.md). Sweep 4.10.3→4.10.4 (root
  Cargo.toml only — members inherit; lock regenerated; continuity status line); CHANGELOG
  Unreleased → `## Version 4.10.4, 7/24/2026` + release summary. Gate: workspace 260 /
  clippy 0 / fmt. Tagged + published on both repos — CLOSED.
  <!-- id: thread-release-4-10-4 | created: 2026-07-24 | last_used: 2026-07-24 | uses: 2 | tier: active | origin: 2026-07-24-183956.md -->

- [x] (in flight — 2026-07-24; RELEASED same day in v4.10.4, [[thread-release-4-10-4]])
  **Interop header-hygiene round, mirrored from the Java reference branch of the same
  name.** The ce_traceparent interop matrix PASSED (report on `docs/interop-ce-traceparent`,
  pushed as `62210de8`) but exposed pre-existing header-hygiene asymmetries; all fixed.
  (1) **Aligned invariant:** a function's delivered ENVELOPE view never contains my_route/
  my_trace_id/my_trace_path/my_correlation_id/x-event-api — the matrix leak was TRANSPORT (the
  hello-flow demo copied its injected view onto the outgoing envelope), NOT injection: the Rust
  delivery never injected my_* into the envelope view and already removed cid/x-event-api; the
  three my_* keys passed through as transported headers. The worker now scrubs all 5 keys from
  the delivered envelope for NON-interceptor functions (shared ENGINE_METADATA_KEYS with the
  exit filter); interceptors keep raw transport fidelity (also RESTORES Java semantics — the
  port previously stripped cid/x-event-api from interceptor envelopes too); legacy
  my_correlation_id honored-then-scrubbed. (2) Demo: hello-flow EventOverHttpRpc filters the 4
  injected my_* keys from its header-copy loop. (3) Wire hygiene on the /api/event client leg:
  insert-semantics engine stamps (stamp_header, Java http.set — kills the doubled
  x-trace-id/traceparent/custom-name headers); NO x-correlation-id on the transport leg (the
  business cid rides the my_cid tag; leg marked via an HTTP-level x-event-api client
  instruction, consumed by the client + HEADERS_TO_IGNORE — Java's "client-side instruction"
  precedent); + accept:*/* + x-small-payload-as-bytes:true (Java's header set, same order);
  + the 3 startup header-name log lines (Java wording). Wire VERIFIED with a raw header-dump
  listener: single trace headers, no x-correlation-id, full Java set, marker off the wire.
  2 regression twins of Java PostOfficeTest (CleanEnvelopeEcho probe reporting both views).
  Gate: workspace 259 (257+2) / clippy 0 / fmt. Docs: event-over-http engine-internals note,
  CHANGELOG Unreleased Fixed ×3, increment 69. **Matrix re-run VERIFIED the hygiene fixes
  independently (all 8 runs leak-free, wire = exact Java header set, startup parity line,
  cross-language span parenting intact); ONE residual fixed as a 2nd commit: the endpoint
  timeout now rides the request dataset as the x-ttl header (ms, Java setTimeoutSeconds
  semantics; caller-sent value WINS — route-wins broke the /api/event in-band remote-timeout
  race, caught by remote_timeout_arrives_in_band). Regression
  endpoint_timeout_rides_the_dataset_as_the_x_ttl_header; gate now workspace 260 / clippy 0 /
  fmt.** **FINAL RULING (Eric) applied as a 3rd commit, mirroring Java `5401f1f8`: inbound,
  the STANDARD traceparent always wins; the custom name is a fallback only when the standard
  is absent/malformed (a well-formed standard traceparent = the legacy system already
  upgraded; residual proprietary header safely ignored — self-consistent with the
  trace.id.header fallback). Precedence regression inverted
  (standard_traceparent_wins_over_custom_header_name); gateway-simulation test unchanged
  (proves the fallback); all "custom name first" doc phrases flipped; outbound dual stamping
  unchanged. Gate: workspace 260 / clippy 0 / fmt.** **MERGED 2026-07-24 as PR #178 (merge `b58d2163`),
  lock-step with Java PR #228 (merge `fcd4fbc1`); ships in v4.10.4
  ([[thread-release-4-10-4]]) — close with that release.** Relates
  [[inv-telemetry-presentation-parity]], [[thread-configurable-traceparent]].
  <!-- id: thread-interop-header-hygiene | created: 2026-07-24 | last_used: 2026-07-24 | uses: 2 | tier: archive-candidate | origin: 2026-07-24-173129.md -->

- [x] (in flight — 2026-07-24; **MERGED same day as PR #177**, merge commit `e99013cb`; the
  ce_traceparent interop matrix verified the custom carrier cross-language; **RELEASED in
  v4.10.4 with the standards-first precedence flip — CLOSED**) **Configurable traceparent header name (field request), mirrored from
  the Java reference in lock-step** (mercury-composable branch of the same name, commit
  `5ee496dd`). `http.traceparent.header` (default `traceparent`) + per-entry
  `traceparent.header` in rest.yaml (precedence per-entry > global > default). Inbound
  (REST automation): parse the custom name FIRST — a well-formed value under it wins, so a
  sidecar-injected standard `traceparent` cannot override the peer's context — standard header
  as fallback for compliant callers. Outbound (async HTTP client + the Event-over-HTTP encode
  path): the same W3C value stamped under BOTH names when they differ (case-insensitive).
  Escape hatch for a gateway allow-list that strips the standard header — the full W3C context
  crosses, so cross-app span parenting survives (beats trace-id conflation). Java's Kafka twins
  (`kafka.traceparent.header` / `secondary.kafka.traceparent.header` + the adapter per-binding
  override) have NO Rust surface (mesh not ported) — skipped per the mirror-what-exists rule.
  Test pattern mirrored: suite-wide `http.traceparent.header=X-Trace-Context` in
  tests/resources/application.properties (the whole platform-core suite runs feature-active,
  proving it additive) + 5 regression twins of the Java tests (custom carries context / custom
  wins over injected standard / standard fallback / per-entry override / dual-stamp
  echo-chain). Gate: workspace 257 (252+5) / clippy 0 / fmt. Docs: config reference,
  observability (impedance row + "renamed traceparent beats conflation" tip), rest-automation
  grammar, reserved-names, HTTP-client guide, CHANGELOG Unreleased, increment 68. Close when
  merged (+ released in the next lock-step patch). Relates [[inv-telemetry-presentation-parity]].
  <!-- id: thread-configurable-traceparent | created: 2026-07-24 | last_used: 2026-07-24 | uses: 3 | tier: archive-candidate | origin: 2026-07-24-160634.md -->

- [x] (release — 2026-07-24; CLOSED same day) **v4.10.3 SHIPPED via the normal flow, in
  lock-step with the Java engine** — tag `v4.10.3` on merge commit `b3804a67`
  (PR [#176](https://github.com/Accenture/mercury/pull/176), CI green, 252 tests),
  release published; the Java v4.10.3 (the field-deployment artifact) published the
  same day (tag on squash `bd7e909d`). Field-deployment roll-up — no engine behavior
  changes; consolidates the 4.10 line for the field: wire format, presentation parity,
  metadata contract, temporary.inbox, collection plugins. Branch `chore/release-4.10.3`
  from main `d2048eea` (PR #175 merge): bump 4.10.2→4.10.3 (root Cargo.toml only; lock
  regenerated), fresh CHANGELOG `## Version 4.10.3, 7/23/2026` — demo hygiene (clean
  envelope-header echo = live proof my_* never rides the wire; hello-flow
  log.format=json, PR #175) + playground webapp npm refresh (incl. dependabot #173;
  audit clean, PR #174). Gate: workspace 252 / clippy 0 / fmt.
  <!-- id: thread-release-4-10-3 | created: 2026-07-24 | last_used: 2026-07-24 | uses: 1 | tier: archive-candidate | origin: 2026-07-24-025820.md -->

- [x] (release — 2026-07-23; CLOSED same day) **v4.10.2 SHIPPED via the normal flow, in
  lock-step with the Java engine** — tag `v4.10.2` on merge commit `6a39bccc`
  (PR [#172](https://github.com/Accenture/mercury/pull/172), CI green after the
  config-race fix `0d09154d`), release published 2026-07-23; the Java v4.10.2 shipped
  the same day (tag on `61ddb772`, PR #222). Patch: metadata
  contract + temporary.inbox alignment + the mirrored collection plugins. Branch
  `chore/release-4.10.2` from main `f86fbec2` (PR #171 merge): collection-plugins
  feature commit (isEmpty/getFirst/getLast — Java PR #220 mirror, error text verbatim,
  45 built-ins) + release bump 4.10.1→4.10.2 (root Cargo.toml only; lock regenerated),
  CHANGELOG `## Version 4.10.2, 7/23/2026` led by the metadata contract + reply-path
  alignment, four-way empty diff noted, interop report linked, PR #171 stamped, plus
  the CI-flake fix (the flow_runtime config-snapshot race — latent on main since
  PR #171; reproduced under load, fixed with the Once setup pattern, 0/80 loaded
  failures after). Gate: workspace 252 / clippy 0 / fmt.
  <!-- id: thread-release-4-10-2 | created: 2026-07-23 | last_used: 2026-07-23 | uses: 1 | tier: archive-candidate | origin: 2026-07-23-231506.md -->

- [x] (feature branch — 2026-07-23; MERGED as PR [#171](https://github.com/Accenture/mercury/pull/171),
  merge `f86fbec2`) **Metadata injection hardening
  (increment 65) IMPLEMENTED on branch `feature/metadata-injection-hardening`** (mirror
  of the Java reference branch of the same name; NOT pushed — Eric gates). Eric's design
  ruling: a function's headers are a COPY with read-only metadata INJECTED at entry and
  SANITIZED at exit — metadata is never transported in the event. Business cid rides the
  new engine-managed `my_cid` tag (wire-compatible `tags` envelope field; converted at
  apply_current_trace, flow task dispatch, REST service events); the worker now injects
  ALL FOUR my_* keys into the input copy (this port previously injected none — echo demos
  are now Java replicas), honors + strips the legacy pre-4.10.2 header, scrubs
  x-event-api and tags from the function's view, and filters my_*/x-event-api at exit.
  REST response echoes X-Correlation-Id (function-set wins); edge stamps the resolved cid
  onto the dataset headers (function/model.cid/response all see the SAME id — verified
  live end-to-end incl. the generated-cid case). Four regression twins + fixture updates.
  **Second commit (increment 66): temporary.inbox alignment** — ONE reserved reply route
  keyed by correlation id (the `inbox.*` namespace freed for applications, e.g.
  inbox.approval); RPC marker = the reserved `rpc` tag; `@origin` never generated,
  parsed away inbound (Eric: mesh-era syntax only); reply dispatch direct on the
  sender's runtime; AsyncHttpClientService global-platform reply bug fixed. Workspace
  250/clippy 0/fmt; span signature UNCHANGED (empty diff re-verified). Serves
  [[inv-telemetry-presentation-parity]]. Close when
  merged.
  <!-- id: thread-metadata-injection-hardening | created: 2026-07-23 | last_used: 2026-07-23 | uses: 3 | tier: archive-candidate | origin: 2026-07-23-213440.md -->

### Blueprint — gaps from Current State (greenfield) to the Vision  (serves: vision-mercury)
> Derived 2026-07-15 from the maintainer-set Vision. Each `(blueprint)` thread is a
> Vision↔reality gap that closes when delivered. Bottom-up order (foundation → UI). Detailed
> per-layer Designs are TODO — the authoritative behavior spec is the Java mercury-composable
> project (map, don't mirror); harvest it into per-layer Designs when a local checkout is
> available and authorized (see the harvest thread below).

  **Design drafted 2026-07-17** (`docs/design/knowledge-graph-port.md` v1) — gate pending.
- [ ] **(blueprint)** Continue **foundation → user interface** once the three layers stand.
  → serves: vision-mercury
  <!-- id: bp-foundation-to-ui | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(backlog) Port the lightweight cloud-native connectors + sync-over-async.** Maintainer
  scope refinement 2026-07-20 (stated while reviewing the docs site): `minimalist-kafka` and
  `twin-kafka` are **lightweight, cloud-native connectors** — distinct from the Kafka service
  mesh (service discovery + sync-over-Kafka, which stays permanently out of scope) — and will
  be ported in future iterations, along with **`sync-over-async`** (the request/response
  bridge over async transports). This is also why HTTP config keys keep their `http.` prefix
  (connector counterparts arrive later). Vision non-goals + instructions + the public
  `docs/background/port-scope.md` all updated to the refined wording. → serves: vision-mercury
  <!-- id: bp-kafka-connectors-backlog | created: 2026-07-20 | last_used: 2026-07-21 | uses: 2 | tier: working | origin: 2026-07-20-030615.md -->

- [ ] **(backlog) Port `ManagedCache` (+ sibling `SimpleCache`).** Java platform-core ships
  `org.platformlambda.core.util.ManagedCache` — a named, self-managing TTL+size-bounded
  cache utility (Caffeine: `expireAfterWrite`, `maximumSize`, default 2000 items, min TTL
  1s; static registry createCache/getInstance/getCacheCollection). NOT ported — Rust
  platform-core has no cache utility; current stand-ins are ad-hoc (playground WS dedup =
  unbounded `Mutex<HashMap>` in `commands.rs::is_duplicate`; fetcher provider cache =
  per-instance state in BOTH engines, so not affected). Needed for: the future connectors
  port ([[bp-kafka-connectors-backlog]] — minimalist-kafka's schema-registry client is a
  heavy ManagedCache user) and Java-API-surface completeness for app developers. Candidate:
  `moka` crate (the Rust Caffeine analog) or a small hand-rolled TTL+LRU; the WS dedup
  cache should adopt it and gain bounded eviction. → serves: vision-mercury
  <!-- id: ot-managedcache-port | created: 2026-07-21 | last_used: 2026-07-21 | uses: 1 | tier: working | origin: 2026-07-21-030938.md -->

- [ ] **(knowledge-harvest) Harvest the canonical vision/specs from mercury-composable (Java).**
  **Gate satisfied 2026-07-15** — the maintainer added `~/sandbox/mercury-composable` and
  authorized reading it (read-only reference). **Harvested this session:** the north-star
  vision (AKG-is-the-application / AI-assisted Semantic Application Development), the accurate
  three-layer model, platform-core's architecture (functions/route-name/`EventEnvelope`/
  `PostOffice`/`Platform`/in-memory bus, virtual-thread execution, lifecycle), the module map,
  and the canonical version (4.8.6) — folded into vision/instructions/invariants above.
  **Still to harvest** (as each layer is ported): platform-core internals (EventEmitter,
  WorkerHandler, serializers), then event-script and knowledge-graph specs + their ADRs.
  → serves: vision-mercury
  <!-- id: ot-harvest-mercury-composable | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: working | origin: 2026-07-15-215538.md -->

## User Preferences

(none recorded yet — record ONLY what the user explicitly states; never infer)

## Team / Members

(none recorded yet)
