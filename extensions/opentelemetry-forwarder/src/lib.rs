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

//! OpenTelemetry trace forwarder — the Rust twin of the Java
//! `opentelemetry-forwarder` extension (`org.platformlambda.opentelemetry`).
//!
//! The platform's built-in `distributed.tracing` service hands every completed
//! span's performance-metrics dataset to whatever function is registered at the
//! reserved route **`distributed.trace.forwarder`**. This crate registers that
//! function: it maps each dataset to an OpenTelemetry span — carrying the
//! **exact** W3C trace / span / parent-span ids the engine already propagated,
//! never minting new ones — and exports it over **OTLP/HTTP** (binary protobuf,
//! `application/x-protobuf`) to a collector, or straight to a SaaS backend such
//! as Dynatrace or Splunk with its API token in a request header.
//!
//! **Opt-in by configuration.** Linking the crate registers nothing: the
//! forwarder is `#[optional_service("otel.forwarding")]`, so the route exists
//! only when `otel.forwarding=true` — in `application.yml`, or at launch with
//! the runtime override `-Dotel.forwarding=true`. That separates two decisions
//! that belong to different people: a developer adds the dependency, DevOps
//! decides per environment whether traces leave the process.
//!
//! **Activation.** A binary that names nothing from this crate must still link
//! it, so the annotation entries are collected: add
//! `use opentelemetry_forwarder as _;` next to the other engine crates.
//!
//! **No OpenTelemetry SDK.** The OTLP payload is one `ExportTraceServiceRequest`
//! per span, written by this crate's own protobuf encoder (`otlp` module) and
//! sent through the platform's `async.http.request` client — no new HTTP stack,
//! no SDK, no generated code. The wire format is the OTLP v1 trace schema,
//! which is frozen.
//!
//! Configuration keys are the Java module's, read from application
//! configuration (values support `${ENV_VAR:default}` substitution):
//!
//! | Key | Default | Meaning |
//! |-----|---------|---------|
//! | `otel.forwarding` | `false` | master switch — nothing registers unless `true` |
//! | `otel.exporter.otlp.endpoint` | `http://localhost:4318/v1/traces` | the OTLP/HTTP traces URL (the full signal path) |
//! | `otel.exporter.otlp.timeout` | `10000` | per-export timeout, ms |
//! | `otel.exporter.otlp.headers` | — | comma-separated `key=value` (or `key: value`) request headers — where the backend credential goes; re-read on every export |
//! | `otel.service.name` | `application.name`, else `mercury` | the `service.name` resource attribute |
//! | `otel.exporter.otlp.compression` | `none` | accepted for parity; only `none` is honoured on this engine |
//! | `otel.exporter.otlp.connect.timeout` | — | no effect here: the platform client's `http.client.connection.timeout` governs the connect phase |
//!
//! See the crate README and the observability guide for the mapping table and
//! the credential guidance.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use async_trait::async_trait;
use platform_core::{
    before_application, preload, AppConfigReader, AppError, ComposableFunction, EntryPoint,
    EventEnvelope,
};

pub mod config;
pub mod export;
pub mod headers;
pub mod otlp;
pub mod runtime;
pub mod span;

pub use config::{ForwarderSettings, HeaderSupplier};
pub use export::{ExportFailure, Exporter};
pub use headers::parse_headers;
pub use span::{AttributeValue, Span, SpanKind, SpanStatus, StatusCode};

/// The reserved extension route this crate registers (the platform constant
/// `platform_core::telemetry::DISTRIBUTED_TRACE_FORWARDER`).
pub const FORWARDER_ROUTE: &str = "distributed.trace.forwarder";

/// The master switch (Java `@OptionalService("otel.forwarding")`).
pub const FORWARDING_SWITCH: &str = "otel.forwarding";

/// The OpenTelemetry instrumentation-scope name stamped on every exported span.
/// It names this crate, so a backend can tell the two engines' forwarders apart
/// (the Java module reports `org.platformlambda.opentelemetry-forwarder`).
pub const INSTRUMENTATION_SCOPE: &str = "mercury-opentelemetry-forwarder";

