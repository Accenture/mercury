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

//! The example end to end over the real HTTP stack against the in-process
//! RESP double — the Rust twin of the Java `ProfileCacheTest`: the same CRUD
//! cycle on Layer 1 and Layer 2, the Layer 3 graph through the standard graph
//! endpoint (with its closed dispatch table rejecting an unknown or absent
//! action), and the cross-layer interop that is the whole point: a profile
//! written on one layer reads back on the other two, because every layer
//! packs the same plain MsgPack map under the same key. One booted server,
//! sequential scenarios (repo convention).

use redis_test_double as common;

use platform_core::{automation, overrides, AutoStart};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// the application's functions and entry point ride into this test binary
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

/// Minimal raw HTTP/1.1 call (no client dependency): status + body text.
async fn http(port: u16, method: &str, path: &str, body: Option<&str>) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let payload = body.unwrap_or("");
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: localhost\r\ncontent-type: application/json\r\n\
         accept: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
        payload.len()
    );
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).await.expect("read");
    let text = String::from_utf8_lossy(&raw).to_string();
    let (head, payload) = text.split_once("\r\n\r\n").unwrap_or((text.as_str(), ""));
    let status: u16 = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or_else(|| panic!("status code missing in: {text:?}"));
    (status, payload.to_string())
}

fn json(payload: &str) -> serde_json::Value {
    serde_json::from_str(payload).unwrap_or_else(|e| panic!("not json ({e}): {payload}"))
}

