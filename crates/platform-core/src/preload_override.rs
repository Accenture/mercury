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

//! Java `yaml.preload.override` (AppStarter `getPreloadOverride` /
//! `overrideTasks` / `getMatchedPreload` / `overridePreloadInfo`): a
//! boot-time, config-driven transform over the collected `#[preload]` set —
//! rename, fan out, or re-tune the `instances` of any preloaded route
//! without recompiling. Part of the registration-metadata contract's boot
//! sequence: discover → register → **override (config)** → resolve (env) →
//! validate → route table (with the note that env-instances resolution
//! happens before the override applies, so the resolved value is the "old"
//! count in the log line — Java parity).
//!
//! The config key holds a comma-separated list of file locations
//! (`classpath:/...` or `file:/...`). A missing or malformed file is logged
//! and skipped, never a boot error. Each file carries a top-level `preload`
//! list; every entry:
//!
//! - `original` (required) — a route declared in a `#[preload]`;
//! - `routes` (required) — non-empty list of replacement route names;
//! - `instances` (optional) — replaces the resolved instance count when > 0;
//! - `keep-original: true` (optional) — adds the original route back into
//!   the replacement set.
//!
//! Multi-file merge (Java `overrideTasks`): for the same `original` across
//! files the route sets are UNIONed, and the FIRST file to set `instances`
//! wins.

use std::collections::{BTreeSet, HashMap};

use crate::util::app_config_reader::AppConfigReader;
use crate::util::config_reader::ConfigReader;
use crate::util::multi_level_map::ConfigValue;

/// One merged override entry (Java `PreLoadInfo`): the replacement route set
/// (sorted — `BTreeSet` mirrors Java's `Collections.sort` on apply) and the
/// instance count (`-1` = unset, Java `DEFAULT_INSTANCES`).
pub(crate) struct PreloadInfo {
    pub routes: BTreeSet<String>,
    pub instances: i64,
}

/// Load and merge every override file named by `yaml.preload.override`
/// (Java `getPreloadOverride`). A file that cannot be read or parsed is
/// logged and skipped, so a deployment can chain optional locations.
pub(crate) fn preload_override(config: &AppConfigReader) -> HashMap<String, PreloadInfo> {
    let mut result: HashMap<String, PreloadInfo> = HashMap::new();
    let Some(path) = config.get_property("yaml.preload.override") else {
        return result;
    };
    for location in path.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        match parse_override_file(location) {
            Ok(tasks) => merge_tasks(&mut result, tasks),
            Err(message) => {
                log::error!("Unable to load PreLoad entries from {location} - {message}");
            }
        }
    }
    result
}

/// Java `overrideTasks`: same `original` across files → route-set union;
/// `instances` keeps the first set value (`-1` = still unset).
fn merge_tasks(result: &mut HashMap<String, PreloadInfo>, tasks: Vec<(String, PreloadInfo)>) {
    for (original, info) in tasks {
        match result.get_mut(&original) {
            Some(prior) => {
                prior.routes.extend(info.routes);
                if prior.instances == -1 {
                    prior.instances = info.instances;
                }
            }
            None => {
                result.insert(original, info);
            }
        }
    }
}

/// Java `parsePreloadOverride` + `getOverrideEntry`: parse one override file.
/// Any shape error fails the WHOLE file with the Java message (the caller
/// logs and skips it).
fn parse_override_file(location: &str) -> Result<Vec<(String, PreloadInfo)>, String> {
    let config = ConfigReader::load(location).map_err(|e| e.to_string())?;
    let Some(ConfigValue::List(items)) = config.get("preload") else {
        return Err("preload must be a list of key-values for original and routes".to_string());
    };
    let mut result = Vec::new();
    for i in 0..items.len() {
        if !matches!(
            config.get(&format!("preload[{i}]")),
            Some(ConfigValue::Map(_))
        ) {
            return Err(format!("preload[{i}] is not a map of original and routes"));
        }
        let original = config
            .get_property(&format!("preload[{i}].original"))
            .unwrap_or_default();
        if original.is_empty() {
            return Err(format!("preload[{i}] does not contain 'original'"));
        }
        let Some(ConfigValue::List(route_items)) = config.get(&format!("preload[{i}].routes"))
        else {
            return Err(format!("preload[{i}].routes must be a list"));
        };
        let mut routes = BTreeSet::new();
        for j in 0..route_items.len() {
            if let Some(route) = config.get_property(&format!("preload[{i}].routes[{j}]")) {
                routes.insert(route);
            }
        }
        if config
            .get_property(&format!("preload[{i}].keep-original"))
            .as_deref()
            == Some("true")
        {
            routes.insert(original.clone());
        }
        let instances = config
            .get_property(&format!("preload[{i}].instances"))
            .and_then(|value| value.trim().parse::<i64>().ok())
            .unwrap_or(-1);
        result.push((original, PreloadInfo { routes, instances }));
    }
    Ok(result)
}

/// Java `getMatchedPreload` + `overridePreloadInfo`: match a function's
/// declared route list (comma-split, first match wins) against the merged
/// override map and apply — the route list is REPLACED by the override's
/// sorted route set, and a positive override `instances` replaces the
/// resolved count. Returns the effective `(routes, instances)`; a
/// non-matched function passes through unchanged.
pub(crate) fn apply(
    overrides: &HashMap<String, PreloadInfo>,
    declared_route: &str,
    resolved_instances: usize,
) -> (Vec<String>, usize) {
    let declared: Vec<String> = declared_route
        .split(',')
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(str::to_string)
        .collect();
    let Some(info) = declared.iter().find_map(|route| overrides.get(route)) else {
        return (declared, resolved_instances);
    };
    let routes: Vec<String> = info.routes.iter().cloned().collect();
    let rendered = format!("[{}]", routes.join(", "));
    if info.instances > 0 {
        log::info!(
            "Preload [{declared_route}] as {rendered}, instances {resolved_instances} to {}",
            info.instances
        );
        (routes, info.instances as usize)
    } else {
        log::info!("Preload [{declared_route}] as {rendered}");
        (routes, resolved_instances)
    }
}
