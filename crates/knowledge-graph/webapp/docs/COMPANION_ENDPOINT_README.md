# Companion Endpoint — Usage Guide

A dev-only REST endpoint that lets an HTTP client (script, test harness, or AI
agent) dispatch commands into an **already-open** MiniGraph Playground
WebSocket session. Output continues to stream to the browser console.

> Design and rationale: see [COMPANION_ENDPOINT_SPEC.md](COMPANION_ENDPOINT_SPEC.md).

---

## When to use it

- You have the Playground open in a browser and want an agent (or a curl
  script) to drive graph-building commands while you watch the output.
- You want to automate a reproducible graph setup without re-typing commands.
- You're writing an integration test that needs to issue commands as if a
  human were typing into the Playground CLI.

This endpoint is **not** a headless way to run MiniGraph — it requires a live
WebSocket session. It is also gated by `@OptionalService("app.env=dev")` and
is not registered in production.

---

## End-to-end flow

```
1. Open  ws://<host>/ws/graph/playground
2. Read  first inbound frame: { "type": "session", "id": "ws-123456-7" }
3. POST  /api/companion/{id}  with Content-Type: text/plain  and the command in the body
4. GET   /api/graph/session/{id}  when you want the current graph model as JSON
5. Watch output appear on the WebSocket (HTTP response is just an ack)
```

### Request

```
POST /api/companion/{id}
Content-Type: text/plain

<any command the Playground CLI accepts>
```

- `{id}` is the public session id from the WS welcome message (format
  `ws-<6 digits>-<counter>`).
- Body may be single-line or multi-line. Commands match the Playground CLI
  grammar exactly.

### Response (always 200 on dispatch success)

```json
{
  "type": "companion",
  "status": "accepted",
  "id": "ws-123456-7",
  "message": "Command dispatched to graph.command.service. Output streams to the WebSocket console for this session."
}
```

- `400` — missing `id`, missing body, empty body, or non-text body.
- `404` — no active session for `id` (WebSocket not open, or already closed).
- **Command syntax errors still return 200.** The error text (`ERROR: ...`
  or `Please try 'help' for details`) shows up on the WebSocket, not in the
  HTTP response. Phase 2 will add opt-in synchronous capture.

---

## Minimal curl example

```bash
# After reading the session id from the WS welcome frame:
SESSION_ID="ws-384729-17"

curl -sS -X POST "http://localhost:8300/api/companion/${SESSION_ID}" \
  -H 'Content-Type: text/plain' \
  --data-binary $'create node root\nwith type Root\nwith properties\nskill=graph.math'
```

Output (the confirmation `"node root created"`) appears on the open
WebSocket, not in the curl response.

To check the current graph state for that same live session, use the
live-session graph endpoint instead of sending `describe graph` just to fetch
JSON:

```bash
curl -sS "http://localhost:8300/api/graph/session/${SESSION_ID}"
```

This returns the active graph model for the open Playground session as JSON.
Use it when a script, test harness, or agent needs to inspect the current
graph state directly.

---

## Command grammar (cheat sheet)

Anything the Playground CLI accepts works. Common building blocks:

| Intent | Command |
|---|---|
| Create a node | `create node <name>` + `with type <Type>` + `with properties` + `k=v` lines |
| Connect nodes | `connect <a> to <b> with <label>` |
| Instantiate graph | `instantiate graph` + optional `text(<v>) -> input.body.<field>` seed lines |
| Run traversal | `run` |
| Inspect state | `inspect <node>` / `inspect model` / `describe graph` |
| List | `list nodes` / `list connections` |
| Persist | `export graph as <name>` / `import graph from <name>` |
| Help | `help` / `help <topic>` / `describe skill <skill>` |

Built-in skills: `graph.math`, `graph.data.mapper`, `graph.js`,
`graph.api.fetcher`, `graph.extension`, `graph.island`, `graph.join`.

---

## Example prompt — asking Claude to build a graph for you

Paste something like this into Claude with the Playground open in your
browser and the session id handy:

> I have the MiniGraph Playground open at `http://localhost:8300` with
> session id `ws-384729-17`. Use the companion endpoint at
> `POST /api/companion/{id}` (Content-Type: text/plain) to build the
> following graph for me, one command per request, and wait for me to
> confirm on the WebSocket console between steps:
>
> **Goal:** given an `input.body.person_id`, fetch the person record,
> extract their `name`, and return it as the graph output.
>
> Nodes I want:
> 1. `root` — type `Root`, skill `graph.math` (entry point only).
> 2. `fetcher` — skill `graph.api.fetcher`, fetches a Person by
>    `person_id` using whatever data-dictionary stub is appropriate.
> 3. `mapper` — skill `graph.data.mapper`, maps
>    `fetcher.response.name -> end.message`.
> 4. `end` — type `End`.
>
> Connect `root -> fetcher` with `fetch`, `fetcher -> mapper` with `map`,
> `mapper -> end` with `done`. Then `instantiate graph` with
> `int(42) -> input.body.person_id`, `run`, and then call
> `GET /api/graph/session/{id}` to show me the current graph state. If any
> command errors on the WebSocket, stop and ask
> me before continuing.

That prompt gives me enough to (a) build each node with the correct
multi-line command, (b) issue the connects in order, (c) seed and run the
graph, and (d) pause for your feedback so mistakes don't cascade.

---

## Gotchas

- **Session must exist first.** The endpoint does not create sessions; it
  piggybacks on an open WebSocket. Close the tab → 404.
- **HTTP response is just an ack.** Watch the WS for actual command output.
- **One command per POST.** Multi-line is fine, but multiple independent
  commands must be separate requests.
- **No auth.** Dev-only by design. Do not expose beyond `localhost` /
  trusted dev environments.
- **Single operator.** No locking — don't have a human typing in the
  browser at the same wall-clock instant an agent is POSTing.
