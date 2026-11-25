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
//! Hot deploy and undeploy of graph sets (RFC-0017 in the Java repository's register; Java
//! `HotDeployment`): a certified set deployed to a running engine, registered through the
//! deployment gate, and removed by name — without a restart.
//!
//! The hot folder (`location.graph.deploy`, default `file:/tmp/deploy`) holds, per set, the
//! package `<id>.pack`, one unpacked `<graph-id>.json` per graph, and one engine-written record
//! `<id>.deployed.yaml` — the set's deployment record and its revision marker in one. A
//! hot-deployable set has exactly one main graph and is named after it, so the set name is the
//! main graph's id and one string is the handle everywhere. Two switches, both off by default,
//! govern the folder: `graph.hot.deploy` (a set may be deployed) and `graph.hot.undeploy` (a set
//! may be removed); either on makes the engine read the folder at startup and on demand.
//!
//! - a configured manifest owns its ids: a set that holds a bundled id is refused rather than
//!   shadowing it; one hot set owns an id; a re-upload of the same set name replaces the set all
//!   or none;
//! - the registry is replaced in one swap per deployment, so a run never sees half a set;
//! - the records are the allowlist: a package copied into the folder by hand is ignored with a
//!   warning;
//! - every other instance converges on demand: a registry lookup for a hot-deployed id reads the
//!   set's record and reconciles that set when the record changed or is gone, and a lookup for
//!   an unknown id checks whether a record of that name exists — no poll, no broadcast, no
//!   global file.
//!
//! The REST entries that call [`deploy`], [`undeploy`] and [`list`] are in `rest.rs`; this module
//! is the engine side they share with startup.
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock, RwLock};

use platform_core::{AppConfigReader, AppError, ConfigReader, ConfigValue};

use crate::graph_set::{self, Contents, GraphSetError};
use crate::graphs::{self, DeployedSet};
use crate::{graph_set_loader, model_gate};

pub const DEPLOY_KEY: &str = "graph.hot.deploy";
pub const UNDEPLOY_KEY: &str = "graph.hot.undeploy";
pub const LOCATION_KEY: &str = "location.graph.deploy";
pub const THROTTLE_KEY: &str = "graph.hot.deploy.throttle";
pub const DEFAULT_LOCATION: &str = "file:/tmp/deploy";
pub const RECORD_EXTENSION: &str = ".deployed.yaml";
const JSON_EXT: &str = ".json";
const FILE: &str = "file:";
const FILE_PREFIX: &str = "file:/";
const STAGING_PREFIX: &str = ".staging-";
const DEFAULT_TEMP_DIR: &str = "/tmp/graph";
const NOT_DEPLOYED: &str = "not deployed - ";
const DEPLOYED: &str = "deployed";
const OFF_FOR_THIS_RUN: &str = "Hot deployment is OFF for this run - ";
const CANNOT_WRITE: &str = "Unable to write the hot deployment folder - ";
const SET_FIELD: &str = graph_set::SET;
const VERSION_FIELD: &str = "version";
const SHA256_FIELD: &str = "sha256";
const DEPLOYED_AT_FIELD: &str = "deployed_at";
const GRAPHS_FIELD: &str = "graphs";
const FILES_FIELD: &str = "files";

/// The folder's configuration for this run.
#[derive(Clone, Debug)]
struct Settings {
    deploy: bool,
    undeploy: bool,
    location: String,
    /// The folder as a path, or `None` when both switches are off or the folder is unusable.
    folder: Option<PathBuf>,
    throttle_ms: i64,
}

impl Settings {
    fn off() -> Self {
        Settings {
            deploy: false,
            undeploy: false,
            location: DEFAULT_LOCATION.to_string(),
            folder: None,
            throttle_ms: 0,
        }
    }

    fn enabled(&self) -> bool {
        self.folder.is_some()
    }
}

fn settings() -> &'static RwLock<Settings> {
    static SETTINGS: OnceLock<RwLock<Settings>> = OnceLock::new();
    SETTINGS.get_or_init(|| RwLock::new(Settings::off()))
}

fn current_settings() -> Settings {
    settings()
        .read()
        .expect("hot deployment settings poisoned")
        .clone()
}

