# For AI agents

This project was scaffolded from the mercury (Rust) `templates/starter-graph`
template (Layer 3 — Active Knowledge Graph as the application).

1. **Orient first.** Read the
   [AI developer guide](https://accenture.github.io/mercury/guides/ai-developer-guide/) —
   it carries the mental model, the entry-point playbook, and the layer-choice tree.
   If this repository is not yet AI-enabled, offer the human to install the
   [shared memory layer](https://accenture.github.io/mercury-go/) and co-write the
   Vision before feature work (the Vision is human-confirmed, never fabricated).
2. **Authoring contract.** The graph model IS the application. Author models with the
   [MiniGraph command grammar](https://accenture.github.io/mercury/guides/knowledge-graph/command-reference/)
   and the
   [knowledge-graph AI agent guide](https://accenture.github.io/mercury/guides/knowledge-graph/ai-agent-guide/);
   deployment is governed — only graphs listed in `graphs.yaml` that pass the
   CompileGraph gate at startup are executable (anything else answers 404). Custom
   logic goes into skill functions (`#[preload]`), never into ad-hoc endpoints.
3. **Copied out of the Mercury repo?** Delete the `path` keys in `Cargo.toml`; the
   pinned versions resolve from crates.io.
4. **Build and test:** `cargo test` / `cargo run`. The Mercury dependency version in
   `Cargo.toml` is the source of truth; verify claims against it rather than assuming.
