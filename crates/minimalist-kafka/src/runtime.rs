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
//! (Java `KafkaRuntime`): the publisher used by `simple.kafka.notification`,
//! the Schema Registry codec (when `schema.registry.url` is set) and the
//! running flow consumers. Populated once at startup by the library's
//! auto-start entry point.

use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::consumer::KafkaFlowConsumer;
use crate::publisher::KafkaRequestPublisher;
use crate::schema::SchemaCodec;

static PUBLISHER: RwLock<Option<Arc<KafkaRequestPublisher>>> = RwLock::new(None);
static SCHEMA_CODEC: RwLock<Option<Arc<SchemaCodec>>> = RwLock::new(None);
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

/// Install the shared Schema Registry codec (the auto-start entry point, when
/// `schema.registry.url` is configured; tests install their own against an
/// embedded registry).
pub fn set_schema_codec(codec: Arc<SchemaCodec>) {
    *SCHEMA_CODEC.write().expect("kafka runtime poisoned") = Some(codec);
}

/// The shared codec, or `None` when schema features are off.
pub fn schema_codec() -> Option<Arc<SchemaCodec>> {
    SCHEMA_CODEC.read().expect("kafka runtime poisoned").clone()
}

/// Release the shared codec (test lifecycle).
pub fn clear_schema_codec() {
    SCHEMA_CODEC.write().expect("kafka runtime poisoned").take();
}

/// Install the running flow-adapter consumers (the auto-start entry point).
pub fn set_flow_consumers(consumers: Vec<KafkaFlowConsumer>) {
    *FLOW_CONSUMERS.write().expect("kafka runtime poisoned") = consumers;
}

/// How long a stop waits for the binding consumers to finish their in-flight
/// records before the process goes on shutting down (a Kubernetes pod's default
/// termination grace is 30 s; a record still in flight after this redelivers).
const SHUTDOWN_GRACE: Duration = Duration::from_secs(10);

/// Stop every running binding consumer and wait — bounded by
/// [`SHUTDOWN_GRACE`] — for each to finish its in-flight record, so a stop
/// (Ctrl-C, `SIGTERM`) commits the last record and leaves the group explicitly
/// instead of abandoning work mid-flow (Java parity: the running flag is
/// honoured per iteration and the JVM waits for the thread). Runs from the
/// platform's shutdown hook on the entry point's thread, while the consumer
/// tasks finish on the runtime's workers. Returns how many consumers were still
/// running when the grace period ended (their records redeliver: at-least-once).
pub fn stop_flow_consumers() -> usize {
    let consumers = FLOW_CONSUMERS.read().expect("kafka runtime poisoned");
    if consumers.is_empty() {
        return 0;
    }
    for consumer in consumers.iter() {
        consumer.close();
    }
    let deadline = Instant::now() + SHUTDOWN_GRACE;
    loop {
        let running = consumers.iter().filter(|c| !c.is_stopped()).count();
        if running == 0 {
            log::info!("Kafka flow consumers stopped");
            return 0;
        }
        if Instant::now() >= deadline {
            log::warn!(
                "{running} Kafka flow consumer(s) still running {} s after the stop - their in-flight \
                 records redeliver (at-least-once)",
                SHUTDOWN_GRACE.as_secs()
            );
            return running;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}
