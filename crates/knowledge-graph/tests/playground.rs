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
    main_application, overrides, preload, AppError, AutoStart, ComposableFunction, EventEnvelope,
    Platform, PostOffice,
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

/// A deliberately slow composable function for the dry-run watcher scenarios:
/// it outlives the run-level `model.ttl` deadline, so the traversal can only
/// end through the watcher's terminal — never through this function's reply
/// (Java `SlowTask` twin).
#[preload(route = "v1.slow.task", instances = 10)]
struct SlowTask;

#[async_trait]
impl ComposableFunction for SlowTask {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        _input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        tokio::time::sleep(Duration::from_millis(4000)).await;
        EventEnvelope::new().set_body("late")
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

    // --- discovery: deployed graph models (with root purpose) and flows —
    // the read-only surface that makes extension= delegation self-service
    command(&po, in_route, out_route, "list graphs").await;
    assert!(
        console_has(&lines, "extension={graph-id} targets"),
        "list graphs header expected"
    );
    assert!(
        console_has(&lines, "tutorial-1"),
        "deployed tutorial-1 expected in the listing"
    );
    assert!(
        console_has(&lines, "rust-join-chain"),
        "manifest-compiled fixture expected in the listing"
    );
    command(&po, in_route, out_route, "list flows").await;
    assert!(
        console_has(&lines, "extension=flow://{flow-id} targets"),
        "list flows header expected"
    );
    assert!(
        console_has(&lines, "graph-executor"),
        "the engine's own flow is a flow and must be listed"
    );
    assert!(
        console_has(
            &lines,
            "flow-11 - This event flow will echo all input parameters"
        ),
        "flow listing carries the mandatory flow.description"
    );
    assert!(
        console_has(&lines, "describe graph {graph-id}"),
        "list graphs advertises the contract command"
    );

    // --- discovery: the contract view of a deployed model (finding #53)
    command(&po, in_route, out_route, "describe graph tutorial-3").await;
    assert!(
        console_has(&lines, "Deployed graph model 'tutorial-3'"),
        "deployed-model header expected"
    );
    // exact indented lines: a stray trailing character (e.g. the Java
    // engine's `output.body]` toString leak, 2026-07-20) must fail here
    assert!(
        console_has(&lines, "  input.body.person_id\n"),
        "derived input surface expected"
    );
    assert!(
        console_has(&lines, "  output.body.name\n"),
        "derived output surface expected"
    );
    command(&po, in_route, out_route, "describe graph no-such-model-xyz").await;
    assert!(
        console_has(&lines, "Graph model 'no-such-model-xyz' not found"),
        "unknown model reported not found"
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

    // --- the retired fire-and-forget companion URL answers 404 (retired
    // 2026-09-02, lock-step with the Java engine): only /sync exists
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
    assert_eq!(
        404,
        reply.status(),
        "the retired async companion URL must answer 404: {:?}",
        reply.body()
    );

    // --- the SYNCHRONOUS companion hop over REST: POST /api/companion/{id}/sync
    // returns the command outcome in-band. `run` is asynchronous (the traveler
    // streams its output after the handler replies), so this is the regression
    // guard for the drain: the response must carry the WHOLE traversal, drained
    // on the traveler's terminal line — not a sentinel-raced truncation.
    lines.lock().expect("console").clear();
    let sync_run = platform_core::automation::AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(&base_url())
        .set_url(&format!("/api/companion/{public_id}/sync"))
        .set_header("content-type", "text/plain")
        .set_body(Value::from("run"));
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(sync_run.to_value()),
            Duration::from_secs(35),
        )
        .await
        .expect("sync run request");
    let outcome = MultiLevelMap::from_value(reply.body().clone());
    assert_eq!(
        Some(Value::Boolean(true)),
        outcome.get_element("ok"),
        "sync run → ok:true: {:?}",
        reply.body()
    );
    // The output array must span the full async tail, ending on the terminal.
    let output_lines: Vec<String> = match outcome.get_element("output") {
        Some(Value::Array(items)) => items.iter().map(display).collect(),
        other => panic!("sync run output must be an array, got {other:?}"),
    };
    assert!(
        output_lines.iter().any(|l| l == "Walk to root"),
        "sync run captures the traversal start: {output_lines:?}"
    );
    assert!(
        output_lines
            .iter()
            .any(|l| l.starts_with("Executed mapper with skill graph.data.mapper")),
        "sync run captures mid-traversal skill execution: {output_lines:?}"
    );
    assert!(
        output_lines
            .iter()
            .any(|l| l.starts_with("Graph traversal completed in")),
        "sync run must capture the traversal terminal (drain waited for it): {output_lines:?}"
    );
    // The structured result carries the run's output.body ("hello world").
    let result_json = event_script::conversions::to_json_string(
        &outcome.get_element("result").unwrap_or(Value::Nil),
    );
    assert!(
        result_json.contains("hello world"),
        "sync run returns the output.body as structured result: {result_json}"
    );
    // On success `error` is Nil; the JSON edge omits it by default
    // (serializer.null.transport=false) — the Java Gson parity the field reported.
    assert!(
        outcome.get_element("error").is_none(),
        "the null `error` field is omitted from the JSON response by default: {:?}",
        reply.body()
    );
    // The same output is teed live to the session console (human co-view).
    assert!(
        console_has(&lines, "Graph traversal completed"),
        "sync run output is teed to the session console"
    );

