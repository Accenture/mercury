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

//! The graph-set loader (ADR-0027 in the Java repository's ledger): a manifest's `sets` are unpacked into its
//! `unpack` folder and deployed all or none, right after the manifest's own graphs. Twin of the Java
//! `GraphSetLoaderTest`: each step builds its packages from JSON in a scratch folder and compiles its own
//! manifest in the running test application, with graph ids no other step uses.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use async_trait::async_trait;
use event_script::FlowExecutor;
use knowledge_graph::{compiler, graph_set, graph_set_loader, graphs};
use platform_core::canonical_packager::Builder;
use platform_core::{
    main_application, trace, AppError, AutoStart, EntryPoint, EventEnvelope, Platform,
};
use rmpv::Value;

/// A scratch folder removed when the step ends - never inside the Playground's temporary folder.
struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "graph-set-loader-{}-{}",
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

fn graph(id: &str, answer: &str) -> Value {
    event_script::conversions::from_json(&serde_json::json!({
        "nodes": [
            {"alias": "root", "types": ["Root"], "properties": {"purpose": "a graph deployed from a set", "name": id}},
            {"alias": "end", "types": ["End"],
             "properties": {"skill": "graph.data.mapper", "mapping": [format!("text({answer}) -> output.body")]}}],
        "connections": [{"source": "root", "target": "end", "relations": [{"type": "done", "properties": {}}]}]
    }))
}

fn graph_without_end() -> Value {
    event_script::conversions::from_json(&serde_json::json!({
        "nodes": [{"alias": "root", "types": ["Root"],
                   "properties": {"purpose": "no end node", "name": "unit-test-set-no-end"}}],
        "connections": []
    }))
}

fn pack(folder: &Path, set_name: &str, version: Option<&str>, graphs: &[(&str, Value)]) {
    std::fs::create_dir_all(folder).expect("folder");
    let fields: Vec<(String, String)> = version
        .map(|v| vec![("version".to_string(), v.to_string())])
        .unwrap_or_default();
    let models: BTreeMap<String, Value> = graphs
        .iter()
        .map(|(id, model)| (id.to_string(), model.clone()))
        .collect();
    let bytes = graph_set::pack(set_name, &fields, &models).expect("a set");
    std::fs::write(
        folder.join(format!("{set_name}{}", graph_set::EXTENSION)),
        bytes,
    )
    .expect("written");
}

/// A package the packager would refuse, built with the canonical packager directly - what a deployment can meet.
fn craft(folder: &Path, set_name: &str, entries: &[(&str, Value)]) {
    std::fs::create_dir_all(folder).expect("folder");
    let mut builder = Builder::new()
        .manifest(graph_set::SET, set_name)
        .expect("a field");
    for (name, model) in entries {
        builder = builder.add(name, model.clone()).expect("an entry");
    }
    std::fs::write(
        folder.join(format!("{set_name}{}", graph_set::EXTENSION)),
        builder.build().expect("bytes"),
    )
    .expect("written");
}

fn manifest(
    file: &Path,
    location: &str,
    loose: &[&str],
    sets: &[&str],
    unpack: Option<&str>,
) -> String {
    let mut text = String::new();
    if !loose.is_empty() {
        text.push_str("graphs:\n");
        for id in loose {
            text.push_str(&format!("  - '{id}'\n"));
        }
    }
    text.push_str(&format!("location: '{location}'\n"));
    if !sets.is_empty() {
        text.push_str("sets:\n");
        for name in sets {
            text.push_str(&format!("  - '{name}'\n"));
        }
    }
    if let Some(unpack) = unpack {
        text.push_str(&format!("unpack: '{unpack}'\n"));
    }
    std::fs::write(file, text).expect("a manifest");
    format!("file:{}", file.display())
}

fn write_graph(folder: &Path, id: &str, model: &Value) {
    std::fs::create_dir_all(folder).expect("folder");
    let json = event_script::conversions::to_json(model).expect("JSON");
    std::fs::write(folder.join(format!("{id}.json")), json.to_string()).expect("a graph file");
}

fn forget(ids: &[&str]) {
    // the registry is shared by every step in this process
    for id in ids {
        graphs::remove_graph(id);
    }
}

async fn run(platform: &Platform, graph_id: &str) -> EventEnvelope {
    let dataset = serde_json::json!({
        "body": {},
        "header": {},
        "path_parameter": {"graph_id": graph_id},
        "method": "POST",
    });
    FlowExecutor::request(
        platform,
        "graph-executor",
        event_script::conversions::from_json(&dataset),
        &format!("cid-{graph_id}"),
        Duration::from_secs(8),
        Some((&trace::new_trace_id(), &format!("TEST /graph/{graph_id}"))),
    )
    .await
    .unwrap_or_else(|e| panic!("graph {graph_id} failed: {} {}", e.status(), e.message()))
}

