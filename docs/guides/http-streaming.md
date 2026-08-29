# HTTP Response Streaming

*How a function streams an HTTP response progressively - token by token, event by event.*

> **At a glance**
>
> - **What** — a callee sends a *sequence* of events to the caller's reply route until an
>   end-of-transmission signal; the HTTP edge renders each segment as it arrives.
> - **For** — LLM token streams, agent progress events, and live updates of a running
>   transaction. The wire is standard HTTP: Server-Sent Events (`text/event-stream`) or
>   chunked transfer with JSON lines.

Everything on this page describes this repository's engine
(`crates/platform-core/src/automation/server.rs` and
`crates/platform-core/src/event_stream.rs`); the envelope protocol, the rest.yaml
surface, and the wire framing are engine-identical with the Java engine, so a flow or a
client moves between the two engines without adaptation.

## The model

Streaming is native to the event system: a caller provides a `reply_to` address, and the
callee may send it as many events as it likes. For a regular request the return route is
single-shot - the first response event completes the HTTP exchange. A *streaming*
response makes it multi-shot: each event carries one segment, and a final signal declares
end of transmission.

Each segment event carries the reserved envelope header:

```text
x-event-stream: data | eof | exception
```

This marker is internal protocol between the callee and the HTTP edge - it never appears
on the wire. The HTTP client sees only standard HTTP: chunked transfer encoding, and
Server-Sent Events framing when the content type is `text/event-stream`.

## Declare a streaming endpoint

Add `stream: true` to the endpoint definition in rest.yaml:

```yaml
  - service: "v1.token.producer"
    methods: ['POST']
    url: "/api/chat"
    timeout: 60s
    stream: true
```

The flag makes the request check out a *dedicated ordered reply lane* - a
single-instance route drawn from a pool of 500 (matching the `async.http.response`
concurrency). All segments of the request ride its own lane, so they render in the exact
order the function sent them, while different requests stream concurrently through their
own lanes. The lane returns to the pool when the request completes; when all 500 lanes
are busy, further streaming requests are rejected immediately with HTTP-503
(`Streaming response pool exhausted`) - deterministic back-pressure instead of silent
queuing. An idle lane costs only a little memory and no CPU. The endpoint may still
answer single-shot - a response without the `x-event-stream` marker behaves exactly as
before.

The endpoint's other declarations compose normally: `cors` headers and the optional
`headers` response transform (add/keep/drop) apply to the streamed response head
exactly as they do to a single-shot response.

## Produce a stream

A streaming producer is an interceptor function - it receives the raw event envelope
(including `reply_to` and correlation id) and replies by sending events itself:

```rust
#[preload(route = "v1.token.producer", instances = 50, interceptor)]
struct TokenProducer;

#[async_trait]
impl ComposableFunction for TokenProducer {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let mut out = EventStreamWriter::from_request(&Platform::get_instance(), &input)?;
        out.first(200, "text/event-stream");
        out.write("Hello").await?;                                    // one segment
        out.write_named("tokens", serde_json::json!({"n": 2})).await?; // a named SSE event
        out.close_with(serde_json::json!({"usage": 42})).await?;     // end of transmission
        Ok(EventEnvelope::new())
    }
}
```

`EventStreamWriter` is thin sugar over plain event sends:

| Method | Meaning |
| ------ | ------- |
| `first(status, content_type)` | optional head control, carried by the first outgoing event |
| `first_with_ttl(status, content_type, ttl_seconds)` | as above, plus an idle-allowance override between segments |
| `write(segment)` | one `data` segment - text, bytes or a map |
| `write_named(event_name, segment)` | a named segment - the name maps to the SSE `event:` field |
| `close()` / `close_with(metadata)` | end of transmission - metadata rides the terminal SSE `done` event |
| `fail(&AppError)` | in-band failure - rendered as an SSE `error` event, or truncation in chunked mode |

The first event commits the HTTP response head (status, content type, other headers).
Later events cannot change it. Writes after `close` or `fail` are dropped, mirroring
the edge dropping late segments after a timeout or client disconnect.

## What the client sees

**SSE mode** (`Content-Type: text/event-stream`) - the de facto wire for LLM token
streams and agent progress events:

```text
data: Hello

event: tokens
data: {"n":2}

event: done
data: {"usage":42}
```

- A map segment renders as compact one-line JSON; multi-line text splits into
  successive `data:` lines per the SSE specification.
- End of transmission is the terminal `event: done`, carrying the `close_with(...)`
  metadata.
- An in-band failure is `event: error` with `{"status":n,"message":"...","type":"error"}` -
  the HTTP status is already committed by then, which is why the failure travels in-band.
- While the producer is quiet, the edge emits an SSE comment (`: ping`) every
  `event.stream.keep.alive` interval (default `30s`, `0` disables) so idle proxies do not
  drop the connection. Pings do not extend the idle timeout.

**Chunked mode** (any other content type): text and byte segments append verbatim;
map segments stream as JSON Lines (one compact JSON object per line). End of
transmission simply ends the response.

## Timeouts, disconnects and slow clients

- The endpoint `timeout` acts as an *idle* allowance - each arriving segment extends it.
  A producer may override the idle allowance with
  `first_with_ttl(status, content_type, ttl_seconds)`.
- A stalled stream fails in-band: the client receives an SSE `error` event with status
  408 (chunked mode: the response truncates).
- When the client disconnects, the context closes and late segments are dropped as
  no-ops - producers do not need to handle cancellation.
- A client that stops reading beyond the idle allowance is truncated - for SSE, the
  missing terminal `done` event is the in-band truncation signal.

## Failure before the first segment

`fail(...)` before any `write(...)` renders a normal HTTP error response (the head is not
yet committed), so early failures keep proper HTTP status codes.

## Try it

The hello-world example ships a runnable demo: the `hello.sse` function serves
`GET /api/hello/sse` (declared with `stream: true`) and streams test messages slowly so
you can watch the progressive rendering. Start the application and consume the endpoint
with the companion Node.js script, which prints each event with its arrival time:

```shell
cargo run -p hello-world
```

```shell
node examples/hello-world/scripts/sse-client.mjs
```

or with curl:

```shell
curl -N -H 'accept: text/event-stream' http://127.0.0.1:8085/api/hello/sse
```

The `-N` (`--no-buffer`) flag matters: curl receives the events progressively either
way, but without `-N` it holds output in an internal buffer, so the messages would
appear all at once when the stream ends.

## Relation to `x-stream-id`

The Java engine also carries a legacy `x-stream-id` relay (object streams, file
downloads, `Flux` results), which is a documented deferral in this port. `x-event-stream`
is the idiom for progressive text/JSON delivery - tokens and events - with SSE framing
and in-band terminal signals, and it is fully supported here. If both headers appear on
a response, `x-event-stream` wins and the stray `x-stream-id` is ignored with a warning.

## See also

- [Event over HTTP](event-over-http.md) - the same envelope protocol between applications
- [Actuators & HTTP Client](actuators-and-http-client.md)