    // A failing traversal still returns promptly with the uniform terminal,
    // proving the drain never hangs to the safety timeout on an early failure.
    // Use a fresh session with no instantiated graph (a realistic companion
    // mistake: `run` before `instantiate`) so the main session is untouched.
    let bad_in = "ws.100009.1.in";
    let bad_id = "ws-100009-1";
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(knowledge_graph::commands::ROUTE)
                .set_raw_body(Value::Map(vec![
                    (Value::from("type"), Value::from("open")),
                    (Value::from("in"), Value::from(bad_in)),
                ])),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(60)).await;
    let sync_bad = platform_core::automation::AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(&base_url())
        .set_url(&format!("/api/companion/{bad_id}/sync"))
        .set_header("content-type", "text/plain")
        .set_body(Value::from("run"));
    let started = std::time::Instant::now();
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(sync_bad.to_value()),
            Duration::from_secs(35),
        )
        .await
        .expect("sync bad run request");
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "a failed run must drain on the terminal, not the safety timeout"
    );
    let bad_outcome = MultiLevelMap::from_value(reply.body().clone());
    assert_eq!(
        Some(Value::Boolean(false)),
        bad_outcome.get_element("ok"),
        "run with no instance → ok:false: {:?}",
        reply.body()
    );
    let bad_lines: Vec<String> = match bad_outcome.get_element("output") {
        Some(Value::Array(items)) => items.iter().map(display).collect(),
        other => panic!("sync bad run output must be an array, got {other:?}"),
    };
    assert!(
        bad_lines.iter().any(|l| l == "Graph traversal aborted"),
        "every run ends with a terminal, even on early failure: {bad_lines:?}"
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

    // --- the dry-run watcher (Java DryRunTimeoutTest twins). A hung run:
    // the graph.task node's own child-call ttl (8s) is deliberately LONGER
    // than model.ttl (1.5s), so only the run-level watcher can end this
    // traversal - the sync drain must end on the watcher's terminal, ok:false.
    let hung_in = "ws.100002.1.in";
    let hung_out = "ws.100002.1.out";
    let hung_id = "ws-100002-1";
    let hung_lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    platform
        .register(
            hung_out,
            Arc::new(Console {
                lines: hung_lines.clone(),
            }),
            1,
        )
        .expect("hung console route");
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(knowledge_graph::commands::ROUTE)
                .set_raw_body(Value::Map(vec![
                    (Value::from("type"), Value::from("open")),
                    (Value::from("in"), Value::from(hung_in)),
                ])),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(60)).await;
    command(&po, hung_in, hung_out, "create node root\nwith type Root").await;
    command(&po, hung_in, hung_out, "create node end").await;
    command(
        &po,
        hung_in,
        hung_out,
        "create node slow\nwith type task\nwith properties\nskill=graph.task\ntask=v1.slow.task\nttl=8s",
    )
    .await;
    command(&po, hung_in, hung_out, "connect root to slow with first").await;
    command(&po, hung_in, hung_out, "connect slow to end with second").await;
    command(
        &po,
        hung_in,
        hung_out,
        "instantiate graph\nlong(1500) -> model.ttl",
    )
    .await;
    let started = std::time::Instant::now();
    let sync_hung = platform_core::automation::AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(&base_url())
        .set_url(&format!("/api/companion/{hung_id}/sync"))
        .set_header("content-type", "text/plain")
        .set_body(Value::from("run"));
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(sync_hung.to_value()),
            Duration::from_secs(35),
        )
        .await
        .expect("sync hung run request");
    assert!(
        started.elapsed() < Duration::from_secs(15),
        "the drain must end on the watcher's terminal, not the safety net"
    );
    let hung_outcome = MultiLevelMap::from_value(reply.body().clone());
    assert_eq!(
        Some(Value::Boolean(false)),
        hung_outcome.get_element("ok"),
        "a timed-out dry-run is a failure: {:?}",
        reply.body()
    );
    let hung_output: Vec<String> = match hung_outcome.get_element("output") {
        Some(Value::Array(items)) => items.iter().map(display).collect(),
        other => panic!("hung run output must be an array, got {other:?}"),
    };
    assert!(
        hung_output
            .iter()
            .any(|l| l == "Graph traversal timed out after 1500 ms"),
        "the watcher must report the model.ttl deadline: {hung_output:?}"
    );
    assert!(
        hung_output.iter().any(|l| l == "Graph traversal aborted"),
        "a timed-out run must end with the canonical failure terminal: {hung_output:?}"
    );

    // --- a completed run cancels its watcher: the run's console IS the
    // tapped route (through the sync endpoint the capture route is released
    // before a stale watcher could fire, proving nothing - the Java
    // mutation-test lesson), so a late timeout/abort line would land here.
    let fast_in = "ws.100003.1.in";
    let fast_out = "ws.100003.1.out";
    let fast_lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    platform
        .register(
            fast_out,
            Arc::new(Console {
                lines: fast_lines.clone(),
            }),
            1,
        )
        .expect("fast console route");
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(knowledge_graph::commands::ROUTE)
                .set_raw_body(Value::Map(vec![
                    (Value::from("type"), Value::from("open")),
                    (Value::from("in"), Value::from(fast_in)),
                ])),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(60)).await;
    command(&po, fast_in, fast_out, "create node root\nwith type Root").await;
    command(&po, fast_in, fast_out, "create node end").await;
    command(
        &po,
        fast_in,
        fast_out,
        "create node mapper\nwith type mapper\nwith properties\nskill=graph.data.mapper\nmapping[]=input.body.id -> output.body",
    )
    .await;
    command(&po, fast_in, fast_out, "connect root to mapper with first").await;
    command(&po, fast_in, fast_out, "connect mapper to end with second").await;
    command(
        &po,
        fast_in,
        fast_out,
        "instantiate graph\ntext(hello) -> input.body.id\nlong(3000) -> model.ttl",
    )
    .await;
    command(&po, fast_in, fast_out, "run").await;
    let mut completed = false;
    for _ in 0..100 {
        if fast_lines
            .lock()
            .expect("fast console")
            .iter()
            .any(|l| l.starts_with("Graph traversal completed in"))
        {
            completed = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(completed, "the fast run must complete on the console");
    // mechanism pin (not just the CAS masking a late line): completion must
    // have RELEASED the watcher slot - the cancel half of claim_terminal
    {
        let instance = knowledge_graph::model::get_instance(fast_in).expect("fast instance");
        assert!(
            instance.get_run_watcher().is_none(),
            "a completed run must release its watcher slot"
        );
    }
    // sleep past the 3s deadline: a canceled watcher must stay silent - a
    // stale firing would send a spurious timeout/abort line to this console
    tokio::time::sleep(Duration::from_millis(3800)).await;
    {
        let lines = fast_lines.lock().expect("fast console");
        let late: Vec<&String> = lines
            .iter()
            .filter(|l| {
                l.starts_with("Graph traversal timed out") || *l == "Graph traversal aborted"
            })
            .collect();
        assert!(
            late.is_empty(),
            "a completed run must cancel its watcher - no late terminal allowed: {late:?}"
        );
    }

    // --- tutorial-13 dry-run (Java CompanionSyncTest twin): 'instantiate graph'
    // loads the model through the config reader, so ${rest.server.port:8080} in
    // the task node's 'host' resolves to the application's actual port (success
    // proves the 8080 default was NOT used), and the graph.task input mapping
    // stages model.person_id for the {model.person_id} dynamic variable in the
    // 'url'. Deployed execution of the same model is covered in graph_runtime.rs
    // - the two lanes must behave the same.
    let t13_in = "ws.100004.1.in";
    let t13_out = "ws.100004.1.out";
    let t13_id = "ws-100004-1";
    let t13_lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    platform
        .register(
            t13_out,
            Arc::new(Console {
                lines: t13_lines.clone(),
            }),
            1,
        )
        .expect("tutorial-13 console route");
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(knowledge_graph::commands::ROUTE)
                .set_raw_body(Value::Map(vec![
                    (Value::from("type"), Value::from("open")),
                    (Value::from("in"), Value::from(t13_in)),
                ])),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(60)).await;
    // this harness boots on an ephemeral port (rest.server.port=0 override), so
    // point the SAME config key at the actual bound port: 'instantiate graph'
    // resolves ${rest.server.port:8080} against the app config chain, exactly
    // as a deployment environment injects the real value
    let bound = platform_core::automation::server_address()
        .expect("server bound")
        .port();
    overrides::set("rest.server.port", &bound.to_string());
    // the tutorial model must come from the deployed classpath copy, not a
    // stale export from an earlier manual run
    let _ = std::fs::remove_file("/tmp/graph/tutorial-13.json");
    command(&po, t13_in, t13_out, "import graph from tutorial-13").await;
    assert!(
        console_has(&t13_lines, "Graph model imported"),
        "tutorial-13 import expected"
    );
    command(
        &po,
        t13_in,
        t13_out,
        "instantiate graph\nint(100) -> input.body.person_id",
    )
    .await;
    assert!(
        console_has(&t13_lines, "Graph instance created"),
        "tutorial-13 instantiate expected"
    );
    let sync_t13 = platform_core::automation::AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(&base_url())
        .set_url(&format!("/api/companion/{t13_id}/sync"))
        .set_header("content-type", "text/plain")
        .set_body(Value::from("run"));
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(sync_t13.to_value()),
            Duration::from_secs(35),
        )
        .await
        .expect("sync tutorial-13 run request");
    let t13_outcome = MultiLevelMap::from_value(reply.body().clone());
    assert_eq!(
        Some(Value::Boolean(true)),
        t13_outcome.get_element("ok"),
        "tutorial-13 dry-run must succeed: {:?}",
        reply.body()
    );
    // the traversal's JSON payload arrives in 'result' (narration is 'output')
    let t13_result = event_script::conversions::to_json_string(
        &t13_outcome.get_element("result").unwrap_or(Value::Nil),
    );
    assert!(
        t13_result.contains("Peter") && t13_result.contains("100 World Blvd"),
        "tutorial-13 dry-run returns the fetched profile: {t13_result}"
    );
    assert!(
        t13_result.contains("5000"),
        "the mock echoes the X-TTL request header (observed_ttl): {t13_result}"
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

    // additional playground cases run sequentially on the same booted server
    // (a second `#[tokio::test]` would drop this runtime and kill the server)
    export_name_guard_accepts_missing_and_rejects_mismatch(&platform).await;
}

