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
mercury-platform-core = "4.12.7"
mercury-event-script = "4.12.7"
mercury-knowledge-graph = "4.12.7"
```

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

## What to look at

| File | Role |
|:---|:---|
| `resources/graph/starter-quote.json` | The application — a graph whose nodes execute during traversal |
| `resources/graphs.yaml` | The deployment manifest: only listed graphs that pass the CompileGraph gate are executable ("compiled or 404") |
| `resources/rest.yaml` | Exposes deployed graphs through the engine's graph-executor flow |
| `tests/quote_graph_test.rs` | End-to-end graph tests, including the 404 gate behavior |

## Next steps

- Evolve the model: add nodes, decisions, and skills — the
  [built-in skills reference](https://accenture.github.io/mercury/guides/knowledge-graph/skills-reference/)
  catalogs what nodes can do without code.
- Draft and dry-run models interactively in the Playground — see
  [Playground & AI companion](https://accenture.github.io/mercury/guides/knowledge-graph/playground-and-companion/).
- Custom logic when the model needs it: attach a skill function (`#[preload]`) to a node —
  the deliberate seam between the model and code. See the
  [AI developer guide](https://accenture.github.io/mercury/guides/ai-developer-guide/).
