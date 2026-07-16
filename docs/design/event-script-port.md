# Design — event-script → Rust (layer-2 port)

> **Status:** DRAFT v1 for maintainer review · **Realizes:** `bp-event-script` (Blueprint) ·
> **Serves:** `vision-mercury` · **Author:** Claude Code · **Date:** 2026-07-16
> **Canonical source:** `mercury-composable` (Java, v4.8.6) — `system/event-script-engine`
> (~8.0K LOC / 69 main files; 90 `@Test` methods; 51 flow fixtures + 20 negative parser
> fixtures). Authoritative DSL spec: `docs/guides/event-script/flow-grammar.md` (which
> mirrors `CompileFlows` validation) + `flow-schema-reference.md`; tutorial: `syntax.md`.
> This is a *Design-altitude* artifact in the VBDI loop. **Approved 2026-07-16** (defaults accepted on all four open questions).

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
| E3 | **Flow state machine on the bus value tree (`rmpv::Value`)** with a runtime `MultiLevelMap` — **direct composite-key traversal is the PRIMARY data-mapping tool** (lightweight, zero conversion, byte-fidelity like Java's `Object` tree); **`$.…` is the user-defined complex query**, delegated to a real JSONPath crate (`serde_json_path`, RFC 9535) over an on-demand JSON view *(refined from serde_json::Value at the E-2 gate — maintainer decision 2026-07-16)* | Java's `MultiLevelMap.getElement` delegates `$`-prefixed paths to the Jayway JsonPath library — a genuine JSONPath engine, not a subset; matching it needs a real implementation. Envelope bodies (rmpv) already convert to/from JSON at the HTTP boundary; flow state is boundary data. |
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

## 5a. Increment E-1 — flow model + compiler (implemented 2026-07-16)

`crates/event-script` created: `model` (`Flow`/`Task`), `flows` (template registry),
`compiler` (full `CompileFlows` port — `yaml.flow.automation` discovery, grammar
validation, Java failure semantics: unreadable list = WARN+skip, invalid flow =
ERROR+skip, invalid mapping = drop the TASK but keep the flow), `converter` (legacy
`:type` → `f:plugin(...)` rewriting incl. negate / boolean value-match / concat /
substring forms), `validator` (mapping-syntax rules + the reserved-state-machine-key
guard, exported for the E-4 runtime re-check) and `plugins` (the plugin **name**
registry with the 42 built-in names — pulled forward from E-8 because `validInput`
checks `f:` names at compile time; execution bodies stay E-8). The engine
self-registers via `#[before_application(sequence = 5)]`.

**Fixtures:** all 90 Java flow files reused verbatim (55 in `flows.yaml` + 35 in
`more-flows.yaml`, incl. the intentional duplicate and a missing-file location).
Parity pinned by tests: the exact loaded-flow set, every whole-flow rejection, the
task-dropped-flow-loads semantics (parser-tests 23/25/26/27/28–31), normalized
mapping strings (3-part decomposition, negation, plugin rewrites — asserted against
the greetings fixture), loop/fork/sub-flow metadata. **Two findings where the code,
not the fixture comment, is authoritative** (verified against the Java source):
`invalid-condition-mode` (parser-test-7) is grammar-valid legacy naming and LOADS;
parser-test-19's `ext.user` (dot form) is a plain body key — only the `ext:`
namespace triggers the `external.state.machine` requirement — so it loads too.

## 5b. Increment E-2 — data-mapping engine (implemented 2026-07-16)

*Maintainer refinement at the gate: MultiLevelMap (direct composite-key access) is the
primary data-mapping tool; JSONPath serves user-defined complex queries — which is also
exactly how the Java code is layered (`MultiLevelMap.getElement` delegates only
`$`-prefixed paths to Jayway).* Modules:

- **`mlm`** — the runtime `MultiLevelMap` over `rmpv::Value` (the bus currency; byte
  arrays are real `Binary` values like Java's `byte[]`): get/set/remove/exists/
  key_exists with Java semantics (null resolves to None but the key exists; lists pad
  with nulls; removal never shifts indices; `key[]` appends via `appendIndex`); `$.…`
  → `serde_json_path` on an on-demand JSON view (0 matches → none, 1 → scalar, many →
  array — Jayway-style).
- **`mapping`** — the `DataMappingHelper` resolution half: `get_constant_value`
  (text/int/long/float/double/boolean, `map(k=v,…)` literals + `map(config.key)` from
  the base configuration, `file()`/`classpath()` content via `SimpleFileDescriptor`
  with text/json/binary/append modes), `get_lhs_element` (`$` queries, `f:plugin(...)`
  invocation with top-level-comma argument splitting and the nested-plugin null guard,
  legacy `:type` commands on model selectors), `get_value_by_type` (simple commands +
  `substring`/`concat`/`and`/`or`/`boolean(…)` — bad commands log ERROR and pass the
  value through, Java parity), and the `{model.key}` runtime interpolation
  (`substituteRuntimeVarsIfAny`: strings/numbers interpolate, anything else renders
  "null", non-model braces pass through).
- **`conversions`** — `TypeConversionUtils` port, incl. Java `String.valueOf` display
  parity (null → "null", floats keep the decimal point) and the Java numeric fallbacks
  (`str2int`/`str2long` → −1 with decimal-drop).
- **`plugins`** — executable bodies for the 19 core conversion/logical plugins the
  legacy-syntax converter emits (`f:int`, `f:not`, `f:isNull`, `f:concat`,
  `f:substring`, `f:eq`, …); the remaining built-ins stay name-only and fail loudly
  until E-8.

**Tests:** module unit tests for every resolution rule, plus an integration test that
takes the **compiled greetings fixture from E-1** and evaluates its normalized input
mappings against a simulated HTTP dataset — asserting the exact function-input body the
Java engine produces (type conversions, negation chains, uuid identity across the
3-part decomposition, `{model.pointer}` interpolation, `f:concat` with a `text(,)`
argument, header targets).

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
