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
- **status:** **Rust port of `mercury-composable`** (canonical Java, released lock-step), delivered bottom-up; all three in-scope layers (platform-core, event-script, active knowledge graph + Playground) ported and milestone-closed, **GRADUATED to github.com/Accenture/mercury 2026-07-20** (docs at accenture.github.io/mercury; regular PR process). Kafka service mesh + Spring out of scope. Current release **v4.12.7** (2026-09-11, lock-step with Java v4.12.7 same-day — a documentation-and-templates release, Increments 116–117: the three-layer starter template crates and the AI developer guide, plus the Mercury Family page and the IDD methodology synthesis; NO engine behavior change — the Java headline gate fix has no Rust counterpart; all seven mercury-* crates on crates.io at 4.12.7, API-verified. Prior: v4.12.6 — the sprint release, Increments 110–115). History lives in `docs/INCREMENTS.md`, session logs, and CHANGELOG — not this line. (Condensed 2026-09-04: the smoke test flagged this line for carrying version history against its own rule.)
- **last_enabled:** 2026-07-15
- **last_review:** 2026-09-17 | through 2026-09-17-004239.md
- **last_invariant_check:** 2026-09-17 | 2026-09-17-004239.md (all 7 never-decay facts + the Vision (8 ids) CONFIRMED by Eric after an evidence walkthrough — inv-never-couple-functions, inv-telemetry-presentation-parity, port-bottom-up-faithful, conventions-rust-baseline, conv-declare-consulted-references-rust, eric-release-rhythm-rust, team-eric-maintainer, vision-mercury; the Vision's current-state context refreshed, both Blueprint gaps having closed at the same review's closure gate; thread-reverify-invariants-20260917 closed. Prior: 2026-09-02 | 2026-09-02-184705.md (5 ids) and 2026-07-26 | 2026-07-26-014908.md)
- **repo:** github.com/Accenture/mercury (official home; graduated 2026-07-20 from the private R&D repo acn-ericlaw/mercury)
- **vision:** `memory/vision.md` (north star, set at enable — Blueprint gaps to be derived)

## Stack & Tools

> Canonical live home for the current stack — language version, dependencies, tool
> versions. `instructions.md` keeps only a high-level descriptor and points here.

**Rust edition 2021**, toolchain = **current stable, kept in sync with CI** (1.98.1 as of
2026-09-08; CI installs `dtolnay/rust-toolchain@stable` with no repo pin, so run
`rustup update stable` when formatting disagrees — a 1.95-vs-1.98 rustfmt skew over
match-arm block wrapping failed PR #242's format gate). Cargo **workspace**
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
`trust_all_cert`; rcgen dev-dep for the self-signed TLS test), **moka 0.12 (sync)**
(increment 71 — the ManagedCache engine, Caffeine's Rust lineage, wrapped as an internal
detail; built with `EvictionPolicy::lru` per Eric's deterministic-eviction ruling), **redis 1.5 (`tokio-comp`, `connection-manager`,
`tokio-rustls-comp`, `cluster-async`)** (Increment 119 — the `mercury-redis-connection` foundation shared by
`mercury-sync-over-async` and `mercury-distributed-cache`; `mercury-minigraph-state-redis` keeps its own
connection, as its Java twin does). Stack rationale:
`platform-core-stack` + design doc D1–D10. `.gitignore` is stack-aware (Rust section:
`target/`, `**/*.rs.bk`, `*.pdb`; Cargo.lock tracked).

