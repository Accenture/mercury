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

//! The generic half of a UI-pod streaming facade — Rust port of the Java
//! `StreamBridge` + `EventStreamSink` (design D2): wires a streaming
//! rendezvous to the HTTP edge's reply lane and owns its lifecycle, so an
//! application interceptor is a few lines plus its own request leg:
//!
//! ```no_run
//! # use std::sync::Arc;
//! # use platform_core::{AppError, EventEnvelope, Platform};
//! # use sync_over_async::{ReturnRouteCoordinator, StreamBridge};
//! # async fn facade(
//! #     coordinator: Arc<ReturnRouteCoordinator>,
//! #     platform: Platform,
//! #     request: EventEnvelope,
//! # ) -> Result<(), AppError> {
//! let cid = request.correlation_id().unwrap_or_default().to_string();
//! StreamBridge::open(&coordinator, &platform, &request, &cid, 30).await?;
//! // ... start the backend work however the application likes (the request leg) ...
//! # Ok(())
//! # }
//! ```
//!
//! [`StreamBridge::open`] commits the SSE head with `idle_seconds` as the
//! edge's idle allowance (widen it for a deliberately quiet notification
//! channel), registers an [`EventStreamSink`] via `begin_stream`, and arms an
//! idle watchdog with the same allowance. Every drained segment resets the
//! watchdog; the terminal segment disarms it (the drain already closed the
//! stream and freed the keys). At idle expiry the watchdog performs **one
//! final drain** — the streaming analogue of the one-shot final read before
//! timeout, recovering a dropped final notification (design D4's confirmed
//! nuance). Only if that drain does not complete the stream does it fail the
//! render in-band (408) and close the rendezvous, deleting the route so every
//! producer stops. This single watchdog is also what reclaims an abandoned
//! stream: the edge drops late writes after a client disconnect on its own,
//! and the idle expiry then releases the stream slot and the Redis keys, so
//! nothing leaks.
//!
//! At stream capacity the exchange is failed with a real HTTP 503 (the head
//! is not yet committed) and the returned writer is already closed —
//! mirroring the edge's own reply-lane back-pressure. (Java renders 503 for
//! the duplicate-cid rejection too; this port keeps that presentation.)
//!
//! **Port delta — the edge backstop gets explicit headroom.** In the Java
//! engine the edge's own between-segment timeout is enforced by a 10-second
//! sweep housekeeper ("does not need to be very accurate"), so the exact
//! watchdog always acts first by at least the sweep slack — that ordering is
//! what makes the idle-expiry final drain (and the in-band 408) reach the
//! client instead of the edge's own timeout. The Rust edge enforces its idle
//! allowance precisely per await, so this bridge commits the SSE head with
//! [`EDGE_GRACE_SECONDS`] of headroom over the watchdog allowance — the
//! housekeeper-interval analog. Observable contract unchanged: the watchdog
//! owns idle expiry at exactly `idle_seconds`; the edge backstop reclaims a
//! dead-facade stream within the same envelope as Java's sweep.
//!
//! The returned writer may be used for facade-authored events (e.g.
//! announcing the session's cid to the UI on a notification channel)
//! **before** any producer knows the cid; once producers post, the serialized
//! drain is the only writer.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::{AppError, EventEnvelope, EventStreamWriter, Platform};
use tokio::sync::{watch, Mutex};
use tokio::time::Instant;

use crate::coordinator::ReturnRouteCoordinator;
use crate::pending::SegmentSink;
use crate::segment::{self, StreamSegment};

const TEXT_EVENT_STREAM: &str = "text/event-stream";

/// Headroom granted to the HTTP edge's own between-segment timeout over the
/// watchdog allowance, so the watchdog (and its final drain) deterministically
/// acts first — the analog of the Java edge housekeeper's 10-second sweep
/// interval, which gives the Java watchdog the same structural margin.
pub const EDGE_GRACE_SECONDS: u64 = 10;

/// A clonable, shareable handle on one request's [`EventStreamWriter`] — the
/// bridge's drain sink, its idle watchdog, and the facade itself all write
/// through the same head-once, close-once state (the writer's own `closed`
/// check makes a late `fail` after a concurrent close a no-op, exactly like
/// the Java writer's CAS).
#[derive(Clone)]
pub struct SharedStreamWriter {
    inner: Arc<Mutex<EventStreamWriter>>,
}

