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

//! **Cross-pod progressive rendering** — the streaming-return-route demo
//! (Rust twin of the Java `sync-over-async-demo` stream roles).
//!
//! In a horizontally scaled deployment, the pod that *produces* progressive
//! events is generally not the pod holding the user's HTTP connection. The
//! [streaming return route](../../draft-design-specs/sync-over-async-port.md)
//! closes that gap with Redis alone — no broker anywhere. This one binary
//! plays either side, selected by an active profile:
//!
//! ```bash
//! # pod A - holds the SSE connection (coordinator + StreamBridge facade), port 8600
//! cargo run -p sync-over-async-demo -- -Dapp.profiles.active=stream-ui
//!
//! # pod B - posts the events (StreamResponder only - no coordinator), port 8601
//! cargo run -p sync-over-async-demo -- -Dapp.profiles.active=stream-producer
//! ```
//!
//! The client opens `GET /api/notifications` on pod A (`stream: true`, SSE);
//! the facade announces the session's correlation id as the first SSE event
//! (`event: cid`); the client quotes that cid in POSTs to pod B's
//! `/api/produce`, which posts segments into the rendezvous — the notification
//! use case's exact shape. `scripts/sse-client.mjs` renders the SSE side with
//! wall-clock timestamps, and the README walks the full chaos runbook
//! (lost notifications, lost close, a killed producer, a killed UI pod).

use std::collections::HashMap;

use async_trait::async_trait;
use platform_core::automation::AsyncHttpRequest;
use platform_core::{
    before_application, main_application, preload, AppConfigReader, AppError, ComposableFunction,
    EntryPoint, EventEnvelope, EventStreamWriter, Platform,
};
use sync_over_async::{
    runtime, segment, RedisHealthCheck, RedisSettings, ReturnRouteStore, StreamBridge,
    StreamResponder, StreamSegment, SyncOverAsyncConfig,
};
use tokio::sync::OnceCell;

// ---------------------------------------------------------------------------
// stream-ui role: the notification channel facade
// ---------------------------------------------------------------------------

const CID_EVENT: &str = "cid";
const IDLE_HEADER: &str = "x-stream-idle-seconds";
const DEFAULT_IDLE_SECONDS: u64 = 30;

/// The stream-ui pod's notification channel (`GET /api/notifications`,
/// declared `stream: true`): an interceptor addressed directly by the
/// endpoint — the streaming-return-route facade shape (design D2).
/// [`StreamBridge`] does the heavy lifting (SSE head, drain-to-reply-lane,
/// idle watchdog with the single final drain); this function only announces
/// the session's correlation id back to the UI as the first SSE event
/// (`event: cid`), which the UI quotes when it POSTs to producer pods so they
/// know where to post.
///
/// The idle allowance defaults to 30s for the demo and can be set per request
/// with the `x-stream-idle-seconds` HTTP header (the chaos runs use a short
/// one to show the idle-expiry endings quickly; a production notification
/// channel would use minutes).
#[preload(route = "demo.stream.facade", instances = 50, interceptor)]
struct StreamNotifyFacade;

#[async_trait]
impl ComposableFunction for StreamNotifyFacade {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let platform = Platform::get_instance();
        let Some(coordinator) = runtime::coordinator() else {
            // sync.over.async.enabled=false on this pod - a teaching failure
            let mut out = EventStreamWriter::from_request(&platform, &input)?;
            out.fail(&AppError::new(
                503,
                "sync-over-async is not enabled on this pod - run the stream-ui profile",
            ))
            .await?;
            return Ok(EventEnvelope::new());
        };
        let http = AsyncHttpRequest::from_value(input.body());
        let idle_seconds = http
            .header(IDLE_HEADER)
            .and_then(|value| value.trim().parse::<u64>().ok())
            .unwrap_or(DEFAULT_IDLE_SECONDS);
        let cid = match input.correlation_id() {
            Some(id) => id.to_string(),
            None => uuid::Uuid::new_v4().simple().to_string(),
        };
        let writer =
            StreamBridge::open(&coordinator, &platform, &input, &cid, idle_seconds).await?;
        // hand the session's cid to the UI before any producer can know it
        writer.write_named(CID_EVENT, &cid).await?;
        Ok(EventEnvelope::new())
    }
}

