# minigraph-playground — the active knowledge graph (layer 3) workbench

The **MiniGraph Playground**: a browser workbench for building, running and
inspecting graphs whose nodes carry executable skills — traversing the graph
*is* running the application. Layer 3 (`knowledge-graph`) riding layer 2
(`event-script`) on layer 1 (`platform-core`).

Linking the `knowledge-graph` crate self-registers the whole engine through the
annotation inventory: the graph runtime + core skills (`#[preload]`), the
graph/flow compilers (`#[before_application]`), and — because this app runs with
`app.env=dev` — the dev-gated `PlaygroundLoader`, which registers the command
service, the websocket UI (`/ws/graph`, `/ws/json`) and the companion REST
endpoints. `main()` is still one line.

The React webapp (`@xyflow/react`) is built once with `npm run release` in
`crates/knowledge-graph/webapp/`; its compiled bundle travels with the engine
crate at `crates/knowledge-graph/resources/public/` and REST automation serves
it as static content at `/`.

## Run it

```bash
cargo run -p minigraph-playground
# then open http://127.0.0.1:8085/ in a browser
```

The startup log shows the layered boot: the graph/flow compilers → preloaded
skills → `Playground loaded (app.env=dev)` → REST automation on port **8085**.

## Test drive

**In the browser** — open `http://127.0.0.1:8085/`. Build a graph on the canvas,
run it, and inspect the state machine; the workbench talks to the engine over
`/ws/graph`.

**The websocket command grammar** (what the canvas drives) — any WebSocket
client works; the session's `.out` stream echoes results:

```text
connect  ws://127.0.0.1:8085/ws/graph/playground
< session ws-100001-1 started
          Companion endpoint: /api/companion/ws-100001-1
> create node root
< node root created
> help connect
< Connect two nodes together …          (served from the engine's help/*.md)
```

**The AI-companion REST hop** — an agent POSTs a text command to the session and
the output streams to that session's WebSocket console (the field use case):

```bash
curl -X POST 'http://127.0.0.1:8085/api/companion/ws-100001-1' \
     -H 'content-type: text/plain' -d 'list nodes'
# {"status":"accepted","type":"companion", …}   ← output streams to the WS console
```

**Deploy-and-run a graph over HTTP** — a graph exported from the Playground runs
through the `graph-executor` flow:

```bash
curl -X POST 'http://127.0.0.1:8085/api/graph/{graph-id}' -H 'content-type: application/json' -d '{...}'
```

**Production note** — the Playground is dev-only. Set `app.env` to anything but
`dev` and the workbench (command service, websocket UI, companion endpoints)
does not register; deployed graphs still run through `POST /api/graph/{graph-id}`.

## AI-token streaming and the support-triage graph (experiment E0)

Two pieces of the agent-orchestration experiment ride in this Playground, both reaching an AI node
that is a plain function on a polyglot wrapper host — the engine stays LLM-free:

- **`support-triage`** (`resources/graph/support-triage.json`): bounded-agency triage — the
  `llm.chat` node classifies a request into an enumerated category and the *graph* decides the
  route. `POST /api/graph/support-triage` with `{"text": "..."}`.
- **`POST /api/llm/stream`**: the `llm.stream.relay` function forwards its reply lane into the
  event-over-http mapped `llm.stream` node, and the provider's real token batches re-render
  progressively out this application's edge as SSE.

Both routes live on a wrapper host — `resources/event-over-http.yaml` points them at the Node.js
demo app (port 8087) by default; `-Dllm.peer.port=8086` selects the Python demo app. Start one with
a provider credential (`GEMINI_API_KEY` with `-Dllm.provider=gemini`, or `ANTHROPIC_API_KEY`), then:

```bash
curl -N -H 'accept: text/event-stream' -H 'content-type: application/json' \
  -d '{"prompt":"Write two sentences about event-driven design","params":{"provider":"gemini"}}' \
  http://127.0.0.1:8085/api/llm/stream
```

Without a mapped peer the relay answers a 503 that names the missing configuration. With
`otel.forwarding=true` on the engine and the wrapper host, one distributed trace spans the edge,
the relay and the AI node (see the OpenTelemetry certification report).

## Where things live

| File | Role |
|---|---|
| `src/main.rs` | the one-line main; links the engine's inventory |
| `resources/application.yml` | app name, `app.env=dev`, port, temp folder |
| `resources/rest.yaml` | the Playground/companion endpoint bindings |
| `../../crates/knowledge-graph/webapp/` | the React webapp source (`npm run release`) |
| `../../crates/knowledge-graph/resources/public/` | the served bundle (built) |
| `../../crates/knowledge-graph/resources/help/` | the `describe`/help markdown |
