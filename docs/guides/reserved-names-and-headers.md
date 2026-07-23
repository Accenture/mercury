# Reserved Names & Headers

The platform reserves some route names, event headers, and HTTP headers for its own
routing. Overloading a reserved name can silently break system behavior — this is the
do-not-collide list. Every name on this page was enumerated from this repository's source
(the constants and `#[preload]` declarations in `crates/*/src`), so the list reflects what
this port actually registers.

!!! note "Rust port"
    The Java list includes many names from unported subsystems, absent here: the connector
    and service-mesh routes (`cloud.connector`, `presence.*`, `system.service.*`,
    `*.multiplex.*`), the scheduler (`cron.scheduler`, `run.scheduled.job`), the Kafka and
    sync-over-async extensions,
    `actuator.services` / `lib.actuator.service` / `routes.actuator.service`
    (`/info/lib` and `/info/routes` are deferred), and `http.auth.handler` — a rest.yaml
    `authentication` entry names your authentication function's route directly here (the
    simple route form; the tag-based auth router is not ported).

## Reserved route names

### platform-core

| Route | Purpose |
|---|---|
| `distributed.tracing` | Telemetry sink — traced executions send their performance-metrics dataset here (a deliberate singleton: serializes trace-record processing to preserve ordering) |
| `temporary.inbox` | Event listener for RPC — the ONE reserved reply route: every `PostOffice::request` reply resolves through it by correlation id (private, zero-tracing, 500 instances) |
| `info.actuator.service` | `/info` actuator endpoint |
| `env.actuator.service` | `/env` actuator endpoint |
| `health.actuator.service` | `/health` actuator endpoint |
| `liveness.actuator.service` | `/livenessprobe` actuator endpoint |
| `no.op` | Echo placeholder function for skeletons and simple flow decisions (500 instances, tunable via `worker.instances.no.op`) |
| `async.http.request` | The built-in async HTTP client (an event interceptor) |
| `ws.server.housekeeper` | Websocket server cleanup service |

One route **prefix** is also reserved by platform-core:

- **`ws.`** — websocket connections. Each live connection gets a session id of the form
  `ws.{random}.{n}` with a `{session}.in` route (delivers lifecycle events to your
  `#[websocket_service]` function) and a `{session}.out` route (sends frames back to the
  client).

The **`inbox.*` namespace belongs to applications** — RPC replies resolve through the
single reserved `temporary.inbox` route by correlation id, so no per-request route or
prefix is claimed. Workflow applications are free to register routes like
`inbox.approval` (a staging area queued to a human operator); they behave — and are
traced — like any other user function.

### event-script

| Route | Purpose |
|---|---|
| `event.script.manager` | Instantiates a new flow instance |
| `task.executor` | Performs the event choreography of a running flow |
| `http.flow.adapter` | The built-in HTTP-to-flow adapter (rest.yaml `flow:` endpoints) |
| `resilience.handler` | Retry / alternative-path / circuit-breaker handler for flows |
| `simple.exception.handler` | Placeholder exception handler for rapid prototyping |

`event.script.manager` and `task.executor` are **reserved engine routes**: events to them
execute directly on a fresh task, bypassing the manager-worker queue, so orchestration
never waits on its own mailbox. This bypass is an engine privilege — it is deliberately not
exposed through registration options or the annotation macros, so application functions
always stay on the reactive back-pressure path.

The **`flow://` prefix** is a reserved addressing scheme, not a route: a flow task's
`process`, an external-state-machine target, or a graph node's `extension` property may
name `flow://{flow-id}` to launch a flow instead of calling a function.

### knowledge-graph

Registered whenever the knowledge-graph crate is part of the application:

| Route | Purpose |
|---|---|
| `graph.executor` | Runs a deployed graph (`POST /api/graph/{graph-id}`) |
| `graph.traveler`* | The interactive graph walker (dev) |
| `graph.api.fetcher` | The `graph.api-fetcher` skill — provider HTTP calls |
| `graph.data.mapper` | The `graph.data-mapper` skill |
| `graph.math` | The `graph.math` skill — statements and branching |
| `graph.task` | The `graph.task` skill — custom-logic seam to a composable function |
| `graph.join` | The `graph.join` skill — parallel fan-in |
| `graph.island` | The `graph.island` skill — the knowledge/entity layer |
| `graph.extension` | Delegation to another graph or a `flow://` flow |
| `graph.exception.handler` | Graph failure routing |
| `graph.housekeeper` | Graph instance cleanup |
| `graph.health` | Health probe for the graph engine |
| `get.index.html` | The home page (serves `/public` in dev, `/template` otherwise) |

Routes marked * — and the whole Playground developer surface below — are gated by
`#[optional_service("app.env=dev")]` and exist only when `app.env` is `dev`:

| Route | Purpose |
|---|---|
| `graph.command.service` | The Playground command grammar |
| `graph.command.singleton` | Single-worker command handler (orderly AI-companion requests) |
| `post.companion.command` | AI-companion endpoint (fire-and-forget) |
| `post.companion.command.sync` | AI-companion `/sync` endpoint (in-band outcome) |
| `show.graph.model`, `get.live.graph`, `inspect.state.machine` | Model and state inspection |
| `upload.json.content`, `upload.mock.content` | Playground uploads |
| `get.ws.html` | The raw websocket workbench pages |
| `mock.mdm.profile`, `mock.account.details`, `v1.hello.task` | Dev mock functions for the tutorials |

The **`companion.sync.` prefix** is reserved in dev: the `/sync` endpoint opens an
ephemeral capture route (`companion.sync.{uuid}`) per request. The websocket service names
`/ws/graph/{token}` and `/ws/json/{token}` are likewise dev-gated.

## Optional user-defined routes

