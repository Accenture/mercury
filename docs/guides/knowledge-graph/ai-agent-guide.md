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
   - The "ok" flag is derived from the console lines with whole-output context (import's
     normal "Graph model not found in /tmp/... Found deployed graph model" fallback is
     correctly reported ok:true). When ok is false, the error field carries the first
     failing line — still read the output for the full picture.
   - A malformed command answered with a "Syntax: ..." usage hint is a FAILURE: ok:false
     with the hint as the error (the command did nothing).
   - Repeating an identical command back-to-back is safe: /sync commands are never
     dedup-dropped (the engine's 1-second duplicate guard protects only the WS UI path).
   - If ok is false, read error/output, fix it, and re-issue — self-correct; no human relay needed.
   - Use result to verify a run/inspect (e.g. output.body).
4. The same output is ALSO teed to the human's WebSocket console, so a watcher — and any
   `session subscribe`d session (e.g. a product owner) — sees it live: real-time human+AI collaboration.
5. Read the model shape any time with  GET /api/graph/session/{session-id}.
```

Status codes: `200` executed (read `ok`/`error` in the body); `400` missing/empty/non-text body;
`404` no active session for that id.

> **Retired:** the fire-and-forget `POST /api/companion/{session-id}` (no `/sync`) was removed in
> 2026-09, lock-step with the Java engine — it returned only `{status:"accepted"}`, leaving the
> caller **blind to errors**. The bare URL answers 404. There is exactly one companion endpoint:
> `/sync`.

**Rules of engagement:** one command per POST (multi-line commands are fine — see the grammar);
the session must already be open (over the companion endpoint you do not create it — but see
[Hosting the session yourself](#hosting)); take turns — co-editing with humans is the design
intent, just don't POST in the same instant a human is mid-keystroke; never expose this beyond a
trusted dev host.
**Session topology is off-limits over HTTP:** a companion is an *assistant to* the session in the
URL, not a WebSocket session of its own — the companion endpoint rejects `session subscribe` /
`session unsubscribe` / `session reset` (the read-only `session` status query is allowed).
Subscriptions are managed from WebSocket-connected sessions only.
**Session sync is symmetric:** when sessions are joined by `session subscribe`, **every command
except the `session` topology commands propagates to the primary and all subscribers alike** —
AI and humans are equal co-authors of one shared model, and anyone's command (typos included) is
seen by everyone. It is collaboration, not a one-way broadcast.

## Hosting the session yourself {#hosting}

The flow above borrows a human's browser session. The stronger topology is the inverse: **the
agent hosts the session and humans subscribe to it.** If a human's tab drops (backgrounded past
the idle timeout, laptop lid closed), they simply re-subscribe and the current work-in-progress
graph syncs back to them — nothing lives in anyone's browser.

The WebSocket contract a host needs (identical in the Java and Rust engines):

1. Connect to `ws://{host}/ws/graph/playground`.
2. On open, send `{"type":"welcome"}`.
3. The server announces the id as a plain-text frame: `session ws-NNNNNN-N started`.
4. Keep-alive: send `{"type":"ping","message":"keep alive","time":"..."}` on an interval
   (the web UI uses 20 s); the server answers `{"type":"pong"}`. Filter ping/pong frames from
   any console you render.
5. A restart of the app destroys the session (and any unexported graph — export first);
   reconnect and parse the **new** id.

You do not need to implement this: the example ships a zero-dependency reference,
`scripts/playground-session-broker.mjs` (Node ≥ 22). It holds the session, keeps it alive,
auto-reconnects across app restarts, and exposes a localhost control API (`GET /session`,
`GET /console`, `POST /start`, `POST /stop`) so the agent reads the session id over HTTP, hands
it to the humans (`session subscribe {id}` in their browsers), and keeps driving commands
through `/sync` as usual. See `scripts/README.md` in the example.

## Generate deterministically {#deterministic}

1. **Use the grammar as source of truth** — [`command-reference.md`](command-reference.md) for the
   rules, [`minigraph-commands.json`](minigraph-commands.json) to look up a command's exact syntax,
   params, and allowed values. Do not infer syntax from a single example.
2. **Validate before sending** — check each command against this list (the engine's
   [invariants](command-reference.md#invariants)):

> **Pre-send checklist**
> - [ ] The root node is named `root`; the end node is named `end`.
> - [ ] The root node's properties include `name={graph-id}` and `created={current timestamp}` —
>       export authorization and content-uniqueness (see the recipe's best practice).
> - [ ] Node names are **lowercase letters, digits and hyphen** (types: descriptive labels, conventionally Capitalized).
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

1. **Plan** the nodes and the connections (root → … → end) before issuing commands. Composing
   by delegation? **Discover the valid targets first**: `list graphs` (deployed graph models,
   with each root's purpose) and `list flows` (Event Script flows), then
   `describe graph {graph-id}` for the chosen model's **contract view** (its `input.*` /
   `output.*` data surface) — read-only commands, so no out-of-band brief and no trial
   execution are needed for `extension=` targets.
2. **Create nodes:** `create node root` (type `Root`), the active/skill nodes, and `create node end`
   (type `End`, usually with `graph.data.mapper` to shape `output.body`).
   **Best practice — give the root node these two properties at creation time:**
   ```
   name={graph-id}          # the id you will export/deploy as
   created={ISO timestamp}  # e.g. 2026-09-03T18:55:00Z — the current time
   ```
   `name=` is what authorizes `export graph as {graph-id}` to overwrite an existing file —
   without it the export is refused (or, in older engines, silently skipped); `created=` makes
   every build's model content-unique, so a re-export is always recognized as a new graph and
   always saved. Set both up front and export friction disappears.
3. **Connect** them so traversal flows root → end, with no orphans.
4. **Wire the knowledge layer:** whenever the graph has `Dictionary`/`Provider` or data-entity
   nodes, an `Island` (`skill=graph.island`) is **required** — connect
   `root -[contains]-> island -[data]-> dictionary -[provider]-> provider`; **no node is left
   unconnected**. For a graph with none, an island with data-entity nodes documenting the domain
   is **encouraged** ([convention](command-reference.md#island)).
5. **Instantiate** with mock input: `instantiate graph` + `{constant} -> input.body.{key}` lines.
6. **Run and inspect:** `run` (or `execute {node}`), then `inspect output.body`; iterate.
   (`{node}` is a placeholder — you write e.g. `execute fetcher`, `inspect output.body`.)
7. **Export & deploy:** export is file-based — it writes `/tmp/graph/{name}.json`. **Delete any
   stale file of that name first** (`rm -f /tmp/graph/{name}.json`): an export onto an existing
   file can report `ok: true` while writing **nothing** (overwriting requires `name={name}` on
   the root node, and exporting as an already-deployed graph id never writes). Then
   `export graph as {name}` and **verify the file now exists with fresh content** — never trust
   the success line alone. (With the root-node `name=`/`created=` best practice from step 2 the
   export overwrites cleanly; the delete-and-verify here is defense in depth.) Deploy the JSON
   into your project's `resources/graph/`, list the id in `graphs.yaml`, rebuild, restart, then
   call `POST /api/graph/{name}`.

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

## Scaffolding a project from the example {#scaffolding}

Start every knowledge-graph project from `examples/minigraph-playground` and trim — **against
this manifest, not against your build passing** (`cargo test` and `curl` both stay green with
Playground UI routes missing; only the browser notices).

**Boilerplate manifest** — what a derived project keeps:

| File | Role | Trim? |
|---|---|---|
| `Cargo.toml` | Build; set your own package name | keep (edit ids) |
| `resources/application.yml` | App name, `rest.server.port`, `app.env: dev` for the Playground | keep (edit values) |
| `resources/rest.yaml` | REST routes | keep — trim **by profile, below** |
| `resources/graphs.yaml` | The CompileGraph deployment gate — list every graph id you serve, and point `location:` at your own models | replace with yours |
| Your `resources/graph/*.json` models | Deployed graph models | yours |
| `src/main.rs` | App entry point | keep (adapt) |

(The `graph-executor` flow binding is engine-provided in the Rust engine — unlike the Java
template there is no `flows.yaml` to carry over.)

**`rest.yaml` — two named profiles.** The example's route list mixes three kinds of routes;
know which bar you are building to:

- **Example-specific (always safe to drop):** `mock.mdm.profile`, `mock.account.details` —
  they belong to the support-triage demo, not the platform.
- **Profile `headless-minimal`** — enough for CI, `curl`, and an agent-driven dry-run over
  `/sync`; the Playground **UI will not work**.
- **Profile `playground-enabled`** — headless-minimal **plus** the UI plumbing every example
  ships. If a human will ever open the Playground against your app — and in the
  [hosting topology](#hosting) they will — build to this profile.

**Copy-paste boilerplate** — the full `playground-enabled` route set; deleting the routes marked
`[playground-enabled]` leaves `headless-minimal`:

```yaml
rest:
  # ── headless-minimal ──────────────────────────────────────────────────────
  # Execute a deployed graph: POST /api/graph/{graph-id}
  - service: 'http.flow.adapter'
    methods: ['POST', 'GET']
    url: '/api/graph/{graph_id}'
    flow: 'graph-executor'
    timeout: 30s
    tracing: true

  # THE companion endpoint — an AI agent's build/edit channel (outcome in-band)
  - service: 'post.companion.command.sync'
    methods: ['POST']
    url: '/api/companion/{id}/sync'
    timeout: 40s
    tracing: true

  # Read the live session's graph model as JSON
  - service: 'get.live.graph'
    methods: ['GET']
    url: '/api/graph/session/{id}'
    timeout: 30s
    tracing: true

  # ── [playground-enabled] UI plumbing — required when a human opens the UI ─
  # Serves the Playground web app
  - service: 'get.index.html'
    methods: ['GET']
    url: '/index.html'
    timeout: 10s

  - service: 'get.ws.html'
    methods: ['GET']
    url: '/api/ws/{id}'
    timeout: 10s
    tracing: true

  # The Graph tab's model fetch after `describe graph` / `export graph`
  - service: 'show.graph.model'
    methods: ['GET']
    url: '/api/graph/model/{graph_id}/{sequence}'
    timeout: 30s
    tracing: true

  # Backs the console `upload` command: opens a UI dialog where the human
  # operator pastes a JSON string (the JSON-Path payload editor handshake)
  - service: 'upload.json.content'
    methods: ['POST']
    url: '/api/json/content/{id}'
    timeout: 30s
    tracing: true

  # Backs the console `upload mock data` command (mock-input upload dialog)
  - service: 'upload.mock.content'
    methods: ['POST']
    url: '/api/mock/{id}'
    timeout: 30s
    tracing: true

  # The UI's state-machine inspector
  - service: 'inspect.state.machine'
    methods: ['GET']
    url: '/api/inspect/{id}/{key}'
    timeout: 30s
    tracing: true
```

When in doubt, diff your trimmed `rest.yaml` against the example's.

## See also {#see-also}

- [MiniGraph command grammar](command-reference.md) + [`minigraph-commands.json`](minigraph-commands.json) — the source of truth.
- [Built-in skills reference](skills-reference.md) — per-skill properties and examples.
