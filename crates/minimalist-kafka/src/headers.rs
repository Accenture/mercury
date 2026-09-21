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

//! Header names used by the minimalist Kafka building blocks (Java
//! `KafkaHeaders`). The W3C `traceparent` header is not declared here — the
//! platform's `w3c_trace::TRACEPARENT` is used.

/// Destination topic for `simple.kafka.notification` (required routing header).
pub const TOPIC: &str = "topic";

/// Target partition for `simple.kafka.notification` (optional routing header).
pub const PARTITION: &str = "partition";

/// Correlation-id convention: carried as a Kafka header and used as the
/// flow's correlation-id.
pub const CORRELATION_ID: &str = "cid";

/// Optional for `simple.kafka.notification`: the Schema Registry **subject**
/// to serialize against. The schema must be pre-registered; the producer
/// resolves the subject to a global schema id and its type and frames the
/// body (a JSON document) in the Confluent wire format — it never registers.
pub const SUBJECT: &str = "subject";

/// Optional companion to [`SUBJECT`]: the subject version to resolve — a
/// positive integer pins a version, `latest` (the default) tracks the current
/// one.
pub const VERSION: &str = "version";

/// The default [`VERSION`].
pub const DEFAULT_VERSION: &str = "latest";

// Read-only reserved headers injected by the framework worker; never
// forwarded to Kafka as raw headers.
pub(crate) const MY_ROUTE: &str = "my_route";
pub(crate) const MY_TRACE_ID: &str = "my_trace_id";
pub(crate) const MY_TRACE_PATH: &str = "my_trace_path";
pub(crate) const MY_CORRELATION_ID: &str = "my_correlation_id";
