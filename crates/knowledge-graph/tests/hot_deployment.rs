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
//! Hot deploy and undeploy of graph sets (RFC-0017) - the twins of the Java `HotDeploymentTest` and
//! `HotDeployEndpointTest`: a set deployed in one step and served at its main graph, the same bytes changing
//! nothing and a new version replacing the set as a unit, the main-graph rule, ownership refusals with the
//! owner named, a refused upload leaving no file, undeploy removing the set and its files, the records
//! restoring the sets at startup with a stray package ignored, another instance's deployment, replacement
//! and undeployment seen at the next lookup, a tampered package unregistered, the switches governing deploy
//! and undeploy separately, the hot folder and a manifest's unpack folder refusing each other, the one-swap
//! registry; then the REST entries: the main graph served and the subgraphs hidden (graph and OpenAPI
//! endpoints), a subgraph reached from its own set and refused to another with the advisory saying so,
//! refusals answering their status, the list and the undeploy entries.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;
use std::time::Duration;

use async_trait::async_trait;
use event_script::conversions::{display, from_json};
use event_script::mlm::MultiLevelMap;
use knowledge_graph::graphs::DeployedSet;
use knowledge_graph::{compiler, graph_set, graphs, hot_deployment};
use platform_core::automation::AsyncHttpRequest;
use platform_core::canonical_packager::Builder;
use platform_core::{
    main_application, overrides, AppError, AutoStart, EntryPoint, EventEnvelope, Platform,
    PostOffice,
};
use rmpv::Value;
use sha2::{Digest, Sha256};

const JSON: &str = "application/json";
const OCTET_STREAM: &str = "application/octet-stream";
const DEPLOY: &str = "/api/graph-set/deploy";
const DEPLOYED: &str = "/api/graph-set/deployed";
const DIGEST_HEADER: &str = "x-graph-set-sha256";

