---
title: Test Report — OpenTelemetry forwarder against Dynatrace
summary: Permanent record of the live certification run of the Rust `mercury-opentelemetry-forwarder`
  crate - the hello-flow example exporting real engine spans to a Dynatrace SaaS OTLP endpoint over
  the crate's own protobuf encoder and the platform HTTP client, with the A-B-A credential experiment
  that establishes what a clean run actually proves; the twin of the Java module's certification.
layer: reference
audience: [developer, architect, devops]
keywords: [opentelemetry, otlp, dynatrace, splunk, tracing, forwarder, certification, test report, rust]
---

# Test Report — OpenTelemetry forwarder against Dynatrace

*Live validation of the [OpenTelemetry forwarder](../guides/observability.md#otel-forwarder)
(`extensions/opentelemetry-forwarder`, crate `mercury-opentelemetry-forwarder`) against a
**Dynatrace SaaS OTLP endpoint**, conducted 2026-09-22 (UTC) on a single developer machine running
the `hello-flow` example as the subject application. It is the twin of the Java module's
certification (mercury-composable `docs/test-reports/otel-dynatrace-certification.md`, 2026-09-16)
and follows the same method: what was run, the evidence, and the observations the round produced.*

*No endpoint URL, token, or tenant identifier appears in this report or in any committed file. All
three come from the environment at launch; the committed configuration keeps the feature off.*

## The scenario under test

The engine emits its own distributed trace for every transaction. The forwarder turns each completed
span into OTLP and ships it to a vendor backend, so a Mercury application appears in the same tracing
tool as everything else the enterprise runs. This port does it **without the OpenTelemetry SDK** —
the OTLP protobuf is written by the crate itself and sent through the platform's `async.http.request`
client — so three things needed proving on real infrastructure rather than against the in-process
collector double the unit and integration suites use:

1. **The opt-in switch does what it claims.** Linking the crate must register nothing;
   `otel.forwarding=true` must be the only thing that turns the route on.
2. **A real vendor backend accepts the hand-written wire format** — the `ExportTraceServiceRequest`
   bytes, `application/x-protobuf`, HTTPS from the platform client, the endpoint path and the
   vendor-specific credential header, all together.
3. **The exports are real**, not a silent no-op — which zero failures alone cannot show.

| Element | Value |
|---------|-------|
| Subject application | `examples/hello-flow`, the `hello-flow` flow behind `GET /api/hello/{user}?lang=fr` |
| Backend | Dynatrace SaaS OTLP/HTTP traces endpoint (non-prod tenant), the `.../v1/traces` signal path |
| Service name | `hello-flow` (the `${OTLP_SERVICE_NAME:hello-flow}` default); the drive was then repeated under `mercury-otel-cert`, the Java certification's service name, for the UI check (Scenario 3) |
| Credential | `Authorization: Api-Token` + token, composed from `OTLP_AUTH_HEADER` and `OTLP_TOKEN` |
| Compression | none (this engine honours `none` only) |
| Instrumentation scope | `mercury-opentelemetry-forwarder`, version `4.12.12` (the workspace version of the build) |
| Runtime | rustc 1.98.1, cargo 1.98.1, macOS 26.7 |
| Build under test | branch `feat/opentelemetry-forwarder`, commit `f6e69af5` (debug profile) |

The application was launched three times from the same binary with the switch on as a runtime
override — the committed `application.yml` keeps it off:

```bash
target/debug/hello-flow -Dotel.forwarding=true -Dlog.format=text
curl -si 'http://127.0.0.1:8100/api/hello/otel-cert?lang=fr'
```

## Scenario 1 — the switch is the only thing that turns it on

`hello-flow` links the crate (`use opentelemetry_forwarder as _;`) and ships `otel.forwarding: false`.
The forwarder and its start-up hook are both `#[optional_service("otel.forwarding")]`, so with the
switch off neither runs and the route does not exist — the application behaves exactly as it did
before the dependency was added. This is pinned by a test in the example itself
(`examples/hello-flow/tests/otel_forwarding_switch.rs`), so the "dependency present, feature off"
property cannot regress silently.

Launched with the switch on, the hook validates the configuration, announces the forwarder and the
route appears:

```text
OpenTelemetry trace forwarder ready - service=hello-flow, OTLP endpoint=<redacted>,
compression=none, credential headers=["Authorization"]
```

Note what that line does **not** contain. It names the credential header but never its value, so
the start-up log of a production pod cannot leak the token. (The `OTLP credential header resolved -
["Authorization"]` line that precedes it is the one-time announcement the header supplier makes when
the credential first resolves — here at start-up, because the variables were exported before launch.)

## Scenario 2 — five spans, one trace, zero export failures

One transaction, HTTP 200. The engine's own trace contains five spans — the REST edge, the flow
engine's summary span, and the three tasks the flow ran — and the forwarder exported each one
individually as it completed:

```text
http.flow.adapter          9d1b702a12b83b91   server
├─ task.executor           a7cfd59289e31d87   internal   (from event.script.manager)
└─ language.router         aade817947c5678a   internal
   └─ greeting.composer    b552f03b28c643bf   internal
      └─ async.http.response  a35b2f11eb0def7c   internal

trace 0ad577c2bef646cba174147ebf923c01 — 2026-09-22T00:52:04.707Z
```

The tree above is drawn from the `parent_span_id` values the engine recorded in its telemetry
datasets — the same ids the forwarder placed in the OTLP `Span.parent_span_id` field. Whether the
backend reconstructs the same nesting is the UI check (Scenario 5): the backend is the authority on
parentage, and it is what proves the W3C context survived the wire.

Zero `OTLP export failed` lines. Because the forwarder exports one span per call *inline* in its
worker — the HTTP round trip is awaited before the next dataset is taken from the mailbox — there is
no batch window in which a failure could still be pending: every span's outcome is known, and logged,
by the time the transaction's spans have drained.

## Scenario 3 — the A-B-A credential experiment

**Zero failures is only evidence if a failure was possible.** A forwarder that silently skipped
export, never attached the credential, or sent bytes the backend discarded with a 2xx, would also
produce zero failures. So the same run was repeated with a deliberately invalid token, then restored:

| Leg | Credential | Export failures | App response | Trace |
|-----|-----------|-----------------|--------------|-------|
| **A** — clean | real token | **0** of 5 | HTTP 200 | `0ad577c2bef646cba174147ebf923c01` at 00:52:04Z |
| **B** — negative control | bogus token | **5** of 5 | HTTP 200 | `eedf60cd9bbe47618859a1a7b5967837` at 00:52:15Z |
| **A′** — restored | real token | **0** of 5 | HTTP 200 | `092b2a1947e54f298d5ab945aa901331` at 00:52:26Z |

Leg B's diagnostic, one per span:

```text
OTLP export failed for span a401f13a50dd04c7 of trace eedf60cd9bbe47618859a1a7b5967837
- HTTP 401 - Token Authentication failed | the backend rejected the credential itself - check
  otel.exporter.otlp.headers (the header name and any auth scheme must match what the backend expects)
```

Three conclusions follow, none of which the clean run alone could support:

- The forwarder **really exports** — five attempts, one per span, not a silent no-op — and the
  credential header **really reaches the backend**: it is what Dynatrace rejected.
- A 401 is **final**, not retried: five failures for five spans, one attempt each. Retrying a
  rejected credential would only have hidden the problem behind a delay.
- The bytes are **well-formed OTLP**: the backend authenticated the request before looking at the
  payload, and with the real token the same payload produced no rejection — a malformed body would
  have answered 400 in legs A and A′.

Legs A and A′ bracket B, so the zero in each is not a token that happened to be unset — the bracket
shows the credential was live before and after the failure was induced.

**Repeated under the Java certification's service name.** The maintainer's UI check looks for the
service the Java module certified under, `mercury-otel-cert` (its `OTLP_SERVICE_NAME`), while this
example's default is `hello-flow`. The same three legs were driven again with
`OTLP_SERVICE_NAME=mercury-otel-cert` exported for the run — same binary, same endpoint, the
service name being the only change — with the same outcome:

| Leg | Credential | Export failures | App response | Trace |
|-----|-----------|-----------------|--------------|-------|
| **A** — clean | real token | **0** of 5 | HTTP 200 | `f658763c71844a998d02521c033c5918` at 01:18:25Z |
| **B** — negative control | bogus token | **5** of 5 | HTTP 200 | `ababd406bfcf43c181ded9446d1c0198` at 01:18:36Z |
| **A′** — restored | real token | **0** of 5 | HTTP 200 | `b0ee5e2087ff495a9b8f13977620c758` at 01:18:47Z |

## Scenario 4 — failures that name their own cause

The Java certification met a dead end on its first live run: the SDK's HTTP failure exception
rendered as a bare class name while the actual cause (a 404 from a base URL without the signal path)
sat in a separate logger. This port inherited the lesson from the start: the forwarder's own warning
line leads with the status, follows with the backend's explanation (whitespace-collapsed, at most
256 characters) and adds a hint for the rejections that actually happen — a **404** points at the
signal path (`.../v1/traces`) missing from the endpoint, a **401** at the credential, a **403** at a
token that authenticated but lacks the ingest scope, which the body names. Request headers are never
rendered. The 401 above is the live instance; the 404, 403 and bounded-body cases are pinned against
the collector double in `tests/otlp_export.rs` and as unit tests in `src/export.rs`.

