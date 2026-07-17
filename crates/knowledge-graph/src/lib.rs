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

//! The Active Knowledge Graph engine (layer 3) — Rust port of the Java
//! `minigraph-playground-engine`. A property graph whose nodes carry
//! executable skills: traversing the graph IS running the application.
//!
//! Builds on [`platform_core::graph::MiniGraph`](../platform_core/graph)
//! (layer 1) and exposes deployed graphs through event-script flows
//! (layer 2). This crate starts with the self-contained [`math`] expression
//! engine (increment K-2); the compiler, runtime and skills follow.
//!
//! Deliberately absent: `graph.js` — the Java engine embeds a GraalVM
//! JavaScript interpreter for lack of an alternative; an interpreter running
//! arbitrary user-supplied code is an attack surface this port retires
//! (maintainer decision, 2026-07-17). `graph.math` (typed, bounded) and
//! `graph.task` (reviewed, compiled functions) cover the use cases.

pub mod compiler;
pub mod graphs;
pub mod math;

use async_trait::async_trait;
use platform_core::{before_application, AppError, EntryPoint};

/// The engine's bundled resources (graphs, flows, help, mock data, the
/// webapp bundle) — the Rust analog of a jar's classpath resources. Appended
/// (not prepended), so the application's own `resources/` always wins.
/// Runs early (sequence 1) so both the flow compiler (5) and the graph
/// compiler (6) can see `classpath:` resources that travel with this crate.
#[before_application(sequence = 1)]
pub struct GraphResources;

#[async_trait]
impl EntryPoint for GraphResources {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        platform_core::resources::append_resource_root(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources"
        ));
        Ok(())
    }
}

/// The graph-model quality gate (Java `CompileGraph`,
/// `@BeforeApplication(sequence=6)`).
#[before_application(sequence = 6)]
pub struct CompileGraph;

#[async_trait]
impl EntryPoint for CompileGraph {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        compiler::compile_graphs();
        Ok(())
    }
}
