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

//! Structured logging with the **application log context** — Rust port of the
//! Java `LogContextConfig` + `JsonLogger`/`CompactAppender` design
//! (`org.platformlambda.core.logging`).
//!
//! Spans tell you the causal path; application logs tell you what happened
//! inside each step. When the optional **`app-log-context.yaml`** is present
//! on the resource path, every structured (JSON) log line emitted inside a
//! traced function carries a `context` block — correlation id, trace/span
//! ids, service name, and any business key-values added via
//! `PostOffice::update_context` — so logs and spans join up in the backend.
//!
//! The template maps an output key (your choice) to one of three forms:
//! a reserved **`$token`** (`$cid`, `$traceId`, `$tracePath`, `$spanId`,
//! `$parentSpanId`, `$service`, `$utc` — resolved live per log line), a
//! **`${ENV:default}`** substitution (resolved once at load, via the standard
//! `ConfigReader`), or a **literal**. A key that resolves to nothing is
//! omitted, never printed as null.
//!
//! [`init`] installs the process logger with three formats (the log4j2
//! appender-selection analog): the default **`text`** is a plain console line
//! and — like Java's plain `Console` appender — is unaffected by the log
//! context; **`json`** pretty-prints each record (Java `log4j2-json.xml`);
//! **`compact`** emits single-line jsonl records, no CR/LF (Java
//! `log4j2-compact.xml`). `-Dkey=value` runtime arguments (the JVM `-D`
//! analog) are honored, so `-Dlog.format=json` switches at launch without
//! editing configuration. Deliberate simplifications (doc'd): UTC timestamps,
//! no thread id.

use std::sync::OnceLock;

use crate::trace;
use crate::util::app_config_reader::AppConfigReader;
use crate::util::config_reader::{ConfigError, ConfigReader};

const CONFIG_FILE: &str = "classpath:/app-log-context.yaml";
const CONTEXT: &str = "context";

/// Parsed `app-log-context.yaml` template (Java `LogContextConfig`).
pub struct LogContextConfig {
    enabled: bool,
    /// output key → reserved token name (without the `$`), resolved per line
    tokens: Vec<(String, String)>,
    /// output key → constant (env-resolved or literal), fixed at load
    constants: Vec<(String, String)>,
}

