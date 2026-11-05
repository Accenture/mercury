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

//! The relay route of one node in a watched run (Java `ProgressRelay`): the `x-return-path` the
//! engine passes to the node's function or synchronous subgraph. It hands each progress event to
//! the run's channel, which frames it for the caller's transport. One instance, so a sender's
//! events keep their order through it, and a drain request is answered only after every event
//! queued before it.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use platform_core::{AppError, ComposableFunction, EventEnvelope};
use rmpv::Value;

use super::return_channel::{ReturnChannel, DRAIN};

pub(crate) struct ProgressRelay {
    channel: Arc<ReturnChannel>,
    node: String,
}

impl ProgressRelay {
    pub(crate) fn new(channel: Arc<ReturnChannel>, node: &str) -> Self {
        ProgressRelay {
            channel,
            node: node.to_string(),
        }
    }
}

#[async_trait]
impl ComposableFunction for ProgressRelay {
    async fn handle_event(
        &self,
        headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        if headers.contains_key(DRAIN) {
            // every event queued before this request has been handed to the channel
            return EventEnvelope::new().set_body(true);
        }
        if let Value::Map(_) = input.body() {
            self.channel.relay(&self.node, input.body()).await;
        }
        Ok(EventEnvelope::new())
    }
}
