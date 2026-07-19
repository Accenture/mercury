---
title: AI agent guide — building graphs via the companion endpoint
summary: The authoritative context an AI agent needs to build Active Knowledge Graphs
  deterministically — the companion-endpoint contract, how to use the command grammar as the
  source of truth, a validation checklist, and a canonical build recipe.
layer: knowledge-graph
audience: [ai-agent, developer]
keywords: [ai companion, companion endpoint, context engineering, deterministic, minigraph, websocket, graph generation]
related:
  - guides/knowledge-graph/command-reference.md
  - guides/knowledge-graph/minigraph-commands.json
  - guides/knowledge-graph/skills-reference.md
---

# AI agent guide — building graphs via the companion endpoint

> **At a glance**
>
> - **Read this if you are an AI agent** asked to build or modify a MiniGraph. It is the
>   single context you need — you should **not** need to read the engine source.
> - **Generate from rules, not guesses.** The [command grammar](command-reference.md) and its
>   machine-readable form [`minigraph-commands.json`](minigraph-commands.json) are the source of
>   truth. Validate every command against them before sending.
> - **Two endpoints, two jobs** — see below. Both are **dev-only** (`app.env=dev`), no auth.

## Which endpoint? {#endpoints}

| Goal | Endpoint | Notes |
|---|---|---|
| **Execute a deployed graph** | `POST /api/graph/{graph-id}` | Send the request body; get the response. No session. |
| **Build/edit a graph — AI agents, preferred** | `POST /api/companion/{session-id}/sync` | **Synchronous** — returns the command outcome **in-band** `{ok, output, error, result}`; output is *also* teed to the human's WS console. |
| **Build/edit a graph — fire-and-forget (legacy)** | `POST /api/companion/{session-id}` | Dispatches the command; outcome streams to the WS console only (HTTP returns just an ack). Kept for Java parity. |
| **Read the live model** | `GET /api/graph/session/{session-id}` | Returns the current graph as JSON. |

This guide is about the **companion** flow — co-authoring a graph with a human watching the
Playground.

## The companion contract {#contract}

An **AI agent should use the synchronous `/sync` endpoint** so it sees every outcome in-band and
can self-correct without a human relaying the console:

```
1. A human opens the Playground (ws://{host}/ws/graph); the first WebSocket frame carries the
   session id (ws-<6 digits>-<counter>, e.g. ws-384729-17). Get this id from the human.
2. For each command:  POST /api/companion/{session-id}/sync   Content-Type: text/plain,
   exactly ONE command in the body.
3. The HTTP response returns the outcome IN-BAND as JSON:
     { "ok": bool, "id": "ws-...", "command": "...", "output": ["...console lines..."],
       "error": "...", "result": [ ... structured, e.g. a run's output.body ... ] }
   - NULL FIELDS ARE OMITTED from the wire (serializer null-omission): a success carries no
     "error" key and, unless the command yields data, no "result" key. Treat ABSENT as null —
     do not require the keys.
   - If ok is false, read error/output, fix it, and re-issue — self-correct; no human relay needed.
   - Use result to verify a run/inspect (e.g. output.body).
4. The same output is ALSO teed to the human's WebSocket console, so a watcher — and any
   `session subscribe`d session (e.g. a product owner) — sees it live: real-time human+AI collaboration.
5. Read the model shape any time with  GET /api/graph/session/{session-id}.
```

Status codes: `200` executed (read `ok`/`error` in the body); `400` missing/empty/non-text body;
`404` no active session for that id.

> **Legacy fire-and-forget** (`POST /api/companion/{session-id}`, no `/sync`): returns only
> `{status:"accepted"}`; the outcome streams to the WS console, not the HTTP response, so the caller
> is **blind to errors**. Prefer `/sync`.

**Rules of engagement:** one command per POST (multi-line commands are fine — see the grammar);
the session must already be open (you do not create it); single operator — don't POST while a
human is typing in the same instant; never expose this beyond a trusted dev host.
**Session topology is off-limits:** a companion is an *assistant to* the session in the URL, not a
WebSocket session of its own — both companion endpoints reject `session subscribe` /
`session unsubscribe` / `session reset` (the read-only `session` status query is allowed).
Subscriptions are managed from WebSocket-connected sessions only.

## Generate deterministically {#deterministic}

1. **Use the grammar as source of truth** — [`command-reference.md`](command-reference.md) for the
   rules, [`minigraph-commands.json`](minigraph-commands.json) to look up a command's exact syntax,
   params, and allowed values. Do not infer syntax from a single example.
