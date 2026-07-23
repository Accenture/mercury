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

//! Increment 48 — outbound HTTPS for the async HTTP client. A local TLS
//! server with a self-signed certificate proves both verification modes:
//! `trust_all_cert` succeeds against the self-signed endpoint (Java's
//! `InsecureTrustManagerFactory` analog), and the default strict mode
//! rejects it with an in-band TLS error (the OS trust store does not know
//! the issuer). No external network access is involved.

use std::sync::{Arc, Once};
use std::time::Duration;

use platform_core::automation::http_client::AsyncHttpClientService;
use platform_core::automation::{AsyncHttpRequest, ASYNC_HTTP_REQUEST};
use platform_core::platform::FunctionOptions;
use platform_core::{overrides, resources, AppConfigReader, EventEnvelope, Platform, PostOffice};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::rustls;
use tokio_rustls::TlsAcceptor;

fn setup_config() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding = std::env::temp_dir().join(format!("mercury-tls-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        let _ = AppConfigReader::get_instance();
    });
}

fn register_http_client(platform: &Platform) {
    platform
        .register_with_options(
            ASYNC_HTTP_REQUEST,
            Arc::new(AsyncHttpClientService::new(platform)),
            10,
            FunctionOptions {
                zero_traced: false,
                interceptor: true,
                private: false,
            },
        )
        .expect("http client route");
}

/// Minimal HTTPS server: self-signed cert for localhost/127.0.0.1, answers
/// every request with a fixed JSON body, tolerates failed handshakes (the
/// strict-mode test aborts mid-handshake by design).
async fn start_tls_server() -> u16 {
    let signed =
        rcgen::generate_simple_self_signed(vec!["localhost".to_string(), "127.0.0.1".to_string()])
            .expect("self-signed cert");
    let cert_der = signed.cert.der().clone();
    let key_der = rustls::pki_types::PrivateKeyDer::Pkcs8(signed.key_pair.serialize_der().into());
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], key_der)
        .expect("server tls config");
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let port = listener.local_addr().expect("addr").port();
    tokio::spawn(async move {
        loop {
            let Ok((tcp, _)) = listener.accept().await else {
                break;
            };
            let acceptor = acceptor.clone();
            tokio::spawn(async move {
                // a rejected certificate aborts the handshake here — expected
                let Ok(mut tls) = acceptor.accept(tcp).await else {
                    return;
                };
                let mut buffer = vec![0u8; 4096];
                let mut request = Vec::new();
                while !request.windows(4).any(|w| w == b"\r\n\r\n") {
                    match tls.read(&mut buffer).await {
                        Ok(0) | Err(_) => return,
                        Ok(n) => request.extend_from_slice(&buffer[..n]),
                    }
                }
                let body = "{\"hello\":\"tls\"}";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\
                     content-length: {}\r\nconnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = tls.write_all(response.as_bytes()).await;
                let _ = tls.shutdown().await;
            });
        }
    });
    port
}

async fn https_request(platform: &Platform, port: u16, trust_all_cert: bool) -> EventEnvelope {
    let request = AsyncHttpRequest::new()
        .set_method("GET")
        .set_url("/api/hello")
        .set_target_host(&format!("https://127.0.0.1:{port}"))
        .set_trust_all_cert(trust_all_cert);
    let po = PostOffice::new(platform);
    po.request(
        EventEnvelope::new()
            .set_to(ASYNC_HTTP_REQUEST)
            .set_raw_body(request.to_value()),
        Duration::from_secs(10),
    )
    .await
    .expect("reply envelope")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_with_trust_all_cert_reaches_a_self_signed_endpoint() {
    setup_config();
    let platform = Platform::new();
    register_http_client(&platform);
    let port = start_tls_server().await;
    let reply = https_request(&platform, port, true).await;
    assert_eq!(200, reply.status(), "unexpected reply: {:?}", reply.body());
    let body = reply.body();
    let map = body.as_map().expect("json body decoded to a map");
    let hello = map
        .iter()
        .find(|(k, _)| k.as_str() == Some("hello"))
        .map(|(_, v)| v.as_str().unwrap_or_default())
        .unwrap_or_default();
    assert_eq!("tls", hello);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn https_strict_mode_rejects_a_self_signed_certificate() {
    setup_config();
    let platform = Platform::new();
    register_http_client(&platform);
    let port = start_tls_server().await;
    let reply = https_request(&platform, port, false).await;
    assert_eq!(500, reply.status(), "unexpected reply: {:?}", reply.body());
    let message = reply.body().as_str().unwrap_or_default().to_string();
    assert!(
        message.contains("TLS handshake failed"),
        "error should surface the TLS failure in-band: {message}"
    );
}

#[test]
fn trust_all_cert_round_trips_with_the_target_host() {
    // Java toMap parity: trust_all_cert travels alongside the relay target
    let request = AsyncHttpRequest::new()
        .set_method("GET")
        .set_url("/")
        .set_target_host("https://example.com")
        .set_trust_all_cert(true);
    let restored = AsyncHttpRequest::from_value(&request.to_value());
    assert!(restored.trust_all_cert());
    assert_eq!(Some("https://example.com"), restored.target_host());
    // absent flag parses as false
    let plain = AsyncHttpRequest::new().set_target_host("https://example.com");
    let restored = AsyncHttpRequest::from_value(&plain.to_value());
    assert!(!restored.trust_all_cert());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unsupported_protocol_is_rejected_in_band() {
    setup_config();
    let platform = Platform::new();
    register_http_client(&platform);
    let request = AsyncHttpRequest::new()
        .set_method("GET")
        .set_url("/")
        .set_target_host("ftp://example.com");
    let po = PostOffice::new(&platform);
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to(ASYNC_HTTP_REQUEST)
                .set_raw_body(request.to_value()),
            Duration::from_secs(5),
        )
        .await
        .expect("reply envelope");
    assert_eq!(400, reply.status());
}
