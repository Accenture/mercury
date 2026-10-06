//! The graph-set endpoints (ADR-0027 WP4), through the real HTTP edge, in lock-step with the Java
//! `GraphSetEndpointTest`: `POST /api/graph/pack` answers the set as a download - the same bytes
//! `graph_set::pack` writes for the same graphs and fields - and refuses a graph that the import validation or
//! the deployment gate refuses; `POST /api/graph/unpack` answers the manifest and the graphs of a package and
//! refuses bytes that are not a canonical package or a set that breaks a rule.

use std::collections::BTreeMap;
use std::sync::Once;
use std::time::Duration;

use async_trait::async_trait;
use event_script::conversions::{display, from_json};
use knowledge_graph::graph_set;
use platform_core::automation::AsyncHttpRequest;
use platform_core::canonical_packager::Builder;
use platform_core::{
    main_application, overrides, AppError, AutoStart, EntryPoint, EventEnvelope, Platform,
    PostOffice,
};
use rmpv::Value;

const JSON: &str = "application/json";
const OCTET_STREAM: &str = "application/octet-stream";
const PACK: &str = "/api/graph/pack";
const UNPACK: &str = "/api/graph/unpack";
const SET: &str = "unit-test-endpoint-set";
const GRAPH_A: &str = "unit-test-endpoint-a";

#[main_application]
struct GraphSetEndpointTestApp;

#[async_trait]
impl EntryPoint for GraphSetEndpointTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "graph-set endpoint test app started; graphs compiled: {}",
            knowledge_graph::graphs::get_all_graphs().len()
        );
        Ok(())
    }
}

async fn boot() -> Platform {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let dir = test_support::temp_path("graph-set-endpoints");
        std::fs::create_dir_all(&dir).expect("temp dir");
        overrides::set("location.graph.temp", &format!("file:{}", dir.display()));
        overrides::set("app.env", "dev");
        overrides::set("rest.server.port", "0");
    });
    platform_core::resources::prepend_resource_root("tests/resources");
    AutoStart::main(vec![]).await.expect("lifecycle");
    Platform::get_instance()
}

fn base_url() -> String {
    let addr = platform_core::automation::server_address().expect("server bound");
    format!("http://127.0.0.1:{}", addr.port())
}

fn graph(id: &str) -> Value {
    from_json(&serde_json::json!({
        "nodes": [
            {"alias": "root", "types": ["Root"],
             "properties": {"purpose": "a graph packed by the endpoint", "name": id}},
            {"alias": "end", "types": ["End"],
             "properties": {"skill": "graph.data.mapper", "mapping": ["text(packed) -> output.body"]}}],
        "connections": [{"source": "root", "target": "end", "relations": [{"type": "done", "properties": {}}]}]
    }))
}

fn graph_without_end(id: &str) -> Value {
    from_json(&serde_json::json!({
        "nodes": [{"alias": "root", "types": ["Root"], "properties": {"purpose": "no end node", "name": id}}]
    }))
}

fn map(pairs: Vec<(&str, Value)>) -> Value {
    Value::Map(
        pairs
            .into_iter()
            .map(|(k, v)| (Value::from(k), v))
            .collect(),
    )
}

fn manifest(set_name: &str, fields: &[(&str, &str)]) -> Value {
    let mut pairs = vec![("set", Value::from(set_name))];
    pairs.extend(fields.iter().map(|(k, v)| (*k, Value::from(*v))));
    map(pairs)
}

fn field<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    match value {
        Value::Map(entries) => entries
            .iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .map(|(_, v)| v),
        _ => None,
    }
}

fn message(reply: &EventEnvelope) -> String {
    field(reply.body(), "message")
        .map(display)
        .unwrap_or_else(|| display(reply.body()))
}

fn header(reply: &EventEnvelope, name: &str) -> String {
    reply
        .headers()
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.clone())
        .unwrap_or_default()
}

