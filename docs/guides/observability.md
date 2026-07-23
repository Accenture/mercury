# Observability Model

In an event-driven, composable system a single request fans out across **decoupled**
functions, flows, and graph nodes that never call each other directly. Observability is
therefore not optional — it is the only way to see the causal path a request actually took.
mercury answers this with a built-in distributed-tracing engine whose trace and span ids
follow the **W3C Trace Context / OpenTelemetry** format, a real-time telemetry stream, and
a default-on log context that joins application logs to their spans. Everything on this
page is verified against this repository's source and the running `hello-world` example.

## Two ids, two concerns

The platform keeps two identifiers deliberately separate:

`business correlation-id`
:   Identifies the *transaction* to the business. It is **always ensured at the HTTP
    edge, independent of tracing**: captured from the caller's `X-Correlation-Id` header
    (configurable), or generated when absent. It rides the envelope's `cid` field, is
    handed to the target function inside the request's `my_correlation_id` header, and —
    inside a traced execution — is available as `po.my_correlation_id()`.

`trace id`
:   Identifies the *causal path* to the platform: a 32-hex W3C-shaped id that drives the
    distributed tracing below. Trace ids exist only for traced requests.

Not every caller names its headers the way this framework does, so the header *names* are
configuration — the platform matches the impedance at the edge while everything downstream
works with the same two ids:

| Key | Default | Where it applies |
|:----|:--------|:-----------------|
| `http.trace.id.header` | `X-Trace-Id` | inbound (when no `traceparent`) and the async HTTP client outbound |
| `http.correlation.id.header` | `X-Correlation-Id` | edge capture inbound; async HTTP client outbound |

A `rest.yaml` endpoint entry may override both per endpoint (`trace.id.header`,
`correlation.id.header`); precedence is per-entry > global > built-in default. Legacy
conflation — pointing both names at one shared header — is supported: the edge then
resolves **one** id for both rather than minting two.

!!! note "Rust port"
    The Java guide also documents `kafka.trace.id.header` / `kafka.correlation.id.header`
    for its Kafka flow adapters. The Kafka service mesh is not ported, so those keys do
    not exist here — HTTP is the only protocol boundary.

## Turning tracing on

Tracing is opt-in per entry point:

- **HTTP endpoints** — set `tracing: true` on the `rest.yaml` entry. The edge starts the
  trace, with the trace path set to `METHOD /path`.
- **Programmatically** — stamp an event with `set_trace(trace_id, trace_path)` and every
  downstream hop is traced end to end (the `hello-world` main application does exactly
  this).

Two controls tune what is recorded: routes listed in `skip.rpc.tracing` (default
`async.http.request`) are excluded, and the telemetry plumbing itself
(`distributed.tracing` and its forwarder routes) never traces its own executions.

## Edge-started traces and W3C `traceparent`

At a traced endpoint the edge resolves the trace in strict order:

1. a well-formed **`traceparent`** header wins — the upstream trace continues, and the
   caller's span id becomes this hop's parent span;
2. otherwise the trace-id header (`X-Trace-Id` by default) is adopted;
3. otherwise a new 32-hex trace id is generated.

Outbound, the async HTTP client injects `traceparent` (current trace id + current span)
alongside the configured trace-id and correlation-id headers on every call made inside a
trace — so a chain of mercury applications, or a mercury application behind any
OpenTelemetry-compliant caller, produces one continuous distributed trace.

## A span per function execution

Every traced function execution mints its **own 16-hex span id**. The caller's span rides
the envelope's `span_id` field and becomes the callee's `parent_span_id` — producing a
causal span tree without any coupling between functions. In the `hello-world` example one
HTTP request yields a two-span tree: `greeting.api` (parented to the edge) →
`greeting.demo` (parented to `greeting.api`).

The layers above produce the same picture, because they ride the Layer-1 engine: every
Event Script task and every knowledge-graph node skill executes as a function invocation
and gets its own span in the same tree. Event Script adds one **synthetic flow-summary
span** (service `task.executor`) per flow, annotated with the flow id and a per-task
timing breakdown.

!!! note "Rust port"
    Java threads the span context through a per-worker registry, deliberately avoiding
    ThreadLocal/MDC (an anti-pattern on a virtual-thread runtime). The Rust analog is a
    **tokio `task_local!`** scoped around the function invocation. The boundary is the
    same in both engines: work you `tokio::spawn` from inside a function does not inherit
    the trace context, just as Java's post-return `Mono`/`Flux` completions do not.

