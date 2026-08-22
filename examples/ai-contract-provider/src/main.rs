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

//! Mercury AI contract provider - a standalone composable app (the Rust
//! lock-step twin of the Java `system/ai-contract-provider`).
//!
//! Default mode is a read-only REST discovery server (rest.server.port, default
//! 8999) whose endpoints are wired rest.yaml -> Event Script flow -> function -
//! the app is itself a reference implementation of the composable pattern. With
//! `--export <directory>` it performs one offline skill export through the same
//! export-skill flow and exits. The flow YAML files are byte-identical to the
//! Java app's; the packaged references are this port's own documentation set.
//!
//! ```bash
//! cargo run -p ai-contract-provider
//! curl 'http://127.0.0.1:8999/api/discovery'
//! cargo run -p ai-contract-provider -- --export /tmp
//! ```

// pub so the integration tests (which include this bin source via #[path])
// can reach the module APIs
pub mod catalog;
pub mod exporter;
pub mod functions;
pub mod snapshot;

use std::time::Duration;

use async_trait::async_trait;
use event_script::conversions::from_json;
use event_script::FlowExecutor;
use platform_core::{main_application, trace, AppConfigReader, AppError, EntryPoint, Platform};

const EXPORT_FLAG: &str = "--export";
const EXPORT_FLOW: &str = "export-skill";
const EXPORT_TIMEOUT: Duration = Duration::from_secs(30);

#[main_application]
struct AiContractProvider;

#[async_trait]
impl EntryPoint for AiContractProvider {
    async fn start(&self, args: &[String]) -> Result<(), AppError> {
        // force the snapshot render and catalog validation - fail closed at startup
        let snapshot = snapshot::SkillSnapshot::get_instance();
        let contracts = catalog::ContractCatalog::get_instance().contracts();
        log::info!(
            "Mercury {} operational contract ready - {} contracts, {} snapshot files",
            snapshot.mercury_version(),
            contracts.len(),
            snapshot.files().len()
        );
        if let Some(directory) = export_directory(args)? {
            std::process::exit(export_skill(&directory).await);
        }
        let port = AppConfigReader::get_instance().get_property_or("rest.server.port", "8999");
        log::info!(
            "AI discovery endpoints ready - start with GET http://127.0.0.1:{port}/api/discovery"
        );
        Ok(())
    }
}

fn export_directory(args: &[String]) -> Result<Option<String>, AppError> {
    for (i, arg) in args.iter().enumerate() {
        if arg == EXPORT_FLAG {
            return match args.get(i + 1).map(|d| d.trim()) {
                Some(directory) if !directory.is_empty() => Ok(Some(directory.to_string())),
                _ => Err(AppError::new(400, "Usage: --export <existing-directory>")),
            };
        }
    }
    Ok(None)
}

/// One-shot CLI export through the export-skill flow; returns the process exit code.
async fn export_skill(directory: &str) -> i32 {
    let platform = Platform::get_instance();
    let dataset = from_json(&serde_json::json!({
        "headers": {},
        "body": {"directory": directory},
    }));
    let trace_id = trace::new_trace_id();
    let cid = format!("cli-export-{}", std::process::id());
    match FlowExecutor::request(
        &platform,
        EXPORT_FLOW,
        dataset,
        &cid,
        EXPORT_TIMEOUT,
        Some((&trace_id, "EXPORT /skill")),
    )
    .await
    {
        Ok(reply) => {
            let body: serde_json::Value = reply.body_as().unwrap_or(serde_json::Value::Null);
            if let Some(skill_directory) = body["skill_directory"].as_str() {
                log::info!(
                    "Mercury platform skill exported to {skill_directory} ({} files, snapshot {})",
                    body["files"],
                    body["snapshot_sha256"]
                );
                0
            } else {
                log::error!("Skill export failed - {body}");
                1
            }
        }
        Err(e) => {
            log::error!("Unable to export skill - {e}");
            1
        }
    }
}

platform_core::auto_start_main!();
