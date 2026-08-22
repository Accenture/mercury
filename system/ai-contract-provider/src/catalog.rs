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

//! The operational contract catalog (Java `ContractCatalog` analog), loaded from
//! the embedded contracts.yaml and validated eagerly.
//!
//! Behavior anchors are declared as strings so this app does not need a runtime
//! dependency on every crate it describes (a runtime dependency on the
//! knowledge-graph crate would preload the playground into this app). The anchor
//! test resolves every anchor at COMPILE TIME through dev-only dependencies -
//! the Java Class.forName analog - so a renamed or removed behavior item still
//! fails the workspace build.

use std::sync::OnceLock;

const CONTRACTS_YAML: &str = include_str!("../resources/contracts.yaml");

#[derive(Debug, Clone)]
pub struct ContractEntry {
    pub id: String,
    pub module: String,
    pub summary: String,
    pub anchors: Vec<String>,
    pub references: Vec<String>,
}

pub struct ContractCatalog {
    contracts: Vec<ContractEntry>,
}

impl ContractCatalog {
    /// The validated singleton - construction fails closed (panics at first use)
    /// on an invalid catalog.
    pub fn get_instance() -> &'static ContractCatalog {
        static INSTANCE: OnceLock<ContractCatalog> = OnceLock::new();
        INSTANCE.get_or_init(|| ContractCatalog {
            contracts: load(CONTRACTS_YAML).unwrap_or_else(|e| panic!("{e}")),
        })
    }

    pub fn contracts(&self) -> &[ContractEntry] {
        &self.contracts
    }

    pub fn contract(&self, id: &str) -> Option<&ContractEntry> {
        self.contracts.iter().find(|c| c.id == id)
    }
}

/// Parse and validate a catalog document (public for the negative-fixture tests).
pub fn load(yaml: &str) -> Result<Vec<ContractEntry>, String> {
    let document: serde_yaml::Value =
        serde_yaml::from_str(yaml).map_err(|e| format!("Invalid contracts.yaml - {e}"))?;
    let entries = document
        .get("contracts")
        .and_then(|v| v.as_sequence())
        .filter(|list| !list.is_empty())
        .ok_or("contracts.yaml must contain a non-empty 'contracts' list")?;
    let mut ids: Vec<String> = Vec::new();
    let mut result = Vec::new();
    for item in entries {
        if !item.is_mapping() {
            return Err(
                "Each contract must be a map of id, module, summary, anchors, references"
                    .to_string(),
            );
        }
        let entry = ContractEntry {
            id: text(item, "id"),
            module: text(item, "module"),
            summary: text(item, "summary"),
            anchors: text_list(item, "anchors"),
            references: text_list(item, "references"),
        };
        validate(&entry, &mut ids)?;
        result.push(entry);
    }
    result.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(result)
}

fn validate(entry: &ContractEntry, ids: &mut Vec<String>) -> Result<(), String> {
    if !valid_id(&entry.id) || ids.contains(&entry.id) {
        return Err(format!("Invalid or duplicate contract id: {}", entry.id));
    }
    ids.push(entry.id.clone());
    if !valid_id(&entry.module) || !valid_summary(&entry.summary) {
        return Err(format!(
            "Invalid module or summary for contract {}",
            entry.id
        ));
    }
    if entry.anchors.is_empty()
        || entry.anchors.len() > 16
        || entry.anchors.iter().any(|a| !valid_anchor(a))
    {
        return Err(format!(
            "Invalid behavior anchors for contract {}",
            entry.id
        ));
    }
    if entry.references.is_empty()
        || entry.references.len() > 32
        || entry.references.iter().any(|r| {
            !r.starts_with("references/") || r.contains("..") || r.contains('\\') || r.contains(':')
        })
    {
        return Err(format!("Invalid references for contract {}", entry.id));
    }
    Ok(())
}

/// Java CONTRACT_ID: `[a-z][a-z0-9-]{1,63}`.
fn valid_id(value: &str) -> bool {
    let mut chars = value.chars();
    (2..=64).contains(&value.len())
        && matches!(chars.next(), Some(c) if c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Java SUMMARY: `[A-Za-z0-9][A-Za-z0-9 .,;:/()_+\-]{0,239}`.
fn valid_summary(value: &str) -> bool {
    let mut chars = value.chars();
    (1..=240).contains(&value.len())
        && matches!(chars.next(), Some(c) if c.is_ascii_alphanumeric())
        && chars.all(|c| {
            c.is_ascii_alphanumeric()
                || matches!(
                    c,
                    ' ' | '.' | ',' | ';' | ':' | '/' | '(' | ')' | '_' | '+' | '-'
                )
        })
}

/// A behavior anchor is a fully-qualified Rust path: `::`-separated identifier
/// segments, at least two (the Java ANCHOR_CLASS analog).
fn valid_anchor(value: &str) -> bool {
    let segments: Vec<&str> = value.split("::").collect();
    segments.len() >= 2 && segments.iter().all(|s| valid_identifier(s))
}

fn valid_identifier(segment: &str) -> bool {
    let mut chars = segment.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn text(item: &serde_yaml::Value, key: &str) -> String {
    item.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn text_list(item: &serde_yaml::Value, key: &str) -> Vec<String> {
    item.get(key)
        .and_then(|v| v.as_sequence())
        .map(|list| {
            list.iter()
                .map(|v| v.as_str().unwrap_or_default().to_string())
                .collect()
        })
        .unwrap_or_default()
}