## What a trace records

When a traced function finishes, its worker sends a performance-metrics dataset to the
built-in **`distributed.tracing`** service. The dataset carries a `trace` block and, when
present, your `annotations`:

`id`, `span_id`, `parent_span_id`
:   The W3C/OpenTelemetry-compatible ids (32-hex trace, 16-hex spans).
    `parent_span_id` is omitted on a root span — absent keys are never rendered as null.

`service`, `from`, `path`, `origin`
:   The executed route, the calling route, the trace path (e.g. `GET /api/greeting/eric`),
    and the application instance id.

`start`, `exec_time`
:   ISO-8601 UTC start time, and the function's execution time in milliseconds
    (rounded to 3 decimal places at the source, so every consumer reports the same value).

`status`, `success`, `exception`
:   HTTP-style status code, a boolean verdict, and — only on failure — the error message.

A traced **RPC** produces a second record from the caller's side when the reply arrives —
same shape, plus a `round_trip` metric (the full request/response cycle in milliseconds;
the reply envelope carries the same value). Its span lineage chains like the worker's
record: `span_id` is the callee's own span (carried on the reply) and `parent_span_id` is
the caller's span — including across an Event-over-HTTP hop, which is what stitches a
remote function's record into the calling application's span tree. Routes listed in
`skip.rpc.tracing` (default `async.http.request`) are excluded (Java parity).

By default the dataset is **logged** — that is the real-time telemetry stream. This is a
real record from a `hello-world` run (`log.format: json`), emitted by `distributed.tracing`
for the `greeting.demo` span of one HTTP request:

```json
{
  "context": {
    "environment": "dev",
    "hello": "world",
    "timestamp": "2026-07-20T01:33:54.763Z"
  },
  "level": "INFO",
  "message": {
    "annotations": {
      "greeting.for": "eric"
    },
    "trace": {
      "exec_time": 0.101,
      "from": "greeting.api",
      "id": "18529fd5dd4445ff8cd5bfe100f0ced3",
      "origin": "26f1710d906d481d9ccb6226ee05e89f",
      "parent_span_id": "8bb32a631ca4beef",
      "path": "GET /api/greeting/eric",
      "service": "greeting.demo",
      "span_id": "bd75e3cb9cb462a1",
      "start": "2026-07-20T01:33:54.763Z",
      "status": 200,
      "success": true
    }
  },
  "source": "platform_core::telemetry(crates/platform-core/src/telemetry.rs:113)",
  "time": "2026-07-20T01:33:54.763Z"
}
```

(The telemetry dataset embeds as a structured `message` object, not an escaped string;
JSON object keys render in alphabetical order.)

