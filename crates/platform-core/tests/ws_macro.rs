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

//! The declarative websocket endpoint (Java `@WebSocketService` parity):
//! `#[websocket_service("...")]` registers through the link-time inventory,
//! the AppStarter lifecycle loads the URL path, and the HTTP server starts
//! because a websocket service exists — even with `rest.automation`
//! disabled (Java `startHttpServerIfAny` semantics).

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use platform_core::{
    main_application, overrides, websocket_service, AppError, AutoStart, ComposableFunction,
    EntryPoint, EventEnvelope, Platform, PostOffice,
};
use rmpv::Value;
use tokio_tungstenite::tungstenite::Message;

/// A declarative websocket endpoint at `/ws/macro-echo/{token}`.
#[websocket_service("macro-echo")]
struct MacroEcho;

#[async_trait]
impl ComposableFunction for MacroEcho {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let platform = Platform::get_instance();
        let po = PostOffice::new(&platform);
        let tx_path = headers.get("tx_path").cloned().unwrap_or_default();
        match headers.get("type").map(String::as_str) {
            Some("open") => {
                let greeting = format!(
                    "declared {}",
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
            Some("string") => {
                let text = match input.body() {
                    Value::String(s) => s.as_str().unwrap_or_default().to_string(),
                    other => other.to_string(),
                };
                let _ = po
                    .send(
                        EventEnvelope::new()
                            .set_to(&tx_path)
                            .set_raw_body(Value::from(text.to_uppercase().as_str())),
                    )
                    .await;
            }
            _ => {}
        }
        EventEnvelope::new().set_body("ok")
    }
}

#[main_application]
struct WsMacroApp;

#[async_trait]
impl EntryPoint for WsMacroApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!("ws macro test app started");
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn declarative_websocket_service_registers_through_the_lifecycle() {
    let dir = std::env::temp_dir().join(format!("mercury-ws-macro-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let rest_file = dir.join("rest.yaml");
    std::fs::write(&rest_file, "rest: []\n").expect("rest.yaml");
    overrides::set(
        "yaml.rest.automation",
        &format!("file:{}", rest_file.display()),
    );
    overrides::set("rest.server.port", "8095");
    // rest.automation stays FALSE: the server must start anyway because a
    // websocket service is declared (Java startHttpServerIfAny parity) —
    // and the app stays alive, so AutoStart runs in the background
    tokio::spawn(AutoStart::main(vec![]));
    // wait for the server to come up
    let url = "ws://127.0.0.1:8095/ws/macro-echo/tok9";
    let mut socket = None;
    for _ in 0..50 {
        match tokio_tungstenite::connect_async(url).await {
            Ok((ws, response)) => {
                assert_eq!(101, response.status().as_u16());
                socket = Some(ws);
                break;
            }
            Err(_) => tokio::time::sleep(Duration::from_millis(100)).await,
        }
    }
    let mut socket = socket.expect("websocket server should start for a declared service");
    // the declared service answered the open event through its tx path
    let greeting = socket.next().await.expect("greeting").expect("frame");
    assert_eq!(Message::Text("declared tok9".into()), greeting);
    // and echoes text messages
    socket
        .send(Message::Text("declarative".into()))
        .await
        .expect("send");
    let reply = socket.next().await.expect("reply").expect("frame");
    assert_eq!(Message::Text("DECLARATIVE".into()), reply);
    socket.close(None).await.expect("close");
}