impl SharedStreamWriter {
    pub fn new(writer: EventStreamWriter) -> Self {
        SharedStreamWriter {
            inner: Arc::new(Mutex::new(writer)),
        }
    }

    /// Send one unnamed `data` segment.
    pub async fn write<T: serde::Serialize + Send>(&self, body: T) -> Result<(), AppError> {
        self.inner.lock().await.write(body).await
    }

    /// Send one named segment — the name maps to the SSE `event:` field.
    pub async fn write_named<T: serde::Serialize + Send>(
        &self,
        event_name: &str,
        body: T,
    ) -> Result<(), AppError> {
        self.inner.lock().await.write_named(event_name, body).await
    }

    /// Declare end of transmission.
    pub async fn close(&self) -> Result<(), AppError> {
        self.inner.lock().await.close().await
    }

    /// Declare end of transmission with trailing metadata.
    pub async fn close_with<T: serde::Serialize + Send>(
        &self,
        metadata: T,
    ) -> Result<(), AppError> {
        self.inner.lock().await.close_with(metadata).await
    }

    /// Declare an in-band failure and end the stream.
    pub async fn fail(&self, error: &AppError) -> Result<(), AppError> {
        self.inner.lock().await.fail(error).await
    }

    /// True when the stream has been closed or failed.
    pub async fn is_closed(&self) -> bool {
        self.inner.lock().await.is_closed()
    }
}

/// The canonical `begin_stream` sink (Java `EventStreamSink`): forwards each
/// drained [`StreamSegment`] into the request's [`EventStreamWriter`], so the
/// shipped `x-event-stream` edge does the last mile (SSE/chunked framing,
/// ordered reply lane, back-pressure, disconnect handling):
///
/// - `data` → `write` (named when the segment carries an SSE event name);
/// - `eof` → `close_with` (the segment body rides as the terminal event's
///   metadata);
/// - `exception` → `fail` with HTTP 500 and the segment body as the message.
///
/// Most facades want [`StreamBridge`], which combines this sink with the
/// idle-expiry lifecycle; this type stands alone for facades that manage
/// their own lifecycle.
pub struct EventStreamSink {
    writer: SharedStreamWriter,
}

impl EventStreamSink {
    pub fn new(writer: SharedStreamWriter) -> Self {
        EventStreamSink { writer }
    }
}

#[async_trait]
impl SegmentSink for EventStreamSink {
    async fn accept(&self, segment: StreamSegment) -> Result<(), AppError> {
        match segment.segment_type() {
            segment::EOF => self.writer.close_with(segment.body()).await,
            segment::EXCEPTION => {
                self.writer
                    .fail(&AppError::new(
                        500,
                        segment.body().unwrap_or("Stream failed"),
                    ))
                    .await
            }
            _ => {
                let body = segment.body().unwrap_or("");
                match segment.name() {
                    Some(name) => self.writer.write_named(name, body).await,
                    None => self.writer.write(body).await,
                }
            }
        }
    }
}

/// Opens a streaming rendezvous and bridges it to the request's reply lane as
/// an SSE response (see the module documentation for the full lifecycle).
pub struct StreamBridge;

