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

//! Java `Feature` — evaluate an `@OptionalService` condition against the
//! application configuration. Reserved for system use (the AppStarter decides
//! whether an `#[optional_service("…")]`-gated route registers).
//!
//! A condition is a comma-separated list evaluated with **OR** — if any term
//! holds, the service is required. Each term:
//! - `!term` — negated (required if `term` does not hold);
//! - `key=value` — required if config `key` equals `value` (case-insensitive);
//! - `key=` — required if config `key` equals `"true"`;
//! - `key` — required if config `key` equals `"true"`.

use crate::util::app_config_reader::AppConfigReader;

/// Java `Feature.isRequired`: does the optional-service condition hold? `None`
/// (no `@OptionalService`) means always required.
pub fn is_required(condition: Option<&str>, config: &AppConfigReader) -> bool {
    is_required_with(condition, |key| config.get_property(key))
}

/// The pure condition evaluator, parameterized on a config lookup so it can be
/// unit-tested without the `AppConfigReader` singleton.
fn is_required_with(condition: Option<&str>, lookup: impl Fn(&str) -> Option<String>) -> bool {
    let Some(value) = condition else {
        return true;
    };
    for term in value.split(',') {
        let statement = term.trim();
        let matched = match statement.strip_prefix('!') {
            Some(rest) => !evaluate(rest, &lookup),
            None => evaluate(statement, &lookup),
        };
        if matched {
            return true;
        }
    }
    false
}

fn evaluate(statement: &str, lookup: &impl Fn(&str) -> Option<String>) -> bool {
    let condition = statement.trim();
    if let Some(eq) = condition.find('=') {
        let key = condition[..eq].trim();
        let value = condition[eq + 1..].trim();
        if key.is_empty() {
            return false;
        }
        let want = if value.is_empty() { "true" } else { value };
        lookup(key).is_some_and(|actual| actual.eq_ignore_ascii_case(want))
    } else {
        lookup(condition).is_some_and(|actual| actual.eq_ignore_ascii_case("true"))
    }
}

#[cfg(test)]
mod tests {
    use super::is_required_with;
    use std::collections::HashMap;

    fn config(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |key: &str| map.get(key).cloned()
    }

    #[test]
    fn no_condition_is_always_required() {
        assert!(is_required_with(None, config(&[])));
    }

    #[test]
    fn key_equals_value() {
        let cfg = config(&[("app.env", "dev")]);
        assert!(is_required_with(Some("app.env=dev"), &cfg));
        assert!(!is_required_with(Some("app.env=prod"), &cfg));
        // unset key never matches an expected value (Java parity: no default-dev)
        assert!(!is_required_with(Some("missing.key=dev"), &cfg));
    }

    #[test]
    fn value_match_is_case_insensitive() {
        let cfg = config(&[("app.env", "Dev")]);
        assert!(is_required_with(Some("app.env=dev"), &cfg));
    }

    #[test]
    fn bare_key_and_empty_value_mean_true() {
        let cfg = config(&[("feature.on", "true"), ("feature.off", "false")]);
        assert!(is_required_with(Some("feature.on"), &cfg));
        assert!(is_required_with(Some("feature.on="), &cfg));
        assert!(!is_required_with(Some("feature.off"), &cfg));
        assert!(!is_required_with(Some("missing"), &cfg));
    }

    #[test]
    fn negation() {
        let cfg = config(&[("kill.switch", "true")]);
        assert!(!is_required_with(Some("!kill.switch"), &cfg));
        assert!(is_required_with(Some("!other.switch"), &cfg)); // unset → false → !false = true
    }

    #[test]
    fn comma_separated_is_or() {
        let cfg = config(&[("key.two", "200")]);
        assert!(is_required_with(Some("key.one=100,key.two=200"), &cfg));
        assert!(!is_required_with(Some("key.one=100,key.two=999"), &cfg));
    }

    #[test]
    fn empty_key_is_false() {
        assert!(!is_required_with(Some("=dev"), config(&[])));
    }
}
