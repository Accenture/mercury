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
| `http.traceparent.header` | `traceparent` | The header carrying the full W3C trace context; REST automation inbound (standard `traceparent` first, custom name only when the standard is absent) and HTTP client / Event-over-HTTP outbound (stamped under **both** names) |

A `rest.yaml` endpoint entry may override all three per endpoint (`trace.id.header`,
`correlation.id.header`, `traceparent.header`); precedence is per-entry > global > built-in
default. Legacy conflation — pointing the trace-id and correlation-id names at one shared
header — is supported: the edge then resolves **one** id for both rather than minting two.

!!! warning "The standard W3C `traceparent` is our position — use it"
    It is the header OpenTelemetry and the wider observability ecosystem interoperate on,
    and the framework implements it as the default with zero configuration. The optional
    `traceparent.header` family exists for **backward compatibility with legacy systems
    only**; departure from the standard is discouraged, because a renamed carrier is
    invisible to OpenTelemetry SDKs, service meshes and APM agents, and every participant
    must be configured alike. Treat a custom name as a temporary bridge and plan the
    migration back to the standard header.

!!! tip "Within that constraint: a renamed traceparent beats conflation for the gateway case"
    The conflation above carries only the trace **id**, so spans in different applications
    can be stitched by id but not **parented** across the hop. Renaming the traceparent
    carrier (`http.traceparent.header=X-Trace-Context`) moves the *full* W3C context —
    trace-id, parent span-id and flags — through the gateway under an allow-listed name, so
    cross-application span parenting survives. Outbound calls stamp the same value under
    both the custom and the standard name; inbound, the **standard `traceparent` always
    wins** and the custom name is read only when the standard is absent — a well-formed
    standard traceparent means the caller already speaks W3C/OTel, so a residual
    proprietary header alongside it is safely ignored. The durable fix is always the
    gateway allow-list — retire the custom name once it lands.

!!! note "Rust port"
    The Java guide also documents `kafka.trace.id.header` / `kafka.correlation.id.header` /
    `kafka.traceparent.header` (plus the `secondary.kafka.*` twins) for its Kafka flow
    adapters. The Kafka service mesh is not ported, so those keys do not exist here — HTTP
    is the only protocol boundary.

## Turning tracing on

Tracing is opt-in per entry point:

- **HTTP endpoints** — set `tracing: true` on the `rest.yaml` entry. The edge starts the
  trace, with the trace path set to `METHOD /path`.
- **Programmatically** — stamp an event with `set_trace(trace_id, trace_path)` and every
  downstream hop is traced end to end (the `hello-world` main application does exactly
  this).

Two controls tune what is recorded: an RPC to a route listed in `skip.rpc.tracing`
(default `async.http.request`) produces no caller-side `round_trip` record — the HTTP
client's RPC leg folds into its caller — and the telemetry plumbing itself
(`distributed.tracing` and its forwarder routes) never traces its own executions. A
callback-mode execution of a listed route — the Event-over-HTTP stream relay's client
leg — still records its own span, parented onto the sender.

## Edge-started traces and W3C `traceparent`

At a traced endpoint the edge resolves the trace in strict order:

1. a well-formed **`traceparent`** header wins — the upstream trace continues, and the
   caller's span id becomes this hop's parent span (the standard name always wins; a
   custom `http.traceparent.header` name is read only when the standard is absent);
2. otherwise the trace-id header (`X-Trace-Id` by default) is adopted;
3. otherwise a new 32-hex trace id is generated.

Outbound, the async HTTP client injects `traceparent` (current trace id + current span)
alongside the configured trace-id and correlation-id headers on every call made inside a
trace — so a chain of mercury applications, or a mercury application behind any
OpenTelemetry-compliant caller, produces one continuous distributed trace.

## The edge's round-trip span

A traced endpoint records one more span than its functions: the **round trip** itself.
REST automation mints a span id when the request arrives, makes it the first function's
parent, and emits its record when the response completes — the buffered response, the
end of a streamed response, an edge error or the edge timeout. The record's `service` is
`http.request`, the same marker the first function carries as `from`, so the vocabulary
stays one word: *the request came from the edge; the edge's own span is the root*.

