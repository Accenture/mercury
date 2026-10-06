// Scaffolded from Mercury's starter templates (https://github.com/Accenture/mercury, Apache-2.0)

//! Boots the whole starter once and drives greeting-flow end to end through
//! the REST endpoint - the happy path and the validation failure.

use platform_core::{automation, overrides, AutoStart};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// The application under test is a BIN crate — include its source so the
// link-time inventory in this test binary carries the app's functions.
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

async fn http_post(port: u16, path: &str, body: &str) -> (u16, String) {
    let mut stream = tokio::net::TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("connect");
    let request = format!(
        "POST {path} HTTP/1.1\r\nHost: localhost\r\ncontent-type: application/json\r\n\
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
/// A test binary never reaches the lifecycle's graceful exit, so this guard runs the elastic
/// queue's cleanup when the test ends, a failed one included: its holding folder goes too.
struct StoreCleanup;

impl Drop for StoreCleanup {
    fn drop(&mut self) {
        platform_core::util::elastic_queue::shutdown_cleanup();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn greeting_flow_end_to_end() {
    let _store = StoreCleanup;
    overrides::set("rest.server.port", "0"); // an ephemeral port for the test
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let port = automation::server_address().expect("server started").port();

    // happy path: the flow validates, then composes the greeting
    let (status, payload) = http_post(port, "/api/greeting", r#"{"name": "Mercury"}"#).await;
    assert_eq!(status, 200, "unexpected: {payload}");
    assert!(payload.contains("Hello, Mercury"), "unexpected: {payload}");
    assert!(
        payload.contains("v1.make.greeting"),
        "unexpected: {payload}"
    );

    // validation failure: the first task rejects and the flow aborts with 400
    let (status, payload) = http_post(port, "/api/greeting", r#"{"unexpected": "payload"}"#).await;
    assert_eq!(status, 400, "unexpected: {payload}");
}
