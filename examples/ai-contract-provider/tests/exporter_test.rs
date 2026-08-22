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

//! Twin of the Java `OfflineSkillWriterTest`: two exports of the same build are
//! byte-identical, the exporter never overwrites an existing snapshot (and the
//! refusal leaves the first export untouched), and invalid roots are rejected.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

use app::exporter::{export, SKILL_DIRECTORY};
use app::snapshot::{sha256_hex, SkillSnapshot};

fn fresh_root(label: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "ai-contract-provider-{label}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create export root");
    root
}

fn tree_bytes(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut result = BTreeMap::new();
    collect(root, "", &mut result);
    result
}

fn collect(root: &Path, prefix: &str, into: &mut BTreeMap<String, Vec<u8>>) {
    for entry in fs::read_dir(root).expect("read export dir") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            collect(&path, &format!("{prefix}{name}/"), into);
        } else {
            into.insert(
                format!("{prefix}{name}"),
                fs::read(&path).expect("read file"),
            );
        }
    }
}

#[test]
fn two_exports_are_byte_identical_and_never_overwritten() {
    let root_one = fresh_root("one");
    let root_two = fresh_root("two");
    let first = export(Some(&root_one.display().to_string())).expect("first export");
    let second = export(Some(&root_two.display().to_string())).expect("second export");
    let snapshot = SkillSnapshot::get_instance();
    // the result names the target and counts snapshot files + manifest.json
    let target = root_one.join(SKILL_DIRECTORY);
    assert_eq!(first["skill_directory"], target.display().to_string());
    assert_eq!(first["files"], (snapshot.files().len() + 1) as u64);
    assert_eq!(
        first["snapshot_sha256"],
        snapshot.manifest()["snapshot_sha256"]
    );
    // byte-identical across the two exports
    let tree_one = tree_bytes(&target);
    let tree_two = tree_bytes(&root_two.join(SKILL_DIRECTORY));
    assert_eq!(tree_one, tree_two);
    assert_eq!(tree_one.len(), snapshot.files().len() + 1);
    assert_eq!(second["snapshot_sha256"], first["snapshot_sha256"]);
    // the exported manifest parses and matches the served manifest
    let manifest: serde_json::Value =
        serde_json::from_slice(&tree_one["manifest.json"]).expect("manifest json");
    assert_eq!(&manifest, snapshot.manifest());
    // a second export into the SAME root is refused (409) and mutates nothing
    let skill_before = sha256_hex(&tree_one["SKILL.md"]);
    let refused = export(Some(&root_one.display().to_string()));
    assert!(
        refused.is_err(),
        "expected refusal over an existing snapshot"
    );
    let skill_after = sha256_hex(&fs::read(target.join("SKILL.md")).expect("re-read"));
    assert_eq!(
        skill_before, skill_after,
        "refusal must not touch the snapshot"
    );
    let _ = fs::remove_dir_all(&root_one);
    let _ = fs::remove_dir_all(&root_two);
}

#[test]
fn invalid_export_roots_are_rejected() {
    assert!(export(None).is_err(), "missing directory");
    assert!(export(Some("   ")).is_err(), "blank directory");
    let missing = std::env::temp_dir().join(format!(
        "ai-contract-provider-missing-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&missing);
    assert!(
        export(Some(&missing.display().to_string())).is_err(),
        "nonexistent root"
    );
}
