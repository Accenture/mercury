# EventEnvelope Reference

`EventEnvelope` is the immutable message container between composable functions — the only
thing that ever passes from one function to another. It is the Rust port of the Java
`org.platformlambda.core.models.EventEnvelope`, and it has the same three parts:

| Part | Contents |
|---|---|
| **metadata** | routing (`id`, `to`, `from`, `reply_to`), correlation (`cid`), tracing (`trace_id`, `trace_path`, `span_id`), `status`, `exec_time` |
| **headers** | `HashMap<String, String>` — parameters and metadata about the body |
| **body** | a dynamic `rmpv::Value` — the analog of Java's untyped `Object` payload |

Envelopes are built with fluent setters and read with getters; every accessor documented
here exists on the struct in `crates/platform-core/src/envelope.rs`.

```rust
use platform_core::EventEnvelope;

let event = EventEnvelope::new()
    .set_to("v1.get.profile")
    .set_header("type", "query")
    .set_body(ProfileRequest { id: 12 })?;
```

## Constructing and addressing

#### `EventEnvelope::new()`

| Returns | Notes |
|---|---|
| `EventEnvelope` | fresh envelope with a generated `id`, empty headers, `Nil` body |

Every new envelope gets a unique dash-less UUID as its `id`.

#### `set_to(route)` / `to()`

| Setter | Getter |
|---|---|
| `set_to(&str) -> Self` | `to() -> Option<&str>` |

The destination route name. `PostOffice::send` returns a 400 error when `to` is missing and
404 when the route is not registered.

#### `set_from(route)` / `from()`

| Setter | Getter |
|---|---|
| `set_from(&str) -> Self` | `from() -> Option<&str>` |

The sender's route. Inside a traced request the platform stamps this automatically on
outbound events (when unset), so replies and telemetry can attribute the caller.

#### `set_reply_to(route)` / `reply_to()`

| Setter | Getter |
|---|---|
| `set_reply_to(&str) -> Self` | `reply_to() -> Option<&str>` |

Where the function's return value is delivered. `PostOffice::request` sets this to a
temporary RPC inbox automatically; event interceptors read it to reply manually.

#### `set_correlation_id(cid)` / `correlation_id()`

| Setter | Getter |
|---|---|
| `set_correlation_id(&str) -> Self` | `correlation_id() -> Option<&str>` |

The correlation id. On the RPC path this is transport plumbing (the caller's id is kept, or
one is minted); the worker copies the request's correlation id onto the reply. The
**business** correlation-id of the current request is read through
`PostOffice::my_correlation_id` instead.

## Tracing metadata

#### `set_trace(trace_id, trace_path)` / `trace_id()` / `trace_path()`

| Setter | Getters |
|---|---|
| `set_trace(&str, &str) -> Self` | `trace_id() -> Option<&str>`, `trace_path() -> Option<&str>` |

The distributed-trace identity and path (e.g. `GET /api/greeting/{user}`). Normally the
platform manages these: the REST edge starts a trace when the endpoint declares
`tracing: true`, and every `PostOffice` send/RPC inside a traced function propagates them.

#### `set_span_id(span_id)` / `span_id()`

| Setter | Getter |
|---|---|
| `set_span_id(&str) -> Self` | `span_id() -> Option<&str>` |

The **sender's** span id, carried so the receiver knows its own parent span — this is how
parent-child span lineage chains across hops (and across HTTP boundaries via the W3C
`traceparent` header).

## Status and errors

#### `set_status(code)` / `status()` / `has_error()`

| Method | Behavior |
|---|---|
| `set_status(i32) -> Self` | set an HTTP-style status code |
| `status() -> i32` | unset means `200` |
| `has_error() -> bool` | `status() >= 400` |

Status codes are HTTP-style throughout the platform: **a status of 400 or higher is an
error**. A function returning `Err(AppError)` becomes an error reply whose status is the
`AppError`'s code and whose body is its message; a timed-out RPC yields status 408.

```rust
let reply = po.request(event, Duration::from_secs(5)).await?;
if reply.has_error() {
    log::warn!("call failed with status {}", reply.status());
}
```

## Headers

#### `set_header(key, value)` / `headers()` / `header(key)`

| Method | Behavior |
|---|---|
| `set_header(&str, &str) -> Self` | add or replace one header |
| `headers() -> &HashMap<String, String>` | all headers |
| `header(&str) -> Option<&str>` | one header |

