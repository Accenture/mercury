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

//! Rust port of the Java `AppConfigReader`
//! (`org.platformlambda.core.util.AppConfigReader`) — the process-wide base
//! configuration singleton.
//!
//! Bootstrapped by an `app-config-reader.yml` manifest (an application copy in
//! its `resources/` overrides the built-in default): the `resources:` list names
//! the base configuration files merged **in order** (later files override
//! earlier ones), and `profiles:` gives the overlay prefix for active profiles.
//!
//! Active profiles come from the `SPRING_PROFILES_ACTIVE` environment variable,
//! the `spring.profiles.active` process override, or the consolidated config key
//! — names kept **verbatim** from the Java original for side-by-side comparison
//! during migration (Spring itself is not ported; a generic
//! `app.profiles.active` alias may be added once the foundation is robust).

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::util::config_reader::ConfigReader;
use crate::util::multi_level_map::{ConfigValue, MultiLevelMap};
use crate::util::overrides;

const APP_CONFIG_READER_YML: &str = "classpath:/app-config-reader.yml";
const DEFAULT_PROFILE_PREFIX: &str = "classpath:/application-";
const SPRING_ACTIVE_PROFILES: &str = "spring.profiles.active";
const ENV_SPRING_ACTIVE_PROFILES: &str = "SPRING_PROFILES_ACTIVE";

/// Built-in default manifest, embedded so a consuming application works without
/// shipping its own copy (the Java analog: the file inside platform-core.jar).
const EMBEDDED_MANIFEST: &str = include_str!("../../resources/app-config-reader.yml");

static INSTANCE: OnceLock<AppConfigReader> = OnceLock::new();

/// Crate-internal: the global base config for `${...}` reference resolution,
/// once initialized (Java `ConfigReader.baseConfig`). Deliberately does **not**
/// auto-initialize — mirroring Java, where the base is set only when
/// `AppConfigReader.getInstance()` first runs.
pub(crate) fn try_base_reader() -> Option<&'static ConfigReader> {
    INSTANCE.get().map(|app| &app.reader)
}

/// The singleton holding the merged base configuration
/// (`bootstrap.properties` → `bootstrap.yml` → `application.properties` →
/// `application.yml`, plus active-profile overlays).
pub struct AppConfigReader {
    reader: ConfigReader,
}

impl AppConfigReader {
    /// The process-wide instance, built on first use (Java `getInstance()`).
    pub fn get_instance() -> &'static AppConfigReader {
        INSTANCE.get_or_init(Self::build)
    }

    fn build() -> Self {
        let (files, profile_prefix) = load_manifest();
        let mut consolidated: BTreeMap<String, ConfigValue> = BTreeMap::new();
        // load base configuration file(s), in order — later files override earlier
        for filename in &files {
            merge_config(&mut consolidated, filename);
        }
        // load additional configuration file(s) for active profiles
        let profiles = active_profiles(&consolidated);
        let mut updated = 0usize;
        for profile in &profiles {
            updated += merge_config(
                &mut consolidated,
                &format!("{profile_prefix}{profile}.properties"),
            );
            updated += merge_config(&mut consolidated, &format!("{profile_prefix}{profile}.yml"));
        }
        if updated > 0 {
            log::info!(
                "Updated {updated} parameter{} from active profiles {profiles:?}",
                if updated == 1 { "" } else { "s" }
            );
        }
        // the dataset is normalized from a sorted flat map of key-values
        let map = MultiLevelMap::from_flat_map(&consolidated);
        let reader = ConfigReader::new_base(map);
        if reader.is_empty() {
            log::error!("Configuration is empty - please check");
        }
        AppConfigReader { reader }
    }

    // ---- delegating lookup API (Java ConfigBase) ----

    pub fn get(&self, key: &str) -> Option<ConfigValue> {
        self.reader.get(key)
    }

    pub fn get_or(&self, key: &str, default: ConfigValue) -> ConfigValue {
        self.reader.get_or(key, default)
    }

    pub fn get_property(&self, key: &str) -> Option<String> {
        self.reader.get_property(key)
    }

    pub fn get_property_or(&self, key: &str, default: &str) -> String {
        self.reader.get_property_or(key, default)
    }

    pub fn exists(&self, key: &str) -> bool {
        self.reader.exists(key)
    }

    pub fn is_empty(&self) -> bool {
        self.reader.is_empty()
    }

    pub fn get_map(&self) -> &MultiLevelMap {
        self.reader.get_map()
    }

    pub fn get_composite_key_values(&self) -> &BTreeMap<String, ConfigValue> {
        self.reader.get_composite_key_values()
    }

    pub fn is_base_config(&self) -> bool {
        self.reader.is_base_config()
    }
}

/// Read the manifest (application copy from the resource roots, else the
/// embedded default): the ordered file list and the profile prefix.
fn load_manifest() -> (Vec<String>, String) {
    let manifest = match ConfigReader::load_raw(APP_CONFIG_READER_YML) {
        Ok(reader) => reader.get_map().clone(),
        Err(_) => {
            let value: serde_yaml::Value = serde_yaml::from_str(EMBEDDED_MANIFEST)
                .expect("embedded app-config-reader.yml is valid YAML");
            match ConfigValue::from_yaml(&value) {
                ConfigValue::Map(m) => MultiLevelMap::from_map(m),
                _ => MultiLevelMap::new(),
            }
        }
    };
    let files = match manifest.get_element("resources") {
        Some(ConfigValue::List(list)) => list
            .iter()
            .filter_map(|v| v.as_text().map(str::to_string))
            .collect(),
        _ => {
            // Java: "missing 'resources' section in app-config-reader.yml"
            log::error!("Unable to parse configuration - missing 'resources' section in app-config-reader.yml");
            Vec::new()
        }
    };
    let prefix = match manifest.get_element("profiles") {
        Some(ConfigValue::Text(p)) => p.clone(),
        _ => DEFAULT_PROFILE_PREFIX.to_string(),
    };
    (files, prefix)
}

/// Merge one configuration file's flat key-values into the consolidated map.
/// A missing file is skipped silently (Java catches `IllegalArgumentException`).
fn merge_config(consolidated: &mut BTreeMap<String, ConfigValue>, filename: &str) -> usize {
    match ConfigReader::load_raw(filename) {
        Ok(reader) => {
            let flat = reader.get_map().flat_map();
            if !flat.is_empty() {
                log::info!("Loaded {filename}");
                let n = flat.len();
                consolidated.extend(flat);
                n
            } else {
                0
            }
        }
        Err(_) => 0, // ok to ignore
    }
}

/// Resolve the active profiles: environment variable → process override →
/// consolidated config key (comma/space separated).
fn active_profiles(consolidated: &BTreeMap<String, ConfigValue>) -> Vec<String> {
    let raw = std::env::var(ENV_SPRING_ACTIVE_PROFILES)
        .ok()
        .or_else(|| overrides::get(SPRING_ACTIVE_PROFILES))
        .or_else(|| {
            consolidated
                .get(SPRING_ACTIVE_PROFILES)
                .and_then(|v| v.as_text().map(str::to_string))
        });
    match raw {
        Some(text) => text
            .split([',', ' '])
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect(),
        None => Vec::new(),
    }
}
