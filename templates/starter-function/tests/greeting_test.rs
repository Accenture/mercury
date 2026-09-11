// Scaffolded from Mercury's starter templates (https://github.com/Accenture/mercury, Apache-2.0)

//! Boots the whole starter once and exercises the greeting end to end
//! through the REST edge: the JSON-body path, the query-parameter path,
//! and the validation failure.

use platform_core::{automation, overrides, AutoStart};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// The application under test is a BIN crate — include its source so the
// link-time inventory in this test binary carries the app's functions.
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

async fn http_call(port: u16, request_head: &str, body: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request = format!(
        "{request_head} HTTP/1.1\r\nHost: localhost\r\ncontent-type: application/json\r\n\
         accept: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
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

// One test function on purpose: the app boots ONCE per process (AutoStart is
// a run-once lifecycle), so all cases run in a single sequential test.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn greeting_end_to_end() {
    overrides::set("rest.server.port", "0"); // an ephemeral port for the test
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let port = automation::server_address().expect("server started").port();

    // 1. JSON body path
    let (status, payload) = http_call(port, "POST /api/greeting", r#"{"name": "Mercury"}"#).await;
    assert_eq!(status, 200, "unexpected: {payload}");
    assert!(payload.contains("Hello, Mercury"), "unexpected: {payload}");

    // 2. query-parameter path
    let (status, payload) = http_call(port, "GET /api/greeting?name=Mercury", "").await;
    assert_eq!(status, 200, "unexpected: {payload}");
    assert!(payload.contains("Hello, Mercury"), "unexpected: {payload}");

    // 3. validation: a request without a name is rejected with 400
    let (status, payload) = http_call(port, "POST /api/greeting", r#"{}"#).await;
    assert_eq!(status, 400, "unexpected: {payload}");
    assert!(payload.contains("Missing 'name'"), "unexpected: {payload}");
}
