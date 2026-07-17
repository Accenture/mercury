# hello-flow — the event-script (layer 2) taste

Composable orchestration: the transaction lives in
`resources/flows/hello-flow.yml` — a **flow** that routes by language and
composes a greeting. The two Rust functions never call each other and never
see HTTP; the flow engine choreographs them by route name, and `rest.yaml`
binds the endpoint to the flow with one line:

```yaml
  - service: "http.flow.adapter"
    url: "/api/hello/{user}"
    flow: 'hello-flow'
```

Linking the `event-script` crate self-registers the whole engine (compiler,
manager, task executor, HTTP adapter, resilience handler, 42 built-in
plugins) through the same annotation inventory that collects the app's own
functions — `main()` is still one line.

## Run it

```bash
cargo run -p hello-flow
```

The startup log shows the layered boot: the plugin loader (sequence 3) → the
flow compiler (sequence 5, `Event scripts deployed: 1`) → preloaded functions
→ REST automation on port **8086** → the main application announcing
`Flows ready: ["hello-flow"]`.

## Test drive

**The flow, both branches** — the `language.router` decision task picks the
greeting path:

```bash
curl 'http://127.0.0.1:8086/api/hello/eric?lang=fr'
# {"cid":"...","handled_by_instance":N,"message":"Bonjour, eric!","served_by":"hello-flow"}

curl 'http://127.0.0.1:8086/api/hello/eric?lang=en'
# {"cid":"...","message":"Hello, eric!",...}
```

**Business correlation-id propagation** — the HTTP edge's correlation header
becomes the flow's `model.cid`, which the flow maps into the greeting
function's input; the response proves the id survived the whole transaction:

```bash
curl -H 'X-Correlation-Id: order-777' 'http://127.0.0.1:8086/api/hello/eric?lang=en'
# {"cid":"order-777", ...}
```

**Distributed tracing** — the endpoint declares `tracing: true`, so each
request produces spans for the flow's tasks plus the flow-summary span
(watch the `distributed.tracing` lines in the terminal listing every task
and its timing).

**The landing page** — static HTML from `resources/public/` with links to
everything below:

```bash
curl 'http://127.0.0.1:8086/'          # or open it in a browser
```

**Actuators** — `/health` runs the sample `flow.engine.health` dependency,
which reports on the flow engine itself (healthy while flows are deployed):

```bash
curl 'http://127.0.0.1:8086/info'
curl 'http://127.0.0.1:8086/env'       # the properties selected in application.yml
curl 'http://127.0.0.1:8086/health'
# {"dependency":[{"message":"flow engine is running with 1 flow deployed",...}],"status":"UP"}
curl 'http://127.0.0.1:8086/livenessprobe'
```

## Change the transaction without touching code

Edit `resources/flows/hello-flow.yml` — add a task, remap a value, change the
decision — and restart. For example, add a Spanish branch: a third entry in
`next:`, a `greet.spanish` task with `text(Hola) -> greeting`, and a
three-way decision from `language.router`. The functions stay untouched;
orchestration is configuration.

## Where things live

| File | Role |
|---|---|
| `src/main.rs` | two composable functions + the one-line main |
| `resources/flows/hello-flow.yml` | **the transaction** (decision + 2 end tasks) |
| `resources/flows.yaml` | which flows to compile at startup |
| `resources/rest.yaml` | the endpoint → flow binding |
| `resources/application.yml` | app name, port, health dependency, /env selections |
| `resources/public/index.html` | the static landing page |