/// A scratch folder removed when the step ends - never inside the Playground's temporary folder.
struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "hot-deployment-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch folder");
        Scratch(dir)
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    fn file(&self, name: &str) -> String {
        format!("file:{}", self.path(name).display())
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// One hot folder per step: configured as the startup sequence does (before the manifests, the folder read
/// after them), forgotten when the step ends.
struct HotFolder {
    scratch: Scratch,
    location: String,
    registered: Vec<String>,
}

impl HotFolder {
    fn new() -> Self {
        let scratch = Scratch::new();
        let location = scratch.file("deploy");
        hot_deployment::configure_with(true, true, &location, 0);
        hot_deployment::reconcile_all();
        assert!(hot_deployment::is_enabled());
        HotFolder {
            scratch,
            location,
            registered: Vec::new(),
        }
    }

    fn folder(&self) -> PathBuf {
        self.scratch.path("deploy")
    }

    fn deploy(&mut self, bytes: &[u8], ids: &[&str]) -> hot_deployment::Deployment {
        self.registered.extend(ids.iter().map(|id| id.to_string()));
        hot_deployment::deploy(bytes, None)
            .unwrap_or_else(|e| panic!("deploy failed: {} {}", e.status(), e.message()))
    }

    /// Another instance over the same folder: this instance forgets everything it registered from the folder,
    /// the files stay, and the registry no longer holds the set's graphs.
    fn become_another_instance(&self, ids: &[&str]) {
        hot_deployment::configure_with(true, true, &self.location, 0);
        for id in ids {
            graphs::remove_graph(id);
        }
    }

    fn files(&self) -> Vec<String> {
        files_in(&self.folder())
    }

    fn record_text(&self, set_name: &str) -> String {
        std::fs::read_to_string(
            self.folder()
                .join(format!("{set_name}{}", hot_deployment::RECORD_EXTENSION)),
        )
        .expect("the record")
    }
}

impl Drop for HotFolder {
    fn drop(&mut self) {
        // the registry is shared by every step in this process
        for id in &self.registered {
            graphs::remove_graph(id);
        }
        hot_deployment::reset();
    }
}

fn files_in(folder: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(folder)
        .map(|entries| {
            entries
                .flatten()
                .map(|e| e.file_name().to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

fn graph(id: &str, answer: &str) -> Value {
    from_json(&serde_json::json!({
        "nodes": [
            {"alias": "root", "types": ["Root"], "properties": {"purpose": "a hot-deployed graph", "name": id}},
            {"alias": "end", "types": ["End"],
             "properties": {"skill": "graph.data.mapper", "mapping": [format!("text({answer}) -> output.body")]}}],
        "connections": [{"source": "root", "target": "end", "relations": [{"type": "done", "properties": {}}]}]
    }))
}

fn graph_without_end(id: &str) -> Value {
    from_json(&serde_json::json!({
        "nodes": [{"alias": "root", "types": ["Root"], "properties": {"purpose": "no end node", "name": id}}],
        "connections": []
    }))
}

/// A main graph whose one node extends another graph and returns what it answered.
fn extending(id: &str, extension: &str) -> Value {
    from_json(&serde_json::json!({
        "nodes": [
            {"alias": "root", "types": ["Root"], "properties": {"purpose": "a graph that extends another", "name": id}},
            {"alias": "delegate", "types": ["Extension"],
             "properties": {"skill": "graph.extension", "extension": extension,
                            "input": ["input.body -> *"], "output": ["result -> output.body"]}},
            {"alias": "end", "types": ["End"], "properties": {}}],
        "connections": [
            {"source": "root", "target": "delegate", "relations": [{"type": "next", "properties": {}}]},
            {"source": "delegate", "target": "end", "relations": [{"type": "done", "properties": {}}]}]
    }))
}

fn models(graphs: &[(&str, Value)]) -> BTreeMap<String, Value> {
    graphs
        .iter()
        .map(|(id, model)| (id.to_string(), model.clone()))
        .collect()
}

fn pack(set_name: &str, version: Option<&str>, graphs: &[(&str, Value)]) -> Vec<u8> {
    let fields: Vec<(String, String)> = version
        .map(|v| vec![("version".to_string(), v.to_string())])
        .unwrap_or_default();
    graph_set::pack(set_name, &fields, &models(graphs)).expect("a set")
}

/// The pairs `id, answer, id, answer, ...` as graphs.
fn answers(ids_and_answers: &[&str]) -> Vec<(&'static str, Value)> {
    ids_and_answers
        .chunks(2)
        .map(|pair| {
            let id: &'static str = Box::leak(pair[0].to_string().into_boxed_str());
            (id, graph(pair[0], pair[1]))
        })
        .collect()
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn base_url() -> String {
    let addr = platform_core::automation::server_address().expect("server bound");
    format!("http://127.0.0.1:{}", addr.port())
}

fn message(reply: &EventEnvelope) -> String {
    match reply.body() {
        Value::Map(entries) => entries
            .iter()
            .find(|(k, _)| k.as_str() == Some("message"))
            .map(|(_, v)| display(v))
            .unwrap_or_else(|| display(reply.body())),
        other => display(other),
    }
}

async fn http(
    po: &PostOffice,
    method: &str,
    path: &str,
    body: Option<(&str, Value)>,
    headers: &[(&str, &str)],
) -> EventEnvelope {
    let mut request = AsyncHttpRequest::new()
        .set_method(method)
        .set_target_host(&base_url())
        .set_url(path)
        .set_header("accept", JSON);
    if let Some((content_type, body)) = body {
        request = request
            .set_header("content-type", content_type)
            .set_body(body);
    }
    for (name, value) in headers {
        request = request.set_header(name, value);
    }
    po.request(
        EventEnvelope::new()
            .set_to("async.http.request")
            .set_raw_body(request.to_value()),
        Duration::from_secs(10),
    )
    .await
    .expect("http request")
}

async fn run(po: &PostOffice, graph_id: &str) -> EventEnvelope {
    http(
        po,
        "POST",
        &format!("/api/graph/{graph_id}"),
        Some((JSON, Value::Map(vec![]))),
        &[],
    )
    .await
}

async fn deploy_http(po: &PostOffice, bytes: Vec<u8>, headers: &[(&str, &str)]) -> EventEnvelope {
    http(
        po,
        "POST",
        DEPLOY,
        Some((OCTET_STREAM, Value::Binary(bytes))),
        headers,
    )
    .await
}

fn body_map(reply: &EventEnvelope) -> MultiLevelMap {
    MultiLevelMap::from_value(reply.body().clone())
}

fn text_list(mm: &MultiLevelMap, key: &str) -> Vec<String> {
    match mm.get_element(key) {
        Some(Value::Array(items)) => items.iter().map(display).collect(),
        other => panic!("{key} is a list: {other:?}"),
    }
}

#[main_application]
struct HotDeploymentTestApp;

#[async_trait]
impl EntryPoint for HotDeploymentTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "Flows ready: {:?}, graphs compiled: {}",
            event_script::flows::get_all_flows().len(),
            graphs::get_all_graphs().len()
        );
        Ok(())
    }
}

async fn boot() -> Platform {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let dir = test_support::temp_path("hot-deployment-temp");
        std::fs::create_dir_all(&dir).expect("temp dir");
        overrides::set("location.graph.temp", &format!("file:{}", dir.display()));
        overrides::set("rest.server.port", "0");
    });
    platform_core::resources::prepend_resource_root("tests/resources");
    AutoStart::main(vec![]).await.expect("lifecycle");
    Platform::get_instance()
}

/// One runtime for every step: a second `#[tokio::test]` gets its own runtime, which would drop the platform the
/// first one started.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn hot_deployment_end_to_end() {
    test_support::run_at_exit(platform_core::util::elastic_queue::shutdown_cleanup);
    let platform = boot().await;
    let po = PostOffice::new(&platform);
    a_set_deploys_in_one_step_and_the_endpoint_serves_its_main_graph(&po).await;
    the_same_bytes_change_nothing_and_a_new_version_replaces_the_set_as_a_unit(&po).await;
    a_set_is_named_after_its_main_graph();
    a_bundled_id_and_another_sets_id_are_refused_with_the_owner_named(&po).await;
    a_refused_upload_leaves_no_file_and_the_digest_is_checked();
    undeploy_removes_the_set_its_files_and_its_record(&po).await;
    the_records_restore_the_sets_at_startup_and_a_stray_package_is_ignored(&po).await;
    another_instances_deployment_replacement_and_undeployment_are_seen_at_the_next_lookup(&po)
        .await;
    a_package_that_no_longer_matches_its_record_is_unregistered(&po).await;
    the_switches_govern_deploy_and_undeploy_separately();
    the_hot_folder_and_a_manifests_unpack_folder_refuse_each_other();
    replace_graphs_swaps_removals_and_additions_as_one_unit();
    // the REST entries, on the application's own folder
    hot_deployment::reset();
    hot_deployment::reconcile_all();
    assert!(
        hot_deployment::can_deploy() && hot_deployment::can_undeploy(),
        "the test application turns both on"
    );
    a_deployed_set_serves_its_main_graph_and_hides_its_subgraphs(&po).await;
    a_subgraph_is_reached_from_its_own_set_and_refused_to_another_set(&po).await;
    refusals_answer_their_status(&po).await;
    the_list_and_the_undeploy_entries(&po).await;
}