async fn post(po: &PostOffice, url: &str, content_type: &str, body: Value) -> EventEnvelope {
    let request = AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(&base_url())
        .set_url(url)
        .set_header("content-type", content_type)
        .set_header("accept", JSON)
        .set_body(body);
    po.request(
        EventEnvelope::new()
            .set_to("async.http.request")
            .set_raw_body(request.to_value()),
        Duration::from_secs(10),
    )
    .await
    .expect("http request")
}

/// One runtime for every step: a second `#[tokio::test]` gets its own runtime, which would drop the platform
/// the first one started.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn graph_set_endpoints_end_to_end() {
    test_support::run_at_exit(platform_core::util::elastic_queue::shutdown_cleanup);
    let platform = boot().await;
    let po = PostOffice::new(&platform);
    pack_answers_the_set_as_a_download(&po).await;
    pack_refuses_a_graph_the_gate_refuses(&po).await;
    pack_refuses_a_model_with_a_foreign_section(&po).await;
    pack_needs_the_set_name(&po).await;
    unpack_answers_the_manifest_and_the_graphs(&po).await;
    unpack_refuses_what_is_not_a_graph_set(&po).await;
}

async fn pack_answers_the_set_as_a_download(po: &PostOffice) {
    let fields = [("version", "1.0.0"), ("author", "the endpoint test")];
    let body = map(vec![
        ("manifest", manifest(SET, &fields)),
        ("graphs", map(vec![(GRAPH_A, graph(GRAPH_A))])),
    ]);
    let reply = post(po, PACK, JSON, body).await;
    assert_eq!(200, reply.status(), "{}", message(&reply));
    assert!(
        header(&reply, "content-type").starts_with(OCTET_STREAM),
        "{:?}",
        reply.headers()
    );
    assert_eq!(
        format!("attachment; filename=\"{SET}{}\"", graph_set::EXTENSION),
        header(&reply, "content-disposition")
    );
    let Value::Binary(bytes) = reply.body() else {
        panic!("the set is the body: {:?}", reply.body())
    };
    // the same bytes the graph packager writes for the same graphs and fields
    let owned: Vec<(String, String)> = fields
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    let graphs: BTreeMap<String, Value> = BTreeMap::from([(GRAPH_A.to_string(), graph(GRAPH_A))]);
    assert_eq!(graph_set::pack(SET, &owned, &graphs).expect("pack"), *bytes);
    let contents = graph_set::read(bytes).expect("read");
    assert_eq!(Some(SET), contents.manifest_field(graph_set::SET));
    assert_eq!(Some("1.0.0"), contents.manifest_field("version"));
    let ids: Vec<&str> = contents.graphs.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(vec![GRAPH_A], ids);
}

async fn pack_refuses_a_graph_the_gate_refuses(po: &PostOffice) {
    let graphs = map(vec![
        (GRAPH_A, graph(GRAPH_A)),
        (
            "unit-test-endpoint-no-end",
            graph_without_end("unit-test-endpoint-no-end"),
        ),
    ]);
    let body = map(vec![("manifest", manifest(SET, &[])), ("graphs", graphs)]);
    let reply = post(po, PACK, JSON, body).await;
    assert_eq!(400, reply.status(), "{}", message(&reply));
    assert_eq!(
        "Set not packed - unit-test-endpoint-no-end: graph must have an 'end' node",
        message(&reply)
    );
}

async fn pack_refuses_a_model_with_a_foreign_section(po: &PostOffice) {
    let Value::Map(mut entries) = graph("unit-test-endpoint-b") else {
        unreachable!("a graph model is a map")
    };
    entries.push((Value::from("manifest"), Value::Map(vec![])));
    let graphs = map(vec![("unit-test-endpoint-b", Value::Map(entries))]);
    let body = map(vec![("manifest", manifest(SET, &[])), ("graphs", graphs)]);
    let reply = post(po, PACK, JSON, body).await;
    assert_eq!(400, reply.status(), "{}", message(&reply));
    assert_eq!(
        "Set not packed - unit-test-endpoint-b: Unexpected top-level section(s): manifest - a graph model has only 'nodes' and 'connections'",
        message(&reply)
    );
}

