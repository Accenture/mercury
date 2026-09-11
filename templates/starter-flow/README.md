# starter-flow — Layer 2 template

A minimal Event Script application: an HTTP endpoint launches a flow that sequences two
decoupled functions (`v1.validate.request` → `v1.make.greeting`) with declarative data
mapping. The flow YAML is engine-portable — the same file runs on the Java engine. Copy
this directory out of the Mercury repository to begin a new project.

## After copying out

The manifest uses an in-repo `path` for the Mercury crates. In your copy, delete the
`path` keys in `Cargo.toml` — the pinned versions then resolve from crates.io:

```toml
mercury-platform-core = "4.12.7"
mercury-event-script = "4.12.7"
```

## Build, test, run

```bash
cargo test
cargo run
```

Then:

```bash
curl -s -X POST http://127.0.0.1:8302/api/greeting \
     -H "content-type: application/json" \
     -d '{"name": "Mercury"}'
# → {"greeting": "Hello, Mercury", ...}
```

## What to look at

| File | Role |
|:---|:---|
| `resources/flows/greeting-flow.yml` | The orchestration — task order and data mapping, no code |
| `resources/flows.yaml` | Registry of active flows |
| `resources/rest.yaml` | Binds `/api/greeting` to the flow (`service` + `flow`, both required) |
| `src/main.rs` | The two functions — each knows nothing about the other |
| `tests/greeting_flow_test.rs` | End-to-end flow test over HTTP |

## Next steps

- Grow the flow: add tasks, a `decision` branch, or an exception handler — see the
  [Event Script syntax](https://accenture.github.io/mercury/guides/event-script/syntax/).
- Modeling a whole service as a graph? Move up a layer — see the
  [starter-graph](../starter-graph) template and the
  [AI developer guide](https://accenture.github.io/mercury/guides/ai-developer-guide/).