To ship the datasets elsewhere, register a function at the reserved route
**`distributed.trace.forwarder`** — the telemetry service detects it and forwards every
dataset to it. A companion hook, **`transaction.journal.recorder`**, receives
request/response journals when journaling is enabled (journals may contain PII — handle
per your organization's security policy).

!!! note "Rust port"
    Java ships a ready-made `opentelemetry-forwarder` extension that exports the datasets
    over OTLP/HTTP. That extension has not been ported yet — the extension point is
    identical, so an OTLP exporter is a `distributed.trace.forwarder` function you
    register yourself. The dataset preserves the exact W3C trace/span/parent ids, so the
    mapping to OpenTelemetry spans is direct.

## Trace annotations vs. log context

Two distinct sinks accept business context, and neither leaks into the other. From the
`hello-world` greeting function:

```rust
let po = PostOffice::new(&Platform::get_instance());
// business context for the APPLICATION LOG stream (context block)
po.update_context("user", &input.user)?;
// business context for the DISTRIBUTED-TRACE dataset (span annotation)
po.annotate_trace("greeting.for", &input.user);
log::info!("processing greeting request");
```

`annotate_trace(key, value)` attaches data to the **telemetry dataset** (the
`annotations` block in the record above). `update_context(key, value)` attaches data to
the **application log** stream only — it appears in the `context` block of every
subsequent structured log line of the same request; a `null` value removes the key, the
reserved keys are rejected with an error, and outside a trace both calls are no-ops.

## Log formats

The process logger has three output formats, selected by `log.format` in the application
configuration (default `text`) or at launch by the `-D` runtime override
(`cargo run -p hello-world -- -Dlog.format=compact`):

`text`
:   A plain console line — `time LEVEL [module] message`. Like Java's plain `Console`
    appender, it is unaffected by the log context.

`json`
:   Pretty-printed JSON, one record over multiple lines (the `log4j2-json.xml` analog).

`compact`
:   Single-line JSON records — jsonl, no CR/LF within a record (the `log4j2-compact.xml`
    analog).

Both JSON forms emit `time`, `level`, `source`, `message`, and the `context` block. The
log level comes from `log.level` (default `info`); a `RUST_LOG` environment variable wins.

!!! note "Rust port"
    Two deliberate simplifications over the log4j2 appenders: timestamps are always UTC,
    and there is no thread id field. Format selection is a configuration key plus the
    `-Dlog.format=` runtime argument (the JVM `-D` analog) instead of log4j2 XML files.

## The application log context

Spans tell you the causal path; application logs tell you what happened inside each step.
The log-context feature closes the gap: every structured log line carries a `context`
block — correlation id, trace/span ids, service name, and your own key-values — so you
can pivot from a span in your backend straight to the log lines that belong to it.

The feature is **on by default**: platform-core ships a built-in
`default-log-context.yaml` (embedded at compile time) that emits the standard trace
context (`cid`, `traceId`, `tracePath`, `spanId`, `parentSpanId`, `service`,
`timestamp`) on every structured log line. You can adjust it in two ways:

- **Customize** — provide your own `app-log-context.yaml` on the resource path
  (`resources/`); it replaces the built-in template entirely.
- **Opt out** — set `app.log.context: false` in the application configuration.

A custom template looks like this — from
`examples/hello-world/resources/app-log-context.yaml`, which extends the standard set
with two custom keys:

```yaml
context:
  cid: $cid
  traceId: $traceId
  tracePath: $tracePath
  spanId: $spanId
  parentSpanId: $parentSpanId
  service: $service
  timestamp: $utc
  environment: '${ENV_NAME:dev}'
  hello: world
```

The left side is the output key (your choice). The right side is one of three forms:

| Form | Example | Resolved |
|:-----|:--------|:---------|
| Reserved **`$token`** | `service: $service` | live, per log line, from the trace context |
| **`${ENV:default}`** substitution | `environment: '${ENV_NAME:dev}'` | once at load, from the environment |
| **Literal** | `hello: world` | emitted verbatim |

The reserved tokens are `$cid`, `$traceId`, `$tracePath`, `$spanId`, `$parentSpanId`,
`$service` (the current function's route), and `$utc` (the log line's UTC timestamp). A
key that resolves to nothing is **omitted**, never printed as null — a root span simply
has no `parentSpanId` key. This is the log line the greeting function emitted for the same
request as the telemetry record above — same `traceId` and `spanId`, so the log line and
the span join up in the backend, with the `user` key added by `update_context`:

```json
{
  "context": {
    "cid": "39ecf9f00aec4f4cb5dd6b4362cda0b2",
    "environment": "dev",
    "hello": "world",
    "parentSpanId": "8bb32a631ca4beef",
    "service": "greeting.demo",
    "spanId": "bd75e3cb9cb462a1",
    "timestamp": "2026-07-20T01:33:54.763Z",
    "traceId": "18529fd5dd4445ff8cd5bfe100f0ced3",
    "tracePath": "GET /api/greeting/eric",
    "user": "eric"
  },
  "level": "INFO",
  "message": "processing greeting request",
  "source": "hello_world(examples/hello-world/src/main.rs:96)",
  "time": "2026-07-20T01:33:54.763Z"
}
```

!!! note "Rust port"
    Two deliberate divergences from the Java feature. **Outside a trace, the block still
    renders its trace-independent keys** (constants and `$utc`) — in Java the block
    appears only on traced lines; here application-level lines (startup, framework
    events) carry the static context too. **An invalid `$token` is advisory** — reported
    and skipped — where Java throws at load.

## Scope and boundaries

- Trace-bound context keys appear only on lines emitted **inside** a traced function
  execution; framework boot logs and work `tokio::spawn`ed from inside a function carry
  only the trace-independent keys — the same boundary the distributed trace has.
- Feature off (`app.log.context: false`) costs one boolean check per log line.
- The `text` format never renders a context block, whatever the configuration.

## See also

- [Architecture Overview](architecture.md) — the three layers the span tree mirrors.
- [Getting Started](getting-started.md) — run `hello-world` and watch these records live.
- [Function agent guide](event-driven/ai-agent-guide.md) — the PostOffice API surface.

*Adapted from the mercury-composable guide `observability.md`; behavior verified against this repository's source.*