- `path` is `METHOD /path`, `start` is the receipt time and `exec_time` is the whole round
  trip — for a streamed response, until the terminal is rendered.
- `parent_span_id` is the inbound `traceparent` span when the caller sent one; otherwise the
  record is the trace's root.
- `status` is the HTTP status sent, except that a stream failing in-band after its head
  was committed reports the failure's own status and message.

An OpenTelemetry backend sees this record as the SERVER span of the request (see the
[forwarder](https://github.com/Accenture/mercury/tree/main/extensions/opentelemetry-forwarder)),
which is what makes a service's response time in a trace the real one — not the first
function's own execution time.

**Streamed responses are traced at their head and their tail, never per token.** The
`EventStreamWriter` stamps the producer's trace and span on the first segment (it carries
the head control) and on the terminal (`eof` or `exception`); the data segments in between
carry no trace, because one span per token would flood a tracing backend. The reply lane
that renders the stream therefore records two spans, both parented onto the producer, and
annotates the terminal's record with `frames` — the number of data segments it rendered.
The Event-over-HTTP stream relay follows the same rule on the consuming side: decoded
envelope frames keep the remote producer's span, synthesized control frames parent onto the
relay's client leg (`async.http.request`, itself parented onto the sender), and raw token
frames are forwarded untraced.

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

**Exactly one record per span.** For an **RPC-served** execution the worker suppresses
its own record when the reply reaches the caller, and the caller emits the span's single
record when the reply arrives — same shape, plus a `round_trip` metric (the full
request/response cycle in milliseconds; the reply envelope carries the same value) and
the callee's `annotations`, which ride the reply envelope. Its span lineage chains like
a worker-emitted record: `parent_span_id` is the caller's span, and `span_id` is the
callee's own span — adopted only from a **direct responder** (the reply's `from` equals
the requested route); a relayed reply (e.g. a flow answering on behalf of the
flow-adapter route) keeps the parent but omits `span_id`, because the relaying
function's span is reported by its own record. This works across an Event-over-HTTP hop
too, which is what stitches a remote function's record into the calling application's
span tree. **Callback**-style invocations (a `reply_to` that is a route, not an RPC
inbox — e.g. Event Script task dispatch) keep self-recording from the worker. For a route
listed in `skip.rpc.tracing` (default `async.http.request`) the caller emits no
`round_trip` record — the HTTP client's RPC leg folds into its caller (Java parity).

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

### The OpenTelemetry forwarder (ready-made) {#otel-forwarder}

You do not have to write the forwarder for OpenTelemetry. The
**`mercury-opentelemetry-forwarder`** crate (`extensions/opentelemetry-forwarder`, the twin of
the Java `opentelemetry-forwarder` extension) ships a `distributed.trace.forwarder` that maps
each dataset to an OpenTelemetry span — preserving the exact W3C trace, span and parent-span
ids — and exports it over **OTLP/HTTP** (binary protobuf) to a collector, and on to Dynatrace,
Splunk, Jaeger, Tempo, …. Add the dependency and one link line; no code:

```toml
[dependencies]
mercury-opentelemetry-forwarder = "x.y.z"
```

```rust
// nothing in the application names the crate - this line is what links it
use opentelemetry_forwarder as _;
```

> **Note**: `x.y.z` denotes the current Mercury version shown in the root `Cargo.toml`.

Configure it in `application.yml` (values support `${ENV_VAR:default}` substitution):

```yaml
# Master switch - DEFAULT OFF. Linking the crate registers nothing.
otel.forwarding: true
otel.exporter.otlp.endpoint: '${OTLP_API_ENDPOINT:http://localhost:4318/v1/traces}'
otel.service.name: '${OTLP_SERVICE_NAME:my-app}'
# Backend credentials come from the environment - no secret hard-coded:
otel.exporter.otlp.headers: '${OTLP_AUTH_HEADER} ${OTLP_TOKEN}'
```