/// A set registered from the hot folder on this instance.
#[derive(Clone, Debug)]
struct Installed {
    sha256: String,
    version: String,
    graphs: Vec<String>,
    deployed_at: String,
    /// The record as read when the set was registered, for the cheap "unchanged" check.
    record_bytes: Vec<u8>,
}

/// What this instance knows of the folder's sets; deploy, undeploy and every reconcile serialize
/// on its lock, and a lookup reads it without holding it longer than a map lookup.
#[derive(Default)]
struct State {
    by_set: HashMap<String, Installed>,
    refusals: HashMap<String, String>,
    last_check: HashMap<String, i64>,
}

fn state() -> &'static Mutex<State> {
    static STATE: OnceLock<Mutex<State>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(State::default()))
}

/// A set's record as read from `<id>.deployed.yaml`.
#[derive(Clone, Debug)]
struct DeployRecord {
    set: String,
    version: String,
    sha256: String,
    deployed_at: String,
    graphs: Vec<String>,
    files: Vec<String>,
}

/// The answer of a deploy or an undeploy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deployment {
    /// The set name, which is its main graph's id.
    pub set: String,
    /// The set's version field, empty when the set has none.
    pub version: String,
    /// The package's digest.
    pub sha256: String,
    /// The graph ids the set holds (deploy) or held (undeploy).
    pub graphs: Vec<String>,
    /// The record's time.
    pub deployed_at: String,
    /// The previous version and digest when the set was already deployed.
    pub replaced: Option<(String, String)>,
    /// False when the same bytes were already deployed and nothing moved.
    pub changed: bool,
}

/// One row of the list of deployed sets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetStatus {
    pub set: String,
    pub version: String,
    pub sha256: String,
    pub deployed_at: String,
    pub graphs: Vec<String>,
    /// `deployed`, or `not deployed - ` and the reason on this instance.
    pub status: String,
}

fn refused(status: i32, message: impl Into<String>) -> AppError {
    AppError::new(status, message)
}

/// Configure from the application's properties: the two switches, the folder and the throttle.
/// Either switch on makes the folder live: it must be a writable `file:/` folder outside the
/// Playground's temporary folder. A folder that cannot be used turns hot deployment off for this
/// run, with an error.
pub fn configure() {
    let config = AppConfigReader::get_instance();
    let deploy = config
        .get_property_or(DEPLOY_KEY, "false")
        .trim()
        .eq_ignore_ascii_case("true");
    let undeploy = config
        .get_property_or(UNDEPLOY_KEY, "false")
        .trim()
        .eq_ignore_ascii_case("true");
    let location = config.get_property_or(LOCATION_KEY, DEFAULT_LOCATION);
    let throttle = config
        .get_property_or(THROTTLE_KEY, "0")
        .trim()
        .parse::<i64>()
        .unwrap_or(0)
        .max(0);
    configure_with(deploy, undeploy, location.trim(), throttle);
}

/// Configure explicitly (the test seam of [`configure`]); a new configuration forgets every set
/// this instance had registered from the previous folder.
pub fn configure_with(deploy: bool, undeploy: bool, location: &str, throttle_ms: i64) {
    let mut st = state().lock().expect("hot deployment state poisoned");
    st.by_set.clear();
    st.refusals.clear();
    st.last_check.clear();
    if !deploy && !undeploy {
        *settings()
            .write()
            .expect("hot deployment settings poisoned") = Settings::off();
        graphs::set_refresh(None);
        return;
    }
    if let Some(problem) = check_folder(location) {
        log::error!("{OFF_FOR_THIS_RUN}{problem}");
        *settings()
            .write()
            .expect("hot deployment settings poisoned") = Settings::off();
        graphs::set_refresh(None);
        return;
    }
    let folder = graph_set_loader::folder_of(location);
    *settings()
        .write()
        .expect("hot deployment settings poisoned") = Settings {
        deploy,
        undeploy,
        location: location.to_string(),
        folder: Some(folder),
        throttle_ms,
    };
    graphs::set_refresh(Some(check));
    log::info!(
        "Hot deployment folder ({LOCATION_KEY}) - {location}, deploy={deploy}, undeploy={undeploy}"
    );
}

/// Return to the application's own configuration, forgetting every set a test registered from
/// another folder.
pub fn reset() {
    configure();
}

