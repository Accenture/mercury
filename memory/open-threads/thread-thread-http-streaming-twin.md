- [x] (feature — **MERGED 2026-08-29 as
  [PR #216](https://github.com/Accenture/mercury/pull/216) merge `b191cf1e` carrying
  gated `fa2f2654` (tree verified), CI green, branch deleted both ends**)
  **HTTP response streaming — the Java engine's feature (Java PR #299/ADR-0018)
  ported engine-identical; Increment 91, ADR-0015 Proposed (flip to Accepted rides
  the next docs commit — the merge is the acceptance event).** `x-event-stream:
  data|eof|exception` multi-shot reply route; `stream: true` rest.yaml surface;
  dedicated ordered reply lane per request from a LIFO pool of 500
  (`async.http.response.stream.{n}`), HTTP-503 "Streaming response pool exhausted"
  on empty; SSE/chunked+NDJSON standards-only wire; `EventStreamWriter` producer
  (`event_stream.rs`); hyper edge now stream-capable (one boxed body type +
  ChannelBody); renderer-task idle deadline replaces Java's housekeeper
  (wire-identical in-band 408 "Timeout for N seconds"); response header transform
  parity on streamed heads; `/info/routes` family compression + 10-min cached
  routing view; demo hello.sse + scripts/sse-client.mjs in examples/hello-world.
  Known deferral unchanged: the x-stream-id relay. Full detail: origin log.
  <!-- id: thread-http-streaming-twin | created: 2026-08-29 | last_used: 2026-08-30 | uses: 4 | tier: active | origin: 2026-08-29-012914 -->
