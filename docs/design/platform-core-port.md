# Design — platform-core → Rust (foundation port)

> **Status:** DRAFT v2 for maintainer review · **Realizes:** `bp-platform-core` (Blueprint) ·
> **Serves:** `vision-mercury` · **Author:** Claude Code · **Date:** 2026-07-15
> **Canonical source:** `mercury-composable` (Java, v4.8.6) — `system/platform-core`.
> This is a *Design-altitude* artifact in the VBDI loop: it turns the Blueprint gap into a
> concrete plan and traces every choice back to the Vision. Implementation waits on approval.
>
> **v2 (maintainer hint, 2026-07-15):** configuration management (`AppConfigReader`,
> `ConfigReader`, the `resources/` folder convention) is the true first step — *everything*
> (main app, unit tests, integration tests) relies on it. Increment 1 is now **config
> management**; the event-bus foundation moves to increment 2. Spring is confirmed fully
> **out of scope** (Java-only), not merely deferred.

## 1. Goal & scope

Port **platform-core** — the event-driven foundation of mercury-composable — to Rust,
**bottom-up**, starting with the layer everything else stands on. platform-core is ~24.5K LOC /
121 Java files, so this is a **multi-increment** effort. Order of increments:

1. **Configuration management** — `MultiLevelMap`, `ConfigReader`, `AppConfigReader`, the
   `resources/` folder convention (this doc, §4).
2. **Event-bus foundation** — `EventEnvelope`, function trait, `Platform` registry,
   `PostOffice` send/RPC (this doc, §5).
3. Later increments — §8.

The foundation's essence (from the Java architecture guide): self-contained **function actors**
addressed only by **route name**, exchanging immutable **`EventEnvelope`** messages over an
**in-memory event bus**, through a **`PostOffice`** against a **`Platform`** registry —
**functions never call each other directly** (`inv-never-couple-functions`) — all bootstrapped
and parameterized by **configuration files in `resources/`**.

## 2. Confirmed decisions (maintainer gate, 2026-07-15)

| # | Decision | Rationale |
|---|---|---|
| D1 | **tokio** async runtime | The idiomatic analog of Java 21 virtual threads: cheap async tasks, M:N scheduling. `instances = N` → N worker tasks per route. |
| D2 | **`async_trait`** for the function trait | dyn-dispatched async fns in trait objects (registry stores `Arc<dyn ComposableFunction>`). |
| D3 | **serde + rmp-serde** (MsgPack bus) / **serde_json** (HTTP boundary) | Matches Java's "MsgPack on the bus, JSON at HTTP edges." |
| D4 | **Idiomatic serde wire format** (not byte-compatible with Java) | The Kafka mesh / cross-JVM distribution is out of scope, so Rust↔Java envelope interop isn't needed. Revisit only if interop is ever required. |
| D5 | **Cargo workspace**, `crates/platform-core` first | Room for `crates/event-script` and `crates/knowledge-graph` later without restructuring. |
| D6 | **Explicit registration** now; compile-time macro (`inventory`) later *(shipped — increment 10, §5i)* | Rust has no runtime annotation scanning (Java uses classgraph for `@PreLoad`). |
| D7 | **Spring is fully OUT of scope** (maintainer, 2026-07-15) | Spring is Java-only. `rest-spring-3/-4` are not ported — but platform-core's *own* REST automation (`automation/` package, Vert.x-based, no Spring) remains in scope as a later increment. |
| D8 | **Config management is increment 1** (maintainer, 2026-07-15) | Everything relies on it — main app, unit tests, integration tests. Port `AppConfigReader` + `ConfigReader` + the `resources/` convention before the event bus. |
| D9 | **Config files are data — keep syntax verbatim** | `classpath:/` & `file:/` prefixes, `application.yml` semantics, `${ENV_VAR:default}` substitution, dot-bracket composite keys — kept identical so config files port between the Java and Rust versions unchanged. (Same principle as keeping dotted route names.) |

## 3. Architecture mapping (Java → Rust)

| mercury-composable (Java) | mercury (Rust) | Notes |
|---|---|---|
| classpath (`src/main/resources`, `src/test/resources`) | **resource roots** — ordered search dirs (`resources/`, tests add `tests/resources` first) | §4.1 |
| `MultiLevelMap` (+ `Utility.getFlatMap`) | `MultiLevelMap` over `serde_json::Value`-like tree | dot-bracket composite keys |
| `ConfigReader` | `ConfigReader` | yml/yaml/json/properties; `${}` substitution |
| `AppConfigReader` (singleton) | `AppConfigReader` (process-wide `OnceLock`) | bootstrap manifest + profile merge |
| `System.getProperty` override | process-level **override registry** (programmatic `-D` analog) | tests need it; checked first like Java |
| `EventEnvelope` (Object body, MsgPack) | `EventEnvelope` struct; body as `rmpv::Value` | increment 2 |
| `TypedLambdaFunction<I,O>` / `LambdaFunction` | untyped `ComposableFunction` trait (+ typed adapter) | increment 2 |
| `Platform` (singleton registry) | `Platform` (`Arc`, shared) | increment 2 |
| Vert.x event bus | per-route **MPMC channel** (`async-channel`) + N worker tasks | increment 2 |
| `PostOffice.send` / `.request` | `PostOffice::send` / `::request(timeout)` | increment 2 |
| `AppException(status, message)` | `AppError { status, message }` | shared |
| `@PreLoad` classpath scan | `#[preload]` → link-time `inventory` collection (explicit `register` remains underneath) | D6, §5i |

## 4. Increment 1 — configuration management

Faithful port of the Java behavior (read from `AppConfigReader.java`, `ConfigReader.java`,
`MultiLevelMap.java`, `app-config-reader.yml` at v4.8.6):

### 4.1 The `resources/` convention (classpath analog)

Rust has no classpath, but the *convention* is what matters and is kept:

- A **`ResourceResolver`** holds an **ordered list of resource roots**. `classpath:/x.yml`
  searches the roots in order; first hit wins.
