# Driving the Playground from an HTTP client (the companion endpoint)

The **companion endpoint** lets a script, test harness, or AI agent drive an
already-open Playground session over plain HTTP. There is exactly **one**
companion endpoint — the synchronous `/sync` (the fire-and-forget variant was
retired 2026-09; its bare URL answers 404). The authoritative, always-current
contract for AI drivers is the published **AI agent guide**
(`docs/guides/knowledge-graph/ai-agent-guide.md`); this file is the short
version for webapp developers.

## End-to-end flow

```
1. Open  ws://<host>/ws/graph/playground
2. Read  first inbound frame: { "type": "session", "id": "ws-123456-7" }
3. POST  /api/companion/{id}/sync  with Content-Type: text/plain, ONE command per request
4. Read  the outcome from the HTTP response ({ok, output, error, result})
5. GET   /api/graph/session/{id}  when you want the current graph model as JSON
```

Every output line is also teed to the session's WebSocket console, so a human
watching the browser sees what the driver does, live.

## Request / response

```
POST /api/companion/{id}/sync
Content-Type: text/plain

<any command the Playground CLI accepts>
```

```json
{
  "ok": true,
  "id": "ws-123456-7",
  "command": "create node root ...",
  "output": ["> create node root ...", "node root created"],
  "result": [ ... structured data, e.g. a run's output.body ... ]
}
```

Null fields are omitted: `error` appears only on failure (with the first
failing line), `result` only when the command yields data. Status codes:
`200` executed (read `ok`), `400` missing/empty/non-text body, `404` no
active session — **a 404 means the session is gone** (e.g. the app was
restarted): obtain a fresh session id. And remember: **restarting the app
ends every session — `export graph as {name}` first** or unexported work is
lost.

## Minimal curl example

```bash
SESSION_ID="ws-384729-17"

curl -sS -X POST "http://localhost:8300/api/companion/${SESSION_ID}/sync" \
  -H 'Content-Type: text/plain' \
  --data-binary $'create node root\nwith type Root\nwith properties\nskill=graph.math'
```

The outcome is in the curl response; the same lines appear on the WebSocket.
Current graph state for the live session:

```bash
curl -sS "http://localhost:8300/api/graph/session/${SESSION_ID}"
```

## Command grammar (cheat sheet)

Anything the Playground CLI accepts works. Common building blocks:

| Intent | Command |
|---|---|
| Create a node | `create node <name>` + `with type <Type>` + `with properties` + `k=v` lines |
| Connect nodes | `connect <a> to <b> with <label>` |
| Instantiate graph | `instantiate graph` + optional `text(<v>) -> input.body.<field>` seed lines |
| Run traversal | `run` |
| Inspect state | `inspect <namespace.key>` / `describe graph` |
| List | `list nodes` / `list connections` |
| Persist | `export graph as <name>` / `import graph from <name>` |
| Help | `help` / `help <topic>` / `describe skill <skill>` |

Built-in skills: `graph.math`, `graph.data.mapper`, `graph.api.fetcher`, `graph.task`,
`graph.extension`, `graph.island`, `graph.join`, `graph.suspend`, `graph.resume`
(this Rust port never carried `graph.js`).

## Example prompt — asking an AI agent to build a graph for you

> I have the MiniGraph Playground open at `http://localhost:8300` with
> session id `ws-384729-17`. Use the synchronous companion endpoint
> `POST /api/companion/{id}/sync` (Content-Type: text/plain), exactly one
> command per request. Read each response's `ok`/`error`/`output` and
> self-correct before moving on — do not wait for me to relay the console.
> Validate commands against the published command grammar first.
>
> **Goal:** given an `input.body.person_id`, fetch the person record,
> extract their `name`, and return it as the graph output.
