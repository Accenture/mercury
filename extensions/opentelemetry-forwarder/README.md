# OpenTelemetry trace forwarder

A ready-made `distributed.trace.forwarder` for Mercury — the Rust twin of the Java
`opentelemetry-forwarder` extension. It receives every completed span's performance-metrics
dataset from the engine's built-in `distributed.tracing` service and exports it to an OpenTelemetry
collector (or straight to Dynatrace, Splunk, Jaeger, Tempo, …) over **OTLP/HTTP**, preserving the
W3C trace / span / parent-span ids the engine already propagated across the Event Script, Knowledge
Graph and HTTP layers.

Because a forwarder reports spans that have *already happened*, this crate builds the OTLP span
directly from the dataset with the engine's exact ids — an OpenTelemetry `Tracer` would mint new
ones and break the lineage.

## Use it

Add the dependency and one link line to your application:

```toml
[dependencies]
mercury-opentelemetry-forwarder = "x.y.z"
```

```rust
// nothing in the application names the crate, so this line is what links it
// (collecting its annotation entries) - the Java "jar on the classpath"
use opentelemetry_forwarder as _;
```

> `x.y.z` denotes the current Mercury version shown in the root `Cargo.toml`.

That is the wiring. The forwarder is `#[preload(route = "distributed.trace.forwarder")]`, gated by
`#[optional_service("otel.forwarding")]` — **linking the crate turns nothing on**. Enable it per
environment with `otel.forwarding: true` in `application.yml`, or at launch without a rebuild:

```bash
cargo run -p hello-flow -- -Dotel.forwarding=true
```

Enable tracing on the endpoints and flows you care about (`tracing: true` in `rest.yaml`) and the
spans flow to your collector.

## Configuration (`application.yml`)

| Key | Default | Description |
|-----|---------|-------------|
| `otel.forwarding` | `false` | **Master switch.** The route is not registered at all unless this is `true` — linking the crate does not turn forwarding on. Set it per environment, or at launch with `-Dotel.forwarding=true`. |
| `otel.exporter.otlp.endpoint` | `http://localhost:4318/v1/traces` | OTLP/HTTP traces endpoint — the full URL including the signal path. |
| `otel.exporter.otlp.timeout` | `10000` | Per-export timeout in milliseconds. |
| `otel.exporter.otlp.headers` | — | Comma-separated `key=value` (or `key: value`) request headers — where backend credentials go. Re-read on every export. |
| `otel.service.name` | `application.name`, else `mercury` | `service.name` resource attribute on every span. |
| `otel.exporter.otlp.compression` | `none` | Accepted for parity with the Java module; only `none` is honoured on this engine (a `gzip` setting logs a warning and exports uncompressed — the payload is one span per request). |
| `otel.exporter.otlp.connect.timeout` | — | No effect on this engine (a warning says so): the platform HTTP client's `http.client.connection.timeout` governs the connect phase. |

Every value supports `${ENV_VAR:default}` substitution, so secrets and per-environment settings
stay out of the file and in the environment.

### Credentials (uploading directly to Dynatrace / Splunk / etc.)

When you export straight to a SaaS backend instead of a local collector, it needs an API token,
passed as a request header via `otel.exporter.otlp.headers`. **Source it from environment variables
with no default** so no secret is hard-coded — an unset variable resolves to nothing, which parses to
zero headers. Backends differ in the header *name* and in whether the value carries an auth scheme,
so the `hello-flow` example composes the header from a vendor-specific prefix and a bare token:

```yaml
otel.forwarding: false
otel.exporter.otlp.endpoint: '${OTLP_API_ENDPOINT}'
otel.exporter.otlp.headers: '${OTLP_AUTH_HEADER} ${OTLP_TOKEN}'
otel.service.name: '${OTLP_SERVICE_NAME:hello-flow}'
```

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

> **Naming caution:** referencing the OpenTelemetry standard variables (`OTEL_SERVICE_NAME`,
> `OTEL_EXPORTER_OTLP_ENDPOINT`, …) works, but those names are commonly exported machine-wide on
> instrumented hosts and CI agents, so an app can silently inherit a service name or an endpoint that
> redirects its telemetry. Prefer your own prefixed variables when that matters — this crate's tests
> deliberately avoid `${OTEL_*}` references for the same reason.

Each pair is split on the first `=` **or** `:`, whichever comes first — `=` is the OpenTelemetry
environment-variable convention, `:` is literal HTTP header syntax — so a token value may itself
contain either character. A value cannot contain a comma (the list is split on `,` first); header
values are never logged, only their names.

**Credentials are resolved per export, not once at start-up.** A runtime override published later
(`platform_core::overrides::set("otel.exporter.otlp.headers", …)` — the `-D` / `System.setProperty`
analog a credential bootstrap would use) takes effect on the very next export, with a single
`OTLP credential header resolved` line when it first appears. `${ENV_VAR}` references themselves are
resolved when the configuration loads, so a variable exported before launch behaves exactly as you
would expect. Guarded by `otlp_export::exporter_end_to_end` (scenario 5).

