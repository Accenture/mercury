# starter-graph — Layer 3 template

A minimal Active Knowledge Graph application with **zero imperative code**: the deployed
graph model (`starter-quote`) validates the request and answers from its own knowledge.
Changing what the service does means editing the model, not writing code. The model file
is engine-portable — the same JSON runs on the Java engine. Copy this directory out of
the Mercury repository to begin a new project.

## After copying out

The manifest uses an in-repo `path` for the Mercury crates. In your copy, delete the
`path` keys in `Cargo.toml` — the pinned versions then resolve from crates.io:

```toml
mercury-platform-core = "4.12.12"
mercury-knowledge-graph = "4.12.12"
```

Two crates are enough: an application declares what its code names — `platform_core::` for the
macros and the event types, `knowledge_graph::` for the Layer 3 engine — and `mercury-knowledge-graph`
brings `mercury-event-script` and the flow engine along transitively.

## Build, test, run

```bash
cargo test
cargo run
```

Then:

```bash
curl -s -X POST http://127.0.0.1:8303/api/graph/starter-quote \
     -H "content-type: application/json" \
     -d '{"item": "widget"}'
# → {"item": "widget", "unit_price": 100, "currency": "USD", "status": "quoted"}
```

**One endpoint serves every graph.** `/api/graph/{graph_id}` takes the id from the URL path, so
deploying a second model means adding its id to `graphs.yaml` — never a second REST entry.

## Dev mode is on — build graphs with an AI agent

`application.yml` ships with **`app.env: dev`**, so the app serves the MiniGraph Playground
alongside your graph endpoint: the UI at <http://127.0.0.1:8303>, the session WebSocket, and the
AI companion endpoint `POST /api/companion/{session-id}/sync`. Open the URL and you are in the
workbench; there is nothing else to install.

To have an AI agent **host** the session — so you and the agent co-author one live model as equals,
and a dropped browser tab loses nothing — start the bundled broker and subscribe to the id it
prints:

```bash
node scripts/playground-session-broker.mjs --target http://127.0.0.1:8303
# → session id: ws-NNNNNN-N   ·  in the Playground console, type: session subscribe ws-NNNNNN-N
```

Every Playground service is gated by `#[optional_service("app.env=dev")]`, so **deleting that one
line from `application.yml` closes the whole surface** — the matching `rest.yaml` entries are then
skipped at start-up. Do that before you ship to production; there is no auth on these endpoints.
The home page follows the same switch: `get.index.html` (routed at `/index.html`, also reached at
`/`) serves the Playground web app in dev mode and a plain service page otherwise, so a production
deployment never shows the Playground UI.

## What to look at

| File | Role |
|:---|:---|
| `resources/graph/starter-quote.json` | The application — a graph whose nodes execute during traversal |
| `resources/graphs.yaml` | The deployment manifest: only listed graphs that pass the CompileGraph gate are executable ("compiled or 404") |
| `resources/rest.yaml` | The one graph endpoint, the home page, plus the dev-mode Playground / companion routes |
| `resources/application.yml` | App config — including the `app.env: dev` switch that opens the Playground |
| `tests/quote_graph_test.rs` | End-to-end graph tests, including the 404 gate behavior |
| `scripts/playground-session-broker.mjs` | Lets an AI agent **host** a Playground session for you (keep-alive, auto-reconnect, localhost control API) — see `scripts/README.md` |

## Next steps

- Evolve the model: add nodes, decisions, and skills — the
  [built-in skills reference](https://accenture.github.io/mercury/guides/knowledge-graph/skills-reference/)
  catalogs what nodes can do without code.
- Draft and dry-run models interactively in the Playground (already enabled — see above) — the
  [Playground & AI companion guide](https://accenture.github.io/mercury/guides/knowledge-graph/playground-and-companion/)
  covers the command grammar; use the bundled broker rather than a hand-rolled WebSocket client,
  because the keep-alive is easy to miss.
- Custom logic when the model needs it: attach a skill function (`#[preload]`) to a node —
  the deliberate seam between the model and code. See the
  [AI developer guide](https://accenture.github.io/mercury/guides/ai-developer-guide/).
