# distributed-cache-example

The worked example of the **distributed cache** (`mercury-distributed-cache`): the same profile
GET/POST/DELETE CRUD exposed **three times, one route family per layer**, all backed by ONE shared
Redis cache (`v1.cache.redis`). It is the Rust twin of the Java engine's
`examples/distributed-cache-example` — same port, same routes, same value format — which makes it
the **cross-engine interop harness**: a profile written by either engine reads back through the other.

| Layer | Route | How the cache is reached |
|---|---|---|
| 1 — Platform Core | `GET/POST/DELETE /api/l1/profile/{profile_id}` | one function (`v1.profile.l1`) calls `v1.cache.redis` with the PostOffice RPC API, in code |
| 2 — Event Script | `GET/POST/DELETE /api/l2/profile/{profile_id}` | ONE flow (`resources/flows/l2-profile.yml`) derives the action from the HTTP method and composes the cache task with the encode/decode helpers, declaratively |
| 3 — Knowledge Graph | `POST /api/graph/profile-cache` with `{ "action": get \| save \| delete, "id", "profile"? }` | ONE graph (`resources/graph/profile-cache.json`) drives the same route from `graph.task` nodes through the **standard graph endpoint** — a Layer 3 app needs no endpoint of its own |

The flow and the graph files are **byte-identical** to the Java example's: orchestration is
configuration, and configuration ports unchanged.

## The shared value: a MsgPack-packed map

The cache stores opaque bytes; this example packs the profile map with **standard MsgPack** — no
`EventEnvelope` wrapper, no type tags — which is exactly the Java `MsgPack.packMapOrList` form. The key
is the raw profile id under the app namespace `redis.cache.key.prefix=cache-demo:`. So a profile POSTed
through Layer 1 here reads back through Layer 2 and Layer 3 here, **and** through any layer of the Java
example pointed at the same Redis.

## What's app-specific vs. reused

Reused, unchanged: the cache function and its `redis.health` check (the `mercury-distributed-cache`
crate), the Redis client layer (`mercury-redis-connection`), the `graph-executor` flow (the engine's,
listed in `flows.yaml` and resolved from the engine crate's resources), the Playground and companion
services in dev mode.

App-specific: `v1.profile.l1` (Layer 1), `v1.profile.encode` / `v1.profile.decode` (pack / unpack the
profile), `v1.http.method.action` (Layer 2's method-to-action mapper), `v1.profile.exception` (renders
the standard error body), plus `rest.yaml`, `flows.yaml` + `l2-profile.yml`, `graphs.yaml` +
`profile-cache.json` and `application.yml`.

## Run it

Start a Redis (the Java repository's `helpers/redis-standalone`, or any server), then:

```bash
cargo run -p distributed-cache-example            # port 8305; REDIS_HOST / REDIS_PORT override 127.0.0.1:6379
```

Exercise one layer (Layer 1 shown; Layer 2 is identical on `/api/l2/…`):

```bash
curl -s -X POST localhost:8305/api/l1/profile/42 -H 'content-type: application/json' \
     -d '{"name":"Carol","email":"carol@example.com"}'      # 201 {"id":"42","layer":1,"status":"stored"}
curl -s localhost:8305/api/l1/profile/42                     # 200 {"name":"Carol","email":"carol@example.com"}
curl -s -X DELETE localhost:8305/api/l1/profile/42           # 200 {"id":"42","layer":1,"deleted":true}
curl -s localhost:8305/api/l1/profile/42                     # 404 {"type":"error","status":404,"message":"Profile not found"}
```

Layer 3 — one endpoint for every graph, the action is in the payload:

```bash
curl -s -X POST localhost:8305/api/graph/profile-cache -H 'content-type: application/json' \
     -d '{"action":"save","id":"42","profile":{"name":"Erin","email":"erin@example.com"}}'
curl -s -X POST localhost:8305/api/graph/profile-cache -d '{"action":"get","id":"42"}'
curl -s -X POST localhost:8305/api/graph/profile-cache -d '{"action":"delete","id":"42"}'
curl -s -X POST localhost:8305/api/graph/profile-cache -d '{"action":"purge","id":"42"}'   # 400 Invalid action. Use get, save or delete
```

The interop demo — write on one layer (or one engine), read on another:

```bash
curl -s -X POST localhost:8305/api/l1/profile/7 -d '{"name":"Dave"}'   # Layer 1 writes
curl -s localhost:8305/api/l2/profile/7                                # Layer 2 reads the same profile
curl -s -X POST localhost:8305/api/graph/profile-cache -d '{"action":"get","id":"7"}'   # so does Layer 3
```

Run the Java example against the same Redis (its port is 8305 too — start one at a time, or set
`rest.server.port`) and read `7` there: same key, same bytes, same profile.

## Co-author the graph with an AI agent (dev mode)

`app.env=dev` (the default here; `APP_ENV=prod` turns it off) registers the Playground and companion
services, so the Playground UI is at `http://localhost:8305/` and an agent can host a session with
`scripts/playground-session-broker.mjs` (see `scripts/README.md`). What promotes to production is the
graph; the authoring surface is switched off behind it.

## Test it

```bash
cargo test -p distributed-cache-example
```

The suite boots the whole application against the in-process RESP double (no Redis binary, no
Docker): the CRUD cycle on Layers 1 and 2, the Layer 3 graph including its closed dispatch table, the
cross-layer interop, and the stored value's plain-MsgPack shape under the `cache-demo:` namespace.
