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
| D6 | **Explicit registration** now; compile-time macro (`inventory`) later | Rust has no runtime annotation scanning (Java uses classgraph for `@PreLoad`). |
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
| `@PreLoad` classpath scan | explicit `register` (→ macro later) | D6 |

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

**Formats:** `.yml`/`.yaml` (YAML), `.json`, `.properties` (line-based `k=v`, sorted-key load).
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
- **Active profiles** (precedence, ported): env `SPRING_PROFILES_ACTIVE` → override-registry
  `spring.profiles.active` → consolidated config key `spring.profiles.active` (comma-separated
  list). Names kept verbatim per D9 — config compatibility, though Spring itself is not ported
  (§9 Q1 offers a rename option).

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
  (`SPRING_PROFILES_ACTIVE=test` merges `application-test.yml` on top), singleton identity,
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

## 6. Out of scope (confirmed)

- **Kafka service mesh** — `minimalist-kafka`, `twin-kafka`, all of `connectors/` (enable-time
  decision).
- **Spring adapters** (`rest-spring-3/-4`) — Spring is Java-only (maintainer, 2026-07-15).
  platform-core's own Vert.x-based REST automation (`automation/` package) **is** in scope,
  as a later increment (§8).

## 7. Deferred to later increments

Broadcast delivery · streams (`Flux`/`Mono` → Rust `Stream`) · kernel-thread analog
(`spawn_blocking` pool) · lifecycle (`AutoStart`/`AppStarter`, `@BeforeApplication`/
`@MainApplication` ordering) · `#[preload]` proc-macro registration · **REST automation** (HTTP
boundary, `rest.yaml` — no Spring) · Event-over-HTTP · distributed tracing propagation · full
envelope fields · `Utility` grab-bag (ported piecemeal as callers need it) · crypto/caches/
elastic overflow buffer. Each becomes its own Design increment tracing to `bp-platform-core`.

## 8. Open questions for the maintainer

1. **Profile env-var naming** — keep `SPRING_PROFILES_ACTIVE` / `spring.profiles.active`
   verbatim (D9 config-compatibility; Java keeps them for Spring compat), or rename (e.g.
   `MERCURY_PROFILES_ACTIVE`) with the Spring names as accepted aliases? *(Default assumption:
   keep verbatim + alias-free, matching Java exactly.)*
2. **Resource-root defaults** — is `./resources` (runtime) + `tests/resources` (tests, shadowing)
   the right convention for apps built on the Rust port? *(Assumed yes — mirrors Java's
   main/test split.)*
3. **Route naming** — keep Java's dotted route names (`v1.get.profile`) verbatim? *(Assumed
   yes — routes are data.)*
4. **Edition/MSRV** — Rust 2021, latest stable toolchain? *(Assumed yes.)*
5. **`async-channel`** for the per-route MPMC queue (increment 2), or a tokio-only dispatcher
   fan-out? *(Recommend `async-channel`.)*
6. Anything to pull into / push out of increments 1–2?
