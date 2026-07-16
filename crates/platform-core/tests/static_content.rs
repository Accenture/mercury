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

//! End-to-end tests for increment 8 — the static-content protocol: the etag /
//! HTTP-304 cycle, no-cache pages, and the request filter (the SSO-redirection
//! hook). Server-per-test.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};

use async_trait::async_trait;
use platform_core::{
    automation, overrides, resources, AppConfigReader, AppError, ComposableFunction, EventEnvelope,
    Platform,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const REST_YAML: &str = r#"
rest:
  - service: "noop.static.demo"
    methods: ['GET']
    url: "/api/noop"
static-content:
  no-cache-pages: ["/", "/index.html"]
  filter:
    path: ["/", "/assets/*", "*.html", "*.js"]
    exclusion: ["*.css"]
    service: "http.request.filter"
"#;

/// The filter: records inspected paths; redirects when `?redirect=true`;
/// stamps a response header otherwise (200 = continue serving).
struct RequestFilter {
    seen: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl ComposableFunction for RequestFilter {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let request: serde_json::Value = input.body_as()?;
        let url = request["url"].as_str().unwrap_or("?").to_string();
        self.seen.lock().expect("filter mutex").push(url);
        if request["parameters"]["query"]["redirect"] == "true" {
            // the SSO-style use case: redirect instead of serving
            return Ok(EventEnvelope::new()
                .set_status(302)
                .set_header("Location", "https://sso.example.com/login")
                .set_body("redirecting")?);
        }
        // 200 = continue serving; the header rides onto the HTTP response
        EventEnvelope::new()
            .set_header("x-filter", "inspected")
            .set_body(serde_json::Value::Null)
    }
}

struct Noop;

#[async_trait]
impl ComposableFunction for Noop {
    async fn handle_event(
        &self,
        _h: HashMap<String, String>,
        _i: EventEnvelope,
        _n: usize,
    ) -> Result<EventEnvelope, AppError> {
        EventEnvelope::new().set_body("ok")
    }
}

fn setup_config() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding =
            std::env::temp_dir().join(format!("mercury-static-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        let rest_file =
            std::env::temp_dir().join(format!("rest-static-{}.yaml", std::process::id()));
        std::fs::write(&rest_file, REST_YAML).unwrap();
        overrides::set(
            "yaml.rest.automation",
            &format!("file:{}", rest_file.display()),
        );
        overrides::set("rest.server.port", "0");
        let _ = AppConfigReader::get_instance();
    });
}

async fn server(with_filter: bool) -> (u16, Arc<Mutex<Vec<String>>>) {
    setup_config();
    let platform = Platform::new();
    platform
        .register("noop.static.demo", Arc::new(Noop), 1)
        .unwrap();
    let seen = Arc::new(Mutex::new(Vec::new()));
    if with_filter {
        platform
            .register(
                "http.request.filter",
                Arc::new(RequestFilter { seen: seen.clone() }),
                1,
            )
            .unwrap();
    }
    let addr = automation::start_http_server(&platform).await.unwrap();
    (addr.port(), seen)
}

async fn http_get(
    port: u16,
    path: &str,
    extra_headers: &[(&str, &str)],
) -> (u16, HashMap<String, String>, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let mut request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\n");
    for (name, value) in extra_headers {
        request.push_str(&format!("{name}: {value}\r\n"));
    }
    request.push_str("Connection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await.expect("write");
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).await.expect("read");
    let text = String::from_utf8_lossy(&raw).to_string();
    let (head, payload) = text.split_once("\r\n\r\n").unwrap_or((text.as_str(), ""));
    let mut lines = head.lines();
    let status: u16 = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or_else(|| panic!("no status in: {text:?}"));
    let mut headers = HashMap::new();
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_lowercase(), value.trim().to_string());
        }
    }
    (status, headers, payload.to_string())
}

