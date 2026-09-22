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
| Service name | `hello-flow` (the `${OTLP_SERVICE_NAME:hello-flow}` default) |
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
  rejected 5 of 5), so the tenant holds seven `hello-flow` traces from this session, all accepted.
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

*Pending the maintainer's check in the Dynatrace non-prod UI.* What to look for, from the legs above:
service `hello-flow`, traces `0ad577c2bef646cba174147ebf923c01` (00:52:04Z) and
`092b2a1947e54f298d5ab945aa901331` (00:52:26Z), each with five spans nested as in Scenario 2, span
kinds `server` (the REST edge) and `internal`, the `route` / `path` / `status` / `exec_time_ms`
attributes, and the instrumentation scope `mercury-opentelemetry-forwarder` version `4.12.12`. Trace
`eedf60cd9bbe47618859a1a7b5967837` (leg B) must be **absent**.

## What remains

- The UI confirmation above, then field acceptance on the released `4.12.14` crate (scope version
  `4.12.14`), as the Java module did for its release build.
- Splunk Observability Cloud: the `X-SF-Token:` header form is parsed and documented but not run
  live.
- `otel.exporter.otlp.compression=gzip` is a declared delta (warns, exports uncompressed); a
  compression implementation is a follow-up only if a field installation needs it for volume.
