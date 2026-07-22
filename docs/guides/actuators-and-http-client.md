# Actuators & HTTP Client

Every application ships with a set of built-in **actuator endpoints** for health checks and
introspection, and a built-in **async HTTP client** for outbound calls — no extra wiring.
Everything on this page describes the behavior of this repository's engine
(`crates/platform-core/src/actuator.rs` and
`crates/platform-core/src/automation/http_client.rs`).

## Actuator endpoints

When [REST automation](rest-automation.md) is on, four endpoints are added automatically —
unless your `rest.yaml` already claims their URLs, in which case your entry wins (that is how
you attach authentication, CORS, or a different timeout to an actuator):

```text
GET /info
GET /env
GET /health
GET /livenessprobe
```

!!! note "Rust port"
    Two Java defaults are not ported: **`/info/lib`** (Java lists JAR dependencies from the
    archive manifest — a Rust binary has no runtime dependency manifest) and **`/info/routes`**
    (the route listing). XML actuator responses are also not supported — actuators answer in
    JSON. **`POST /api/event` (Event over HTTP) IS ported** (increment 61) and ships in the
    default rest.yaml — see [Event over HTTP](event-over-http.md).

#### `GET /info`

Describes the application: identity, runtime, origin, and uptime.

```json
{
  "app": { "name": "hello-world", "version": "…", "description": "…" },
  "runtime": { "language": "rust", "platform_core": "…" },
  "origin": "…",
  "time": { "start": "…", "current": "…" },
  "up_time": "1 hour 5 minutes 2 seconds"
}
```