- **Default root:** `./resources` (relative to the app's working dir; for tests, the crate's
  `CARGO_MANIFEST_DIR/resources`). **Tests prepend** `tests/resources` — mirroring Java, where
  test resources shadow main resources. Roots are also programmatically extendable (apps and
  higher-layer crates can contribute their own resource dirs, the analog of a jar's resources).
- `file:/path` reads the filesystem directly (absolute or relative), exactly as Java.
- **`../` parent traversal is rejected** (ported guard).
- `.yml` ↔ `.yaml` are interchangeable: if the requested extension misses, the alternative is
  tried (ported behavior).

### 4.2 `MultiLevelMap` (+ flat map)

The composite-key engine used by everything:

```rust
pub struct MultiLevelMap { /* nested map/list tree (ConfigValue) */ }

impl MultiLevelMap {
    pub fn get_element(&self, composite_path: &str) -> Option<&ConfigValue>;   // "a.b[0].c"
    pub fn set_element(&mut self, composite_path: &str, value: ConfigValue);   // creates intermediates
    pub fn remove_element(&mut self, composite_path: &str);
    pub fn exists(&self, composite_path: &str) -> bool;      // non-null value
    pub fn key_exists(&self, composite_path: &str) -> bool;  // key present (may be null)
    pub fn flat_map(&self) -> BTreeMap<String, ConfigValue>; // Utility.getFlatMap analog
}
```

`ConfigValue` is a small enum (Null / Bool / Int / Float / Text / List / Map) — the Rust analog
of Java's untyped `Object` tree, convertible to/from `serde_json::Value` and YAML/properties.
Dot-bracket syntax is validated as in Java (`validateCompositePathSyntax`).

### 4.3 `ConfigReader`

```rust
pub struct ConfigReader { /* MultiLevelMap + cached flat map + resolved flag */ }

impl ConfigReader {
    pub fn load(path: &str) -> Result<Self, ConfigError>;              // resolves references
    pub fn load_raw(path: &str) -> Result<Self, ConfigError>;          // defers resolution (internal, for merging)
    pub fn from_map(map: impl Into<MultiLevelMap>) -> Self;
    pub fn get(&self, key: &str) -> Option<ConfigValue>;
    pub fn get_or(&self, key: &str, default: ConfigValue) -> ConfigValue;
    pub fn get_property(&self, key: &str) -> Option<String>;           // string-enforced
    pub fn get_property_or(&self, key: &str, default: &str) -> String;
    pub fn exists(&self, key: &str) -> bool;
    pub fn is_empty(&self) -> bool;
    pub fn get_map(&self) -> &MultiLevelMap;                            // raw, no substitution
    pub fn get_composite_key_values(&self) -> &BTreeMap<String, ConfigValue>; // substituted, cached
}
```

**Formats:** `.yml`/`.yaml` (YAML), `.json`, `.properties` (full `java.util.Properties.load`
syntax since increment 55, parity F13: `=`/`:`/whitespace separators, backslash line
continuations, `\uXXXX` + single-char escapes, value trailing-whitespace preserved;
sorted-key load). `${...}` resolution keeps a per-segment chain (increment 55, parity F11 —
a repeated `${a} ${a}` or diamond reference is not a false cycle; genuine cycles still
resolve to empty with a warning).
Tabs in YAML are tolerated (replaced with two spaces — ported quirk).

**Lookup precedence in `get(key)`** (ported exactly):
1. **Process override registry** (the `System.getProperty` analog) — if set for `key`, wins.
2. The loaded config tree (composite key).
3. If the value is a string containing `${...}` segments, each segment resolves as:
   **environment variable** → **base-config key reference** (recursive, with **loop
   detection** — a cycle logs a warning and yields empty) → **`:default`** fallback inside
   the braces. Multiple segments per value reconstruct the surrounding text.

### 4.4 `AppConfigReader` (the base config singleton)

Process-wide singleton (`OnceLock`), the substitution base for all other readers:

- Reads **`app-config-reader.yml`** from the resource roots (a built-in default ships in the
  crate — embedded via `include_str!` — and an application-provided copy in its `resources/`
  overrides it, as in Java).
- Manifest shape (kept verbatim): `resources:` — ordered file list to merge (default
  `classpath:/bootstrap.properties`, `bootstrap.yml`, `application.properties`,
  `application.yml`; missing files skipped silently); `profiles:` — overlay prefix (default
  `classpath:/application-`).
- **Merge algorithm** (ported): each file loads *unresolved* → flattened → merged into one
  consolidated flat map (later files override earlier) → **active profiles** resolved →
  `<prefix><profile>.properties` + `.yml` merged on top → keys **sorted** and normalized into a
  `MultiLevelMap` → loaded as the base config → references resolved once at the end.
- **Active profiles** (precedence, ported): env `APP_PROFILES_ACTIVE` → override-registry
  `app.profiles.active` → consolidated config key `app.profiles.active` (comma-separated
  list). **Renamed from the Java original's `SPRING_PROFILES_ACTIVE`/`spring.profiles.active`
  (maintainer decision 2026-07-19, §8 Q1): Spring is irrelevant to the Rust port, so the
  generic names are used outright — a rename, not an alias.** Mechanism/precedence unchanged.

### 4.5 Crate layout (increment 1)

```
mercury/
  Cargo.toml                      # [workspace] members = ["crates/*"]
  crates/platform-core/
    Cargo.toml                    # serde, serde_yaml, serde_json, thiserror; (tokio enters in increment 2)
    resources/
      app-config-reader.yml       # built-in default (embedded with include_str!)
    src/
      lib.rs
      util/
        multi_level_map.rs        # MultiLevelMap, ConfigValue, flat_map, path validation
        config_reader.rs          # ConfigReader + ${} substitution + loop detection
        app_config_reader.rs      # AppConfigReader singleton + profile merge
        resources.rs              # ResourceResolver (roots, classpath:/file:, yml↔yaml)
        overrides.rs              # process override registry (System.getProperty analog)
    tests/
      resources/                  # test fixtures: application.yml, application.properties,
                                  #   application-test.yml, test.properties, test.yaml, …
      config.rs                   # integration tests (§4.6)
```

### 4.6 Test plan — increment 1 (acceptance criteria)

Fixtures modeled on the Java module's own `src/test/resources`:

- **MultiLevelMap:** composite get/set (`a.b.c`, `x.y[0].z`), intermediate creation, remove,
  exists vs key_exists, flat-map round-trip, invalid-path rejection.
- **Formats:** load `.yml`, `.yaml` (and the ext-fallback), `.json`, `.properties`; identical
  keys visible via composite get.
- **resources convention:** `classpath:/` hit from a root; test root shadows main root;
  `file:/` absolute path; `../` rejected; missing file → error (`load`) / skipped (manifest merge).
- **Substitution:** `${ENV_VAR}` (set via std::env in test), `${ENV_VAR:fallback}` default,
  `${config.key}` base-config reference, multi-segment reconstruction (`http://${host}:${port}/x`),
  **loop detection** (`a → b → a` warns, doesn't hang), override registry beats file value.
- **AppConfigReader:** merge order (properties < yml on same key), profile overlay
  (`APP_PROFILES_ACTIVE=test` merges `application-test.yml` on top), singleton identity,
  `get_property` string enforcement.

`cargo build` + `cargo test` + `cargo clippy` clean = increment 1 done.

## 5. Increment 2 — event-bus foundation

*(Unchanged from v1 of this doc; summarized — full type designs preserved below.)*

### 5.1 `EventEnvelope`

Owned, cloneable; metadata (`id`, `to`, `from`, `reply_to`, `cid`, `trace_id`/`trace_path`,
`status` [None ⇒ 200; ≥400 = error], `exec_time`), `headers: HashMap<String,String>`, and a
dynamic `body: rmpv::Value` with `set_body<T: Serialize>` / `body_as<T: DeserializeOwned>`;
fluent builders; `to_bytes`/`from_bytes` via rmp-serde. Later fields (tags, annotations,
span/exception) added as increments need them.

### 5.2 `ComposableFunction` + typed adapter

```rust
#[async_trait]
pub trait ComposableFunction: Send + Sync {
    async fn handle_event(&self, headers: HashMap<String, String>,
                          input: EventEnvelope, instance: usize)
        -> Result<EventEnvelope, AppError>;
}
```

Plus a thin `TypedFunction<I,O>` adapter (deserialize body → typed handler → wrap output) —
the `TypedLambdaFunction<I,O>` bridge and the recommended authoring surface.

### 5.3 `AppError`

`{ status: i32, message: String }` — the `AppException` analog; workers convert `Err` into a
response envelope with that status.

### 5.4 `Platform` (registry + worker pools)

`register(route, f, instances)` / `has_route` / `release` / `routes`. Route validation as Java
(lowercase, dot-separated, ≥1 dot). Each route: one `async-channel` (MPMC) sender + `instances`
worker tasks sharing the receiver → point-to-point to exactly one free worker. (tokio's mpsc is
single-consumer — hence `async-channel`.) Broadcast deferred.

### 5.5 `PostOffice`

`send(event)` fire-and-forget; `request(event, timeout)` RPC via a temporary reply inbox
(unique route + `tokio::sync::oneshot`, `reply_to` + fresh `cid`, `tokio::time::timeout`,
timeout → `AppError{408}`, inbox released either way).

### 5.6 Worker invocation

Worker loop: receive → `handle_event` → `Ok(out)`/`Err(e)→error envelope` → stamp `exec_time`
→ if input had `reply_to`, deliver the response (same `cid`) via the platform. The minimal
analog of `WorkerHandler` + `ServiceQueue` + inbox correlation.

### 5.7 Test plan — increment 2

Envelope round-trip · send reaches exactly one worker · request/RPC with `cid` correlation ·
timeout → 408 · N-instance concurrency (≤ N in flight, all complete) · error path (status
propagates, `has_error()`) · route validation. **Plus, now that config lands first:** worker
counts / kernel-pool-style limits read via `AppConfigReader` where the Java original does.

## 5b. Increment 3 — FIFO reactive back-pressure (manager-worker + elastic queue)

*(Added 2026-07-15 from a maintainer hint: port the FIFO reactive back-pressure handler
for the manager-worker design; **ignore the Berkeley DB implementation**.)*

Increment 2's shared-MPMC dispatch was a simplification; the faithful Java design is:

- **`ElasticQueue`** (port of `ElasticQueue` + `FileElasticStore`, collapsed — with BDB
  ignored, the `ElasticStore` strategy facade has one implementation): a per-route two-tier
  FIFO — first **20** events (`MEMORY_BUFFER`) in memory, overflow spilled to fixed-size
  append-only **segment files** with the **byte-identical record format**
  `[4-byte BE length][payload]`. Segments seal at `elastic.queue.segment.size.bytes`
  (default 16 MB, min 512) and a sealed, fully-consumed segment is **deleted immediately**
  (O(1) reclamation — no compaction/cleaner, the reason the file store replaced BDB).
  Drained queue → counters reset, `generation++` (fresh segment filenames). Holding area:
  `transient.data.store` (default `/tmp/reactive`) + `<application.name>-<origin>` unless
  `running.in.cloud=true`; leftover segments purged at startup; RUNNING marker written.
- **Manager-worker dispatch** (port of the `ServiceQueue` state machine + `WorkerHandler`
  ready-signal protocol): per route, one **manager task** + N **worker tasks**. Workers
  *pull*: announce `Ready` → take one event → process → announce again (at most one
  in-flight event per worker). The manager keeps a ready-worker FIFO (+ uniqueness set);
  with no free worker it enters **buffering** and spills into the elastic queue, draining
  one event per ready signal until empty (then the queue closes and direct dispatch
  resumes). `buffering` starts true (Java parity). The manager's inbound **mailbox is
  bounded** (`elastic.queue.dispatch.mailbox.size`, default 1024, min 20): full mailbox →
  senders await (back-pressure, not drops).
- **Deliberate divergences** (doc-commented): envelopes are serialized (MsgPack) only when
  crossing into the elastic queue (Java serializes every bus message; in-process Rust moves
  are free — the on-disk format stays byte-identical); RPC inboxes ride the same route
  machinery (Java's `AsyncInbox` bypasses `ServiceQueue` — a lighter dedicated inbox is a
  possible later refinement); the RUNNING keep-alive timer, expired-store scan, and
  shutdown-hook housekeeping await the lifecycle increment.

**Tests:** elastic queue (FIFO order across tiers, peek, reuse/generation, incremental
segment reclamation, destroy-purge, empty-write) + end-to-end back-pressure (60-event burst
into a 20 ms single worker → observable disk spill → strict FIFO delivery order → all
segments reclaimed after drain) + the increment-2 suite re-passing over the new dispatch.

## 5c. Increment 4 — application lifecycle (AutoStart/AppStarter + example app)

*(Implemented 2026-07-15.)* Port of `AutoStart` + `AppStarter` + `EntryPoint`
(+ the `EssentialServiceLoader` slot), with the Java startup order exactly:

1. **Essential services** (sequence 0, framework-reserved) — here, the elastic store's
   **housekeeping**, completing increment 3's deferred items: RUNNING liveness marker,
   20 s keep-alive refresh, expired-store scan (stale > 1 h marker or unknown dirs holding
   segments → removed), and `shutdown_cleanup()` for graceful exit.
2. **Before-application hooks** by `sequence` (1–999, clamped; failure **aborts** startup).
3. **Preload** — functions registered and bound to routes (callable from this point).
4. `rest.automation=true` would start the HTTP server — later increment (logs a notice).
5. **Main applications** by `sequence`; missing main = error (Java parity).

`EntryPoint` is one async trait for both hook kinds (Java parity). **Platform identity**
lands here too: `Platform::get_instance()` (process-wide registry; `Platform::new()` stays
for isolated tests), `Platform::name()` (`application.name` → `application` — the Java primary key and
default; the `spring.application.name` fallback retired with the other Spring names,
maintainer decision 2026-07-19), `Platform::origin()` (uuid per process) — the elastic store's holding-area
naming now uses them. **Divergences (doc-commented):** no classpath scanning (D6) —
`AppStarter` is an explicit builder (the `#[preload]` macro is the later ergonomic layer);
Java's global run-once guard not ported (builder is consumed; framework phases idempotent);
shutdown is an explicit call (OS-signal wiring later); Spring branch of `AutoStart` skipped
(out of scope, D7).

**Example app** — the README "greeting.demo" taste, proving increments 1–4 end-to-end:
config with `${GREETING_USER:world}` substitution → preflight hook → `greeting.demo`
preload (instances from config) → main performs route-name RPC and prints the reply.
*(Born here as a cargo example; increment 10 (§5i) moved it to the standalone
`examples/hello-world/` app crate — `cargo run -p hello-world`.)*

**Tests:** phase ordering (out-of-order sequences sort), multiple mains by sequence,
failing hook aborts (no preload, no main), missing-main error, shared global platform,
unknown-holding-area removal + fresh-marker survival, shutdown cleanup. The 1-hour
stale-marker branch is code-reviewed but not time-simulated (no mtime manipulation without
a new dependency — honest note).

## 5d. Increment 5 — OpenTelemetry tracing + business correlation-id + app-log-context

*(Implemented 2026-07-15; maintainer directive: telemetry is foundation, before REST
automation.)* Port of the `Telemetry` service, the trace bracket in `WorkerHandler`, the
`TraceInfo`/`LogContext` design, `W3cTrace`, and the log-context appenders:

- **IDs are W3C/OTel-compatible** (32-hex trace, 16-hex span; `trace::new_trace_id()` /
  `new_span_id()`, Java's `%016x` formula). `util/w3c_trace.rs` ports the `traceparent`
  format/parse for the HTTP boundary (used by REST automation later).
- **Trace bracket** (worker): a traced event (trace id + path) gets a per-execution
  `TraceState` — its own span, parented to the sender's span carried on the envelope's new
  `span_id` field. Java threads this through a per-worker registry keyed by thread id
  (deliberately not ThreadLocal/MDC); the Rust analog is a **tokio `task_local!`** scoped
  around the invocation — torn down when the function returns, same spawn/`Mono`-completion
  boundary as Java. Zero-traced routes: the telemetry plumbing, `inbox.*` (Java's AsyncInbox
  bypasses ServiceQueue), and `skip.rpc.tracing` (default `async.http.request`, verbatim).
  **A zero-traced hop keeps the bracket for CONTINUITY** (increment 51, parity F3): Java
  gates only `startTracing`/`sendTracingInfo` on the flag — the trace context still flows
  to replies and nested calls; the hop emits no telemetry and mints no span into the chain
  (`TraceState.zero_traced`). Deliberate log-only divergence: the hop's own JSON log lines
  resolve trace tokens (Java registers no log context there); nothing changes on the wire.
- **Automatic propagation** (Java `PostOffice.touch()`, exact since increment 51 —
  parity F7/F8): `PostOffice::send`/`request` inside a trace **fill** the outbound event's
  trace id/path **only when absent** (an explicit trace identity always wins), stamp this
  span unconditionally (→ receiver's parent; withheld inside a zero-traced hop), sender
  route, and the **business correlation-id** when the event carries none — cid is a
  separate concern from the trace id, readable via `my_correlation_id()`. `send_later`
  captures the whole context **at scheduling time** (Java wraps in `touch()` before the
  timer; the spawned timer task inherits no task-local bracket). Responses carry the trace back
  (applyTraceContext parity).
- **`Telemetry` service** (`distributed.tracing`, registered by the essential-services
  phase): logs each span dataset `{trace:{id, span_id, parent_span_id, service, path, from,
  origin, start, exec_time, success, status, exception?}, annotations}` in real time; filters
  the plumbing routes; trims `@origin`; forwards to the reserved `distributed.trace.forwarder`
  (the OTLP-exporter hook) and `transaction.journal.recorder` when registered.
- **App-log-context** (opt-in `app-log-context.yaml`): output key → `$token` (live:
  cid/traceId/tracePath/spanId/parentSpanId/service/utc) | `${ENV:default}` (via ConfigReader)
  | literal; absent values omitted, never null. `PostOffice::update_context` adds business
  keys (reserved keys rejected; no-op untraced); `annotate_trace` feeds the span instead —
  two sinks, neither leaks into the other. `logging::init()` installs the process logger
  with **three formats** (the log4j2 appender-selection analog): `text` = plain console,
  context-free (Java Console-appender parity, the default); `json` = **pretty-print** JSON
  (Java `log4j2-json.xml`); `compact` = single-line jsonl, no CR/LF per record (Java
  `log4j2-compact.xml`). Both JSON forms carry the `context` block.
- **`-D` runtime overrides** (maintainer request): `-Dkey=value` command-line arguments are
  parsed into the increment-1 override registry (the `System.getProperty` analog — already
  checked first in every config lookup), so `hello_world -- -Dlog.format=json` switches
  format at launch with no file edit. Idempotent, loaded by `logging::init()` and
  `AppStarter::run()`; other arguments pass through to the application.
- **Divergences (doc-commented):** invalid log-context token = advisory skip (Java throws);
  UTC timestamps; no thread id. One real bug found live: the log-context OnceLock deadlocked
  when the first log line initialized it from inside its own initializer — `logging::init()`
  now initializes it eagerly before installing the logger.

**Demo:** `hello_world` runs traced end-to-end — the function's JSON log line and the
telemetry span carry the **same trace/span ids** (the logs↔spans join), with `cid=order-12345`,
`user` from `update_context`, and `greeting.for` as a span annotation.

**Tests:** two-hop span lineage (same trace; hop-2 `parent_span_id` == hop-1 `span_id`; `from`
recorded; cid propagated to both hops; annotations flow), untraced = no telemetry + no-op
APIs, failure spans (success=false/status/exception), reserved-key rejection, log-context
render (tokens/constants/custom keys/omit-absent/skip-invalid), W3C round-trip, id shapes.

## 5e. Increment 6 — REST automation (core)

*(Implemented 2026-07-16.)* The HTTP protocol boundary — port of the `automation/` package's
core (`RoutingEntry` config, `HttpRouter` dispatch), scoped to **function binding**; the
authoritative schema is the Java project's own `docs/guides/rest-automation/rest-grammar.md`
(the agent-ready spec, mirrored by the parser invariants).

- **D10 — HTTP stack: hyper 1.x** (+ hyper-util, http-body-util) on tokio. Deliberately
  *not* a web framework: `rest.yaml` **is** the router — axum/actix would impose a second
  routing layer; hyper is the minimal, canonical HTTP/1.1 server. tokio gains `net`+`signal`.
- **`rest.yaml`** (loaded from `yaml.rest.automation`, default `classpath:/rest.yaml`):
  `rest` entries (service [function route], methods [GET PUT POST DELETE HEAD PATCH;
  OPTIONS auto], url with `{param}` + trailing `*` [case-insensitive], timeout [default 30 s,
  clamped 1 s–5 m], `cors`/`headers` refs [must exist], `authentication` [simple route form],
  `tracing`, per-entry `trace.id.header`/`correlation.id.header` impedance overrides) +
  `cors` blocks (options/headers, `Access-Control-*` lines) + `headers` blocks
  (request/response add/drop/keep). Parser invariants enforced per the grammar.
- **Dispatch:** method+path match with the FULL Java RoutingEntry grammar (increment 56,
  parity F14): exact literals > `{param}` captures > wildcards; mid-path `*` matches one
  segment, `foo*` prefix-matches one segment, and a trailing `*`/`foo*` lets the URL run
  longer — but never shorter (`/api/files/*` does not match `/api/files`). A known path
  under a wrong method answers **405** "Method not allowed" (the Java getSimilarRoute
  marker); OPTIONS without a CORS block (or with empty options) is 405, never a bare 204.
  Request mapped to the **`AsyncHttpRequest`** shape (`method`, `url`, `ip`, `headers`,
  `parameters.path`/`parameters.query` — repeated query values become a LIST, single stays
  a string — `body`, `https` from `x-forwarded-proto`, `host`, top-level `query` = the raw
  query string, `cookies` = the parsed cookie map with the raw header withheld; the trace
  path carries the query string). The body follows the Java
  `handlePayload` content-type dispatch **exactly** (parity fix 2026-07-19; no JSON sniffing,
  no default content type): `application/json` → map/list when bracket-wrapped, else raw text
  (parse failure falls back to text; empty → `{}`); `application/xml` → raw text (XML parse
  deferred, as on the client's response side); `application/x-www-form-urlencoded` (exact) →
  fields into `parameters.query`, body null; `text/html`/`text/plain` → raw text; anything
  else incl. a missing content type → MsgPack **binary** (Java `byte[]`; empty → null).
  Content-type is matched on the `;charset`-stripped value, case-sensitively like Java.
  N.B. the Java client sends **no default content-type** (wire-verified) — callers must
  declare `application/json` to get a parsed map, which is why the canonical POST-provider
  fixtures map `text(application/json) -> header.content-type`
  → `po.request(service, timeout)` → envelope mapped back (status; body: map/list→JSON,
  text→text/plain, bytes→octet-stream; response header transforms + CORS headers).
  **Function response-envelope headers are preserved** (increment 50, Java
  `AsyncHttpResponse.updateHeaders` parity): `content-type` overrides the body-derived type
  (lowercased; skipped for HEAD), `set-cookie` splits on `|` into one header line per cookie,
  `x-stream-id` (`stream.*.in` shape) + `x-ttl` are recognized as the response-streaming
  contract and **withheld** (streaming stays deferred), all other headers join the response
  map that the rest.yaml response transform filters (content-type/cookies bypass the filter,
  as in Java). HEAD responses carry headers, never a body. Envelope header model matches Java
  `EventEnvelope`: case-insensitive `header()` lookup, CR/LF filtered on `set_header()`.
  The fallback content type is negotiated from the request's `Accept` header (increment 56,
  Java `updateContentType`): html → text/html (map/list bodies HTML-wrapped, `handleMapContent`),
  json or `*/*` → application/json, NO Accept → no content-type header at all, anything else →
  text/plain; an `application/xml` Accept negotiates JSON (the port's XML deferral — never
  claiming xml on the wire). Actuator endpoints set explicit envelope content types exactly
  like Java `ActuatorServices`.
  Errors are the Java JSON shape `{status, message, type:"error"}`; timeout → 408; OPTIONS
  preflight → CORS options headers.
- **The edge starts traces** (the piece increments 5 was built for): a **business
  correlation-id is always ensured** (per-entry/global header, else generated) — independent
  of tracing — set on the envelope and exposed via the reserved `my_correlation_id` request
  header; with `tracing: true` the trace id comes from a valid W3C **`traceparent`**
  (wins; its parent-id becomes our parent span) else the trace-id header else generated;
  trace path = `METHOD /path`. Legacy conflation (trace + cid sharing one header name)
  resolves to one id. Authentication runs before dispatch (non-true / error → 401/custom).
- **Deferred** (per §7): ~~flow binding~~ *(shipped with event-script E-3 — the `flow:` key injects x-flow-id)*, HTTP(S)
  relay + `url_rewrite`/`trust_all_cert`, A/B dual service, multipart upload, static-content,
  the default-rest.yaml actuator merge (needs actuator services), response streaming.

**Tests:** rest.yaml parse (invariants: bad method, missing cors ref, timeout clamp),
matcher precedence + param extraction, end-to-end HTTP over an ephemeral port (200 JSON,
path params, 404 shape, CORS preflight + response headers, header transforms, auth 401,
timeout 408, **traced request → telemetry span with `GET /path` + edge-started trace id,
`traceparent` parent-span adoption, cid always present**).

## 5f. Increment 7 — actuator endpoints + static HTML content

*(Implemented 2026-07-16, maintainer-directed scope.)* Port of `ActuatorServices` + the
default-endpoint merge + the static-content behavior:

- **`src/actuator.rs`** — one implementation, four registrations (Java switches on the
  `my_route` header; the Rust port parameterizes by `ActuatorKind` at registration —
  cleaner, no header magic), registered in the lifecycle's essential-services phase with a
  shared `ActuatorContext` (app identity resolved once; the **liveness flag follows the
  most recent health outcome** — Java `healthStatus`):
  - `/info` — app{name, version (`info.app.version`, default platform-core's), description
    (`info.app.description`)}, runtime{rust, platform_core}, origin, time{start, current},
    up_time (humanized). Java's JVM/memory/streams/personality blocks have no direct Rust
    analog — omitted rather than faked.
  - `/env` — **opt-in** lists only (`show.env.variables`, `show.application.properties`),
    so secrets are never dumped wholesale (Java parity).
  - `/health` — `mandatory.health.dependencies`/`optional.health.dependencies` routes,
    each called `type=info` (3 s, advisory — merged into the dependency entry) then
    `type=health` (10 s, decides); all-mandatory-up → UP/200, any-mandatory-down →
    DOWN/**400** (Java parity); no-deps → the "Did you forget…" hint; outcome stored for
    liveness.
  - `/livenessprobe` — `OK` text, or 400 "Unhealthy. Please check '/health' endpoint."
- **Default-endpoint merge** (Java `default-rest.yaml`): `/info` `/env` `/health`
  `/livenessprobe` are appended to the routing table only when `rest.yaml` doesn't claim
  the URL — user entries always win. Built via the same parser (same invariants).
- **Static HTML content** from **`resources/public`** (through the resource-roots
  convention): served when no rest.yaml route matches a GET/HEAD — `/` → `index.html`,
  directory paths → `<dir>/index.html`, parent traversal rejected, content type by
  extension (minimal `MimeTypeResolver` analog). A `/` entry in rest.yaml always wins.
- **Deferred:** `/info/lib` (maintainer-approved — Java reads the JAR manifest at runtime;
  a Rust binary has no runtime dependency manifest; a `build.rs`-embedded cargo metadata
  could provide it later), `/info/routes`, XML responses, etag/cache headers,
  `mime-types.yml` customization, the Java per-route info cache.

**Tests:** 10 end-to-end (info identity/uptime, env opt-in-only exposure, liveness default,
health no-deps hint, health UP with mandatory dep (info-merge asserted), health DOWN → 400 +
liveness flip, static index at `/`, nested asset content-type, traversal + miss + POST = 404,
rest.yaml endpoints still win) + elapsed-time unit tests.

## 5g. Increment 8 — static-content protocol: etag/304, no-cache pages, request filter

*(Implemented 2026-07-16, maintainer-directed; reference: the Java platform-core
`test/resources/rest.yaml` `static-content` block.)*

- **ETag / HTTP-304** (Java `EtagFile` + `sendStaticFile`): a quoted **SHA-256** content
  hash (new dep: `sha2` — std has no hash); `If-None-Match` compared **comma-list aware**
  → 304 with `content-length: 0`; else 200 + `ETag`.
- **No-cache pages** (`static-content.no-cache-pages`, default `["/", "/index.html"]`):
  served with `Cache-Control: no-cache, no-store` + `Pragma: no-cache` + epoch `Expires`
  instead of the etag protocol — entry pages must always revalidate (the SSO use case).
- **Request filter** (`static-content.filter`: `path` / `exclusion` / `service`): a
  composable function inspects matching static requests (patterns: exact, `prefix*`,
  `*suffix` — Java `matchedElement`; validation per `invalidFilterParameters`). The
  filter receives an `AsyncHttpRequest`-shaped event (10 s timeout, Java
  `FILTER_TIMEOUT`); its response **headers are always copied** onto the HTTP response;
  **status 200 → continue serving**, any other status passes the filter's response
  through — the SSO-redirection hook (302 + `Location`). Unregistered filter service →
  warn + serve normally (Java parity). Divergence (doc'd): a filter *call failure* logs
  and serves anyway (Java leaves the request to time out).
- **Path resolution** tightened to Java `getStaticFile` rules: trailing `/` →
  `index.html`, **extensionless filename → `.html`**, traversal rejected.
- Refactor: `envelope_payload`/`status_of` shared between normal dispatch and the filter
  pass-through.

**Tests:** etag cycle (200+ETag → 304 on match, comma-list, stale tag re-serves), no-cache
headers + no etag + If-None-Match ignored, filter inspect + header stamp, filter redirect
pass-through (static not served), exclusion bypass (`*.css`), unregistered-service
fallback, extensionless `.html` assumption. Example: `http.request.filter` interceptor
logging url/ip/user-agent (a real deployment would do SSO here).

## 5h. Increment 9 — lightweight RPC inbox + benchmark-reporter (milestone closure)

*(Implemented 2026-07-16, maintainer-directed: benchmark the foundation before layer 2.)*

- **Lightweight RPC inbox** (the Java **TemporaryInbox pattern**, precisely: Java registers
  ONE shared route `temporary.inbox@<origin>` (500 instances, EssentialServiceLoader) and
  correlates replies via `InboxBase.getHolder(cid)` with a sequenced correlation id that
  temporarily replaces the business cid — `InboxCorrelation` restores it): the Rust port
  keeps the **correlation-map half** (`src/inbox.rs`: unique `inbox.<uuid>` → oneshot) and
  simplifies the rest — correlation by the reply-to id itself, so the **business cid rides
  through untouched**; replies complete the oneshot at the delivery boundary (both the
  worker's automatic reply and a **manual `po.send(reply_to)`** — the `@EventInterceptor`
  pattern — resolve through `Platform::deliver`'s inbox check, keeping inbox ids uniformly
  addressable, which is what event-over-HTTP will need). The `cid-seq` composite machinery
  is only needed for fork-n-join multi-inboxes (§7); `@origin` qualification is a mesh
  concern (out of scope). No throwaway route per RPC. All prior tests pass unchanged.
- **`benchmark/benchmark-reporter`** (new workspace member) — the Java harness ported
  faithfully: the same six-scenario suite (RPC 1→C / C→C / paced callback = normal;
  RPC 2C→C / callback flood = overload; latency probe under background flood = mixed
  isolation), the same `Stats` (nearest-rank percentiles, log-spaced bins) and the same
  self-contained HTML report (inline SVG histogram + percentile plot), so records sit
  side-by-side with the Java `analysis/` snapshots. Parameters are `-D` runtime args
  (`bench.*`), Java-parity defaults. Known divergence (doc'd): paced scenarios ride
  tokio's ~1 ms timer vs Java's `parkNanos` — compare latency, not throughput, there.
- **The saved record** (`analysis/rust-tokio.html`, defaults, Apple Silicon 12-core,
  release build) vs the Java `file-vthread` record on the same machine class:
  RPC 1→50 **155K ops/s @ 6 µs mean (8.4×)**; RPC 50→50 **411K ops/s (2.3×)**; overload
  ~1.4× and **loss-free through the disk spill**; the mixed probe **17 µs mean / 210 µs
  max vs 157 µs / 1.62 ms (~9×)** — the no-GC tail story. 1,003,000 timed ops, 0 failures.
  Full comparison: `benchmark/benchmark-reporter/analysis/README.md`.

**This closes the platform-core milestone**: the foundation is configured, evented,
back-pressured, lifecycled, observable, HTTP-serving, operable — and now measured.

## 5i. Increment 10 — annotation macros + `AutoStart` one-liner + `examples/` convention

*(Implemented 2026-07-16, maintainer-directed: two enhancements before event-script.)*
Closes the D6 deferral — the ergonomic layer over explicit registration, so a user
application declares itself the way a Java mercury app does with annotations:

- **`crates/platform-macros`** (new proc-macro crate, re-exported by platform-core so apps
  never depend on it directly): `#[preload(route = "...", instances = N,
  env_instances = "config.key", typed)]`, `#[before_application(sequence = N)]`,
  `#[main_application(sequence = N)]` (default 10) — the Java `@PreLoad` /
  `@BeforeApplication` / `@MainApplication` analogs. `@ZeroTracing` is a **stacked marker**
  (`#[zero_tracing]` under `#[preload]`) or the `zero_tracing` flag. `typed` wraps the
  struct via `TypedAdapter::arc` (a `TypedFunction` impl); untyped structs implement
  `ComposableFunction`. Unit structs construct directly, anything else via `Default`.
- **Registration is link-time, not classpath-scan-time** (the D6 answer): each macro
  emits an `inventory::submit!` of a `registry::{Preload,BeforeApp,MainApp}Entry`
  (`&'static` data + a `fn() -> Arc<...>` factory); the `inventory` crate (0.3) collects
  them across every linked crate — so annotated functions in layer-2/3 library crates
  will register exactly like app-local ones, mirroring Java's cross-JAR scanning.
- **`AutoStart`** (Java `AutoStart.main(args)` parity): `AutoStart::main` = `-D` overrides
  → structured logging → collect the three inventories (with `env_instances` resolved
  through the config layer, override-aware) → `AppStarter` lifecycle → park on Ctrl-C
  while `rest.automation=true` → `shutdown_cleanup()`. That also ships the previously
  deferred **OS-signal shutdown wiring**. `AutoStart::run()` owns the tokio runtime, and
  the exported **`auto_start_main!()`** generates the whole `fn main()` — including
  prepending the *invoking crate's* `resources/` (compile-time `CARGO_MANIFEST_DIR`), so
  an app's configuration travels with the app. `Platform::register_with_options` +
  `AppStarter::preload_zero_traced` carry the zero-trace flag into the worker loop
  (route-name config `zero.tracing.filter` still works; the annotation is per-function).
- **`examples/` convention**: hello-world left `crates/platform-core/examples/` (a cargo
  example) and became the standalone **`examples/hello-world/`** workspace app crate —
  `src/main.rs` is the annotated functions plus the one-line
  `platform_core::auto_start_main!();`, with its `resources/` beside it. Event-script and
  knowledge-graph example apps will land as sibling `examples/<name>/` crates. The app's
  dependency list shrank to platform-core + serde/serde_json + async-trait + log — no
  tokio, no lifecycle plumbing.
- **Tests** (`tests/annotations.rs`): one end-to-end lifecycle (single test on purpose —
  the global platform's workers live on the first test's runtime) asserting hook
  ordering, all `#[preload]` routes registered, `env_instances` beating the literal
  count, typed RPC round-trip inside a trace bracket, and the stacked `#[zero_tracing]`
  marker suppressing the bracket. Live verification: `cargo run -p hello-world` + curl
  against REST, etag/304, no-cache + filter headers, `/info`, `/health`.

## 5j. Outbound HTTP client (`async.http.request`) — HTTPS + redirect semantics

**HTTPS (increment 48, 2026-07-20).** TLS via `tokio-rustls` (`ring` provider) +
`rustls-native-certs`: strict mode verifies against the **OS certificate store** (the
JDK-default-truststore analog the Java client uses); per-request `trust_all_cert` skips
chain validation only (handshake signatures still verified) — mirroring Java's
`InsecureTrustManagerFactory` escape hatch. `validate_url` accepts `https` (default 443);
TLS failures surface in-band (500). Tests: `tests/http_client_tls.rs` (local self-signed
server — trust-all 200 + strict rejection); field-validated end-to-end against the live
google.com CA chain by companion drive #3 (`docs/AI-companion-test.md`).

**Redirect semantics (decision record, 2026-07-20 — Rust redirect story WITHDRAWN).**
Neither engine follows redirects: Reactor-Netty supports `followRedirect` but
mercury-composable never enables it (the flag appears nowhere in the Java repo), so the
raw 3xx reaching the caller is **canonical shared behavior** — nothing to port. Verified
empirically with a temporary JUnit probe in the Java repo (`RedirectProbeTest`, modeled on
`HttpsTest`: `EventEmitter → async.http.request` against live `https://google.com`;
removed after evidence capture — a live-endpoint test is not CI material). Captured
stdout (surefire, `tests: 1, failures: 0, time: 6.005`):

```
=== REDIRECT PROBE ===
status   = 301
location = https://www.google.com/
body     = <HTML><HEAD><meta http-equiv="content-type" content="text/html;charset=utf-8"> <TITLE>301 Moved</TITLE></HEAD><BODY> <H1>301 Moved</H1> The document has moved <A HREF="https://www.google.com/">here</A>. </BODY></HTML>
body.len = 220
======================
```

The Rust port's result for the same call (companion drive #3) is byte-identical:
`{status: 301, page_size: 220}` with the same 220-character body. The documented contract
(AI grammar finding #61) — "the fetcher never follows redirects; point the Provider `url`
at the redirect target" — therefore holds for **both** engines.

**Design rationale (maintainer, 2026-07-20): this is deliberate, not an omission.** The
engines are designed for developers writing **backend applications**; automatic HTTP
redirection is a **browser-side automation**. Where a backend genuinely must handle a
redirect flow (e.g. SSO), it does so **programmatically at layer 1** (a composable
function reads the 3xx status + `location` header and issues the follow-up call) or
**declaratively at layers 2/3** (an Event Script flow / graph routes on `{node}.status`
and re-fetches — exactly the failure-routing/decision machinery the grammar already
documents). Years of the Java engine in production confirm that automatic
redirect-following is not normally required. Any future revisiting would be a new
shared-design feature for both engines in one increment — but the default stance is: the
raw 3xx **is** the correct answer for a backend client.

## 6. Out of scope (confirmed)

- **Kafka service mesh** (service discovery + sync-over-Kafka) — all of `connectors/`
  (enable-time decision). **Refined 2026-07-20:** `minimalist-kafka` and `twin-kafka` are
  *lightweight cloud-native connectors*, NOT part of the mesh exclusion — future-port
  backlog together with `sync-over-async` (see `memory/continuity.md` +
  `docs/background/port-scope.md`).
- **Spring adapters** (`rest-spring-3/-4`) — Spring is Java-only (maintainer, 2026-07-15).
  platform-core's own Vert.x-based REST automation (`automation/` package) **is** in scope,
  as a later increment (§8).

## 7. Deferred to later increments

Broadcast delivery · streams (`Flux`/`Mono` → Rust `Stream`) · kernel-thread analog
(`spawn_blocking` pool) · Event-over-HTTP · an OTLP forwarder extension (the
`distributed.trace.forwarder` hook is ready) · trace annotations on the envelope wire ·
full envelope fields · `yaml.preload.override` · `Utility` grab-bag (ported piecemeal as
callers need it) · crypto/caches · a lightweight dedicated RPC inbox (Java `AsyncInbox`
parity). Each becomes
its own Design increment tracing to `bp-platform-core`. *(Shipped: elastic overflow buffer — increment 3, §5b; lifecycle — §5c; telemetry — §5d;
REST automation — §5e; actuators/static — §5f; static-content protocol — §5g; RPC inbox +
benchmark — §5h; annotation macros + `AutoStart` one-liner + OS-signal shutdown — §5i;
event-interceptor mode + scheduled events (`send_later`/cancel) + rest.yaml `flow:` binding —
event-script design E-3, `event-script-port.md` §5c.)*

## 8. Open questions for the maintainer

1. **Profile env-var naming** — ~~keep `SPRING_PROFILES_ACTIVE` / `spring.profiles.active`
   verbatim, or rename with aliases?~~ **DECIDED (maintainer, 2026-07-19): renamed to
   `APP_PROFILES_ACTIVE` / `app.profiles.active`, no alias** — Spring is irrelevant to the
   Rust port, and the interim verbatim naming had served its side-by-side migration purpose
   (the original gate “once the foundation port is robust” is met).
2. **Resource-root defaults** — is `./resources` (runtime) + `tests/resources` (tests, shadowing)
   the right convention for apps built on the Rust port? *(Assumed yes — mirrors Java's
   main/test split.)*
3. **Route naming** — keep Java's dotted route names (`v1.get.profile`) verbatim? *(Assumed
   yes — routes are data.)*
4. **Edition/MSRV** — Rust 2021, latest stable toolchain? *(Assumed yes.)*
5. **`async-channel`** for the per-route MPMC queue (increment 2), or a tokio-only dispatcher
   fan-out? *(Recommend `async-channel`.)*
6. Anything to pull into / push out of increments 1–2?
