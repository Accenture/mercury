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

//! The cross-pod return-route engine for one pod — Rust port of the Java
//! `ReturnRouteCoordinator`.
//!
//! One storage mechanism — a per-cid Redis List of [segments](StreamSegment),
//! drained destructively — serves two rendezvous patterns: a **one-shot**
//! response is simply the degenerate stream whose first entry is terminal.
//!
//! - [`begin`](ReturnRouteCoordinator::begin) /
//!   [`await_response`](ReturnRouteCoordinator::await_response) /
//!   [`deliver`](ReturnRouteCoordinator::deliver) — the one-shot pattern:
//!   register a pending request, wait for the response (with a **final drain**
//!   on timeout so a missed Pub/Sub notification still resolves), and deliver =
//!   post one terminal segment then wake the originator.
//! - [`begin_stream`](ReturnRouteCoordinator::begin_stream) /
//!   [`close_stream`](ReturnRouteCoordinator::close_stream) /
//!   [`final_drain`](ReturnRouteCoordinator::final_drain) — the streaming
//!   pattern: register a sink, and each wake-up drains the queue into it —
//!   serialized per cid, so forward order equals list order — until a terminal
//!   segment completes the stream. Any producer may post the terminal entry;
//!   the route's deletion then stops the rest (orphan).
//! - one subscription on this pod's return channel serves both: a wake-up
//!   carrying a cid is checked against the open streams first, then the pending
//!   one-shot requests.
//!
//! **Reconnect delta from Java (port spec §5.2).** Lettuce resubscribes
//! automatically; the `redis` crate's dedicated Pub/Sub connection does not, so
//! the subscriber task owns a reconnect-and-resubscribe loop and runs one
//! [recovery pass](Rendezvous::recovery_pass) afterwards — a final drain per
//! open stream and a queue check per pending request — so any wake-up lost
//! during the gap is healed by machinery that already exists. Wake-ups are
//! best-effort by contract; correctness rests on the queue.

use std::sync::Arc;
use std::time::Duration;

use futures_util::StreamExt;
use platform_core::AppError;
use tokio::sync::watch;

use crate::config::SyncOverAsyncConfig;
use crate::connection::RedisSettings;
use crate::pending::{PendingEntry, PendingRequests, PendingStreams, SegmentSink, StreamEntry};
use crate::segment::{self, StreamSegment};
use crate::store::ReturnRouteStore;

/// How long to wait between Pub/Sub reconnect attempts.
const RECONNECT_DELAY: Duration = Duration::from_millis(500);

/// The state a wake-up acts on, shared with the subscriber task.
struct Rendezvous {
    store: ReturnRouteStore,
    pending: PendingRequests,
    streams: PendingStreams,
}

impl Rendezvous {
    /// A wake-up is a bare cid; one channel serves both patterns — streams
    /// first, then one-shot requests.
    async fn on_wake_up(&self, business_correlation_id: &str) {
        if self.streams.contains(business_correlation_id) {
            self.drain_stream(business_correlation_id).await;
            return;
        }
        if let Some(payload) = self.pop_response_body(business_correlation_id).await {
            // completes in place; the entry stays until await_response (or
            // abort) removes it, so an await-by-cid arriving after this
            // wake-up still finds the response (early-arrival path)
            self.pending.complete(business_correlation_id, payload);
        }
    }

    /// Pop the first queued segment of a one-shot rendezvous and return its
    /// body. The one-shot producer contract is a single terminal entry, so the
    /// first entry *is* the response; a malformed entry is discarded (logged)
    /// rather than delivered.
    async fn pop_response_body(&self, business_correlation_id: &str) -> Option<String> {
        let json = match self.store.pop_segment(business_correlation_id).await {
            Ok(Some(json)) => json,
            Ok(None) => return None,
            Err(e) => {
                log::warn!("Unable to read the queue for {business_correlation_id} - {e}");
                return None;
            }
        };
        match StreamSegment::from_json(&json) {
            Ok(response) => response.body().map(str::to_string),
            Err(e) => {
                log::warn!("Discarding malformed segment for {business_correlation_id} - {e}");
                None
            }
        }
    }

