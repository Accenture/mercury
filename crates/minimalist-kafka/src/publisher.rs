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

//! Thread-safe wrapper around the shared Kafka producer (Java
//! `KafkaRequestPublisher`). `publish` resolves when the broker acknowledges
//! the record — a caller that awaits through `po.request` (RPC) learns
//! whether publishing succeeded and fails fast on error, while a `po.send`
//! (async) caller simply does not observe it; delivery failures are always
//! logged, so a failed publish is visible either way (the Java `Mono`
//! contract, carried by an async fn).

use std::collections::HashMap;
use std::time::Duration;

use platform_core::AppError;
use rdkafka::error::KafkaError;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::util::Timeout;

/// How long a send may wait for local queue capacity before failing —
/// librdkafka's own delivery timeouts govern the broker leg.
const ENQUEUE_TIMEOUT: Duration = Duration::from_secs(10);

/// The shared producer handle used by `simple.kafka.notification` (and, from
/// K2, the dead-letter writer).
pub struct KafkaRequestPublisher {
    producer: FutureProducer,
}

/// A [`KafkaRequestPublisher::flush`] that ran out of time: `undelivered`
/// records were still waiting for the broker when the grace ended.
#[derive(Debug)]
pub struct FlushIncomplete {
    pub undelivered: usize,
    pub cause: KafkaError,
}

impl std::fmt::Display for FlushIncomplete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} message(s) undelivered - {}",
            self.undelivered, self.cause
        )
    }
}

impl std::error::Error for FlushIncomplete {}

impl KafkaRequestPublisher {
    pub fn new(producer: FutureProducer) -> Self {
        KafkaRequestPublisher { producer }
    }

    /// Deliver every record accepted so far, waiting at most `timeout` for the
    /// broker's acknowledgements — the shutdown path (`runtime::close_publisher`).
    /// The wait runs through the client's linger (`linger.ms`, 5 ms by default):
    /// the crate's flush cannot cut a linger short, so a template with a long
    /// linger flushes no faster than that linger. A timeout reports how many
    /// records were still undelivered, so the caller can decide whether a
    /// stopping process waits any longer.
    pub fn flush(&self, timeout: Duration) -> Result<(), FlushIncomplete> {
        self.producer
            .flush(Timeout::After(timeout))
            .map_err(|cause| FlushIncomplete {
                undelivered: self.in_flight_count(),
                cause,
            })
    }

    /// Records accepted by [`publish`](Self::publish) whose delivery report has
    /// not arrived yet.
    pub fn in_flight_count(&self) -> usize {
        usize::try_from(self.producer.in_flight_count()).unwrap_or(0)
    }

    /// Publish one record and await the broker acknowledgement. `partition`
    /// `None` lets the configured partitioner choose (default
    /// `murmur2_random` — uniform random for these keyless records); a `None`
    /// body is a Kafka tombstone.
    pub async fn publish(
        &self,
        topic: &str,
        partition: Option<i32>,
        headers: HashMap<String, Vec<u8>>,
        body: Option<Vec<u8>>,
    ) -> Result<(), AppError> {
        let mut record: FutureRecord<'_, str, Vec<u8>> = FutureRecord::to(topic);
        if let Some(chosen) = partition {
            record = record.partition(chosen);
        }
        if let Some(payload) = &body {
            record = record.payload(payload);
        }
        let mut kafka_headers = OwnedHeaders::new();
        for (key, value) in &headers {
            kafka_headers = kafka_headers.insert(Header {
                key,
                value: Some(value),
            });
        }
        record = record.headers(kafka_headers);
        self.producer
            .send(record, ENQUEUE_TIMEOUT)
            .await
            .map(|_| ())
            .map_err(|(error, _)| {
                log::error!("Failed to publish to topic {topic}: {error}");
                AppError::new(500, format!("Failed to publish to topic {topic} - {error}"))
            })
    }

    /// The topic's partition count from the producer's own metadata view (no
    /// admin client). Bounds an application-supplied partition number — an
    /// out-of-range explicit partition does not fail fast on send. A stale
    /// count is conservative: partition counts only ever grow.
    pub fn partition_count(&self, topic: &str, timeout: Duration) -> Result<usize, AppError> {
        let metadata = self
            .producer
            .client()
            .fetch_metadata(Some(topic), Timeout::After(timeout))
            .map_err(|e| {
                AppError::new(500, format!("Unable to fetch metadata for {topic} - {e}"))
            })?;
        Ok(metadata
            .topics()
            .first()
            .map(|t| t.partitions().len())
            .unwrap_or(0))
    }
}