The system detects these route names for additional user-defined features — register your
own function under them to opt in:

| Route | Purpose |
|---|---|
| `distributed.trace.forwarder` | Receives a copy of every telemetry dataset (e.g. an OpenTelemetry exporter) |
| `transaction.journal.recorder` | Receives request/response journals when journaling is enabled — payloads may contain PII/PHI/PCI; handle per your organization's security policy |

Both are part of the telemetry plumbing and are never themselves traced.

!!! note "Rust port"
    Java's optional `additional.info` function (merged into the `/info` response) is not
    ported.

## Reserved event headers

`my_route`, `my_trace_id`, `my_trace_path`, `my_correlation_id`
:   **Read-only metadata, injected at entry and sanitized at exit — never transported.**
    The worker injects these keys into the function's input header COPY at delivery time:
    the invoked route name, the trace id/path (from envelope fields), and the business
    correlation-id (from an **engine-managed envelope tag** — every `PostOffice` touch
    point, the flow engine's task dispatch, and the REST edge carry it on the tag, never
    as an envelope header). At exit the worker filters these keys (plus the
    engine-internal `x-event-api` relay guard) from a returned envelope's headers, so a
    function that accidentally copies its input headers onto its reply cannot leak them.
    Do not set them yourself; `po.my_correlation_id()` / `po.my_trace_id()` /
    `po.my_trace_path()` read the same data at any depth of the call graph.

`flow_id`, `correlation_id`
:   Engine-internal headers on events addressed to `event.script.manager` (flow selection
    and the business correlation-id handoff). Application code never sets these — the flow
    adapter and the task executor own them.

## Reserved envelope tags

The envelope's `tags` wire field is engine-managed and never visible to a user function.
Two tag names are reserved with cross-language semantics: **`rpc`** (marks an RPC request,
value = timeout in ms — drives the worker-record suppression so each RPC-served span
reports once) and **`my_cid`** (transports the business correlation-id; injected as the
read-only `my_correlation_id` header-copy key at delivery). See the
[EventEnvelope reference](event-envelope-reference.md#reserved-envelope-tags).

!!! note "Compatibility"
    A callee on this version still honors the business correlation-id transported as a
    `my_correlation_id` envelope header by a **pre-4.10.2 peer** (injected, then stripped
    from the function's view), but no longer sends it — business-cid continuity in mixed
    fleets requires both sides on this version, the same upgrade-together posture as the
    wire format.

## Reserved HTTP headers

| Header | Purpose |
|---|---|
| `X-Trace-Id` | Carries the trace id — recognized inbound at the REST edge, emitted outbound by the async HTTP client. The header **name** is configurable (`http.trace.id.header`; per-endpoint `trace.id.header` in rest.yaml) |
| `traceparent` | W3C Trace Context: `00-{trace-id}-{parent-span-id}-01` (32-hex trace id, 16-hex span id). Inbound it **takes precedence** over `X-Trace-Id` and contributes the caller's span as this hop's parent; outbound it is emitted alongside `X-Trace-Id`, built from this hop's own span |
| `X-Correlation-Id` | The business correlation-id (configurable: `http.correlation.id.header`; per-endpoint `correlation.id.header`). Captured at the edge (a fresh dash-less UUID is generated when absent), stamped onto the request dataset headers, and **echoed on the HTTP response** so an edge caller can correlate without parsing the body — a response header of the same name set by the function takes precedence |
| `x-flow-id` | Selects the flow a request launches. A rest.yaml entry's `flow:` value becomes this header on the request event; `http.flow.adapter` rejects a request without it |
| `x-ttl` | Time-to-live in **milliseconds** for an async HTTP client call (`AsyncHttpRequest::set_timeout_seconds` writes it; default 30 seconds when absent) |
| `x-content-length` | Set by the async HTTP client on the **response** event when the origin response carried no `content-length` (chunked transfer) — the actual body length in bytes |
| `x-stream-id` | Reserved but inert: it is on the client's strip list (headers that may interfere with the underlying HTTP client), and object streams are not yet ported |

When a call is traced, the platform stamps `X-Trace-Id` and `traceparent` on every outbound
HTTP request from the current trace context — do not set them in application code. The
framework echoes the business correlation-id back to the HTTP client (see the table) but
never the trace id.

!!! note "Rust port"
    Event-over-HTTP headers `x-ttl` / `x-async` (dispatch semantics on `POST /api/event`)
    and the engine-internal `x-event-api` relay-guard envelope header are ported — the
    relay guard never reaches a user function's view (removed at delivery). Java headers
    with no counterpart here (their features are unported): `X-Small-Payload-As-Bytes`,
    `X-Raw-Xml` (the XML parser/writer pair is not ported — XML bodies pass through as
    text), and `X-App-Instance` (`protect.info.endpoints` is not ported).

## Reserved graph node names and properties

Every knowledge-graph model has exactly two framework-created nodes, addressed by alias:

`root`
:   The traversal entry point. Connections written *from* `root` anchor the flow
    (`root -[contains]-> ...`).

`end`
:   The traversal sink. Nodes route *to* `end` to finish
    (`... -[then]-> end`).

Do not name your own nodes `root` or `end`. In addition, these node **property** names are
owned by the engine — they configure a node's skill and are never copied into the state
machine or usable as data-mapping sources:

```text
skill  mapping  statement  input  output  feature  exception
extension  status  error  dictionary  for_each  concurrency  purpose  task
```

## Transient data store

The elastic queue spills event bursts to `/tmp/reactive` (per-instance subdirectory unless
`running.in.cloud=true`). The location is configurable — see
[`transient.data.store`](configuration-reference.md#transientdatastore).

---

*Adapted from the mercury-composable guide `docs/guides/reserved-names-and-headers.md`; keys/APIs enumerated from this repository's source.*
