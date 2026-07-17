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

//! Increment K-7a: the WebSocket server (Java `WsRequestHandler` protocol)
//! exercised end-to-end with a real tungstenite client — session route pair,
//! open/string/bytes/close lifecycle events, transmitter text/binary/JSON
//! replies, and route cleanup through the housekeeper.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use platform_core::{
    automation, overrides, AppConfigReader, AppError, ComposableFunction, EventEnvelope, Platform,
    PostOffice,
};
use rmpv::Value;
use tokio_tungstenite::tungstenite::Message;

/// Records lifecycle events and echoes messages back through the tx path.
struct EchoWs {
    platform: Platform,
    events: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl ComposableFunction for EchoWs {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let po = PostOffice::new(&self.platform);
        let kind = headers.get("type").cloned().unwrap_or_default();
        let tx_path = headers.get("tx_path").cloned().unwrap_or_default();
        match kind.as_str() {
            "open" => {
                self.events.lock().expect("events").push(format!(
                    "open token={} path={} query={}",
                    headers.get("token").map(String::as_str).unwrap_or(""),
                    headers.get("path").map(String::as_str).unwrap_or(""),
                    headers.get("query").map(String::as_str).unwrap_or(""),
                ));
                let greeting = format!(
                    "hello {}",
                    headers.get("token").map(String::as_str).unwrap_or("")
                );
                let _ = po
                    .send(
                        EventEnvelope::new()
                            .set_to(&tx_path)
                            .set_raw_body(Value::from(greeting.as_str())),
                    )
                    .await;
            }
            "string" => {
                let text = match input.body() {
                    Value::String(s) => s.as_str().unwrap_or_default().to_string(),
                    other => other.to_string(),
                };
                if text == "json-please" {
                    // a map body renders as a JSON text frame
                    let _ = po
                        .send(
                            EventEnvelope::new()
                                .set_to(&tx_path)
                                .set_raw_body(Value::Map(vec![(
                                    Value::from("kind"),
                                    Value::from("json"),
                                )])),
                        )
                        .await;
                } else if text == "close-please" {
                    // server-side close through the transmitter
                    let _ = po
                        .send(
                            EventEnvelope::new()
                                .set_to(&tx_path)
                                .set_header("type", "close")
                                .set_header("status", "1000")
                                .set_header("message", "server closing"),
                        )
                        .await;
                } else {
                    let _ = po
                        .send(
                            EventEnvelope::new()
                                .set_to(&tx_path)
                                .set_raw_body(Value::from(text.to_uppercase().as_str())),
                        )
                        .await;
                }
            }
            "bytes" => {
                let mut bytes = match input.body() {
                    Value::Binary(b) => b.clone(),
                    _ => Vec::new(),
                };
                bytes.reverse();
                let _ = po
                    .send(
                        EventEnvelope::new()
                            .set_to(&tx_path)
                            .set_raw_body(Value::from(bytes)),
                    )
                    .await;
            }
            "close" => {
                self.events.lock().expect("events").push(format!(
                    "close code={} reason={}",
                    headers.get("close_code").map(String::as_str).unwrap_or(""),
                    headers
                        .get("close_reason")
                        .map(String::as_str)
                        .unwrap_or(""),
                ));
            }
            _ => {}
        }
        EventEnvelope::new().set_body("ok")
    }
}

async fn start_server(events: Arc<Mutex<Vec<String>>>) -> (u16, Platform) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let dir = std::env::temp_dir().join(format!("mercury-ws-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let rest_file = dir.join("rest.yaml");
        std::fs::write(&rest_file, "rest: []\n").expect("rest.yaml");
        overrides::set(
            "yaml.rest.automation",
            &format!("file:{}", rest_file.display()),
        );
        overrides::set("rest.server.port", "0");
        let _ = AppConfigReader::get_instance();
    });
    let platform = Platform::new();
    let ws_platform = platform.clone();
    automation::register_ws_service("echo", move || {
        Arc::new(EchoWs {
            platform: ws_platform.clone(),
            events: events.clone(),
        })
    });
    let addr = automation::start_http_server(&platform)
        .await
        .expect("server");
    (addr.port(), platform)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn websocket_sessions_follow_the_java_protocol() {
    let events: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let (port, _platform) = start_server(events.clone()).await;

    // --- connect with a token and a query string
    let url = format!("ws://127.0.0.1:{port}/ws/echo/token-1?hello=world");
    let (mut socket, response) = tokio_tungstenite::connect_async(&url)
        .await
        .expect("ws connect");
    assert_eq!(101, response.status().as_u16());

    // open event greeting (Java: the service sends through the tx path)
    let greeting = socket.next().await.expect("greeting").expect("frame");
    assert_eq!(Message::Text("hello token-1".into()), greeting);

    // --- text echo (uppercased by the service)
    socket
        .send(Message::Text("abc".into()))
        .await
        .expect("send text");
    let reply = socket.next().await.expect("reply").expect("frame");
    assert_eq!(Message::Text("ABC".into()), reply);

    // --- binary echo (reversed by the service)
    socket
        .send(Message::Binary(vec![1u8, 2, 3]))
        .await
        .expect("send bytes");
    let reply = socket.next().await.expect("reply").expect("frame");
    assert_eq!(Message::Binary(vec![3u8, 2, 1]), reply);

    // --- a map body arrives as a JSON text frame
    socket
        .send(Message::Text("json-please".into()))
        .await
        .expect("send");
    let reply = socket.next().await.expect("reply").expect("frame");
    assert_eq!(Message::Text("{\"kind\":\"json\"}".into()), reply);

    // --- client-initiated close delivers the close event to the service
    socket.close(None).await.expect("close");
    tokio::time::sleep(Duration::from_millis(300)).await;
    {
        let log = events.lock().expect("events");
        assert!(
            log.iter()
                .any(|e| e.starts_with("open token=token-1 path=/ws/echo")
                    && e.contains("query=hello=world")),
            "open event expected: {log:?}"
        );
        assert!(
            log.iter().any(|e| e.starts_with("close code=")),
            "close event expected: {log:?}"
        );
    }

    // --- server-initiated close through the transmitter
    let url2 = format!("ws://127.0.0.1:{port}/ws/echo/token-2");
    let (mut socket2, _) = tokio_tungstenite::connect_async(&url2)
        .await
        .expect("ws connect 2");
    let _greeting = socket2.next().await.expect("greeting").expect("frame");
    socket2
        .send(Message::Text("close-please".into()))
        .await
        .expect("send");
    // the server closes with the requested code/reason
    let mut saw_close = false;
    while let Some(Ok(frame)) = socket2.next().await {
        if let Message::Close(Some(frame)) = frame {
            assert_eq!("server closing", frame.reason.to_string());
            saw_close = true;
            break;
        }
    }
    assert!(saw_close, "expected a server-side close frame");

    // --- a non-registered path is NOT upgraded (falls into REST routing)
    let bad =
        tokio_tungstenite::connect_async(format!("ws://127.0.0.1:{port}/ws/nothing/token")).await;
    assert!(bad.is_err(), "unregistered ws path must not upgrade");
}
