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

//! The version-matched skill snapshot (Java `SkillSnapshot` analog): the packaged
//! documentation closure plus the generated installed-contracts.md, rendered once
//! and cached. The REST endpoints serve this snapshot and the offline exporter
//! writes exactly the same bytes, so "what the runtime says" and "what the export
//! contains" cannot differ.
//!
//! All reference bytes are embedded at COMPILE TIME by build.rs from
//! `resources/skill/files.list` (the Maven resource-include analog), so the
//! binary is self-contained under every deployment.
//!
//! Port divergences from the Java module, both structural: (1) `mercury_version`
//! is the workspace-pinned crate version - a Cargo workspace has one lockfile, so
//! the mixed platform-core/event-script assembly the Java app refuses at startup
//! cannot be built here; (2) the packaged `references/llms.txt` replaces Java's
//! llms-link rewrite - the Rust llms.txt links only into `guides/`, so it is
//! self-contained inside the snapshot and included as a first-class reference.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use platform_core::AppError;
use serde_json::json;
use sha2::{Digest, Sha256};

use super::catalog::ContractCatalog;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/skill_files.rs"));
}

pub const MANIFEST: &str = "manifest.json";
pub const INSTALLED_CONTRACTS: &str = "references/installed-contracts.md";
// mkdocs snippet includes must never survive into the snapshot (fail closed;
// the Rust guides use none today - this guards the future)
const INCLUDE_MARKER: &str = "--8<--";

pub struct SkillSnapshot {
    files: BTreeMap<String, Vec<u8>>,
    manifest: serde_json::Value,
    mercury_version: String,
}

impl SkillSnapshot {
    /// The rendered singleton - construction fails closed (panics at first use)
    /// on an unexpanded include, a broken relative link, or a contract
    /// reference missing from the packaged inventory.
    pub fn get_instance() -> &'static SkillSnapshot {
        static INSTANCE: OnceLock<SkillSnapshot> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let mercury_version = env!("CARGO_PKG_VERSION").to_string();
            let files = render(&mercury_version).unwrap_or_else(|e| panic!("{e}"));
            let manifest = build_manifest(&files, &mercury_version);
            SkillSnapshot {
                files,
                manifest,
                mercury_version,
            }
        })
    }

    /// Mercury framework version (workspace-pinned - see the module docs).
    pub fn mercury_version(&self) -> &str {
        &self.mercury_version
    }

    /// Every file of the snapshot except the manifest, path -> content.
    pub fn files(&self) -> &BTreeMap<String, Vec<u8>> {
        &self.files
    }

    /// Deterministic manifest: per-file SHA-256 and a whole-snapshot hash.
    pub fn manifest(&self) -> &serde_json::Value {
        &self.manifest
    }

    /// Read one snapshot file as text, or HTTP-404 for anything else.
    pub fn read_file(&self, path: Option<&str>) -> Result<serde_json::Value, AppError> {
        let name = path.unwrap_or_default();
        match self.files.get(name) {
            Some(content) => Ok(json!({
                "content": String::from_utf8_lossy(content),
                "type": content_type(name),
            })),
            None => Err(AppError::new(
                404,
                format!("Reference {name} is not in this snapshot"),
            )),
        }
    }
}

fn render(mercury_version: &str) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut files: BTreeMap<String, Vec<u8>> = generated::SKILL_FILES
        .iter()
        .map(|(path, bytes)| (path.to_string(), bytes.to_vec()))
        .collect();
    for (path, content) in &files {
        if path.ends_with(".md") && String::from_utf8_lossy(content).contains(INCLUDE_MARKER) {
            return Err(format!("Unexpanded mkdocs include in {path}"));
        }
    }
    files.insert(
        INSTALLED_CONTRACTS.to_string(),
        installed_contracts(mercury_version).into_bytes(),
    );
    for contract in ContractCatalog::get_instance().contracts() {
        for reference in &contract.references {
            if !files.contains_key(reference) {
                return Err(format!(
                    "Contract {} references a file missing from the snapshot: {reference}",
                    contract.id
                ));
            }
        }
    }
    validate_links(&files)?;
    Ok(files)
}

