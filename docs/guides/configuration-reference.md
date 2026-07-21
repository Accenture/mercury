# Configuration Reference

Every configuration key the Rust port reads, with its type, default, and the crate that reads
it. All keys were enumerated from this repository's source (`AppConfigReader` /
`ConfigReader` lookups across `crates/*/src`), so each entry is guaranteed to exist in the
running code — nothing here is inherited from the Java documentation unverified.

Applications are configured through `resources/application.yml` (or the equivalent
`application.properties`). The configuration syntax is kept **verbatim** from the Java
original — `classpath:/` and `file:/` location prefixes, `${ENV_VAR:default}` environment
substitution, and dot/bracket composite keys — so configuration files port between the two
implementations unchanged.

Resolution order for any key: `-Dkey=value` **runtime arguments** (the JVM `-D` analog,
passed after `--` on the cargo command line) win over configuration-file values, and the
files themselves merge in the manifest order described below.

!!! note "Rust port"
    The Java configuration reference documents many keys that do not exist in this port and
    are therefore absent from this page: the Spring Boot integration keys (`spring.*`,
    `server.port`), TLS and server extras (`rest.server.ssl*`, `api.origin`, `hsts.feature`,
    `oversize.http.response.header`, `websocket.binary.size`, `websocket.server.port`),
    the service mesh / cloud connector and all Kafka keys, the scheduler, PostgreSQL,
    sync-over-async/Redis, the OpenTelemetry forwarder (`otel.*`), classpath scanning
    (`web.component.scan`, `yaml.preload.override`, `modules.autostart`), threading
    (`kernel.thread.pool`, `deferred.commit.log`), event-script extras
    (`yaml.event.over.http`, `yaml.journal`, `yaml.multicast`),
    HTTP extras (`async.http.temp`, `stack.trace.transport.size`,
    `protect.info.endpoints`), and serialization extras (`snake.case.serialization`,
    `custom.content.types`, `mime.types`). If a key is not on this page, the Rust port
    does not read it.

## Bootstrap: files, profiles, and overrides

### `app-config-reader.yml` (manifest)

| Type | Default |
|---|---|
| resource file | built-in |

The manifest that bootstraps `AppConfigReader` (`crates/platform-core`). Its `resources:`
list names the base configuration files, merged **in order** (later files override earlier
ones); `profiles:` gives the location prefix for profile overlays. An application copy in
its `resources/` folder overrides the built-in default:

```yaml
resources:
  - classpath:/bootstrap.properties
  - classpath:/bootstrap.yml
  - classpath:/application.properties
  - classpath:/application.yml

profiles: 'classpath:/application-'
```

#### `app.profiles.active`

| Type | Default |
|---|---|
| string (comma/space-separated) | — |

Active configuration profiles. For each profile `p`, the overlays
`application-p.properties` and `application-p.yml` are merged over the base configuration.
Resolution precedence (read by `crates/platform-core`): the `APP_PROFILES_ACTIVE`
environment variable → the `-Dapp.profiles.active=...` process override → this key in the
consolidated configuration.

!!! note "Rust port"
    Renamed from the Java original's `SPRING_PROFILES_ACTIVE` / `spring.profiles.active` —
    Spring is not ported, so the port uses the generic names outright (a rename, not an
    alias). The mechanism and precedence are unchanged, so profile overlay files
    themselves port without modification.

## Application identity

#### `application.name`

| Type | Default |
|---|---|
| string | `application` |

The application name (`Platform::name()`), shown in startup logs and the `/info` actuator
response. Read by `crates/platform-core`.

!!! note "Rust port"
    The Java `spring.application.name` fallback is retired along with the other Spring
    names; `application.name` is the only key consulted.

#### `info.app.version`

| Type | Default |
|---|---|
| string | the platform-core crate version |

Application version string shown in the `/info` actuator response. Read by
`crates/platform-core` (actuator).

#### `info.app.description`

| Type | Default |
|---|---|
| string | the application name |

Application description shown in the `/info` actuator response. Read by
`crates/platform-core` (actuator).

