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

//! The lightweight RPC inbox — Rust analog of the Java `AsyncInbox`
//! (`org.platformlambda.core.models.InboxBase`), which deliberately
//! **bypasses the `ServiceQueue` machinery**: a reply inbox is a one-shot
//! correlation entry, not a managed route, so an RPC costs one map
//! insert/remove instead of a route registration (manager task + workers +
//! elastic queue) per request.
//!
//! `PostOffice::request` registers a unique `inbox.<uuid>` id here and sets it
//! as the envelope's `reply_to`; the route worker's reply delivery recognizes
//! the `inbox.` prefix and completes the oneshot directly. A late reply after
//! timeout finds the entry gone and is dropped silently (Java parity).

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
/// the inbox) drops the reply silently — the correct outcome.
pub(crate) fn deliver(id: &str, reply: EventEnvelope) {
    let sender = registry()
        .lock()
        .expect("inbox registry poisoned")
        .remove(id);
    if let Some(sender) = sender {
        let _ = sender.send(reply);
    }
}

/// Close an inbox without a reply (timeout / send failure cleanup).
pub(crate) fn close(id: &str) {
    registry()
        .lock()
        .expect("inbox registry poisoned")
        .remove(id);
}