fn check_folder(location: &str) -> Option<String> {
    if !location.starts_with(FILE_PREFIX) {
        return Some(format!(
            "{LOCATION_KEY} must be a file:/ folder the application can write, not {location}"
        ));
    }
    let folder = graph_set_loader::folder_of(location);
    let temp = graph_set_loader::folder_of(
        &AppConfigReader::get_instance().get_property_or("location.graph.temp", DEFAULT_TEMP_DIR),
    );
    if folder.starts_with(&temp) || temp.starts_with(&folder) {
        return Some(format!(
            "{LOCATION_KEY} must not be the Playground's temporary folder (location.graph.temp), \
             inside it or above it"
        ));
    }
    if let Err(e) = std::fs::create_dir_all(&folder) {
        return Some(format!("cannot write in {location} - {e}"));
    }
    let probe = folder.join(format!(".probe-{}", uuid::Uuid::new_v4().simple()));
    match std::fs::write(&probe, b"") {
        Ok(()) => {
            let _ = std::fs::remove_file(&probe);
            None
        }
        Err(e) => Some(format!("cannot write in {location} - {e}")),
    }
}

/// True when either switch is on and the folder is usable.
pub fn is_enabled() -> bool {
    current_settings().enabled()
}

/// True when a set may be deployed through this instance.
pub fn can_deploy() -> bool {
    let s = current_settings();
    s.enabled() && s.deploy
}

/// True when a set may be undeployed through this instance.
pub fn can_undeploy() -> bool {
    let s = current_settings();
    s.enabled() && s.undeploy
}

/// The hot folder as a path, or `None` when hot deployment is off.
pub fn folder() -> Option<PathBuf> {
    current_settings().folder
}

/// The hot folder as configured (`file:/...`).
pub fn location() -> String {
    current_settings().location
}

/// Whether a graph is a private subgraph of a hot-deployed set — any graph of such a set but its
/// main graph. A private subgraph is reached from its own set through `graph.extension` and from
/// nowhere else.
pub fn is_private_subgraph(graph_id: &str) -> bool {
    let s = current_settings();
    if !s.enabled() || graphs::graph_location(graph_id).as_deref() != Some(s.location.as_str()) {
        return false;
    }
    graphs::graph_set(graph_id).is_some_and(|set| set.name != graph_id)
}

/// Read the folder at startup, after the configured manifests: every set a record names is
/// registered through the gate, a package no record names is noted, and a folder that another
/// manifest unpacks into is refused.
pub fn reconcile_all() {
    let s = current_settings();
    let Some(folder) = s.folder.clone() else {
        return;
    };
    if graph_set_loader::meets_unpack_folder(&folder) {
        log::error!(
            "{OFF_FOR_THIS_RUN}{LOCATION_KEY} {} is a manifest's 'unpack' folder, inside one or above one",
            s.location
        );
        *settings()
            .write()
            .expect("hot deployment settings poisoned") = Settings::off();
        graphs::set_refresh(None);
        return;
    }
    graphs::add_deployed_location(&s.location);
    let mut st = state().lock().expect("hot deployment state poisoned");
    remove_staging(&folder);
    let sets = recorded_sets(&folder);
    for id in &sets {
        reconcile(&mut st, &s, id);
    }
    for stray in stray_packages(&folder, &sets) {
        log::warn!(
            "Package {stray} in {} has no record and is ignored - a deployment goes through the deploy endpoint",
            s.location
        );
    }
    log::info!(
        "Hot deployment folder {} - {}",
        s.location,
        count(st.by_set.len())
    );
}

/// The refresh hook: a lookup for a hot-deployed id reads its set's record and reconciles the set
/// when the record changed or is gone; a lookup for an unknown id checks whether a record of that
/// name exists and loads the set if it does. A manifest's id costs nothing.
pub fn check(graph_id: &str, location: Option<&str>) {
    let s = current_settings();
    let Some(folder) = s.folder.as_ref() else {
        return;
    };
    match location {
        None => {
            if model_gate::is_valid_graph_id(graph_id)
                && record_path(folder, graph_id).is_file()
                && !throttled(&s, graph_id)
            {
                let mut st = state().lock().expect("hot deployment state poisoned");
                reconcile(&mut st, &s, graph_id);
            }
        }
        Some(loc) if loc == s.location => {
            let Some(set) = graphs::graph_set(graph_id) else {
                return;
            };
            if throttled(&s, &set.name) {
                return;
            }
            let on_disk = read_bytes(&record_path(folder, &set.name));
            let mut st = state().lock().expect("hot deployment state poisoned");
            let unchanged = st.by_set.get(&set.name).is_some_and(|installed| {
                on_disk.as_deref() == Some(installed.record_bytes.as_slice())
            });
            if !unchanged {
                reconcile(&mut st, &s, &set.name);
            }
        }
        Some(_) => {}
    }
}

fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn throttled(s: &Settings, set_name: &str) -> bool {
    if s.throttle_ms <= 0 {
        return false;
    }
    let now = now_millis();
    let mut st = state().lock().expect("hot deployment state poisoned");
    if let Some(last) = st.last_check.get(set_name) {
        if now - last < s.throttle_ms {
            return true;
        }
    }
    st.last_check.insert(set_name.to_string(), now);
    false
}

/// Bring one set to the state its record describes — the caller holds the state lock.
fn reconcile(st: &mut State, s: &Settings, set_name: &str) {
    let Some(folder) = s.folder.as_ref() else {
        return;
    };
    let record_file = record_path(folder, set_name);
    let Some(record_bytes) = read_bytes(&record_file) else {
        if st.by_set.contains_key(set_name) {
            unregister(st, set_name, s);
            log::info!("Set {set_name} undeployed elsewhere - its record is gone");
        }
        st.refusals.remove(set_name);
        return;
    };
    if st
        .by_set
        .get(set_name)
        .is_some_and(|installed| installed.record_bytes == record_bytes)
    {
        return;
    }
    let Some(agreed) = agreed_record(st, s, set_name, &record_file, record_bytes) else {
        return;
    };
    if let Some(installed) = st.by_set.get(set_name) {
        if installed.sha256 == agreed.sha256 {
            // the record was rewritten with the same digest - nothing to load
            let refreshed = Installed {
                deployed_at: agreed.record.deployed_at.clone(),
                record_bytes: agreed.record_bytes.clone(),
                ..installed.clone()
            };
            st.by_set.insert(set_name.to_string(), refreshed);
            return;
        }
    }
    let outcome = read_set(Some(set_name), &agreed.pack_bytes)
        .and_then(|contents| check_ownership(set_name, &contents, s).map(|_| contents))
        .and_then(|contents| install(st, set_name, &contents, &agreed, s));
    if let Err(e) = outcome {
        refuse(st, set_name, e.message());
    }
}

/// A record and a package that agree on the digest.
struct Agreed {
    record: DeployRecord,
    record_bytes: Vec<u8>,
    pack_bytes: Vec<u8>,
    sha256: String,
}

fn agreed_record(
    st: &mut State,
    s: &Settings,
    set_name: &str,
    record_file: &Path,
    first_bytes: Vec<u8>,
) -> Option<Agreed> {
    let attempt = agreement(s, set_name, record_file, Some(first_bytes))
        // a deploy may have moved the package before its record: once more, from the file
        .or_else(|| agreement(s, set_name, record_file, read_bytes(record_file)));
    if attempt.is_none() {
        if st.by_set.contains_key(set_name) {
            unregister(st, set_name, s);
        }
        refuse(st, set_name, "the package does not match its record");
    }
    attempt
}

fn agreement(
    s: &Settings,
    set_name: &str,
    record_file: &Path,
    record_bytes: Option<Vec<u8>>,
) -> Option<Agreed> {
    let record_bytes = record_bytes?;
    let record = parse_record(record_file)?;
    if record.set != set_name || record.sha256.is_empty() {
        return None;
    }
    let folder = s.folder.as_ref()?;
    let pack_bytes = read_bytes(&folder.join(format!("{set_name}{}", graph_set::EXTENSION)))?;
    let sha256 = graph_set_loader::sha256_hex(&pack_bytes);
    (sha256 == record.sha256).then_some(Agreed {
        record,
        record_bytes,
        pack_bytes,
        sha256,
    })
}

fn refuse(st: &mut State, set_name: &str, reason: &str) {
    st.refusals.insert(set_name.to_string(), reason.to_string());
    log::error!("Set {set_name} {NOT_DEPLOYED}{reason}");
}

