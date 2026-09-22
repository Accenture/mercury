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

//! The forwarder's settings — the Java module's `otel.*` keys, read from
//! application configuration.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use platform_core::AppConfigReader;

use crate::headers::parse_headers;

pub const ENDPOINT: &str = "otel.exporter.otlp.endpoint";
pub const TIMEOUT: &str = "otel.exporter.otlp.timeout";
pub const CONNECT_TIMEOUT: &str = "otel.exporter.otlp.connect.timeout";
pub const COMPRESSION: &str = "otel.exporter.otlp.compression";
pub const HEADERS: &str = "otel.exporter.otlp.headers";
pub const SERVICE_NAME: &str = "otel.service.name";
const APP_NAME: &str = "application.name";
const APP_VERSION: &str = "info.app.version";

pub const DEFAULT_ENDPOINT: &str = "http://localhost:4318/v1/traces";
pub const DEFAULT_TIMEOUT_MS: u64 = 10_000;
pub const DEFAULT_COMPRESSION: &str = "none";
pub const DEFAULT_SERVICE: &str = "mercury";

/// Resolves the request headers for ONE export. Called on every export, so a
/// credential published after start-up is picked up (see [`reloading_headers`]).
pub type HeaderSupplier = Arc<dyn Fn() -> Vec<(String, String)> + Send + Sync>;

/// Everything the exporter needs; built from configuration in production and
/// directly by tests.
pub struct ForwarderSettings {
    /// The OTLP/HTTP traces URL, including the signal path.
    pub endpoint: String,
    /// The `service.name` resource attribute.
    pub service_name: String,
    /// The instrumentation-scope version: `info.app.version`, else this crate's
    /// version (the Java `Utility.getVersion()` shape — resolved at runtime, so
    /// it never goes stale across releases).
    pub scope_version: String,
    /// Per-export timeout in milliseconds.
    pub timeout_ms: u64,
    /// The configured compression, kept for the startup line; only `none` is
    /// honoured on this engine.
    pub compression: String,
    /// The request headers, resolved per export.
    pub headers: HeaderSupplier,
}

impl ForwarderSettings {
    /// Read the `otel.*` keys (Java `OpenTelemetryForwarder()` constructor).
    /// Infallible: an unparseable number falls back to its default with a
    /// warning; the endpoint URL is validated when the exporter is built.
    pub fn from_config(config: &'static AppConfigReader) -> Self {
        let service_name = config.get_property(SERVICE_NAME).unwrap_or_else(|| {
            config
                .get_property(APP_NAME)
                .unwrap_or_else(|| DEFAULT_SERVICE.to_string())
        });
        let endpoint = config
            .get_property(ENDPOINT)
            .unwrap_or_else(|| DEFAULT_ENDPOINT.to_string());
        let timeout_ms = millis(config.get_property(TIMEOUT), TIMEOUT, DEFAULT_TIMEOUT_MS);
        let compression = config
            .get_property(COMPRESSION)
            .map(|c| c.trim().to_string())
            .filter(|c| !c.is_empty())
            .unwrap_or_else(|| DEFAULT_COMPRESSION.to_string());
        if !compression.eq_ignore_ascii_case(DEFAULT_COMPRESSION) {
            log::warn!(
                "{COMPRESSION}={compression} is not supported on this engine - exporting \
                 uncompressed (the payload is one span per request); set none to silence this"
            );
        }
        if config.exists(CONNECT_TIMEOUT) {
            log::warn!(
                "{CONNECT_TIMEOUT} has no effect on this engine - the platform HTTP client's \
                 http.client.connection.timeout governs the connect phase"
            );
        }
        let scope_version = config.get_property_or(APP_VERSION, env!("CARGO_PKG_VERSION"));
        // Read through a supplier so a credential published AFTER this start-up
        // read is picked up rather than frozen out: a runtime override
        // (`overrides::set`, the -D / System.setProperty analog a credential
        // bootstrap uses) is consulted first on every configuration lookup.
        // (${ENV_VAR} references themselves resolve once, when the
        // configuration loads - the environment of a running process does not
        // change underneath it.)
        let headers = reloading_headers(move || config.get_property(HEADERS));
        ForwarderSettings {
            endpoint,
            service_name,
            scope_version,
            timeout_ms,
            compression,
            headers,
        }
    }

    /// The built-in defaults (a local collector, no credential).
    pub fn defaults() -> Self {
        Self::fixed(DEFAULT_ENDPOINT, DEFAULT_TIMEOUT_MS, Vec::new())
    }

    /// Settings with a FIXED header list — for tests and callers whose
    /// credentials are known up front.
    pub fn fixed(endpoint: &str, timeout_ms: u64, headers: Vec<(String, String)>) -> Self {
        ForwarderSettings {
            endpoint: endpoint.to_string(),
            service_name: DEFAULT_SERVICE.to_string(),
            scope_version: env!("CARGO_PKG_VERSION").to_string(),
            timeout_ms,
            compression: DEFAULT_COMPRESSION.to_string(),
            headers: Arc::new(move || headers.clone()),
        }
    }

    /// Replace the service name (builder style).
    pub fn with_service_name(mut self, service_name: &str) -> Self {
        self.service_name = service_name.to_string();
        self
    }
}

/// Wrap a raw `OTEL_EXPORTER_OTLP_HEADERS`-style source as a header supplier
/// that reparses on every call and announces ONCE when a credential first
/// resolves — so an operator whose bootstrap publishes the token after
/// start-up sees "no credential yet" at boot and a single confirmation when
/// exports start carrying it. Header VALUES are never logged, only names.
pub fn reloading_headers(
    raw: impl Fn() -> Option<String> + Send + Sync + 'static,
) -> HeaderSupplier {
    let announced = AtomicBool::new(false);
    Arc::new(move || {
        let headers = parse_headers(raw().as_deref());
        if !headers.is_empty()
            && announced
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        {
            let names: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
            log::info!("OTLP credential header resolved - {names:?}");
        }
        headers
    })
}

fn millis(value: Option<String>, key: &str, default: u64) -> u64 {
    match value {
        None => default,
        Some(text) => match text.trim().parse::<u64>() {
            Ok(ms) if ms > 0 => ms,
            _ => {
                log::warn!(
                    "{key}={text} is not a positive number of milliseconds - using {default}"
                );
                default
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_fall_back_to_their_default() {
        assert_eq!(millis(None, TIMEOUT, 10_000), 10_000);
        assert_eq!(millis(Some(" 2500 ".into()), TIMEOUT, 10_000), 2500);
        assert_eq!(millis(Some("soon".into()), TIMEOUT, 10_000), 10_000);
        assert_eq!(millis(Some("0".into()), TIMEOUT, 10_000), 10_000);
    }

    #[test]
    fn reloading_headers_reparse_on_every_call() {
        let source = Arc::new(std::sync::Mutex::new(None::<String>));
        let reading = source.clone();
        let supplier = reloading_headers(move || reading.lock().unwrap().clone());
        assert!(supplier().is_empty(), "unset -> no headers");
        *source.lock().unwrap() = Some("Authorization=Api-Token late".into());
        assert_eq!(
            supplier(),
            vec![("Authorization".to_string(), "Api-Token late".to_string())],
            "a value published after construction is picked up"
        );
    }
}