## Server and REST automation

#### `rest.automation`

| Type | Default |
|---|---|
| boolean | `false` |

Enables the REST automation engine: the HTTP server starts and serves the endpoints
declared in `rest.yaml`. Read by `crates/platform-core` (application lifecycle). The HTTP
server also starts when at least one `#[websocket_service]` is registered, even with this
key off.

#### `rest.server.port`

| Type | Default |
|---|---|
| int | `8085` |

Listening port of the REST automation server. `0` binds an ephemeral port (tests and
embedders recover the assigned port through `automation::server_address()`). Read by
`crates/platform-core`; also read by `crates/knowledge-graph` to render the websocket
workbench page URLs.

#### `yaml.rest.automation`

| Type | Default |
|---|---|
| string (location) | `classpath:/rest.yaml` |

Location of the REST endpoint configuration file. Read by `crates/platform-core`
(automation server) at HTTP server start.

!!! note "Rust port"
    A single location — the Java comma-separated multi-file merge is not ported. Static
    content is always served from the application's `resources/public` folder (the Java
    `static.html.folder` key is not ported), and the `static-content.filter` /
    `static-content.no-cache-pages` blocks are `rest.yaml` sections here, not application
    configuration keys.

#### `websocket.idle.timeout`

| Type | Default |
|---|---|
| int (seconds) | `60` |

Idle expiry for websocket connections (floor 10 seconds). A connection with no traffic for
this long is closed with code 1003. Read by `crates/platform-core` (websocket server).

!!! note "Rust port"
    The unit is **seconds** in this port (verified against the websocket server source);
    the Java reference documents its key in minutes.

## HTTP client

#### `http.client.connection.timeout`

| Type | Default |
|---|---|
| int (milliseconds) | `5000` |

Connection timeout of the built-in async HTTP client (`async.http.request`). Read by
`crates/platform-core` (automation). The per-request time-to-live is separate — it travels
as the `x-ttl` header on the request event (default 30 seconds; see
[Reserved Names & Headers](reserved-names-and-headers.md)).

## Tracing and observability

#### `http.trace.id.header`

| Type | Default |
|---|---|
| string (header name) | `X-Trace-Id` |

The HTTP header recognized inbound (when no W3C `traceparent` is present — a well-formed
`traceparent` always takes precedence) and emitted outbound by the async HTTP client as
the trace id. Read by `crates/platform-core` (REST automation server and HTTP client). A
`rest.yaml` endpoint entry may override the header name per endpoint with its optional
`trace.id.header` key.

#### `http.correlation.id.header`

| Type | Default |
|---|---|
| string (header name) | `X-Correlation-Id` |

The HTTP header carrying the **business correlation-id**. Captured at the REST automation
edge (a fresh dash-less UUID is generated when absent), exposed to the target function as
the read-only `my_correlation_id` request header, preserved by the flow engine, propagated
on every `PostOffice` send/RPC inside a traced request, and emitted by the async HTTP
client on downstream calls. Read by `crates/platform-core` (server and HTTP client) and
`crates/event-script` (flow adapter). A `rest.yaml` endpoint entry may override the header
name per endpoint with its optional `correlation.id.header` key.

#### `skip.rpc.tracing`

| Type | Default |
|---|---|
| string (comma/space-separated routes) | `async.http.request` |

Route names excluded from distributed trace recording (in addition to the telemetry
plumbing itself, which is always excluded). Read by `crates/platform-core` (worker
dispatch).

## Logging

#### `log.format`

| Type | Default |
|---|---|
| `text` \| `json` \| `compact` | `text` |

Application log output format: `text` is a plain console line; `json` pretty-prints each
record; `compact` emits single-line jsonl records (no CR/LF within a record). Both JSON
forms carry the application log context block when `app-log-context.yaml` is present. Read
by `crates/platform-core` (logging). Switch at launch with `-Dlog.format=json`.

#### `log.level`

| Type | Default |
|---|---|
| `error` \| `warn` \| `info` \| `debug` \| `trace` \| `off` | `info` |

