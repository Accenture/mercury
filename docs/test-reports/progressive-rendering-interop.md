---
title: Interop Test Report — Progressive Rendering, all four runtimes
summary: Permanent record of the live cross-runtime validation of the progressive
  streaming contract - the multi-shot reply protocol and the Event-over-HTTP
  envelope-mode SSE dialect - across the Java and Rust engines and the Python and
  Node.js function hosts.
layer: reference
audience: [developer, architect]
keywords: [interop, streaming, sse, event over http, envelope mode, test report]
---

# Interop Test Report — Progressive Rendering, all four runtimes

*Live cross-runtime validation of the progressive streaming contract across the four
Mercury Composable runtimes — the Java engine
([mercury-composable](https://github.com/Accenture/mercury-composable)), the Rust engine
([mercury](https://github.com/Accenture/mercury)), and the Python
([mercury-python](https://github.com/Accenture/mercury-python)) and Node.js
([mercury-nodejs](https://github.com/Accenture/mercury-nodejs)) function hosts — conducted
2026-08-30 (UTC) at the close of the streaming program's wrapper round. This report is a
permanent record in the tradition of the
[Event over HTTP interop report](event-over-http-interop.md): what was tested, the
evidence, and the defects the round surfaced with their fixes.*

## The contract under test

One paradigm on all four runtimes: **the caller provides a reply address; the callee
streams events to it until a terminal signal.** Each segment is one event to the
caller's `reply_to`, marked with the reserved envelope header
`x-event-stream: data | eof | exception`. Across the Event-over-HTTP hop, the peer
answers the one POST with a Server-Sent Events response in the **hybrid envelope-mode
dialect**: envelope frames (the reserved SSE event name `envelope`, one base64-encoded
serialized envelope per frame) wherever envelope semantics matter — the head, the
terminals, and any segment that cannot round-trip as plain text — and raw SSE frames
for text tokens. The consuming client decodes the dialect and forwards each event to
the original reply address with the original correlation id, so a local stream and a
remote stream are indistinguishable to the consumer.

Delivered by: Java engine PRs
[#299](https://github.com/Accenture/mercury-composable/pull/299) (edge streaming),
[#300](https://github.com/Accenture/mercury-composable/pull/300) (SSE consumption),
[#301](https://github.com/Accenture/mercury-composable/pull/301) (envelope mode) —
ADR-0018/0019; Rust engine PRs
[#216](https://github.com/Accenture/mercury/pull/216),
[#217](https://github.com/Accenture/mercury/pull/217),
[#218](https://github.com/Accenture/mercury/pull/218) — ADR-0015/0016; and the
matching wrapper round in mercury-python and mercury-nodejs (interceptor functions,
`EventStreamWriter`, streaming `/api/event` host, `stream()`/`stream_to()` client).

## The live matrix

Ten producer/consumer combinations were driven live with the SHIPPED demo applications
— no test shims. Producers pace their segments (300 ms in these drives), so
progressive delivery is directly observable in the arrival timestamps; buffered
delivery would show all segments arriving together.

| # | Consumer | Producer | Result |
|---|----------|----------|--------|
| 1 | Node.js client | Python `hello.tokens` | segments at ~11/312/612/916 ms, eof metadata `{count, language: python}` |
| 2 | Python client | Node.js `hello.tokens` | segments at ~5/304/606/908 ms, eof metadata `{count, language: node.js}` |
| 3 | Python client | **Java engine** `hello.sse` | segments at ~103/407/708/1012 ms, terminal `eof` |
| 4 | Node.js client | **Java engine** `hello.sse` | segments at ~13/320/620/924 ms, terminal `eof` |
| 5 | Python client | **Rust engine** `hello.sse` | segments at ~3/306/607 ms, terminal `eof` |
| 6 | Node.js client | **Rust engine** `hello.sse` | segments at ~10/312/615 ms, terminal `eof` |
| 7 | **Java engine** edge (`/api/hello/remote`) | Python `hello.tokens` | SSE out the engine edge at ~195/493/793/1094 ms, terminal `event: done` with the Python eof metadata |
| 8 | **Java engine** edge | Node.js `hello.tokens` | ~217/516/817 ms, terminal metadata `{count, language: node.js}` |
| 9 | **Rust engine** edge | Python `hello.tokens` | ~20/323/624 ms, same shape |
| 10 | **Rust engine** edge | Node.js `hello.tokens` | ~19/320/622 ms, same shape |

Common observations across all ten:

- **Progressive on the wire** — arrival gaps match the producer's pacing; nothing
  buffers end-to-end (rows 7–10 traverse the full chain: HTTP edge → engine relay
  function → Event-over-HTTP → wrapper host → wrapper function and back).
- **Correlation** — the caller's correlation id is restored on every delivered
  envelope (D7), and the engines' demo authentication (`authorization: demo`) rides
  the per-target security headers unchanged.
- **Exact types** — eof trailing metadata arrives as a real map, not text, on every
  path (the envelope-frame escape hatch).
- **Explicit degradation, observed live** — a missing token produced the engine's
  401 and a private target its 403, both delivered as clean envelopes through the
  buffered fallback; a caller without the `accept: text/event-stream` opt-in receives
  the pinned `406 Streaming function requires a caller that accepts
  text/event-stream`.

## Engine ⇄ engine and per-runtime coverage

Because the protocol signatures are identical across the four runtimes, each
repository's unit suite exercises the full protocol against its own application
instance (client consuming its own `/api/event` host in one process): Java
`EventOverHttpStreamTest` (14 cases), Rust `event_over_http_stream.rs` (14), Python
`test_event_stream.py` (17), Node.js `event-stream.test.ts` (17). Each suite also
carries **misbehaving-peer fixtures** — a raw first frame, a transport end without a
decoded terminal, trailing frames after the terminal — because a self-loop alone
cannot catch a deviation implemented identically in both halves of one runtime; the
live matrix above is the cross-implementation conformance check, and it passed with
zero shims.

## Trace continuity (OpenTelemetry span / parent-span verification)

The drives were repeated with telemetry capture to verify distributed-trace
continuity across the streaming hop. Findings, with the observed evidence:

- **One trace id end to end, both directions.** In the engine→wrapper drive, the
  Java edge minted trace `f22af2e005f2445198563e2dc4f1ba54`; every engine span
  carried it (the relay function, the HTTP client leg, each reply-lane segment
  delivery), the Python function received it (trace id and path ride inside the
  wire envelope), and the demo now echoes it in the eof trailing metadata - so
  the terminal frame rendered out the engine's own edge carries the same trace
  id the edge minted: continuity is self-documenting in the demo output. In the
  wrapper→engine drive, a Python caller supplied W3C trace id
  `4bf92f3577b34da6a3ce929d0e0e4736` with trace path `PY /stream-drive`; the
  engine's `event.api.auth`, `event.api.service` and `hello.sse` spans all
  carried that id, and the function's span carried the caller's trace path.
- **Span parenting chains on the engines.** The streaming target's span parents
  onto the Event API service span - observed live:
  `event.api.service span_id=bfcddb32dd597ddb` →
  `hello.sse parent_span_id=bfcddb32dd597ddb`. Outbound, the engines' relay leg
  sends the W3C `traceparent` header carrying the sending function's span id
  (the trace-aware PostOffice stamps the span onto the outbound event), so a
  receiving ENGINE parents its spans onto the caller's - the same cross-engine
  parenting validated in the Event over HTTP interop report.
- **Wrapper executions are real spans** (closed at this round - the wrappers
  originally propagated the trace id but minted no spans, which broke the
  lineage into disconnected segments at every wrapper hop). The function hosts
  now implement the engines' exact span model: every traced execution mints a
  16-hex span with the caller's span (from the inbound envelope) as its
  parent; outbound calls and stream segments carry the current span onward;
  and non-RPC executions emit the engines' distributed-trace dataset record on
  the `distributed.tracing` log stream - the same
  `{"trace": {...}, "annotations": {...}}` shape the Java engine logs - so one
  log aggregation (or a stdout log-ingest agent forwarding to an observability
  dashboard) stitches the full span tree across all four runtimes. RPC
  round-trips emit no dataset, exactly like the engines: their metrics fold
  into the caller's view.
- **The connected tree, live-proven in all three directions.** Engine→wrapper:
  under one Java-edge trace, the relay function's span `8cbc0b4d4be75362`
  became the Python `hello.tokens` span's parent (`span_id=7b90847f4b8b617a`,
  `parent_span_id=8cbc0b4d4be75362` in the wrapper's own trace record), and
  the Java reply-lane delivery spans then parented onto the Python span -
  edge → engine function → wrapper function → engine deliveries, one unbroken
  chain. Wrapper→engine: a Python caller carrying external span
  `00f067aa0ba902b7` (the shape of a user-edge OpenTelemetry span) produced
  `event.api.auth` and `event.api.service` spans parented on it, with
  `hello.sse` parented on the service span. Wrapper⇄wrapper: a Node.js
  execution's record showed the Python caller's span as its parent. This is
  the lineage the AI SDLC requires - user → agent → MCP → tools in one tree.
- **One deliberate exception**: plain-text token segments ride raw SSE frames,
  which carry no envelope metadata (the zero-overhead token path), so their
  engine-side delivery spans join the trace unparented; a stream's head and
  terminal segments ride envelope frames and parent correctly. The engines'
  HTTP client leg (`async.http.request`) remains a sibling span - classic
  Event-over-HTTP parity.
- **Correlation id**: the caller's cid rode every delivered envelope in every
  drive (`cid-trace-drive` on all four segments of the supplied-trace drive).

The check itself surfaced defect #5: the engine demo relay originally built its
forward event with the raw EventEmitter, which stamps no trace - the hop
continued cid but DROPPED the trace id. Fixed by using a trace-aware
PostOffice (its `touch` fill-stamps from/trace/span), and the wrapper demos'
`hello.tokens` now echo their received trace id in the eof metadata so the
continuity is visible in every future drive.

## Business correlation-id continuity

The same verification was run for the business correlation-id (`my_cid`) - the
engine-managed envelope tag captured at an engine's HTTP edge from the
configured header (default `X-Correlation-Id`) and injected into every
receiving function's input headers as the read-only `my_correlation_id` view.
The wrapper demos' `hello.tokens` echo the view in the eof metadata alongside
the trace id, so every future drive self-documents both continuity dimensions.
Live results:

- **Java engine edge → Python**: `X-Correlation-Id: biz-e2w-001` on the edge
  request came back in the terminal metadata rendered out the same edge -
  edge header → `my_cid` tag → relay function's injected header view →
  trace-aware PostOffice re-stamp → packed envelope over the hop → wrapper
  host injection → function echo.
- **Rust engine edge → Python**: `biz-rust-005` echoed identically. The Rust
  demo relay needed no change: the Rust engine has one PostOffice and
  `apply_current_trace` always stamps trace and business correlation-id from
  the ambient context.
- **Python → Node.js and Node.js → Python** (wrapper ⇄ wrapper): a caller-side
  context (`trace_context(..., my_correlation_id=...)` / `runWithTrace({...,
  myCorrelationId})`) produced `biz-w2w-002` / `biz-w2w-003` echoes from the
  opposite wrapper - the tag crossed the hop and the receiving host injected
  the view.
- **Wrapper → engine**: the wrappers stamp the identical tag bytes (the same
  codec proven above), and the engines' pinned suites cover the receiving
  half: an `/api/event` caller carrying the `my_cid` tag reaches the target
  function as `po.getMyCorrelationId()` (Java
  `EventHttpTest.eventOverHttpPropagatesTraceAndCorrelationId`; Rust twin).

The check surfaced one parity gap in the (unreleased) wrapper round, fixed and
test-pinned in both wrappers: the wrapper clients inherited trace id/path and
the internal correlation id into outbound events but did not re-stamp the
business correlation-id as the `my_cid` tag, and local bus deliveries skipped
the header-view injection that the HTTP host performs. Both halves now mirror
the engines (`PostOffice.touch` / WorkerHandler parity): outbound events carry
the context's business correlation-id as the tag, local deliveries inject the
read-only view, and the trace context (`get_trace()` / `getTrace()`) exposes
it to handlers and relays.

By design (confirmed at this verification round): an engine's `/api/event`
ingress serves LOCAL routes only - it answers 404 for a route it does not
host, even when its own `yaml.event.over.http` map points that route at a
peer. The map is caller-side routing for the app's own outbound calls;
forwarding inbound calls onward would make every application an
Event-over-HTTP relay and open routing loops (the `x-event-api` wire marker
exists precisely to prevent such re-forwarding). Callers address the owning
peer directly; deliberate hop-through composition is an explicit relay
function - the demo `hello.remote.relay` is exactly that pattern.

## Defects surfaced by the round (fixed and re-verified)

Honest engineering record — each was caught by the twin-building discipline before
any release:

1. **Rust engine**: the envelope-mode single-shot reply initially reached the caller
   unwrapped (rendered as plain JSON instead of the classic packed-envelope wire);
   fixed by wrapping at the one `SingleShot` outcome site — caught by the twin suite's
   first run.
2. **Node.js host**: a `Promise.race` against a queue waiter abandoned the losing
   waiter, which would steal and drop the next envelope; fixed by reusing one pending
   promise across keep-alive cycles (`raceMs` documents the rule) — caught in design
   review before the first test run.
3. **Error-body contract**: the in-band exception bodies converged on the standard
   error key-values `type` / `status` / `message` across all four runtimes (three
   Java sites and their twins were missing the `type` key or used a plain-text body).
4. **Node.js**: object error bodies would have stringified as `[object Object]`;
   fixed with JSON rendering (`errorText`).
5. **Java demo relay**: the forward event was built with the raw (untraced)
   EventEmitter, silently dropping the distributed trace across the hop; fixed
   with a trace-aware PostOffice - found by the trace-continuity verification
   above.

## Reproduce

Every row of the matrix uses shipped demo applications:

- **Engine as producer**: run the Java `lambda-example` (port 8085) or Rust
  `hello-world` (8085; any port with `-Drest.server.port=…`); their public
  `hello.sse` streams paced test messages. Consume from a wrapper with
  `PostOffice.stream("hello.sse", …, endpoint="http://host:port/api/event")` and the
  demo `authorization: demo` header.
- **Engine as consumer**: run a wrapper demo app (python `mercury-serve
  examples/demo_app.py`, port 8086; node demo, 8087), then the engine demo with its
  routing map (Java: `-Dyaml.event.over.http=classpath:/event-over-http.yaml`; Rust:
  ships enabled) and `-Dpeer.demo.port` as needed, and watch
  `curl -N -H 'accept: text/event-stream'
  'http://127.0.0.1:8085/api/hello/remote?delay=300&count=3'` render the wrapper's
  tokens progressively out the engine's edge.
- **Wrapper ⇄ wrapper**: point either wrapper's `stream()` at the other demo's
  `/api/event`.

Baselines at the time of the drives: Java engine main after PR #301, Rust engine main
after PR #218, mercury-python and mercury-nodejs at the streaming feature round -
shipped together as the v4.12.0 milestone release across all four repositories.

## Appendix - telemetry and app context example (live capture)

One live request, captured end to end, showing how the telemetry stream and
the application log context connect the engine to the wrapper. Setup: the Java
`lambda-example` (port 8085) with its event-over-http map pointing
`hello.tokens` at the Python demo app (port 8086, `-Dlog.format=compact`).
The caller supplies a business correlation-id; the engine's HTTP edge mints
the trace:

```bash
curl -N -H 'accept: text/event-stream' -H 'X-Correlation-Id: biz-e2e-777' \
  'http://127.0.0.1:8085/api/hello/remote?delay=100&count=1'
```

**1. Java engine - the relay function's telemetry record** (the root span of
trace `710dcb0b706e4b5b949be138d0992b0b`, minted at the edge):

```json
{
  "level": "INFO",
  "time": "2026-08-29 21:24:17.734",
  "source": "org.platformlambda.core.services.Telemetry.handleEvent(Telemetry.java:81)",
  "thread": 762,
  "message": {
    "trace": {
      "path": "GET /api/hello/remote?delay=100&count=1",
      "span_id": "afa2897918e3871b",
      "service": "hello.remote.relay",
      "success": true,
      "origin": "20260830a0dcb550832b4683b4853c609778cd82",
      "start": "2026-08-30T04:24:17.733Z",
      "exec_time": 0.728,
      "from": "http.request",
      "id": "710dcb0b706e4b5b949be138d0992b0b",
      "status": 200
    }
  }
}
```

**2. The wire** - the relay's trace-aware PostOffice stamps the outbound event
with the trace id and path, its own span id (`afa2897918e3871b`) and the
`my_cid` tag (`biz-e2e-777`); the whole envelope crosses `POST /api/event`,
with `X-Trace-Id` and the W3C `traceparent` on the HTTP headers.

**3. Python wrapper - the function's own application log line.** The handler
runs `log.info("Streaming %d messages", count)`; the app-log-context feature
adds the `context` block. Note the join keys: the engine's trace id, the
business `cid` from the edge header, and this execution's `spanId` with the
relay's span as `parentSpanId`:

```json
{"time": "2026-08-29 21:24:17.847", "level": "INFO", "logger": "mercury_user_app:93", "message": "Streaming 1 messages", "context": {"cid": "biz-e2e-777", "traceId": "710dcb0b706e4b5b949be138d0992b0b", "tracePath": "GET /api/hello/remote?delay=100&count=1", "spanId": "55ebd6464ffdf9c2", "parentSpanId": "afa2897918e3871b", "service": "hello.tokens", "timestamp": "2026-08-30T04:24:17.847Z"}}
```

**4. Python wrapper - the telemetry record for the same execution** (same
`span_id` as the app log line's `spanId` - the join key between the two
streams; `from` names the calling engine function):

```json
{"time": "2026-08-29 21:24:17.949", "level": "INFO", "logger": "distributed.tracing:158", "message": {"trace": {"origin": "202608303cbae673831a4ab08cbb6f108fec0171", "id": "710dcb0b706e4b5b949be138d0992b0b", "path": "GET /api/hello/remote?delay=100&count=1", "service": "hello.tokens", "start": "2026-08-30T04:24:17.847Z", "success": true, "from": "hello.remote.relay", "exec_time": 101.673, "status": 200, "span_id": "55ebd6464ffdf9c2", "parent_span_id": "afa2897918e3871b"}}}
```

**5. Java engine - a reply-lane delivery record.** The wrapper's stream
segments carry its span, so the engine-side delivery span parents onto the
Python function (`parent_span_id = 55ebd6464ffdf9c2`):

```json
{
  "level": "INFO",
  "time": "2026-08-29 21:24:17.853",
  "source": "org.platformlambda.core.services.Telemetry.handleEvent(Telemetry.java:81)",
  "thread": 779,
  "message": {
    "trace": {
      "path": "GET /api/hello/remote?delay=100&count=1",
      "parent_span_id": "55ebd6464ffdf9c2",
      "span_id": "a50c19b7415406df",
      "service": "async.http.response.stream.499",
      "success": true,
      "origin": "20260830a0dcb550832b4683b4853c609778cd82",
      "start": "2026-08-30T04:24:17.851Z",
      "exec_time": 1.202,
      "from": "hello.tokens",
      "id": "710dcb0b706e4b5b949be138d0992b0b",
      "status": 200
    }
  }
}
```

**6. The caller's view** - the terminal SSE frame rendered back out the Java
edge echoes both continuity dimensions:

```text
event: done
data: {"trace_id":"710dcb0b706e4b5b949be138d0992b0b","count":1,"my_correlation_id":"biz-e2e-777","language":"python"}
```

**Connectivity, in one paragraph.** Every record above carries the one trace
id the engine minted at its HTTP edge, and the span ids chain across the
runtime boundary in both directions: the edge request → the relay function's
span (`afa289...`) → the Python execution's span (`55ebd6...`, parented on the
relay) → the engine's reply-lane delivery spans (parented on the Python span).
The business correlation-id from the `X-Correlation-Id` edge header rides the
`my_cid` envelope tag through every hop and surfaces as the `cid` in the
wrapper's app-log context and in the demo's terminal metadata. The wrapper's
application log line and its telemetry record share the same `span_id`, which
is what lets a log aggregation (or a stdout log-ingest agent) attach a
function's own log output to the exact span of the distributed trace - the
complete telemetry and app-context story from user to engine to wrapper and
back, assembled entirely from the four runtimes' stdout logs.