// ---- etag / 304 cycle ----

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn etag_cycle_returns_304_on_matching_if_none_match() {
    let (port, _) = server(false).await;
    // first fetch: 200 + a quoted SHA-256 ETag (css is excluded from no-cache)
    let (status, headers, body) = http_get(port, "/assets/style.css", &[]).await;
    assert_eq!(status, 200);
    let etag = headers.get("etag").expect("ETag header").clone();
    assert!(etag.starts_with('"') && etag.ends_with('"') && etag.len() == 66);
    assert!(body.contains("static-css-marker"));
    // revalidation: 304, empty body
    let (status, _, body) = http_get(port, "/assets/style.css", &[("If-None-Match", &etag)]).await;
    assert_eq!(status, 304);
    assert!(body.is_empty());
    // comma-separated If-None-Match list is honored (Java sameTag)
    let list = format!("\"nope\", {etag}");
    let (status, _, _) = http_get(port, "/assets/style.css", &[("If-None-Match", &list)]).await;
    assert_eq!(status, 304);
    // a stale tag serves the full file again
    let (status, _, body) =
        http_get(port, "/assets/style.css", &[("If-None-Match", "\"stale\"")]).await;
    assert_eq!(status, 200);
    assert!(!body.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn no_cache_pages_get_no_store_headers_and_no_etag() {
    let (port, _) = server(false).await;
    let (status, headers, body) = http_get(port, "/", &[]).await;
    assert_eq!(status, 200);
    assert_eq!(headers["cache-control"], "no-cache, no-store");
    assert_eq!(headers["pragma"], "no-cache");
    assert!(headers.contains_key("expires"));
    assert!(!headers.contains_key("etag"));
    assert!(body.contains("static-index-marker"));
    // If-None-Match is irrelevant on a no-cache page — always 200
    let (status, _, _) = http_get(port, "/index.html", &[("If-None-Match", "\"anything\"")]).await;
    assert_eq!(status, 200);
}

// ---- the request filter (SSO hook) ----

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn filter_inspects_matching_paths_and_stamps_headers() {
    let (port, seen) = server(true).await;
    let (status, headers, body) = http_get(port, "/index.html", &[]).await;
    assert_eq!(status, 200);
    assert!(body.contains("static-index-marker"));
    // the filter's response header rides onto the HTTP response
    assert_eq!(headers["x-filter"], "inspected");
    assert_eq!(
        seen.lock().unwrap().clone(),
        vec!["/index.html".to_string()]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn filter_redirect_passes_through_without_serving() {
    let (port, seen) = server(true).await;
    let (status, headers, body) = http_get(port, "/index.html?redirect=true", &[]).await;
    assert_eq!(status, 302);
    assert_eq!(headers["location"], "https://sso.example.com/login");
    assert!(!body.contains("static-index-marker")); // static file NOT served
    assert_eq!(seen.lock().unwrap().len(), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn excluded_extensions_bypass_the_filter() {
    let (port, seen) = server(true).await;
    // *.css is excluded even though /assets/* matches
    let (status, headers, _) = http_get(port, "/assets/style.css", &[]).await;
    assert_eq!(status, 200);
    assert!(!headers.contains_key("x-filter"));
    assert!(seen.lock().unwrap().is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn missing_filter_service_serves_without_filtering() {
    // filter configured in rest.yaml but the service is not registered:
    // warn + serve normally (Java parity)
    let (port, _) = server(false).await;
    let (status, headers, body) = http_get(port, "/index.html", &[]).await;
    assert_eq!(status, 200);
    assert!(body.contains("static-index-marker"));
    assert!(!headers.contains_key("x-filter"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn extensionless_path_assumes_html() {
    let (port, _) = server(false).await;
    // /about -> public/about.html (Java getStaticFile rule)
    let (status, headers, body) = http_get(port, "/about", &[]).await;
    assert_eq!(status, 200);
    assert_eq!(headers["content-type"], "text/html");
    assert!(body.contains("about-page-marker"));
}
