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

//! Deploys the graph sets a deployment manifest lists (ADR-0027 in the Java repository's ledger; Java
//! `GraphSetLoader`, messages word for word). Beside its loose `graphs`, a manifest may name packaged sets in
//! `sets` and a folder in `unpack`:
//!
//! - each set is read from `{location}/{set}.pack` with the strict read, and its names are checked before any
//!   path is built;
//! - its graphs are unpacked into `{unpack}/{graph-id}.json` and pass the deployment gate the way a loose graph
//!   does, read back with their `${...}` references resolved;
//! - a set registers all of its graphs or none.
//!
//! A generated manifest, `{unpack}/graphs.yaml`, records what was deployed and which files the loader wrote, so
//! the next start removes a graph that a new version of a set no longer holds. The loader never deletes a file it
//! did not write.
//!
//! Precedence follows the manifest list: a manifest's sets compile right after its loose graphs and before the
//! next manifest, and a later graph owns a duplicate id. A duplicate that involves a set is logged as an error.

use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use platform_core::{AppConfigReader, ConfigReader, ConfigValue};
use sha2::{Digest, Sha256};

use crate::graph_set::{self, Contents, GraphSetError};
use crate::graphs::{self, DeployedSet};
use crate::model_gate;

const SETS: &str = "sets";
const UNPACK: &str = "unpack";
/// The generated manifest's file name in the unpack folder.
pub const GENERATED_MANIFEST: &str = "graphs.yaml";
const GENERATED: &str = "generated";
const VERSION: &str = "version";
const FILE: &str = "file:";
const FILE_PREFIX: &str = "file:/";
const CLASSPATH: &str = "classpath:";
const JSON_EXT: &str = ".json";
const DEFAULT_TEMP_DIR: &str = "/tmp/graph";

fn unpack_folders() -> &'static Mutex<HashSet<PathBuf>> {
    static FOLDERS: OnceLock<Mutex<HashSet<PathBuf>>> = OnceLock::new();
    FOLDERS.get_or_init(|| Mutex::new(HashSet::new()))
}

/// A new start: no unpack folder is in use yet.
pub fn reset() {
    unpack_folders()
        .lock()
        .expect("unpack folders poisoned")
        .clear();
}

/// What one manifest's sets left behind: the graphs that deployed, the files written per set (all of them, a
/// refused set's included, for the next start to remove) and one provenance line per set.
#[derive(Default)]
struct Outcome {
    deployed: Vec<String>,
    generated: Vec<(String, Vec<String>)>,
    provenance: Vec<String>,
}

/// Deploy the sets a manifest lists, after the manifest's loose graphs.
pub(crate) fn deploy(manifest: &str, reader: &ConfigReader, location: &str) {
    let sets = set_names(reader);
    if sets.is_empty() {
        return;
    }
    let unpack = reader
        .get_property(UNPACK)
        .unwrap_or_default()
        .trim()
        .to_string();
    if let Some(refusal) = check_unpack(&unpack) {
        log::error!("Graph sets in {manifest} not deployed - {refusal}");
        return;
    }
    // the unpacked graphs are deployed graphs: 'list graphs' and 'import graph from' search this folder too
    graphs::add_deployed_location(&unpack);
    let folder = folder_of(&unpack);
    remove_previous_files(&folder);
    let mut outcome = Outcome::default();
    for set_name in &sets {
        deploy_set(location, &unpack, &folder, set_name, &mut outcome);
    }
    let generated_manifest = folder.join(GENERATED_MANIFEST);
    write_generated_manifest(&generated_manifest, manifest, &unpack, &outcome);
    log::info!(
        "Graph sets in {manifest} unpacked into {unpack} - generated manifest {}",
        generated_manifest.display()
    );
}

