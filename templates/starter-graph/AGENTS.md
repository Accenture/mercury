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
3. **Hosting a Playground session** (the agent hosts, humans subscribe): this starter
   runs the production shape and serves no Playground UI — run the `minigraph-playground`
   app alongside and host THAT session with the shipped broker:
   `node scripts/playground-session-broker.mjs --target http://127.0.0.1:8085`. Read the
   session id from `GET http://127.0.0.1:8765/session`, hand it to the humans
   (`session subscribe <id>` in their browsers), drive commands through the companion
   endpoint, and export the finished model into `resources/graph/` + `graphs.yaml` here.
   **Never hand-roll the WebSocket client**: the session contract's keep-alive (ping every
   ~20 s) is easy to miss, and a client without it dies silently at the idle timeout. See
   [Hosting the session yourself](https://accenture.github.io/mercury/guides/knowledge-graph/ai-agent-guide/#hosting).
4. **Copied out of the Mercury repo?** Delete the `path` keys in `Cargo.toml`; the
   pinned versions resolve from crates.io.
5. **Build and test:** `cargo test` / `cargo run`. The Mercury dependency version in
   `Cargo.toml` is the source of truth; verify claims against it rather than assuming.
