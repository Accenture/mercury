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

//! K-7b: the Playground command grammar (Java `GraphCommandService` +
//! `GraphUserInterface` protocol) driven directly through the command
//! service, then the AI-companion REST hop end-to-end (the field use case:
//! an agent POSTs a text command, output streams to the session console).

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;

use async_trait::async_trait;
use event_script::conversions::display;
use event_script::mlm::MultiLevelMap;
use platform_core::{
    main_application, overrides, AppError, AutoStart, ComposableFunction, EventEnvelope, Platform,
    PostOffice,
};
use rmpv::Value;

/// A stand-in websocket console: records every line the command service
/// sends to the session's `.out` route.
struct Console {
    lines: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl ComposableFunction for Console {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let line = match input.body() {
            Value::String(s) => s.as_str().unwrap_or_default().to_string(),
            other => display(other),
        };
        self.lines.lock().expect("console").push(line);
        EventEnvelope::new().set_body("ok")
    }
}

#[main_application]
struct PlaygroundTestApp;

#[async_trait]
impl platform_core::EntryPoint for PlaygroundTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "playground test app started; graphs compiled: {}",
            knowledge_graph::graphs::get_all_graphs().len()
        );
        Ok(())
    }
}

async fn boot() -> Platform {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let dir = std::env::temp_dir().join(format!("mercury-playground-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        overrides::set("location.graph.temp", &format!("file:{}", dir.display()));
        overrides::set("app.env", "dev");
        overrides::set("rest.server.port", "0");
    });
    platform_core::resources::prepend_resource_root("tests/resources");
    // the dev-gated PlaygroundLoader registers websocket services, so the
    // lifecycle starts the single HTTP server itself (on the ephemeral port
    // from `rest.server.port=0`). AutoStart::main returns once the app is
    // booted — the server is already bound — so we read its address after.
    AutoStart::main(vec![]).await.expect("lifecycle");
    Platform::get_instance()
}

fn base_url() -> String {
    let addr = platform_core::automation::server_address().expect("server bound");
    format!("http://127.0.0.1:{}", addr.port())
}

/// Drive one command through the command service as the websocket UI would,
/// then wait briefly for the async console output to settle.
async fn command(po: &PostOffice, in_route: &str, out_route: &str, message: &str) {
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(knowledge_graph::commands::ROUTE)
                .set_raw_body(Value::Map(vec![
                    (Value::from("type"), Value::from("command")),
                    (Value::from("in"), Value::from(in_route)),
                    (Value::from("out"), Value::from(out_route)),
                    (Value::from("message"), Value::from(message)),
                ])),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(60)).await;
}

