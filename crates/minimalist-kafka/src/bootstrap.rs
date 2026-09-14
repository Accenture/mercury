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
use platform_core::{main_application, AppConfigReader, AppError, EntryPoint};
use rdkafka::producer::FutureProducer;

use crate::client_config::{self, CONSUMER_ENABLED, PRODUCER_ENABLED};
use crate::publisher::KafkaRequestPublisher;
use crate::{notification, runtime};

const ADAPTER_CONFIG: &str = "yaml.kafka.flow.adapter";

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
        } else if config.get_property(ADAPTER_CONFIG).is_some() {
            log::info!(
                "{ADAPTER_CONFIG} is set; the inbound flow adapter arrives in a later increment of this port"
            );
        } else {
            log::info!("{ADAPTER_CONFIG} not set; Kafka flow adapter not started");
        }
        Ok(())
    }
}