fn installed_contracts(mercury_version: &str) -> String {
    let mut out = String::new();
    out.push_str("# Installed Mercury contracts\n\n");
    out.push_str(&format!("- Mercury version: `{mercury_version}`\n"));
    out.push_str("- Engine: `rust`\n\n");
    for contract in ContractCatalog::get_instance().contracts() {
        out.push_str(&format!(
            "## `{}`\n\n{} (module `{}`)\n\nBehavior anchors:\n",
            contract.id, contract.summary, contract.module
        ));
        for anchor in &contract.anchors {
            out.push_str(&format!("- `{anchor}`\n"));
        }
        out.push_str("\nReferences:\n");
        for reference in &contract.references {
            // installed-contracts.md lives under references/, so links are relative to it
            let relative = reference.trim_start_matches("references/");
            out.push_str(&format!("- [{reference}]({relative})\n"));
        }
        out.push('\n');
    }
    out
}

/// Validate every relative markdown link in the snapshot (Java `validateLinks`):
/// a link must resolve to another member of the snapshot - never escape it.
fn validate_links(files: &BTreeMap<String, Vec<u8>>) -> Result<(), String> {
    for (path, content) in files {
        if !path.ends_with(".md") {
            continue;
        }
        let text = String::from_utf8_lossy(content);
        for target in markdown_link_targets(&text) {
            validate_link(files, path, target.trim())?;
        }
    }
    Ok(())
}

/// Extract `](target)` markdown link targets (the Java MARKDOWN_LINK regex analog).
fn markdown_link_targets(text: &str) -> Vec<&str> {
    let mut targets = Vec::new();
    let mut rest = text;
    while let Some(open) = rest.find("](") {
        let after = &rest[open + 2..];
        match after.find(')') {
            Some(close) => {
                targets.push(&after[..close]);
                rest = &after[close + 1..];
            }
            None => break,
        }
    }
    targets
}

fn validate_link(
    files: &BTreeMap<String, Vec<u8>>,
    source: &str,
    target: &str,
) -> Result<(), String> {
    if target.is_empty()
        || target.starts_with('#')
        || target.starts_with("//")
        || has_uri_scheme(target)
    {
        return Ok(());
    }
    let local = target.split(['#', '?']).next().unwrap_or_default().trim();
    let parent = match source.rfind('/') {
        Some(index) => &source[..index],
        None => "",
    };
    match resolve_relative(parent, local) {
        Some(resolved) if files.contains_key(&resolved) => Ok(()),
        _ => Err(format!("Broken relative link in {source} -> {target}")),
    }
}

/// Lexically resolve a relative target against a parent directory; None when
/// the path is absolute or escapes the snapshot root.
fn resolve_relative(parent: &str, target: &str) -> Option<String> {
    if target.starts_with('/') {
        return None;
    }
    let mut segments: Vec<&str> = if parent.is_empty() {
        Vec::new()
    } else {
        parent.split('/').collect()
    };
    for segment in target.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop()?;
            }
            other => segments.push(other),
        }
    }
    Some(segments.join("/"))
}

fn has_uri_scheme(target: &str) -> bool {
    let Some(colon) = target.find(':') else {
        return false;
    };
    let scheme = &target[..colon];
    let mut chars = scheme.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
}

fn build_manifest(files: &BTreeMap<String, Vec<u8>>, mercury_version: &str) -> serde_json::Value {
    let mut snapshot_input = String::new();
    let mut entries = Vec::new();
    for (path, content) in files {
        let hash = sha256_hex(content);
        snapshot_input.push_str(path);
        snapshot_input.push('\n');
        snapshot_input.push_str(&hash);
        snapshot_input.push('\n');
        entries.push(json!({"path": path, "sha256": hash}));
    }
    json!({
        "type": "mercury-platform-skill",
        "mercury_version": mercury_version,
        "snapshot_sha256": sha256_hex(snapshot_input.as_bytes()),
        "files": entries,
    })
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn content_type(path: &str) -> &'static str {
    if path.ends_with(".md") {
        "text/markdown"
    } else if path.ends_with(".json") {
        "application/json"
    } else {
        "text/plain"
    }
}