The identity comes from configuration: `application.name`, `info.app.description`, and
`info.app.version` (falling back to the platform-core crate version when the application does
not declare its own — a Rust library cannot read the app's build metadata at runtime).

#### `GET /env`

Shows selected environment variables and application parameters — **opt-in lists only, so
secrets are never dumped wholesale**. Nothing appears here unless you name it:

```yaml
show.env.variables: 'GREETING_USER, ENV_NAME'
show.application.properties: 'application.name, log.format, rest.server.port'
```

The response is `{app, env: {environment: {…}, properties: {…}}}` with one entry per listed
name (a missing environment variable shows as an empty string).

#### `GET /health`

Runs the application's health-check functions and aggregates the outcome:

```yaml
mandatory.health.dependencies: 'demo.health'
optional.health.dependencies: 'other.service.health'
```

Each listed route is called twice: header `type=info` (3-second timeout, advisory — whatever
map the service returns is merged into its dependency entry, typically `service` and `href`),
then header `type=health` (10-second timeout, decisive — any status ≥ 400 marks the dependency
down). The response lists every dependency with its `route`, `required` flag, `status_code`,
and message. The overall verdict:

- all mandatory dependencies up → `"status": "UP"` with HTTP **200**;
- any mandatory dependency down → `"status": "DOWN"` with HTTP **400**.

Optional dependencies are reported but never affect the verdict. When neither list is
configured, the response carries a reminder message instead of dependencies. The outcome also
feeds `/livenessprobe`.

#### `GET /livenessprobe`

A lightweight probe for orchestrators: returns the text `OK` (HTTP 200) while the most recent
health outcome was good, otherwise HTTP **400** with
`Unhealthy. Please check '/health' endpoint.`

### Custom health services

A health service is an ordinary composable function that answers the two request types. This
is the shipped `examples/hello-world` implementation:

```rust
#[preload(route = "demo.health")]
struct DemoHealth;

#[async_trait]
impl ComposableFunction for DemoHealth {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        match headers.get("type").map(String::as_str) {
            Some("info") => EventEnvelope::new().set_body(serde_json::json!({
                "service": "demo.store",
                "href": "memory://demo",
            })),
            Some("health") => EventEnvelope::new().set_body("demo store is running"),
            _ => Err(AppError::new(400, "unknown health request type")),
        }
    }
}
```

Return a string or a map from the `health` branch when the dependency is fine; return
`Err(AppError::new(status, message))` when it is not. The `href` in the `info` branch should
tell the operator where the dependency lives (its URL or connection target).

## The async HTTP client (`async.http.request`)

Outbound HTTP is a built-in composable function: build an `AsyncHttpRequest`, send it to the
route `async.http.request`, and await the reply. The client is registered automatically at
startup (500 instances, as an event interceptor) and is untraced by default — it appears in
the `skip.rpc.tracing` list so HTTP-client plumbing does not flood your traces.

```rust
use platform_core::automation::AsyncHttpRequest;

let request = AsyncHttpRequest::new()
    .set_method("GET")
    .set_header("accept", "application/json")
    .set_url("/api/hello/world")
    .set_query_parameter("x1", "y")
    .set_target_host("http://127.0.0.1:8083")
    .set_timeout_seconds(10);

let event = EventEnvelope::new()
    .set_to("async.http.request")
    .set_raw_body(request.to_value());
let response = po.request(event, Duration::from_secs(10)).await?;
// response.status(), response.headers(), response.body()
```

The builder covers the full request contract: `set_method`, `set_url`, `set_target_host`,
`set_header`, `set_body`, `set_query_parameter`, `set_path_parameter` (substituted into
`{param}` tokens in the URL), `set_timeout_seconds`, plus cookie and session key-values. A
failed call comes back **in-band** as a reply with an error status — check
`response.has_error()` / `response.status()` like any other RPC.

### Target host validation and HTTPS

`set_target_host` takes `http(s)://host` or `http(s)://host:port` — protocol and authority
only (default ports 80/443). A target with a URI path, a missing host, or an unknown
protocol is rejected with status 400 (put the path in `set_url`). Allowed methods are `GET`,
`HEAD`, `PUT`, `POST`, `PATCH`, `DELETE`, and `OPTIONS`; anything else returns **405**.

For `https` targets, server certificates verify against the **operating-system trust
store** — corporate CAs installed on the host are honored, matching the Java client's
default-truststore behavior. For a self-signed endpoint (a dev or test system), opt out of
chain validation per request with `set_trust_all_cert(true)` — the same escape hatch as the
Java client's `trust_all_cert` flag. Use it only for endpoints you control: it disables
certificate validation for that call.

**Redirects are never followed** — one request is one HTTP call, and a `3xx` answer comes
back to you like any other response (status, `location` header, body). This is deliberate,
in both the Java and Rust engines: the client serves **backend** applications, and
automatic redirection is a browser-side behavior. Where a backend genuinely needs to
follow a redirect flow (e.g. SSO), handle it in your function — read the status and
`location` from the reply and issue the follow-up call — or declaratively in a flow or
graph that routes on the returned status.

!!! note "Rust port"
    TLS uses [rustls](https://github.com/rustls/rustls) with the OS certificate store
    (increment 48) — behavior matches the Java client, including the `trust_all_cert`
    escape hatch. The Java client's streaming surface is still deferred: object streams
    for upload/download, multipart file upload, and the `X-Small-Payload-As-Bytes`
    optimization are not ported — request and response bodies are always aggregated in
    memory (the client adds an `x-content-length` response header when the origin did
    not send `content-length`).

### Timeouts

Connect timeout
:   `http.client.connection.timeout` in milliseconds (default `5000`, floor 2000). Expiry
    returns status **408** `Connection timeout for host:port`.

Response timeout
:   Per request, via `set_timeout_seconds` (default 30 seconds; it travels as the `x-ttl`
    header in milliseconds). Expiry returns status **408** `Timeout for N ms`. Give the
    surrounding `po.request` at least as long, or the RPC will time out first.

### Request headers and trace propagation

Headers you set are forwarded, except a small managed list that would interfere with the
underlying client (`content-length`, `host`, `connection`, `user-agent`,
`content-encoding`, `transfer-encoding`, `accept-encoding`, `x-stream-id`, and the
`sec-fetch-*` / `upgrade-insecure-requests` browser headers). Header values are trimmed of
surrounding whitespace — a token read from a file with a trailing newline stays valid.

**Trace headers are managed for you.** When the calling context is traced, the client injects
the trace id (header name from `http.trace.id.header`, default `X-Trace-Id`) and a W3C
`traceparent` carrying the current span — so distributed traces stay connected across the HTTP
boundary; do not set these yourself in a traced flow or function. In an untraced call nothing
is injected, and a trace header you set is forwarded as-is — which is what lets you propagate
a trace to a third-party system with full control. The business **correlation id** of the
current request is likewise injected (header name from `http.correlation.id.header`, default
`X-Correlation-Id`) unless you already set that header. Cookies set on the request are folded
into one `cookie` header with URL-encoded values.

### Request body

For `POST`, `PUT`, and `PATCH` (other methods send no body):

- a **map or list** body is serialized as JSON (null entries omitted unless
  `serializer.null.transport=true`);
- a **string** is sent as its UTF-8 bytes;
- **bytes** are sent raw.

!!! note "Rust port"
    The Java client serializes a map body as XML when the content type says so; the XML writer
    is not ported — a map body is always JSON. Set the `content-type` header accordingly.

### Response body decoding

The reply envelope carries the HTTP status, all response headers, and a body decoded by the
response `content-type`:

`application/json`
:   An empty body becomes an empty map. A body wrapped in matching JSON brackets is parsed
    into a map or list; a failed parse (or a non-bracketed body) stays raw text.

`text/*`, `application/javascript`, `application/xml`
:   Text (XML is not parsed into a map — the same parser deferral as the server side).

anything else, or no content type
:   Raw bytes.

### Using the client from a flow

Because the request contract is a plain map, a flow task can call the client with no code —
build the `AsyncHttpRequest` shape through input data mapping:

```yaml
tasks:
  - name: 'http.client'
    input:
      - 'text(/api/hello/world) -> url'
      - 'text(GET) -> method'
      - 'text(http://127.0.0.1:8085) -> host'
      - 'text(application/json) -> headers.accept'
      - 'input.query.x1 -> parameters.query.x1'
    process: 'async.http.request'
    description: 'Call the external endpoint'
    output:
      - 'result -> output.body'
      - 'status -> output.status'
    execution: end
```

`method`, `host`, and `url` are the required keys; `headers.*`, `parameters.query.*`,
`parameters.path.*`, `body`, and `cookies.*` are optional. See the
[Flow Schema Reference](flow-schema-reference.md) for the mapping syntax.

## See also

- [REST Automation](rest-automation.md) — the inbound HTTP boundary these endpoints ride on.
- [Function Execution](event-driven/function-execution.md) — RPC semantics, timeouts, and
  tracing behavior.
- [Observability](observability.md) — what the propagated traces record.

---

*Adapted from the mercury-composable guide `docs/guides/actuators-and-http-client.md`;
behavior verified against this repository's source.*