async fn pack_needs_the_set_name(po: &PostOffice) {
    let body = map(vec![
        ("manifest", map(vec![("version", Value::from("1"))])),
        ("graphs", map(vec![(GRAPH_A, graph(GRAPH_A))])),
    ]);
    let reply = post(po, PACK, JSON, body).await;
    assert_eq!(400, reply.status(), "{}", message(&reply));
    assert_eq!(
        "Set not packed - manifest field 'set' is required - it names the set and its file",
        message(&reply)
    );
    let no_graphs = post(po, PACK, JSON, map(vec![("manifest", manifest(SET, &[]))])).await;
    assert_eq!(400, no_graphs.status(), "{}", message(&no_graphs));
    assert_eq!(
        "Set not packed - 'graphs' is a JSON object keyed by graph id, each value a graph model",
        message(&no_graphs)
    );
}

async fn unpack_answers_the_manifest_and_the_graphs(po: &PostOffice) {
    let fields = vec![
        ("version".to_string(), "2.0.0".to_string()),
        (graph_set::GRAPH_ID.to_string(), GRAPH_A.to_string()),
    ];
    let graphs: BTreeMap<String, Value> = BTreeMap::from([(GRAPH_A.to_string(), graph(GRAPH_A))]);
    let bytes = graph_set::pack(SET, &fields, &graphs).expect("pack");
    let reply = post(po, UNPACK, OCTET_STREAM, Value::Binary(bytes)).await;
    assert_eq!(200, reply.status(), "{}", message(&reply));
    assert!(
        header(&reply, "content-type").starts_with(JSON),
        "{:?}",
        reply.headers()
    );
    let manifest = field(reply.body(), "manifest").expect("manifest");
    assert_eq!(
        Some(SET),
        field(manifest, graph_set::SET).and_then(Value::as_str)
    );
    assert_eq!(
        Some("2.0.0"),
        field(manifest, "version").and_then(Value::as_str)
    );
    assert_eq!(
        Some(GRAPH_A),
        field(manifest, graph_set::GRAPH_ID).and_then(Value::as_str)
    );
    assert_eq!(
        Some("mercury-package"),
        field(manifest, "format").and_then(Value::as_str)
    );
    let graphs = field(reply.body(), "graphs").expect("graphs");
    let Value::Map(entries) = graphs else {
        panic!("graphs is a map: {graphs:?}")
    };
    let ids: Vec<&str> = entries.iter().filter_map(|(k, _)| k.as_str()).collect();
    assert_eq!(vec![GRAPH_A], ids);
    let nodes = field(&entries[0].1, "nodes").expect("nodes");
    assert_eq!(2, nodes.as_array().map(Vec::len).unwrap_or(0));
}

async fn unpack_refuses_what_is_not_a_graph_set(po: &PostOffice) {
    let text = post(
        po,
        UNPACK,
        OCTET_STREAM,
        Value::Binary(b"not a package".to_vec()),
    )
    .await;
    assert_eq!(400, text.status(), "{}", message(&text));
    assert!(
        message(&text).starts_with("Not a graph set - "),
        "{}",
        message(&text)
    );
    // a canonical package whose entry is not <graph-id>.json is refused before anything is built from its name
    let crafted = Builder::new()
        .manifest(graph_set::SET, SET)
        .expect("manifest")
        .add("../escape.json", graph("escape"))
        .expect("entry")
        .build()
        .expect("build");
    let entry = post(po, UNPACK, OCTET_STREAM, Value::Binary(crafted)).await;
    assert_eq!(400, entry.status(), "{}", message(&entry));
    assert_eq!(
        "Not a graph set - entry '../escape.json' - expect <graph-id>.json, the id in letters, digits, '_' and '-'",
        message(&entry)
    );
    // the body is the package itself, not a JSON document
    let json = post(po, UNPACK, JSON, map(vec![("manifest", map(vec![]))])).await;
    assert_eq!(400, json.status(), "{}", message(&json));
    assert_eq!(
        "The request body is the graph set (.pack) to read, sent as application/octet-stream",
        message(&json)
    );
}
