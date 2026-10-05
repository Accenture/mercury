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

//! The graph packager's command line, twin of the Java `GraphPackagerTest`.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use knowledge_graph::graph_set;
use platform_core::canonical_packager::{self, Builder};
use rmpv::Value;
use sha2::{Digest, Sha256};

const TUTORIALS: usize = 14;

/// A scratch folder removed when the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "graph-packager-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("scratch folder");
        Scratch(dir)
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    fn arg(&self, name: &str) -> String {
        self.path(name).display().to_string()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Outcome {
    code: i32,
    out: String,
    err: String,
}

fn run(args: &[&str]) -> Outcome {
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = graph_packager::run(&args, &mut out, &mut err);
    Outcome {
        code,
        out: String::from_utf8(out).expect("UTF-8"),
        err: String::from_utf8(err).expect("UTF-8"),
    }
}

/// Copy the engine's tutorial graphs, which every application can deploy, into a folder.
fn tutorials(folder: &Path) -> PathBuf {
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/knowledge-graph/resources/graph");
    fs::create_dir_all(folder).expect("folder");
    for i in 1..=TUTORIALS {
        let name = format!("tutorial-{i}.json");
        fs::copy(source.join(&name), folder.join(&name)).expect("a tutorial");
    }
    folder.to_path_buf()
}

fn parse(file: &Path) -> Value {
    let text = fs::read_to_string(file).expect("readable");
    event_script::conversions::from_json(&serde_json::from_str(&text).expect("JSON"))
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn write(file: &Path, text: &str) -> PathBuf {
    fs::create_dir_all(file.parent().expect("a parent")).expect("folder");
    fs::write(file, text).expect("written");
    file.to_path_buf()
}

#[test]
fn packs_the_tutorials_and_every_entry_equals_its_file() {
    let s = Scratch::new();
    let graphs = tutorials(&s.path("graphs"));
    let r = run(&[
        "pack",
        "--set",
        "tutorials",
        "--manifest",
        "version=1.0.0",
        "--out",
        &s.arg("out"),
        &s.arg("graphs"),
    ]);
    assert_eq!(0, r.code, "{}", r.err);
    let file = s.path("out").join("tutorials.pack");
    let bytes = fs::read(&file).expect("a package");
    assert!(
        r.out
            .contains(&format!("Packed 14 graphs into {}", file.display())),
        "{}",
        r.out
    );
    assert!(
        r.out.contains(&format!("SHA-256 {}", sha256(&bytes))),
        "{}",
        r.out
    );
    let contents = graph_set::read(&bytes).expect("a set");
    assert_eq!(Some("mercury-package"), contents.manifest_field("format"));
    assert_eq!(Some("tutorials"), contents.manifest_field("set"));
    assert_eq!(Some("1.0.0"), contents.manifest_field("version"));
    assert_eq!(TUTORIALS, contents.graphs.len());
    for (id, model) in &contents.graphs {
        let original = parse(&graphs.join(format!("{id}.json")));
        assert_eq!(
            canonical_packager::encode(&original).expect("encodes"),
            canonical_packager::encode(model).expect("encodes"),
            "{id}"
        );
    }
}

#[test]
fn the_same_graphs_and_fields_give_the_same_bytes() {
    let s = Scratch::new();
    tutorials(&s.path("a"));
    assert_eq!(
        0,
        run(&[
            "pack",
            "--set",
            "s",
            "--manifest",
            "version=1",
            "--manifest",
            "author=team",
            "--out",
            &s.arg("first"),
            &s.arg("a")
        ])
        .code
    );
    // the same graphs from another folder, named one by one in reverse order, and the fields in reverse order
    let copy = tutorials(&s.path("b"));
    let second = s.arg("second");
    let mut args = vec![
        "pack".to_string(),
        "--set".into(),
        "s".into(),
        "--manifest".into(),
        "author=team".into(),
        "--manifest".into(),
        "version=1".into(),
        "--out".into(),
        second,
    ];
    for i in (1..=TUTORIALS).rev() {
        args.push(
            copy.join(format!("tutorial-{i}.json"))
                .display()
                .to_string(),
        );
    }
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    assert_eq!(0, run(&refs).code);
    assert_eq!(
        fs::read(s.path("first").join("s.pack")).expect("first"),
        fs::read(s.path("second").join("s.pack")).expect("second")
    );
}

#[test]
fn unpack_writes_readable_json_that_packs_to_the_same_bytes() {
    let s = Scratch::new();
    tutorials(&s.path("graphs"));
    run(&[
        "pack",
        "--set",
        "tutorials",
        "--manifest",
        "version=1.0.0",
        "--out",
        &s.arg("out"),
        &s.arg("graphs"),
    ]);
    let package = s.path("out").join("tutorials.pack");
    let r = run(&[
        "unpack",
        &package.display().to_string(),
        "--out",
        &s.arg("unpacked"),
    ]);
    assert_eq!(0, r.code, "{}", r.err);
    assert!(r.out.contains("Unpacked 14 graphs from"), "{}", r.out);
    // the canonical key order and a two-space indent, so two versions of a set diff cleanly
    let text = fs::read_to_string(s.path("unpacked").join("tutorial-1.json")).expect("unpacked");
    assert!(
        text.starts_with("{\n  \"connections\": [\n    {\n      \"relations\": ["),
        "{text}"
    );
    assert!(text.ends_with("}\n"));
    assert_eq!(
        0,
        run(&[
            "pack",
            "--set",
            "tutorials",
            "--manifest",
            "version=1.0.0",
            "--out",
            &s.arg("repacked"),
            &s.arg("unpacked")
        ])
        .code
    );
    assert_eq!(
        fs::read(&package).expect("packed"),
        fs::read(s.path("repacked").join("tutorials.pack")).expect("repacked")
    );
}

#[test]
fn a_set_is_refused_with_every_reason_the_gate_gives() {
    let s = Scratch::new();
    let graphs = tutorials(&s.path("graphs"));
    write(
        &graphs.join("no-end.json"),
        r#"{"nodes": [{"alias": "root", "types": ["Root"],
        "properties": {"purpose": "a graph that cannot complete", "name": "no-end"}}], "connections": []}"#,
    );
    write(
        &graphs.join("no-purpose.json"),
        r#"{"nodes": [{"alias": "root", "types": ["Root"], "properties": {"name": "no-purpose"}},
        {"alias": "end", "types": ["End"], "properties": {}}],
        "connections": [{"source": "root", "target": "end", "relations": [{"type": "done", "properties": {}}]}]}"#,
    );
    let r = run(&[
        "pack",
        "--set",
        "mixed",
        "--out",
        &s.arg("out"),
        &s.arg("graphs"),
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err.contains("no-end: graph must have an 'end' node"),
        "{}",
        r.err
    );
    assert!(
        r.err
            .contains("no-purpose: root node must define a non-empty 'purpose' property"),
        "{}",
        r.err
    );
    assert!(!s.path("out").join("mixed.pack").exists());
}