impl LogContextConfig {
    /// The lazily-loaded singleton; touching [`AppConfigReader`] first
    /// guarantees `${ENV:default}` substitution works regardless of timing
    /// (Java parity).
    pub fn instance() -> &'static LogContextConfig {
        static INSTANCE: OnceLock<LogContextConfig> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let _ = AppConfigReader::get_instance();
            match ConfigReader::load(CONFIG_FILE) {
                Ok(reader) => LogContextConfig::from_reader(&reader),
                Err(ConfigError::NotFound(_)) => {
                    // optional config file absent — feature stays off
                    log::debug!("Optional {CONFIG_FILE} not found - log context feature disabled");
                    LogContextConfig::disabled()
                }
                Err(e) => {
                    log::error!("Unable to load {CONFIG_FILE} - {e}");
                    LogContextConfig::disabled()
                }
            }
        })
    }

    fn disabled() -> Self {
        LogContextConfig {
            enabled: false,
            tokens: Vec::new(),
            constants: Vec::new(),
        }
    }

    /// Build a config from a loaded reader. Public so tests and tooling can
    /// exercise the enabled/disabled paths deterministically (the Java
    /// package-private constructor's analog).
    pub fn from_reader(reader: &ConfigReader) -> Self {
        let mut tokens = Vec::new();
        let mut constants = Vec::new();
        let section: Vec<String> = match reader.get_map().get_element(CONTEXT) {
            Some(crate::ConfigValue::Map(m)) => m.keys().cloned().collect(),
            _ => {
                log::warn!("Log context config has no '{CONTEXT}' section - feature disabled");
                return LogContextConfig::disabled();
            }
        };
        for output_key in section {
            // ConfigReader resolves ${ENV:default} on the leaf value; an unset
            // ${VAR} with no default resolves to nothing and is dropped
            let Some(value) = reader.get_property(&format!("{CONTEXT}.{output_key}")) else {
                continue;
            };
            if let Some(token_name) = value.strip_prefix('$').filter(|_| !value.starts_with("${")) {
                if trace::RESERVED_KEYS.contains(&token_name) {
                    tokens.push((output_key, token_name.to_string()));
                } else {
                    // Java throws here; the Rust port stays advisory —
                    // report and skip (deliberate divergence)
                    log::error!(
                        "Invalid log context token '{value}' for key '{output_key}' - allowed: {:?}",
                        trace::RESERVED_KEYS
                    );
                }
            } else {
                constants.push((output_key, value));
            }
        }
        let enabled = !tokens.is_empty() || !constants.is_empty();
        if enabled {
            log::info!(
                "Application log context enabled with {} context key-value(s)",
                tokens.len() + constants.len()
            );
        }
        LogContextConfig {
            enabled,
            tokens,
            constants,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Build the context block for one log line (Java `render`): reserved
    /// tokens resolved live, constants, then the developer's custom keys.
    /// Keys resolving to nothing are omitted.
    ///
    /// `state` is the current trace bracket when the line runs inside a traced
    /// function. Outside a trace (`None`) the block still renders with the
    /// trace-independent keys — constants and `$utc` — while trace-bound
    /// tokens and custom keys are omitted. (A deliberate refinement over Java,
    /// where the block appears only on traced lines — maintainer-directed so
    /// application-level log lines carry the static context too.)
    pub fn render(
        &self,
        state: Option<&trace::TraceState>,
        log_time: std::time::SystemTime,
    ) -> serde_json::Map<String, serde_json::Value> {
        let mut out = serde_json::Map::new();
        for (output_key, token_name) in &self.tokens {
            let value = match state {
                Some(state) => state.token(token_name, log_time),
                // only $utc resolves without a trace
                None if token_name == "utc" => {
                    Some(serde_json::Value::String(trace::iso8601_utc(log_time)))
                }
                None => None,
            };
            if let Some(value) = value {
                out.insert(output_key.clone(), value);
            }
        }
        for (output_key, constant) in &self.constants {
            out.insert(
                output_key.clone(),
                serde_json::Value::String(constant.clone()),
            );
        }
        if let Some(state) = state {
            for (key, value) in &state.custom_log_keys {
                if !value.is_null() {
                    out.insert(key.clone(), value.clone());
                }
            }
        }
        out
    }
}

/// The three output formats (the log4j2 appender-selection analog):
/// `text` = the default plain console line (context-free, like Java's plain
/// `Console` appender); `json` = pretty-print JSON (Java `log4j2-json.xml`);
/// `compact` = single-line jsonl, no CR/LF within a record (Java
/// `log4j2-compact.xml`). Both JSON forms carry the `context` block.
#[derive(Clone, Copy, PartialEq)]
enum LogFormat {
    Text,
    Json,
    Compact,
}

impl LogFormat {
    fn resolve(name: &str) -> LogFormat {
        match name.to_ascii_lowercase().as_str() {
            "json" => LogFormat::Json,
            "compact" => LogFormat::Compact,
            _ => LogFormat::Text,
        }
    }
}

/// The process logger (the log4j2 appenders' analog).
struct PlatformLogger {
    format: LogFormat,
    level: log::LevelFilter,
}

impl log::Log for PlatformLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let now = std::time::SystemTime::now();
        let time = trace::iso8601_utc(now);
        if self.format == LogFormat::Text {
            println!(
                "{time} {:<5} [{}] {}",
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            );
            return;
        }
        let mut line = serde_json::Map::new();
        line.insert("time".into(), serde_json::Value::String(time));
        line.insert(
            "level".into(),
            serde_json::Value::String(record.level().to_string()),
        );
        line.insert(
            "source".into(),
            serde_json::Value::String(format!(
                "{}({}:{})",
                record.module_path().unwrap_or("unknown"),
                record.file().unwrap_or("?"),
                record.line().unwrap_or(0)
            )),
        );
        let message = record.args().to_string();
        // a message that is itself JSON embeds as a structured object
        // (Java JsonLogger's ObjectMessage handling — the telemetry
        // dataset renders structured, not as an escaped string)
        let message_value = if message.starts_with('{') {
            serde_json::from_str::<serde_json::Value>(&message)
                .unwrap_or(serde_json::Value::String(message))
        } else {
            serde_json::Value::String(message)
        };
        line.insert("message".into(), message_value);
        // the application log context (when the optional config is present):
        // inside a traced worker the full set renders — the logger runs
        // synchronously on the same task, so the task-local is found; outside
        // a trace the trace-independent keys (constants, $utc) still render
        let config = LogContextConfig::instance();
        if config.is_enabled() {
            let context = trace::with_current(|state| config.render(Some(state), now))
                .unwrap_or_else(|| config.render(None, now));
            if !context.is_empty() {
                line.insert("context".into(), serde_json::Value::Object(context));
            }
        }
        let line = serde_json::Value::Object(line);
        match self.format {
            // pretty-print JSON, one record over multiple lines
            LogFormat::Json => println!(
                "{}",
                serde_json::to_string_pretty(&line).unwrap_or_else(|_| line.to_string())
            ),
            // compact jsonl: one record per line, no CR/LF within a record
            _ => println!("{line}"),
        }
    }

    fn flush(&self) {}
}

/// Install the process logger, reading `log.format` (`text` | `json` |
/// `compact`, default `text`) and `log.level` (default `info`; `RUST_LOG` env
/// wins) from the application configuration. `-Dkey=value` runtime arguments
/// (the JVM `-D` analog) are loaded into the override registry first, so
/// `hello_world -- -Dlog.format=json` switches format at launch. Idempotent —
/// a second call is a no-op (the `log` crate accepts one logger per process).
pub fn init() {
    // runtime -D overrides win over configuration files (System.getProperty parity)
    crate::util::overrides::load_runtime_args();
    let config = AppConfigReader::get_instance();
    let format = LogFormat::resolve(&config.get_property_or("log.format", "text"));
    let level_text = std::env::var("RUST_LOG")
        .ok()
        .unwrap_or_else(|| config.get_property_or("log.level", "info"));
    let level = match level_text.to_ascii_lowercase().as_str() {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        "off" => log::LevelFilter::Off,
        _ => log::LevelFilter::Info,
    };
    // initialize the log-context template BEFORE installing the logger: the
    // JSON logger consults it on every line, and letting the first log line
    // trigger the lazy init would re-enter the OnceLock from inside its own
    // initializer (the config logs while loading) — a deadlock
    let context = LogContextConfig::instance();
    if log::set_boxed_logger(Box::new(PlatformLogger { format, level })).is_ok() {
        log::set_max_level(level);
        if context.is_enabled() {
            log::info!(
                "Application log context enabled with {} context key-value(s)",
                context.tokens.len() + context.constants.len()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_resolution() {
        assert!(matches!(LogFormat::resolve("json"), LogFormat::Json));
        assert!(matches!(LogFormat::resolve("JSON"), LogFormat::Json));
        assert!(matches!(LogFormat::resolve("compact"), LogFormat::Compact));
        assert!(matches!(LogFormat::resolve("text"), LogFormat::Text));
        assert!(matches!(LogFormat::resolve("unknown"), LogFormat::Text)); // safe default
    }
}
