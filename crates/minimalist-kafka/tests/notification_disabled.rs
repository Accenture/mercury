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

//! The disabled-producer contract, in its own process (the runtime holder is
//! process-wide): `simple.kafka.notification` stays registered when
//! `kafka.producer.enabled=false`, and a flow that publishes anyway fails
//! with the setting that caused it — never a "route not found" hunt.

use std::collections::HashMap;

use minimalist_kafka::SimpleKafkaNotification;
use platform_core::{ComposableFunction, EventEnvelope};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn disabled_producer_fails_loudly_with_the_flag_name() {
    let headers: HashMap<String, String> =
        HashMap::from([("topic".to_string(), "orders".to_string())]);
    let error = SimpleKafkaNotification
        .handle_event(
            headers,
            EventEnvelope::new().set_raw_body(rmpv::Value::Binary(b"x".to_vec())),
            1,
        )
        .await
        .expect_err("no publisher installed");
    assert_eq!(500, error.status());
    assert!(
        error
            .message()
            .contains("Kafka producer is disabled (kafka.producer.enabled=false)"),
        "got: {}",
        error.message()
    );
}