#[main_application]
struct GraphSetLoaderTestApp;

#[async_trait]
impl EntryPoint for GraphSetLoaderTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        // referencing both engine crates guarantees their inventories link
        log::info!(
            "Flows ready: {:?}, graphs compiled: {}",
            event_script::flows::get_all_flows().len(),
            graphs::get_all_graphs().len()
        );
        Ok(())
    }
}

async fn boot() -> Platform {
    platform_core::resources::prepend_resource_root("tests/resources");
    AutoStart::main(vec![]).await.expect("lifecycle");
    Platform::get_instance()
}

/// One runtime for every step: a second `#[tokio::test]` gets its own runtime, which would drop the platform the
/// first one started.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_set_loader_end_to_end() {
    test_support::run_at_exit(platform_core::util::elastic_queue::shutdown_cleanup);
    let platform = boot().await;
    a_set_deploys_its_graphs_and_the_endpoint_serves_them(&platform).await;
    a_set_with_a_graph_the_gate_refuses_registers_none();
    a_duplicate_involving_a_set_is_won_by_the_later_copy(&platform).await;
    sets_need_a_writable_unpack_folder_of_their_own();
    the_next_start_removes_the_files_of_a_graph_the_set_no_longer_holds();
    a_crafted_entry_name_is_refused_before_any_file_is_written();
    a_missing_package_is_not_deployed_and_the_other_sets_are();
    a_set_is_read_from_the_classpath_too(&platform).await;
}

async fn a_set_deploys_its_graphs_and_the_endpoint_serves_them(platform: &Platform) {
    let s = Scratch::new();
    pack(
        &s.path("packs"),
        "served-set",
        Some("1.0.0"),
        &[
            (
                "unit-test-set-one",
                graph("unit-test-set-one", "one from the set"),
            ),
            (
                "unit-test-set-two",
                graph("unit-test-set-two", "two from the set"),
            ),
        ],
    );
    let unpack = s.file("unpack");
    compiler::compile_manifest(&manifest(
        &s.path("graphs.yaml"),
        &s.file("packs"),
        &[],
        &["served-set"],
        Some(&unpack),
    ));
    for id in ["unit-test-set-one", "unit-test-set-two"] {
        assert!(graphs::graph_exists(id), "{id}");
        assert_eq!(Some(unpack.clone()), graphs::graph_location(id));
        assert_eq!(
            Some(graphs::DeployedSet {
                name: "served-set".to_string(),
                version: "1.0.0".to_string()
            }),
            graphs::graph_set(id)
        );
        assert!(s.path(&format!("unpack/{id}.json")).is_file(), "{id}");
    }
    assert!(graphs::deployed_locations().contains(&unpack));
    let response = run(platform, "unit-test-set-one").await;
    assert_eq!(200, response.status());
    assert_eq!(&Value::from("one from the set"), response.body());
    // the generated manifest records what deployed and the files the loader wrote
    let generated = std::fs::read_to_string(s.path("unpack/graphs.yaml")).expect("generated");
    assert!(
        generated.contains("graphs:\n  - 'unit-test-set-one'\n  - 'unit-test-set-two'\n"),
        "{generated}"
    );
    assert!(
        generated.contains(&format!("location: '{unpack}'")),
        "{generated}"
    );
    assert!(
        generated.contains("generated:\n  'served-set':\n    - 'unit-test-set-one'\n"),
        "{generated}"
    );
    assert!(
        generated.contains(&format!(
            "# set served-set: {}/served-set.pack, SHA-256 ",
            s.file("packs")
        )),
        "{generated}"
    );
    assert!(
        generated.contains(", version=1.0.0 - deployed"),
        "{generated}"
    );
    forget(&["unit-test-set-one", "unit-test-set-two"]);
}

fn a_set_with_a_graph_the_gate_refuses_registers_none() {
    let s = Scratch::new();
    craft(
        &s.path("packs"),
        "half-set",
        &[
            (
                "unit-test-set-good.json",
                graph("unit-test-set-good", "good"),
            ),
            ("unit-test-set-no-end.json", graph_without_end()),
        ],
    );
    compiler::compile_manifest(&manifest(
        &s.path("graphs.yaml"),
        &s.file("packs"),
        &[],
        &["half-set"],
        Some(&s.file("unpack")),
    ));
    assert!(
        !graphs::graph_exists("unit-test-set-good"),
        "all or none: the valid graph is not registered"
    );
    assert!(!graphs::graph_exists("unit-test-set-no-end"));
    // the files stay for the operator to inspect, and the next start removes them
    assert!(s.path("unpack/unit-test-set-good.json").is_file());
    let generated = std::fs::read_to_string(s.path("unpack/graphs.yaml")).expect("generated");
    assert!(generated.contains("graphs: []\n"), "{generated}");
    assert!(generated.contains("  'half-set':\n"), "{generated}");
    assert!(generated.contains(" - rejected\n"), "{generated}");
}

