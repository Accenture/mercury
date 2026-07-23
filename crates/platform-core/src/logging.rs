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
//! inside each step. The log-context feature is **on by default**: the crate
//! ships a built-in `default-log-context.yaml` (embedded at compile time)
//! carrying the standard trace context, so every structured (JSON) log line
//! emitted inside a traced function carries a `context` block — correlation
//! id, trace/span ids, service name, and any business key-values added via
//! `PostOffice::update_context` — with zero setup. An application replaces
//! the template with its own **`app-log-context.yaml`** on the resource path,
//! or opts out entirely with `app.log.context=false` (default `true`).
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
/// The built-in default template (Java `default-log-context.yaml`), shipped
/// under a DISTINCT file name from the application override. Java keeps the
/// two names apart because same-named classpath resources shadow in
/// classloader order; this port embeds the default at compile time — the
/// same defensive design, enforced by the compiler.
const DEFAULT_TEMPLATE: &str = include_str!("../resources/default-log-context.yaml");
/// Feature switch (Java `app.log.context`), default `true`.
const FEATURE_FLAG: &str = "app.log.context";
const CONTEXT: &str = "context";

/// Parsed log-context template (Java `LogContextConfig`) — the application's
/// `app-log-context.yaml` when present, otherwise the built-in default.
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
        INSTANCE.get_or_init(Self::load_config_file)
    }

    /// Resolve the template with the Java `LogContextConfig.loadConfigFile`
    /// order: the `app.log.context` switch (default on) → the application's
    /// own `app-log-context.yaml` (replaces the template entirely) → the
    /// built-in default, so the feature is on out of the box.
    fn load_config_file() -> LogContextConfig {
        let config = AppConfigReader::get_instance();
        if config.get_property_or(FEATURE_FLAG, "true") == "false" {
            log::info!("Application log context disabled by {FEATURE_FLAG}=false");
            return LogContextConfig::disabled();
        }
        match ConfigReader::load(CONFIG_FILE) {
            Ok(reader) => LogContextConfig::from_reader(&reader),
            Err(ConfigError::NotFound(_)) => {
                // no application override — fall back to the built-in default
                match ConfigReader::from_yaml_text(DEFAULT_TEMPLATE) {
                    Ok(reader) => LogContextConfig::from_reader(&reader),
                    Err(e) => {
                        log::warn!("Built-in default-log-context.yaml invalid - {e}");
                        LogContextConfig::disabled()
                    }
                }
            }
            Err(e) => {
                log::error!("Unable to load {CONFIG_FILE} - {e}");
                LogContextConfig::disabled()
            }
        }
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

    /// One sequential test for the three `load_config_file` outcomes — the
    /// resolution reads process-global state (overrides, resource roots), so
    /// the cases must not run as parallel tests.
    #[test]
    fn log_context_is_on_by_default_overridable_and_can_opt_out() {
        // 1. DEFAULT-ON: no app-log-context.yaml on the resource path (this
        // crate's own resources/ has none) → the built-in default template
        // enables the feature with the standard trace-context keys
        let config = LogContextConfig::load_config_file();
        assert!(config.is_enabled(), "log context must be ON by default");
        let token_keys: Vec<&str> = config.tokens.iter().map(|(k, _)| k.as_str()).collect();
        for expected in [
            "cid",
            "traceId",
            "tracePath",
            "spanId",
            "parentSpanId",
            "service",
            "timestamp",
        ] {
            assert!(
                token_keys.contains(&expected),
                "built-in template must carry '{expected}'"
            );
        }
        assert!(
            config.constants.is_empty(),
            "built-in default has no constants"
        );

        // 2. OPT-OUT: app.log.context=false disables the feature entirely
        crate::util::overrides::set(FEATURE_FLAG, "false");
        let config = LogContextConfig::load_config_file();
        crate::util::overrides::clear(FEATURE_FLAG);
        assert!(!config.is_enabled(), "app.log.context=false must opt out");

        // 3. APP FILE OVERRIDES: an app-log-context.yaml on the resource path
        // REPLACES the built-in template entirely (no merge)
        let dir = std::env::temp_dir().join(format!("pc-logctx-default-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("app-log-context.yaml"),
            "context:\n  onlyKey: $service\n",
        )
        .unwrap();
        crate::util::resources::prepend_resource_root(&dir);
        let config = LogContextConfig::load_config_file();
        assert!(config.is_enabled());
        assert_eq!(
            config.tokens,
            vec![("onlyKey".to_string(), "service".to_string())],
            "the application template must replace the built-in default entirely"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