## Observations and round notes

- **The first drive measured nothing for two of its three legs, and said so only indirectly.** Each
  leg stopped its process with `SIGTERM`, waited one second and started the next; the engine's
  graceful shutdown takes longer than that, so legs B and A′ found port 8100 still held by leg A's
  process, their requests were served by it (real token, zero failures) and their own processes exited
  with `Unable to bind port 8100`. The tell was "datasets logged: 0" for those legs — the spans were in
  leg A's log. The redriven script waits for the port to change hands, checks that the listener is the
  new pid, and hard-kills after ten seconds; the same lesson the Redis outage drive taught — assert the
  hand-off, never assume the kill. Those three earlier requests reached Dynatrace with the real token
  (traces `20cf869c977745c3b03bdf62eafdb096`, `b21f2e68e6714802a01a51f5ac870e31`,
  `d62a0e41fef240339c909368103631b9` at 00:44Z, and `973bae01628f473c9156aa8fd03a538f`,
  `54ab25e716474febbaa19957e0ddacd6` from the second run at 00:47Z, with its own bogus-token leg
  rejected 5 of 5), so the tenant holds seven `hello-flow` traces from this session, all accepted —
  plus the two accepted traces of the `mercury-otel-cert` repeat.
- **Name the service the reviewer will look under before the drive.** The first drives used the
  example's default service name; the maintainer's UI check expected the Java certification's
  `mercury-otel-cert`. One environment variable and a 45-second re-drive fixed it, but the report's
  Scenario 5 had pointed the reviewer at the wrong service — agree the service name (or pass it
  explicitly) as part of the drive's set-up, exactly as the endpoint and the token are.