> **Linking the crate does not turn forwarding on.** The forwarder is gated by
> `#[optional_service("otel.forwarding")]` with a default of **false**: linking the crate
> registers nothing, and the route exists only when otel.forwarding is true — in the
> environment's configuration, or `-Dotel.forwarding=true` at launch with no rebuild. That
> separates two decisions that belong to different people: a developer adds the dependency,
> and DevOps decides per environment whether traces leave the process. The `hello-flow`
> example carries the crate with the switch off and pins that shape with a test.

Point the endpoint at an OpenTelemetry Collector, or directly at a SaaS backend with its API
token in the headers. Backends differ in the header **name** and in whether the value carries an
auth scheme, so keep the vendor-specific part and the bare secret in separate variables:

```bash
# Dynatrace
export OTLP_API_ENDPOINT="https://{env-id}.live.dynatrace.com/api/v2/otlp/v1/traces"
export OTLP_AUTH_HEADER="Authorization: Api-Token"   # the header NAME and scheme - no token here
export OTLP_TOKEN="<your-api-token>"

# Splunk Observability Cloud
export OTLP_API_ENDPOINT="https://ingest.{realm}.signalfx.com/v2/trace/otlp"
export OTLP_AUTH_HEADER="X-SF-Token:"
export OTLP_TOKEN="<your-access-token>"
```

> **A caution on `OTEL_*` variable names.** Referencing the OpenTelemetry standard variables
> (`OTEL_SERVICE_NAME`, `OTEL_EXPORTER_OTLP_ENDPOINT`, …) is convenient and works, but those
> names are often exported machine-wide on instrumented hosts and CI agents — so an
> application can silently inherit a service name or, worse, an endpoint that redirects its
> telemetry somewhere unintended. When that matters, reference your own prefixed variables
> instead; `hello-flow` uses `OTLP_SERVICE_NAME` / `OTLP_API_ENDPOINT` / `OTLP_AUTH_HEADER` /
> `OTLP_TOKEN` for exactly this reason, and the forwarder's own tests avoid `${OTEL_*}`
> references so a leaked endpoint cannot redirect a hermetic test.

> **The credential is re-read on every export.** `otel.exporter.otlp.headers` is resolved per
> export, so a credential a bootstrap publishes after start-up as a runtime override
> (`platform_core::overrides::set`, the `-D` / `System.setProperty` analog) takes effect with
> no restart, and the log records a single `OTLP credential header resolved` line when it
> first appears. `${ENV_VAR}` references themselves are resolved when the configuration
> loads — the environment of a running process does not change underneath it.