async fn a_set_deploys_in_one_step_and_the_endpoint_serves_its_main_graph(po: &PostOffice) {
    let mut hot = HotFolder::new();
    let bytes = pack(
        "unit-hot-main",
        Some("1.0.0"),
        &answers(&[
            "unit-hot-main",
            "the main graph",
            "unit-hot-sub",
            "a subgraph",
        ]),
    );
    let deployment = hot.deploy(&bytes, &["unit-hot-main", "unit-hot-sub"]);
    assert!(deployment.changed);
    assert_eq!("unit-hot-main", deployment.set);
    assert_eq!("1.0.0", deployment.version);
    assert_eq!(vec!["unit-hot-main", "unit-hot-sub"], deployment.graphs);
    assert_eq!(None, deployment.replaced);
    assert_eq!(sha256(&bytes), deployment.sha256);
    for id in ["unit-hot-main", "unit-hot-sub"] {
        assert!(graphs::graph_exists(id), "{id}");
        assert_eq!(Some(hot.location.clone()), graphs::graph_location(id));
        assert_eq!(
            Some(DeployedSet {
                name: "unit-hot-main".to_string(),
                version: "1.0.0".to_string()
            }),
            graphs::graph_set(id)
        );
    }
    // the main graph is public, every other graph of the set is private to it (D15)
    assert!(!hot_deployment::is_private_subgraph("unit-hot-main"));
    assert!(hot_deployment::is_private_subgraph("unit-hot-sub"));
    assert!(
        !hot_deployment::is_private_subgraph("tutorial-1"),
        "a bundled graph is never private"
    );
    // the folder holds the package, one file per graph and the record - and nothing else
    assert_eq!(
        vec![
            "unit-hot-main.deployed.yaml",
            "unit-hot-main.json",
            "unit-hot-main.pack",
            "unit-hot-sub.json"
        ],
        hot.files()
    );
    let record = hot.record_text("unit-hot-main");
    assert!(record.contains("set: 'unit-hot-main'\n"), "{record}");
    assert!(
        record.contains(&format!("sha256: '{}'\n", deployment.sha256)),
        "{record}"
    );
    assert!(
        record.contains("graphs:\n  - 'unit-hot-main'\n  - 'unit-hot-sub'\n"),
        "{record}"
    );
    assert!(
        record.contains("files:\n  - 'unit-hot-main.pack'\n  - 'unit-hot-main.json'\n"),
        "{record}"
    );
    assert!(graphs::deployed_locations().contains(&hot.location));
    let response = run(po, "unit-hot-main").await;
    assert_eq!(200, response.status(), "{}", message(&response));
    assert_eq!(Value::from("the main graph"), *response.body());
    let listed = hot_deployment::list();
    assert_eq!(1, listed.len());
    assert_eq!("deployed", listed[0].status);
    assert_eq!(deployment.sha256, listed[0].sha256);
}

async fn the_same_bytes_change_nothing_and_a_new_version_replaces_the_set_as_a_unit(
    po: &PostOffice,
) {
    let mut hot = HotFolder::new();
    let v1 = pack(
        "unit-hot-versioned",
        Some("1"),
        &answers(&["unit-hot-versioned", "v1", "unit-hot-dropped", "dropped"]),
    );
    hot.deploy(
        &v1,
        &["unit-hot-versioned", "unit-hot-dropped", "unit-hot-added"],
    );
    let again = hot_deployment::deploy(&v1, Some(&sha256(&v1))).expect("the same bytes");
    assert!(!again.changed, "the same bytes are already deployed");
    assert_eq!("1", again.version);
    let v2 = pack(
        "unit-hot-versioned",
        Some("2"),
        &answers(&["unit-hot-versioned", "v2", "unit-hot-added", "added"]),
    );
    let replaced = hot_deployment::deploy(&v2, None).expect("a new version");
    assert!(replaced.changed);
    assert_eq!("2", replaced.version);
    assert_eq!(
        Some(("1".to_string(), sha256(&v1))),
        replaced.replaced,
        "the previous version and digest"
    );
    // the dropped graph left the registry and the folder; the added one is served
    assert!(!graphs::graph_exists("unit-hot-dropped"));
    assert!(!hot.folder().join("unit-hot-dropped.json").exists());
    assert!(graphs::graph_exists("unit-hot-added"));
    assert_eq!(
        Some("2".to_string()),
        graphs::graph_set("unit-hot-versioned").map(|s| s.version)
    );
    assert_eq!(
        Value::from("v2"),
        *run(po, "unit-hot-versioned").await.body()
    );
    let record = hot.record_text("unit-hot-versioned");
    assert!(record.contains("version: '2'\n"), "{record}");
    assert!(!record.contains("unit-hot-dropped"), "{record}");
}

