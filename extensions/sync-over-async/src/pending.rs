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

//! The per-pod registries of in-flight rendezvous — Rust port of the Java
//! `PendingRequests` and `PendingStreams`.
//!
//! Both bound their growth with the same rule as Java, by a simpler vehicle:
//! Java reserves a slot on an `AtomicInteger` *before* the map write so two
//! threads cannot both pass the cap check, while here the capacity check and
//! the insert happen under one mutex — so oversubscription is impossible and a
//! rejected registration inherently consumes nothing.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use platform_core::AppError;
use tokio::sync::Notify;

use crate::segment::StreamSegment;

/// The consumer end of a streaming rendezvous: every drained segment is
/// forwarded here in list order, **including the terminal one**.
///
/// An `Err` means the consumer is gone (the HTTP edge disconnected), and the
/// drain closes the stream — producers then stop on their next post.
#[async_trait]
pub trait SegmentSink: Send + Sync {
    async fn accept(&self, segment: StreamSegment) -> Result<(), AppError>;
}

/// One in-flight one-shot request. Completion is **in place**: the entry stays
/// registered (holding its capacity slot) until the awaiting or aborting path
/// removes it, so an await-by-cid that runs after the response arrived still
/// finds the completed value. The response segment is destructively popped
/// from Redis before completion, so this is the only remaining copy.
#[derive(Debug)]
pub struct PendingEntry {
    value: Mutex<Option<String>>,
    arrived: Notify,
}

impl Default for PendingEntry {
    fn default() -> Self {
        PendingEntry {
            value: Mutex::new(None),
            arrived: Notify::new(),
        }
    }
}

impl PendingEntry {
    /// Complete this request in place. Idempotent: `false` means it was
    /// already completed (a duplicate or late response).
    pub fn complete(&self, response: String) -> bool {
        let mut slot = self.value.lock().expect("pending value");
        if slot.is_some() {
            return false;
        }
        *slot = Some(response);
        drop(slot);
        self.arrived.notify_waiters();
        true
    }

    /// The response if it has already arrived, without waiting.
    pub fn get_now(&self) -> Option<String> {
        self.value.lock().expect("pending value").clone()
    }

    /// Wait up to `duration` for the response; `None` on timeout.
    pub async fn wait(&self, duration: Duration) -> Option<String> {
        // register for the wake-up BEFORE reading the slot, so a completion
        // racing this call cannot fall between the two
        let arrived = self.arrived.notified();
        tokio::pin!(arrived);
        if let Some(response) = self.get_now() {
            return Some(response);
        }
        match tokio::time::timeout(duration, arrived).await {
            Ok(()) => self.get_now(),
            Err(_) => None,
        }
    }
}

/// Per-pod registry of in-flight synchronous requests, keyed by correlation-id.
#[derive(Default)]
pub struct PendingRequests {
    pending: Mutex<HashMap<String, Arc<PendingEntry>>>,
    max_pending: usize,
}

impl PendingRequests {
    pub fn new(max_pending: usize) -> Self {
        PendingRequests {
            pending: Mutex::new(HashMap::new()),
            max_pending,
        }
    }

    /// Register a pending request. Call before publishing the request to the
    /// asynchronous backend.
    ///
    /// Returns HTTP-503 at capacity and HTTP-409 for a correlation-id that is
    /// already in flight; neither consumes a slot.
    pub fn register(&self, business_correlation_id: &str) -> Result<Arc<PendingEntry>, AppError> {
        let mut pending = self.pending.lock().expect("pending requests");
        if pending.len() >= self.max_pending {
            return Err(AppError::new(
                503,
                format!("Too many pending requests (max {})", self.max_pending),
            ));
        }
        if pending.contains_key(business_correlation_id) {
            return Err(AppError::new(
                409,
                format!("Duplicate correlation-id in flight: {business_correlation_id}"),
            ));
        }
        let entry = Arc::new(PendingEntry::default());
        pending.insert(business_correlation_id.to_string(), entry.clone());
        Ok(entry)
    }

    /// Complete the waiting request **in place**. `false` for an orphan,
    /// duplicate, or already-completed response.
    pub fn complete(&self, business_correlation_id: &str, response: String) -> bool {
        let entry = self
            .pending
            .lock()
            .expect("pending requests")
            .get(business_correlation_id)
            .cloned();
        match entry {
            Some(entry) => entry.complete(response),
            None => false,
        }
    }

    /// Look up without removing — the await-by-cid path (begin and await run
    /// as separate flow tasks).
    pub fn get(&self, business_correlation_id: &str) -> Option<Arc<PendingEntry>> {
        self.pending
            .lock()
            .expect("pending requests")
            .get(business_correlation_id)
            .cloned()
    }

