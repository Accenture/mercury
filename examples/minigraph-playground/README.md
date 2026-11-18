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

The React webapp is single-sourced in the Java repo (`mercury-composable`,
`system/minigraph-playground-engine/webapp/`): `npm run release:rust` there deploys
its compiled bundle and the help pages into `crates/knowledge-graph/resources/`,
where they travel with the engine crate, and REST automation serves the bundle as
static content at `/`.

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

All fourteen tutorials and `support-triage` are deployed in this app (`resources/graphs.yaml`): tutorial 1 and
`support-triage` as loose graphs, tutorials 2 to 14 as the graph set `tutorials`. Tutorial 13,
for one, fetches a profile from the app's own dev mock endpoint:

```bash
curl -X POST 'http://127.0.0.1:8085/api/graph/tutorial-13' -H 'content-type: application/json' -d '{"person_id": 100}'
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
  event-over-http mapped `llm.stream` node, and the model's real token batches re-render
  progressively out this application's edge as SSE.

Both routes live on the LLM helper app — `resources/event-over-http.yaml` points them at the Node.js
helper (mercury-nodejs `extensions/llm-helper`, port 8087) by default; `-Dllm.peer.port=8086` selects the
Python helper (mercury-python `extensions/llm-helper`). Start one with `ANTHROPIC_API_KEY` exported in its
environment, then:

```bash
curl -N -H 'accept: text/event-stream' -H 'content-type: application/json' \
  -d '{"prompt":"Write two sentences about event-driven design","params":{"max_tokens":2000}}' \
  http://127.0.0.1:8085/api/llm/stream
```

The helper's default model is Claude Opus 5.5. It thinks before it answers, and its thinking tokens count against
`max_tokens`, so give a call a generous budget: this example and the `support-triage` graph ask for 2000, the helper's
own default is 16000, and a budget of a few hundred can end with no text, which the helper reports as
`422 LLM reply is empty`. For a smoother token trickle than Opus's bursts, start the helper with
`-Dllm.model=claude-haiku-4-5` (see its README).

Without a mapped peer the relay answers a 503 that names the missing configuration. With
`otel.forwarding=true` on the engine and the wrapper host, one distributed trace spans the edge,
the relay and the AI node (see the OpenTelemetry certification report).

## The AI demo graphs: an agent loop, an MCP server's tool, the built-in tools

Beside `support-triage`, this example ships the agent graphs of the Java Playground, compiled unchanged by this engine:
`order-assistant` (an agent loop on `llm.run` whose one write, a refund, waits for a supervisor through suspend and
resume on Redis), `diagram-assistant` (the loop's one tool reaches a Mermaid MCP server through `diagram.render`, a
function of this example in front of the pack's MCP binding, with the rendered image stored out of band) and the three
graphs on the built-in agent tools of ADR-0036 - `site-notes` (a crawl of a documentation section, then a supervised write
of the notes into the pack's workspace), `document-ingest` (a document of the synthetic corpus in `corpus/` extracted into
the agent's knowledge graph by a node that declares `mode: auto`) and `document-query` (the graph read, the sources cited).
The example's own tool functions are `src/order_book.rs` and `src/diagram.rs`; the loop comes from the `agent-loop`
extension crate this example links. `resources/graph/README.md` has the setup and the protocols; the token-free end-to-end
test is `tests/agent_graphs.rs` (`cargo test -p minigraph-playground --test agent_graphs`), with a scripted model, stand-ins
for the tool routes and the Mermaid binding, and the RESP2 Redis double of `tests/common/`.

The peers are the language packs' extensions, reached through `resources/event-over-http.yaml`: the LLM helper (8087
Node.js, 8086 Python), the MCP binding (8089 Node.js, 8088 Python) and the agent tools (8090 Node.js, 8089 Python). The
`-D` arguments after `--` select them, for example the Python pack for all three:

```bash
cargo run -p minigraph-playground -- -Dllm.peer.port=8086 -Dmcp.peer.port=8088 -Dtools.peer.port=8089
```

Without the Playground a run is one request, `POST /api/graph/<graph>`, or `POST /api/stream/graph/<graph>` with
`accept: text/event-stream` to watch the loop's events before the result.

## Where things live

| File | Role |
|---|---|
| `src/main.rs` | the one-line main; links the engine's, the state store's and the agent loop's inventories |
| `src/lib.rs`, `src/order_book.rs`, `src/diagram.rs` | the example's own functions: the streaming relay, the order book's three tools, `diagram.render` |
| `resources/application.yml` | app name, `app.env=dev`, port, temp folder |
| `resources/rest.yaml` | the Playground/companion endpoint bindings |
| `resources/graphs.yaml` | the deployment manifest: `tutorial-1`, `support-triage`, the five agent graphs and the set `tutorials` (compiled or 404); the set unpacks into `/tmp/unpack` |
| `resources/event-over-http.yaml` | the peer map: `llm.chat`, `llm.stream`, `mermaid.render` and the twelve built-in tool routes on the language packs |
| `resources/flows.yaml` | the flows this app deploys: `graph-executor` and `flow-11` |
| `resources/flows/graph-executor.yml` | the standard exposure flow behind `POST /api/graph/{graph-id}` |
| `resources/flows/flow-11.yml` | the echo flow that tutorial 11's `graph.extension` node calls (`flow://flow-11`) |
| `resources/graph/support-triage.json` | the E0 triage graph; tutorial 1's model travels with the engine crate |
| `resources/graph/order-assistant.json`, `diagram-assistant.json`, `site-notes.json`, `document-ingest.json`, `document-query.json` | the agent demo graphs, the Java example's models unchanged (`resources/graph/README.md`) |
| `resources/mcp/mermaid-servers.yml` | a copy of the MCP binding's server entry for the Mermaid server behind `mermaid.render` |
| `corpus/` | the synthetic corpus of the document graphs: two agreements and two policies of fictional companies |
| `tests/agent_graphs.rs`, `tests/common/` | the token-free end-to-end test of the five agent graphs; the scripted model, the tool stand-ins and the RESP2 Redis double |
| `resources/graph/tutorials.pack` | the graph set `tutorials` (version 1.0): tutorials 2 to 14, packed by the graph packager |
| `../../crates/knowledge-graph/resources/public/` | the served bundle, deployed from the Java repo's webapp (`npm run release:rust`) |
| `../../crates/knowledge-graph/resources/help/` | the `describe`/help markdown, mirrored from the Java repo by the same command |
