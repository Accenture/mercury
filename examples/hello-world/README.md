# hello-world — the platform-core (layer 1) taste

The Rust analog of mercury-composable's README "greeting.demo" example: an
event-driven application declared with annotations, a one-line `main()`, and
everything else in configuration — REST automation, actuators, static content
with etag/304 caching, structured logging with distributed tracing, and a
business correlation-id that travels the whole transaction.

The whole `main()` is:

```rust
platform_core::auto_start_main!();
```

Functions are annotated (`#[preload]`, `#[before_application]`,
`#[main_application]`) and collected at link time — the Java
`AutoStart.main(args)` experience.

## Run it

```bash
cargo run -p hello-world
```

The startup log shows the lifecycle: preflight validation → preloaded
functions (`greeting.demo` ×10, `greeting.api`, `demo.health`,
`http.request.filter`) → REST endpoints on port **8085** → the main
application performing a traced RPC.

Variations:

```bash
# structured-log formats (-D after -- is the JVM -D runtime-override analog)
cargo run -p hello-world -- -Dlog.format=text      # plain console lines
cargo run -p hello-world -- -Dlog.format=json      # pretty JSON with a context block (default here)
cargo run -p hello-world -- -Dlog.format=compact   # single-line jsonl

# configuration via ${ENV:default} substitution in application.yml
GREETING_USER=eric cargo run -p hello-world
```

## Test drive

**The greeting API** — the HTTP edge starts a distributed trace and ensures a
business correlation-id; the response echoes both:

```bash
curl 'http://127.0.0.1:8085/api/greeting/eric'
# {"correlation_id":"...","handled_by_instance":N,"message":"Welcome, eric","trace_id":"..."}

# supply your own business correlation-id — it wins over the generated one
curl -H 'X-Correlation-Id: order-12345' 'http://127.0.0.1:8085/api/greeting/eric'
```

Watch the terminal while you curl: the `greeting.api` and `greeting.demo` log
lines carry a `context` block (cid, trace/span ids), followed by the telemetry
span from `distributed.tracing` — one trace across both functions.

**Actuators** (registered automatically):

```bash
curl 'http://127.0.0.1:8085/info'            # app identity, uptime
curl 'http://127.0.0.1:8085/env'             # selected env vars + config
curl 'http://127.0.0.1:8085/health'          # runs the demo.health dependency check
curl 'http://127.0.0.1:8085/livenessprobe'
```

**Static content with etag/304 and a request filter** — `resources/public/`
is served with SHA-256 etags; entry pages (`/`, `/index.html`) are no-cache;
the `http.request.filter` interceptor inspects matching requests (the SSO
hook) and stamps `x-filter: inspected`:

```bash
curl -i 'http://127.0.0.1:8085/'                       # no-cache + x-filter headers
ETAG=$(curl -sI 'http://127.0.0.1:8085/assets/app.css' | awk -F': ' '/^etag/{print $2}' | tr -d '\r')
curl -s -o /dev/null -w '%{http_code}\n' -H "If-None-Match: $ETAG" \
     'http://127.0.0.1:8085/assets/app.css'            # 304
```

Or just open <http://127.0.0.1:8085/> in a browser and refresh — the second
load revalidates with 304.

**The Event-over-HTTP interop target** — the app publishes a public echo
function under two route names (`hello.world, hello.declarative` — a
comma-separated alias list, one shared handler). It is the standing callee
of the `hello-flow` example's two Event-over-HTTP demo endpoints, and the
drop-in counterpart of the Java `lambda-example` (same port 8085, same
routes) — so the demo doubles as a cross-language interop demo. See the
hello-flow README and the docs site's *Event over HTTP* guide.

Stop with Ctrl-C (graceful shutdown cleans the elastic store).

## Where things live

| File | Role |
|---|---|
| `src/main.rs` | the annotated functions + the one-line main |
| `resources/application.yml` | app name, log format, port, actuator config |
| `resources/rest.yaml` | declarative endpoints — the YAML *is* the router |
| `resources/app-log-context.yaml` | which keys join every structured log line |
| `resources/public/` | static content (etag/304, filter) |