fn unregister(st: &mut State, set_name: &str, s: &Settings) {
    let ids = graphs::graphs_of_set(set_name, &s.location);
    graphs::replace_graphs(&ids, Vec::new(), &s.location, None);
    st.by_set.remove(set_name);
}

/// Deploy a set from the bytes of its package: the strict read, the main-graph rule, the
/// ownership rule, the files staged and gated, then one swap of the registry and the record
/// written last. The error carries the HTTP status (400, 409, 500, 503) and every reason.
pub fn deploy(bytes: &[u8], expected_sha256: Option<&str>) -> Result<Deployment, AppError> {
    let s = current_settings();
    if !s.enabled() || !s.deploy {
        return Err(refused(
            503,
            format!("hot deploy is off on this engine ({DEPLOY_KEY})"),
        ));
    }
    if bytes.is_empty() {
        return Err(refused(
            400,
            "The request body is the graph set (.pack) to deploy, sent as application/octet-stream",
        ));
    }
    let sha256 = graph_set_loader::sha256_hex(bytes);
    if let Some(expected) = expected_sha256 {
        if !expected.trim().eq_ignore_ascii_case(&sha256) {
            return Err(refused(
                400,
                format!("SHA-256 does not match - the body's digest is {sha256}"),
            ));
        }
    }
    let contents = read_set(None, bytes)?;
    let set_name = contents
        .manifest_field(SET_FIELD)
        .unwrap_or_default()
        .to_string();
    let folder = s.folder.clone().expect("enabled settings have a folder");
    let mut st = state().lock().expect("hot deployment state poisoned");
    check_ownership(&set_name, &contents, &s)?;
    if let Some(installed) = st.by_set.get(&set_name) {
        if installed.sha256 == sha256 {
            return Ok(Deployment {
                set: set_name,
                version: installed.version.clone(),
                sha256,
                graphs: installed.graphs.clone(),
                deployed_at: installed.deployed_at.clone(),
                replaced: None,
                changed: false,
            });
        }
    }
    let version = contents
        .manifest_field(VERSION_FIELD)
        .unwrap_or_default()
        .to_string();
    let ids: Vec<String> = contents.graphs.iter().map(|(id, _)| id.clone()).collect();
    let deployed_at = now_iso();
    let mut files = vec![format!("{set_name}{}", graph_set::EXTENSION)];
    files.extend(ids.iter().map(|id| format!("{id}{JSON_EXT}")));
    let record = DeployRecord {
        set: set_name.clone(),
        version: version.clone(),
        sha256: sha256.clone(),
        deployed_at: deployed_at.clone(),
        graphs: ids.clone(),
        files,
    };
    let record_bytes = record_text(&record).into_bytes();
    let agreed = Agreed {
        record,
        record_bytes: record_bytes.clone(),
        pack_bytes: bytes.to_vec(),
        sha256: sha256.clone(),
    };
    let replaced = st
        .by_set
        .get(&set_name)
        .map(|installed| (installed.version.clone(), installed.sha256.clone()));
    install(&mut st, &set_name, &contents, &agreed, &s)?;
    write_atomically(&record_path(&folder, &set_name), &record_bytes, &folder)
        .map_err(|e| refused(500, format!("{CANNOT_WRITE}{e}")))?;
    log::info!(
        "Deployed set {set_name}{} - {} into {} (SHA-256 {sha256})",
        if version.is_empty() {
            String::new()
        } else {
            format!(" (version {version})")
        },
        graph_count(ids.len()),
        s.location
    );
    Ok(Deployment {
        set: set_name,
        version,
        sha256,
        graphs: ids,
        deployed_at,
        replaced,
        changed: true,
    })
}