## What maps where

| Mercury trace metric | OpenTelemetry span |
|----------------------|--------------------|
| `id` (32-hex) | trace id |
| `span_id` (16-hex) | span id |
| `parent_span_id` (16-hex) | parent span id (root span when absent) |
| `service` (route name) | span name |
| `start` + `exec_time` | start / end timestamps |
| `success` / `status` / `exception` | span status (OK / ERROR + description) |
| `from` = `http.request` | span kind `SERVER` (else `INTERNAL`) |
| `path`, `from`, `origin`, `status`, `exec_time_ms`, `round_trip_ms`, `exception` | span attributes (same names) |
| `service` (route) | `route` attribute |
| `annotations` entries | `annotation.<key>` attributes |

The resource carries `service.name`; the instrumentation scope is `mercury-opentelemetry-forwarder`
with the running version (`info.app.version`, else this crate's version) — so a backend can tell
which engine's forwarder sent a span, and which build. Traces whose ids are not W3C-valid
(32 / 16 lowercase hex, not all zeros) are skipped rather than exported with forged ids.

## How it exports

The OTLP payload is one `ExportTraceServiceRequest` per span, written by this crate's own protobuf
encoder (`src/otlp.rs` — the OTLP v1 trace schema is frozen and needs eight message types) and sent
as `application/x-protobuf` through the platform's `async.http.request` client. No OpenTelemetry SDK,
no generated code, no second HTTP stack: the crate adds no dependency to an application that already
runs the engine.

**Retry.** Telemetry delivery is at-least-once by design — duplicates are tolerated, drops are what
hurt — so a transport failure (connection refused, TLS, a keep-alive killed mid-exchange, a timeout)
and the statuses 408, 429, 502, 503 and 504 are retried on the OpenTelemetry SDK's default bounded
backoff: 5 attempts, 1 s growing by 1.5× (1 s, 1.5 s, 2.25 s, 3.4 s). Any other status is final — a
401 will not get better by waiting. Two forwarder instances export concurrently; a slow backend queues
spans in the route's mailbox instead of dropping them.

**Diagnostics.** A rejected export is actionable from the forwarder's own warning line — the status
leads, the backend's response body follows (whitespace-collapsed, at most 256 characters), and the
usual rejections get a hint:

```text
OTLP export failed for span 9ff320a0b0187a10 of trace 2bbae37f1f3446f28cb0984e4032b5f0
- HTTP 401 - Token Authentication failed | the backend rejected the credential itself - check
  otel.exporter.otlp.headers (the header name and any auth scheme must match what the backend expects)
```

A 404 points at the signal path (`.../v1/traces`) missing from the endpoint; a 403 at a token that
authenticated but lacks the ingest scope (the body names it). Request headers are never rendered.

## Differences from the Java module

| Area | Java module | This crate |
|------|-------------|------------|
| Activation | the jar on the classpath (a base scan package) | one `use opentelemetry_forwarder as _;` line; the switch is the same `otel.forwarding` |
| Exporter | the OpenTelemetry SDK's `OtlpHttpSpanExporter` | this crate's protobuf encoder over the platform's `async.http.request` client |
| Compression | `gzip` or `none` | `none` (a `gzip` setting warns and exports uncompressed) |
| Connect timeout | `otel.exporter.otlp.connect.timeout` | the platform client's `http.client.connection.timeout` |
| Instrumentation scope | `org.platformlambda.opentelemetry-forwarder` | `mercury-opentelemetry-forwarder` |
| Late credential | a system property published after `@PreLoad` construction | a runtime override (`overrides::set`) published after start-up |
| Shutdown | the exporter is flushed from `Platform.onShutdown` | nothing to flush: each span is exported inline by its worker, so there is no buffered batch |

## Try it locally

The test sources include an **in-process OTLP collector double** (`tests/support/mock_collector.rs`)
that captures each POST and **decodes the OTLP protobuf** with the crate's own wire reader —
trace/span/parent ids, name, timing, status, attributes, resource and scope — so a reviewer can see
the exact payload. `tests/otlp_export.rs` drives the exporter against it (round trip, credential
header, retry, diagnostics, late credential) and `tests/trace_pipeline.rs` drives the real forwarder,
registered by configuration alone, with a traced three-function RPC chain.

## Notes

- Each dataset is exported individually; for very high trace volumes a batching layer in front of
  the exporter would reduce HTTP round-trips (future enhancement, as on the Java side).
- The forwarder is `#[zero_tracing]` (and its route is on the platform's zero-tracing filter), so it
  never traces itself.
- The forwarder exports **traces, not logs** — deliberately. Application logs reach your backend
  through your platform's log forwarder, not through the engine; the `trace_id` / `span_id` keys in
  every structured log line are what join them to these spans.