Headers carry parameters and metadata *about* the body — both are delivered to the target
function (`handle_event` receives the headers map alongside the input).

## Body

#### `set_body(value)`

| Signature | Returns |
|---|---|
| `set_body<T: Serialize>(self, value: T)` | `Result<Self, AppError>` |

Serializes any `serde`-serializable value into the dynamic body — the analog of Java's
`setBody(Object)` PoJo transport. Fails with status 500 if the value cannot be represented.

#### `set_raw_body(value)`

| Signature | Returns |
|---|---|
| `set_raw_body(self, value: rmpv::Value)` | `Self` |

Sets the body from an already-dynamic value — no serialization step, no `Result`.

#### `body()` / `body_as::<T>()`

| Method | Behavior |
|---|---|
| `body() -> &rmpv::Value` | the raw dynamic body |
| `body_as<T: DeserializeOwned>() -> Result<T, AppError>` | deserialize into a concrete type (Java `getBody(Class)`) |

```rust
let request: serde_json::Value = input.body_as()?;      // free-form
let profile: Profile = reply.body_as()?;                // typed
```

## Performance metadata

#### `exec_time()`

| Signature | Returns |
|---|---|
| `exec_time(&self)` | `Option<f32>` (milliseconds) |

The function execution time, stamped by the worker on every reply and rounded to three
decimal places at the source — the reply envelope, the telemetry dataset, and the
Playground traveler all report the same value.

## Fields set automatically

Application code normally sets only `to`, `headers`, and `body` (plus `reply_to` for manual
replies). The platform manages the rest:

- **`PostOffice` (outbound, inside a traced request):** trace id/path, the sender's span
  id, `from` (when unset), and the business correlation-id (when the event carries none).
- **`PostOffice::request`:** `reply_to` (a temporary inbox) and the correlation id.
- **The worker (on every reply):** the request's correlation id, `from` (the executing
  route), `to` (the reply route), `exec_time`, and the trace triple so the next hop chains
  correctly.

## Serialization

The body is an `rmpv::Value` (MessagePack's dynamic value type), so a body is *already* in
wire shape in memory — typed values pass through `serde` on `set_body`/`body_as`.

- **In-process events are never serialized.** The in-memory bus moves the envelope
  directly; `to_bytes()`/`from_bytes()` (MsgPack) are used only on the **spill path** —
  when a route's elastic queue buffers a burst to disk.
- **JSON at HTTP boundaries.** REST automation renders a map/array body as
  `application/json`, a string body as `text/plain`, and a binary body as
  `application/octet-stream`; websocket frames and outbound HTTP client bodies are JSON as
  well.
- **Null omission — absent means null.** At every boundary — the wire (JSON and MsgPack
  alike) and every in-memory event-bus hop — null **map** key-values are omitted by
  default, so a consumer must treat an absent key and a null value as the same thing; the
  behavior is deterministic and identical to the Java engine on every delivery path. Set
  `serializer.null.transport: true` to transport nulls explicitly. Array elements are never dropped (ordering is preserved), and an empty
  `[]` or `{}` is a real value. See the
  [Configuration Reference](configuration-reference.md#serializernulltransport).

#### `to_bytes()` / `from_bytes(bytes)`

| Method | Behavior |
|---|---|
| `to_bytes() -> Result<Vec<u8>, AppError>` | encode the envelope as MsgPack |
| `EventEnvelope::from_bytes(&[u8]) -> Result<Self, AppError>` | decode an envelope |

!!! note "Rust port"
    The wire format is idiomatic serde MessagePack — deliberately **not** byte-compatible
    with Java's compact flag-keyed encoding, since cross-JVM interop is out of scope
    (single-runtime, in-memory bus only). Envelope features tied to unported subsystems do
    not exist here: `tags`, envelope-level `annotations` (trace annotations live on the
    trace context via `PostOffice::annotate_trace`), serialized exception/stack-trace
    transport (`getException`, `stack.trace.transport.size`), broadcast flags, and the
    `Kv` helper. Trace annotation and business-context APIs are on the `PostOffice` — see
    the [API Overview](api-overview.md).

---

*Adapted from the mercury-composable guide `docs/guides/event-envelope-reference.md`; keys/APIs enumerated from this repository's source.*