fn a_set_is_named_after_its_main_graph() {
    let _hot = HotFolder::new();
    let no_main = pack("unit-hot-no-main", None, &answers(&["unit-hot-other", "x"]));
    let refused = hot_deployment::deploy(&no_main, None).expect_err("no main graph");
    assert_eq!(400, refused.status());
    assert!(
        refused.message().contains("named after its main graph"),
        "{}",
        refused.message()
    );
    assert!(!graphs::graph_exists("unit-hot-other"));
    // graph_id, when the manifest has it, must agree with the set name
    let disagreeing = graph_set::pack(
        "unit-hot-entry",
        &[("graph_id".to_string(), "unit-hot-entry-sub".to_string())],
        &models(&answers(&[
            "unit-hot-entry",
            "main",
            "unit-hot-entry-sub",
            "sub",
        ])),
    )
    .expect("a set");
    let refused_too = hot_deployment::deploy(&disagreeing, None).expect_err("graph_id disagrees");
    assert_eq!(400, refused_too.status());
    assert!(
        refused_too.message().contains("graph_id"),
        "{}",
        refused_too.message()
    );
    assert!(!graphs::graph_exists("unit-hot-entry"));
}

async fn a_bundled_id_and_another_sets_id_are_refused_with_the_owner_named(po: &PostOffice) {
    let mut hot = HotFolder::new();
    // tutorial-1 is a bundled graph of the test manifest
    let shadowing = pack(
        "unit-hot-shadow",
        None,
        &answers(&["unit-hot-shadow", "main", "tutorial-1", "a copy"]),
    );
    let refused = hot_deployment::deploy(&shadowing, None).expect_err("a bundled id");
    assert_eq!(409, refused.status());
    assert!(
        refused
            .message()
            .contains("tutorial-1 is owned by classpath:/graph"),
        "{}",
        refused.message()
    );
    assert!(!graphs::graph_exists("unit-hot-shadow"));
    assert_eq!(
        Some("classpath:/graph".to_string()),
        graphs::graph_location("tutorial-1"),
        "the bundled copy stands"
    );
    assert!(hot.files().is_empty(), "nothing was written");
    // one hot set owns an id: a second set carrying it is refused, and the first keeps serving
    hot.deploy(
        &pack(
            "unit-hot-first",
            Some("1"),
            &answers(&["unit-hot-first", "first", "unit-hot-shared", "shared"]),
        ),
        &["unit-hot-first", "unit-hot-shared"],
    );
    let second = pack(
        "unit-hot-second",
        Some("1"),
        &answers(&["unit-hot-second", "second", "unit-hot-shared", "copy"]),
    );
    let refused_too = hot_deployment::deploy(&second, None).expect_err("another set's id");
    assert_eq!(409, refused_too.status());
    assert!(
        refused_too
            .message()
            .contains("unit-hot-shared is owned by set unit-hot-first"),
        "{}",
        refused_too.message()
    );
    assert!(!graphs::graph_exists("unit-hot-second"));
    assert_eq!(
        Some("unit-hot-first".to_string()),
        graphs::graph_set("unit-hot-shared").map(|s| s.name)
    );
    assert_eq!(
        Value::from("first"),
        *run(po, "unit-hot-first").await.body()
    );
}

fn a_refused_upload_leaves_no_file_and_the_digest_is_checked() {
    let hot = HotFolder::new();
    // the packager would refuse this set (the gate runs when a set is packed), so it is built with the
    // canonical packager directly - what a deployment can meet
    let bytes = Builder::new()
        .manifest(graph_set::SET, "unit-hot-half")
        .expect("manifest")
        .add("unit-hot-half.json", graph("unit-hot-half", "good"))
        .expect("entry")
        .add(
            "unit-hot-half-no-end.json",
            graph_without_end("unit-hot-half-no-end"),
        )
        .expect("entry")
        .build()
        .expect("bytes");
    let refused = hot_deployment::deploy(&bytes, None).expect_err("a graph the gate refuses");
    assert_eq!(400, refused.status());
    assert!(
        refused.message().contains("unit-hot-half-no-end"),
        "{}",
        refused.message()
    );
    assert!(
        !graphs::graph_exists("unit-hot-half"),
        "all or none: the valid graph is not registered"
    );
    assert!(
        hot.files().is_empty(),
        "a refused upload leaves nothing, not even a staging folder: {:?}",
        hot.files()
    );
    // a digest the caller expects must be the body's
    let good = pack("unit-hot-digest", None, &answers(&["unit-hot-digest", "x"]));
    let mismatch =
        hot_deployment::deploy(&good, Some(&"00".repeat(32))).expect_err("a wrong digest");
    assert_eq!(400, mismatch.status());
    assert!(
        mismatch.message().contains("SHA-256 does not match"),
        "{}",
        mismatch.message()
    );
    assert!(!graphs::graph_exists("unit-hot-digest"));
    let not_a_set = hot_deployment::deploy(b"not a package", None).expect_err("not a package");
    assert_eq!(400, not_a_set.status());
    let empty = hot_deployment::deploy(&[], None).expect_err("an empty body");
    assert_eq!(400, empty.status());
}

