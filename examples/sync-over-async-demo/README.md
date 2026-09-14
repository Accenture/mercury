# sync-over-async-demo — cross-pod progressive rendering

The streaming-return-route demo (Rust twin of the Java `sync-over-async-demo`
stream roles): in a horizontally scaled deployment, the pod that *produces*
progressive events is generally not the pod holding the user's HTTP
connection. The [streaming return route](../../draft-design-specs/sync-over-async-port.md)
closes that gap with **Redis alone — no broker anywhere**.

One binary plays either side, selected by an active profile:

| Role | Port | What runs |
|------|------|-----------|
| `stream-ui` (pod A) | 8600 | return-route coordinator + `StreamBridge` facade behind `GET /api/notifications` (`stream: true`, SSE) |
| `stream-producer` (pod B) | 8601 | `StreamResponder` only — no coordinator, no subscriber — behind `POST /api/produce` |

The client opens the SSE channel on pod A; the facade announces the session's
correlation id as the first SSE event (`event: cid`); the client quotes that
cid in POSTs to pod B, which posts segments into the rendezvous purely through
Redis. Both roles register `soa.redis.health`, so each pod's `/health` shows
the Redis probe.

## Run it (three terminals)

Redis first — the Java repo's `redis-standalone` helper is a plain TCP dev
server (no Docker; `x.y.z` = the current version in that repo):

```shell
java -jar redis-standalone-x.y.z.jar
```

Then the two pods, from this repository's root:

```shell
cargo run -p sync-over-async-demo -- -Dapp.profiles.active=stream-ui
cargo run -p sync-over-async-demo -- -Dapp.profiles.active=stream-producer
```

Open a channel (the timestamped SSE client; the optional second argument sets
the idle allowance in seconds via `x-stream-idle-seconds`):

```shell
node scripts/sse-client.mjs http://127.0.0.1:8600/api/notifications
```

Copy the announced cid, then drive the producer:

```shell
curl -s -X POST http://127.0.0.1:8601/api/produce \
     -H 'content-type: application/json' \
     -d '{"cid": "<cid>", "mode": "chat"}'
```

Five tokens and the `done` terminal render progressively on the SSE side —
**no sequence number exists anywhere in the pipeline**: order is posting
discipline → Redis list order → serialized drain → the edge's ordered reply
lane.

## Produce modes

| Body | Behavior |
|------|----------|
| `{"cid", "mode": "chat"}` | five ordered tokens + `eof` (the AI-chat shape) |
| `{"cid", "mode": "notify", "name": "orders", "body": "order 42 shipped"}` | one named event, as an uncoordinated backend service would emit |
| `{"cid", "mode": "close"}` | terminal `eof` — any producer may close the channel |
| `{"cid", "mode": "stall"}` | two tokens, NO terminal (producer-death chaos) |
| `{"cid", "mode": "lost", "type": "data\|eof", "name", "body"}` | **chaos**: store the segment but suppress its wake-up (a lost notification) |

Every response reports `live`: `false` means the rendezvous is over (the UI
pod closed, timed out, or died — the orphan contract), which is the producer's
signal to stop.

## The chaos runbook

The five scenarios of the cross-pod test report
([docs/test-reports/streaming-return-route-cross-pod.md](../../docs/test-reports/streaming-return-route-cross-pod.md)),
reproducible end to end:

1. **Ordered tokens across pods** — open a channel, produce `chat`: exact
   order, terminal metadata, rendezvous closed.
2. **Lost notification, healed by the next drain** — produce `notify`
   (renders), then `lost` (stored, nothing renders), then `notify` again: the
   second wake-up's drain delivers both queued segments in list order. A
   dropped signal costs latency, never data.
3. **Lost close, recovered by the final drain** — open with a short idle
   allowance (`node scripts/sse-client.mjs <url> 6`), then `lost` a data
   segment AND a `lost` `eof`: nothing wakes pod A, and at idle expiry the
   facade watchdog's single final drain completes the render normally.
4. **Producer killed mid-stream** — produce `stall`, then `kill -9` pod B: no
   terminal can ever arrive, so the watchdog fails the render in-band
   (`event: error`, 408 `Stream idle timeout`) and deletes the rendezvous keys.
5. **UI pod killed** — run pod A with a short route TTL
   (`-Dsync.stream.ttl.seconds=30`), open a channel, `kill -9` pod A: a post
   inside the TTL window is accepted into the void (`live: true` — the
   TTL-bounded crash backstop), and the producer stops (`live: false`) as soon
   as the route expires. Bounded, deterministic, no liveness machinery.