**Canonical source:** `mercury-composable` (Java, `com.accenture.mercury:parent-mercury`
Java 21, Maven reactor) at `~/sandbox/mercury-composable` (added by the maintainer
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
  `docs/arch-decisions/ADR.md` (ADR-0001…0007 adapted from the Java repo; later entries
  native — read on demand).
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
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-08-30 | uses: 104 | tier: core | origin: 2026-07-15-215538.md -->

- **Playground session broker: an AI agent can HOST a Playground session (2026-09-03, Eric's
  design, contributed from ai-enabled-repo-demo).**
  `examples/minigraph-playground/scripts/playground-session-broker.mjs` (zero-dependency,
  Node ≥ 22, byte-identical to the Java repo's copy) holds a `/ws/graph/playground` session with
  the UI's welcome/ping handshake, auto-reconnects across app restarts, and exposes a localhost
  control API (`GET /session`, `POST /start|/stop`). Humans join with `session subscribe <id>` as
  equal co-authors; the agent drives via companion `/sync`. Smoke-tested against the Java engine;
  `ws_ui.rs` implements the same handshake — a Rust-side smoke test is still owed. Dev-only.
  **Reactivated 2026-09-17** (consulted 2026-09-14 for the broker-first hosting docs; the move back
  was owed since then): the AI docs now lead with the broker and name the keep-alive failure mode;
  the scaffolding manifest carries `scripts/` into derived projects; the broker also ships in
  `templates/starter-graph` (Eric, 2026-09-14).
  <!-- id: playground-session-broker | created: 2026-09-03 | last_used: 2026-09-17 | uses: 6 | tier: active | origin: 2026-09-03-172834.md -->

- **A `for_each` iteration of a suspending subgraph suspends under its OWN record — the store key is
  `graph:{graph_id}:{cid}:{index}`, and that key format is a cross-engine contract (lock-step with the
  Java engine's PR #418; Increment 118, 2026-09-19, PR #284 MERGED — merge commit `c239532b`, feature
  commit `3c98043d`).** Every iteration inherits the parent's business cid by design —
  that inheritance is what makes a subgraph resumable at all — so N concurrent iterations of one
  subgraph collided on `graph:{id}:{cid}` and which suspension survived was a race. Mechanism: the
  extension skill's `for_each` branch carries the position as the `x-iteration-index` HEADER of a
  graph invocation (never the body, never the application-owned `graph-executor` flow; a `flow://`
  target gets no index — Java parity); the executor lifts it into the reserved
  `model.iteration_index` (`RESERVED_MODEL_METADATA` is ten names; `NON_PERSISTED_MODEL_KEYS` aliases
  it, so it is never persisted and never restored); `graph.suspend` puts `index` in the envelope,
  `graph.resume` sends it in the `type=get` body; the Redis store and the file-store mock append it
  ONLY when present, so single delegations and pre-upgrade records keep the two-segment key. A mixed
  Java/Rust fleet shares one Redis, so the two engines must compose the key identically — the reason
  this is a lock-step and not an option. Constraints DECLARED in the guide, not enforced (Eric's
  clean-knowledge-design ruling on the Java side): positional consistency (appending safe; insert/
  remove/reorder shifts positions and a shifted iteration MISSES rather than restoring another item's
  record — pinned), parent → `for_each` → flow → suspending graph unsupported, nested `for_each` with
  suspension a non-goal. Pinned end to end by `rust-orchestrator-foreach` (two records keyed by
  iteration, each holding its own item and counter, both resumed and consumed), which the Java suite
  does not have. Amends ADR-0012 in place (see [[conv-proposals-not-in-adr-ledger-rust]]); relates
  [[fork-join-awaits-on-calling-task]] (found by this increment's test).
  <!-- id: for-each-suspend-index-key-rust | created: 2026-09-19 | last_used: 2026-09-19 | uses: 1 | tier: working | origin: 2026-09-19-022252 -->

- **Fork-join batches are awaited ON THE CALLING TASK, never `tokio::spawn`ed (found 2026-09-19 by
  the `for_each` lockstep's end-to-end test; Increment 118).** The distributed-trace bracket is a tokio
  task-local: a spawned child starts outside it, its `po.request` finds no current trace, the launched
  flow or task runs UNTRACED, and every downstream call inside it falls back to a minted correlation
  id instead of the business cid. All three fan-out skills (`graph.extension`, `graph.task`,
  `graph.api.fetcher`) had carried the pattern since their port — under a single delegation every
  child span shares the parent's trace id, under a fan-out the children emitted none — a violation of
  [[inv-telemetry-presentation-parity]] that the Increment-64 signature instrument never saw because
  it had no fan-out shape. `common::join_batch` (`futures_util::future::join_all`) replaces the spawn
  at all three sites: concurrent, responses in request order, the worker's trace and business cid in
  scope — the Java `po.request(batch, timeout)` shape. **Durable lesson:** a parity assertion proven
  on one shape (the orchestrator test's "the working step sees the business cid") must be re-run on
  EVERY shape the feature has — the fan-out failed it on the first run. Pinned by
  `rust-orchestrator-foreach`'s business-cid assertion. Relates [[for-each-suspend-index-key-rust]],
  [[port-bottom-up-faithful]].
  <!-- id: fork-join-awaits-on-calling-task | created: 2026-09-19 | last_used: 2026-09-19 | uses: 1 | tier: working | origin: 2026-09-19-022252 -->

- **The Redis client layer is the shared `mercury-redis-connection` foundation — `RedisConfig` with a
  configurable key prefix and the plain `redis.*` fallback, the `RedisBackend` standalone-or-cluster seam,
  the reusable `RedisHealthProbe` (Increment 119, 2026-09-19; the Java Q2 extraction in lock-step).**
  sync-over-async and the distributed cache both depend on it, in the Java dependency direction; the
  extraction was behaviour-preserving for sync-over-async (`RedisSettings` = alias of `RedisConfig`,
  `from_config()` = `soa.redis.*` with the `redis.*` fallback — so an existing `redis.*` deployment is
  unchanged, and `soa.redis.*` decouples the rendezvous from the cache when both run). The seam is an
  enum over the `redis` crate's `ConnectionManager` (long-lived), `MultiplexedConnection` (the probe's
  non-healing shape) and `cluster_async::ClusterConnection` (feature `cluster-async`), all
  `aio::ConnectionLike` — every command through one `query`; `MGET` routes per slot in the cluster client.
  Two-key selection (`cluster.detect=auto` → `INFO cluster`, else `cluster.mode`, which is also the
  inconclusive fallback) as Java. **Cluster shipped as the seam plus the branch, tested at the selection
  level** (the double is one node; the branch is proven by its `CLUSTER` exchange) — live-cluster
  behaviour is a certification run, as it was for Java; Eric's confirmation pending (port spec §9 Q1).
  `Platform::on_shutdown` (Java `onShutdown`, v4.12.9) landed with it: hooks run once, newest first,
  isolated, from `AutoStart::run`. Relates [[distributed-cache-rust]]; twin of the Java
  `redis-connection-foundation`.
  <!-- id: redis-connection-foundation-rust | created: 2026-09-19 | last_used: 2026-09-19 | uses: 1 | tier: working | origin: 2026-09-19-182617 -->

- **The distributed cache is `mercury-distributed-cache` — ONE action function `v1.cache.redis` over
  opaque bytes, gated by `redis.cache.enabled`, byte-compatible with the Java module (Increment 119,
  2026-09-19; Java v4.12.9 Q1–Q8 in lock-step; PR #285 MERGED by Eric 2026-09-19, merge commit `5ac55eaf`; on main, unreleased until the catch-up release).** Same action names
  (`PUT`/`GET`/`MGET`/`MPUT`/`DELETE`/`PUT_IF_NOT_PRESENT`/`LIST_PUSH`/`LIST_POP`/`LIST_LEN`), same headers
  (`action`/`key`/`ttl`), same error messages, same key layout `{redis.cache.key.prefix}{key}`, same
  config keys — so a Java pod and a Rust pod share one cache. Every key TTL'd from birth: `SETEX`, atomic
  `SET NX EX`, and `RPUSH`+`EXPIRE` as ONE `MULTI`/`EXEC` step — the port's ruled equivalent of Java's
  `EVAL` (the delta the sync-over-async port set; the RESP double cannot run Lua). `MPUT` is a pipelined
  per-entry `SETEX`, never `MSET`. **Bodies are `Value::Binary`**: a `set_body(Vec<u8>)` would serde a
  list of integers — the one thing a Rust caller can get wrong that a Java caller cannot. Runtime: lazy,
  double-checked build over ONE multiplexed connection, config re-read per failed attempt (a late vault
  credential is picked up; the app boots with Redis down), released via `on_shutdown`, `runtime::set` as
  the test seam. `redis.health` = the foundation probe bound to the plain namespace. The worked example
  (`examples/distributed-cache-example`) runs the Java example's flow and graph files byte-identical and
  stores plain-MsgPack maps under `cache-demo:` — **the cross-engine interop harness**: pointed at one
  Redis, the two examples read each other's profiles (a side-by-side run is the certification step).
  **CERTIFIED 2026-09-20 (Increment 120):** the two examples side by side on one `redis-standalone` —
  112/112 hard checks (`docs/test-reports/distributed-cache-interop.md`, twin in the Java repo): 6 × 6 layer
  matrix, raw wire, cross-engine deletes, identical error shapes, a 54 s outage and recovery. It found the
  example's Layer 1 false-miss on a cache failure (fixed — PR #286 MERGED 2026-09-20, merge `01710589` — [[l1-caller-checks-reply-status-rust]]) and
  trimmed the example's direct `mercury-event-script` dependency ([[conv-cargo-declare-what-you-name]]).
  Builds on [[redis-connection-foundation-rust]]; the example applies [[playground-session-broker]].
  <!-- id: distributed-cache-rust | created: 2026-09-19 | last_used: 2026-09-19 | uses: 1 | tier: working | origin: 2026-09-19-182617 -->

- **A function that awaits `po.request` must check the reply's STATUS before reading its body — the
  engines do it for flows and graphs, imperative code must do it itself (Java ⇄ Rust cache interop,
  2026-09-20, Increment 120).** An `Err(AppError)` from a function arrives at the caller as `Ok(reply)`
  with `reply.has_error()` and the message as a `Value::String` body (platform.rs: `set_status(e.status())`
  + `set_raw_body(String)`), never as `Err` — so `po.request(..).await?` propagates only transport failures
  (408 timeout, closed channel). `ProfileCacheL1` matched the body (`Value::Binary` or nil) and read a
  cache outage as 404 *Profile not found*; a POST would have acked `stored`. Fix: `checked()` in the
  example's main.rs, pinned by `tests/l1_cache_failure.rs` (real cache off, a `#[preload]` fail-fast stub
  on `v1.cache.redis`; L1/L2/L3 → 503). The Java example had the identical gap, masked by its RPC timeout
  racing Lettuce's command timeout — fixed in lock-step. Recorded asymmetry, since CLOSED (2026-09-20): an in-function RPC timeout is **408** here (`Result`)
  and WAS **500** on Java — a Java platform-core mapping gap (status from the outermost exception), fixed there
  with a cause-chain rule; 408 on both engines now, this engine unchanged. Applies to every PostOffice caller, not only the cache. Relates [[rest-error-body-standard-shape]].
  <!-- id: l1-caller-checks-reply-status-rust | created: 2026-09-20 | last_used: 2026-09-20 | uses: 1 | tier: working | origin: 2026-09-20-004627 -->

- **The foundation's command path classifies Redis failures — a timeout is 408, an unreachable Redis 503,
  only a server answer stays 500 — so `v1.cache.redis` fails for what it is, in lock-step with Java (Eric,
  2026-09-20, Increment 121; PR #289 merged `017bf8ed`).** `classify_command_error` (redis-crate `is_timeout` → 408 `Redis request
  timed out - …`; `is_connection_refusal` / `is_connection_dropped` / `is_io_error` / `is_cluster_error` →
  503 `Redis unavailable - …`; else 500 `Redis error - …`), `command_timeout` (the per-command deadline →
  408) and `From<ConnectError> for AppError` (a refused or timed-out connect on a caller's path → 503; an
  unbuildable configuration stays 500 — a defect, not an outage) — applied by `RedisBackend::query` and
  `query_pipeline`, so every consumer of that path inherits it (sync-over-async does not use it). Why: the
  flow and graph engines pass a task's status through *faithfully*; the Layer 2/3 500s during the interop's
  outage leg were the default for a client error that carried none, and Layer 1's 408 was only its RPC timer
  winning a race. **Rule:** set the status where the failure is known, in the function that owns the client.
  Proven by the fifth full drive: 122/122, no outage probe on any layer of either engine answers 500 (this
  engine 408 for its deadline, 503 for refused/broken-pipe; Java all 408 because Lettuce buffers to its command
  timeout). Java twin: `RedisFailure.classify` applied by `RedisCache`. Behaviour change to READ: a caller
  that keyed on 500 for a Redis outage now sees 408/503. Relates [[redis-connection-foundation-rust]],
  [[l1-caller-checks-reply-status-rust]].
  <!-- id: redis-failure-classification-rust | created: 2026-09-20 | last_used: 2026-09-20 | uses: 1 | tier: working | origin: 2026-09-20-004627 -->

- **A function's failure reaches a REST client as the standard error body `{status, message, type:
  error}` — never as bare text (found and fixed 2026-09-19 by the cache example's Layer 1 miss).** Java
  `AsyncHttpResponse.handleException`: an error status with no headers and a string body that does not
  look like JSON or XML renders as the standard error map (JSON unless the client negotiated HTML); the
  Rust server had that shape only for its own routing errors, so `Err(AppError::new(404, "..."))` from a
  service arrived as `text/plain` while the same failure from a flow's exception handler arrived as
  JSON — two shapes for one thing, and the Java-parity assertion on the example's miss body is what caught
  it. `automation/server.rs` now applies the same guard and defaults the body to `application/json` when
  nothing negotiated a type. **Lesson (the third instance this sprint): a Java-parity assertion carried
  into a Rust twin test is the cheapest parity instrument there is — copy the assertion, not just the
  scenario.** Pinned by `function_failure_is_java_shaped_error_body`.
  <!-- id: rest-error-body-standard-shape | created: 2026-09-19 | last_used: 2026-09-19 | uses: 1 | tier: working | origin: 2026-09-19-182617 -->

- **A typed function may return an `EventEnvelope` to set the reply's status, headers and body — the
  `TypedAdapter` honours it AS the reply (2026-09-19, d0b0363e; Java `TypedLambdaFunction<I, EventEnvelope>`
  parity, `WorkerHandler.updateResponse`'s `instanceof`).** Before, the adapter wrapped every `O` as the
  body, and because `EventEnvelope` derives `Serialize` a `TypedFunction<I, EventEnvelope>` compiled and
  silently nested the whole envelope inside the reply body — a trap that only the untyped
  `ComposableFunction` (which always returns an envelope) avoided. Now the output is downcast through
  `Any`: an `EventEnvelope` passes through, anything else is wrapped as before (`O: 'static`, which
  `TypedAdapter::arc` already required). Eric's question surfaced it; the fix rode the distributed-cache
  PR at his direction. Pinned over REST by `typed_function_may_return_an_envelope_to_set_status_and_headers`.
  Relates [[rest-error-body-standard-shape]] (found the same day, the same "Java honours the envelope"
  family); documented in the three authoring surfaces.
  <!-- id: typed-function-envelope-reply | created: 2026-09-19 | last_used: 2026-09-19 | uses: 1 | tier: working | origin: 2026-09-19-182617 -->

## Conventions

> Established with the first code (increment 1, 2026-07-15); enforced from the first commit.

- **`cargo fmt` + `cargo clippy --all-targets` clean** is part of "done" for every change
  (default settings, no custom rustfmt.toml yet).
- **Apache-2.0 header** comment on every source file (ported from the Java originals'
  header style). EXCEPTION ruled by Eric 2026-09-11: `templates/*` starter sources carry a
  ONE-LINE scaffold attribution instead — templates seed field applications that are not
  open source, so the full Accenture copyright header must not ride into user code.
- **Release version bumps must include the starter templates (2026-09-11).** Each
  `templates/*/Cargo.toml` carries an EXPLICIT `version = "<version>"` and mercury-* dep
  pins (deliberately NOT workspace-inherited, so a copied-out template builds as-is after
  deleting the in-repo `path` keys) — the release edit list grows from 5 manifests to 8.
  <!-- id: conv-template-version-sweep-rust | created: 2026-09-11 | last_used: 2026-09-11 | uses: 3 | tier: archive-candidate | origin: 2026-09-11-005808 -->
- Each ported module's `//!` doc names the **Java class it ports** (e.g.
  `org.platformlambda.core.util.ConfigReader`) so reviewers can diff behavior side-by-side.
- **Tests:** unit tests in-module (`#[cfg(test)]`), integration tests in `tests/` with
  fixtures under `tests/resources/` (mirrors Java's `src/test/resources`).
- **Behavior-parity notes** in doc comments wherever the Rust port deliberately mirrors a
  Java quirk (e.g. YAML-tab tolerance) or deliberately diverges — no silent divergence.
- Config-file syntax verbatim (D9): `classpath:/`, `file:/`, `${ENV:default}`, dotted routes.
- **`docs/INCREMENTS.md` is the historical ledger** (maintainer-requested, 2026-07-16):
  one overview row + one section per increment, added as part of each increment's
  definition of done (design rationale stays in `draft-design-specs/platform-core-port.md`;
  the ledger records what shipped when).
- **Example apps are standalone `examples/<name>/` workspace crates** (increment 10,
  2026-07-16): annotated functions + `platform_core::auto_start_main!();` with the app's
  `resources/` beside its `Cargo.toml` — never cargo examples inside a library crate.
  Event-script and knowledge-graph demos land as sibling `examples/<name>/` crates.
- **`tests/ui` compile-fail FIXTURES are test resources — no license headers** (Eric,
  2026-07-26: "ok with the tests/ui without license headers"): a header shifts every
  `.stderr` line and forces TRYBUILD regeneration; treated like Java's
  `src/test/resources` files. The ui RUNNERS (`tests/ui.rs`) do carry headers.
  **Packaging rule (2026-09-20):** every file under `docs/guides/**` and `docs/test-reports/**` must also be
  listed in `system/ai-contract-provider/resources/skill/files.list` — the snapshot test
  `inventory_equals_the_documentation_closure` walks both trees (PR #286's first push failed on a new report).
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-09-02 | uses: 113 | tier: core | origin: 2026-07-15-224707.md -->

- **An application crate declares the Mercury crates it NAMES — Cargo has no Maven-style transitive
  classpath (Eric's question, 2026-09-20; proven on the cache example, Increment 120).** The Java rule is
  ONE dependency (`minigraph-playground-engine` brings the rest, and listing more even collides on
  `index.html`); the Rust rule differs in kind: a crate can only `use` a dependency it declares
  (knowledge-graph re-exports only `inventory` and `fetch_feature`), while transitive crates still LINK —
  `#[preload]` inventory registrations included. So `mercury-platform-core` stays wherever code names
  `platform_core::` items (the `preload`/`main_application` macros, `AppError`, `EventEnvelope`,
  `PostOffice` — every app does), `mercury-knowledge-graph` is the Layer 3 engine, and
  `mercury-event-script` is declared only when code names `event_script::` — the cache example named
  nothing from it, the dependency was removed, and its Layer 2 flow suites still pass (the flow engine
  arrives through knowledge-graph). No Rust analogue of the Java classpath-order trap: each crate prepends
  or appends its resource root explicitly. Applied to `templates/starter-graph` and
  `examples/minigraph-playground` on 2026-09-20 at Eric's direction (branch `chore/cargo-declare-what-you-name`,
  `8868613c`, PR #291 MERGED 2026-09-20, merge `548ce651`; the root README, the template README and the getting-started guide now state the rule).
  <!-- id: conv-cargo-declare-what-you-name | created: 2026-09-20 | last_used: 2026-09-20 | uses: 1 | tier: working | origin: 2026-09-20-004627 -->

- **A `rest.yaml` entry whose service is not registered is SKIPPED at load, and `/` falls back to the `/index.html`
  entry — Java REST semantics the port lacked until Increment 122 (Eric found both running the Playground with
  `-Dapp.env=prod`, 2026-09-20; PR #292 MERGED, merge `567bf57e`).** `RoutingTable::retain_available` drops such entries and `start_http_server`
  warns in Java's words (`Skip [GET] /api/x - Service x not available`; `RoutingEntry.resolveServices`); the REST
  server starts after preload and before the main application on BOTH engines, so a function registered in a main
  application is invisible to rest.yaml in either — parity, not a Rust quirk. The request handler retries a `/`
  miss as `/index.html` before static content (Java `HttpRequestHandler`), so `get.index.html` — dev → the
  Playground's `/public/index.html`, otherwise `/template/index.html` — serves the root as well; static
  `public/index.html` is only the last resort. Consequence: one `rest.yaml` serves dev and production; an
  `#[optional_service]` left out by its condition never leaves a live URL behind, and production never shows the
  React bundle. Verified live in both modes. Relates [[conv-cargo-declare-what-you-name]] (the same Playground
  polish round).
  <!-- id: rest-skip-unregistered-and-root-fallback-rust | created: 2026-09-20 | last_used: 2026-09-20 | uses: 1 | tier: working | origin: 2026-09-20-004627 -->
- **A static decision table is GRAPH DATA — a skill-less node's properties, handed whole to a generic
  function by ONE `graph.task` input entry; never hard-coded in a function bundled with the graph (Eric,
  2026-09-20; Increment 123 — a doc gap, no engine change; branch `docs/static-decision-table-on-a-node`
  `a0d2f4aa` + `3186bde1`, PR pending; Java twin `00283800` + `6ae1a628`).** Found when an AI agent compiled a rule-by-state table
  into a composable function shipped with its graph. `initialize_with_node_properties` already copies every
  node's properties into the state machine at instantiation (skill node → non-reserved keys at
  `{node}.{key}`; skill-less node → the whole map at `{node}`) and the shared LHS resolver reads any
  selector, so `state-rules -> table` maps the table in one entry. **Presentation (Eric's review):** each
  value is a JSON array written as text — `keys=[ "a", "b" ]`, `a=[ "CA", "TX" ]` — which reads as a table
  on the node and arrives as a string the function reconstructs (`serde_json::from_str`; Java
  `SimpleMapper`); `key[]=` lines build a real list instead, and a nested table is one triple-quoted JSON
  text property parsed by `f:json(state-rules.table)` at mapping time. **Why (Eric):** readability — the
  product owner certifies the rules on the graph in the business vocabulary — and one table replaces a
  ladder of IF-THEN-ELSE in `graph.math` or inside a function, so neither a human nor an agent hard-codes
  it; the engine was fully capable all along, and the recipe is what steers the design choice. **Rule:** the product owner reads and certifies the table ON
  the graph, a new table is a new graph version (`v2026-08-prime-rates`) and never a code change, and the
  function stays generic by reading rule names from `table.keys`. The pattern is now in
  `skills-reference.md` (graph.task), the in-Playground help and the AI agent guide's pre-send checklist,
  pinned by `unit-test-task-9` (`graph_runtime.rs`) in lockstep with the Java repo. Extends
  [[conventions-rust-baseline]] (docs and fixtures stay byte-aligned with the reference).
  <!-- id: static-decision-table-is-graph-data-rust | created: 2026-09-20 | last_used: 2026-09-20 | uses: 1 | tier: working | origin: 2026-09-20-152809 -->

- **Declare a Memory Reference when a fact is CONSULTED to make a decision — not only when it is
  edited (Eric agreed, 2026-09-04).** `## Memory References` is the sole input to
  `refresh-metadata`, so an undeclared consultation reads as non-use and decays the fact. In the
  Java sibling this demoted a 42-use core convention after one log declared `(none)` while
  reasoning explicitly from it. Rule of thumb: if you would have decided differently without the
  fact, it is a reference. Twin of `conv-declare-consulted-references` in mercury-composable.
  <!-- id: conv-declare-consulted-references-rust | created: 2026-09-04 | last_used: 2026-09-04 | uses: 1 | tier: core -->

- **A proposal is not a decision: raise it in `docs/arch-decisions/RFC.md` as `RFC-NNNN`, never as a
  `Proposed` ADR (Eric, 2026-09-18 on the Java side; adopted here 2026-09-19 in lock-step — "ADR were
  done in a lockstep so it would require the same treatment").** The ADR ledger is an immutable journey
  of decisions, so an entry is written when a decision is *accepted*, never before; a withdrawn
  proposal has no honest ledger status. `RFC-NNNN` and `ADR-NNNN` are separate sequences. The rule is
  also upstream — agent-memory protocol v4.41.2 and the v4.42.0 governance pair, whose `RFC.md`
  skeleton this repo carries byte-for-byte (status vocabulary `Open · Parked · Promoted → ADR-NNNN ·
  Withdrawn`, entries never deleted, newest first). Adopted in the sitting that accepted the three ADRs
  left at *Proposed* (0012–0014), each verified delivered in the tree first, with ADR-0012 amended in
  place rather than superseded because it had never left *Proposed*. **The snapshot rule, learned from
  the Java red main (#421/#424), applied before the change this time:** the AI-contract snapshot
  link-checks every relative link inside itself, so a doc page the ledger links to must be enumerated
  in TWO places here — `system/ai-contract-provider/resources/skill/files.list` (which `build.rs`
  embeds from) and `snapshot_test.rs`'s fixed extras — a Rust change, not docs-only. Twin of
  `conv-proposals-not-in-adr-ledger` in mercury-composable; relates [[eric-release-rhythm-rust]].
  <!-- id: conv-proposals-not-in-adr-ledger-rust | created: 2026-09-19 | last_used: 2026-09-19 | uses: 1 | tier: core | origin: 2026-09-19-022252 -->

## Blueprint  *(gap from Current State → Vision; `(blueprint)` threads serve `vision-mercury`)*

> The `(blueprint)` items live one-per-file in `memory/open-threads/` (v4.39.0). This section is
> the visible Vision link PROTOCOL expects; the threads carry the detail.
>
> - Both derived gaps closed at the 2026-09-17 review's closure gate (Eric): `bp-foundation-to-ui`
>   (delivered/absorbed — the UI is the lock-step Playground webapp) and `bp-kafka-connectors-backlog`
>   (the Kafka service mesh stays out of scope; minimalist-kafka is in scope and continues under
>   `ot-minimalist-kafka-port` → serves: vision-mercury; sync-over-async shipped 2026-09-13).
> - No open `(blueprint)` gap at present — new gaps are derived as threads when they surface
>   (`grep -l '(blueprint)' memory/open-threads/`).
>
> <!-- restored 2026-09-04 after the smoke test found no Blueprint→Vision link in continuity; updated 2026-09-17 at the closure gate -->

## Open Threads

> Open Threads live **one per file** in `memory/open-threads/` (`thread-<id>.md`;
> filename = the thread's fact id) so concurrent thread work never merge-conflicts
> (v4.39.0). List that directory to see them; unchecked `- [ ]` threads are the live
> workstreams and never decay. Mark a completed thread `- [x]` in its file and leave
> it — the review sweeps it to the archive once older than `archive_window` sessions.
> Don't archive by hand. See `.agent/schema.md`.


## User Preferences

- **Release rhythm (Eric; confirmed for this repo 2026-09-04, in force for many iterations
  already).** Claude Code prepares every release artifact — branch, version sweep, build and test
  verification, CHANGELOG, release notes — but never merges, tags, or publishes without Eric's
  explicit go-ahead for that specific step; **PR-open and tag/publish are each individually
  gated.** Same rhythm as the Java engine's `eric-release-rhythm`, which the two repos exercise
  in lock-step at each shared version. Recorded here after the 2026-09-04 smoke test found this
  section empty while the practice was visible throughout the archive and every recent log —
  raised to Eric rather than inferred, since this section forbids inferring, and confirmed by him.
  <!-- id: eric-release-rhythm-rust | created: 2026-09-04 | last_used: 2026-09-04 | uses: 1 | tier: core | note: an operating preference that does not decay in relevance; core so it cannot fade out of the layer as its Java twin nearly did -->

## Team / Members

- **Eric Law** — maintainer. Directs commits, gates every merge/tag/publish (see the release
  rhythm above), and is the human who confirms invariants and the Vision at the
  `verify_invariants_every` cadence. Rulings recorded through the memory layer are his; the
  agent never ratifies on his behalf.
  <!-- id: team-eric-maintainer | created: 2026-09-04 | last_used: 2026-09-04 | uses: 1 | tier: core -->