2. **Validate before sending** — check each command against this list (the engine's
   [invariants](command-reference.md#invariants)):

> **Pre-send checklist**
> - [ ] The root node is named `root`; the end node is named `end`.
> - [ ] Node names and types are **lowercase + hyphen** only.
> - [ ] Each node has **0 or 1** skill (`skill={route}`); the skill's required properties are present
>       (see the [skill→property matrix](command-reference.md#skill-matrix)).
> - [ ] Every node *in the traversal path* connects to ≥1 node (or `export` fails).
> - [ ] **No node is left unconnected.** Config nodes (`Dictionary`/`Provider`) are referenced by
>       name (`dictionary[]=`, `provider=`) and not traversed — wire them under a `graph.island`
>       (`root -[contains]-> island -[data]-> dictionary -[provider]-> provider`): the island is
>       the graph's entity-relationship knowledge layer
>       ([required convention](command-reference.md#island)).
> - [ ] Multi-line commands (`create`/`update`/`instantiate`) are sent as one block; multi-line
>       *values* use `'''…'''`.
> - [ ] `instantiate graph` precedes `run`/`execute`/`inspect`.
> - [ ] `{…}` in a syntax line is a **placeholder** — substitute the value and do **not**
>       type the braces (`inspect output.body`, not `inspect {output.body}`; `execute fetcher`,
>       not `execute {node}`).
> - [ ] Exactly **one** command per POST.

## Canonical build recipe {#recipe}

A reliable order for building a graph:

1. **Plan** the nodes and the connections (root → … → end) before issuing commands.
2. **Create nodes:** `create node root` (type `Root`), the active/skill nodes, and `create node end`
   (type `End`, usually with `graph.data.mapper` to shape `output.body`).
3. **Connect** them so traversal flows root → end, with no orphans.
4. **Wire the knowledge layer:** whenever the graph has `Dictionary`/`Provider` or data-entity
   nodes, an `Island` (`skill=graph.island`) is **required** — connect
   `root -[contains]-> island -[data]-> dictionary -[provider]-> provider`; **no node is left
   unconnected**. For a graph with none, an island with data-entity nodes documenting the domain
   is **encouraged** ([convention](command-reference.md#island)).
5. **Instantiate** with mock input: `instantiate graph` + `{constant} -> input.body.{key}` lines.
6. **Run and inspect:** `run` (or `execute {node}`), then `inspect output.body`; iterate.
   (`{node}` is a placeholder — you write e.g. `execute fetcher`, `inspect output.body`.)
7. **Export & deploy:** `export graph as {name}`, deploy the JSON, then call
   `POST /api/graph/{name}`.

## Worked example {#example}

Building the hello-world graph via the **synchronous** `/sync` endpoint, one command per request.
Each call returns `{ok, id, command, output}` plus `error`/`result` when non-null — check `ok` and
self-correct on failure:

```bash
SID="ws-384729-17"   # from the WebSocket welcome frame

curl -sS -X POST "http://{host}/api/companion/${SID}/sync" -H 'Content-Type: text/plain' \
  --data-binary $'create node root\nwith type Root\nwith properties\npurpose=demo'
# → {"ok":true,"id":"ws-384729-17","command":"create node root...","output":["> create node root...","node root created"]}
#   (no "error"/"result" keys on success — null fields are omitted)

curl -sS -X POST "http://{host}/api/companion/${SID}/sync" -H 'Content-Type: text/plain' \
  --data-binary $'create node end\nwith type End\nwith properties\nskill=graph.data.mapper\nmapping[]=text(hello world) -> output.body'

curl -sS -X POST "http://{host}/api/companion/${SID}/sync" -H 'Content-Type: text/plain' \
  --data-binary 'connect root to end with done'

curl -sS -X POST "http://{host}/api/companion/${SID}/sync" -H 'Content-Type: text/plain' \
  --data-binary 'instantiate graph'

curl -sS -X POST "http://{host}/api/companion/${SID}/sync" -H 'Content-Type: text/plain' \
  --data-binary 'run'
# → {"ok":true,...,"result":[{"output":{"body":"hello world"}}]}   # the run outcome, in-band
```

Because each response carries `ok`/`error`/`result`, an agent verifies and corrects **itself** — no
need to relay the WebSocket console. The same lines are still teed to the human's console, so a
watcher (and any `session subscribe`d session) follows along live.

## See also {#see-also}

- [MiniGraph command grammar](command-reference.md) + [`minigraph-commands.json`](minigraph-commands.json) — the source of truth.
- [Built-in skills reference](skills-reference.md) — per-skill properties and examples.