    /// Drain the stream's queue into its sink until empty or terminal.
    /// Serialized per cid by the entry's drain flag: concurrent wake-ups
    /// (several producers) collapse onto one forwarding loop, so forward order
    /// equals list order. The flag is held **through terminal cleanup**, so no
    /// segment can be forwarded after the sink saw the terminal one; and after
    /// releasing, the queue is re-checked once — a producer that appended
    /// during the hold (its wake-up bounced off the flag) is not left stranded.
    ///
    /// Returns `true` when this call completed the stream (terminal segment
    /// delivered, or sink failure).
    async fn drain_stream(&self, business_correlation_id: &str) -> bool {
        loop {
            let Some(entry) = self.streams.get(business_correlation_id) else {
                return false; // stream closed
            };
            if !entry.try_acquire_drain() {
                return false; // another drain is active (it re-checks before exiting)
            }
            if self
                .forward_queued_segments(business_correlation_id, &entry)
                .await
            {
                // still under the drain hold: nothing can follow the terminal segment
                self.close_stream(business_correlation_id).await;
                entry.release_drain();
                return true;
            }
            entry.release_drain();
            match self.store.queue_length(business_correlation_id).await {
                Ok(0) | Err(_) => return false,
                // lost-wakeup guard: a segment landed while this drain was
                // finishing - loop and re-acquire
                Ok(_) => {}
            }
        }
    }

    /// Forward queued segments to the sink in list order until the queue is
    /// empty or a terminal condition is reached. Runs entirely under the
    /// caller's drain hold.
    ///
    /// Returns `true` on a terminal condition (terminal segment delivered, a
    /// malformed entry, or sink failure — the consumer is gone).
    async fn forward_queued_segments(
        &self,
        business_correlation_id: &str,
        entry: &Arc<StreamEntry>,
    ) -> bool {
        loop {
            let json = match self.store.pop_segment(business_correlation_id).await {
                Ok(Some(json)) => json,
                Ok(None) => return false,
                Err(e) => {
                    // a transport hiccup is not the consumer's fault: leave the
                    // stream open, the next wake-up (or final drain) retries
                    log::warn!("Drain of {business_correlation_id} interrupted - {e}");
                    return false;
                }
            };
            let segment = match StreamSegment::from_json(&json) {
                Ok(segment) => segment,
                Err(e) => {
                    log::warn!(
                        "Closing stream {business_correlation_id} - segment could not be delivered: {e}"
                    );
                    return true;
                }
            };
            let terminal = segment.is_terminal();
            if let Err(e) = entry.sink().accept(segment).await {
                log::warn!(
                    "Closing stream {business_correlation_id} - segment could not be delivered: {e}"
                );
                return true;
            }
            if terminal {
                return true;
            }
        }
    }

    /// Remove the entry and delete the Redis keys eagerly; the route's
    /// disappearance is what tells every producer to stop.
    async fn close_stream(&self, business_correlation_id: &str) {
        self.streams.remove(business_correlation_id);
        if let Err(e) = self.store.cleanup(business_correlation_id).await {
            log::warn!("Unable to clean up {business_correlation_id} - {e}");
        }
    }

    /// After a Pub/Sub resubscribe, heal anything whose wake-up was lost while
    /// the subscription was down: one drain per open stream, one queue check
    /// per pending request.
    async fn recovery_pass(&self) {
        for business_correlation_id in self.streams.ids() {
            self.drain_stream(&business_correlation_id).await;
        }
        for business_correlation_id in self.pending.ids() {
            if let Some(payload) = self.pop_response_body(&business_correlation_id).await {
                self.pending.complete(&business_correlation_id, payload);
            }
        }
    }
}

/// One pod's return-route engine.
pub struct ReturnRouteCoordinator {
    client: redis::Client,
    config: SyncOverAsyncConfig,
    return_channel: String,
    shared: Arc<Rendezvous>,
    shutdown: watch::Sender<bool>,
}

impl ReturnRouteCoordinator {
    /// Connect the command lane and build the engine for this pod. `origin` is
    /// the pod identity (`Platform::origin()` in an application).
    pub async fn connect(
        settings: &RedisSettings,
        origin: &str,
        config: SyncOverAsyncConfig,
    ) -> Result<Self, AppError> {
        let client = settings.client()?;
        let store = ReturnRouteStore::connect(settings).await?;
        let (shutdown, _) = watch::channel(false);
        Ok(ReturnRouteCoordinator {
            return_channel: format!("{}:{origin}", config.return_channel_prefix()),
            shared: Arc::new(Rendezvous {
                store,
                pending: PendingRequests::new(config.max_pending_requests()),
                streams: PendingStreams::new(config.max_pending_streams()),
            }),
            client,
            config,
            shutdown,
        })
    }