Each span carries the route name (`route`), `path`, `from`, `origin`, `status`, timing, and your
`annotation.*` values as span attributes, with `service.name` on the resource and the
instrumentation scope `mercury-opentelemetry-forwarder` at the running version. A rejected
export is diagnosed on the forwarder's own warning line (the HTTP status, the backend's
explanation, and a hint for a missing signal path, a rejected credential or a token without
the ingest scope). Certified live against Dynatrace, with the A-B-A credential experiment that
proves the exports are real:
[Test Report — OpenTelemetry forwarder against Dynatrace](../test-reports/otel-dynatrace-certification.md).
Full key reference: [Configuration Reference](configuration-reference.md#otel-forwarding).

!!! note "Rust port"
    The Java module exports through the OpenTelemetry SDK; this crate writes the OTLP protobuf
    itself and sends it through the platform's `async.http.request` client, so it adds no
    dependency to an application. Three keys differ: `otel.exporter.otlp.compression` honours
    only `none` (a `gzip` setting warns and exports uncompressed — one span per request),
    `otel.exporter.otlp.connect.timeout` has no effect (the platform's
    `http.client.connection.timeout` governs the connect phase), and the instrumentation scope
    is named `mercury-opentelemetry-forwarder` so a backend can tell the engines apart. The
    retry policy (5 attempts, 1 s growing by 1.5× on transport failures and 408/429/502/503/504)
    and the failure diagnostics are the Java module's. A late credential arrives as a runtime
    override rather than a system property. See the crate README for the full table.

> **The forwarder exports traces, not logs** — deliberately. Application logs reach your
> backend through your platform's log forwarder, not through the engine; the `trace_id` /
> `span_id` keys of the [log context](#log-context) are what join them to these spans.

### A custom forwarder {#custom-forwarder}

To target a system without an OTLP path, implement your own function at
`distributed.trace.forwarder` and consume the dataset shown above — for example, write it to
a metrics database or a proprietary APM API.

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
context (`cid`, `trace_id`, `trace_path`, `span_id`, `parent_span_id`, `service`,
`timestamp`) on every structured log line — snake_case, matching the distributed-trace
block on the same record, so a collector maps one vocabulary. You can adjust it in two ways:

- **Customize** — provide your own `app-log-context.yaml` on the resource path
  (`resources/`); it replaces the built-in template entirely.
- **Opt out** — set `app.log.context: false` in the application configuration.

A custom template looks like this — from
`examples/hello-world/resources/app-log-context.yaml`, which extends the standard set
with two custom keys:

```yaml
context:
  cid: $cid
  trace_id: $traceId
  trace_path: $tracePath
  span_id: $spanId
  parent_span_id: $parentSpanId
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
`$service` (the current function's route), and `$utc` (the log line's UTC timestamp). The
token names stay camelCase; the *output* keys are yours — the default template writes them
in snake_case. A key that resolves to nothing is **omitted**, never printed as null — a root
span simply has no `parent_span_id` key. **The context block always carries a UTC
timestamp**: when your template maps `$utc` to no key, the engine adds it as `timestamp`
(as `utc` if `timestamp` is taken) — the record's top-level `time` is a local rendering,
and log-to-trace correlation resolves on a time window, so a line parsed in the wrong zone
would correlate and still be invisible on its trace. `update_context` refuses the reserved
names in **both** spellings (`traceId` and `trace_id`, …) with a 400, and a business key can
never shadow a template key — developer keys render first, the template wins. This is the log
line the greeting function emitted for the same request as the telemetry record above — same
`trace_id` and `span_id`, so the log line and the span join up in the backend, with the `user`
key added by `update_context`:

```json
{
  "context": {
    "cid": "39ecf9f00aec4f4cb5dd6b4362cda0b2",
    "environment": "dev",
    "hello": "world",
    "parent_span_id": "8bb32a631ca4beef",
    "service": "greeting.demo",
    "span_id": "bd75e3cb9cb462a1",
    "timestamp": "2026-07-20T01:33:54.763Z",
    "trace_id": "18529fd5dd4445ff8cd5bfe100f0ced3",
    "trace_path": "GET /api/greeting/eric",
    "user": "eric"
  },
  "level": "INFO",
  "message": "processing greeting request",
  "source": "hello_world(examples/hello-world/src/main.rs:96)",
  "time": "2026-07-20T01:33:54.763Z"
}
```

!!! note "Rust port"
    One deliberate divergence from the Java feature: **an invalid `$token` is
    advisory** — reported and skipped — where Java throws at load.

## Scope and boundaries

- The `context` block appears **only** on lines emitted inside a traced function
  execution with a real request trace (Java parity: the log context registers per worker
  execution in lockstep with the trace bracket). Framework boot logs, telemetry records,
  and work `tokio::spawn`ed from inside a function carry **no context block at all** —
  constants never leak onto context-less lines.
- Feature off (`app.log.context: false`) costs one boolean check per log line.
- The `text` format never renders a context block, whatever the configuration.

## See also

- [Architecture Overview](architecture.md) — the three layers the span tree mirrors.
- [Getting Started](getting-started.md) — run `hello-world` and watch these records live.
- [Function agent guide](event-driven/ai-agent-guide.md) — the PostOffice API surface.

*Adapted from the mercury-composable guide `observability.md`; behavior verified against this repository's source.*
