# Function Execution

How the platform runs your functions: worker-instance pools, the messaging patterns, event
interceptors, and what distributed tracing records on every call. Everything on this page
describes the behavior of this repository's engine (`crates/platform-core`).

!!! note "Rust port"
    The Java original of this guide is largely about *choosing a thread-management strategy*
    — Java 21 virtual threads by default, `@KernelThreadRunner` for blocking legacy code, and
    reactive `Mono`/`Flux` return types. None of that carries over, because the Rust port has
    exactly one execution model: every function is an **`async fn` on the tokio runtime**.
    `.await` is the suspend/resume mechanism that virtual threads emulate, so there is no
    kernel-thread pool, no `kernel.thread.pool` parameter, and no strategy to pick.
    `Mono`/`Flux` return values, the `FluxConsumer` stream API, and `CustomSerializer` are
    likewise not ported — a function returns one `Result`, and `serde` handles typing.

## Worker-instance pools

When a route registers with `instances = N`, the platform spawns one **manager task** and
`N` **worker tasks** for it. Each worker announces readiness, takes **one event at a time**,
invokes your `handle_event` with its 1-based `instance` number, delivers the reply, and
announces readiness again. Point-to-point delivery therefore reaches exactly one worker, and
a single worker never processes two events concurrently — which is why a stateless function
needs no locking of its own.

When every worker is busy, the manager buffers incoming events in a per-route **elastic
queue** — in memory first, overflowing to temporary segment files on disk — and drains one
buffered event to each worker as it frees up, in arrival order. The manager's inbound mailbox
is bounded (`elastic.queue.dispatch.mailbox.size`, default 1024): when it fills, *senders
await* rather than events being dropped. Back-pressure is reactive end to end.

!!! note "Rust port"
    In the Java engine every bus message is already a MsgPack `byte[]`. The Rust port moves
    the `EventEnvelope` in memory for free and serializes it **only** when an event actually
    spills into the elastic queue — the on-disk record format stays byte-identical to the
    Java store, but the in-process fast path pays no serialization cost.

### Sizing the pool

`instances` is per-route capacity planning. A function that finishes quickly can serve many
concurrent callers with a small pool; a function that waits on external resources (databases,
HTTP calls) should reserve more workers, because each in-flight wait occupies one worker.
tokio tasks are cheap — the shipped applications use pools from 1 (health checks) through 10
(business functions) to 500 (the built-in async HTTP client) — but a bigger pool only helps
when callers actually run concurrently. `env_instances` lets you tune the number from
configuration without recompiling.

