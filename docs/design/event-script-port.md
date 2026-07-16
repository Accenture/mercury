# Design — event-script → Rust (layer-2 port)

> **Status:** DRAFT v1 for maintainer review · **Realizes:** `bp-event-script` (Blueprint) ·
> **Serves:** `vision-mercury` · **Author:** Claude Code · **Date:** 2026-07-16
> **Canonical source:** `mercury-composable` (Java, v4.8.6) — `system/event-script-engine`
> (~8.0K LOC / 69 main files; 90 `@Test` methods; 51 flow fixtures + 20 negative parser
> fixtures). Authoritative DSL spec: `docs/guides/event-script/flow-grammar.md` (which
> mirrors `CompileFlows` validation) + `flow-schema-reference.md`; tutorial: `syntax.md`.
> This is a *Design-altitude* artifact in the VBDI loop. Implementation waits on approval.

## 1. Goal & scope

Port **Event Script** — layer 2, *composable orchestration*: a YAML DSL that describes an
end-to-end transaction as a **flow** choreographing composable functions by route name,
executed by a flow engine with a per-transaction **state machine** (`model`). Orchestration
moves out of code into configuration; functions stay the layer-1 building blocks. A flow
never references a function's implementation — only its route (the layer-1 invariant
`inv-never-couple-functions` carried up an altitude).

The engine rides *entirely* on the platform-core foundation we just measured: every engine
component is itself a composable function (registered with the increment-10 annotations),
and every task execution is an event over the bus.

## 2. Proposed decisions (maintainer gate)

| # | Decision | Rationale |
|---|---|---|
| E1 | **New crate `crates/event-script`**, depending only on `platform-core` | D5's workspace layout pays off; mirrors the Java module split (`event-script-engine` depends only on `platform-core`). |
| E2 | **Flow YAML syntax verbatim (D9 extended to the DSL)** — and **reuse the Java test fixtures unchanged** (51 flows + 20 negative parser fixtures + `flows.yaml`) | The flow files are config-data; a flow written for Java mercury must compile on Rust mercury unchanged. Reusing the canonical fixtures gives us behavior-parity tests for free and keeps the two repos side-by-side comparable. |
| E3 | **Flow state machine on `serde_json::Value`** + a JSON-tree composite-key engine; **`$.…` via a real JSONPath crate** (`serde_json_path`, RFC 9535) | Java's `MultiLevelMap.getElement` delegates `$`-prefixed paths to the Jayway JsonPath library — a genuine JSONPath engine, not a subset; matching it needs a real implementation. Envelope bodies (rmpv) already convert to/from JSON at the HTTP boundary; flow state is boundary data. |
| E4 | **Engine components register via the increment-10 annotations** | `event.script.manager`, `task.executor`, `http.flow.adapter`, `resilience.handler` are `#[preload]` functions; `CompileFlows` is `#[before_application(sequence = 5)]`; the plugin loader seq 3 — exactly the Java sequence contract (essential 0 < plugins 3 < flows 5 < user ≥ 6). Cross-crate `inventory` collection (built for this) makes the engine self-register when an app links `event-script`. |
| E5 | **Minimal platform-core extensions, shipped in lockstep**: (a) an **event-interceptor** registration mode (function receives the raw envelope incl. `reply_to`/`cid` metadata and replies manually — no auto-reply); (b) **scheduled events** (`send_later` + cancel) for the flow TTL watcher; (c) rest.yaml **`flow:` key** → REST automation injects the `x-flow-id` header (Java parity); (d) an envelope-body **deep-copy** helper | The engine's core classes are `@EventInterceptor` in Java; our worker currently always auto-replies. The TTL watcher is `sendLater`/`cancelFutureEvent`. All four are additive platform-core features other layers will reuse. |
| E6 | **Simple plugins are compiled Rust registered by a `#[simple_plugin]` annotation**; Java's runtime bytecode-safety allowlist is **not ported** (documented divergence) | Java scans `@SimplePlugin` classes at runtime and verifies their bytecode only touches `java.lang/util/math/time`. Rust has no runtime class loading — plugins are compiled, reviewed code linked into the binary; the allowlist's threat model doesn't exist. The registry + `f:name(args)` resolution semantics are ported faithfully. |
| E7 | **TTL watcher = an abortable tokio timer task** behind the `send_later` API | Same observable behavior (timeout → abort flow with 408, fire end-flow listeners, close instance); map-don't-mirror on the mechanism. |
| E8 | **`EventScriptMock` + task monitors ported** (a later increment, before milestone close) | The Java test suite leans on it; flow tests in downstream apps will too. |
| E9 | **Kotlin/reactor stream fields deferred** (`stream` in the HTTP dataset) with platform-core §7 streams | Same deferral as REST-automation streaming; the dataset key is carried as-is (null) so mappings referencing it don't break. |