    /// Drop a pending request and release its slot, exactly once.
    pub fn cancel(&self, business_correlation_id: &str) {
        self.pending
            .lock()
            .expect("pending requests")
            .remove(business_correlation_id);
    }

    pub fn is_pending(&self, business_correlation_id: &str) -> bool {
        self.pending
            .lock()
            .expect("pending requests")
            .contains_key(business_correlation_id)
    }

    pub fn size(&self) -> usize {
        self.pending.lock().expect("pending requests").len()
    }

    /// Every registered correlation-id — the recovery pass after a Pub/Sub
    /// resubscribe walks these.
    pub fn ids(&self) -> Vec<String> {
        self.pending
            .lock()
            .expect("pending requests")
            .keys()
            .cloned()
            .collect()
    }
}

/// One open stream: the segment sink plus the drain-serialization flag.
pub struct StreamEntry {
    sink: Arc<dyn SegmentSink>,
    draining: AtomicBool,
}

impl std::fmt::Debug for StreamEntry {
    /// The sink is caller-supplied and not required to be `Debug`, so it is
    /// summarized rather than rendered.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamEntry")
            .field("draining", &self.draining)
            .finish_non_exhaustive()
    }
}

impl StreamEntry {
    pub fn sink(&self) -> &Arc<dyn SegmentSink> {
        &self.sink
    }

    /// True when this caller now owns the (single) drain loop for the stream.
    pub fn try_acquire_drain(&self) -> bool {
        self.draining
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub fn release_drain(&self) {
        self.draining.store(false, Ordering::Release);
    }
}

/// Per-pod registry of open streaming rendezvous — the streaming sibling of
/// [`PendingRequests`]. Each entry holds the consumer's sink and the
/// per-stream drain flag that keeps drains **serialized per cid**: several
/// producers may wake the pod concurrently, but only one drain loop forwards
/// at a time, so forward order equals list order.
#[derive(Default)]
pub struct PendingStreams {
    streams: Mutex<HashMap<String, Arc<StreamEntry>>>,
    max_pending: usize,
}

impl PendingStreams {
    pub fn new(max_pending: usize) -> Self {
        PendingStreams {
            streams: Mutex::new(HashMap::new()),
            max_pending,
        }
    }

    /// Register an open stream. Call before any producer can post (that is,
    /// before handing out the cid).
    ///
    /// Returns HTTP-503 at capacity and HTTP-409 for a correlation-id that is
    /// already open; neither consumes a slot.
    pub fn register(
        &self,
        business_correlation_id: &str,
        sink: Arc<dyn SegmentSink>,
    ) -> Result<Arc<StreamEntry>, AppError> {
        let mut streams = self.streams.lock().expect("pending streams");
        if streams.len() >= self.max_pending {
            return Err(AppError::new(
                503,
                format!("Too many open streams (max {})", self.max_pending),
            ));
        }
        if streams.contains_key(business_correlation_id) {
            return Err(AppError::new(
                409,
                format!("Duplicate correlation-id in flight: {business_correlation_id}"),
            ));
        }
        let entry = Arc::new(StreamEntry {
            sink,
            draining: AtomicBool::new(false),
        });
        streams.insert(business_correlation_id.to_string(), entry.clone());
        Ok(entry)
    }

    pub fn get(&self, business_correlation_id: &str) -> Option<Arc<StreamEntry>> {
        self.streams
            .lock()
            .expect("pending streams")
            .get(business_correlation_id)
            .cloned()
    }

    pub fn contains(&self, business_correlation_id: &str) -> bool {
        self.streams
            .lock()
            .expect("pending streams")
            .contains_key(business_correlation_id)
    }

    /// Close the entry and release its slot, exactly once (idempotent).
    pub fn remove(&self, business_correlation_id: &str) {
        self.streams
            .lock()
            .expect("pending streams")
            .remove(business_correlation_id);
    }

    pub fn size(&self) -> usize {
        self.streams.lock().expect("pending streams").len()
    }