async fn undeploy_removes_the_set_its_files_and_its_record(po: &PostOffice) {
    let mut hot = HotFolder::new();
    hot.deploy(
        &pack(
            "unit-hot-gone",
            Some("1"),
            &answers(&["unit-hot-gone", "main", "unit-hot-gone-sub", "sub"]),
        ),
        &["unit-hot-gone", "unit-hot-gone-sub"],
    );
    assert_eq!(200, run(po, "unit-hot-gone").await.status());
    let removed = hot_deployment::undeploy("unit-hot-gone").expect("undeployed");
    let mut ids = removed.graphs.clone();
    ids.sort();
    assert_eq!(vec!["unit-hot-gone", "unit-hot-gone-sub"], ids);
    assert!(!graphs::graph_exists("unit-hot-gone"));
    assert!(!graphs::graph_exists("unit-hot-gone-sub"));
    assert!(
        hot.files().is_empty(),
        "the record, the package and the graph files are gone: {:?}",
        hot.files()
    );
    assert_eq!(404, run(po, "unit-hot-gone").await.status());
    let again = hot_deployment::undeploy("unit-hot-gone").expect_err("already gone");
    assert_eq!(404, again.status());
    // a bundled graph is never in the folder, so it answers 404 too
    let bundled = hot_deployment::undeploy("tutorial-1").expect_err("a bundled graph");
    assert_eq!(404, bundled.status());
    assert!(graphs::graph_exists("tutorial-1"));
    let invalid = hot_deployment::undeploy("../escape").expect_err("an invalid name");
    assert_eq!(400, invalid.status());
}

async fn the_records_restore_the_sets_at_startup_and_a_stray_package_is_ignored(po: &PostOffice) {
    let mut hot = HotFolder::new();
    hot.deploy(
        &pack(
            "unit-hot-restart",
            Some("3"),
            &answers(&["unit-hot-restart", "back"]),
        ),
        &["unit-hot-restart"],
    );
    // a package someone copied in by hand has no record
    std::fs::write(
        hot.folder().join("unit-hot-stray.pack"),
        pack("unit-hot-stray", None, &answers(&["unit-hot-stray", "x"])),
    )
    .expect("written");
    // a restart: the folder's files stay, the registry starts empty
    hot.become_another_instance(&["unit-hot-restart"]);
    assert_eq!(None, graphs::graph_location("unit-hot-restart"));
    hot_deployment::reconcile_all();
    assert!(graphs::graph_exists("unit-hot-restart"));
    assert_eq!(
        Some("3".to_string()),
        graphs::graph_set("unit-hot-restart").map(|s| s.version)
    );
    assert_eq!(
        Value::from("back"),
        *run(po, "unit-hot-restart").await.body()
    );
    assert!(
        !graphs::graph_exists("unit-hot-stray"),
        "the records are the allowlist"
    );
    let listed = hot_deployment::list();
    assert_eq!(1, listed.len());
    assert_eq!("unit-hot-restart", listed[0].set);
    assert_eq!("deployed", listed[0].status);
}

async fn another_instances_deployment_replacement_and_undeployment_are_seen_at_the_next_lookup(
    po: &PostOffice,
) {
    let mut hot = HotFolder::new();
    let v1 = pack(
        "unit-hot-peer",
        Some("1"),
        &answers(&["unit-hot-peer", "v1", "unit-hot-peer-sub", "sub"]),
    );
    hot.deploy(
        &v1,
        &["unit-hot-peer", "unit-hot-peer-sub", "unit-hot-peer-new"],
    );
    // another instance over the same folder has never seen the set: its next lookup for the main graph loads it
    hot.become_another_instance(&["unit-hot-peer", "unit-hot-peer-sub"]);
    assert_eq!(None, graphs::graph_location("unit-hot-peer"));
    assert!(
        graphs::get_graph("unit-hot-peer").is_some(),
        "an unknown id with a record of its name is loaded"
    );
    assert!(
        graphs::graph_exists("unit-hot-peer-sub"),
        "the whole set came with it"
    );
    assert_eq!(Value::from("v1"), *run(po, "unit-hot-peer").await.body());
    // the first instance replaces the set - here the same process, so the registry already holds the result; what
    // the step checks is that the record on disk changed, which is all a peer ever reads
    let record_file = hot.folder().join("unit-hot-peer.deployed.yaml");
    let before = std::fs::read(&record_file).expect("the record");
    let v2 = pack(
        "unit-hot-peer",
        Some("2"),
        &answers(&["unit-hot-peer", "v2", "unit-hot-peer-new", "new"]),
    );
    hot_deployment::deploy(&v2, None).expect("a new version");
    let after = std::fs::read(&record_file).expect("the record");
    assert_ne!(before, after, "the record was rewritten");
    assert_eq!(Value::from("v2"), *run(po, "unit-hot-peer").await.body());
    assert!(graphs::graph_exists("unit-hot-peer-new"));
    assert!(!graphs::graph_exists("unit-hot-peer-sub"));
    // the first instance undeploys: the record is gone, so the peer drops the set at its next lookup
    std::fs::remove_file(&record_file).expect("deleted");
    assert!(!graphs::graph_exists("unit-hot-peer"));
    assert!(!graphs::graph_exists("unit-hot-peer-new"));
    assert_eq!(404, run(po, "unit-hot-peer").await.status());
}