- **HTTPS from the platform client, first live use in this role.** The endpoint is TLS; the client's
  rustls stack with the OS trust store negotiated it without any configuration — the same path the
  schema registry client uses, now against a public SaaS certificate.
- **The trace id came from the engine's telemetry log, not from a response header.** The REST edge
  did not return an `X-Trace-Id` response header on this endpoint; the id is in every
  `distributed.tracing` dataset line, which is where the span trees above were read from.
- **One-shot announcements behaved.** The header supplier announced the resolved credential exactly
  once per process; the earlier double announcement seen during development came from the function
  building its own exporter before the start-up hook had installed the shared one — the exporter is
  now resolved on the first dataset, after the hooks have run.
- **The scope version is the build's.** `mercury-opentelemetry-forwarder` at `4.12.12` — the
  workspace version of this branch; the release cut re-sweeps it to `4.12.14`, so a span exported by
  the released crate will read that version in the backend, the same free artifact check the Java
  certification used to close its field acceptance.

## Scenario 5 — confirmed queryable in Dynatrace

**Confirmed by the maintainer in the Dynatrace non-prod UI, 2026-09-22 (Pacific 2026-09-21 evening),
from screenshots of both `mercury-otel-cert` traces of the repeated drive**,
`f658763c71844a998d02521c033c5918` (start 18:18:25 Pacific = 01:18:25Z) and
`b0ee5e2087ff495a9b8f13977620c758` (18:18:47 Pacific = 01:18:47Z):

- **Service** `mercury-otel-cert`, trace titled `'http.flow.adapter' Trace`, response time 116 µs and
  78 µs — the root span's `exec_time_ms` (0.116 / 0.078), which is what the forwarder sent as the
  span duration.
- **The nesting is as the engine recorded it**, reconstructed by the backend from the
  `parent_span_id` values on the wire: `http.flow.adapter` at the root, `task.executor` as its child,
  and `language.router` → `greeting.composer` → `async.http.response` as the other branch (the UI
  lists the two children of the root in either order). Five spans per trace, none missing, none
  orphaned.
- **Span kinds** `Internal` on every child (the root is the `server` span); **Status** `OK`.
- **Instrumentation scope** `mercury-opentelemetry-forwarder`, **scope version** `4.12.12` — the
  workspace version of the build under test, so the spans came from this branch's build and from this
  engine's forwarder, not from the Java module.