/// Undeploy a set by its main graph's id: its graphs leave the registry in one swap, the record
/// is deleted first and then the files it named. 404 when the folder holds no such set.
pub fn undeploy(graph_id: &str) -> Result<Deployment, AppError> {
    let s = current_settings();
    if !s.enabled() || !s.undeploy {
        return Err(refused(
            503,
            format!("hot undeploy is off on this engine ({UNDEPLOY_KEY})"),
        ));
    }
    if !model_gate::is_valid_graph_id(graph_id) {
        return Err(refused(
            400,
            "a set name uses letters, digits, '_' and '-' only",
        ));
    }
    let folder = s.folder.clone().expect("enabled settings have a folder");
    let mut st = state().lock().expect("hot deployment state poisoned");
    let record_file = record_path(&folder, graph_id);
    let Some(record) = parse_record(&record_file) else {
        return Err(refused(
            404,
            format!("set {graph_id} is not deployed in {}", s.location),
        ));
    };
    let graphs_here = graphs::graphs_of_set(graph_id, &s.location);
    unregister(&mut st, graph_id, &s);
    st.refusals.remove(graph_id);
    std::fs::remove_file(&record_file)
        .map_err(|e| refused(500, format!("Unable to remove the set's files - {e}")))?;
    for name in &record.files {
        delete_own_file(&folder, name)
            .map_err(|e| refused(500, format!("Unable to remove the set's files - {e}")))?;
    }
    log::info!(
        "Undeployed set {graph_id} - {} from {}",
        graph_count(graphs_here.len()),
        s.location
    );
    let ids = if graphs_here.is_empty() {
        record.graphs.clone()
    } else {
        graphs_here
    };
    Ok(Deployment {
        set: graph_id.to_string(),
        version: record.version,
        sha256: record.sha256,
        graphs: ids,
        deployed_at: record.deployed_at,
        replaced: None,
        changed: true,
    })
}

/// The deployed sets as the folder records them, each reconciled first, with its status on this
/// instance, by set name.
pub fn list() -> Vec<SetStatus> {
    let s = current_settings();
    let mut result = Vec::new();
    let Some(folder) = s.folder.clone() else {
        return result;
    };
    let mut st = state().lock().expect("hot deployment state poisoned");
    for set_name in recorded_sets(&folder) {
        reconcile(&mut st, &s, &set_name);
        if let Some(record) = parse_record(&record_path(&folder, &set_name)) {
            let status = if st.by_set.contains_key(&set_name) {
                DEPLOYED.to_string()
            } else {
                format!(
                    "{NOT_DEPLOYED}{}",
                    st.refusals
                        .get(&set_name)
                        .cloned()
                        .unwrap_or_else(|| "not registered on this instance".to_string())
                )
            };
            result.push(SetStatus {
                set: set_name,
                version: record.version,
                sha256: record.sha256,
                deployed_at: record.deployed_at,
                graphs: record.graphs,
                status,
            });
        }
    }
    result
}

fn read_set(expected_name: Option<&str>, bytes: &[u8]) -> Result<Contents, AppError> {
    let contents = match graph_set::read(bytes) {
        Ok(contents) => contents,
        Err(GraphSetError::Refused(reasons)) => return Err(refused(400, reasons.join("; "))),
        Err(GraphSetError::Format(message)) => {
            return Err(refused(400, format!("Not a graph set - {message}")))
        }
    };
    let set_name = contents.manifest_field(SET_FIELD).unwrap_or_default();
    if !model_gate::is_valid_graph_id(set_name) {
        return Err(refused(
            400,
            format!(
                "manifest field '{SET_FIELD}' - a set name uses letters, digits, '_' and '-' only"
            ),
        ));
    }
    if let Some(expected) = expected_name {
        if expected != set_name {
            return Err(refused(
                400,
                format!("the package is set '{set_name}', not '{expected}'"),
            ));
        }
    }
    // a hot-deployable set has exactly one main graph and is named after it (D14)
    if contents.graph(set_name).is_none() {
        return Err(refused(
            400,
            format!(
                "a hot-deployable set is named after its main graph - the set holds no graph '{set_name}'"
            ),
        ));
    }
    if let Some(entry_point) = contents.manifest_field(graph_set::GRAPH_ID) {
        if entry_point != set_name {
            return Err(refused(
                400,
                format!(
                    "manifest field '{}' is '{entry_point}' - it must name the main graph, which is the set name '{set_name}'",
                    graph_set::GRAPH_ID
                ),
            ));
        }
    }
    Ok(contents)
}