// ---------------------------------------------------------------------------
// stream-producer role: the backend service posting into the rendezvous
// ---------------------------------------------------------------------------

const TOKENS: [&str; 5] = ["Streaming", " across", " pods", " by", " design"];
const EOT_METADATA: &str = "{\"tokens\":5}";
const CHAOS_TTL_SECONDS: u64 = 120;

// shared per pod, lazily built on first use from live configuration - never
// at startup registration time (the preload-before-bootstrap lesson)
static RESPONDER: OnceCell<StreamResponder> = OnceCell::const_new();
static CHAOS_STORE: OnceCell<ReturnRouteStore> = OnceCell::const_new();

async fn responder() -> Result<&'static StreamResponder, AppError> {
    RESPONDER
        .get_or_try_init(|| async {
            let settings = RedisSettings::from_config();
            StreamResponder::connect(&settings).await
        })
        .await
}

/// Post one segment through the pod's shared responder.
///
/// Returns `true` while the rendezvous is live; `false` for an orphan (stop
/// producing).
async fn post_segment(
    cid: &str,
    segment_type: &str,
    name: Option<&str>,
    body: Option<&str>,
) -> Result<bool, AppError> {
    responder().await?.post(cid, segment_type, name, body).await
}

/// Chaos-mode plumbing: a store handle of our own, so a segment can be
/// appended with NO wake-up (a lost Pub/Sub notification, simulated).
async fn append_without_wake_up(cid: &str, segment: &StreamSegment) -> Result<(), AppError> {
    let store = CHAOS_STORE
        .get_or_try_init(|| async {
            let settings = RedisSettings::from_config();
            Ok::<ReturnRouteStore, AppError>(ReturnRouteStore::new(
                settings.manager().await?,
                settings.timeout(),
            ))
        })
        .await?;
    store
        .append_segment(cid, &segment.to_json(), CHAOS_TTL_SECONDS)
        .await
}

/// The stream-producer pod's backend service (`POST /api/produce`): posts
/// segments into a streaming rendezvous opened by the stream-ui pod, purely
/// through Redis ([`StreamResponder`] — no coordinator, no broker; design D3).
/// The request body selects a scenario:
///
/// ```text
/// {"cid": "...", "mode": "chat"}                          five ordered tokens + eof
/// {"cid": "...", "mode": "notify", "name": "orders", "body": "order 42 shipped"}
/// {"cid": "...", "mode": "close"}                         terminal eof (any producer may close)
/// {"cid": "...", "mode": "stall"}                         two tokens, NO terminal (producer-death chaos)
/// {"cid": "...", "mode": "lost", "type": "data|eof", ...} CHAOS: store the segment but suppress
///                                                         the wake-up (a lost notification)
/// ```
///
/// The response reports `live`: `false` means the rendezvous is over (the UI
/// pod closed, timed out, or died — the orphan contract), which is the
/// producer's signal to stop. The `lost` mode is chaos tooling only: it
/// appends via [`ReturnRouteStore`] without publishing, to demonstrate the
/// recovery paths (the next real post's drain, or the UI pod's final drain at
/// idle expiry).
#[preload(route = "demo.stream.producer", instances = 10)]
struct StreamProducer;

