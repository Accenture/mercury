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

//! The six composable functions behind the discovery flows - the Rust twins of
//! the Java service classes. Each is addressed only by its route name; the
//! rest.yaml -> flow -> function wiring is identical to the Java app (the flow
//! YAML files are byte-identical across the two engines).

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::{preload, AppError, ComposableFunction, EventEnvelope};
use serde_json::json;

use super::catalog::ContractCatalog;
use super::exporter;
use super::snapshot::SkillSnapshot;

fn input_text(input: &EventEnvelope, key: &str) -> Option<String> {
    let body: serde_json::Value = input.body_as().ok()?;
    body[key].as_str().map(str::to_string)
}

/// One URL for an AI agent to learn this discovery service (Java `DiscoveryIndex`).
#[preload(route = "v1.discovery.index", instances = 10)]
struct DiscoveryIndex;

#[async_trait]
impl ComposableFunction for DiscoveryIndex {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let ids: Vec<&str> = ContractCatalog::get_instance()
            .contracts()
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        EventEnvelope::new().set_body(json!({
            "name": "ai-contract-provider",
            "description": "Version-matched Mercury operational contract for AI discovery (read-only)",
            "mercury_version": SkillSnapshot::get_instance().mercury_version(),
            "contracts": ids,
            "endpoints": {
                "contracts": "GET /api/contracts",
                "contract_detail": "GET /api/contracts/{id}",
                "skill": "GET /api/skill",
                "reference": "GET /api/references?path={reference-path}",
                "manifest": "GET /api/manifest",
            },
        }))
    }
}

/// List the installed contracts (Java `ContractList`).
#[preload(route = "v1.contract.list", instances = 10)]
struct ContractList;

#[async_trait]
impl ComposableFunction for ContractList {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let contracts: Vec<serde_json::Value> = ContractCatalog::get_instance()
            .contracts()
            .iter()
            .map(|c| json!({"id": c.id, "module": c.module, "summary": c.summary}))
            .collect();
        EventEnvelope::new().set_body(json!({
            "mercury_version": SkillSnapshot::get_instance().mercury_version(),
            "total": contracts.len(),
            "contracts": contracts,
        }))
    }
}

/// Describe one installed contract (Java `ContractDetail`).
#[preload(route = "v1.contract.detail", instances = 10)]
struct ContractDetail;

#[async_trait]
impl ComposableFunction for ContractDetail {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let id = input_text(&input, "id").unwrap_or_default();
        match ContractCatalog::get_instance().contract(&id) {
            Some(contract) => EventEnvelope::new().set_body(json!({
                "id": contract.id,
                "module": contract.module,
                "summary": contract.summary,
                "mercury_version": SkillSnapshot::get_instance().mercury_version(),
                "behavior_anchors": contract.anchors,
                "references": contract.references,
            })),
            None => Err(AppError::new(
                404,
                format!("Contract {id} is not installed"),
            )),
        }
    }
}

/// Serve one packaged reference file by its inventory path (Java `ReferenceReader`).
#[preload(route = "v1.reference.reader", instances = 10)]
struct ReferenceReader;

#[async_trait]
impl ComposableFunction for ReferenceReader {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let path = input_text(&input, "path");
        let file = SkillSnapshot::get_instance().read_file(path.as_deref())?;
        EventEnvelope::new().set_body(file)
    }
}

/// Report the deterministic snapshot manifest (Java `ManifestGenerator`).
#[preload(route = "v1.manifest.generator", instances = 10)]
struct ManifestGenerator;

#[async_trait]
impl ComposableFunction for ManifestGenerator {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body(SkillSnapshot::get_instance().manifest().clone())
    }
}

/// Write the offline skill snapshot (Java `SkillExporter`).
#[preload(route = "v1.skill.exporter")]
struct SkillExporter;

#[async_trait]
impl ComposableFunction for SkillExporter {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let directory = input_text(&input, "directory");
        let result = exporter::export(directory.as_deref())?;
        EventEnvelope::new().set_body(result)
    }
}
