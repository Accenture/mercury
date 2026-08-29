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

## Consume an SSE stream (HTTP client)

The other direction: `async.http.request` can consume a Server-Sent Events response
progressively - an LLM provider's token stream, another engine's streaming endpoint, or
any SSE API - and relay each event to your reply route using the same streaming
protocol.

Activation is explicit and standard - all three must hold:

1. the request declares `Accept: text/event-stream`,
2. the response actually arrives as `Content-Type: text/event-stream`, and
3. the request event carries a `reply_to` (a multi-shot-capable consumer).

Anything else keeps the buffered single-shot behavior exactly as before.

Each upstream SSE event becomes one `x-event-stream: data` envelope to your reply
route: the event's data is the body (multi-line data joins with newline per the SSE
specification), an `event:` name maps to `x-event-name`, and comment/id/retry fields
are consumed by the client, never forwarded. The first envelope carries the head
(upstream status and the SSE content type). A clean upstream end sends `eof`; a
mid-stream failure sends an in-band `exception`. Payloads are never interpreted -
provider conventions such as `data: [DONE]` are forwarded verbatim for your function
to handle, keeping the client vendor-neutral.

For a stream, the request's timeout is the *idle* allowance between reads rather than
a total limit: any upstream bytes - keep-alive comments included - reset it, and on
expiry the client fails the stream in-band with status 408 and closes the upstream
connection.

The composition this enables: a streaming endpoint's function can forward its own
`reply_to` and correlation id into the client call, turning the application into an
SSE-to-SSE relay with no imperative streaming code -

```rust
#[preload(route = "v1.sse.relay", instances = 50, interceptor)]
struct SseRelay;

#[async_trait]
impl ComposableFunction for SseRelay {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let upstream = AsyncHttpRequest::new()
            .set_method("GET")
            .set_target_host("https://api.example.com")
            .set_url("/v1/tokens")
            .set_header("accept", "text/event-stream")
            .set_timeout_seconds(30);
        // the caller's reply lane becomes the client's reply route:
        // upstream tokens render progressively out the HTTP edge
        let po = PostOffice::new(&Platform::get_instance());
        po.send(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(upstream.to_value())
                .set_reply_to(input.reply_to().unwrap_or_default())
                .set_correlation_id(input.correlation_id().unwrap_or_default()),
        )
        .await?;
        Ok(EventEnvelope::new())
    }
}
```

## Relation to `x-stream-id`

The Java engine also carries a legacy `x-stream-id` relay (object streams, file
downloads, `Flux` results), which is a documented deferral in this port. `x-event-stream`
is the idiom for progressive text/JSON delivery - tokens and events - with SSE framing
and in-band terminal signals, and it is fully supported here. If both headers appear on
a response, `x-event-stream` wins and the stray `x-stream-id` is ignored with a warning.

## See also

- [Event over HTTP](event-over-http.md) - the same envelope protocol between applications
- [Actuators & HTTP Client](actuators-and-http-client.md)