async fn a_duplicate_involving_a_set_is_won_by_the_later_copy(platform: &Platform) {
    let s = Scratch::new();
    write_graph(
        &s.path("loose"),
        "unit-test-set-dup",
        &graph("unit-test-set-dup", "loose"),
    );
    // a loose graph, then a set holding the same id: the set's copy wins
    compiler::compile_manifest(&manifest(
        &s.path("m1.yaml"),
        &s.file("loose"),
        &["unit-test-set-dup"],
        &[],
        None,
    ));
    assert_eq!(
        Some(s.file("loose")),
        graphs::graph_location("unit-test-set-dup")
    );
    pack(
        &s.path("packs"),
        "dup-set",
        Some("2"),
        &[("unit-test-set-dup", graph("unit-test-set-dup", "set"))],
    );
    compiler::compile_manifest(&manifest(
        &s.path("m2.yaml"),
        &s.file("packs"),
        &[],
        &["dup-set"],
        Some(&s.file("unpack")),
    ));
    assert_eq!(
        Some(s.file("unpack")),
        graphs::graph_location("unit-test-set-dup")
    );
    assert_eq!(
        Some("dup-set".to_string()),
        graphs::graph_set("unit-test-set-dup").map(|set| set.name)
    );
    assert_eq!(
        &Value::from("set"),
        run(platform, "unit-test-set-dup").await.body()
    );
    // a later loose copy wins over the set's, and the graph no longer belongs to a set
    compiler::compile_manifest(&manifest(
        &s.path("m3.yaml"),
        &s.file("loose"),
        &["unit-test-set-dup"],
        &[],
        None,
    ));
    assert_eq!(
        Some(s.file("loose")),
        graphs::graph_location("unit-test-set-dup")
    );
    assert_eq!(None, graphs::graph_set("unit-test-set-dup"));
    assert_eq!(
        &Value::from("loose"),
        run(platform, "unit-test-set-dup").await.body()
    );
    forget(&["unit-test-set-dup"]);
}

fn sets_need_a_writable_unpack_folder_of_their_own() {
    let s = Scratch::new();
    pack(
        &s.path("packs"),
        "folder-set",
        None,
        &[("unit-test-set-folder", graph("unit-test-set-folder", "x"))],
    );
    write_graph(
        &s.path("packs"),
        "unit-test-set-loose",
        &graph("unit-test-set-loose", "loose"),
    );
    // no 'unpack': the sets are skipped and the manifest's own graphs still compile
    compiler::compile_manifest(&manifest(
        &s.path("m1.yaml"),
        &s.file("packs"),
        &["unit-test-set-loose"],
        &["folder-set"],
        None,
    ));
    assert!(graphs::graph_exists("unit-test-set-loose"));
    assert!(!graphs::graph_exists("unit-test-set-folder"));
    // classpath: is refused - the loader writes there
    compiler::compile_manifest(&manifest(
        &s.path("m2.yaml"),
        &s.file("packs"),
        &[],
        &["folder-set"],
        Some("classpath:/unpacked"),
    ));
    assert!(!graphs::graph_exists("unit-test-set-folder"));
    // the Playground's temporary folder, or a folder inside it, is refused - its housekeeping deletes files
    compiler::compile_manifest(&manifest(
        &s.path("m3.yaml"),
        &s.file("packs"),
        &[],
        &["folder-set"],
        Some("file:/tmp/graph/unpacked"),
    ));
    assert!(!graphs::graph_exists("unit-test-set-folder"));
    assert!(!Path::new("/tmp/graph/unpacked").exists());
    // one folder serves one manifest: the second manifest's sets are skipped
    compiler::compile_manifest(&manifest(
        &s.path("m4.yaml"),
        &s.file("packs"),
        &[],
        &["folder-set"],
        Some(&s.file("unpack")),
    ));
    assert!(graphs::graph_exists("unit-test-set-folder"));
    graphs::remove_graph("unit-test-set-folder");
    compiler::compile_manifest(&manifest(
        &s.path("m5.yaml"),
        &s.file("packs"),
        &[],
        &["folder-set"],
        Some(&s.file("unpack")),
    ));
    assert!(!graphs::graph_exists("unit-test-set-folder"));
    forget(&["unit-test-set-loose"]);
}

