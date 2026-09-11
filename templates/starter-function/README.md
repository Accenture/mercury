# starter-function — Layer 1 template

A minimal composable application: one function (`v1.greeting`) exposed over HTTP by
REST automation. Copy this directory out of the Mercury repository to begin a new
project.

## After copying out

The manifest uses an in-repo `path` for the Mercury crates. In your copy, delete the
`path` keys in `Cargo.toml` — the pinned versions then resolve from crates.io:

```toml
mercury-platform-core = "4.12.6"
```

## Build, test, run

```bash
cargo test
cargo run
```

Then:

```bash
curl -s -X POST http://127.0.0.1:8301/api/greeting \
     -H "content-type: application/json" \
     -d '{"name": "Mercury"}'
# → {"greeting": "Hello, Mercury", ...}
```

## What to look at

| File | Role |
|:---|:---|
| `src/main.rs` | The composable function (`#[preload]`) — addressed only by its route name |
| `resources/rest.yaml` | Maps `/api/greeting` to the function; add your endpoints here |
| `resources/application.yml` | App name, port, `rest.automation` |
| `tests/greeting_test.rs` | Direct RPC test + end-to-end HTTP test |

## Next steps

- Add functions (`#[preload]` + `ComposableFunction` or `TypedFunction`) and map them
  in `rest.yaml`.
- Ready to orchestrate several functions? Move up a layer — see the
  [starter-flow](../starter-flow) template and the
  [AI developer guide](https://accenture.github.io/mercury/guides/ai-developer-guide/).
