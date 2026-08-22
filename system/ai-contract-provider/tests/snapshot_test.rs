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

//! Twin of the Java `SkillSnapshotTest`: the packaged inventory equals the
//! documentation closure on disk (a new guide file fails this test until it is
//! added to files.list), the manifest recomputes from the served bytes, and the
//! reference reader honors the 404 contract.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

use app::snapshot::{sha256_hex, SkillSnapshot, INSTALLED_CONTRACTS};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn files_list() -> BTreeSet<String> {
    let text = fs::read_to_string(manifest_dir().join("resources/skill/files.list"))
        .expect("read files.list");
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

fn walk(root: &Path, prefix: &str, into: &mut BTreeSet<String>) {
    for entry in fs::read_dir(root).expect("read docs dir") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            walk(&path, &format!("{prefix}{name}/"), into);
        } else {
            into.insert(format!("{prefix}{name}"));
        }
    }
}

#[test]
fn inventory_equals_the_documentation_closure() {
    let docs = manifest_dir().join("../../docs");
    let mut expected: BTreeSet<String> = [
        "SKILL.md",
        "security.json",
        "references/llms.txt",
        "references/index.md",
        "references/arch-decisions/ADR.md",
        "references/background/port-scope.md",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    // the full guides and test-reports trees are always packaged - a new file
    // there fails this test until files.list carries it
    walk(&docs.join("guides"), "references/guides/", &mut expected);
    walk(
        &docs.join("test-reports"),
        "references/test-reports/",
        &mut expected,
    );
    assert_eq!(files_list(), expected);
}

#[test]
fn snapshot_serves_the_inventory_plus_installed_contracts() {
    let snapshot = SkillSnapshot::get_instance();
    let mut expected = files_list();
    expected.insert(INSTALLED_CONTRACTS.to_string());
    let served: BTreeSet<String> = snapshot.files().keys().cloned().collect();
    assert_eq!(served, expected);
    assert_eq!(snapshot.mercury_version(), env!("CARGO_PKG_VERSION"));
    // every contract reference is a member of the snapshot
    for contract in app::catalog::ContractCatalog::get_instance().contracts() {
        for reference in &contract.references {
            assert!(served.contains(reference), "missing reference {reference}");
        }
    }
    // the generated inventory page names the version, the engine and every contract
    let installed = String::from_utf8_lossy(&snapshot.files()[INSTALLED_CONTRACTS]).to_string();
    assert!(installed.contains(&format!("`{}`", env!("CARGO_PKG_VERSION"))));
    assert!(installed.contains("- Engine: `rust`"));
    for id in [
        "platform-core",
        "rest-automation",
        "event-script",
        "minigraph",
    ] {
        assert!(installed.contains(&format!("## `{id}`")));
    }
}

#[test]
fn manifest_recomputes_from_the_served_bytes() {
    let snapshot = SkillSnapshot::get_instance();
    let manifest = snapshot.manifest();
    assert_eq!(manifest["type"], "mercury-platform-skill");
    assert_eq!(manifest["mercury_version"], env!("CARGO_PKG_VERSION"));
    let entries = manifest["files"].as_array().expect("files array");
    assert_eq!(entries.len(), snapshot.files().len());
    let mut snapshot_input = String::new();
    for entry in entries {
        let path = entry["path"].as_str().expect("path");
        let hash = entry["sha256"].as_str().expect("sha256");
        let recomputed = sha256_hex(&snapshot.files()[path]);
        assert_eq!(hash, recomputed, "hash drift for {path}");
        snapshot_input.push_str(path);
        snapshot_input.push('\n');
        snapshot_input.push_str(hash);
        snapshot_input.push('\n');
    }
    assert_eq!(
        manifest["snapshot_sha256"].as_str().expect("snapshot hash"),
        sha256_hex(snapshot_input.as_bytes())
    );
}

#[test]
fn reference_reader_honors_the_contract() {
    let snapshot = SkillSnapshot::get_instance();
    let skill = snapshot.read_file(Some("SKILL.md")).expect("SKILL.md");
    assert_eq!(skill["type"], "text/markdown");
    assert!(skill["content"]
        .as_str()
        .expect("content")
        .contains("name: mercury-platform"));
    let catalog = snapshot
        .read_file(Some(
            "references/guides/event-script/event-script-flow.json",
        ))
        .expect("json catalog");
    assert_eq!(catalog["type"], "application/json");
    let llms = snapshot
        .read_file(Some("references/llms.txt"))
        .expect("llms.txt");
    assert_eq!(llms["type"], "text/plain");
    for missing in [
        None,
        Some(""),
        Some("references/none.md"),
        Some("manifest.json"),
    ] {
        let denied = snapshot.read_file(missing);
        assert!(denied.is_err(), "expected 404 for {missing:?}");
    }
}