/// A configured manifest owns its ids, and one hot set owns an id (D4): a graph the set holds may
/// be new, or owned by this set's previous version in the hot folder, and nothing else.
fn check_ownership(set_name: &str, contents: &Contents, s: &Settings) -> Result<(), AppError> {
    let mut reasons = Vec::new();
    for (id, _) in &contents.graphs {
        let Some(owner) = graphs::graph_location(id) else {
            continue;
        };
        let set = graphs::graph_set(id);
        let own_version =
            owner == s.location && set.as_ref().is_some_and(|set| set.name == set_name);
        if !own_version {
            reasons.push(format!(
                "graph id {id} is owned by {}",
                graph_set_loader::describe(&owner, set.as_ref())
            ));
        }
    }
    if reasons.is_empty() {
        Ok(())
    } else {
        Err(refused(409, reasons.join("; ")))
    }
}

/// Stage the set's files in a folder of their own, gate them there, then move them into place
/// and swap the registry — all or none. The record is the caller's to write (a deploy) or to
/// leave alone (a reconcile).
fn install(
    st: &mut State,
    set_name: &str,
    contents: &Contents,
    agreed: &Agreed,
    s: &Settings,
) -> Result<(), AppError> {
    let folder = s.folder.clone().expect("enabled settings have a folder");
    let staging = folder.join(format!("{STAGING_PREFIX}{}", uuid::Uuid::new_v4().simple()));
    let result = install_staged(st, set_name, contents, agreed, s, &folder, &staging);
    let _ = std::fs::remove_dir_all(&staging);
    result
}

fn install_staged(
    st: &mut State,
    set_name: &str,
    contents: &Contents,
    agreed: &Agreed,
    s: &Settings,
    folder: &Path,
    staging: &Path,
) -> Result<(), AppError> {
    let io = |e: std::io::Error| refused(500, format!("{CANNOT_WRITE}{e}"));
    std::fs::create_dir_all(staging).map_err(io)?;
    std::fs::write(
        staging.join(format!("{set_name}{}", graph_set::EXTENSION)),
        &agreed.pack_bytes,
    )
    .map_err(io)?;
    for (id, model) in &contents.graphs {
        std::fs::write(
            staging.join(format!("{id}{JSON_EXT}")),
            graph_set::to_json(model),
        )
        .map_err(io)?;
    }
    let ids: Vec<String> = contents.graphs.iter().map(|(id, _)| id.clone()).collect();
    let (models, failures) = graph_set_loader::gate(&format!("{FILE}{}", staging.display()), &ids);
    if !failures.is_empty() {
        return Err(refused(400, failures.join("; ")));
    }
    for entry in std::fs::read_dir(staging).map_err(io)? {
        let entry = entry.map_err(io)?;
        std::fs::rename(entry.path(), folder.join(entry.file_name())).map_err(io)?;
    }
    let previous = graphs::graphs_of_set(set_name, &s.location);
    let version = agreed.record.version.clone();
    graphs::replace_graphs(
        &previous,
        models,
        &s.location,
        Some(DeployedSet {
            name: set_name.to_string(),
            version: version.clone(),
        }),
    );
    for id in &previous {
        if contents.graph(id).is_none() {
            delete_own_file(folder, &format!("{id}{JSON_EXT}")).map_err(io)?;
        }
    }
    st.by_set.insert(
        set_name.to_string(),
        Installed {
            sha256: agreed.sha256.clone(),
            version,
            graphs: ids,
            deployed_at: agreed.record.deployed_at.clone(),
            record_bytes: agreed.record_bytes.clone(),
        },
    );
    st.refusals.remove(set_name);
    st.last_check.insert(set_name.to_string(), now_millis());
    Ok(())
}

/// A name becomes a path only when it is a set's package or a graph's file, inside the folder.
fn delete_own_file(folder: &Path, name: &str) -> std::io::Result<()> {
    let stem = name.rsplit_once('.').map(|(stem, _)| stem).unwrap_or(name);
    let valid = model_gate::is_valid_graph_id(stem)
        && (name.ends_with(JSON_EXT) || name.ends_with(graph_set::EXTENSION));
    if valid {
        let file = folder.join(name);
        if file.parent() == Some(folder) && file.exists() {
            std::fs::remove_file(file)?;
        }
    }
    Ok(())
}

