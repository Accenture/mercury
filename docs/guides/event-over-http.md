# Event over HTTP

**Event over HTTP** lets a function in one application instance call a function in
*another* instance — the same route-name + `EventEnvelope` contract you use locally,
carried across the network. It is the only cross-instance coupling in the platform, and it
is **opt-in by design**: an instance is a closed world unless a developer deliberately
publishes a function to it.

Everything on this page describes this repository's engine
(`crates/platform-core/src/automation/event_api.rs`); the wire format is shared verbatim
with the Java engine (see [EventEnvelope wire format](event-envelope-reference.md)), so a
Rust instance and a Java instance interoperate without adaptation.

## The encapsulation boundary

Every function is reachable **inside** its instance — by REST automation, flows, and
graphs — but nothing crosses the instance boundary unless you expose it:

```rust
// private (the default) — in-instance only, exactly like Java @PreLoad
#[preload(route = "v1.internal.worker")]
struct InternalWorker;

// public — callable from another instance over /api/event
#[preload(route = "v1.public.api", is_private = false)]
struct PublicApi;
```

The programmatic pair is `platform.register_private(...)` (private) versus plain
`platform.register(...)` (public). Private is the default for `#[preload]` — the same
posture as Java `@PreLoad`, whose `isPrivate` defaults to `true`. A remote call to a
private function is rejected with **403**; engine internals (the actuators, the telemetry
sink, the no-op function, the async HTTP client, the event service itself) are all private,
so they can never be reached from outside.

## The endpoint

`POST /api/event` ships in the default `rest.yaml` — every application with
`rest.automation: true` exposes it with no configuration (your own `rest.yaml` entry for
that URL wins if you want to change its timeout, attach authentication, or add CORS).

| Request element | Meaning |
|---|---|
| Body | the serialized request `EventEnvelope` (standard wire format), `content-type: application/octet-stream` |
| `x-ttl` header | RPC timeout in milliseconds (minimum 1000) |
| `x-async: true` header | drop-n-forget: deliver and acknowledge, do not wait for a reply |

The service decodes the envelope, checks its `to` route, and dispatches:

| Outcome | Response |
|---|---|
| RPC to a public route | HTTP 200; body = the target's reply envelope (its own status inside) |
| Async to a public route | HTTP 200; body = a 202 acknowledgement envelope (`type: async, delivered: true`) |
| `to` missing | HTTP 400, "Missing routing path" |
| Route not found | HTTP 404, "Route {to} not found" |
| Route is private | HTTP 403, "{to} is private" |
| RPC timeout | HTTP 408 with the error message |

In every case the HTTP response body is itself a serialized envelope, so the caller reads
the outcome the same way regardless of success or failure.

## Calling another instance

The `event_over_http` client posts a serialized envelope to a peer and returns the reply
envelope (or, for an async call, the 202 acknowledgement):

```rust
use platform_core::automation::event_over_http;
use std::time::Duration;

let reply = event_over_http(
    &po,
    "http://peer-host:8100/api/event",
    EventEnvelope::new()
        .set_to("v1.public.api")
        .set_body(request_payload)?,
    Duration::from_secs(5),
    true, // rpc = true; false for drop-n-forget
).await?;
```

Trace context propagates automatically: the client sets `x-trace-id` and the W3C
`traceparent` header from the envelope's trace, so a trace started in one instance
continues in the other and the remote function's spans parent onto the caller's span — a
single distributed trace across the boundary (and across languages).

!!! note "Port note"
    The v1 service accepts the **standard** wire format only. A legacy Java *compact* envelope
    (single-character keys) is rejected with a clear 400; Java 4.10+ defaults to standard, so
    this only surfaces with an old or misconfigured peer. The Java engine's per-format response
    mirroring is therefore unnecessary here — replies are always standard.
