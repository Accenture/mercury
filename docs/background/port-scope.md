# Port Scope & Fidelity

mercury is a **faithful Rust re-implementation** of
[mercury-composable](https://github.com/Accenture/mercury-composable) (Java). This page states
exactly what that means: what is ported, what is deliberately not, and how fidelity is kept
honest.

## The philosophy: map, don't mirror

The Java project remains the **canonical behavior specification**. This port maps its behavior
onto idiomatic Rust rather than transliterating Java code — same contracts, same configuration
files, same flow YAML, same graph models; different platform idioms where the platforms differ.
Every deliberate difference is **documented where you would meet it**: look for the
`!!! note "Rust port"` boxes throughout this site. There is no silent divergence.

The relationship runs both ways: several capabilities originated in this port and were
contributed upstream to the Java engine — the synchronous AI-companion `/sync` endpoint, the
read-only companion session rule, the discovery commands (`list graphs` / `list flows`), the
join-barrier completion semantics, and the numeric-promotion + `f:round` plugin enhancements.
The two engines keep **byte-identical REST contracts** where they share surfaces.

## Fully ported

- **platform-core** — functions, route names, `EventEnvelope`, the in-memory event bus with
  per-route worker pools, `Platform`/`PostOffice`, configuration management (file syntax kept
  verbatim so config files port unchanged), structured logging, distributed tracing, REST
  automation, actuators, the async HTTP client, WebSocket services.
- **Event Script** — the complete flow engine. Flow YAML is **identical** to the Java
  original and is validated against the canonical Java test fixtures.
- **Active knowledge graph** — the graph engine, all built-in skills (one exception below),
  the Playground web application, and the AI-companion endpoints.

## Deliberately out of scope

These are **scope decisions**, not gaps — they will not be ported:

- **The Kafka service mesh** — the service-discovery and sync-over-Kafka layer: the connector
  tree (cloud connector, service monitor, Kafka connector, Kafka presence). mercury's core is
  a **single-runtime** platform: one process, one in-memory event bus. Consequences ripple
  predictably: no broadcast/multicast, no mesh configuration keys, no presence protocol.
- **Spring adapters** (`rest-spring-3/-4`) — Spring is a Java framework. The port's HTTP
  boundary is platform-core's own REST automation, on [hyper](https://hyper.rs) — deliberately
  not a web framework, because `rest.yaml` *is* the router.
- **`graph.js`** — the Java engine runs full JavaScript on GraalVM; this port **retires the
  skill for security reasons**. The runtime rejects it with a pointer to `graph.math` (inline
  compute/branch) and `graph.task` (a composable function for anything richer).

## Deferred — not yet, not never

Present in the Java platform, absent here today, and candidates for future increments:

- **`minimalist-kafka` and `twin-kafka`** — the **lightweight, cloud-native Kafka
  connectors**. Unlike the mesh above, these are planned ports: they carry events between
  runtimes without the mesh's discovery layer. (This is why HTTP-specific configuration keys
  keep their `http.` prefix — connector-specific counterparts arrive with them.)
- **sync-over-async** — the request/response bridge over asynchronous transports.
- HTTPS in the async HTTP client (a TLS stack), HTTP relay / `url_rewrite`, A/B dual service.
- Multipart file upload and request/response streaming at the REST boundary.
- `/info/lib` and `/info/routes` actuators; the ready-made OpenTelemetry OTLP forwarder
  extension (the `distributed.trace.forwarder` extension point itself is ported).
- Fork-n-join parallel RPC on `PostOffice` (Event Script `fork`/`join` and `tokio::join!`
  cover the pattern today).

Each of these carries a `!!! note "Rust port"` box on the page that documents the surrounding
feature.

## How fidelity is kept honest

- **Canonical fixtures:** the Event Script engine and the graph tutorials run against test
  fixtures taken from the Java project; behavior differences fail tests, not users.
- **Side-by-side traceability:** every ported module's documentation names the Java class it
  ports, so reviewers can diff behavior directly.
- **Engine-verified documentation:** the AI documentation set was hardened by a 13-tutorial
  validation sweep in which fresh AI agents built every tutorial from the docs alone — the
  final eight passed on the first attempt with zero in-band lookups.
- **Fidelity fixes flow upstream:** when a behavioral gap is found in either engine, it is
  fixed in both, with the shared contract kept identical.

## Performance

The ported event bus benchmarks at roughly **155K RPC round-trips per second at ~6 µs**
latency on commodity hardware — about 8× the Java engine's published record — while keeping
the same programming model. Performance is a byproduct of the platform choice, not a fork in
behavior.

*This page states the port scope confirmed by the project maintainer; the Java project's
documentation remains authoritative for the features outside this port's scope.*