/// Startup hook: validates the configuration once, announces the forwarder
/// and installs the shared [`Exporter`] before any function registers — so a
/// misconfigured endpoint fails the application start with a clear message
/// instead of failing every export quietly (Java: the `@PreLoad` constructor
/// throws). Gated by the same switch as the function, so with forwarding off
/// nothing runs and nothing is logged.
#[before_application(sequence = 6)]
#[optional_service("otel.forwarding")]
pub struct OtelForwarderBootstrap;

#[async_trait]
impl EntryPoint for OtelForwarderBootstrap {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let settings = ForwarderSettings::from_config(AppConfigReader::get_instance());
        let exporter = Exporter::from_settings(settings)?;
        let header_names = exporter.header_names();
        log::info!(
            "OpenTelemetry trace forwarder ready - service={}, OTLP endpoint={}, compression={}, \
             credential headers={:?}",
            exporter.service_name(),
            exporter.endpoint(),
            exporter.compression(),
            header_names
        );
        if header_names.is_empty() {
            log::info!(
                "No OTLP credential header yet ({} unset) - it is re-read on every export, so a \
                 credential published later by a startup bootstrap takes effect without a restart",
                config::HEADERS
            );
        }
        runtime::install(Arc::new(exporter));
        Ok(())
    }
}

/// The drop-in `distributed.trace.forwarder` (Java `OpenTelemetryForwarder`):
/// receives one telemetry dataset per completed span and exports it.
///
/// `#[zero_tracing]` keeps the forwarder out of the trace it reports (its route
/// is also on the platform's zero-tracing filter). Two worker instances export
/// concurrently — a slow backend queues spans in the route's mailbox rather
/// than dropping them (the Java exporter's fixed pool with an unbounded queue).
///
/// The exporter is resolved on the first dataset, not at construction: the
/// lifecycle constructs every annotated function BEFORE it runs the
/// `before_application` hooks, so the bootstrap's validated exporter is only
/// available once events flow. Constructed without the bootstrap (a test that
/// registers the function by hand), it configures itself from application
/// configuration at that first dataset.
#[preload(route = "distributed.trace.forwarder", instances = 2)]
#[optional_service("otel.forwarding")]
#[zero_tracing]
#[derive(Default)]
pub struct OpenTelemetryForwarder {
    exporter: OnceLock<Arc<Exporter>>,
}

impl OpenTelemetryForwarder {
    /// Test seam (Java: the package-private constructor taking a context).
    pub fn new(exporter: Arc<Exporter>) -> Self {
        let slot = OnceLock::new();
        let _ = slot.set(exporter);
        OpenTelemetryForwarder { exporter: slot }
    }

    fn exporter(&self) -> &Arc<Exporter> {
        self.exporter.get_or_init(|| {
            runtime::exporter().unwrap_or_else(|| {
                let settings = ForwarderSettings::from_config(AppConfigReader::get_instance());
                match Exporter::from_settings(settings) {
                    Ok(exporter) => Arc::new(exporter),
                    Err(e) => {
                        log::error!(
                            "OpenTelemetry forwarder falls back to the default endpoint - {}",
                            e.message()
                        );
                        Arc::new(
                            Exporter::from_settings(ForwarderSettings::defaults())
                                .expect("the built-in default endpoint is valid"),
                        )
                    }
                }
            })
        })
    }
}

#[async_trait]
impl ComposableFunction for OpenTelemetryForwarder {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        // a dataset without W3C-valid ids cannot become a span without forging ids - skipped
        if let Some(span) = Span::from_dataset(input.body()) {
            if let Err(failure) = self.exporter().export(&span).await {
                log::warn!(
                    "OTLP export failed for span {} of trace {} - {failure}",
                    span.span_id_hex(),
                    span.trace_id_hex()
                );
            }
        }
        Ok(EventEnvelope::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_route_is_the_platform_extension_point() {
        assert_eq!(
            FORWARDER_ROUTE,
            platform_core::telemetry::DISTRIBUTED_TRACE_FORWARDER
        );
    }
}
