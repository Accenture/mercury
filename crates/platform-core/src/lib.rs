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

//! # platform-core (Rust port)
//!
//! Rust port of mercury-composable's `platform-core` — the event-driven
//! foundation layer. Canonical behavior spec: the Java project
//! (`system/platform-core`, v4.8.6) — see `docs/design/platform-core-port.md`.
//!
//! **Increment 1 — configuration management** (this release): the
//! `resources/` folder convention, [`MultiLevelMap`] composite keys,
//! [`ConfigReader`] with `${...}` substitution, and the [`AppConfigReader`]
//! base-config singleton.
//!
//! Increment 2 adds the event-bus foundation (`EventEnvelope`, the composable
//! function trait, `Platform`, `PostOffice`).

pub mod util;

pub use util::app_config_reader::AppConfigReader;
pub use util::config_reader::{ConfigError, ConfigReader};
pub use util::multi_level_map::{ConfigValue, MultiLevelMap};
pub use util::{overrides, resources};