#[test]
fn names_that_break_the_rules_are_refused() {
    let s = Scratch::new();
    let graphs = tutorials(&s.path("graphs"));
    fs::copy(graphs.join("tutorial-1.json"), graphs.join("bad.name.json")).expect("copy");
    fs::copy(graphs.join("tutorial-1.json"), graphs.join("renamed.json")).expect("copy");
    let r = run(&[
        "pack",
        "--set",
        "bad set",
        "--out",
        &s.arg("out"),
        &s.arg("graphs"),
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err
            .contains("set name 'bad set' - use letters, digits, '_' and '-' only"),
        "{}",
        r.err
    );
    assert!(
        r.err
            .contains("bad.name: graph id - use letters, digits, '_' and '-' only"),
        "{}",
        r.err
    );
    assert!(
        r.err
            .contains("renamed: the root node's name 'tutorial-1' differs from the graph id"),
        "{}",
        r.err
    );
}

#[test]
fn manifest_fields_follow_the_set_rules() {
    let s = Scratch::new();
    tutorials(&s.path("graphs"));
    let (graphs, out) = (s.arg("graphs"), s.arg("out"));
    assert_eq!(
        0,
        run(&[
            "pack",
            "--set",
            "entry",
            "--manifest",
            "graph_id=tutorial-1",
            "--out",
            &out,
            &graphs
        ])
        .code
    );
    let contents = graph_set::read(&fs::read(s.path("out").join("entry.pack")).expect("entry"))
        .expect("a set");
    assert_eq!(Some("tutorial-1"), contents.manifest_field("graph_id"));
    let r = run(&[
        "pack",
        "--set",
        "s",
        "--manifest",
        "graph_id=nope",
        "--out",
        &out,
        &graphs,
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err
            .contains("manifest field 'graph_id' - 'nope' is not a graph of the set"),
        "{}",
        r.err
    );
    let r = run(&[
        "pack",
        "--set",
        "s",
        "--manifest",
        "format=x",
        "--manifest",
        "set=y",
        "--out",
        &out,
        &graphs,
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err
            .contains("manifest field 'format' - it is written by the packager"),
        "{}",
        r.err
    );
    assert!(
        r.err
            .contains("manifest field 'set' - it is written from the set name"),
        "{}",
        r.err
    );
    let r = run(&[
        "pack",
        "--set",
        "s",
        "--manifest",
        "novalue",
        "--out",
        &out,
        &graphs,
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err.contains("--manifest takes key=value, not 'novalue'"),
        "{}",
        r.err
    );
    let r = run(&[
        "pack",
        "--set",
        "s",
        "--manifest",
        "k=1",
        "--manifest",
        "k=2",
        "--out",
        &out,
        &graphs,
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err.contains("The manifest field 'k' is given twice"),
        "{}",
        r.err
    );
}

#[test]
fn packs_exactly_what_a_deployment_manifest_lists() {
    let s = Scratch::new();
    let graphs = tutorials(&s.path("graphs"));
    let out = s.arg("out");
    let manifest = write(
        &s.path("deploy/graphs.yaml"),
        &format!(
            "graphs:\n  - 'tutorial-1'\n  - 'tutorial-2'\nlocation: 'file:{}'\n",
            graphs.display()
        ),
    );
    let r = run(&[
        "pack",
        "--set",
        "two",
        "--from-manifest",
        &manifest.display().to_string(),
        "--out",
        &out,
    ]);
    assert_eq!(0, r.code, "{}", r.err);
    let contents =
        graph_set::read(&fs::read(s.path("out").join("two.pack")).expect("two")).expect("a set");
    let ids: Vec<&str> = contents.graphs.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(vec!["tutorial-1", "tutorial-2"], ids);
    // a folder written without file: resolves against the manifest's own folder
    let relative = write(
        &s.path("graphs.yaml"),
        "graphs:\n  - 'tutorial-3'\nlocation: 'graphs'\n",
    );
    assert_eq!(
        0,
        run(&[
            "pack",
            "--set",
            "three",
            "--from-manifest",
            &relative.display().to_string(),
            "--out",
            &out
        ])
        .code
    );
    // a classpath location is inside an application, so the pipeline is told to pass the folder
    let classpath = write(
        &s.path("cp/graphs.yaml"),
        "graphs:\n  - 'tutorial-1'\nlocation: 'classpath:/graph'\n",
    );
    let r = run(&[
        "pack",
        "--set",
        "cp",
        "--from-manifest",
        &classpath.display().to_string(),
        "--out",
        &out,
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err
            .contains("inside an application - pass the folder that holds the graphs instead"),
        "{}",
        r.err
    );
    // the default location is the classpath too
    let no_location = write(&s.path("none/graphs.yaml"), "graphs:\n  - 'tutorial-1'\n");
    assert_eq!(
        1,
        run(&[
            "pack",
            "--set",
            "none",
            "--from-manifest",
            &no_location.display().to_string(),
            "--out",
            &out
        ])
        .code
    );
    // a listed id is checked before it becomes a path
    let crafted = write(
        &s.path("crafted/graphs.yaml"),
        &format!(
            "graphs:\n  - '../graphs/tutorial-1'\nlocation: 'file:{}'\n",
            graphs.display()
        ),
    );
    let r = run(&[
        "pack",
        "--set",
        "crafted",
        "--from-manifest",
        &crafted.display().to_string(),
        "--out",
        &out,
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err.contains("graph id '../graphs/tutorial-1'"),
        "{}",
        r.err
    );
    let r = run(&[
        "pack",
        "--set",
        "x",
        "--from-manifest",
        &manifest.display().to_string(),
        &graphs.display().to_string(),
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err
            .contains("Give the graphs or --from-manifest, not both"),
        "{}",
        r.err
    );
}

#[test]
fn unpack_refuses_a_crafted_entry_name_before_writing_anything() {
    let s = Scratch::new();
    let model = parse(&tutorials(&s.path("graphs")).join("tutorial-1.json"));
    let crafted = s.path("crafted.pack");
    let bytes = Builder::new()
        .manifest("set", "crafted")
        .and_then(|b| b.add("../evil.json", model))
        .and_then(|b| b.build())
        .expect("a package");
    fs::write(&crafted, bytes).expect("written");
    let target = s.path("target/out");
    let r = run(&[
        "unpack",
        &crafted.display().to_string(),
        "--out",
        &target.display().to_string(),
    ]);
    assert_eq!(1, r.code);
    assert!(
        r.err
            .contains("entry '../evil.json' - expect <graph-id>.json"),
        "{}",
        r.err
    );
    assert!(!s.path("target/evil.json").exists());
    assert!(!target.exists());
}

#[test]
fn a_package_that_is_not_canonical_is_a_format_error() {
    let s = Scratch::new();
    tutorials(&s.path("graphs"));
    run(&[
        "pack",
        "--set",
        "tutorials",
        "--out",
        &s.arg("out"),
        &s.arg("graphs"),
    ]);
    let mut bytes = fs::read(s.path("out").join("tutorials.pack")).expect("a package");
    bytes.push(0);
    let corrupt = write(&s.path("corrupt.pack"), "");
    fs::write(&corrupt, bytes).expect("written");
    let r = run(&[
        "unpack",
        &corrupt.display().to_string(),
        "--out",
        &s.arg("x"),
    ]);
    assert_eq!(2, r.code);
    assert!(r.err.starts_with("Error: "), "{}", r.err);
    assert_eq!(2, run(&["inspect", &corrupt.display().to_string()]).code);
}

#[test]
fn inspect_reports_the_set_for_pipelines() {
    let s = Scratch::new();
    tutorials(&s.path("graphs"));
    run(&[
        "pack",
        "--set",
        "tutorials",
        "--manifest",
        "version=1.0.0",
        "--out",
        &s.arg("out"),
        &s.arg("graphs"),
    ]);
    let file = s.path("out").join("tutorials.pack");
    let bytes = fs::read(&file).expect("a package");
    let r = run(&["inspect", &file.display().to_string(), "--json"]);
    assert_eq!(0, r.code, "{}", r.err);
    // standard output holds the report alone: the engine's log goes to standard error
    let report: serde_json::Value = serde_json::from_str(&r.out).expect("a JSON report");
    assert_eq!(sha256(&bytes), report["sha256"]);
    assert_eq!(bytes.len() as u64, report["size"].as_u64().expect("a size"));
    assert_eq!("tutorials", report["manifest"]["set"]);
    let graphs = report["graphs"].as_array().expect("graphs");
    assert_eq!(TUTORIALS, graphs.len());
    assert_eq!("tutorial-1", graphs[0]["id"]);
    assert_eq!(2, graphs[0]["nodes"]);
    assert_eq!(1, graphs[0]["connections"]);
    let r = run(&["inspect", &file.display().to_string()]);
    assert_eq!(0, r.code, "{}", r.err);
    assert!(
        r.out.contains(&format!("SHA-256   {}", sha256(&bytes))),
        "{}",
        r.out
    );
    assert!(
        r.out.contains("  tutorial-1   2 nodes, 1 connection"),
        "{}",
        r.out
    );
    assert!(r.out.contains("  version         1.0.0"), "{}", r.out);
}

#[test]
fn a_missing_file_is_an_io_error() {
    let s = Scratch::new();
    let missing = s.arg("missing");
    assert_eq!(2, run(&["pack", "--set", "s", &missing]).code);
    assert_eq!(
        2,
        run(&["pack", "--set", "s", "--from-manifest", &missing]).code
    );
    assert_eq!(2, run(&["unpack", &missing, "--out", &s.arg("x")]).code);
    let r = run(&["inspect", &missing]);
    assert_eq!(2, r.code);
    assert!(r.err.contains("no such file"), "{}", r.err);
    let not_json = write(&s.path("broken/broken.json"), "{ not json");
    let r = run(&["pack", "--set", "s", &not_json.display().to_string()]);
    assert_eq!(2, r.code);
    assert!(r.err.contains("is not a JSON graph model"), "{}", r.err);
}

#[test]
fn usage_errors_are_refused_with_the_usage() {
    assert_eq!(1, run(&[]).code);
    let r = run(&["nope"]);
    assert_eq!(1, r.code);
    assert!(
        r.err.contains("Unknown command 'nope'") && r.err.contains("Usage:"),
        "{}",
        r.err
    );
    assert!(run(&["pack", "x.json"])
        .err
        .contains("pack needs --set <name>"));
    assert!(run(&["unpack", "x.pack"])
        .err
        .contains("unpack needs --out <dir>"));
    assert!(run(&["inspect", "x.pack", "--verbose"])
        .err
        .contains("Unknown option '--verbose'"));
    let r = run(&["--help"]);
    assert_eq!(0, r.code);
    assert!(
        r.out
            .contains("graph-packager inspect <file.pack> [--json]"),
        "{}",
        r.out
    );
}

#[test]
fn a_reference_is_checked_resolved_but_packed_as_written() {
    // the gate reads a deployed model with its ${...} references resolved; the package keeps them,
    // because they belong to the environment the set is deployed to
    let s = Scratch::new();
    write(
        &s.path("ref/ref-graph.json"),
        r#"{"nodes": [
        {"alias": "root", "types": ["Root"],
         "properties": {"purpose": "${GRAPH_PACKAGER_TEST_PURPOSE:resolved where it is deployed}", "name": "ref-graph"}},
        {"alias": "end", "types": ["End"], "properties": {}}],
        "connections": [{"source": "root", "target": "end", "relations": [{"type": "done", "properties": {}}]}]}"#,
    );
    assert_eq!(
        0,
        run(&[
            "pack",
            "--set",
            "ref",
            "--out",
            &s.arg("out"),
            &s.arg("ref")
        ])
        .code
    );
    let contents =
        graph_set::read(&fs::read(s.path("out").join("ref.pack")).expect("ref")).expect("a set");
    let json = graph_set::to_json(contents.graph("ref-graph").expect("ref-graph"));
    assert!(
        json.contains("${GRAPH_PACKAGER_TEST_PURPOSE:resolved where it is deployed}"),
        "{json}"
    );
}