## 3. Architecture mapping (Java → Rust)

| event-script-engine (Java) | event-script (Rust) | Notes |
|---|---|---|
| `CompileFlows` (`@BeforeApplication` seq 5) | `compiler.rs` — `#[before_application(sequence = 5)]` | reads `yaml.flow.automation` (default `classpath:/flows.yaml`): per file `location` prefix (default `classpath:/flows/`) + `flows:` list; full grammar validation at startup |
| `Flows` (static registries) | `flows.rs` — process-wide registries (`OnceLock` + `RwLock` maps) | flow templates + live flow instances |
| `Flow` / `Task` | `model.rs` structs | compiled task graph: input/output mapping lists, `next_steps`, `pipeline_steps`, `join_task`, `exception_task`, loop metadata (type/init/comparator/sequencer/conditions), `delay`/`delay_var`, monitors |
| `FlowInstance` | `instance.rs` | dataset `{input, model:{instance, cid, ttl, flow, trace, parent, root}}`; pipe map; task metrics; TTL watcher; `responded`/`running` state |
| `EventScriptManager` (`event.script.manager`) | interceptor `#[preload]` | launch event (header `flow_id`, body = input dataset, business `correlation_id` header) → new instance → first task |
| `TaskExecutor` (`task.executor`) | interceptor `#[preload]` — the heart | composite correlation id `uuid#seq`; input mapping → invoke function (reply→task.executor) or `flow://` sub-flow → output mapping → route by execution type; exceptions; per-task trace + metrics |
| `HttpToFlow` (`http.flow.adapter` ×200) | interceptor `#[preload(..., env_instances)]` | AsyncHttpRequest → input dataset (header/body/cookie/path_parameter/method/uri/query/ip/filename/session/ttl); `x-flow-id`; `X-Correlation-Id` (configurable) → business cid |
| `FlowExecutor` (programmatic) | `executor.rs` API | `launch(...)` fire-and-forget + `request(...)` RPC |
| `SimplePluginLoader` (`@BeforeApplication` seq 3) + `@SimplePlugin` | `#[simple_plugin]` + inventory registry | ~40 built-ins: arithmetic, generators (uuid/now/date), logical operators (incl. ternary/and/or), type conversions & list-of-map ops |
| `Resilience4Flow` (`resilience.handler` ×500) | `#[preload]` function | max_attempts, alternative routes, backoff / backoff_trigger / backoff_seconds, cumulative failure counting |
| `SimpleExceptionHandler` | ported with it | |
| `DataMappingHelper` (617 LOC) | `mapping.rs` | LHS/RHS mini-language (below) |
| Jayway JsonPath (via platform-core `MultiLevelMap`) | `serde_json_path` (E3) | `$.…` sources |
| `EventScriptMock` + task monitors | `mock.rs` (E8) | test-time route reassignment + before/after task monitors |

## 4. The DSL (what the compiler enforces)

`flow-grammar.md` is the deterministic spec; the Rust compiler must enforce the same
compile-time invariants (violations fail flow loading, never runtime):

- Flow: `flow.id`, `flow.description`, `flow.ttl` (≥ 1s), `first.task`, `tasks` all present;
  ≥ 1 `end` task; duplicate flow ids rejected; `ext:` targets require
  `external.state.machine`; task `name` required when the same `process` repeats.
- Eight execution types with `next`-shape rules: `sequential`/`response`/`pipeline`
  exactly 1; `decision` ≥ 2 (+ an output mapping to `decision`); `parallel` ≥ 2;
  `fork` ≥ 1 + `join`; `end`/`sink` none.
- Data mapping `'source -> target'` (3-part `LHS -> model.var -> RHS` compiles to two):
  LHS namespaces `input.*`, `model.*` (incl. `model.parent.*`/`model.root.*`, dynamic
  `{model.key}` interpolation, `.ITEM`/`.INDEX`), `error.*`, `$.…`, constants
  (`text/int/long/float/double/boolean/map/file/classpath`), `f:plugin(args)`, and — in
  `output` — `result[.x]`, `status`, `header[.x]`, `datatype`; type-conversion suffixes
  (`:text :int :long :float :double :boolean :binary :b64 :! :uuid :length
  :substring(a[,b]) :concat(…) :and(k) :or(k)`). RHS namespaces: function input body
  (bare) / `header.*` on `input` rules; `output.*`, `model.*`, `decision`,
  `file(path)`/`file(append:path)`, `ext:…` on `output` rules; `[]` appends.