    /// Subscribe to this pod's return channel. Call once at startup; the
    /// subscription is live when this returns, and the spawned task keeps it
    /// live across reconnects.
    pub async fn start(&self) -> Result<(), AppError> {
        let mut pubsub = self
            .client
            .get_async_pubsub()
            .await
            .map_err(|e| AppError::new(500, format!("Unable to subscribe - {e}")))?;
        pubsub
            .subscribe(self.return_channel.as_str())
            .await
            .map_err(|e| AppError::new(500, format!("Unable to subscribe - {e}")))?;
        log::info!(
            "Return-route subscriber listening on {}",
            self.return_channel
        );
        tokio::spawn(subscriber_loop(
            pubsub,
            self.client.clone(),
            self.return_channel.clone(),
            self.shared.clone(),
            self.shutdown.subscribe(),
        ));
        Ok(())
    }

    /// This pod's return channel — `{prefix}:{origin}`.
    pub fn return_channel(&self) -> &str {
        &self.return_channel
    }

    /// Originating pod: register the pending request and publish its return
    /// route. The returned handle is a convenience for a caller that holds it;
    /// [`await_response`](Self::await_response) finds the same entry by cid.
    pub async fn begin(
        &self,
        business_correlation_id: &str,
    ) -> Result<Arc<PendingEntry>, AppError> {
        let entry = self.shared.pending.register(business_correlation_id)?;
        if let Err(e) = self
            .shared
            .store
            .save_route(
                business_correlation_id,
                &self.return_channel,
                self.config.route_ttl_seconds(),
            )
            .await
        {
            self.shared.pending.cancel(business_correlation_id);
            return Err(e);
        }
        Ok(entry)
    }

    /// Originating pod: wait for the response by correlation-id.
    ///
    /// An early response (one that arrived before this call) is covered by the
    /// lookup itself: completion is **in place**, so a completed entry is still
    /// registered until this await collects it. On timeout, one final drain of
    /// the rendezvous queue runs before giving up, and — if that finds nothing
    /// — the entry is checked once more, because a wake-up may have raced the
    /// timeout and popped the only stored copy.
    pub async fn await_response(
        &self,
        business_correlation_id: &str,
        timeout_ms: u64,
    ) -> Result<String, AppError> {
        let Some(entry) = self.shared.pending.get(business_correlation_id) else {
            // no registration to be found (defensive) - a final drain may
            // still recover a stored response
            if let Some(stored) = self.shared.pop_response_body(business_correlation_id).await {
                let _ = self.shared.store.cleanup(business_correlation_id).await;
                return Ok(stored);
            }
            return Err(AppError::new(
                500,
                format!("No pending request for {business_correlation_id}"),
            ));
        };
        let outcome = self
            .collect_response(business_correlation_id, &entry, timeout_ms)
            .await;
        // the sole removal point together with abort(): completion is in place,
        // so the entry (and its capacity slot) is released on every exit path
        self.shared.pending.cancel(business_correlation_id);
        outcome
    }

    async fn collect_response(
        &self,
        business_correlation_id: &str,
        entry: &Arc<PendingEntry>,
        timeout_ms: u64,
    ) -> Result<String, AppError> {
        if let Some(response) = entry.wait(Duration::from_millis(timeout_ms)).await {
            // rendezvous done; free the keys now instead of on TTL
            let _ = self.shared.store.cleanup(business_correlation_id).await;
            return Ok(response);
        }
        if let Some(late) = self.shared.pop_response_body(business_correlation_id).await {
            log::debug!(
                "Recovered response for {business_correlation_id} via final drain (missed notification)"
            );
            let _ = self.shared.store.cleanup(business_correlation_id).await;
            return Ok(late);
        }
        // a wake-up may have raced the timeout: it pops the segment and
        // completes the entry in place, leaving our drain empty - the entry,
        // not Redis, then holds the only copy
        if let Some(raced) = entry.get_now() {
            let _ = self.shared.store.cleanup(business_correlation_id).await;
            return Ok(raced);
        }
        Err(AppError::new(408, format!("Timeout for {timeout_ms} ms")))
    }

    /// Cancel a pending request without waiting for a response — the flow's
    /// fail-fast path, so an entry registered by `begin` cannot leak. Safe to
    /// call when the entry is already gone.
    pub fn abort(&self, business_correlation_id: &str) {
        self.shared.pending.cancel(business_correlation_id);
    }

    /// Responder side (any pod): deliver a one-shot response = post one
    /// terminal segment (store-first, with the one-shot TTL) then wake the
    /// originating pod. Structurally the same act as a stream producer's
    /// closing post — the degenerate stream.
    ///
    /// Returns `false` for an orphan (route expired or unknown correlation-id
    /// — the segment is still queued under its TTL).
    pub async fn deliver(
        &self,
        business_correlation_id: &str,
        response_payload: &str,
    ) -> Result<bool, AppError> {
        let terminal = StreamSegment::of(segment::EOF, None, Some(response_payload))?;
        self.shared
            .store
            .append_segment(
                business_correlation_id,
                &terminal.to_json(),
                self.config.response_ttl_seconds(),
            )
            .await?;
        match self.shared.store.get_route(business_correlation_id).await? {
            Some(channel) => {
                self.shared
                    .store
                    .publish(&channel, business_correlation_id)
                    .await?;
                Ok(true)
            }
            None => {
                log::debug!("Orphan response for {business_correlation_id} - no return route");
                Ok(false)
            }
        }
    }

