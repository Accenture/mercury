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
- **status:** **Rust port of `mercury-composable`** (Accenture's event-driven composable app platform; canonical impl in Java v4.8.6), carrying the same vision. In scope: three layers — platform-core → event-script → active knowledge graph (bottom-up, foundation → UI); **Kafka service mesh and Spring out of scope**. Private prototyping repo; graduates to the official Accenture repo once the foundation is sufficient. **platform-core increment 1 (configuration management) is implemented** — Cargo workspace + `crates/platform-core` (MultiLevelMap, ConfigReader, AppConfigReader, resources convention, override registry; 30 tests, clippy/fmt clean). Increment 2 (event-bus foundation) is designed, not yet built.
- **last_enabled:** 2026-07-15
- **last_session:** 2026-07-15 | agent: Claude Code (2026-07-15-224707)
- **last_review:** (none yet)
- **last_invariant_check:** (none yet)
- **repo:** ~/sandbox/mercury
- **vision:** `memory/vision.md` (north star, set at enable — Blueprint gaps to be derived)

## Stack & Tools

> Canonical live home for the current stack — language version, dependencies, tool
> versions. `instructions.md` keeps only a high-level descriptor and points here.

**Rust edition 2021**, toolchain 1.95.0 (latest stable at increment 1). Cargo **workspace**
(`Cargo.toml` root, members `crates/*`); `crates/platform-core` is the first crate.
**Increment-1 deps (in use):** serde 1, serde_json 1, serde_yaml 0.9 (⚠ archived upstream —
works fine; swap for a maintained fork only if it ever blocks), thiserror 1, log 0.4.
**Increment-2 deps (confirmed at the gate, not yet added):** tokio, async_trait,
async-channel (per-route MPMC queue), rmp-serde + rmpv, uuid (`platform-core-stack`).

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

- **Never couple functions directly** — inter-function coupling stays **route-name +
  `EventEnvelope`** only; no direct calls between user functions. This is the defining
  invariant inherited from mercury-composable (the actor-model decoupling); the whole
  three-layer design rests on it. Preserve it in the Rust port.
  <!-- id: inv-never-couple-functions | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: core | origin: 2026-07-15-221632.md -->

*(More invariants will be distilled from mercury-composable's docs/ADRs as each layer is
ported — e.g. stateless functions, HTTP-style status codes.)*

## Key Decisions

- **AI-enabled at the greenfield stage** — the shared memory layer + Vision were installed
  *before* any code, so development is guided by shared memory and intent-traceability
  (VBDI) from the first commit rather than retrofitted later.
  <!-- id: ai-enabled-greenfield | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- **Port bottom-up, faithfully to the Java original** — re-implement mercury-composable in
  Rust layer by layer, foundation → UI (platform-core, then event-script, then active
  knowledge graph), preserving the Java project's behavior. The Java repo is the canonical
  spec (map, don't mirror).
  <!-- id: port-bottom-up-faithful | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: active | origin: 2026-07-15-215538.md -->
- **Kafka service mesh is out of scope** — deliberately not ported, for simplicity.
  <!-- id: kafka-mesh-out-of-scope | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: active | origin: 2026-07-15-215538.md -->
- **Config management first; Spring fully out of scope** (maintainer, 2026-07-15): port
  `AppConfigReader`/`ConfigReader`/`MultiLevelMap` + the `resources/` folder convention as
  **increment 1** — everything (main app, unit tests, integration tests) relies on
  configuration management. Spring (`rest-spring-3/-4`) is **not ported** (Java-only);
  platform-core's own Vert.x-based REST automation remains in scope later. Config-file syntax
  kept verbatim (D9) so config files port unchanged between Java and Rust.
  <!-- id: config-first-spring-out | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: active | origin: 2026-07-15-222816.md -->
- **Private prototyping repo → official Accenture repo later** — iterate rapidly here to keep
  noise away from public readers; move to the official Accenture repo once the foundation is
  sufficient.
  <!-- id: private-repo-then-accenture | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- **platform-core Rust stack (confirmed at the gate, 2026-07-15):** **tokio** async runtime
  (virtual-thread analog; N `instances` = N worker tasks/route), **async_trait**, **serde +
  rmp-serde** (MsgPack bus) + **serde_json** (HTTP edge), **idiomatic serde wire format** (NOT
  byte-compatible with Java — cross-JVM interop is out of scope), Cargo **workspace**
  (`crates/platform-core` first), **explicit registration** now (compile-time `inventory`-style
  macro later; Rust has no runtime annotation scanning). Full rationale + type designs:
  `docs/design/platform-core-port.md`.
  <!-- id: platform-core-stack | created: 2026-07-15 | last_used: 2026-07-15 | uses: 3 | tier: active | origin: 2026-07-15-222242.md -->

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
  <!-- id: conventions-rust-baseline | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-224707.md -->

## Open Threads

> Mark completed items `- [x]` and leave them in place — the review sweeps them to
> the archive once older than `archive_window` sessions. Don't archive them by hand.

- [x] **(vision-bootstrap) Vision set by the maintainer at enable — blueprint derived.**
  Greenfield inverts the bootstrap: the maintainer set the Vision directly (Rust port of
  mercury-composable). `memory/vision.md` is confirmed (no DRAFT), and the high-level
  Blueprint below was derived from the stated plan. VBDI drift-detection is now active
  (advisory until the Designs land).
  <!-- id: ot-vision-confirm-blueprint | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->

- [x] **Greenfield — no code yet.** Resolved 2026-07-15: increment 1 landed the first real
  code (Cargo workspace + `crates/platform-core` config management). Stack recorded in
  `## Stack & Tools`, conventions seeded in `## Conventions` (`conventions-rust-baseline`),
  first invariant recorded at harvest (`inv-never-couple-functions`).
  <!-- id: ot-greenfield-no-code | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: active | origin: 2026-07-15-215538.md -->

### Blueprint — gaps from Current State (greenfield) to the Vision  (serves: vision-mercury)
> Derived 2026-07-15 from the maintainer-set Vision. Each `(blueprint)` thread is a
> Vision↔reality gap that closes when delivered. Bottom-up order (foundation → UI). Detailed
> per-layer Designs are TODO — the authoritative behavior spec is the Java mercury-composable
> project (map, don't mirror); harvest it into per-layer Designs when a local checkout is
> available and authorized (see the harvest thread below).

- [ ] **(blueprint)** Port **platform-core** to Rust — the foundation layer; everything else
  builds on it. First real porting increment. → serves: vision-mercury
  <!-- id: bp-platform-core | created: 2026-07-15 | last_used: 2026-07-15 | uses: 5 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** Port **event-script** to Rust (on top of platform-core).
  → serves: vision-mercury
  <!-- id: bp-event-script | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** Port the **active knowledge graph** to Rust. → serves: vision-mercury
  <!-- id: bp-active-knowledge-graph | created: 2026-07-15 | last_used: 2026-07-15 | uses: 2 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** Continue **foundation → user interface** once the three layers stand.
  → serves: vision-mercury
  <!-- id: bp-foundation-to-ui | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->
- [ ] **(blueprint)** **Graduate to the official Accenture repo** once the foundation is
  sufficient (this private repo is the prototyping stage). → serves: vision-mercury
  <!-- id: bp-graduate-to-accenture | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-215538.md -->

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

- [ ] **(design) Rust port of platform-core — increment 1 DONE; increment 2 (event bus) next.**
  Realizes `bp-platform-core`. Design doc: **`docs/design/platform-core-port.md`** (D1–D9).
  **Increment 1 (configuration management) implemented 2026-07-15** — §8 gate answered
  (Spring naming **verbatim** for side-by-side comparison; a generic `app.profiles.active`
  alias deferred until the foundation is robust): Cargo workspace + `crates/platform-core`
  with `MultiLevelMap`/`ConfigValue` (dot-bracket engine + flat map), `ConfigReader`
  (yaml/yml/json/properties, `classpath:`/`file:`, `${}` substitution + loop detection),
  `AppConfigReader` (`OnceLock` singleton, embedded manifest + app override, merge order,
  profile overlays), `resources` roots (test-shadowing), `overrides` registry. **30 tests
  (12 unit + 18 integration), clippy + fmt clean.** **Next: increment 2** — event-bus
  foundation (EventEnvelope, ComposableFunction, Platform + MPMC worker pool, PostOffice
  send/RPC; §5 of the doc; adds tokio/async_trait/async-channel/rmp-serde/uuid).
  → serves: vision-mercury
  <!-- id: ot-design-platform-core | created: 2026-07-15 | last_used: 2026-07-15 | uses: 4 | tier: working | origin: 2026-07-15-221632.md -->

- [ ] **(backlog) Generic `app.profiles.active` alias for profile selection.** Maintainer
  decision 2026-07-15: keep `SPRING_PROFILES_ACTIVE`/`spring.profiles.active` **verbatim**
  during migration for side-by-side comparison with the Java original; add a generic alias
  once the foundation port is robust. Don't build until then. → serves: vision-mercury
  <!-- id: ot-profiles-alias-backlog | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: working | origin: 2026-07-15-224707.md -->

## User Preferences

(none recorded yet — record ONLY what the user explicitly states; never infer)

## Team / Members

(none recorded yet)