- Reserved read-only model keys (`model.cid/instance/flow/ttl/trace/none` and the
  `parent`/`root` roots) must never be mapping targets.
- Pipeline `loop`: `statement: 'for (init; comparator; sequencer)'` (`++`/`--`) or
  `while (model.key)`; `condition: 'if (model.x) break/continue'`.

## 5. Increment plan

Bottom-up so each increment is independently testable; Java fixtures reused from the start.

1. **E-1 — Flow model + compiler.** `crates/event-script`: model structs, `flows.yaml`
   discovery (`yaml.flow.automation`), the full grammar validation, `#[before_application]`
   registration. Tests: all 51 Java flow fixtures compile with identical results; the 20
   `parser-flows` negatives fail with the right complaints. No runtime yet.
2. **E-2 — Data-mapping engine.** `mapping.rs` as pure functions over a JSON dataset:
   LHS/RHS resolution, constants, type suffixes, dynamic keys, JSONPath, `file()`/
   `classpath()` descriptors, the reserved-key guard. Unit-testable without the bus.
3. **E-3 — platform-core extensions (lockstep, additive).** Interceptor registration mode
   (annotation flag + worker no-auto-reply), `send_later`/cancel, rest.yaml `flow:` key →
   `x-flow-id` injection, body deep-copy. Ships as a platform-core increment with its own
   tests (INCREMENTS.md row); event-script waits on it.
4. **E-4 — Core runtime.** `Flows` registries, `FlowInstance`, `EventScriptManager`,
   `TaskExecutor` for `sequential` / `response` / `end` / `decision`, exception routing
   (task-level > `flow.exception`), TTL abort (408), per-task trace + metrics,
   `FlowExecutor` launch/request. First E2E fixtures: greetings, decision, response,
   timeout, exception.
5. **E-5 — Concurrency constructs.** `parallel`, `fork`/`join` (incl. dynamic `source`
   iteration with `.ITEM`/`.INDEX`), `sink`.
6. **E-6 — Pipelines & loops.** `pipeline` execution, `for`/`while`, `break`/`continue`.
7. **E-7 — Sub-flows & shared state.** `flow://` processes, `model.parent.*`/`model.root.*`,
   external state machine (`ext:`).
8. **E-8 — Simple plugins.** `#[simple_plugin]` macro + registry + the built-in set
   (arithmetic/generators/logical/types), `f:` resolution in mappings.
9. **E-9 — HTTP adapter, resilience, mock — milestone close.** `HttpToFlow`,
   `Resilience4Flow`, `SimpleExceptionHandler`, `EventScriptMock` + monitors, and a new
   **`examples/hello-flow`** app (the increment-10 `examples/<name>/` convention): a
   rest.yaml `flow:` endpoint running a real flow end-to-end.

Each increment: `cargo test` + clippy + fmt clean, INCREMENTS.md row + section, design-doc
increment note — the platform-core definition of done, unchanged.

## 6. Out of scope (confirmed defaults)

- **Kafka flow adapter** — the mesh is out of scope (enable-time decision).
- **Kotlin suspend variants** — Java/Kotlin-only concern; Rust is async end-to-end.
- **minigraph / knowledge-graph engine** — layer 3, its own design doc later.
- Reactor `Flux/Mono` stream payloads (E9 deferral, with platform-core §7 streams).

## 7. Open questions for the maintainer

1. **E2 fixture reuse** — copy the Java `flows/` + `parser-flows/` fixtures verbatim into
   `crates/event-script/tests/resources/` (attributing the source), or author a smaller
   Rust-native fixture set? *(Default assumption: verbatim reuse — parity for free.)*
2. **E3 JSONPath dependency** — `serde_json_path` (RFC 9535, pure Rust) acceptable as the
   Jayway analog? *(Alternative: defer `$.…` support to a later increment and fail such
   mappings at compile time until then.)*
3. **E5 scope** — comfortable with the four platform-core extensions landing as one
   platform-core increment (E-3) gated on its own tests?
4. **Plugin built-ins** — port all ~40 simple plugins in E-8, or the subset the reused
   fixtures actually exercise first? *(Default: fixture-driven subset first, rest to reach
   parity before milestone close.)*