    /// Originating pod: open a streaming rendezvous. Registers the sink and
    /// publishes the return route with the session-scale streaming TTL, so
    /// producers can discover this pod. The sink receives every drained segment
    /// **including the terminal one** in list order; after the terminal segment
    /// the stream is closed and its keys deleted. If the sink fails, the stream
    /// is closed the same way (the consumer is gone), and producers stop on
    /// their next post.
    pub async fn begin_stream(
        &self,
        business_correlation_id: &str,
        sink: Arc<dyn SegmentSink>,
    ) -> Result<(), AppError> {
        self.shared
            .streams
            .register(business_correlation_id, sink)?;
        if let Err(e) = self
            .shared
            .store
            .save_route(
                business_correlation_id,
                &self.return_channel,
                self.config.stream_ttl_seconds(),
            )
            .await
        {
            self.shared.streams.remove(business_correlation_id);
            return Err(e);
        }
        Ok(())
    }

    /// Originating pod: close a stream from the consumer side — client
    /// disconnect or edge idle expiry. Idempotent (a stream already completed
    /// by its terminal segment is a no-op).
    pub async fn close_stream(&self, business_correlation_id: &str) {
        self.shared.close_stream(business_correlation_id).await;
    }

    /// Originating pod: one last-chance drain of an open stream, for the edge
    /// idle-expiry path — the streaming analogue of the one-shot final read
    /// before timeout, and the same recovery cornerstone: if the final
    /// notification was dropped, the queued segments (terminal included) are
    /// still delivered before the caller fails the render in-band. This is a
    /// single drain, not a periodic sweeper.
    ///
    /// Returns `true` if the drain completed the stream.
    pub async fn final_drain(&self, business_correlation_id: &str) -> bool {
        self.shared.drain_stream(business_correlation_id).await
    }

    /// The number of in-flight one-shot requests on this pod.
    pub fn pending_count(&self) -> usize {
        self.shared.pending.size()
    }

    /// The number of open streaming rendezvous on this pod.
    pub fn active_streams(&self) -> usize {
        self.shared.streams.size()
    }

    /// Stop the subscriber task. Idempotent; also runs on drop.
    pub fn close(&self) {
        let _ = self.shutdown.send(true);
    }
}

impl Drop for ReturnRouteCoordinator {
    fn drop(&mut self) {
        let _ = self.shutdown.send(true);
    }
}

/// Consume wake-ups until shutdown, reconnecting and resubscribing whenever the
/// Pub/Sub connection drops (the `redis` crate does not do this for us).
async fn subscriber_loop(
    mut pubsub: redis::aio::PubSub,
    client: redis::Client,
    channel: String,
    shared: Arc<Rendezvous>,
    mut shutdown: watch::Receiver<bool>,
) {
    loop {
        {
            let mut messages = pubsub.on_message();
            loop {
                tokio::select! {
                    _ = shutdown.changed() => return,
                    message = messages.next() => match message {
                        Some(message) => {
                            if let Ok(business_correlation_id) = message.get_payload::<String>() {
                                let shared = shared.clone();
                                // off the subscriber task: a drain is a
                                // sequence of blocking round trips
                                tokio::spawn(async move {
                                    shared.on_wake_up(&business_correlation_id).await;
                                });
                            }
                        }
                        None => break, // the connection dropped
                    }
                }
            }
        }
        match resubscribe(&client, &channel, &mut shutdown).await {
            Some(fresh) => {
                pubsub = fresh;
                // heal whatever was missed while the subscription was down
                shared.recovery_pass().await;
            }
            None => return,
        }
    }
}

async fn resubscribe(
    client: &redis::Client,
    channel: &str,
    shutdown: &mut watch::Receiver<bool>,
) -> Option<redis::aio::PubSub> {
    loop {
        if *shutdown.borrow() {
            return None;
        }
        tokio::select! {
            _ = shutdown.changed() => return None,
            _ = tokio::time::sleep(RECONNECT_DELAY) => {}
        }
        if let Ok(mut fresh) = client.get_async_pubsub().await {
            if fresh.subscribe(channel).await.is_ok() {
                log::info!("Return-route subscriber re-subscribed to {channel}");
                return Some(fresh);
            }
        }
    }
}