fn console_has(lines: &Arc<Mutex<Vec<String>>>, needle: &str) -> bool {
    lines
        .lock()
        .expect("console")
        .iter()
        .any(|line| line.contains(needle))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn playground_command_grammar_and_companion() {
    let platform = boot().await;
    let po = PostOffice::new(&platform);

    // a session: the UI opens it, then a console captures the .out route
    let lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let in_route = "ws.100001.1.in";
    let out_route = "ws.100001.1.out";
    let console_lines = lines.clone();
    platform
        .register(
            out_route,
            Arc::new(Console {
                lines: console_lines,
            }),
            1,
        )
        .expect("console route");
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(knowledge_graph::commands::ROUTE)
                .set_raw_body(Value::Map(vec![
                    (Value::from("type"), Value::from("open")),
                    (Value::from("in"), Value::from(in_route)),
                ])),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(60)).await;

    // --- help: served from the ported help/*.md resources
    command(&po, in_route, out_route, "help connect").await;
    assert!(console_has(&lines, "connect"), "help connect expected");

    // --- describe skill: resolves help graph-math.md
    command(&po, in_route, out_route, "describe skill graph.math").await;
    assert!(
        console_has(&lines, "Graph Math"),
        "graph.math help expected"
    );

    // --- build a graph: root, end, a mapper, and connections
    command(&po, in_route, out_route, "create node root").await;
    assert!(console_has(&lines, "node root created"));
    command(&po, in_route, out_route, "create node end").await;
    assert!(console_has(&lines, "node end created"));
    command(
        &po,
        in_route,
        out_route,
        "create node mapper\nwith type mapper\nwith properties\nskill=graph.data.mapper\nmapping[]=input.body.id -> output.body",
    )
    .await;
    assert!(console_has(&lines, "node mapper created"));
    command(
        &po,
        in_route,
        out_route,
        "connect root to mapper with first",
    )
    .await;
    assert!(console_has(&lines, "root connected to mapper"));
    command(
        &po,
        in_route,
        out_route,
        "connect mapper to end with second",
    )
    .await;
    assert!(console_has(&lines, "mapper connected to end"));

    // --- list nodes / connections
    lines.lock().expect("console").clear();
    command(&po, in_route, out_route, "list nodes").await;
    assert!(console_has(&lines, "mapper"), "list nodes expected");
    command(&po, in_route, out_route, "list connections").await;
    assert!(
        console_has(&lines, "root -[first]-> mapper"),
        "list connections expected"
    );

    // --- instantiate with mock data, then run the traveler
    lines.lock().expect("console").clear();
    command(
        &po,
        in_route,
        out_route,
        "instantiate graph\ntext(hello world) -> input.body.id",
    )
    .await;
    assert!(
        console_has(&lines, "Graph instance created"),
        "instantiate expected"
    );
    command(&po, in_route, out_route, "run").await;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(
        console_has(&lines, "Graph traversal completed"),
        "run expected"
    );

    // --- inspect the state machine: the mapper wrote input.body.id into
    // output.body — the graph's result namespace (what execution_complete
    // returns as the response), so `output` carries "hello world"
    lines.lock().expect("console").clear();
    command(&po, in_route, out_route, "inspect output").await;
    assert!(
        console_has(&lines, "hello world"),
        "inspect output expected"
    );

    // --- export the draft; describe graph writes the temp file
    command(&po, in_route, out_route, "export graph as playtest").await;
    assert!(console_has(&lines, "Graph exported"), "export expected");

    // --- the AI-companion REST hop: POST a command, output streams to the
    // same session console (the field use case)
    lines.lock().expect("console").clear();
    let public_id = "ws-100001-1";
    assert!(
        knowledge_graph::commands::has_session(public_id),
        "session should be discoverable by public id"
    );
    let request = platform_core::automation::AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(&base_url())
        .set_url(&format!("/api/companion/{public_id}"))
        .set_header("content-type", "text/plain")
        .set_body(Value::from("list nodes"));
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(request.to_value()),
            Duration::from_secs(5),
        )
        .await
        .expect("companion request");
    let body = MultiLevelMap::from_value(reply.body().clone());
    assert_eq!(Some(Value::from("accepted")), body.get_element("status"));
    tokio::time::sleep(Duration::from_millis(120)).await;
    assert!(
        console_has(&lines, "mapper"),
        "companion command output should reach the console"
    );

    // --- the live-graph REST download returns the session's draft
    let get = platform_core::automation::AsyncHttpRequest::new()
        .set_method("GET")
        .set_target_host(&base_url())
        .set_url(&format!("/api/graph/session/{public_id}"));
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(get.to_value()),
            Duration::from_secs(5),
        )
        .await
        .expect("live graph request");
    let graph = MultiLevelMap::from_value(reply.body().clone());
    assert!(graph.get_element("nodes").is_some(), "live graph has nodes");

    // --- the inspect REST endpoint resolves a COMPOSITE key through the state
    // machine and wraps the result in {inspect, outcome} (the same shape as the
    // `inspect {key}` console command), so a scalar value serializes as clean
    // JSON instead of 404ing (the AI-companion read-back path)
    let inspect = platform_core::automation::AsyncHttpRequest::new()
        .set_method("GET")
        .set_target_host(&base_url())
        .set_url(&format!("/api/inspect/{public_id}/output.body"));
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(inspect.to_value()),
            Duration::from_secs(5),
        )
        .await
        .expect("inspect request");
    let inspected = MultiLevelMap::from_value(reply.body().clone());
    assert_eq!(
        Some(Value::from("output.body")),
        inspected.get_element("inspect"),
        "inspect endpoint echoes the composite key"
    );
    assert_eq!(
        Some(Value::from("hello world")),
        inspected.get_element("outcome"),
        "inspect endpoint resolves the composite scalar key and wraps it"
    );

    // --- close the session
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(knowledge_graph::commands::ROUTE)
                .set_raw_body(Value::Map(vec![
                    (Value::from("type"), Value::from("close")),
                    (Value::from("in"), Value::from(in_route)),
                ])),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(60)).await;
    assert!(
        !knowledge_graph::commands::has_session(public_id),
        "session cleared on close"
    );
}