/// The Java `crudCycle`: miss → create → read → delete → miss, on one layer's
/// route family.
async fn crud_cycle(port: u16, base: &str, layer: i64, id: &str) {
    let url = format!("{base}/{id}");
    let (status, body) = http(port, "GET", &url, None).await;
    assert_eq!(404, status, "a fresh id is a miss: {body}");
    assert_eq!(
        "Profile not found",
        json(&body)["message"],
        "layer {layer} miss body: {body}"
    );
    let (status, body) = http(
        port,
        "POST",
        &url,
        Some(r#"{"name": "Carol", "email": "carol@example.com"}"#),
    )
    .await;
    assert_eq!(201, status, "layer {layer} create: {body}");
    assert_eq!(
        layer,
        json(&body)["layer"],
        "the layer that served the write"
    );
    let (status, body) = http(port, "GET", &url, None).await;
    assert_eq!(200, status, "layer {layer} read: {body}");
    let profile = json(&body);
    assert_eq!("Carol", profile["name"]);
    assert_eq!("carol@example.com", profile["email"]);
    let (status, body) = http(port, "DELETE", &url, None).await;
    assert_eq!(200, status, "layer {layer} delete: {body}");
    // Layer 1 reports a boolean it derived in code; Layer 2 maps the cache's
    // DELETE count straight through - both say "one profile removed"
    let deleted = json(&body)["deleted"].clone();
    assert!(
        deleted == serde_json::json!(true) || deleted == serde_json::json!(1),
        "delete reports what it removed: {body}"
    );
    let (status, _) = http(port, "GET", &url, None).await;
    assert_eq!(404, status, "deleted = a miss again");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn profile_crud_on_all_three_layers_over_one_cache() {
    let (redis_port, _raw_store, _journal) = common::start_resp_double("7.4.1").await;
    overrides::set("redis.host", "127.0.0.1");
    overrides::set("redis.port", &redis_port.to_string());
    overrides::set("rest.server.port", "0"); // an ephemeral port for the test
                                             // boot the whole application: cache functions (redis.cache.enabled=true in
                                             // application.yml), the flow compiler, the CompileGraph gate, REST
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let port = automation::server_address().expect("server started").port();
    assert!(
        knowledge_graph::graphs::graph_exists("profile-cache"),
        "the Layer 3 model must pass the CompileGraph gate"
    );

    // Layer 1 (Java layer1Crud) and Layer 2 (Java layer2Crud): the same cycle
    crud_cycle(port, "/api/l1/profile", 1, "carol-l1").await;
    crud_cycle(port, "/api/l2/profile", 2, "carol-l2").await;

    // Layer 3 (Java layer3GraphCrud): one endpoint for every graph, the action
    // in the payload
    let graph = "/api/graph/profile-cache";
    let (status, body) = http(
        port,
        "POST",
        graph,
        Some(r#"{"action": "save", "id": "erin", "profile": {"name": "Erin", "email": "erin@example.com"}}"#),
    )
    .await;
    assert_eq!(200, status, "graph save: {body}");
    assert_eq!(3, json(&body)["layer"]);
    assert_eq!("stored", json(&body)["status"]);
    let (status, body) = http(
        port,
        "POST",
        graph,
        Some(r#"{"action": "get", "id": "erin"}"#),
    )
    .await;
    assert_eq!(200, status, "graph get: {body}");
    assert_eq!("Erin", json(&body)["name"]);
    assert_eq!("erin@example.com", json(&body)["email"]);
    let (status, body) = http(
        port,
        "POST",
        graph,
        Some(r#"{"action": "delete", "id": "erin"}"#),
    )
    .await;
    assert_eq!(200, status, "graph delete: {body}");
    assert_eq!(
        1,
        json(&body)["deleted"],
        "the DELETE count rides back: {body}"
    );
    let (status, body) = http(
        port,
        "POST",
        graph,
        Some(r#"{"action": "get", "id": "erin"}"#),
    )
    .await;
    assert_eq!(404, status, "a deleted profile is a miss: {body}");
    assert_eq!("Profile not found", json(&body)["message"]);

    // Layer 3 rejects an unknown or absent action (Java layer3RejectsAnUnknownAction):
    // the decision node is a CLOSED dispatch table, so a typo answers 400
    // instead of falling through to a branch
    let (status, body) = http(
        port,
        "POST",
        graph,
        Some(r#"{"action": "save", "id": "frank", "profile": {"name": "Frank"}}"#),
    )
    .await;
    assert_eq!(200, status, "{body}");
    let (status, body) = http(
        port,
        "POST",
        graph,
        Some(r#"{"action": "purge", "id": "frank"}"#),
    )
    .await;
    assert_eq!(400, status, "an unknown action is rejected: {body}");
    assert_eq!(
        "Invalid action. Use get, save or delete",
        json(&body)["message"]
    );
    let (status, body) = http(port, "POST", graph, Some(r#"{"id": "frank"}"#)).await;
    assert_eq!(400, status, "an absent action is rejected too: {body}");
    let (status, body) = http(
        port,
        "POST",
        graph,
        Some(r#"{"action": "get", "id": "frank"}"#),
    )
    .await;
    assert_eq!(200, status, "the rejected calls touched nothing: {body}");
    assert_eq!("Frank", json(&body)["name"]);
    let (status, _) = http(
        port,
        "POST",
        graph,
        Some(r#"{"action": "delete", "id": "frank"}"#),
    )
    .await;
    assert_eq!(200, status);

    // cross-layer interop (Java crossLayerInterop): written on Layer 1, read on
    // Layers 2 and 3 - one key, one plain-MsgPack value, three surfaces
    let (status, body) = http(
        port,
        "POST",
        "/api/l1/profile/dave",
        Some(r#"{"name": "Dave", "email": "dave@example.com"}"#),
    )
    .await;
    assert_eq!(201, status, "{body}");
    let (status, body) = http(port, "GET", "/api/l2/profile/dave", None).await;
    assert_eq!(200, status, "read on Layer 2: {body}");
    assert_eq!("Dave", json(&body)["name"]);
    assert_eq!("dave@example.com", json(&body)["email"]);
    let (status, body) = http(
        port,
        "POST",
        graph,
        Some(r#"{"action": "get", "id": "dave"}"#),
    )
    .await;
    assert_eq!(200, status, "read on Layer 3: {body}");
    assert_eq!("Dave", json(&body)["name"]);
    let (status, body) = http(port, "DELETE", "/api/l2/profile/dave", None).await;
    assert_eq!(200, status, "delete on Layer 2: {body}");
    let (status, _) = http(port, "GET", "/api/l1/profile/dave", None).await;
    assert_eq!(404, status, "gone on Layer 1 too");

    // the value format is the interop contract: what Layer 1 stored is a plain
    // MsgPack map (no envelope wrapper, no type tags) under the namespaced key
    let (status, _) = http(
        port,
        "POST",
        "/api/l1/profile/grace",
        Some(r#"{"name": "Grace", "level": 7}"#),
    )
    .await;
    assert_eq!(201, status);
    let stored = {
        let store = _raw_store.lock().expect("raw store");
        store
            .get(&b"cache-demo:grace".to_vec())
            .and_then(|entry| entry.text().map(|bytes| bytes.to_vec()))
            .expect("stored under the cache-demo: namespace")
    };
    let value = rmpv::decode::read_value(&mut &stored[..]).expect("plain msgpack");
    let rmpv::Value::Map(entries) = value else {
        panic!("a profile is a MsgPack map, got {value:?}");
    };
    assert!(entries
        .iter()
        .any(|(k, v)| k.as_str() == Some("name") && v.as_str() == Some("Grace")));
    assert!(entries
        .iter()
        .any(|(k, v)| k.as_str() == Some("level") && v.as_i64() == Some(7)));
}