    /// Every open correlation-id — the recovery pass after a Pub/Sub
    /// resubscribe drains each once.
    pub fn ids(&self) -> Vec<String> {
        self.streams
            .lock()
            .expect("pending streams")
            .keys()
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NoOpSink;

    #[async_trait]
    impl SegmentSink for NoOpSink {
        async fn accept(&self, _segment: StreamSegment) -> Result<(), AppError> {
            Ok(())
        }
    }

    fn sink() -> Arc<dyn SegmentSink> {
        Arc::new(NoOpSink)
    }

    #[tokio::test]
    async fn register_then_complete_resolves_the_request_in_place() {
        let requests = PendingRequests::new(100);
        let entry = requests.register("cid-1").expect("registered");
        assert!(requests.complete("cid-1", "the answer".to_string()));
        // completion does NOT remove the entry: an await-by-cid that starts
        // after the response arrived must still find it
        assert_eq!(1, requests.size());
        assert!(requests.is_pending("cid-1"));
        assert_eq!(Some("the answer".to_string()), entry.get_now());
        assert_eq!(
            Some("the answer".to_string()),
            entry.wait(Duration::from_millis(10)).await,
            "an already-completed entry resolves without waiting"
        );
    }

    #[test]
    fn capacity_is_released_on_cancel_not_on_complete() {
        let requests = PendingRequests::new(1);
        requests.register("cid-1").expect("registered");
        assert!(requests.complete("cid-1", "done".to_string()));
        // the slot is still held - completion is in place
        assert_eq!(
            503,
            requests
                .register("cid-2")
                .expect_err("at capacity")
                .status()
        );
        requests.cancel("cid-1");
        requests
            .register("cid-3")
            .expect("the freed slot is usable");
        assert_eq!(1, requests.size());
    }

    #[test]
    fn completion_is_idempotent_and_orphan_safe() {
        let requests = PendingRequests::new(100);
        requests.register("cid-1").expect("registered");
        assert!(requests.complete("cid-1", "first".to_string()));
        assert!(
            !requests.complete("cid-1", "second".to_string()),
            "a duplicate or late response is a no-op"
        );
        assert!(
            !requests.complete("nobody", "orphan".to_string()),
            "an unknown correlation-id is a no-op"
        );
    }

    #[test]
    fn rejects_a_duplicate_correlation_id() {
        let requests = PendingRequests::new(100);
        requests.register("cid-1").expect("registered");
        assert_eq!(
            409,
            requests.register("cid-1").expect_err("rejected").status()
        );
        assert_eq!(1, requests.size(), "the rejected attempt consumed nothing");
    }

    #[tokio::test]
    async fn wait_times_out_without_a_response() {
        let requests = PendingRequests::new(100);
        let entry = requests.register("cid-1").expect("registered");
        assert_eq!(None, entry.wait(Duration::from_millis(20)).await);
    }

    #[test]
    fn streams_register_and_remove_idempotently() {
        let streams = PendingStreams::new(100);
        streams.register("cid-1", sink()).expect("registered");
        assert!(streams.contains("cid-1"));
        assert_eq!(1, streams.size());
        streams.remove("cid-1");
        assert!(!streams.contains("cid-1"));
        assert!(streams.get("cid-1").is_none());
        streams.remove("cid-1"); // idempotent
        assert_eq!(0, streams.size());
    }

    #[test]
    fn enforces_max_open_streams_and_never_leaks_a_slot() {
        let streams = PendingStreams::new(1);
        streams.register("cid-1", sink()).expect("registered");
        // repeated over-cap attempts must not consume anything
        assert_eq!(
            503,
            streams
                .register("cid-2", sink())
                .expect_err("at capacity")
                .status()
        );
        assert_eq!(
            503,
            streams
                .register("cid-2", sink())
                .expect_err("at capacity")
                .status()
        );
        streams.remove("cid-1");
        streams.remove("cid-1"); // a double close must not widen the cap
        streams
            .register("cid-3", sink())
            .expect("the freed slot is usable");
        assert_eq!(
            503,
            streams
                .register("cid-4", sink())
                .expect_err("at capacity")
                .status()
        );
        assert_eq!(1, streams.size());
    }

    #[test]
    fn rejects_a_duplicate_open_stream() {
        let streams = PendingStreams::new(100);
        streams.register("cid-1", sink()).expect("registered");
        assert_eq!(
            409,
            streams
                .register("cid-1", sink())
                .expect_err("rejected")
                .status()
        );
    }

    #[test]
    fn the_drain_flag_is_exclusive_until_released() {
        let streams = PendingStreams::new(100);
        let entry = streams.register("cid-1", sink()).expect("registered");
        assert!(
            entry.try_acquire_drain(),
            "first caller owns the drain loop"
        );
        assert!(
            !entry.try_acquire_drain(),
            "a concurrent wake-up must not start a second drain"
        );
        entry.release_drain();
        assert!(
            entry.try_acquire_drain(),
            "a released flag can be re-acquired by the next wake-up"
        );
    }
}
