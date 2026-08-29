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

//! Utility layer of platform-core — configuration management first
//! (`draft-design-specs/platform-core-port.md`, increment 1).

pub mod app_config_reader;
pub mod config_reader;
pub mod elastic_queue;
pub mod feature;
pub mod managed_cache;
pub mod multi_level_map;
pub mod overrides;
pub mod resources;
pub mod w3c_trace;

/// Human-readable elapsed time — an EXACT port of the Java
/// `Utility.elapsedTime(long)`, shared by the actuator (`/info` uptime) and
/// the ManagedCache create log (both are Java-parity log/presentation
/// surfaces). Java's quirks are kept verbatim: strict `>` at the day/hour/
/// minute boundaries (exactly 1 minute renders "60 seconds", exactly 1 day
/// renders "24 hours"), zero components are omitted, and a sub-second value
/// renders as "N ms".
pub(crate) fn elapsed_time(duration: std::time::Duration) -> String {
    const ONE_SECOND_MS: u128 = 1000;
    const ONE_MINUTE_MS: u128 = 60 * ONE_SECOND_MS;
    const ONE_HOUR_MS: u128 = 60 * ONE_MINUTE_MS;
    const ONE_DAY_MS: u128 = 24 * ONE_HOUR_MS;
    let mut time = duration.as_millis();
    let mut parts: Vec<String> = Vec::new();
    if time > ONE_DAY_MS {
        let days = time / ONE_DAY_MS;
        parts.push(format!("{days} day{}", if days == 1 { "" } else { "s" }));
        time -= days * ONE_DAY_MS;
    }
    if time > ONE_HOUR_MS {
        let hours = time / ONE_HOUR_MS;
        parts.push(format!("{hours} hour{}", if hours == 1 { "" } else { "s" }));
        time -= hours * ONE_HOUR_MS;
    }
    if time > ONE_MINUTE_MS {
        let minutes = time / ONE_MINUTE_MS;
        parts.push(format!(
            "{minutes} minute{}",
            if minutes == 1 { "" } else { "s" }
        ));
        time -= minutes * ONE_MINUTE_MS;
    }
    if time >= ONE_SECOND_MS {
        let seconds = time / ONE_SECOND_MS;
        parts.push(format!(
            "{seconds} second{}",
            if seconds == 1 { "" } else { "s" }
        ));
    }
    if parts.is_empty() {
        format!("{time} ms")
    } else {
        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn elapsed_time_matches_java_utility() {
        // sub-second renders as ms (Java: sb.isEmpty() -> "N ms")
        assert_eq!(elapsed_time(Duration::from_millis(0)), "0 ms");
        assert_eq!(elapsed_time(Duration::from_millis(500)), "500 ms");
        assert_eq!(elapsed_time(Duration::from_secs(1)), "1 second");
        assert_eq!(elapsed_time(Duration::from_secs(61)), "1 minute 1 second");
        // Java's strict `>` boundary quirks, kept verbatim
        assert_eq!(elapsed_time(Duration::from_secs(60)), "60 seconds");
        assert_eq!(elapsed_time(Duration::from_secs(3600)), "60 minutes");
        assert_eq!(elapsed_time(Duration::from_secs(86_400)), "24 hours");
        // zero components are omitted (Java: no seconds part when remainder 0)
        assert_eq!(elapsed_time(Duration::from_secs(120)), "2 minutes");
        assert_eq!(elapsed_time(Duration::from_secs(1800)), "30 minutes");
        assert_eq!(
            elapsed_time(Duration::from_secs(90_061)),
            "1 day 1 hour 1 minute 1 second"
        );
    }
}
