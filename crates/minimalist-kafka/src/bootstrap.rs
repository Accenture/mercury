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

//! Autoloads the Kafka building blocks at startup (Java `KafkaFlowAutoStart`).
//! Runs as a main-application entry point — after the platform has registered
//! every composable function — collected automatically from this library by
//! the application's `auto_start_main!()` (the Java classpath-scan parity).
//!
//! This increment (K1) builds the shared producer; the inbound flow adapter
//! (`yaml.kafka.flow.adapter`) arrives in a later increment of the port.

use std::sync::Arc;

use async_trait::async_trait;
use platform_core::{main_application, AppConfigReader, AppError, EntryPoint, Platform};
use rdkafka::producer::FutureProducer;

use std::time::Duration;

use platform_core::ConfigReader;

use crate::adapter;
use crate::client_config::{self, CONSUMER_ENABLED, PRODUCER_ENABLED};
use crate::consumer::{KafkaFlowConsumer, RetryPolicy};
use crate::publisher::KafkaRequestPublisher;
use crate::{notification, runtime};

const ADAPTER_CONFIG: &str = "yaml.kafka.flow.adapter";
const DLQ_TIMEOUT: &str = "kafka.dlq.timeout.ms";
const MAX_RETRIES: &str = "kafka.flow.max.retries";
const RETRY_BACKOFF: &str = "kafka.flow.retry.backoff.ms";

/// The library's startup hook: sequence 20 keeps it after a typical
/// application's own entry point (default 10) — order is not load-bearing,
/// but a deterministic one keeps startup logs stable.
#[main_application(sequence = 20)]
#[derive(Default)]
pub struct KafkaAutoStart;

#[async_trait]
impl EntryPoint for KafkaAutoStart {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let config = AppConfigReader::get_instance();
        let producer_enabled = client_config::producer_enabled();
        let consumer_enabled = client_config::consumer_enabled();
        if !producer_enabled && !consumer_enabled {
            // a legitimate "Kafka off in this profile" switch - stated loudly
            log::warn!("Kafka is inert - both {PRODUCER_ENABLED} and {CONSUMER_ENABLED} are false");
        }
        if producer_enabled {
            let producer: FutureProducer = client_config::producer_client_config()?
                .create()
                .map_err(|e| AppError::new(500, format!("Unable to build Kafka producer - {e}")))?;
            runtime::set_publisher(Arc::new(KafkaRequestPublisher::new(producer)));
            log::info!(
                "Kafka producer started - [{}] is available",
                notification::ROUTE
            );
        } else {
            log::info!(
                "{PRODUCER_ENABLED}=false; Kafka producer not started - [{}] is unavailable",
                notification::ROUTE
            );
        }
        if !consumer_enabled {
            log::info!("{CONSUMER_ENABLED}=false; Kafka flow adapter not started");
        } else if let Some(adapter_location) = config.get_property(ADAPTER_CONFIG) {
            start_flow_adapter(&adapter_location).await?;
        } else {
            log::info!("{ADAPTER_CONFIG} not set; Kafka flow adapter not started");
        }
        Ok(())
    }
}

/// Start one consumer per validated binding (the Java `KafkaFlowAdapter`
/// start): parse + validate the YAML (fail-fast), enforce the
/// dead-letter-needs-producer guard, build each binding's consumer from the
/// template with the pinned delivery-mode overlay, and launch the poll loops.
async fn start_flow_adapter(adapter_location: &str) -> Result<(), AppError> {
    let config = AppConfigReader::get_instance();
    let reader = ConfigReader::load(adapter_location).map_err(|e| {
        AppError::new(
            500,
            format!("Unable to read {ADAPTER_CONFIG} at {adapter_location} - {e}"),
        )
    })?;
    let bindings = adapter::parse_bindings(&reader)?;
    let publisher = runtime::publisher();
    if publisher.is_none() {
        // no producer to dead-letter through: a binding's dlq-topic would
        // silently drop messages - the contradiction fails the deployment
        adapter::reject_dead_letter_without_producer(&bindings, PRODUCER_ENABLED)?;
    }
    let dlq_timeout = Duration::from_millis(
        config
            .get_property_or(DLQ_TIMEOUT, "10000")
            .trim()
            .parse()
            .unwrap_or(10_000),
    );
    let retry_policy = RetryPolicy {
        max_retries: config
            .get_property_or(MAX_RETRIES, "3")
            .trim()
            .parse()
            .unwrap_or(3),
        backoff_ms: config
            .get_property_or(RETRY_BACKOFF, "500")
            .trim()
            .parse()
            .unwrap_or(500),
        dead_letter_publisher: publisher,
    };
    let platform = Platform::get_instance();
    let mut consumers = Vec::with_capacity(bindings.len());
    for binding in bindings {
        // the pinned delivery-mode overlay: the binding's group id and manual
        // commit-after-process (per-record recv IS the poll-batch-of-one -
        // librdkafka has no max.poll.records and needs none here)
        let mut consumer_config = client_config::consumer_client_config()?;
        consumer_config.set("group.id", &binding.group_id);
        consumer_config.set("enable.auto.commit", "false");
        if consumer_config.get("group.protocol").map(str::trim) == Some("auto") {
            // the Java module's 'auto' probes the cluster; this port resolves
            // it at a later increment - classic is every broker's safe answer
            log::info!("group.protocol=auto is not resolved by this increment; using classic");
            consumer_config.remove("group.protocol");
        }
        let stream_consumer = consumer_config.create().map_err(|e| {
            AppError::new(
                500,
                format!(
                    "Unable to build Kafka consumer for '{}' - {e}",
                    binding.topic
                ),
            )
        })?;
        consumers.push(KafkaFlowConsumer::start(
            platform.clone(),
            stream_consumer,
            binding,
            retry_policy.clone(),
            dlq_timeout,
        )?);
    }
    let started = consumers.len();
    runtime::set_flow_consumers(consumers);
    log::info!("Kafka flow adapter started from {adapter_location} ({started} binding(s))");
    Ok(())
}
