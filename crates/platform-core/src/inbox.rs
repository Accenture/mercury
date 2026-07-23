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

//! The lightweight RPC inbox — the Rust analog of Java's **TemporaryInbox
//! pattern** (`org.platformlambda.core.services.TemporaryInbox` +
//! `InboxBase.getHolder`): a reply is resolved through a **correlation map**,
//! not a per-request route registration, so an RPC costs one map insert/remove
//! instead of a manager task + workers + elastic queue per request.
//!
//! `PostOffice::request` registers a unique `inbox.<uuid>` id here and sets it
//! as the envelope's `reply_to`; any delivery addressed to an `inbox.` id —
//! the worker's automatic reply *or* a manual `po.send(reply_to)` (the
//! `@EventInterceptor` pattern) — completes the oneshot. A late reply after
//! timeout finds the entry gone and is dropped silently (Java parity).
//!
//! **Deliberate divergences from Java** (doc'd, not silent):
//! - Java routes every reply through one **shared registered route**
//!   (`temporary.inbox@<origin>`, 500 instances) and correlates by a
//!   **sequenced cid** that temporarily replaces the business cid
//!   (`InboxCorrelation` restores it). The Rust port correlates by the unique
//!   `reply_to` id itself, so the **business correlation-id rides through
//!   untouched** — one less moving part; the composite `cid-seq` machinery is
//!   only needed for fork-n-join multi-inboxes (deferred, §7).
//! - The reply completes the oneshot at the delivery boundary instead of
//!   dispatching through an inbox function — same observable behavior, one
//!   less hop. Java's origin-qualified reply address (`@origin`) is a
//!   service-mesh concern, out of scope with the mesh.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use tokio::sync::oneshot;

use crate::envelope::EventEnvelope;

/// Reply-route ids with this prefix are lightweight inboxes, not routes.
pub(crate) const INBOX_PREFIX: &str = "inbox.";

fn registry() -> &'static Mutex<HashMap<String, oneshot::Sender<EventEnvelope>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, oneshot::Sender<EventEnvelope>>>> =
        OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Create a new inbox: returns its unique id (already `inbox.`-prefixed) and
/// the receiving end for the caller to await.
pub(crate) fn open() -> (String, oneshot::Receiver<EventEnvelope>) {
    let id = format!("{INBOX_PREFIX}{}", uuid::Uuid::new_v4().simple());
    let (tx, rx) = oneshot::channel();
    registry()
        .lock()
        .expect("inbox registry poisoned")
        .insert(id.clone(), tx);
    (id, rx)
}

/// Deliver a reply to an inbox. A missing entry (caller timed out and closed
/// the inbox) drops the reply silently — the correct outcome. Returns whether
/// the reply actually reached a waiting caller — the Java
/// `ProcessStatus.isNotDelivered` signal, which reopens the worker-side
/// telemetry record for an RPC-served execution (otherwise the caller's
/// `round_trip` record is THE record for the span and the worker stays quiet).
pub(crate) fn deliver(id: &str, reply: EventEnvelope) -> bool {
    let sender = registry()
        .lock()
        .expect("inbox registry poisoned")
        .remove(id);
    match sender {
        Some(sender) => sender.send(reply).is_ok(),
        None => false,
    }
}

/// Close an inbox without a reply (timeout / send failure cleanup).
pub(crate) fn close(id: &str) {
    registry()
        .lock()
        .expect("inbox registry poisoned")
        .remove(id);
}