Process log level. The `RUST_LOG` environment variable, when set, wins over this key. Read
by `crates/platform-core` (logging).

!!! note "Rust port"
    A port addition — the Java original configures levels through log4j2 XML instead.

### `app-log-context.yaml` (optional resource file)

| Type | Default |
|---|---|
| resource file | absent (feature off) |

When present on the resource path, every structured (JSON) log line emitted inside a traced
function carries a `context` block. Its `context:` section maps an output key of your
choice to a reserved `$token` (`$cid`, `$traceId`, `$tracePath`, `$spanId`,
`$parentSpanId`, `$service`, `$utc` — resolved live per line), a `${ENV:default}`
substitution (resolved once at load), or a literal. Keys that resolve to nothing are
omitted. Business key-values added with `PostOffice::update_context` join the same block.
Read by `crates/platform-core` (logging).

## Actuators and health

#### `show.env.variables`

| Type | Default |
|---|---|
| string (comma-separated names) | — |

Environment variable names to expose through the `/env` actuator endpoint. Read by
`crates/platform-core` (actuator).

#### `show.application.properties`

| Type | Default |
|---|---|
| string (comma-separated keys) | — |

Application configuration keys to expose through the `/env` actuator endpoint. Read by
`crates/platform-core` (actuator).

#### `mandatory.health.dependencies`

| Type | Default |
|---|---|
| string (comma-separated routes) | — |

Route names of health-check functions that must all report healthy for `/health` to return
HTTP 200. Read by `crates/platform-core` (actuator).

#### `optional.health.dependencies`

| Type | Default |
|---|---|
| string (comma-separated routes) | — |

Route names of health-check functions whose failure is reported in the `/health` response
without failing it. Read by `crates/platform-core` (actuator).

## Performance and back-pressure

#### `worker.instances.no.op`

| Type | Default |
|---|---|
| int | `500` |

Worker-instance count of the built-in `no.op` echo function. Read by `crates/platform-core`
(application lifecycle).

!!! note "Rust port"
    The Java generic `worker.instances.<route>` override pattern is not ported. A function's
    instance count is configuration-driven through the `env_instances` parameter of its own
    `#[preload]` attribute instead (see the [Macros Reference](macros-reference.md)).

#### `elastic.queue.dispatch.mailbox.size`

| Type | Default |
|---|---|
| int | `1024` |

Capacity of each route manager's inbound mailbox (floor 20). When it fills, senders await —
reactive back-pressure, not drops. Read by `crates/platform-core` (platform registry).

#### `elastic.queue.segment.size.bytes`

| Type | Default |
|---|---|
| int (bytes) | `16777216` |

Segment-file size of the elastic queue's disk spill (minimum 512). The first 20 events of a
burst are held in memory before spilling. Read by `crates/platform-core` (elastic queue).

#### `transient.data.store`

| Type | Default |
|---|---|
| string (path) | `/tmp/reactive` |

Base directory for the elastic-queue overflow buffer that absorbs event bursts when
consumers are slower than producers. Read by `crates/platform-core` (elastic queue).

#### `running.in.cloud`

| Type | Default |
|---|---|
| boolean | `false` |

When `false`, a per-instance subdirectory (`<application.name>-<origin>`) is created under
`transient.data.store` and expired sibling stores are swept at startup. When `true`, the
path is used as-is (for ephemeral containers). Read by `crates/platform-core` (elastic
queue).

## Serialization

#### `serializer.null.transport`

| Type | Default |
|---|---|
| boolean | `false` |

Null handling at every wire boundary (JSON HTTP responses, websocket frames, outbound HTTP
request bodies, and the MsgPack envelope encoder). Default `false`: null **map** key-values
are omitted from the output — absent means null. Set `true` when a downstream must
distinguish "key present with null value" from "key absent". Only map key-values are
affected — array elements (including nulls) are always kept so ordering is preserved, and
an empty `[]` or `{}` is a real value, never treated as null. Read once at startup by
`crates/platform-core` (serializer) and cached for the process lifetime.