impl StreamBridge {
    /// Open a streaming rendezvous and bridge it to the request's reply lane.
    ///
    /// * `coordinator` — the pod's return-route coordinator
    /// * `platform` — the platform the facade is registered on (its reply
    ///   lane lives there)
    /// * `request` — the incoming envelope of the interceptor facade
    /// * `cid` — the rendezvous correlation id (typically the request's)
    /// * `idle_seconds` — idle allowance between segments, applied to the
    ///   HTTP edge and the watchdog alike
    ///
    /// Returns the writer bound to the request — already failed and closed if
    /// the pod was at stream capacity (503, delivered in-band before the head
    /// was committed). A transport failure while publishing the return route
    /// is a real error and propagates.
    pub async fn open(
        coordinator: &Arc<ReturnRouteCoordinator>,
        platform: &Platform,
        request: &EventEnvelope,
        cid: &str,
        idle_seconds: u64,
    ) -> Result<SharedStreamWriter, AppError> {
        let mut writer = EventStreamWriter::from_request(platform, request)?;
        // the edge idle allowance rides the first event's x-ttl header - with
        // headroom, so the watchdog below owns idle expiry (see EDGE_GRACE_SECONDS)
        writer.first_with_ttl(200, TEXT_EVENT_STREAM, idle_seconds + EDGE_GRACE_SECONDS);
        let writer = SharedStreamWriter::new(writer);
        let (disarm, disarmed) = watch::channel(false);
        let session = Arc::new(Session {
            coordinator: coordinator.clone(),
            writer: writer.clone(),
            forwarder: EventStreamSink::new(writer.clone()),
            cid: cid.to_string(),
            idle: Duration::from_secs(idle_seconds.max(1)),
            last_activity: StdMutex::new(Instant::now()),
            done: AtomicBool::new(false),
            disarm,
        });
        if let Err(rejected) = coordinator.begin_stream(cid, session.clone()).await {
            if rejected.status() == 503 || rejected.status() == 409 {
                // registration back-pressure: the head is not committed yet, so
                // this renders a proper HTTP error - deterministic like the
                // edge's own 503 (and 503 for the duplicate case too, matching
                // the Java presentation)
                let refusal = AppError::new(503, rejected.message());
                if let Err(e) = writer.fail(&refusal).await {
                    log::debug!("Unable to refuse stream {cid} in-band - {}", e.message());
                }
                return Ok(writer);
            }
            return Err(rejected);
        }
        tokio::spawn(watchdog(session, disarmed));
        Ok(writer)
    }
}

/// Per-stream lifecycle (Java `StreamBridge.Session`): forwards segments,
/// tracks activity, and owns the idle-expiry final drain.
struct Session {
    coordinator: Arc<ReturnRouteCoordinator>,
    writer: SharedStreamWriter,
    forwarder: EventStreamSink,
    cid: String,
    idle: Duration,
    last_activity: StdMutex<Instant>,
    done: AtomicBool,
    disarm: watch::Sender<bool>,
}

impl Session {
    fn deadline(&self) -> Instant {
        *self.last_activity.lock().expect("last_activity poisoned") + self.idle
    }

    fn disarm(&self) {
        self.done.store(true, Ordering::Release);
        let _ = self.disarm.send(true);
    }

    async fn expire(&self) {
        // ONE last-chance drain (design D4's confirmed nuance): a dropped
        // final notification still completes the render here - the same
        // recovery cornerstone as the one-shot final read
        let completed = self.coordinator.final_drain(&self.cid).await;
        if completed || self.done.load(Ordering::Acquire) {
            return; // the drain (this one or a concurrent wake-up's) delivered the terminal
        }
        self.disarm();
        log::debug!(
            "Stream {} idle for {:?} - failing in-band and closing the rendezvous",
            self.cid,
            self.idle
        );
        // if a concurrent drain just closed the writer, the writer's own
        // closed check makes this a no-op
        if let Err(e) = self
            .writer
            .fail(&AppError::new(408, "Stream idle timeout"))
            .await
        {
            log::debug!(
                "Unable to fail stream {} in-band - {}",
                self.cid,
                e.message()
            );
        }
        self.coordinator.close_stream(&self.cid).await;
    }
}

#[async_trait]
impl SegmentSink for Session {
    async fn accept(&self, segment: StreamSegment) -> Result<(), AppError> {
        *self.last_activity.lock().expect("last_activity poisoned") = Instant::now();
        let terminal = segment.is_terminal();
        self.forwarder.accept(segment).await?;
        if terminal {
            // the drain that delivered the terminal also closes the stream
            // and its keys
            self.disarm();
        }
        Ok(())
    }
}

/// The idle watchdog: sleeps to the activity deadline, re-arms after
/// activity, and runs the one final drain at expiry. Disarm (the terminal
/// segment, or expiry itself) ends the task promptly via the watch channel.
async fn watchdog(session: Arc<Session>, mut disarmed: watch::Receiver<bool>) {
    loop {
        let deadline = session.deadline();
        tokio::select! {
            _ = disarmed.changed() => return,
            _ = tokio::time::sleep_until(deadline) => {}
        }
        if session.done.load(Ordering::Acquire) {
            return;
        }
        if session.deadline() > Instant::now() {
            continue; // activity since the last arm - sleep out the remainder
        }
        session.expire().await;
        return;
    }
}