- **Attributes** exactly as mapped: on the root `path: GET /api/hello/otel-cert?lang=fr`,
  `from: http.request`, `route: http.flow.adapter`, `exec_time_ms`, `origin`, `status: 200`; on
  `task.executor` the flow engine's annotations arrived as `annotation.execution` (`Run 2 tasks in 0
  ms`), `annotation.flow` (`hello-flow`) and `annotation.tasks` (the task list with per-task `spent`),
  plus `from: event.script.manager`; the trace and span ids in the UI match the engine's telemetry
  datasets digit for digit.

The leg B trace `ababd406bfcf43c181ded9446d1c0198` is not in the tenant — the 401s were real
rejections, not deferred acceptances. With this the certification of the branch build is closed on
both sides of the wire: the forwarder's own log on the sending side, the vendor UI on the receiving
side.

## Scenario 6 — field acceptance on the published crate (2026-09-22)

The certification above ran the branch build (scope version 4.12.12). After the v4.12.14 release the
same drive was repeated from **the crates.io artifacts only**: a standalone consumer application built
outside the workspace with registry dependencies — `mercury-platform-core = "4.12.14"` and
`mercury-opentelemetry-forwarder = "4.12.14"`, no `path` into this repository (`cargo tree` shows both
resolved from the registry) — one traced endpoint `GET /api/accept/{id}` calling one worker function, so
each request is a three-span trace (`accept.api` SERVER → `accept.work`, `async.http.response`). It was
launched with `-Dotel.forwarding=true`, the endpoint and credential from the environment, service
`mercury-otel-cert`, and `info.app.version` deliberately unset so the instrumentation-scope version is
the published forwarder crate's own.

| Leg | Credential | Export failures | Traces (service `mercury-otel-cert`) |
|-----|-----------|-----------------|--------------------------------------|
| **A** — clean, two requests | real token | **0** of 6 | `580cbdc0f96441aa9afd9c6ed51bc4ce` (02:41:23Z), `5fd1cd3db9ba41a0a8b8b5de8e603b1a` (02:41:24Z) |
| **B** — negative control | bogus token | **3** of 3, `HTTP 401 - Token Authentication failed` | `9b25cb38cb0944f786d91e9ad2e56af0` (absent) |

What the UI should show for the two leg-A traces: three spans nested `accept.api` → `accept.work` and
`accept.api` → `async.http.response`, scope `mercury-opentelemetry-forwarder` **version 4.12.14**, the
`annotation.acceptance.id` and `annotation.acceptance` attributes. That version is the artifact check:
the spans came from the crate a field application downloads, not from a checkout.

## Scenario 7 — two engines, one trace (2026-09-22)

The maintainer's suggestion, and the strongest acceptance available: the minimalist-kafka interop of the
K5 gate re-run with the OpenTelemetry forwarder **on both engines**, so one request's spans arrive at the
backend from two processes and two forwarders. Setup: `kafka-standalone`, `redis-standalone` and
`schema-registry-standalone` 4.12.14 (the six demo topics, ten partitions each); the Java
`sync-over-async-demo` 4.12.14 rebuilt with the `opentelemetry-forwarder` dependency added for the
drive, and this repository's `sync-over-async-demo` with the crate linked for the drive (neither edit
committed — the examples ship without the forwarder); both launched with `-Dotel.forwarding=true`, the
same endpoint and credential from the environment, and distinct service names so the hop is visible:
**`mercury-otel-cert-java`** and **`mercury-otel-cert-rust`**. Each request is `POST /api/sync-to-async`
with a caller-set `traceparent`, so the trace id below is the caller's; the facade publishes the request
to Kafka, the backend on the *other* engine consumes it and publishes the reply, and the facade's
`sync.await` completes through the Redis return route. A fresh broker per pairing (see finding 1).

| Pairing | Facade | Backend | Traces | Spans exported (failures) |
|---------|--------|---------|--------|---------------------------|
| **A** | Java `:8500` | Rust | `f78de6d2d9a649d425acaec09a6bba53` (02:52:59Z), `72b2e692bac8478e1e2a9c148e3d1606` (02:53:01Z) | Java 18 (0), Rust 6 (0) |
| **B** | Rust `:8400` | Java | `ec6b3fc64b769c9f79c1f80d50371a2e` (02:53:36Z), `3481c84b80e6804849a2df2ea6967237` (02:53:37Z) | Rust 16 (0), Java 8 (0) |

Every trace crosses the engine boundary **twice**, and the parent ids on the wire say so — the Kafka
record's `traceparent` header carries the span context across the hop in both directions. For trace
`72b2e692…` (pairing A):

```text
mercury-otel-cert-java   http.flow.adapter b3c801d640de134e            server
├─ task.executor, sync.prepare 8808c62ce9d7da7a
│  └─ simple.kafka.notification b97f815f845b01e7      ── Kafka: soa.request ──▶
│     mercury-otel-cert-rust   task.executor / system.of.record 8d94bd61ca44a0a6   (parent b97f815f…)
│                              └─ simple.kafka.notification 8ddccc5536bf7cdb  ── Kafka: soa.response ──▶
│     mercury-otel-cert-java   task.executor / soa.reply a8024e25466d1cf1       (parent 8ddccc55…)
└─ sync.await 93730de9cb688bc9 → async.http.response ac59e2472382cb4f
```

and for trace `3481c84b…` (pairing B) the mirror image: the Rust facade's `simple.kafka.notification`
`a08a87cd3e5e34da` parents the Java backend's `system.of.record` `915af201c949fbc7`, and the Java
backend's `simple.kafka.notification` `82563cebebb18192` parents the Rust facade's `soa.reply`
`97813ba9f84bd050`. What the UI should show: **one trace, two services**, the Java spans under scope
`org.platformlambda.opentelemetry-forwarder` 4.12.14 and the Rust spans under
`mercury-opentelemetry-forwarder` 4.12.14. (The flow engine's `event.script.manager` records carry no
span id of their own and are skipped by both forwarders, as designed.)

### Findings of the round

1. **Java consumers do not leave their groups on SIGTERM; Rust consumers do.** The first attempt at
   pairing B answered 408 twice although the Java backend processed both requests within 40 ms and
   published both replies: the broker log shows the Java *facade* of the previous pairing, stopped with
   SIGTERM 4 s earlier, still a member of `soa-reply-group` holding all ten `soa.response` partitions —
   it was **fenced by session expiry 40 s later** — so the Rust facade that had just joined the same group
   owned nothing, and both replies landed on parked partitions. Every Java member in this drive went the
   same way (`Member … fenced from the group because the member session expired`), every Rust member
   `left the consumer group` on stop. On the Java side `KafkaFlowAdapter.close()` →
   `KafkaFlowConsumer.close()` (wake-up + `consumer.close()`, which sends LeaveGroup) exists, but nothing
   calls it at shutdown — `KafkaFlowAutoStart.start()` starts the adapter and registers no
   `Platform.onShutdown(adapter::close)`; `KafkaRequestPublisher.close()` (a `producer.close()`, which
   flushes) is in the same position. On Kubernetes that is a rolling restart parking the old pod's
   partitions for the KIP-848 session timeout. Recorded for the Java engine as an open thread; the fix is
   the platform shutdown lifecycle that already exists.
2. **The Java demo's error handler assumes the return-route coordinator is up.** In the first attempt at
   pairing A the first request arrived 100 ms before the facade's `Return-route subscriber listening`
   line; the flow aborted correctly, but `SyncErrorHandler` then called
   `SyncRuntime.coordinator().abort(cid)` on a null coordinator and the client got a 500 from the NPE
   instead of the flow's own error. A null check (or readiness gating) is the demo-level fix.
3. **Readiness is per component, and a stopped member is not a gone member.** The re-run waited for
   each app's own readiness lines (Java: `Assigned partitions` for every binding and the return-route
   subscriber; Rust: `Kafka flow adapter started`, `Return-route coordinator started` and a few seconds
   for the joins) and gave each pairing a fresh broker — after which all four requests answered 200 and
   all 48 spans exported. Same lesson as the earlier port-hand-off one: assert the hand-off, never assume
   the kill.

## What remains

- The maintainer's UI confirmation of Scenarios 6 and 7: the published-crate traces under
  `mercury-otel-cert` at scope version 4.12.14, and the four two-engine traces under
  `mercury-otel-cert-java` + `mercury-otel-cert-rust` joined into one trace each. That closes the
  forwarder's certification for 4.12.14.
- Splunk Observability Cloud: the `X-SF-Token:` header form is parsed and documented but not run
  live.
- `otel.exporter.otlp.compression=gzip` is a declared delta (warns, exports uncompressed); a
  compression implementation is a follow-up only if a field installation needs it for volume.
