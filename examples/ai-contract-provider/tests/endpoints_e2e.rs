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

//! End-to-end coverage of every discovery endpoint through the real REST
//! server (Java `DiscoveryEndpointTest` twin): rest.yaml -> http.flow.adapter
//! -> Event Script flow -> function, using the byte-identical flow files the
//! Java engine ships. One test function on purpose: the app boots ONCE per
//! process (AutoStart is a run-once lifecycle).

use platform_core::{automation, overrides, AutoStart};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// The application under test is a BIN crate; include its source so the
// link-time inventory in this test binary carries the app's functions, the
// flow engine and the main application.
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

/// Minimal raw HTTP/1.1 GET (no client dependency).
async fn http_get(port: u16, path: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: localhost\r\naccept: application/json\r\nConnection: close\r\n\r\n"
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn all_discovery_endpoints_serve_the_snapshot() {
    let holding = std::env::temp_dir().join(format!("ai-contract-e2e-{}", std::process::id()));
    overrides::set("transient.data.store", &holding.display().to_string());
    // an ephemeral port keeps the test parallel-safe
    overrides::set("rest.server.port", "0");
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let port = automation::server_address().expect("server started").port();
    let version = env!("CARGO_PKG_VERSION");

    // GET /api/discovery - the front door names everything else
    let (status, body) = http_get(port, "/api/discovery").await;
    assert_eq!(status, 200, "discovery body: {body}");
    let discovery: serde_json::Value = serde_json::from_str(&body).expect("discovery json");
    assert_eq!(discovery["name"], "ai-contract-provider");
    assert_eq!(discovery["mercury_version"], version);
    let ids: Vec<&str> = discovery["contracts"]
        .as_array()
        .expect("contract ids")
        .iter()
        .map(|v| v.as_str().unwrap_or_default())
        .collect();
    assert_eq!(
        ids,
        [
            "event-script",
            "minigraph",
            "platform-core",
            "rest-automation"
        ]
    );
    assert_eq!(
        discovery["endpoints"].as_object().expect("endpoints").len(),
        5
    );

    // GET /api/contracts
    let (status, body) = http_get(port, "/api/contracts").await;
    assert_eq!(status, 200, "contract list body: {body}");
    let list: serde_json::Value = serde_json::from_str(&body).expect("list json");
    assert_eq!(list["total"], 4);
    assert_eq!(list["mercury_version"], version);

    // GET /api/contracts/{id} - present and absent
    let (status, body) = http_get(port, "/api/contracts/platform-core").await;
    assert_eq!(status, 200, "contract detail body: {body}");
    let detail: serde_json::Value = serde_json::from_str(&body).expect("detail json");
    assert_eq!(detail["id"], "platform-core");
    assert!(!detail["behavior_anchors"]
        .as_array()
        .expect("anchors")
        .is_empty());
    assert!(!detail["references"]
        .as_array()
        .expect("references")
        .is_empty());
    let (status, _) = http_get(port, "/api/contracts/no-such-contract").await;
    assert_eq!(status, 404);

    // GET /api/skill - the SKILL.md entrypoint as markdown
    let (status, body) = http_get(port, "/api/skill").await;
    assert_eq!(status, 200);
    assert!(
        body.contains("name: mercury-platform"),
        "skill body: {body}"
    );

    // GET /api/references?path=... - member, non-member
    let (status, body) = http_get(port, "/api/references?path=references/llms.txt").await;
    assert_eq!(status, 200);
    assert!(body.contains("Mercury (Rust)"), "llms body: {body}");
    let (status, _) = http_get(port, "/api/references?path=references/none.md").await;
    assert_eq!(status, 404);
    let (status, _) = http_get(port, "/api/references").await;
    assert_eq!(
        status, 404,
        "missing path parameter reads as an absent member"
    );

    // GET /api/manifest - deterministic, version-matched
    let (status, body) = http_get(port, "/api/manifest").await;
    assert_eq!(status, 200);
    let manifest: serde_json::Value = serde_json::from_str(&body).expect("manifest json");
    assert_eq!(manifest["type"], "mercury-platform-skill");
    assert_eq!(manifest["mercury_version"], version);
    let served = app::snapshot::SkillSnapshot::get_instance();
    assert_eq!(
        manifest["files"].as_array().expect("files").len(),
        served.files().len()
    );
    assert_eq!(
        manifest["snapshot_sha256"],
        served.manifest()["snapshot_sha256"]
    );
}
