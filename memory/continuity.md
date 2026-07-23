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
- **status:** **Rust port of `mercury-composable`** (canonical Java v4.8.6), same vision, delivered bottom-up. **All three in-scope layers are ported and milestone-closed** — platform-core (2026-07-16; benchmarked: RPC 155K ops/s @ 6µs, ~8.4× the Java record), event-script (2026-07-17; full engine validated on the canonical Java fixtures), active knowledge graph + Playground webapp (2026-07-18). Kafka service mesh + Spring out of scope. 49 increments — ledger: `docs/INCREMENTS.md`; designs: `docs/design/`; AI-companion validation sweep COMPLETE (all 13 tutorials passed, 2026-07-19; AI grammar self-sufficient — 10 consecutive zero-lookup first-attempt passes incl. two post-sweep drives). Companion surface byte-identical in both ports (Java upstream PRs #188–#199 merged). Human docs site COMPLETE (MkDocs, 20 pages, published via gh-deploy). **GRADUATED to github.com/Accenture/mercury 2026-07-20** (docs live at accenture.github.io/mercury; Rust CI gates in place) — regular PR process from here on. **Version 4.10.1**: tracks the canonical mercury-composable line (Java 4.10.1 released the same day — one version, two languages; telemetry presentation parity: empty normalized-signature diff in all four direction combinations).
- **last_enabled:** 2026-07-15
- **last_session:** 2026-07-23 | agent: Claude Code (2026-07-23-231506)
- **last_review:** 2026-07-23 | through 2026-07-23-013514.md
- **last_invariant_check:** 2026-07-21 | through 2026-07-21-023208 (confirmed — inv-never-couple-functions + Vision both hold; Vision context refreshed post-graduation)
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
  `docs/arch-decisions/ADR.md` (ADR-0001…0007, adapted from the Java repo — read on demand).
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

- **Port bottom-up, faithfully to the Java original** — re-implement mercury-composable in
  Rust layer by layer, foundation → UI (platform-core, then event-script, then active
  knowledge graph), preserving the Java project's behavior. The Java repo is the canonical
  spec (map, don't mirror).
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-07-23 | uses: 84 | tier: active | origin: 2026-07-15-215538.md -->
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
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-07-23 | uses: 83 | tier: active | origin: 2026-07-15-224707.md -->

## Open Threads

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

- [ ] (release in flight — 2026-07-23) **v4.10.2 release prepared** (patch: metadata
  contract + temporary.inbox alignment + the mirrored collection plugins). Branch
  `chore/release-4.10.2` from main `f86fbec2` (PR #171 merge): collection-plugins
  feature commit (isEmpty/getFirst/getLast — Java PR #220 mirror, error text verbatim,
  45 built-ins) + release bump 4.10.1→4.10.2 (root Cargo.toml only; lock regenerated),
  CHANGELOG `## Version 4.10.2, 7/23/2026` led by the metadata contract + reply-path
  alignment, four-way empty diff noted, interop report linked, PR #171 stamped. Gate:
  workspace 252 / clippy 0 / fmt. NOT pushed — Eric pushes and opens the PR; the Java
  v4.10.2 (its #221 + #220) releases in parallel. Close when tagged and published.
  <!-- id: thread-release-4-10-2 | created: 2026-07-23 | last_used: 2026-07-23 | uses: 1 | tier: working | origin: 2026-07-23-231506.md -->

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
  <!-- id: thread-metadata-injection-hardening | created: 2026-07-23 | last_used: 2026-07-23 | uses: 3 | tier: active | origin: 2026-07-23-213440.md -->

- [x] (release — 2026-07-23; CLOSED same day) **v4.10.1 SHIPPED via the normal flow, in
  lock-step with the Java engine** — tag `v4.10.1` on merge commit `2c4e4066`
  (PR [#170](https://github.com/Accenture/mercury/pull/170), CI green), release
  published 2026-07-23; the Java v4.10.1 shipped the same day (tag on `9ae666df`,
  PR #218). Patch: the telemetry presentation-parity batch.
  Branch `chore/release-4.10.1` from main `ecec21c5` (PR #169
  merge): workspace version 4.10.0→4.10.1 (root Cargo.toml only — members inherit;
  Cargo.lock regenerated; sweep re-verified the surface is just the workspace field),
  CHANGELOG `## Version 4.10.1, 7/23/2026` led by the presentation-parity story (empty
  normalized-signature diff in all four directions; polyglot-DevSecOps rationale) with
  PR #169 stamped on every entry. Gate green at the new version: workspace 245 / clippy
  0 / fmt.
  <!-- id: thread-release-4-10-1 | created: 2026-07-23 | last_used: 2026-07-23 | uses: 1 | tier: working | origin: 2026-07-23-163133.md -->

- [x] (feature branch — 2026-07-23; MERGED as PR [#169](https://github.com/Accenture/mercury/pull/169),
  merge `ecec21c5`) **Telemetry presentation-parity batch
  (increment 64) IMPLEMENTED on branch `feature/event-api-span-and-auth`** (mirror of the
  Java reference branch of the same name; NOT pushed — Eric gates). rust-to-rust trace =
  EXACT replica of java-to-java (empty normalized-signature diff, both patterns; signature
  file /tmp/java-to-java-reference-signature.md). Structural changes: REST automation
  CALLBACK dispatch + `async.http.response` span (first + response legs are real spans;
  business cid moved to the `my_correlation_id` header channel), log-context gating
  (traced lines only — the increment-5 outside-a-trace divergence retired), `my_*`
  response-header strip, `event.api.auth` demo + session-info forwarding,
  demo→declarative rename, hello.pojo forward. Workspace 245/clippy 0/fmt; acceptance
  drive verified (topology/gating/headers/auth). Close when merged.
  <!-- id: thread-telemetry-parity-batch | created: 2026-07-23 | last_used: 2026-07-23 | uses: 2 | tier: active | origin: 2026-07-23-152724.md -->

- [x] (release — 2026-07-23; CLOSED same day) **v4.10.0 SHIPPED via the normal flow, in
  lock-step with the Java engine** — tag `v4.10.0` on merge commit `4dc70337`
  (PR [#168](https://github.com/Accenture/mercury/pull/168), CI green), release notes
  delivered, release published 2026-07-22. (Java v4.10.0 shipped/published the same day,
  tag on merge `af21e6f6`, PR #216; release gate = both parity PRs merged + the live
  bidirectional interop drives passed — permanent record: the Java docs'
  `test-reports/event-over-http-interop` page.)
  Branch `chore/release-4.10.0` (cut from main `1d18883e`, carries the memory-review
  commit): workspace version 4.9.0→4.10.0 (root `Cargo.toml` only — members inherit;
  Cargo.lock regenerated), CHANGELOG `## Version 4.10.0, 7/22/2026` covering PR #166
  (wire format, /api/event service+client, private-by-default, declarative routing, D2/D3
  fixes) + PR #167 (aliases, log-context default-on, one-record-per-span telemetry, demo
  pair, zero-trace span-leak fix) with the interop-report link, event-over-http guide
  links the Java site's Interop Test Report. Gate green at the new version: workspace
  244 / clippy 0 / fmt.
  <!-- id: thread-release-4-10-0 | created: 2026-07-23 | last_used: 2026-07-23 | uses: 1 | tier: working | origin: 2026-07-23-022937.md -->

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
- [x] **(blueprint)** **Synchronous AI-companion feedback — CLOSED 2026-07-23 (human gate
  approved by Eric 2026-07-22: gap already complete and merged in both ports per the
  thread's own text, validated by earlier testing).** Original thread text: make the
  companion a real AI *tool*, not
  a write-then-poll bus. The current `POST /api/companion/{id}` is fire-and-forget (`{status:accepted}`);
  command outcome + errors stream WS-only, so an AI caller is blind (Tut-4: HTTP 200 while the run had
  aborted). Design: an **additive** `POST /api/companion/{id}/sync` returning a structured envelope
  `{ok, output, error, run}` — errors in-band, run/inspect results folded in; WS console + existing
  endpoint unchanged. Mechanism reuses `PostOffice::request` (RPC) + a private capture route
  (`Platform::register`). Design note: `docs/design/ai-companion-sync.md`. **Rust prototype SHIPPED +
  verified end-to-end** (`52318c3`, `post.companion.command.sync`, dev-gated; integration test
  `companion_sync_returns_outcome_in_band`): a fresh Tut-4 companion (session ws-776722-2) rebuilt +
  ran + self-validated all 3 cases **fully autonomously** via `/command`, explicitly never needing the
  WS console or a GET fallback; errors + run results both return in-band. **Real-time human+AI
  collaboration DONE (v1):** the capture sink now **tees** each line to the session's real WS `.out`
  (maintainer's refinement), so a watching human sees it live and — via the command service's existing
  subscriber fan-out — any `session subscribe`d watcher (e.g. a PO on the UI) does too; suspend/resume
  across sprints via export/import. Deterministically tested (`OutTap` in `companion_sync_returns_outcome_in_band`).
  **Live multi-party demo DONE (2026-07-18, session ws-240155-2):** a fresh companion built+ran tut-4
  via `/command` while the maintainer watched TWO UI sessions (architect + PO subscribed); it even hit
  `graph.js`-retired / bitwise / `parseInt` dead-ends and **self-corrected from the in-band errors,
  live** — the whole thesis proven. **AI docs ported into this repo (b DONE):** `docs/llms.txt` +
  `docs/guides/knowledge-graph/{ai-agent-guide,command-reference,minigraph-commands,skills-reference}`
  now live here (reflect Rust: `graph.js` retired, sync endpoint documented, graph.math dialect
  narrowness). URI renamed `/command` → **`/sync`** (`b89a997`; communicates intent). Future companion
  tests reference the Rust repo. **Java upstream: ADR-0008 MERGED (#188) + implementation MERGED
  (#189** — `PostCompanionCommandSync`, mirrors the Rust mechanism: RPC + private capture route + FIFO
  sentinel + WS tee; `CompanionSyncTest`; 64-test module suite green; ADR-0008 → Accepted**) + follow-up
  [#190](https://github.com/Accenture/mercury-composable/pull/190) OPEN** (the standalone
  `examples/minigraph-playground` app loads its own rest.yaml which #189 missed → the route 404'd on the
  example app until #190 adds it). **Java live regression PASSED** (example app :8085): `/sync`
  bad→`ok:false`+error, build+run hello→`ok:true`, `inspect output.body`→`outcome:"hello world"`, and the
  **tee confirmed** (the maintainer's UI streamed every command + rendered the graph live).
  **Drain fix (`b37b131` Rust + PR [#191](https://github.com/Accenture/mercury-composable/pull/191)
  Java):** the earlier "cross-port note" (Java `run` emits less to console) was **WRONG** — the Java WS
  console already echoes output.body/Executed/completed (maintainer screenshot). The real gap was the
  **`/sync` drain**: `run` is the only *asynchronous* command (handler replies, then the traveler
  streams its output), so the post-reply FIFO sentinel raced (and beat) the traversal tail →
  truncated `/sync run` to "Walk to root/end". Fix (both ports): drain a traversal on the traveler's
  **terminal line** (`Graph traversal completed in N ms` | `Graph traversal aborted`, always last);
  sync commands keep the sentinel; the wait is a safety net, the signal drives correctness. Made the
  terminal a contract: every `run` now ends with completed/aborted (early-failure paths emit the reason
  + `emit_aborted`/`emitAborted`), so `run` before `instantiate` returns promptly (ok:false), not on the
  timeout. **`/sync` REST contract now byte-identical across engines** (the companion surface is
  language-neutral). Tests: Rust `playground.rs` (full traversal capture + tee + early-failure
  terminal), Java `CompanionSyncTest` (same); 64-test Java module green. **ALL UPSTREAM PRs MERGED
  (2026-07-18):** #188 (ADR) + #189 (impl) + #190 (example rest.yaml) + the drain fix + #192
  (Thread.sleep→`Utility.sleep` Sonar cleanup) are on `Accenture/mercury-composable` `main`. Note: the
  drain-fix PR #191 was **auto-closed** by GitHub when #190's base branch was deleted on squash-merge
  (GitHub closed the stacked PR instead of retargeting it); recovered as **#193** (same commit
  cherry-picked onto the updated main) and merged. Lesson → [[github-squash-merge-closes-stacked-pr]].
  **The companion `/sync` feature is COMPLETE and MERGED in both ports** (Rust R&D main + Java upstream
  main); the `/sync` REST contract is byte-identical and language-neutral. Blueprint gap closed.
  → serves: vision-mercury
  <!-- id: bp-companion-sync | created: 2026-07-18 | last_used: 2026-07-19 | uses: 22 | tier: archive-candidate | origin: 2026-07-18-162832.md -->

- [ ] **(backlog) Port the lightweight cloud-native connectors + sync-over-async.** Maintainer
  scope refinement 2026-07-20 (stated while reviewing the docs site): `minimalist-kafka` and
  `twin-kafka` are **lightweight, cloud-native connectors** — distinct from the Kafka service
  mesh (service discovery + sync-over-Kafka, which stays permanently out of scope) — and will
  be ported in future iterations, along with **`sync-over-async`** (the request/response
  bridge over async transports). This is also why HTTP config keys keep their `http.` prefix
  (connector counterparts arrive later). Vision non-goals + instructions + the public
  `docs/background/port-scope.md` all updated to the refined wording. → serves: vision-mercury
  <!-- id: bp-kafka-connectors-backlog | created: 2026-07-20 | last_used: 2026-07-21 | uses: 2 | tier: working | origin: 2026-07-20-030615.md -->

- [x] **(blueprint) Parity remediation — COMPLETE 2026-07-21 (increments 50–58: all 8 items + the F2 decision).** A GitHub Copilot
  correctness assessment (24 findings) was independently verified finding-by-finding against
  both codebases (4 parallel agents + direct checks, file:line evidence both sides):
  **20 CONFIRMED real divergences, 2 INTENTIONAL-documented (null-on-spill design; XML
  console text), 2 PARTIAL/overstated (dotted config keys — config normalization makes both
  ports identical; wildcard grammar)**. Maintainer reviewed and directed remediation —
  "serious side effects... affect functional integrity such as telemetry and response
  headers." Fix order (amended from the report): **(1) CRITICAL: REST boundary drops
  function response-envelope headers — DONE 2026-07-21 (increment 50):** the boundary now
  mirrors Java `AsyncHttpResponse.updateHeaders` (content-type override, `|`-separated
  multi-Set-Cookie, `x-stream-id`/`x-ttl` withheld as the deferred streaming contract,
  rest.yaml response filter on the merged map, HEAD = headers-no-body) + the F9 envelope
  header model (case-insensitive `header()`, CR/LF-filtered `set_header()`); 4 new tests
  (raw-socket multi-Set-Cookie assertions); workspace 213/clippy 0/fmt. NEW sub-item
  discovered while porting: Java's `Accept`-based fallback content negotiation — CLOSED
  with item 7 (increment 56); (2) trace continuity — DONE
  2026-07-21 (increment 51): zero-traced hops keep a telemetry-suppressed bracket (trace
  flows to replies + nested calls, no dataset, no span — Java gates only
  startTracing/sendTracingInfo), `send_later` captures context at schedule time (Java
  `touch()` pre-timer), `apply_current_trace` = exact `touch()` mirror (trace id/path
  fill-if-absent, span unconditional; F8 false parity comment fixed); 3 red/green tests in
  telemetry.rs + annotations.rs contract update; workspace 216/clippy 0/fmt; (3)
  Event Script safety — DONE 2026-07-21 (increment 52): `max.model.array.size` cap
  enforced on dynamic RHS indices (Java resolveModelIndex order + message; docs
  contradiction fixed — configuration-reference.md now documents the key), flow-launch
  `body` precondition ("Missing body in dataset", 400, both launch+request); red/green
  fixture dynamic-index-cap + body-precondition test; workspace 217/clippy 0/fmt; (4) date/time plugins — DONE 2026-07-21
  (increment 53): real pattern tokenizer (names/12h/AM-PM/SSS/quoted literals/offsets;
  unsupported letters fail loudly), `f:dateTime` zone arg via chrono-tz + ISO_DATE_TIME
  no-arg form with [zone-id] suffix; parse plugins ride the same converter; deterministic
  zone tests; workspace 218/clippy 0/fmt; deps + micro-divergences documented; (5) fetcher cache key — DONE 2026-07-21
  (increment 54, the last High): key = the dd-namespace map `{node}.dd.{alias}.*`
  (declared inputs only, Java makeRegularHttpCall parity; log + trace annotation now
  report dd-scoped keys); call-counting red/green regression (rust-cache-key fixture +
  mock.cache.counter — pre-fix 2 calls, fixed 1; Java repo can adopt the same fixture);
  workspace 218/clippy 0/fmt; (6) registration + config semantics — DONE
  2026-07-21 (increment 55): register replaces (Java "Reloading" + release) + clamps
  1..=1000 (ServiceDef.setConcurrency; zero no longer 400), resolver loop guard = true
  push/pop chain (repeated/diamond refs resolve; genuine cycles still warn+empty),
  .properties = full java.util.Properties.load (separators/continuations/escapes/
  trailing-whitespace; malformed \u errors); red/green tests incl. reworked
  registration contract test; workspace 220/clippy 0/fmt; (7) REST routing/request/response parity — DONE
  2026-07-21 (increment 56): full Java wildcard grammar (mid-path `*`, `foo*` prefixes,
  no empty-remainder match), 405 for known-path-wrong-method + OPTIONS-without-CORS,
  multi-value query params, cookies map (raw header withheld), raw `query` key, https from
  x-forwarded-proto, trace-path query suffix, AND the increment-50 leftover: Accept-driven
  fallback content negotiation (html wrap; xml→json deferral; no Accept ⇒ no content-type)
  + actuator explicit envelope types (Java ActuatorServices); red/green suite; workspace
  224/clippy 0/fmt; (8) remaining mappings — DONE
  2026-07-21 (increment 57, the FINAL code item): nested `[]` append recursion, list→text
  `List.toString()`, UTF-16 length/substring (emoji = 2 units; split-surrogate → U+FFFD
  documented), launch-failure 500 (manager; client preconditions stay 400 like Java),
  session-guard case-sensitivity, `-0` concat rendering (direct view keeps "-0.0" =
  Double.toString — both Java surfaces), HostUri lastIndexOf split with the
  repeat-in-query quirk; red/green; workspace 230/clippy 0/fmt. **ALL 8 ITEMS DONE — every
  CONFIRMED assessment finding is fixed.** Remaining in this thread: ONLY the F2
  null-on-spill maintainer decision below.
  **Meta-fix:** several Rust doc comments claim "Java parity" where behavior diverges (F8,
  F19, F21, fetcher cache, HostUri) — fix alongside code (no-silent-divergence convention).
  **F2 RESOLVED 2026-07-21 (increment 58): maintainer chose NORMALIZE** — Nil map entries
  strip deterministically on every hop (predicate-guarded `normalize_null_transport` at
  deliver + worker reply); the no-serialization fast path stays; red/green on a warmed
  route (the cold-route race demonstrated the nondeterminism live). Ripple: the HTTP
  boundary's null body key strips on the wire exactly like Java's.
  Full verdict table in the session log. → serves: vision-mercury (faithful port)
  <!-- id: ot-parity-remediation | created: 2026-07-21 | last_used: 2026-07-22 | uses: 11 | tier: archive-candidate | origin: 2026-07-21-030938.md -->

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

- [x] **(blueprint) Event over HTTP — phase 2, cross-language envelope interop — CLOSED
  2026-07-23: PR [#167](https://github.com/Accenture/mercury/pull/167) MERGED (merge
  `c64d0683`) — the parity batch (increment 63) + the one-record-per-span telemetry fixes
  are on main; everything this thread tracked (increments 59–63) is merged in both
  repos.** Original thread text: Handoff
  from the Java session 2026-07-21 (`/tmp/event-envelope-rust-handoff.md`; Java reference
  PR #212, branch `feature/event-envelope-standard-format`): Java adopted a named-key
  "standard" wire format matching this port's `to_vec_named` output (D4's revisit clause
  resolved in the port's favor); normative spec = Java repo
  `docs/guides/event-envelope-wire-format.md`. Handoff REVIEWED (verdict: sound; findings:
  the 403 gate assumes a private-function concept the Rust platform lacks — security
  gate required before /api/event; the vectors never exercise span_id though
  cross-language trace parenting rides on it — flagged to the Java session; round_trip is
  vector-required, not optional). **Increment 1 DONE 2026-07-21 (increment 59):** envelope
  wire-format conformance — body absent-as-nil default (previously FAILED to decode),
  round_trip field, unset-field omission; the five Java golden vectors (copied verbatim)
  decode + round-trip; encoder contract locked (fresh envelope = id + headers only).
  Compact (legacy) format deliberately NOT decoded — v1 standard-only, 400 on compact
  peers. **Increment 3 DONE 2026-07-21 (increment 61): /api/event service + client** — Java
  EventApiService + EventEmitter client ported (`automation/event_api.rs`): service
  registered PRIVATE + in the default rest.yaml (merge_default_endpoints), 403 private
  gate / 404 / 400 / 408, async 202 ack vs RPC mirror, compact-envelope rejection (v1
  standard-only), trace propagation via x-trace-id + traceparent; `event_over_http`
  client returns the reply/ack envelope. E2E test = real HTTP round trip (all paths +
  service-rejects-itself + client round-trip). New `event-over-http.md` guide; two stale
  "not ported" notes corrected. Workspace 235/clippy 0/fmt. **ONLY REMAINING: the live
  cross-language interop pairing with the Java session (composable-example :8100 /
  lambda-example :8085, both directions, RPC + async, 404/403/408 + trace continuity;
  interop target = is_private = false).** **Interop Phase A DONE 2026-07-22** (branch
  `feature/interop-test-service`, UNCOMMITTED — maintainer review pending): hello-world
  gains a public `hello.world` echo (`{body, headers, instance, origin}`; optional
  `sleep_ms` body key delays the reply for the 408 case), app live detached on :8086
  (`-Drest.server.port=8086`; 8085 reserved for the Java lambda-example), all four
  dispatch paths smoke-verified over real HTTP with hand-rolled standard envelopes
  (200 echo + trace context, 408 on ttl, 403 private, 404 unknown). **Phase B DONE
  2026-07-22** (session 2026-07-22-015354): (1) echo binary bug FIXED — the JSON detour
  dropped MsgPack-bin bodies (Nil-normalization stripped the key); echo now reflects raw
  `rmpv::Value` (lesson: relay/echo functions must stay in the MsgPack domain);
  (2) Rust→Java matrix vs the live Java apps: torture echo (unicode + 9007199254740993 +
  list + BINARY), async 202 ack, in-band 404/403, and IN-BAND trace-continuity proof
  (Java ran under the sent trace id) all PASS via the production `event_over_http`
  client; (3) findings — `hello.world`@8085 was still private on the Java side (their
  item), and a Rust platform-core bug found:
  `AsyncHttpRequest::timeout_seconds()` rounded x-ttl DOWN (1500ms→1s read timeout) so a
  peer's in-band 408 lost to the local abort (Java fixed the same bug class during the
  drive; their in-band 408 proven at raw protocol level). **D2 fix DONE 2026-07-22
  (Eric-authorized; session 2026-07-22-023009):** `timeout_seconds()` → ceiling division
  (Java `getTimeoutSeconds` parity), response-timeout site +1s wire grace (Java
  `AsyncHttpClient` parity), `event_over_http` local wait +100ms grace over x-ttl (Java
  EventEmitter parity); regression test `remote_timeout_arrives_in_band` (the Rust twin
  of Java's `EventHttpTest.remoteTimeoutArrivesInBand`); platform-core 162/0, clippy 0;
  LIVE case-6 re-run vs the Java sleeper now passes through the production client
  (in-band 408 "Timeout for 1500 ms" @1.505s). NOTE: the Phase-B task chip for this bug
  was already started by the user — that spun-off session is redundant now.
  **Declarative Event over HTTP DONE 2026-07-22 (increment 62, maintainer-requested —
  "zero code at user app level"; session 2026-07-22-025232):** `yaml.event.over.http`
  (default classpath:/event-over-http.yaml, absent = off) → route→{target, security
  headers} registry (lazy OnceLock; `${...}` substitution; @instance stripped);
  PostOffice send/request hooks forward transparently (request = RPC returning the peer
  reply; send+reply_to = callback dance restoring from/trace/cid; plain send = async 202;
  `x-event-api` recursion guard; send_later rides send); new
  `event_over_http_with_headers`, `EventEnvelope::clear_reply_to`, hook-free
  `request_direct` for the internal HTTP leg (breaks async type recursion +
  defense-in-depth). Twin tests of Java EventHttpTest (config + declarative round trip);
  workspace 237/clippy 0/fmt; docs (guide section + configuration-reference entry +
  INCREMENTS §62; strict docs build deferred to CI — no mkdocs in env). LIVE
  cross-language proof: zero-code po.request + callback dance reached the JAVA peer
  (:8299) through a scratch map. Remaining:
  Eric's review of the interop branch (platform-core fix + declarative feature). Earlier: **Increment 2 (increment 60): private
  functions, both Java paths** —
  `#[preload]` is PRIVATE BY DEFAULT with `is_private = false` opt-out (mirrors Java
  `@PreLoad isPrivate() default true` — a deliberate posture: engine internals become
  private automatically), `register_private` API + `is_private()` query, engine internals
  (distributed.tracing, actuators, no.op, async.http.request, ws routes) registered
  private like Java's EssentialServiceLoader/WsRequestHandler. **Next: increment 3 =
  /api/event service + client (x-ttl/x-async semantics, 403 via is_private, trace
  propagation via x-trace-id + traceparent) + live interop pairing with the Java session
  (both directions, RPC + async, 404/403/408 + trace continuity; the interop target
  function must be declared `is_private = false`). MAINTAINER DIRECTIVES: private-by-
  default = the encapsulation boundary is the app instance (functions cross it only via
  deliberate is_private = false exposure); the /api/event endpoint ships in the DEFAULT
  rest.yaml (merge_default_endpoints, like the actuators — zero user configuration).**
  **UPDATE 2026-07-22/23: increments 59–62 MERGED as PR #166 (merge `e36e5dc5`). Increment
  63 (Java-parity batch pre-4.10) IMPLEMENTED on branch `feature/parity-log-context-aliases`
  (3 commits `9dd64126`/`91347823`/`98644826`, NOT pushed — Eric pushes/opens the PR):
  `#[preload]` comma-separated route aliases; app-log-context ON by default (built-in
  `default-log-context.yaml` via include_str! + `app.log.context` switch — Java 9f9050e1
  mirror); caller-side RPC `round_trip` record with span lineage (Java 04e5618f mirror —
  the port previously emitted NO RPC record at all; companion fix: a zero-traced hop no
  longer leaks a nested reply's span); hello-flow 8086→8100 + the two Event-over-HTTP demo
  endpoints (declarative `/api/event/http/demo` + programmatic
  `/api/event/http/programmatic`, loopback e2e + live two-app cross-app span-tree
  verification); docs walk-through mirrors the Java guide. Workspace 243/clippy 0/fmt.
  Follow-up candidates: port Java's worker-injected `my_route` header (docs claim scoped
  honestly meanwhile); the Java demo flows' `output.body.content-type` mapping looks like
  a typo (flagged to Eric). See session 2026-07-23-005048. **TELEMETRY FIX ROUND
  2026-07-23 (commit `8328d720`, same branch — Eric's interop drives on PR #167 surfaced
  three findings, all fixed + span-level acceptance PASSED): I1 exactly ONE record per
  span (worker suppresses its record for a delivered RPC — Java sendTracingInfo gate;
  RPC marker = inbox reply address; annotations now ride the reply envelope, a new
  wire-compatible field, and fold into the caller's record); I2 round_trip span_id only
  from a DIRECT responder (reply.from == requested route — Java spanIdFromResponder
  140640d8; relayed flow replies keep parent, omit span); I3 programmatic
  event_over_http stamps the caller's trace/span on the wire envelope (apply_current_
  trace at client entry). Workspace 244/clippy 0/fmt. See session 2026-07-23-013514.**
  → serves: vision-mercury
  <!-- id: ot-event-over-http | created: 2026-07-21 | last_used: 2026-07-23 | uses: 10 | tier: active | origin: 2026-07-21-233234.md -->

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
