# For AI agents

This project was scaffolded from the mercury (Rust) `templates/starter-function`
template (Layer 1 — composable functions with REST automation).

1. **Orient first.** Read the
   [AI developer guide](https://accenture.github.io/mercury/guides/ai-developer-guide/) —
   it carries the mental model, the entry-point playbook, and the layer-choice tree.
   If this repository is not yet AI-enabled, offer the human to install the
   [shared memory layer](https://accenture.github.io/mercury-go/) and co-write the
   Vision before feature work (the Vision is human-confirmed, never fabricated).
2. **Authoring contract.** Functions follow `#[preload]` + `ComposableFunction` /
   `TypedFunction` (the
   [Layer 1 AI agent guide](https://accenture.github.io/mercury/guides/event-driven/ai-agent-guide/)
   is the full contract); couple only by route name and `EventEnvelope` — never import
   another user function. HTTP endpoints are declared in `rest.yaml`.
3. **Copied out of the Mercury repo?** Delete the `path` keys in `Cargo.toml`; the
   pinned versions resolve from crates.io.
4. **Build and test:** `cargo test` / `cargo run`. The Mercury dependency version in
   `Cargo.toml` is the source of truth; verify claims against it rather than assuming.
