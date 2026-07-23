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

//! The RPC reply listener — Rust port of Java's **`TemporaryInbox`**
//! (`org.platformlambda.core.services.TemporaryInbox` + `InboxBase.getHolder`):
//! **one reserved private route**, `temporary.inbox`, receives every RPC reply
//! and resolves the waiting caller through a **correlation-id-keyed registry**
//! — an RPC costs one map insert/remove, and no per-request route or route
//! prefix is ever claimed, so the `inbox.*` namespace belongs entirely to
//! applications (workflow apps commonly use routes like `inbox.approval` for
//! human-operator staging areas).
//!
//! `PostOffice::request` registers a unique correlation id here and sends the
//! request with `reply_to = temporary.inbox` and that id as the envelope `cid`
//! (the caller's original correlation id is restored on the reply — Java
//! `AsyncInbox.originalCid`). Any delivery addressed to `temporary.inbox` —
//! the worker's automatic reply *or* a manual `po.send(reply_to)` (the
//! `#[event_interceptor]` pattern) — reaches this service like any other
//! function, and it completes the caller's oneshot. A late reply after timeout
//! finds the entry gone and is dropped silently (Java parity). For the
//! fork-n-join multi-inbox case the correlation id is composite `{cid}-{seq}`;
//! the lookup strips after the LAST `-` exactly like Java (single-inbox ids
//! are dash-less uuids, so they never split).
//!
//! The route is registered at platform construction (and asserted by the
//! lifecycle's essential-service step, the Java `EssentialServiceLoader`
//! analog) — private, zero-tracing, 500 instances.
//!
//! **Deliberate divergences from Java** (doc'd, not silent):
//! - Java's reply address is origin-qualified (`temporary.inbox@{origin}`)
//!   because the `route@origin` syntax selects a target instance under the
//!   legacy Kafka service mesh. That burden is not carried here (Eric's
//!   ruling): the reply address is the plain route name; an inbound
//!   `@origin` suffix (e.g. from a Java peer's envelope) is parsed away and
//!   never generated.
//! - Dispatch to this route is DIRECT on the sender's runtime (the reserved
//!   engine-route path), like `event.script.manager`/`task.executor`: the
//!   registered workers are the route's addressable identity, but a
//!   multi-runtime process (test binaries) cannot rely on their liveness,
//!   and a reply must always complete.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use async_trait::async_trait;
use tokio::sync::oneshot;

use crate::envelope::EventEnvelope;
use crate::function::{AppError, ComposableFunction};

/// The reserved RPC reply-listener route (Java `TemporaryInbox.TEMPORARY_INBOX`).
pub const TEMPORARY_INBOX: &str = "temporary.inbox";

fn registry() -> &'static Mutex<HashMap<String, oneshot::Sender<EventEnvelope>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, oneshot::Sender<EventEnvelope>>>> =
        OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Create a new pending RPC entry: returns its unique correlation id (a
/// dash-less uuid — it must never contain `-`, see the composite-id split)
/// and the receiving end for the caller to await.
pub(crate) fn open() -> (String, oneshot::Receiver<EventEnvelope>) {
    let cid = uuid::Uuid::new_v4().simple().to_string();
    let (tx, rx) = oneshot::channel();
    registry()
        .lock()
        .expect("inbox registry poisoned")
        .insert(cid.clone(), tx);
    (cid, rx)
}

/// Close a pending entry without a reply (timeout / send failure cleanup).
pub(crate) fn close(cid: &str) {
    registry()
        .lock()
        .expect("inbox registry poisoned")
        .remove(cid);
}

/// The `temporary.inbox` service (Java `TemporaryInbox`): resolves the reply's
/// correlation id — stripping a composite `{cid}-{seq}` suffix on the last
/// `-`, Java parity — and hands the envelope to the waiting caller. A missing
/// entry (the caller timed out) drops the reply silently — the correct
/// outcome. The worker delivers the envelope to this route PRISTINE (no
/// metadata scrubbing or injection — see `worker_loop`), because the envelope
/// IS the payload the caller receives.
pub struct TemporaryInbox;

#[async_trait]
impl ComposableFunction for TemporaryInbox {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if let Some(composite) = input.correlation_id() {
            // for the multi-inbox fork-n-join case the correlation id is
            // composite "{cid}-{seq}" — strip after the LAST '-' (Java
            // TemporaryInbox; single-inbox ids are dash-less, never split)
            let cid = match composite.rfind('-') {
                Some(sep) => &composite[..sep],
                None => composite,
            };
            let sender = registry()
                .lock()
                .expect("inbox registry poisoned")
                .remove(cid);
            if let Some(sender) = sender {
                let _ = sender.send(input);
            }
        }
        Ok(EventEnvelope::new())
    }
}
