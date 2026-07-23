# hello-flow — the event-script (layer 2) taste

Composable orchestration: the transaction lives in
`resources/flows/hello-flow.yml` — a **flow** that routes by language and
composes a greeting. The Rust functions never call each other and never
see HTTP; the flow engine choreographs them by route name, and `rest.yaml`
binds the endpoint to the flow with one line:

```yaml
  - service: "http.flow.adapter"
    url: "/api/hello/{user}"
    flow: 'hello-flow'
```

Linking the `event-script` crate self-registers the whole engine (compiler,
manager, task executor, HTTP adapter, resilience handler, 45 built-in
plugins) through the same annotation inventory that collects the app's own
functions — `main()` is still one line.

This example is the structural parallel of the Java **composable-example**
(same port **8100**, same demo endpoints) — it also ships the two
[Event-over-HTTP demo endpoints](#the-event-over-http-demo-both-patterns)
below.

## Run it

```bash
cargo run -p hello-flow
```

The startup log shows the layered boot: the plugin loader (sequence 3) → the
flow compiler (sequence 5, `Event scripts deployed: 3`) → preloaded functions
→ REST automation on port **8100** → the main application announcing
`Flows ready: [...]`.

## Test drive

**The flow, both branches** — the `language.router` decision task picks the
greeting path:

```bash
curl 'http://127.0.0.1:8100/api/hello/eric?lang=fr'
# {"cid":"...","handled_by_instance":N,"message":"Bonjour, eric!","served_by":"hello-flow"}

curl 'http://127.0.0.1:8100/api/hello/eric?lang=en'
# {"cid":"...","message":"Hello, eric!",...}
```

**Business correlation-id propagation** — the HTTP edge's correlation header
becomes the flow's `model.cid`, which the flow maps into the greeting
function's input; the response proves the id survived the whole transaction:

```bash
curl -H 'X-Correlation-Id: order-777' 'http://127.0.0.1:8100/api/hello/eric?lang=en'
# {"cid":"order-777", ...}
```

**Distributed tracing** — the endpoint declares `tracing: true`, so each
request produces spans for the flow's tasks plus the flow-summary span
(watch the `distributed.tracing` lines in the terminal listing every task
and its timing).

**The landing page** — static HTML from `resources/public/` with links to
everything below:

```bash
curl 'http://127.0.0.1:8100/'          # or open it in a browser
```

**Actuators** — `/health` runs the sample `flow.engine.health` dependency,
which reports on the flow engine itself (healthy while flows are deployed):

```bash
curl 'http://127.0.0.1:8100/info'
curl 'http://127.0.0.1:8100/env'       # the properties selected in application.yml
curl 'http://127.0.0.1:8100/health'
# {"dependency":[{"message":"flow engine is running with 3 flows deployed",...}],"status":"UP"}
curl 'http://127.0.0.1:8100/livenessprobe'
```

## The Event-over-HTTP demo (both patterns)

Two endpoints call the **same peer function** in another application through
the two Event-over-HTTP patterns. The callee is the `hello-world` example —
or, interchangeably, the Java `lambda-example`: both listen on port **8085**
and register the same public echo under two route names
(`hello.world, hello.declarative`). Start the callee in another terminal:

```bash
cargo run -p hello-world      # or the Java lambda-example — same port, same routes
```

**Declarative** (zero code — the target address is deployment configuration):
the flow's task is the foreign route `hello.declarative`, which does not
exist here; `resources/event-over-http.yaml` resolves it to the peer's
`/api/event` endpoint and the call crosses the wire automatically — carrying
the shared demo token as a security header (the peer's `/api/event` is
protected by its `event.api.auth` service; both sides resolve the token from
the `DEMO_PEER_TOKEN` environment variable, default `demo` for local dev).

```bash
curl -s -X POST -H 'content-type: application/json' \
     -d '{"hello":"world"}' http://127.0.0.1:8100/api/event/http/declarative
```

**Programmatic** (the code computes the target at runtime): the flow's task
`v1.event.over.http.rpc` builds the peer's URL from `peer.demo.host` /
`peer.demo.port` and passes it directly to the request API — so its target
route `hello.world` needs **no** entry in `event-over-http.yaml`.

```bash
curl -s -X POST -H 'content-type: application/json' \
     -d '{"hello":"world"}' http://127.0.0.1:8100/api/event/http/programmatic
```

Both replies echo the body, headers, worker instance and the **callee's**
`origin` id — proof the function ran in the other application. Watch the
`distributed.tracing` records on both terminals: one continuous trace, with
the callee's span chained onto the caller's task span across the HTTP hop.
Point `peer.demo.host`/`peer.demo.port` at any peer that exposes the routes —
the Java and Rust callees are drop-in replacements for each other, which is
the point of the demo. The full walk-through (including the cross-language
angle) is in the docs site's *Event over HTTP* guide.

## Change the transaction without touching code

Edit `resources/flows/hello-flow.yml` — add a task, remap a value, change the
decision — and restart. For example, add a Spanish branch: a third entry in
`next:`, a `greet.spanish` task with `text(Hola) -> greeting`, and a
three-way decision from `language.router`. The functions stay untouched;
orchestration is configuration.

## Where things live

| File | Role |
|---|---|
| `src/main.rs` | the composable functions + the one-line main |
| `resources/flows/hello-flow.yml` | **the transaction** (decision + 2 end tasks) |
| `resources/flows/event-over-http-declarative.yml` | the declarative Event-over-HTTP demo flow |
| `resources/flows/event-over-http-programmatic.yml` | the programmatic Event-over-HTTP demo flow |
| `resources/event-over-http.yaml` | the declarative route → peer mapping |
| `resources/flows.yaml` | which flows to compile at startup |
| `resources/rest.yaml` | the endpoint → flow bindings |
| `resources/application.yml` | app name, port, peer address, health dependency |
| `resources/public/index.html` | the static landing page |
| `tests/event_over_http_demo.rs` | loopback e2e test of both demo endpoints |
