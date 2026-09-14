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

//! Process-wide holder for the singletons the Kafka building blocks share
//! (Java `KafkaRuntime`): the publisher used by `simple.kafka.notification`.
//! Populated once at startup by the library's auto-start entry point.

use std::sync::{Arc, RwLock};

use crate::consumer::KafkaFlowConsumer;
use crate::publisher::KafkaRequestPublisher;

static PUBLISHER: RwLock<Option<Arc<KafkaRequestPublisher>>> = RwLock::new(None);
static FLOW_CONSUMERS: RwLock<Vec<KafkaFlowConsumer>> = RwLock::new(Vec::new());

/// Install the shared publisher (the auto-start entry point; tests install
/// their own against a mock cluster).
pub fn set_publisher(publisher: Arc<KafkaRequestPublisher>) {
    *PUBLISHER.write().expect("kafka runtime poisoned") = Some(publisher);
}

/// The shared publisher, or `None` when the producer is switched off.
pub fn publisher() -> Option<Arc<KafkaRequestPublisher>> {
    PUBLISHER.read().expect("kafka runtime poisoned").clone()
}

/// Release the shared publisher (test lifecycle).
pub fn clear_publisher() {
    PUBLISHER.write().expect("kafka runtime poisoned").take();
}

/// Install the running flow-adapter consumers (the auto-start entry point).
pub fn set_flow_consumers(consumers: Vec<KafkaFlowConsumer>) {
    *FLOW_CONSUMERS.write().expect("kafka runtime poisoned") = consumers;
}

/// Stop every running binding consumer after its in-flight record completes.
pub fn stop_flow_consumers() {
    for consumer in FLOW_CONSUMERS
        .read()
        .expect("kafka runtime poisoned")
        .iter()
    {
        consumer.close();
    }
}