#[async_trait]
impl ComposableFunction for StreamProducer {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // addressed directly by rest.yaml (no flow in between), so the input
        // is the whole AsyncHttpRequest dataset - the JSON body is inside it
        let request: serde_json::Value = input.body_as()?;
        let body = &request["body"];
        if !body.is_object() {
            return Err(AppError::new(400, "Request body must be a JSON object"));
        }
        let (Some(cid), Some(mode)) = (body["cid"].as_str(), body["mode"].as_str()) else {
            return Err(AppError::new(
                400,
                "Request body must carry 'cid' and 'mode'",
            ));
        };
        let mut result = serde_json::json!({"mode": mode, "cid": cid});
        match mode {
            "chat" => produce_chat_tokens(cid, &mut result).await?,
            "notify" => {
                let live = post_segment(
                    cid,
                    segment::DATA,
                    body["name"].as_str(),
                    body["body"].as_str(),
                )
                .await?;
                result["live"] = live.into();
                result["posted"] = 1.into();
            }
            "close" => {
                result["live"] = post_segment(cid, segment::EOF, None, None).await?.into();
            }
            "stall" => {
                // the producer "dies" after two tokens - no terminal ever
                // arrives, so the UI pod's idle expiry must fail the render
                // in-band (the kill-the-producer chaos check)
                let live = post_segment(cid, segment::DATA, None, Some("first")).await?
                    && post_segment(cid, segment::DATA, None, Some("second")).await?;
                result["live"] = live.into();
                result["posted"] = 2.into();
            }
            "lost" => {
                // CHAOS ONLY: store-first happens, the wake-up never does.
                // Recovery is the next real post's drain, or the UI pod's
                // final drain at idle expiry.
                let segment_type = body["type"].as_str().unwrap_or(segment::DATA);
                let segment =
                    StreamSegment::of(segment_type, body["name"].as_str(), body["body"].as_str())?;
                append_without_wake_up(cid, &segment).await?;
                result["stored"] = true.into();
                result["wakeUpSuppressed"] = true.into();
            }
            other => return Err(AppError::new(400, format!("Unknown mode: {other}"))),
        }
        EventEnvelope::new()
            .set_header("content-type", "application/json")
            .set_body(result)
    }
}

/// Mode `chat`: a single sequential producer over one connection — list order
/// == posting order (design D7).
async fn produce_chat_tokens(cid: &str, result: &mut serde_json::Value) -> Result<(), AppError> {
    let mut live = true;
    let mut posted = 0;
    for token in TOKENS {
        live = post_segment(cid, segment::DATA, None, Some(token)).await?;
        if !live {
            break; // orphan - the rendezvous is over, stop producing
        }
        posted += 1;
    }
    if live {
        live = post_segment(cid, segment::EOF, None, Some(EOT_METADATA)).await?;
        posted += 1;
    }
    result["live"] = live.into();
    result["posted"] = posted.into();
    Ok(())
}

// ---------------------------------------------------------------------------
// lifecycle
// ---------------------------------------------------------------------------

/// Start the return-route coordinator when the role enables it, and register
/// the Redis health check on every role — the Java `SyncOverAsyncAutoStart`
/// analog. `/health` includes the probe wherever `mandatory.health.dependencies`
/// lists `soa.redis.health`.
#[before_application(sequence = 10)]
struct SyncOverAsyncBootstrap;

#[async_trait]
impl EntryPoint for SyncOverAsyncBootstrap {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let platform = Platform::get_instance();
        RedisHealthCheck::from_config().register(&platform)?;
        let config = AppConfigReader::get_instance();
        if config.get_property_or("sync.over.async.enabled", "false") == "true" {
            let settings = RedisSettings::from_config();
            let coordinator = runtime::init(
                &settings,
                Platform::origin(),
                SyncOverAsyncConfig::from_config(),
            )
            .await?;
            log::info!(
                "Return-route coordinator started for pod {} (redis {}:{}, channel {})",
                Platform::origin(),
                settings.host(),
                settings.port(),
                coordinator.return_channel()
            );
        } else {
            log::info!("No return-route coordinator on this pod (producer role)");
        }
        Ok(())
    }
}

#[main_application]
struct MainApp;

#[async_trait]
impl EntryPoint for MainApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let config = AppConfigReader::get_instance();
        let port = config.get_property_or("rest.server.port", "8085");
        if config.get_property_or("sync.over.async.enabled", "false") == "true" {
            log::info!(
                "stream-ui ready - try: node scripts/sse-client.mjs http://127.0.0.1:{port}/api/notifications"
            );
        } else {
            log::info!(
                "stream-producer ready - POST http://127.0.0.1:{port}/api/produce with {{\"cid\": \"...\", \"mode\": \"chat\"}}"
            );
        }
        Ok(())
    }
}

// the whole startup — runtime, -D overrides, structured logging, annotation
// collection, lifecycle, serve until Ctrl-C
platform_core::auto_start_main!();
