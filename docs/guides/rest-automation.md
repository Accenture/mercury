# REST Automation

Expose HTTP endpoints by configuration: **`rest.yaml` *is* the router**. An endpoint entry maps
a URL and its methods to a composable function (or an Event Script flow) — there are no
controllers, no handler registrations, and no web framework. The engine
(`crates/platform-core/src/automation/`) serves the routing table directly over hyper, so the
route string in the YAML is the only link between the HTTP edge and your code.

```yaml
rest:
  - service: "greeting.api"
    methods: ['GET']
    url: "/api/greeting/{user}"
    timeout: 10s
    cors: cors_1
    headers: header_1
    tracing: true
```

That entry — from the shipped [`examples/hello-world`](getting-started.md#layer-1-hello-world-functions-and-rest-automation)
— is the whole HTTP layer for the greeting endpoint. The function behind it is addressed only by
its route name, exactly as it would be from any other function.

A `rest.yaml` file has three sections: the `rest` endpoint list, reusable `cors` blocks, and
reusable `headers` transform blocks, plus an optional `static-content` block. Invalid entries
**fail the load** — a misconfigured endpoint is a startup error, not a silent 404.

## Turning it on

REST automation is optional. Enable it in `application.yml`:

```yaml
rest.automation: true
rest.server.port: 8085
```

#### `rest.automation`

| Type | Default |
|---|---|
| boolean | `false` |

Starts the HTTP server at boot. The server also starts automatically when the application
registers any websocket service, even if this flag is off.

#### `rest.server.port`

| Type | Default |
|---|---|
| int | `8085` |

The listen port. Port `0` binds an ephemeral port — useful in tests; the assigned address is
recoverable via `platform_core::automation::server_address()`.

#### `yaml.rest.automation`

| Type | Default |
|---|---|
| string | `classpath:/rest.yaml` |

Location of the routing configuration (`classpath:/` or `file:/`).

!!! note "Rust port"
    The Java engine accepts a comma-separated list of locations here and merges them
    sequentially; this port reads a **single** location. The Java `server.port` alias (for
    Spring Boot co-existence) is also not read — there is no Spring in the Rust port, so
    `rest.server.port` is the one key.

At startup every loaded route is logged as `METHOD /url -> service`, so the effective routing
table is always visible in the application log.

## A `rest:` endpoint entry

#### `service`

| Type | Required |
|---|---|
| string | yes |

The route name of the composable function that handles matched requests. Use the built-in
`http.flow.adapter` together with [`flow`](#flow) to bind the endpoint to an Event Script flow
instead.

!!! note "Rust port"
    Two Java `service` forms are deferred: an `http(s)://…` URL (**HTTP relay**, with its
    `url_rewrite` and `trust_all_cert` companions) is rejected at load time with an explicit
    "not yet ported" error, and a two-element list (**A/B dual service** — primary plus a
    listen-only secondary) fails to load because `service` must be a single route. Multipart
    upload (the Java `upload` flag) is also deferred; unrecognized keys in an entry are ignored.

#### `methods`

| Type | Required |
|---|---|
| list | yes |

Any subset of `GET`, `PUT`, `POST`, `DELETE`, `HEAD`, `PATCH`. Anything else fails the load.
Do not list `OPTIONS` — it is added automatically for CORS preflight and matches every entry.

#### `url`

| Type | Required |
|---|---|
| string | yes |

The URI path. Literal segments match case-insensitively. Two dynamic forms are supported:

- `{param}` captures one path segment (case preserved) and delivers it to the function as a
  path parameter — `/api/greeting/{user}`;
- a trailing `*` matches the remainder of the path (it must be the **last** segment) —
  `/api/files/*`.

When several entries match, the most specific wins: a non-wildcard route beats a wildcard one,
then the route with more literal segments wins — so `/api/greeting/system` is chosen over
`/api/greeting/{user}` for that exact path.

#### `timeout`

| Type | Default |
|---|---|
| duration | `30s` |

Maximum time to wait for the function's response, written as `10s`, `2m`, or `1500ms` (a bare
number means seconds). Values are clamped to the range 1 second – 5 minutes; an unparsable
value falls back to the default. On expiry the caller receives an HTTP **408** error response.
For a flow-bound endpoint this value also rides into the flow input dataset as the `timeout`
field (in seconds), from which the flow adapter derives the flow instance TTL.

#### `cors`

| Type | Default |
|---|---|
| string | none |

The `id` of a [`cors` block](#cors-blocks). The reference must exist or the load fails.

#### `headers`

| Type | Default |
|---|---|
| string | none |

The `id` of a [`headers` block](#headers-blocks). The reference must exist or the load fails.

#### `authentication`

| Type | Default |
|---|---|
| string | none |

The route name of an authentication function. When set, the incoming request event is first
sent to this route (with the endpoint's timeout). The contract:

- return body `true` → the request proceeds to `service`;
- return body `false` (or anything that is not boolean `true`) → **401 Unauthorized**;
- return an error (`Err(AppError)` / status ≥ 400) → that status and message are returned
  to the caller, so the function can reject with a custom status such as 403.

!!! note "Rust port"
    Only the simple form — one route for the endpoint — is ported. The Java routing specs
    (`'default: svc'`, `'header: svc'`, `'header: value: svc'`) and the Java option of
    returning an `EventEnvelope` whose headers become session info for the target function
    are not yet available: the verdict must be a boolean.

#### `tracing`

| Type | Default |
|---|---|
| boolean | `false` |

Starts a distributed trace at the HTTP edge for every request on this endpoint. The trace id
is resolved in precedence order: a well-formed W3C `traceparent` header wins (and the caller's
span becomes the parent span); otherwise the trace-id header (`X-Trace-Id` by default);
otherwise a fresh id is generated. See
[Function Execution — tracing](event-driven/function-execution.md#tracing-behavior-per-call-type)
for what happens downstream.

#### `trace.id.header` / `correlation.id.header`

| Type | Default |
|---|---|
| string | global config |

Per-endpoint overrides of the header names used to capture the trace id and the business
correlation id — impedance matching for one legacy caller with its own conventions. Precedence
is per-entry override, then the `application.yml` globals (`http.trace.id.header`, default
`X-Trace-Id`; `http.correlation.id.header`, default `X-Correlation-Id`), then the built-in
default. Header capture is case-insensitive, and a well-formed W3C `traceparent` still takes
precedence for the trace id.

A business **correlation id is always ensured** — independent of tracing. If the caller sends
none, one is generated (when one header name serves both roles, the trace id is reused so a
legacy caller sees one id, not two). It is exposed to the target function as the read-only
`my_correlation_id` request header, and to a flow as `model.cid`.

#### `flow`

| Type | Default |
|---|---|
| string | none |

An Event Script flow id, used together with `service: http.flow.adapter`. The complete
flow-bound endpoint from [`examples/hello-flow`](getting-started.md#layer-2-hello-flow-orchestration-as-configuration):

```yaml
rest:
  - service: "http.flow.adapter"
    methods: ['GET']
    url: "/api/hello/{user}"
    flow: 'hello-flow'
    timeout: 10s
    tracing: true
```

REST automation injects the flow id as the `x-flow-id` request header; the flow adapter
reshapes the HTTP request into the flow's `input.*` dataset and launches the flow, whose final
`output.*` mappings become the HTTP response. See the
[Flow Schema Reference](flow-schema-reference.md) and the
[flow grammar](event-script/flow-grammar.md).

## `cors:` blocks

Reusable CORS configurations, referenced from an entry by `id`. Convenient in development;
production systems usually do CORS at the API gateway.

```yaml
cors:
  - id: cors_1
    options:
      - "Access-Control-Allow-Origin: *"
      - "Access-Control-Allow-Methods: GET, OPTIONS"
      - "Access-Control-Allow-Headers: Content-Type, X-Correlation-Id, traceparent"
    headers:
      - "Access-Control-Allow-Origin: *"
```

`options` lines become the response headers of the **preflight**: an `OPTIONS` request to any
matching URL is answered directly by the engine with **204 No Content** plus these headers —
it never reaches your function. `headers` lines are added to every **normal** response from
the endpoint. Every line in both lists must have the form `Access-Control-*: value`; anything
else fails the load.

## `headers:` blocks

Reusable request/response header transforms, referenced from an entry by `id`.

```yaml
headers:
  - id: header_1
    request:
      drop: ['upgrade-insecure-requests', 'cache-control']
    response:
      add:
        - "x-served-by: mercury-rust"
```

Both `request` and `response` accept three directives, applied in this order:

`keep`
:   Retain only the listed header names, dropping everything else. When `keep` is non-empty
    it takes precedence over `drop`.

`drop`
:   Remove the listed header names.

`add`
:   Insert `'name: value'` lines (after keep/drop, so an added header always survives).

Header-name matching is case-insensitive throughout.

## What your function receives

A matched request is delivered as an event whose body is the **AsyncHttpRequest** map — the
same shape the Java engine uses:

```json
{
  "method": "GET",
  "url": "/api/greeting/eric",
  "ip": "127.0.0.1",
  "https": false,
  "host": "127.0.0.1:8085",
  "headers": { "accept": "*/*", "my_correlation_id": "…" },
  "parameters": {
    "path":  { "user": "eric" },
    "query": { "lang": "fr" }
  },
  "body": null,
  "timeout": 10
}
```

Header names arrive lowercase. Path and query parameter values are percent-decoded (`+`
becomes a space). `timeout` is the endpoint's timeout in seconds. See
[Write Your First Function](event-driven/write-your-first-function.md) for handling this event
in Rust.

### Content-type dispatch — how the body is parsed

The request body is parsed by the **declared** `content-type` (any `;charset=…` suffix is
ignored; the *value* is matched case-sensitively, Java parity). This port mirrors the Java
`HttpRouter` dispatch exactly:

`application/json`
:   An empty body becomes an empty map `{}`. A body wrapped in matching JSON brackets
    (`{…}` or `[…]`) is parsed into a map or list; if that parse fails, the **raw text** is
    delivered instead of an error. A body without the wrapping brackets stays raw text.
    There is **no JSON sniffing** under any other content type — a JSON-looking body sent as
    `text/plain` arrives as text.

`application/xml`
:   Delivered as **raw text** — see the port note below.

`application/x-www-form-urlencoded`
:   Form fields are decoded into **query parameters** (merged with any URL query string);
    the body itself stays null.

`text/html`, `text/plain`
:   Delivered as text.

anything else, or no content type
:   Delivered as **raw bytes** on the event envelope (an empty payload stays null). Bodies
    with or without a `content-length` header are aggregated in full before dispatch.

!!! note "Rust port"
    The XML-to-map parse is deferred with the rest of the XML surface (the Java
    `SimpleXmlParser`/writer pair is not ported) — XML bodies pass through as raw text on both
    the server and the HTTP-client side. Request/response **streaming** is also deferred:
    bodies are aggregated, matching Java's fixed-length path. The Java `custom.content.types`
    mapping feature is not ported.

## The response

The function's reply envelope maps back to HTTP:

- **Status** — the envelope status (HTTP semantics; ≥ 400 is an error).
- **Body** — by type: a map or list is rendered as JSON with `content-type: application/json`
  (null map entries are omitted unless `serializer.null.transport=true`); a string becomes
  `text/plain`; bytes become `application/octet-stream`; an empty body sends no content type.
- **Headers** — the response transforms of the entry's `headers` block are applied, then the
  entry's CORS `headers` lines are added.

Errors — a missing route, a rejected authentication, a function error, or a timeout — share
one JSON shape (Java parity):

```json
{ "status": 408, "message": "Request timeout for 10000 ms", "type": "error" }
```

with the matching HTTP status: **404** for an unmatched URL, **401** from authentication,
**408** when the endpoint `timeout` expires, and the function's own error status otherwise.

## Static content

When no `rest.yaml` entry claims a URL, `GET`/`HEAD` requests fall through to static content
served from the application's `resources/public` folder (an entry in `rest.yaml` always wins
over a static file). `/` and any path ending in `/` resolve to `index.html`; an extensionless
filename assumes `.html`; parent-directory traversal is rejected. Content types are resolved
from the file extension.

!!! note "Rust port"
    The Java `static.html.folder` parameter is not read — static content always comes from
    the application's `resources/public`. The `mime-types.yml` customization is deferred; a
    built-in extension table covers the common web types.

The optional `static-content` block tunes the behavior — this is the shipped
`examples/hello-world` configuration:

```yaml
static-content:
  no-cache-pages: ["/", "/index.html"]
  filter:
    path: ["/", "/assets/*", "*.html", "*.js"]
    exclusion: ["*.css"]
    service: "http.request.filter"
```

#### `no-cache-pages`

| Type | Default |
|---|---|
| list | `["/", "/index.html"]` |

Pages served with `Cache-Control: no-cache, no-store` (plus `Pragma`/`Expires`) instead of the
etag protocol — entry pages must always revalidate, e.g. for SSO redirection. Everything else
gets an **ETag** (a SHA-256 content hash); a matching `If-None-Match` returns **304** with an
empty body. Patterns here and in the filter are exact, `prefix*`, or `*suffix`.

#### `filter`

| Type | Default |
|---|---|
| map | none |

A composable function (`service`) that inspects static requests matching `path` but not
`exclusion` — the typical use is cookie checks and SSO browser redirection, or adding security
response headers. The filter receives an AsyncHttpRequest-shaped event (no body); its response
**headers are always copied** onto the HTTP response. Status 200 lets the file be served; any
other status — e.g. a 302 with a `Location` header — passes the filter's response through to
the client instead. `path` and `service` are mandatory when a filter is configured.

!!! note "Rust port"
    If the filter function itself fails, this port logs the error and serves the file anyway —
    a resilient divergence from Java, which leaves the request to time out.

## Built-in endpoints

When `rest.yaml` does not claim their URLs, four actuator endpoints are added automatically:
`/info`, `/env`, `/health` and `/livenessprobe` — your own entry for one of these URLs always
wins, which is how you attach `authentication`, CORS, or a different timeout to them. See
[Actuators & HTTP Client](actuators-and-http-client.md).

!!! note "Rust port"
    The Java engine also provides `POST /api/event` (Event-over-HTTP), `/info/lib`, and
    `/info/routes` as defaults. None of these are ported yet — see the
    [actuator port notes](actuators-and-http-client.md#actuator-endpoints).

## See also

- [Getting Started](getting-started.md) — run the examples these snippets come from.
- [Actuators & HTTP Client](actuators-and-http-client.md) — the built-in endpoints and the
  outbound async HTTP client.
- [Flow Schema Reference](flow-schema-reference.md) — the flows that `flow:` endpoints launch.
- [Write Your First Function](event-driven/write-your-first-function.md) — handling the
  AsyncHttpRequest event.

---

*Adapted from the mercury-composable guides `docs/guides/rest-automation/index.md` and
`docs/guides/rest-automation/rest-grammar.md`; behavior verified against this repository's
source.*
