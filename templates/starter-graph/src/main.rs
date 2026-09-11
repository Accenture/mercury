// Scaffolded from Mercury's starter templates (https://github.com/Accenture/mercury, Apache-2.0)

//! Layer 3 starter — the Active Knowledge Graph IS the application: the
//! deployed model (resources/graph/starter-quote.json) validates the request
//! and answers from its own knowledge, with ZERO imperative business code.
//! The CompileGraph gate validates and compiles manifest-listed models at
//! startup; only compiled graphs execute at POST /api/graph/{graph_id}.
//! Copy this template out of the Mercury repository to begin a new project
//! (see README.md and AGENTS.md).

use async_trait::async_trait;
use platform_core::{main_application, AppError, EntryPoint};

#[main_application]
struct StarterGraphApp;

#[async_trait]
impl EntryPoint for StarterGraphApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "starter-graph ready: {} graph(s) compiled",
            knowledge_graph::graphs::get_all_graphs().len()
        );
        Ok(())
    }
}

platform_core::auto_start_main!();