/// Open a playground session and return its captured console (the .out route).
async fn open_console(
    platform: &Platform,
    po: &PostOffice,
    seq: &str,
) -> (Arc<Mutex<Vec<String>>>, String, String) {
    let lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let in_route = format!("ws.{seq}.1.in");
    let out_route = format!("ws.{seq}.1.out");
    platform
        .register(
            &out_route,
            Arc::new(Console {
                lines: lines.clone(),
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
                    (Value::from("in"), Value::from(in_route.as_str())),
                ])),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(60)).await;
    (lines, in_route, out_route)
}

/// The export guard validates the root name only when one is DECLARED: a missing or
/// blank root name has no identity evidence to contradict the export target, so the
/// export adopts the target id as the name (exactly like the no-root path already
/// does). A declared, mismatching name still rejects the overwrite. (Java twin:
/// CompanionSyncTest.exportAcceptsMissingRootNameAndStillRejectsMismatch)
///
/// A plain `async fn` (not a second `#[tokio::test]`): the harness gives each
/// `#[tokio::test]` its own runtime, which drops and kills the shared HTTP server
/// task when the first test finishes, so every playground case runs sequentially
/// inside the single booted test.
async fn export_name_guard_accepts_missing_and_rejects_mismatch(platform: &Platform) {
    let po = PostOffice::new(platform);
    let file = std::env::temp_dir()
        .join(format!("mercury-playground-{}", std::process::id()))
        .join("export-guard-test.json");
    let _ = std::fs::remove_file(&file);

    // an unnamed root exports fine when the file does not exist yet
    let (lines1, in1, out1) = open_console(platform, &po, "100031").await;
    command(&po, &in1, &out1, "create node root\nwith type Root").await;
    command(&po, &in1, &out1, "export graph as export-guard-test").await;
    assert!(
        console_has(&lines1, "Graph exported"),
        "first export expected: {:?}",
        lines1.lock().expect("console")
    );
    assert!(file.exists(), "first export created the file");

    // the friction case: ANOTHER session's unnamed graph re-exports over the
    // existing file - a missing name must be accepted, not compared as "null"
    let (lines2, in2, out2) = open_console(platform, &po, "100032").await;
    command(&po, &in2, &out2, "create node root\nwith type Root").await;
    command(&po, &in2, &out2, "export graph as export-guard-test").await;
    assert!(
        console_has(&lines2, "Graph exported"),
        "an unnamed root must be accepted over an existing file: {:?}",
        lines2.lock().expect("console")
    );
    assert!(
        !console_has(&lines2, "Expect root node name="),
        "no identity rejection without a declared name"
    );

    // a DECLARED mismatching name still rejects the overwrite
    let (lines3, in3, out3) = open_console(platform, &po, "100033").await;
    command(
        &po,
        &in3,
        &out3,
        "create node root\nwith type Root\nwith properties\nname=some-other-graph",
    )
    .await;
    command(&po, &in3, &out3, "export graph as export-guard-test").await;
    assert!(
        console_has(
            &lines3,
            "Expect root node name=export-guard-test, Actual: some-other-graph"
        ),
        "a declared mismatch must still reject the overwrite: {:?}",
        lines3.lock().expect("console")
    );
    assert!(
        !console_has(&lines3, "Graph exported"),
        "the mismatching graph must not be exported"
    );
    let _ = std::fs::remove_file(&file);
}
