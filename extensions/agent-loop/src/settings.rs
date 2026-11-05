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

//! Rust port of the Java `LoopSettings` (`org.platformlambda.agentloop.LoopSettings`) and its nested
//! `Defaults`: the loop node's settings (RFC-0011, section 6.1). Each is a constant input mapping on
//! the node, so it is visible on the graph and certified with it; a global default for each lives in
//! the application's configuration.

use platform_core::AppConfigReader;
use rmpv::Value;

use crate::tool_schema::Decimal;
use crate::value::{get, java_trim, value_of};

/// A tool marked `write: true` waits for a person; every other tool runs. The default.
pub const WRITE: &str = "write";
/// Every tool runs, a marked write included: the node is designed for automation.
pub const AUTO: &str = "auto";
/// Every tool call waits for a person: a supervised run.
pub const MANUAL: &str = "manual";
/// A marked write goes back to the model as a refusal and never waits: a run with nobody to ask.
pub const UNATTENDED: &str = "unattended";
pub(crate) const MODE_INPUT: &str = "mode";
const MODES: [&str; 4] = [WRITE, AUTO, MANUAL, UNATTENDED];
pub(crate) const MAX_TURNS_INPUT: &str = "max_turns";
pub(crate) const RESULT_LIMIT_INPUT: &str = "result_limit";
pub(crate) const TRANSCRIPT_TTL_INPUT: &str = "transcript_ttl";
pub(crate) const RETENTION_INPUT: &str = "retention";
pub(crate) const TIMEOUT_INPUT: &str = "timeout";
const DURATION_RULE: &str = " must be a duration such as 30m, 12h or 2d (seconds, s, m, h, d)";

/// The loop node's settings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoopSettings {
    /// The approval mode: write (the default), auto, manual or unattended.
    pub mode: String,
    /// The conversation's turn limit, a proxy for the input tokens the run accumulates.
    pub max_turns: i32,
    /// The largest tool result, in characters of JSON text.
    pub result_limit: usize,
    /// How long a running or waiting transcript lives, as sent to `llm.chat`.
    pub transcript_ttl: String,
    /// How long a finished run's transcript is kept, as sent to `llm.chat`.
    pub retention: String,
    /// The loop's deadline, from its start.
    pub timeout_ms: i64,
}

/// The global defaults of the settings, from the application's configuration. Each is text, as a
/// configuration value is, and is checked when a run resolves its settings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Defaults {
    /// `llm.run.max.turns`, default 10.
    pub max_turns: String,
    /// `llm.run.result.limit`, default 65536.
    pub result_limit: String,
    /// `llm.run.transcript.ttl`, default 7d.
    pub transcript_ttl: String,
    /// `llm.run.transcript.retention`, default 0.
    pub retention: String,
    /// `llm.run.timeout`, default 25s - below the graph's default `model.ttl` of 30 seconds.
    pub timeout: String,
}

impl Defaults {
    pub const MAX_TURNS_KEY: &'static str = "llm.run.max.turns";
    pub const RESULT_LIMIT_KEY: &'static str = "llm.run.result.limit";
    pub const TRANSCRIPT_TTL_KEY: &'static str = "llm.run.transcript.ttl";
    pub const RETENTION_KEY: &'static str = "llm.run.transcript.retention";
    pub const TIMEOUT_KEY: &'static str = "llm.run.timeout";

    /// The built-in defaults of section 6.1.
    pub fn built_in() -> Defaults {
        Defaults {
            max_turns: "10".to_string(),
            result_limit: "65536".to_string(),
            transcript_ttl: "7d".to_string(),
            retention: "0".to_string(),
            timeout: "25s".to_string(),
        }
    }

    /// The defaults from the application's configuration, the built-in ones where a key is not set.
    pub fn from_config() -> Defaults {
        let config = AppConfigReader::get_instance();
        let built_in = Defaults::built_in();
        Defaults {
            max_turns: config.get_property_or(Self::MAX_TURNS_KEY, &built_in.max_turns),
            result_limit: config.get_property_or(Self::RESULT_LIMIT_KEY, &built_in.result_limit),
            transcript_ttl: config
                .get_property_or(Self::TRANSCRIPT_TTL_KEY, &built_in.transcript_ttl),
            retention: config.get_property_or(Self::RETENTION_KEY, &built_in.retention),
            timeout: config.get_property_or(Self::TIMEOUT_KEY, &built_in.timeout),
        }
    }
}

