# Macros Reference

The Rust port's attribute macros are the analog of the Java annotations (`@PreLoad`,
`@MainApplication`, `@BeforeApplication`, `@OptionalService`, `@WebSocketService`,
`@SimplePlugin`, `@FetchFeature`). Java discovers annotated classes by classpath scanning at
startup; Rust has no runtime scanning, so each macro registers its item in a **link-time
inventory** that the application lifecycle collects at startup — no manual wiring in `main()`.

The macros live in three crates and re-export through the layer they serve:

| Macro | Import from |
|---|---|
| `#[preload]`, `#[websocket_service]`, `#[main_application]`, `#[before_application]`, `#[optional_service]`, `auto_start_main!` | `platform_core` |
| `#[simple_plugin]` | `event_script` |
| `#[fetch_feature]` | `knowledge_graph` |

Every attribute-macro target must be a **unit struct or implement `Default`** (the Java
no-argument-constructor analog); `#[simple_plugin]` targets a function.

All code samples on this page come from this repository's shipped examples and integration
tests.

## `#[preload]`

Registers a composable function at startup and binds it to a route — the Java
`@PreLoad(route, instances, envInstances)` analog.

| Parameter | Meaning |
|---|---|
| `route = "..."` | **Required.** The function's route name (lowercase, at least one dot). |
| `instances = N` | Worker count, default `1`. |
| `env_instances = "config.key"` | Read the worker count from application configuration at startup; the literal `instances` is the fallback. |
| `typed` | The struct implements `TypedFunction<I, O>` instead of `ComposableFunction`. |
| `zero_tracing` | Flag form of the stacked `#[zero_tracing]` marker (below). |
| `interceptor` | Flag form of the stacked `#[event_interceptor]` marker (below). |

Without `typed`, the struct must implement `ComposableFunction` (raw `EventEnvelope` in and
out). With `typed`, it implements `TypedFunction<I, O>` with your own `serde` types and the
platform bridges it with an adapter:

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Ping {
    n: u64,
}

#[preload(route = "anno.typed.echo", instances = 4, typed)]
struct TypedEcho;

#[async_trait]
impl TypedFunction<Ping, Ping> for TypedEcho {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: Ping,
        _instance: usize,
    ) -> Result<Ping, AppError> {
        Ok(Ping { n: input.n + 1 })
    }
}
```

With `env_instances`, the instance count is configuration-driven — the literal loses to the
configured value when the key is set:

```rust
#[preload(
    route = "anno.untyped.echo",
    env_instances = "anno.pool.size",
    instances = 2
)]
struct UntypedEcho;
```

An invalid route name or a duplicate route fails the application at startup, so wiring
mistakes never reach runtime.

!!! note "Rust port"
    Java's `@PreLoad` classpath scanning becomes a link-time inventory collected by
    `auto_start_main!()`. The Java `yaml.preload.override` file (route/instance overrides
    without recompiling) is not ported — use `env_instances` for configuration-driven
    instance counts.

### Stacked markers

Three marker attributes stack **below** `#[preload]`, mirroring Java's annotation stacking;
`#[preload]` consumes them at expansion time:

`#[zero_tracing]`
:   The Java `@ZeroTracing`: this route's executions are excluded from distributed-trace
    recording.

`#[event_interceptor]`
:   The Java `@EventInterceptor`: the function receives the raw envelope (`reply_to` and
    correlation id intact) and replies manually via `po.send`; the worker sends no
    automatic reply on success. A failure still routes to `reply_to`.

`#[optional_service("condition")]`
:   Conditional registration — a first-class attribute in its own right (next section) that
    also works stacked above.

```rust
#[preload(route = "anno.zero.traced")]
#[zero_tracing]
struct QuietService;

#[preload(route = "anno.interceptor")]
#[event_interceptor]
struct ManualReplyService;
```

## `#[optional_service]`

The Java `@OptionalService("condition")` analog — makes a `#[preload]` function, a
`#[websocket_service]`, a `#[before_application]`, or a `#[main_application]` **conditional
on application configuration**: the item registers only when the condition holds at
startup. Using it without one of those four primary attributes is a compile error.

The condition is a comma-separated list evaluated with **OR** — the service registers if
any term holds:

| Term | Holds when |
|---|---|
| `key=value` | config `key` equals `value` (case-insensitive) |
| `key` or `key=` | config `key` equals `true` |
| `!term` | `term` does **not** hold |

It works in **either stacking order** — above the primary attribute (the Java order) or
below it as a marker:

```rust
#[optional_service("app.env=dev")]      // Java order — condition on top
#[preload(route = "dev.only.service")]
struct DevOnly;

#[preload(route = "also.dev.only")]     // marker order — condition below
#[optional_service("app.env=dev")]
struct AlsoDevOnly;
```

The knowledge-graph crate uses exactly this to gate the entire Playground developer surface
on `app.env=dev` — commands, traveler, websocket UI, mocks — while the graph engine itself
registers unconditionally. A skipped item is logged at startup
(`Skip optional <route> (condition: ...)`).

## `#[main_application]`

The Java `@MainApplication(sequence)` analog: declares the application entry point, run
after all functions are registered. The struct implements `EntryPoint`. A missing main
application is a startup error.

| Parameter | Meaning |
|---|---|
| `sequence = N` | Execution order when there is more than one (lower first, default `10`, clamped to `999`). |

```rust
use platform_core::{main_application, EntryPoint};

#[main_application]
struct MainApp;

#[async_trait]
impl EntryPoint for MainApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!("hello-world started");
        Ok(())
    }
}
```