fn recorded_sets(folder: &Path) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    if let Ok(entries) = std::fs::read_dir(folder) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(set_name) = name.strip_suffix(RECORD_EXTENSION) {
                if entry.path().is_file() && model_gate::is_valid_graph_id(set_name) {
                    result.insert(set_name.to_string());
                }
            }
        }
    } else {
        log::error!(
            "Unable to read the hot deployment folder {}",
            folder.display()
        );
    }
    result
}

fn stray_packages(folder: &Path, recorded: &BTreeSet<String>) -> Vec<String> {
    let mut result = Vec::new();
    if let Ok(entries) = std::fs::read_dir(folder) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(set_name) = name.strip_suffix(graph_set::EXTENSION) {
                if !recorded.contains(set_name) {
                    result.push(name);
                }
            }
        }
    }
    result
}

fn remove_staging(folder: &Path) {
    if let Ok(entries) = std::fs::read_dir(folder) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.path().is_dir() && name.starts_with(STAGING_PREFIX) {
                let _ = std::fs::remove_dir_all(entry.path());
            }
        }
    }
}

pub(crate) fn record_path(folder: &Path, set_name: &str) -> PathBuf {
    folder.join(format!("{set_name}{RECORD_EXTENSION}"))
}

fn read_bytes(file: &Path) -> Option<Vec<u8>> {
    file.is_file().then(|| std::fs::read(file).ok()).flatten()
}

fn now_iso() -> String {
    jiff::Timestamp::now()
        .strftime("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string()
}

/// The record's text: one set, written by the engine and read by every instance.
fn record_text(record: &DeployRecord) -> String {
    let mut text = String::from(
        "# Written by the hot-deploy functions - rewritten at every change of this set; do not edit\n",
    );
    text.push_str(&format!("{SET_FIELD}: {}\n", quoted(&record.set)));
    text.push_str(&format!("{VERSION_FIELD}: {}\n", quoted(&record.version)));
    text.push_str(&format!("{SHA256_FIELD}: {}\n", quoted(&record.sha256)));
    text.push_str(&format!(
        "{DEPLOYED_AT_FIELD}: {}\n",
        quoted(&record.deployed_at)
    ));
    append_list(&mut text, GRAPHS_FIELD, &record.graphs);
    append_list(&mut text, FILES_FIELD, &record.files);
    text
}

fn append_list(text: &mut String, field: &str, values: &[String]) {
    if values.is_empty() {
        text.push_str(&format!("{field}: []\n"));
    } else {
        text.push_str(&format!("{field}:\n"));
        for value in values {
            text.push_str(&format!("  - {}\n", quoted(value)));
        }
    }
}

fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

/// A set's record as the file holds it, or `None` when there is none or it cannot be read.
fn parse_record(file: &Path) -> Option<DeployRecord> {
    if !file.is_file() {
        return None;
    }
    let reader = match ConfigReader::load(&format!("{FILE}{}", file.display())) {
        Ok(reader) => reader,
        Err(e) => {
            log::error!(
                "Unable to read the deployment record {} - {e}",
                file.display()
            );
            return None;
        }
    };
    Some(DeployRecord {
        set: reader.get_property_or(SET_FIELD, ""),
        version: reader.get_property_or(VERSION_FIELD, ""),
        sha256: reader.get_property_or(SHA256_FIELD, ""),
        deployed_at: reader.get_property_or(DEPLOYED_AT_FIELD, ""),
        graphs: strings(&reader, GRAPHS_FIELD),
        files: strings(&reader, FILES_FIELD),
    })
}

fn strings(reader: &ConfigReader, field: &str) -> Vec<String> {
    let mut result = Vec::new();
    if let Some(ConfigValue::List(list)) = reader.get(field) {
        for i in 0..list.len() {
            if let Some(value) = reader.get_property(&format!("{field}[{i}]")) {
                result.push(value);
            }
        }
    }
    result
}

fn write_atomically(target: &Path, bytes: &[u8], folder: &Path) -> std::io::Result<()> {
    let temp = folder.join(format!(".record-{}.tmp", uuid::Uuid::new_v4().simple()));
    std::fs::write(&temp, bytes)?;
    std::fs::rename(&temp, target)
}

fn count(n: usize) -> String {
    format!("{n} set{}", if n == 1 { "" } else { "s" })
}

fn graph_count(n: usize) -> String {
    format!("{n} graph{}", if n == 1 { "" } else { "s" })
}
