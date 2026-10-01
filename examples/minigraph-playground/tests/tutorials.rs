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

//! The deployed tutorial set of the example app: all fourteen tutorials and `support-triage` are in
//! the manifest and pass the CompileGraph gate, and the two tutorials that reach outside a graph run
//! end to end through this app's OWN configuration - tutorial 13 (`graph.task` as an HTTP client that
//! fetches a profile from the app's dev mock endpoint) and tutorial 11 (`graph.extension` calling
//! `flow-11`, which this app's `flows.yaml` lists). The Java example's `GraphTests` is the twin for
//! tutorials 1 to 12; tutorial 14 has its own suite (`suspend_resume_tutorial`).
//!
//! The second test guards the sample flow files. The application's own `resources` win over the
//! engine crate's, so a copy that falls behind its default would hide an engine fix; the test fails
//! the moment the default changes and names the copies to refresh.

use std::time::Duration;

use platform_core::automation::AsyncHttpRequest;
use platform_core::{automation, overrides, AutoStart, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

// The application under test is a BIN crate - include its source so the link-time inventory in this
// test binary carries the engine skills the app links.
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

const TIMEOUT: Duration = Duration::from_secs(8);

// ---- helpers ----

fn get_element<'a>(body: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = body;
    for key in path {
        let Value::Map(entries) = current else {
            return None;
        };
        current = &entries.iter().find(|(k, _)| k.as_str() == Some(*key))?.1;
    }
    Some(current)
}

fn text_of(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(text)) => text.as_str().unwrap_or_default().to_string(),
        Some(other) => format!("{other}"),
        None => String::new(),
    }
}

fn json_map(entries: &[(&str, Value)]) -> Value {
    Value::Map(
        entries
            .iter()
            .map(|(k, v)| (Value::from(*k), v.clone()))
            .collect(),
    )
}

async fn post_graph(po: &PostOffice, target: &str, graph_id: &str, body: Value) -> EventEnvelope {
    let request = AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(target)
        .set_url(&format!("/api/graph/{graph_id}"))
        .set_header("content-type", "application/json")
        .set_header("accept", "application/json")
        .set_body(body);
    po.request(
        EventEnvelope::new()
            .set_to("async.http.request")
            .set_raw_body(request.to_value()),
        TIMEOUT,
    )
    .await
    .expect("graph run reply")
}

// ---- the deployed tutorials ----

// One test function on purpose (the repo convention): the app boots ONCE per process, so all
// scenarios run sequentially against the same server.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_tutorials_are_deployed_and_run() {
    // tutorial 13 calls this app over HTTP, and CompileGraph resolves ${rest.server.port:8080} when
    // the deployed model is loaded, so the port must be known before the app boots (not 0)
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("find a free port")
        .local_addr()
        .expect("local address")
        .port();
    overrides::set("rest.server.port", &port.to_string());
    let holding = std::env::temp_dir().join(format!("tutorials-{}", std::process::id()));
    overrides::set("transient.data.store", &holding.display().to_string());
    AutoStart::main(vec![]).await.expect("app lifecycle");
    assert_eq!(
        port,
        automation::server_address().expect("server started").port()
    );
    let target = format!("http://127.0.0.1:{port}");
    let po = PostOffice::new(&Platform::get_instance());

    // --- the manifest: fourteen tutorials and the triage graph, every one compiled
    let deployed = knowledge_graph::graphs::get_all_graphs();
    for n in 1..=14 {
        let id = format!("tutorial-{n}");
        assert!(deployed.contains(&id), "{id} is not deployed: {deployed:?}");
    }
    assert!(
        deployed.contains(&"support-triage".to_string()),
        "support-triage is not deployed: {deployed:?}"
    );

    // --- tutorial 13: graph.task invoking the AsyncHttpClient - the input mapping stages
    // 'model.person_id' and resolves it as a dynamic variable in the url
    let reply = post_graph(
        &po,
        &target,
        "tutorial-13",
        json_map(&[("person_id", Value::from(100))]),
    )
    .await;
    assert_eq!(200, reply.status(), "tutorial-13: {:?}", reply.body());
    let body = reply.body();
    assert_eq!("100", text_of(get_element(body, &["profile", "id"])));
    assert_eq!("Peter", text_of(get_element(body, &["profile", "name"])));
    assert_eq!(
        "100 World Blvd",
        text_of(get_element(body, &["profile", "address"]))
    );
    // 'text(5000) -> headers.x-ttl' rides the wire as the X-TTL request header - the mock echoes it
    assert_eq!("5000", text_of(get_element(body, &["observed_ttl"])));

    // --- tutorial 13 negative: the mock throws for an unknown profile and the graph returns the
    // HTTP error as its output
    let reply = post_graph(
        &po,
        &target,
        "tutorial-13",
        json_map(&[("person_id", Value::from(999))]),
    )
    .await;
    assert_ne!(200, reply.status(), "an unknown profile is an error");
    let text = format!("{:?}", reply.body());
    assert!(
        text.contains("Profile 999 not found"),
        "tutorial-13: {text}"
    );

    // --- tutorial 11: graph.extension -> flow://flow-11 (the echo flow this app's flows.yaml lists)
    let reply = post_graph(
        &po,
        &target,
        "tutorial-11",
        json_map(&[
            ("hello", Value::from("world")),
            ("message", Value::from("this is a good day")),
        ]),
    )
    .await;
    assert_eq!(200, reply.status(), "tutorial-11: {:?}", reply.body());
    let body = reply.body();
    assert_eq!("world", text_of(get_element(body, &["hello"])));
    assert_eq!(
        "this is a good day",
        text_of(get_element(body, &["message"]))
    );
}

// ---- the sample flow files ----

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The template's copy of the exposure flow opens with a comment block; the flow itself is the rest.
fn without_leading_comments(text: &str) -> String {
    text.lines()
        .skip_while(|line| line.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn the_sample_flows_equal_the_engines_defaults() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let engine = format!("{manifest}/../../crates/knowledge-graph/resources/flows");
    for flow in ["graph-executor.yml", "flow-11.yml"] {
        assert_eq!(
            read(&format!("{engine}/{flow}")),
            read(&format!("{manifest}/resources/flows/{flow}")),
            "examples/minigraph-playground/resources/flows/{flow} differs from the engine's default: \
             copy it again (the application's resources win, so a stale copy hides an engine fix)"
        );
    }
    assert_eq!(
        read(&format!("{engine}/graph-executor.yml")).trim_end(),
        without_leading_comments(&read(&format!(
            "{manifest}/../../templates/starter-graph/resources/flows/graph-executor.yml"
        )))
        .trim_end(),
        "templates/starter-graph/resources/flows/graph-executor.yml differs from the engine's default \
         (its opening comment aside): copy the flow again"
    );
}
