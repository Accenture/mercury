# Polyglot Functions

**Polyglot functions** let you write a composable function in **Python** or **Node.js**
and wire it into this engine's Event Script flows and MiniGraph knowledge graphs with
**zero orchestration code** in the foreign language. The mechanism is the declarative
[Event over HTTP](event-over-http.md) map — the wrappers below are long-lived Event API
peers, not language ports.

> **Versions** — Event Script flows call polyglot functions on any engine that speaks the
> standard wire format; MiniGraph `graph.task` requires engine **v4.11.11+** (both the
> Rust and Java engines shipped the guard change in that release).

## Why peers, not ports

A Mercury function is addressed by a route-name string and receives an `EventEnvelope` —
nothing in that contract says "Rust" or "Java". So the shortest path to another language
is not porting the composable core (this repository *is* such a port, and knows the
cost), it is letting a function in that language **speak the same envelope over the
existing Event API endpoint**:

| Package | Repository | Documentation |
|---------|------------|---------------|
| Composable for Python | [Accenture/mercury-python](https://github.com/Accenture/mercury-python) | <https://accenture.github.io/mercury-python/> |
| Composable for Node.js | [Accenture/mercury-nodejs](https://github.com/Accenture/mercury-nodejs) | <https://accenture.github.io/mercury-nodejs/> |

Each wrapper hosts `POST /api/event` with the engines' exact semantics, registers
functions with the engines' `preload` contract (route name, `instances`, private
visibility), and carries a thin `PostOffice` client, a primitive in-process event bus for
leaf-side composition, and the minimalist utilities (configuration with the `resources/`
convention and `-Dkey=value` overrides, engine-format logging with
`log.format=text|json|compact`, trace context). The envelope codec implements the
language-neutral [standard wire format](event-envelope-reference.md) and is verified
against the same golden conformance vectors this engine shares with the Java engine.

What the wrappers deliberately do **not** contain: flows, graphs, persistence, pub/sub.
Orchestration — sequencing, branching, retries, compensation — stays in Event Script and
MiniGraph on the engine, where it is declarative, inspectable, and governed. A polyglot
function is a **unit of work**; the architecture keeps it that way.

## Wiring: one map entry per route

On the engine application, a polyglot route is declared exactly like any remote route —
`application.yml`:

```yaml
yaml.event.over.http: 'classpath:/event-over-http.yaml'
```

`event-over-http.yaml`:

```yaml
event.http:
  - route: 'hello.declarative'
    target: 'http://${peer.demo.host:127.0.0.1}:${peer.demo.port}/api/event'
```

Any flow task or `graph.task` node that names `hello.declarative` now executes the Python
(or Node.js) function — the flow and the graph neither know nor care.

### The zero-code demo, third language

The [Event over HTTP zero-code demo](event-over-http.md#zero-code-demo-hello-flow-to-hello-world)
already swaps its Rust callee for the Java lambda-example with no changes. The polyglot
wrappers extend the same swap: their demo apps register the same public
`hello.declarative` route, and the wrapper's default port is 8085 — hello-world's slot.

1. Start hello-flow as in the demo walk-through (`cargo run -p hello-flow`).
2. Instead of hello-world, start the Python demo (from a clone of mercury-python, after
   its documented setup):

    ```shell
    mercury-serve examples/demo_app.py -Drest.server.port=8085
    ```

    or the Node.js demo (from a clone of mercury-nodejs, after `npm install` and
    `npm run build`):

    ```shell
    node dist/src/cli.js examples/demo-app.mjs -Drest.server.port=8085
    ```

    (The `-Dkey=value` override syntax is the engines' own — the wrappers carry the same
    configuration conventions, so operating them feels identical.)

3. Run the declarative demo endpoint:

    ```shell
    curl -s -X POST -H "content-type: application/json" \
         -d '{"hello": "world"}' http://127.0.0.1:8100/api/event/http/declarative
    ```

The reply now carries `"language": "python"` (or `"node.js"`), and the wrapper's log line
shows the **engine's trace id** in the engine-consistent log format. The acceptance
drives behind this pattern — the golden-vector conformance run, the cross-wrapper drive,
and an unchanged engine flow executing the Python function — are recorded in the
[interop report's polyglot wrapper round](../test-reports/event-over-http-interop.md#the-polyglot-wrapper-round-2026-08-22).

## Calling from a knowledge graph

`graph.task` invokes composable functions from graph nodes, and a declarative
Event-over-HTTP target is a composable function. From engine **v4.11.11** the
route-existence guard consults the declarative map
(`event_api::get_event_http_target` in this engine), so a deployed graph can name a
polyglot route directly:

```text
skill=graph.task, task=hello.declarative
```

On engines before v4.11.11 the guard only checked local routes and a graph naming a
remote target failed validation — upgrade both engines to at least v4.11.11 before
pointing graphs at polyglot functions.

## The contract, end to end

The function contract is the engines' own, restated in each wrapper's documentation
(start at the wrapper's *AI agent guide* for the token-efficient version):

- **Input** — `(headers, body)` exactly as an engine function sees them; reserved engine
  headers are cleaned at ingress; the caller's business correlation id arrives as the
  read-only `my_correlation_id` header.
- **Errors are portable** — a wrapper `AppException(400, "missing 'text'")` becomes a
  400 envelope on HTTP 200 (handler errors ride HTTP 200 with envelope status, exactly
  like the engines); the calling flow's `exception:` task or the graph's `error.*`
  context fires as if the function were local. Transport-level failures keep the engine
  status codes: 403 private target, 404 unknown route, 408 timeout.
- **Trace continuity** — the engine's trace id and path ride the wire; wrapper telemetry
  and log lines join the same aggregated trace, and trace annotations return on the
  reply envelope.
- **Timeouts** — the flow's `ttl` (or the graph's) bounds the call; on breach the engine
  receives the standard 408 and the exception path decides recovery. Back-pressure and
  retries belong to the engine tier by design — the wrappers keep no spill queue.
- **Wire format** — the wrappers speak the **standard** envelope format only (the
  engines' default for Event over HTTP); the classic compact format is rejected with a
  teaching error.

## Operating a polyglot installation

The wrappers serve the engines' actuator endpoints on the same port as `/api/event` —
`/info`, `/info/routes`, `/env`, `/health`, `/livenessprobe` with the `type=info` /
`type=health` health-function contract. Kubernetes probes, dashboards, and log
aggregation treat a Python or Node.js app exactly like an engine app: one operational
surface, no per-language tooling — the same presentation-parity requirement the two
engines hold each other to.

## See also

- [Event over HTTP](event-over-http.md) — the underlying mechanism, endpoint security,
  and the full demo walk-through.
- [Composable for Python](https://accenture.github.io/mercury-python/) and
  [Composable for Node.js](https://accenture.github.io/mercury-nodejs/) — write the
  functions: handler styles, local composition, testing, AI agent guides.
- [Interop Test Report](../test-reports/event-over-http-interop.md) — the conformance
  evidence, including the polyglot wrapper round.
- [EventEnvelope](event-envelope-reference.md) — the envelope and the standard wire
  format.
