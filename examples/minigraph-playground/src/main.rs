//
// Copyright 2018-2026 Accenture Technology
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! The MiniGraph Playground application — layer 3 (active knowledge graph)
//! riding layer 2 (event-script) on layer 1 (platform-core). Linking the
//! `knowledge_graph` crate pulls in its annotation inventory: the graph
//! runtime + core skills (`#[preload]`), the graph/flow compilers
//! (`#[before_application]`), and — because this app runs with `app.env=dev`
//! (`resources/application.yml`) — the dev-gated `PlaygroundLoader` that
//! registers the command service, the websocket UI (`/ws/graph`, `/ws/json`)
//! and the companion REST endpoints (`resources/rest.yaml`).
//!
//! The compiled React webapp travels with the engine crate
//! (`crates/knowledge-graph/resources/public`, built by `npm run release` in
//! `crates/knowledge-graph/webapp`) and REST automation serves it as static
//! content at `/`.
//!
//! Run it:
//! ```text
//! cargo run -p minigraph-playground
//! # then open http://127.0.0.1:8100/ in a browser
//! ```

use async_trait::async_trait;
use platform_core::{main_application, AppError, EntryPoint};

/// The main application: by the time it runs, the engine has compiled the
/// deployed graphs (`CompileGraph`, sequence 6) and — under `app.env=dev` —
/// loaded the Playground. Referencing the `knowledge_graph` crate here also
/// guarantees the linker keeps its annotation inventory.
#[main_application]
struct PlaygroundApp;

#[async_trait]
impl EntryPoint for PlaygroundApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "MiniGraph Playground ready: {} graph(s) compiled — open http://127.0.0.1:8100/ in a browser",
            knowledge_graph::graphs::get_all_graphs().len()
        );
        Ok(())
    }
}

platform_core::auto_start_main!();