fn set_names(reader: &ConfigReader) -> Vec<String> {
    match reader.get(SETS) {
        Some(ConfigValue::List(list)) => (0..list.len())
            .map(|i| {
                reader
                    .get_property(&format!("{SETS}[{i}]"))
                    .unwrap_or_default()
                    .trim()
                    .to_string()
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn check_unpack(unpack: &str) -> Option<String> {
    if unpack.is_empty() {
        return Some("'unpack' names no folder".to_string());
    }
    if !unpack.starts_with(FILE_PREFIX) {
        return Some(format!(
            "'unpack' must be a file:/ folder the application can write, not {unpack}"
        ));
    }
    let folder = folder_of(unpack);
    let temp = folder_of(
        &AppConfigReader::get_instance().get_property_or("location.graph.temp", DEFAULT_TEMP_DIR),
    );
    if folder.starts_with(&temp) {
        return Some(
            "'unpack' must not be the Playground's temporary folder (location.graph.temp) or inside it"
                .to_string(),
        );
    }
    if !unpack_folders()
        .lock()
        .expect("unpack folders poisoned")
        .insert(folder.clone())
    {
        return Some(format!("'unpack' {unpack} is used by another manifest"));
    }
    writable(&folder).map(|problem| format!("cannot write in {unpack} - {problem}"))
}

/// A `file:` location as an absolute path, normalized without touching the file system (Java
/// `Path.toAbsolutePath().normalize()`).
fn folder_of(location: &str) -> PathBuf {
    let path = location.strip_prefix(FILE).unwrap_or(location);
    let absolute = std::path::absolute(path).unwrap_or_else(|_| PathBuf::from(path));
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

fn writable(folder: &Path) -> Option<String> {
    if let Err(e) = std::fs::create_dir_all(folder) {
        return Some(e.to_string());
    }
    let probe = folder.join(format!(".probe-{}.tmp", uuid::Uuid::new_v4().simple()));
    match std::fs::write(&probe, b"") {
        Ok(()) => {
            let _ = std::fs::remove_file(&probe);
            None
        }
        Err(e) => Some(e.to_string()),
    }
}

/// Remove the files the previous start unpacked here, as its generated manifest records them, so a graph that a
/// new version of a set no longer holds does not linger. Nothing else in the folder is touched.
fn remove_previous_files(folder: &Path) {
    let previous = folder.join(GENERATED_MANIFEST);
    if !previous.is_file() {
        return;
    }
    match ConfigReader::load(&format!("{FILE}{}", previous.display())) {
        Ok(reader) => {
            if let Some(ConfigValue::Map(sets)) = reader.get(GENERATED) {
                for ids in sets.values() {
                    let ConfigValue::List(list) = ids else {
                        continue;
                    };
                    for id in list {
                        // a name becomes a path only when it is a valid graph id
                        if let ConfigValue::Text(name) = id {
                            if model_gate::is_valid_graph_id(name) {
                                let _ =
                                    std::fs::remove_file(folder.join(format!("{name}{JSON_EXT}")));
                            }
                        }
                    }
                }
            }
        }
        Err(e) => log::warn!(
            "Unable to clean up with the previous generated manifest {} - {e}",
            previous.display()
        ),
    }
}

fn deploy_set(location: &str, unpack: &str, folder: &Path, set_name: &str, outcome: &mut Outcome) {
    let loaded = match load(location, set_name) {
        Ok(loaded) => loaded,
        Err(not_deployed) => {
            log::error!("Set {set_name} not deployed - {}", not_deployed.reason);
            outcome.provenance.push(format!(
                "set {set_name} not deployed - {}",
                not_deployed.note
            ));
            return;
        }
    };
    let ids: Vec<String> = loaded
        .contents
        .graphs
        .iter()
        .map(|(id, _)| id.clone())
        .collect();
    // the files are written before the gate runs, and stay when it refuses the set, for the operator to inspect
    outcome.generated.push((set_name.to_string(), ids.clone()));
    let provenance = provenance(set_name, &loaded);
    for (id, model) in &loaded.contents.graphs {
        if let Err(e) = std::fs::write(
            folder.join(format!("{id}{JSON_EXT}")),
            graph_set::to_json(model),
        ) {
            log::error!("Set {set_name} not unpacked into {unpack} - {e}");
            outcome
                .provenance
                .push(format!("{provenance} - not unpacked"));
            return;
        }
    }
    let (models, failures) = gate(unpack, &ids);
    if !failures.is_empty() {
        log::error!(
            "Set {set_name} rejected - {} of {} failed: {}",
            failures.len(),
            count(ids.len()),
            failures.join("; ")
        );
        outcome.provenance.push(format!("{provenance} - rejected"));
        return;
    }
    let version = loaded
        .contents
        .manifest_field(VERSION)
        .unwrap_or("")
        .to_string();
    let set = DeployedSet {
        name: set_name.to_string(),
        version: version.clone(),
    };
    register(&set, unpack, models);
    outcome.deployed.extend(ids.iter().cloned());
    outcome.provenance.push(format!("{provenance} - deployed"));
    let version_note = if version.is_empty() {
        String::new()
    } else {
        format!(" (version {version})")
    };
    log::info!(
        "Deployed set {set_name}{version_note} from {} - {} into {unpack}",
        loaded.source,
        count(ids.len())
    );
}

/// A set's package as read: its source path, the SHA-256 of its bytes and its content.
struct Loaded {
    source: String,
    sha256: String,
    contents: Contents,
}

/// A set that is not deployed: the reason for the log and a short note for the generated manifest.
struct NotDeployed {
    reason: String,
    note: String,
}

fn not_deployed(reason: impl Into<String>, note: impl Into<String>) -> NotDeployed {
    NotDeployed {
        reason: reason.into(),
        note: note.into(),
    }
}

fn load(location: &str, set_name: &str) -> Result<Loaded, NotDeployed> {
    // the set name is checked before it becomes part of a path
    if !model_gate::is_valid_graph_id(set_name) {
        return Err(not_deployed(
            "a set name uses letters, digits, '_' and '-' only",
            "invalid set name",
        ));
    }
    let source = normalized_path(location, &format!("{set_name}{}", graph_set::EXTENSION));
    let bytes = read_package(&source).map_err(|e| not_deployed(&e, one_line(&e)))?;
    match graph_set::read(&bytes) {
        Ok(contents) => Ok(Loaded {
            source,
            sha256: Sha256::digest(&bytes)
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect(),
            contents,
        }),
        Err(GraphSetError::Refused(reasons)) => Err(not_deployed(
            reasons.join("; "),
            "its names break the set rules",
        )),
        Err(GraphSetError::Format(message)) => {
            let note = one_line(&message);
            Err(not_deployed(message, note))
        }
    }
}

/// The deployment gate, all or none: every graph is checked before any is registered. Returns the models that
/// pass, read the way a deployed graph is read, and one failure per graph that does not.
fn gate(unpack: &str, ids: &[String]) -> (Vec<(String, rmpv::Value)>, Vec<String>) {
    let mut models = Vec::new();
    let mut failures = Vec::new();
    for id in ids {
        match crate::compiler::load_raw_graph(unpack, id)
            .and_then(|mut model| model_gate::validate(id, &mut model).map(|_| model))
        {
            Ok(model) => models.push((id.clone(), model)),
            Err(e) => failures.push(format!("{id}: {e}")),
        }
    }
    (models, failures)
}

fn register(set: &DeployedSet, unpack: &str, models: Vec<(String, rmpv::Value)>) {
    for (id, model) in models {
        if let Some(previous) = graphs::graph_location(&id) {
            log::error!(
                "Graph {id} from set {} ({unpack}) replaces the copy from {}",
                set.name,
                describe(&previous, graphs::graph_set(&id).as_ref())
            );
            graphs::remove_graph(&id);
        }
        graphs::add_set_graph(&id, model, unpack, set.clone());
    }
}

/// Where a compiled graph came from, for a replacement log: a set and its folder, or a manifest's location.
pub(crate) fn describe(location: &str, set: Option<&DeployedSet>) -> String {
    match set {
        Some(set) => format!("set {} ({location})", set.name),
        None => location.to_string(),
    }
}

fn count(n: usize) -> String {
    format!("{n} {}", if n == 1 { "graph" } else { "graphs" })
}

fn read_package(source: &str) -> Result<Vec<u8>, String> {
    let not_found = || format!("{source} not found");
    if let Some(resource) = source.strip_prefix(CLASSPATH) {
        let path = platform_core::resources::resolve_classpath(resource).ok_or_else(not_found)?;
        return std::fs::read(path).map_err(|e| e.to_string());
    }
    let path = Path::new(source.strip_prefix(FILE).unwrap_or(source));
    if !path.is_file() {
        return Err(not_found());
    }
    std::fs::read(path).map_err(|e| e.to_string())
}

/// Java `normalizedPath`: rejoin the folder on single slashes, keep the scheme prefix, append the file name.
fn normalized_path(folder: &str, filename: &str) -> String {
    let parts: Vec<&str> = folder.split('/').filter(|p| !p.is_empty()).collect();
    format!("{}/{filename}", parts.join("/"))
}

fn provenance(set_name: &str, loaded: &Loaded) -> String {
    let mut line = format!(
        "set {set_name}: {}, SHA-256 {}",
        loaded.source, loaded.sha256
    );
    for (key, value) in &loaded.contents.manifest {
        if key != graph_set::SET && key != "format" && key != "format_version" {
            line.push_str(&format!(", {key}={}", one_line(value)));
        }
    }
    line
}

fn one_line(text: &str) -> String {
    text.replace(['\r', '\n'], " ")
}

fn write_generated_manifest(file: &Path, manifest: &str, unpack: &str, outcome: &Outcome) {
    let mut text = format!(
        "# Generated by the graph-set loader from {manifest} - rewritten at every start; do not edit\n"
    );
    text.push_str(&format!(
        "# Unpacked {}\n",
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ")
    ));
    for line in &outcome.provenance {
        text.push_str(&format!("# {line}\n"));
    }
    text.push_str(if outcome.deployed.is_empty() {
        "graphs: []\n"
    } else {
        "graphs:\n"
    });
    for id in &outcome.deployed {
        text.push_str(&format!("  - '{id}'\n"));
    }
    text.push_str(&format!("location: '{unpack}'\n"));
    // the files each set's graphs were unpacked into, which the next start removes before it unpacks again
    text.push_str(if outcome.generated.is_empty() {
        "generated: {}\n"
    } else {
        "generated:\n"
    });
    for (set, ids) in &outcome.generated {
        text.push_str(&format!("  '{set}':\n"));
        for id in ids {
            text.push_str(&format!("    - '{id}'\n"));
        }
    }
    if let Err(e) = std::fs::write(file, text) {
        log::error!(
            "Unable to write the generated manifest {} - {e}",
            file.display()
        );
    }
}