impl LoopSettings {
    /// Resolve the settings: a node input wins over the global default. Each value is checked, and a
    /// bad one names the node input, or the configuration key when the default is bad.
    pub fn resolve(input: &Value, defaults: &Defaults) -> Result<LoopSettings, String> {
        let mode = match get(input, MODE_INPUT) {
            None => WRITE.to_string(),
            given => value_of(given),
        };
        if !MODES.contains(&mode.as_str()) {
            return Err("mode must be write, auto, manual or unattended".to_string());
        }
        let max_turns = positive(
            MAX_TURNS_INPUT,
            get(input, MAX_TURNS_INPUT),
            Defaults::MAX_TURNS_KEY,
            &defaults.max_turns,
        )?;
        let result_limit = positive(
            RESULT_LIMIT_INPUT,
            get(input, RESULT_LIMIT_INPUT),
            Defaults::RESULT_LIMIT_KEY,
            &defaults.result_limit,
        )?;
        let transcript_ttl = duration(
            TRANSCRIPT_TTL_INPUT,
            get(input, TRANSCRIPT_TTL_INPUT),
            Defaults::TRANSCRIPT_TTL_KEY,
            &defaults.transcript_ttl,
            1,
        )?;
        let retention = duration(
            RETENTION_INPUT,
            get(input, RETENTION_INPUT),
            Defaults::RETENTION_KEY,
            &defaults.retention,
            0,
        )?;
        let timeout = duration(
            TIMEOUT_INPUT,
            get(input, TIMEOUT_INPUT),
            Defaults::TIMEOUT_KEY,
            &defaults.timeout,
            1,
        )?;
        Ok(LoopSettings {
            mode,
            max_turns,
            result_limit: result_limit as usize,
            transcript_ttl,
            retention,
            timeout_ms: seconds(&timeout) * 1000,
        })
    }
}

fn positive(name: &str, given: Option<&Value>, key: &str, fallback: &str) -> Result<i32, String> {
    let place = if given.is_none() { key } else { name };
    let fallback = Value::from(fallback);
    let value = given.unwrap_or(&fallback);
    let n = match value {
        Value::String(text) => {
            let text = java_trim(text.as_str().unwrap_or_default());
            if !text.is_empty() && text.bytes().all(|c| c.is_ascii_digit()) {
                Decimal::parse(text)
            } else {
                None
            }
        }
        number => Decimal::of(number),
    };
    match n.and_then(|n| n.whole()) {
        Some(n) if n > 0 && n <= i64::from(i32::MAX) => Ok(n as i32),
        _ => Err(format!("{place} must be a whole number >= 1")),
    }
}

/// A duration as the language packs read it: seconds, or a number with s, m, h or d. Returned as text.
fn duration(
    name: &str,
    given: Option<&Value>,
    key: &str,
    fallback: &str,
    minimum: i64,
) -> Result<String, String> {
    let place = if given.is_none() { key } else { name };
    let fallback = Value::from(fallback);
    let value = given.unwrap_or(&fallback);
    let text = match value {
        Value::String(s) => Some(java_trim(s.as_str().unwrap_or_default()).to_string()),
        number => Decimal::of(number)
            .filter(Decimal::is_integral)
            .map(|n| n.render()),
    };
    match text {
        Some(text) if is_duration(&text) && seconds(&text) >= minimum => Ok(text),
        _ => Err(format!("{place}{DURATION_RULE} >= {minimum}")),
    }
}

/// `(\d+)([smhd]?)`, matched whole, with ASCII digits (Java's `\d` without the Unicode flag).
fn is_duration(text: &str) -> bool {
    let digits = text.strip_suffix(['s', 'm', 'h', 'd']).unwrap_or(text);
    !digits.is_empty() && digits.bytes().all(|c| c.is_ascii_digit())
}

/// A duration in seconds; -1 when it is not one. Twelve digits are enough for any duration; a longer
/// one is refused rather than overflowed.
pub(crate) fn seconds(duration: &str) -> i64 {
    if !is_duration(duration) {
        return -1;
    }
    let (digits, unit) = match duration.strip_suffix(['s', 'm', 'h', 'd']) {
        Some(digits) => (digits, &duration[digits.len()..]),
        None => (duration, ""),
    };
    if digits.len() > 12 {
        return -1;
    }
    let n: i64 = digits.parse().unwrap_or(-1);
    match unit {
        "m" => n * 60,
        "h" => n * 3600,
        "d" => n * 86400,
        _ => n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_read_as_the_packs_read_them() {
        assert_eq!(seconds("30"), 30);
        assert_eq!(seconds("30s"), 30);
        assert_eq!(seconds("2m"), 120);
        assert_eq!(seconds("12h"), 43200);
        assert_eq!(seconds("7d"), 604800);
        assert_eq!(seconds("2w"), -1);
        assert_eq!(seconds("s"), -1);
        assert_eq!(seconds("1234567890123"), -1);
        assert_eq!(seconds("\u{0663}"), -1);
    }
}
