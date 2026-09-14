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

//! **Minimalist Kafka** — the opt-in Kafka building blocks for Mercury (Rust
//! port of the Java `system/minimalist-kafka`): publish events to topics
//! through the composable [`notification`] function, health-check the cluster
//! with [`health`], and (from a later increment of the port) route topics
//! into Event Script flows.
//!
//! **Config, not code**: the Kafka client connection/security parameters come
//! from external `kafka-producer` / `kafka-consumer` templates with
//! `${ENV_VAR:default}` substitution ([`client_config`]) — enterprise
//! SASL/OAuth2/mTLS is configured, never coded (build with the `ssl` feature
//! for SASL_SSL connectivity). The library autoloads at startup: depending on
//! this crate registers `simple.kafka.notification` and `kafka.health` and
//! runs [`bootstrap::KafkaAutoStart`], exactly as the Java jar does through
//! classpath scanning. Either client can be switched off
//! (`kafka.producer.enabled` / `kafka.consumer.enabled`) — the flags are
//! vetoes, not triggers.
//!
//! This is **not** a service mesh: it is an application-level building block
//! you opt into, the Kafka counterpart of the HTTP flow adapter. Design:
//! `draft-design-specs/minimalist-kafka-port.md` (the Java module and its
//! guide are the canon).

pub mod bootstrap;
pub mod client_config;
pub mod headers;
pub mod health;
pub mod notification;
pub mod publisher;
pub mod runtime;

pub use health::{KafkaHealthProbe, KAFKA_HEALTH_ROUTE};
pub use notification::{SimpleKafkaNotification, ROUTE as NOTIFICATION_ROUTE};
pub use publisher::KafkaRequestPublisher;