Keep functions **non-blocking**: never call a synchronous, thread-blocking API (e.g.
`std::thread::sleep` or a blocking I/O client) inside `handle_event` — it stalls a runtime
thread that other functions need. Use the async equivalents (`tokio::time::sleep`, the
platform's async HTTP client) and let `.await` do the waiting.

## Send, request, and scheduled events

The `PostOffice` is the messaging client. Three patterns cover orchestration by code:

### `send` — fire-and-forget

```rust
po.send(EventEnvelope::new().set_to("v1.audit.logger").set_body(record)?).await?;
```

Delivery to one worker of the target route. Errors are immediate and local: status 400 when
the envelope has no `to`, 404 when the route is not registered. A failure *inside* the target
function has nowhere to reply, so the worker logs it as a warning. If the route's mailbox is
full, `send` awaits — back-pressure propagates to the producer.

### `request` — RPC with a timeout

```rust
let reply = po
    .request(
        EventEnvelope::new().set_to("greeting.demo").set_body(request)?,
        Duration::from_secs(5),
    )
    .await?;
```

The platform opens a lightweight one-shot **inbox** for the reply (one correlation-map entry
— no route registration), stamps the request's correlation id, and suspends the calling task
until the reply lands. On expiry the call returns status **408**
(`Request timeout for N ms`). A function that returns `Err(AppError)` produces a reply
envelope carrying that status and message, so failures arrive **in-band**: check
`reply.has_error()` / `reply.status()` (any status ≥ 400 is an error). Every reply is also
stamped with the target's execution time, readable as `reply.exec_time()` in milliseconds.

### `send_later` — scheduled send

```rust
let timer_id = po.send_later(event, Duration::from_secs(30));
// ... and if the work completes early:
po.cancel_future_event(&timer_id);
```

Schedules a one-time future delivery and returns a timer id; `cancel_future_event` cancels a
still-pending timer and reports whether it was in time. (The Event Script engine uses this
for flow TTL watchdogs.) Unlike `send` and `request`, a scheduled event fires after the
current request context is gone, so it carries only the trace fields you set on it
explicitly.

!!! note "Rust port"
    The Java `PostOffice` also offers **broadcast** (deliver to every instance) and
    fork-n-join **parallel RPC** (one call, many targets). Neither is ported: broadcast
    belongs to the out-of-scope service mesh, and parallel execution is expressed at the
    orchestration layer instead (an [Event Script](../event-script/index.md) `fork`/`join` —
    or plain `tokio::join!` over two `request` futures when orchestrating by code).

## Event interceptors

A function registered with the `interceptor` flag (or the stacked `#[event_interceptor]`
marker) opts out of the automatic reply. It receives the **raw envelope** with `reply_to` and
the correlation id intact, and answers *manually* — typically later, from a callback or
another task — by sending to `reply_to` itself:

```rust
#[preload(route = "v1.deferred.responder", interceptor)]
struct DeferredResponder;
```

On success the worker sends no auto-reply (your function's return value is ignored); a
**failure** still routes to `reply_to`, so a caller's `request` is never left hanging on an
interceptor crash. The platform's own boundary functions work this way — the async HTTP
client and the Event Script flow adapter are both interceptors, replying only when the real
response is ready.

## Tracing behavior per call type

REST automation starts a **distributed trace** at the HTTP edge for every endpoint declared
with `tracing: true`; a trace can also be started by hand with
`set_trace(trace_id, trace_path)` on the first event. From there, propagation is automatic
for every `send` and `request` issued *inside* a traced function:

- the outbound event carries the current **trace id and path**, the sender's **span id**
  (which becomes the receiver's parent span), the **sender route** (`from`), and the business
  **correlation id** when the event doesn't set its own;
- each traced execution runs in its own **span**, and the reply carries the trace context
  back so the chain continues;
- when the execution finishes, the worker emits a **performance-metrics dataset** to the
  `distributed.tracing` sink — trace and span ids, service, timing (rounded to three decimal
  places), status, success flag, and exception text on failure — logged as an
  OpenTelemetry-compatible record.

An event that arrives *without* a trace simply executes untraced: no span, no telemetry
record. Two per-request annotation channels are available inside any traced function:

```rust
po.annotate_trace("order.total", total);   // -> the trace dataset (telemetry record)
po.update_context("user", &user_id)?;      // -> the application-log context block
```

`my_trace_id()`, `my_trace_path()`, and `my_correlation_id()` read the current context, e.g.
to echo the trace id in an API response.

Three mechanisms exclude a route from tracing: the `zero_tracing` registration flag (or the
stacked `#[zero_tracing]` marker), the `skip.rpc.tracing` configuration list (default:
`async.http.request`, so HTTP-client plumbing doesn't flood the trace), and the platform's
own telemetry routes, which never trace themselves.

## See also

- [Write your first function](write-your-first-function.md) — the authoring walkthrough this
  page assumes.
- [AI agent guide](ai-agent-guide.md) — the engine-verified registration and trait contracts,
  in full.
- [Composable Orchestration](../event-script/index.md) — move the orchestration itself out of
  code and into a flow.

---

*Adapted from the mercury-composable guide `docs/guides/event-driven/function-execution.md`; behavior verified against this repository's source.*