async fn a_package_that_no_longer_matches_its_record_is_unregistered(po: &PostOffice) {
    let mut hot = HotFolder::new();
    hot.deploy(
        &pack(
            "unit-hot-tampered",
            Some("1"),
            &answers(&["unit-hot-tampered", "trusted"]),
        ),
        &["unit-hot-tampered"],
    );
    // the bytes under the record change without the record: not the bytes the record vouches for
    std::fs::write(
        hot.folder().join("unit-hot-tampered.pack"),
        pack(
            "unit-hot-tampered",
            Some("1"),
            &answers(&["unit-hot-tampered", "tampered"]),
        ),
    )
    .expect("written");
    hot.become_another_instance(&["unit-hot-tampered"]);
    assert!(
        graphs::get_graph("unit-hot-tampered").is_none(),
        "the record and the package disagree"
    );
    let listed = hot_deployment::list();
    assert_eq!(1, listed.len());
    assert_eq!(
        "not deployed - the package does not match its record",
        listed[0].status
    );
    assert_eq!(404, run(po, "unit-hot-tampered").await.status());
}

fn the_switches_govern_deploy_and_undeploy_separately() {
    let mut hot = HotFolder::new();
    let bytes = pack(
        "unit-hot-switch",
        Some("1"),
        &answers(&["unit-hot-switch", "x"]),
    );
    hot.deploy(&bytes, &["unit-hot-switch"]);
    // deploy off, undeploy on: the set keeps serving and can still be removed, but nothing new enters
    hot_deployment::configure_with(false, true, &hot.location, 0);
    hot_deployment::reconcile_all();
    assert!(graphs::graph_exists("unit-hot-switch"));
    assert!(!hot_deployment::can_deploy());
    assert!(hot_deployment::can_undeploy());
    let off = hot_deployment::deploy(&bytes, None).expect_err("deploy is off");
    assert_eq!(503, off.status());
    hot_deployment::undeploy("unit-hot-switch").expect("undeploy is on");
    assert!(!graphs::graph_exists("unit-hot-switch"));
    // both off: the folder is never read and both calls answer 503
    std::fs::write(hot.folder().join("unit-hot-switch.pack"), &bytes).expect("written");
    hot_deployment::configure_with(false, false, &hot.location, 0);
    assert!(!hot_deployment::is_enabled());
    hot_deployment::reconcile_all();
    assert!(!graphs::graph_exists("unit-hot-switch"));
    assert_eq!(
        503,
        hot_deployment::deploy(&bytes, None)
            .expect_err("off")
            .status()
    );
    assert_eq!(
        503,
        hot_deployment::undeploy("unit-hot-switch")
            .expect_err("off")
            .status()
    );
    assert!(hot_deployment::list().is_empty());
}

fn the_hot_folder_and_a_manifests_unpack_folder_refuse_each_other() {
    let mut hot = HotFolder::new();
    // a manifest whose 'unpack' lies inside the hot folder: its sets are skipped
    let packs = hot.scratch.path("packs");
    std::fs::create_dir_all(&packs).expect("folder");
    std::fs::write(
        packs.join("unit-hot-unpack-set.pack"),
        pack(
            "unit-hot-unpack-set",
            None,
            &answers(&["unit-hot-unpack-graph", "x"]),
        ),
    )
    .expect("written");
    let manifest_file = hot.scratch.path("graphs.yaml");
    let manifest = format!("file:{}", manifest_file.display());
    let write_manifest = |unpack: &Path| {
        std::fs::write(
            &manifest_file,
            format!(
                "location: 'file:{}'\nsets:\n  - 'unit-hot-unpack-set'\nunpack: 'file:{}'\n",
                packs.display(),
                unpack.display()
            ),
        )
        .expect("a manifest");
    };
    hot.registered.push("unit-hot-unpack-graph".to_string());
    write_manifest(&hot.folder().join("inner"));
    compiler::compile_manifest(&manifest);
    assert!(
        !graphs::graph_exists("unit-hot-unpack-graph"),
        "an unpack folder inside the hot folder"
    );
    // the hot folder inside a manifest's unpack folder: hot deployment turns itself off for the run
    let unpack = hot.scratch.path("unpack");
    write_manifest(&unpack);
    hot_deployment::configure_with(false, false, &hot.location, 0);
    compiler::compile_manifest(&manifest);
    assert!(graphs::graph_exists("unit-hot-unpack-graph"));
    hot_deployment::configure_with(
        true,
        true,
        &format!("file:{}", unpack.join("hot").display()),
        0,
    );
    assert!(hot_deployment::is_enabled());
    hot_deployment::reconcile_all();
    assert!(
        !hot_deployment::is_enabled(),
        "a hot folder inside an unpack folder is refused"
    );
    // the Playground's temporary folder is refused at configuration
    let temp = platform_core::AppConfigReader::get_instance()
        .get_property_or("location.graph.temp", "file:/tmp/graph");
    hot_deployment::configure_with(true, true, &format!("{temp}/hot"), 0);
    assert!(!hot_deployment::is_enabled());
    assert!(!Path::new(&temp.replace("file:", "")).join("hot").exists());
    hot_deployment::configure_with(true, true, "classpath:/hot", 0);
    assert!(!hot_deployment::is_enabled());
}