## `#[before_application]`

The Java `@BeforeApplication(sequence)` analog: an `EntryPoint` that runs **before**
functions are registered — validation or compilation work. A failing hook **aborts
startup**.

| Parameter | Meaning |
|---|---|
| `sequence = N` | Execution order (lower first, default `10`, clamped to `999`; `0` is framework-reserved — user code uses 1–999, conventionally 3–999). |

```rust
use platform_core::{before_application, AppConfigReader};

#[before_application(sequence = 5)]
struct PreflightCheck;

#[async_trait]
impl EntryPoint for PreflightCheck {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let config = AppConfigReader::get_instance();
        if !config.exists("greeting.user") {
            return Err(AppError::new(
                500,
                "greeting.user missing from application.yml",
            ));
        }
        Ok(())
    }
}
```

The Event Script engine's flow compiler is itself a before-application hook at sequence 5,
and the Playground's housekeeping is one at sequence 8 (gated by
`#[optional_service("app.env=dev")]`).

## `#[websocket_service]`

The Java `@WebSocketService(value, namespace)` analog: registers a websocket server
endpoint declaratively. The annotated struct implements `ComposableFunction` and receives
the session lifecycle events (`type: open` / `string` / `bytes` / `close`) on its
per-connection `{session}.in` route; replies go to the `tx_path` given in the headers. One
function object is created per connection.

| Parameter | Meaning |
|---|---|
| `"name"` or `name = "..."` | **Required.** The service name — the URL becomes `/{namespace}/{name}/{token}`. |
| `namespace = "..."` | URL namespace, default `ws`. |

```rust
#[websocket_service("graph")]           // serves /ws/graph/{token}
#[optional_service("app.env=dev")]
pub struct GraphUserInterface;
```

The HTTP server starts when REST automation is enabled **or** at least one websocket
service is registered (Java parity).

## `#[simple_plugin]` (event-script)

The Java `@SimplePlugin` analog for the flow engine: registers a mapping-language plugin so
`f:name(...)` expressions validate at flow-compile time and resolve at runtime. The target
is a **function** with the signature `fn(&[rmpv::Value]) -> Result<rmpv::Value, String>`.
The plugin name defaults to the camelCase form of the function name; override with
`name = "..."`.

```rust
#[event_script::simple_plugin]
fn shout(args: &[rmpv::Value]) -> Result<rmpv::Value, String> {
    match args {
        [one] => Ok(rmpv::Value::from(
            event_script::conversions::get_text_value(one).to_uppercase(),
        )),
        _ => Err("shout expects one argument".to_string()),
    }
}
```

The plugin loader (a before-application hook at sequence 3) collects every entry before
flows compile, so a flow mapping `f:shout(model.word)` resolves to this function.

## `#[fetch_feature]` (knowledge-graph)

The Java `@FetchFeature(value)` analog: registers an API-fetcher feature for pre/post
processing of provider HTTP calls — the OAuth 2.0 bearer-token pattern is the canonical
use. The struct implements the `knowledge_graph::features::FeatureRunner` trait
(`run_before()` selects the request or response side; `execute()` does the work); a
provider node lists the feature by name in its `feature` property.

| Parameter | Meaning |
|---|---|
| `"name"` or `name = "..."` | **Required.** The feature name providers reference. |

```rust
#[knowledge_graph::fetch_feature("demo-auth")]
struct DemoAuth;

impl knowledge_graph::features::FeatureRunner for DemoAuth {
    fn run_before(&self) -> bool {
        true // mutate the outbound request (e.g. insert a bearer token)
    }

    fn execute(
        &self,
        request: Option<&mut platform_core::automation::AsyncHttpRequest>,
        response: Option<&knowledge_graph::features::HttpResponseView>,
        state: &mut event_script::mlm::MultiLevelMap,
        node_name: &str,
    ) {
        // ...
    }
}
```

The engine loads all declared features during startup, before any graph executes. Two
built-in demonstration features — `log-request-headers` and `log-response-headers` — are
registered by the engine itself.

## `auto_start_main!()`

The Java `AutoStart.main(args)` one-liner: generates the application's entire `fn main()`.

```rust
platform_core::auto_start_main!();
```

It expands **in the application crate**, so the application's own `resources/` folder (next
to its `Cargo.toml`) joins the resource roots at compile time. The generated `main()`
builds the tokio runtime, loads `-Dkey=value` launch overrides, installs structured
logging, collects every annotated item from the link-time inventory, runs the lifecycle in
the Java startup order (essential services → before-application hooks → preload → REST
automation → main applications), and — when the application serves HTTP or websockets —
stays alive until Ctrl-C.

Embedders (tests, an existing async context) call `AutoStart::main(args).await` directly,
which returns once the application is booted, or build the lifecycle explicitly with the
`AppStarter` builder.

!!! note "Rust port"
    The macro set covers every Java annotation that applies to this port. Not ported:
    `@KernelThreadRunner` (there is no kernel-thread pool — every function runs as a task
    on the tokio runtime, the virtual-thread analog) and `@CloudConnector` /
    `@CloudService` (the service mesh is out of scope). Java's standalone `@EventInterceptor`
    and `@ZeroTracing` annotations exist here as stacked markers consumed by `#[preload]`
    (see above), not as independent attributes.

---

*Adapted from the mercury-composable guide `docs/guides/annotations-reference.md`; keys/APIs enumerated from this repository's source.*