## Event Script (flow engine)

#### `yaml.flow.automation`

| Type | Default |
|---|---|
| string (location) | `classpath:/flows.yaml` |

Location of the flow manifest, compiled at startup by the flow compiler (a
before-application hook at sequence 5). The manifest lists the flow definition files and
where to find them:

```yaml
flows:
  - 'hello-flow.yml'

location: 'classpath:/flows/'
```

`location` inside the manifest defaults to `classpath:/flows/`. Read by
`crates/event-script` (compiler).

#### `max.model.array.size`

| Type | Default |
|---|---|
| int | `1000` |

Ceiling for a **dynamically resolved** array index on the right-hand side of a data
mapping (a `[model.x]` index) — a mapping whose resolved index exceeds it fails instead of
allocating an arbitrarily large state-machine array. Literal numeric indices are not
capped (same as the Java engine). Read once by `crates/event-script` (task executor).

!!! note "Port note"
    Added in increment 52 (parity remediation): the key existed in the Java engine from
    the start, and the syntax guide already described it — the Rust executor now enforces
    it.

## Knowledge graph and Playground

#### `app.env`

| Type | Default |
|---|---|
| string | `dev` |

The environment gate for the Playground developer surface. The interactive services — the
command grammar, the graph traveler, the websocket UI, the AI-companion endpoints, uploads,
inspection, and the mock functions — are each declared with
`#[optional_service("app.env=dev")]` and register only when this key is `dev`. Any other
value (e.g. `${APP_ENV:dev}` resolved to `prod`) runs graphs only through
`POST /api/graph/{graph-id}` and serves the `/template` home page instead of `/public`.
Read by `crates/knowledge-graph`.

#### `graph.model.automation`

| Type | Default |
|---|---|
| string (location) | — (unset = skip) |

Location of the graph manifest listing the graph models to compile and register at startup
(a `graphs:` list of graph ids, loaded from `location.graph.deployed`). When unset or
empty, graph compilation is skipped. Read by `crates/knowledge-graph` (compiler).

#### `location.graph.deployed`

| Type | Default |
|---|---|
| string (location) | `classpath:/graph` |

Where deployed graph models live (`classpath:/` or `file:/`; anything else falls back to
the default). The Playground `deploy` command writes here and `import graph from
{deployed}` reads from here. Read by `crates/knowledge-graph`.

#### `location.graph.temp`

| Type | Default |
|---|---|
| string (path) | `/tmp/graph` |

The Playground draft-graph scratch folder. Must be on the local file system
(`classpath:` is rejected because of read/write requirements); a `file:` prefix is
accepted and stripped. Invalid values fall back to the default. Read by
`crates/knowledge-graph` (Playground commands).

#### `graph.max.loop.interval`

| Type | Default |
|---|---|
| int (milliseconds) | `1000` |

The observation window of the graph traversal loop guard (floor 100). Together with
`graph.node.high.frequency`, it detects a runaway cycle: a node revisited too often within
the window aborts the traversal. Read by `crates/knowledge-graph`.

#### `graph.node.high.frequency`

| Type | Default |
|---|---|
| int (hits per window) | `10` |

How many visits to the same node within `graph.max.loop.interval` count as a runaway loop
(floor 2). Read by `crates/knowledge-graph`.

## Application-defined keys

Two framework mechanisms read keys whose **names the application chooses**, so any such key
in an example's `application.yml` is application configuration, not a framework key:

- **`env_instances`** — `#[preload(route = "...", env_instances = "my.pool.size")]` reads
  `my.pool.size` at startup for the worker count (the value may itself use
  `${SOME_ENV:default}` syntax); the literal `instances` is the fallback.
- **`#[optional_service("condition")]`** — the condition references any configuration key
  (`key=value`, `key`, `!key`); see the [Macros Reference](macros-reference.md).

---

*Adapted from the mercury-composable guide `docs/guides/configuration-reference.md`; keys/APIs enumerated from this repository's source.*