fn replace_graphs_swaps_removals_and_additions_as_one_unit() {
    let mut hot = HotFolder::new();
    graphs::add_graph(
        "unit-hot-swap-old",
        graph("unit-hot-swap-old", "old"),
        "file:/swap",
    );
    hot.registered.extend([
        "unit-hot-swap-old".to_string(),
        "unit-hot-swap-new".to_string(),
    ]);
    graphs::replace_graphs(
        &["unit-hot-swap-old".to_string()],
        vec![(
            "unit-hot-swap-new".to_string(),
            graph("unit-hot-swap-new", "new"),
        )],
        "file:/swap",
        Some(DeployedSet {
            name: "unit-hot-swap-new".to_string(),
            version: "1".to_string(),
        }),
    );
    assert_eq!(None, graphs::graph_location("unit-hot-swap-old"));
    assert_eq!(
        Some("file:/swap".to_string()),
        graphs::graph_location("unit-hot-swap-new")
    );
    assert_eq!(
        vec!["unit-hot-swap-new"],
        graphs::graphs_of_set("unit-hot-swap-new", "file:/swap")
    );
    assert!(graphs::graphs_of_set("unit-hot-swap-new", "file:/elsewhere").is_empty());
}

// --- the REST entries (Java `HotDeployEndpointTest`)

/// Undeploy the sets a REST step deployed, so the application's folder is clean for the next step.
async fn undeploy_all(sets: &[&str]) {
    for set in sets {
        if hot_deployment::undeploy(set).is_err() {
            for id in graphs::graphs_of_set(set, &hot_deployment::location()) {
                graphs::remove_graph(&id);
            }
        }
    }
}

async fn a_deployed_set_serves_its_main_graph_and_hides_its_subgraphs(po: &PostOffice) {
    let bytes = pack(
        "unit-rest-main",
        Some("1.0"),
        &answers(&["unit-rest-main", "main", "unit-rest-sub", "sub"]),
    );
    let digest = sha256(&bytes);
    let response = deploy_http(po, bytes.clone(), &[(DIGEST_HEADER, &digest)]).await;
    assert_eq!(200, response.status(), "{}", message(&response));
    let answer = body_map(&response);
    assert_eq!(
        Some(Value::from("unit-rest-main")),
        answer.get_element("set")
    );
    assert_eq!(Some(Value::from("1.0")), answer.get_element("version"));
    assert_eq!(
        Some(Value::from(digest.as_str())),
        answer.get_element("sha256")
    );
    assert_eq!(Some(Value::from(true)), answer.get_element("changed"));
    assert_eq!(
        vec!["unit-rest-main", "unit-rest-sub"],
        text_list(&answer, "graphs")
    );
    assert!(text_list(&answer, "advisory.routes").is_empty());
    assert!(text_list(&answer, "advisory.extensions").is_empty());
    // the main graph is served, the subgraph is not - at the graph endpoint and at the OpenAPI endpoint
    let main = run(po, "unit-rest-main").await;
    assert_eq!(200, main.status(), "{}", message(&main));
    assert_eq!(Value::from("main"), *main.body());
    assert_eq!(404, run(po, "unit-rest-sub").await.status());
    assert_eq!(
        200,
        http(po, "GET", "/api/openapi/unit-rest-main", None, &[])
            .await
            .status()
    );
    assert_eq!(
        404,
        http(po, "GET", "/api/openapi/unit-rest-sub", None, &[])
            .await
            .status()
    );
    // the same bytes again: nothing changes
    let again = deploy_http(po, bytes, &[]).await;
    assert_eq!(200, again.status(), "{}", message(&again));
    assert_eq!(
        Some(Value::from(false)),
        body_map(&again).get_element("changed")
    );
    undeploy_all(&["unit-rest-main"]).await;
}