fn the_next_start_removes_the_files_of_a_graph_the_set_no_longer_holds() {
    let s = Scratch::new();
    let unpack = s.path("unpack");
    std::fs::create_dir_all(&unpack).expect("folder");
    std::fs::write(unpack.join("notes.txt"), "the operator's own file").expect("a note");
    pack(
        &s.path("packs"),
        "versioned-set",
        Some("1"),
        &[
            ("unit-test-set-kept", graph("unit-test-set-kept", "kept")),
            (
                "unit-test-set-dropped",
                graph("unit-test-set-dropped", "dropped"),
            ),
        ],
    );
    let manifest = manifest(
        &s.path("graphs.yaml"),
        &s.file("packs"),
        &[],
        &["versioned-set"],
        Some(&s.file("unpack")),
    );
    compiler::compile_manifest(&manifest);
    assert!(unpack.join("unit-test-set-dropped.json").is_file());
    // version 2 drops a graph; a new start unpacks it into the same folder
    pack(
        &s.path("packs"),
        "versioned-set",
        Some("2"),
        &[("unit-test-set-kept", graph("unit-test-set-kept", "v2"))],
    );
    graph_set_loader::reset();
    compiler::compile_manifest(&manifest);
    assert!(
        !unpack.join("unit-test-set-dropped.json").exists(),
        "the dropped graph's file is removed"
    );
    assert!(unpack.join("unit-test-set-kept.json").is_file());
    assert!(
        unpack.join("notes.txt").is_file(),
        "a file the loader did not write stays"
    );
    assert_eq!(
        Some("2".to_string()),
        graphs::graph_set("unit-test-set-kept").map(|set| set.version)
    );
    forget(&["unit-test-set-kept", "unit-test-set-dropped"]);
}

fn a_crafted_entry_name_is_refused_before_any_file_is_written() {
    let s = Scratch::new();
    craft(
        &s.path("packs"),
        "crafted-set",
        &[("../escaped.json", graph("escaped", "x"))],
    );
    compiler::compile_manifest(&manifest(
        &s.path("graphs.yaml"),
        &s.file("packs"),
        &[],
        &["crafted-set"],
        Some(&s.file("unpack/inner")),
    ));
    assert!(!s.path("unpack/escaped.json").exists());
    assert!(!graphs::graph_exists("escaped"));
    let generated = std::fs::read_to_string(s.path("unpack/inner/graphs.yaml")).expect("generated");
    assert!(
        generated.contains("# set crafted-set not deployed - its names break the set rules\n"),
        "{generated}"
    );
}

fn a_missing_package_is_not_deployed_and_the_other_sets_are() {
    let s = Scratch::new();
    pack(
        &s.path("packs"),
        "present-set",
        None,
        &[("unit-test-set-present", graph("unit-test-set-present", "x"))],
    );
    compiler::compile_manifest(&manifest(
        &s.path("graphs.yaml"),
        &s.file("packs"),
        &[],
        &["absent-set", "present-set"],
        Some(&s.file("unpack")),
    ));
    assert!(graphs::graph_exists("unit-test-set-present"));
    // a set without a version field deploys with an empty version
    assert_eq!(
        Some(String::new()),
        graphs::graph_set("unit-test-set-present").map(|set| set.version)
    );
    let generated = std::fs::read_to_string(s.path("unpack/graphs.yaml")).expect("generated");
    assert!(
        generated.contains(&format!(
            "# set absent-set not deployed - {}/absent-set.pack not found\n",
            s.file("packs")
        )),
        "{generated}"
    );
    forget(&["unit-test-set-present"]);
}

async fn a_set_is_read_from_the_classpath_too(platform: &Platform) {
    // the manifest's location may be classpath:/ - a resource root added for this step serves it
    let s = Scratch::new();
    platform_core::resources::append_resource_root(s.path("resources"));
    pack(
        &s.path("resources/graph-set-test"),
        "classpath-set",
        Some("3"),
        &[(
            "unit-test-set-classpath",
            graph("unit-test-set-classpath", "from the classpath"),
        )],
    );
    compiler::compile_manifest(&manifest(
        &s.path("graphs.yaml"),
        "classpath:/graph-set-test",
        &[],
        &["classpath-set"],
        Some(&s.file("unpack")),
    ));
    assert!(graphs::graph_exists("unit-test-set-classpath"));
    assert_eq!(
        &Value::from("from the classpath"),
        run(platform, "unit-test-set-classpath").await.body()
    );
    forget(&["unit-test-set-classpath"]);
}