async fn a_subgraph_is_reached_from_its_own_set_and_refused_to_another_set(po: &PostOffice) {
    let owner = pack(
        "unit-rest-owner",
        Some("1"),
        &[
            (
                "unit-rest-owner",
                extending("unit-rest-owner", "unit-rest-owner-sub"),
            ),
            (
                "unit-rest-owner-sub",
                graph("unit-rest-owner-sub", "from the subgraph"),
            ),
        ],
    );
    let deployed = deploy_http(po, owner, &[]).await;
    assert_eq!(200, deployed.status(), "{}", message(&deployed));
    let served = run(po, "unit-rest-owner").await;
    assert_eq!(200, served.status(), "{}", message(&served));
    assert_eq!(Value::from("from the subgraph"), *served.body());
    // another set extends the private subgraph: the advisory says so at deploy time, the run meets 404
    let intruder = pack(
        "unit-rest-intruder",
        Some("1"),
        &[(
            "unit-rest-intruder",
            extending("unit-rest-intruder", "unit-rest-owner-sub"),
        )],
    );
    let response = deploy_http(po, intruder, &[]).await;
    assert_eq!(200, response.status(), "{}", message(&response));
    let answer = body_map(&response);
    assert_eq!(
        Some(Value::from("unit-rest-owner-sub")),
        answer.get_element("advisory.extensions[0].target")
    );
    assert_eq!(
        Some(Value::from("private")),
        answer.get_element("advisory.extensions[0].status")
    );
    let refused = run(po, "unit-rest-intruder").await;
    assert_ne!(200, refused.status());
    assert!(
        display(refused.body()).contains("not found"),
        "{}",
        display(refused.body())
    );
    undeploy_all(&["unit-rest-owner", "unit-rest-intruder"]).await;
}

async fn refusals_answer_their_status(po: &PostOffice) {
    let bytes = pack(
        "unit-rest-refused",
        Some("1"),
        &answers(&["unit-rest-refused", "x"]),
    );
    let mismatch = deploy_http(po, bytes, &[(DIGEST_HEADER, &"00".repeat(32))]).await;
    assert_eq!(400, mismatch.status());
    assert!(
        message(&mismatch).contains("SHA-256 does not match"),
        "{}",
        message(&mismatch)
    );
    assert!(!graphs::graph_exists("unit-rest-refused"));
    let shadowing = pack(
        "unit-rest-shadow",
        Some("1"),
        &answers(&["unit-rest-shadow", "x", "tutorial-1", "copy"]),
    );
    let conflict = deploy_http(po, shadowing, &[]).await;
    assert_eq!(409, conflict.status());
    assert!(
        message(&conflict).contains("tutorial-1 is owned by classpath:/graph"),
        "{}",
        message(&conflict)
    );
    let not_a_set = deploy_http(po, b"hello".to_vec(), &[]).await;
    assert_eq!(400, not_a_set.status());
    let unnamed = deploy_http(
        po,
        pack(
            "unit-rest-unnamed",
            Some("1"),
            &answers(&["unit-rest-other", "x"]),
        ),
        &[],
    )
    .await;
    assert_eq!(400, unnamed.status());
    assert!(
        message(&unnamed).contains("named after its main graph"),
        "{}",
        message(&unnamed)
    );
    // the body is the package itself, not a JSON document
    let json = http(
        po,
        "POST",
        DEPLOY,
        Some((
            JSON,
            Value::Map(vec![(Value::from("set"), Value::from("x"))]),
        )),
        &[],
    )
    .await;
    assert_eq!(400, json.status(), "{}", message(&json));
}

async fn the_list_and_the_undeploy_entries(po: &PostOffice) {
    let deployed = deploy_http(
        po,
        pack(
            "unit-rest-listed",
            Some("7"),
            &answers(&["unit-rest-listed", "x"]),
        ),
        &[],
    )
    .await;
    assert_eq!(200, deployed.status(), "{}", message(&deployed));
    let listed = http(po, "GET", DEPLOYED, None, &[]).await;
    assert_eq!(200, listed.status(), "{}", message(&listed));
    let answer = body_map(&listed);
    assert_eq!(
        Some(Value::from(hot_deployment::location().as_str())),
        answer.get_element("location")
    );
    let Some(Value::Array(sets)) = answer.get_element("sets") else {
        panic!("sets is a list: {:?}", listed.body())
    };
    let row = sets
        .iter()
        .map(|s| MultiLevelMap::from_value(s.clone()))
        .find(|s| s.get_element("set") == Some(Value::from("unit-rest-listed")))
        .expect("the deployed set is listed");
    assert_eq!(Some(Value::from("7")), row.get_element("version"));
    assert_eq!(Some(Value::from("deployed")), row.get_element("status"));
    let removed = http(
        po,
        "POST",
        "/api/graph-set/undeploy/unit-rest-listed",
        None,
        &[],
    )
    .await;
    assert_eq!(200, removed.status(), "{}", message(&removed));
    let answer = body_map(&removed);
    assert_eq!(Some(Value::from(true)), answer.get_element("undeployed"));
    assert_eq!(vec!["unit-rest-listed"], text_list(&answer, "graphs"));
    assert_eq!(404, run(po, "unit-rest-listed").await.status());
    assert_eq!(
        404,
        http(
            po,
            "POST",
            "/api/graph-set/undeploy/unit-rest-listed",
            None,
            &[]
        )
        .await
        .status()
    );
    assert_eq!(
        404,
        http(po, "POST", "/api/graph-set/undeploy/tutorial-1", None, &[])
            .await
            .status()
    );
}
