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

//! The routes an application declares as return paths (`return.path.routes`, RFC-0011 section
//! 8.4; Java `ReturnPaths`). A caller names its return route in an event's `x-return-path`, and the
//! graph engine sends progress events and the final result there only when the application
//! declares the route: otherwise a caller could make the engine send events to any route it can
//! name, a store or a write operation included.
//!
//! An entry that ends with `*` matches a route prefix, and any other entry matches one route
//! exactly - the rule `invocation.observer.include` applies to route names in the Java engine. A
//! bare `*`, a `*` anywhere but at the end, and an entry whose fixed part is not a valid route name
//! are refused at startup, each logged by name; a refused entry declares nothing.

use std::sync::OnceLock;

use event_script::util::split;
use platform_core::AppConfigReader;

pub const RETURN_PATH_ROUTES: &str = "return.path.routes";
const WILDCARD: &str = "*";

/// The declared return paths (Java `ReturnPaths`).
#[derive(Debug, Default)]
pub struct ReturnPaths {
    exact: Vec<String>,
    prefixes: Vec<String>,
    refusals: Vec<String>,
}

impl ReturnPaths {
    /// Parse a `return.path.routes` value: entries separated by commas or spaces (Java
    /// `util.split(text, ", ")`). A refused entry is kept as a message and declares nothing.
    pub fn new(text: &str) -> Self {
        let mut paths = ReturnPaths::default();
        for entry in split(text, ", ") {
            if let Some(refusal) = refusal(&entry) {
                paths.refusals.push(format!(
                    "{RETURN_PATH_ROUTES} entry '{entry}' refused - {refusal}"
                ));
            } else if let Some(prefix) = entry.strip_suffix(WILDCARD) {
                paths.prefixes.push(prefix.to_string());
            } else {
                paths.exact.push(entry);
            }
        }
        paths
    }

    /// The application's declared return paths, read once from `return.path.routes`. Each
    /// refused entry is logged when the list is first read, which the graph compiler does at
    /// startup.
    pub fn get_instance() -> &'static ReturnPaths {
        static CONFIGURED: OnceLock<ReturnPaths> = OnceLock::new();
        CONFIGURED.get_or_init(|| {
            let text = AppConfigReader::get_instance().get_property_or(RETURN_PATH_ROUTES, "");
            let paths = ReturnPaths::new(&text);
            for refusal in &paths.refusals {
                log::error!("{refusal}");
            }
            if !paths.exact.is_empty() || !paths.prefixes.is_empty() {
                // Java List.toString rendering, so both engines log the same line
                log::info!(
                    "Return paths declared - routes [{}], prefixes [{}]",
                    paths.exact.join(", "),
                    paths.prefixes.join(", ")
                );
            }
            paths
        })
    }

    /// Whether the application declares a route as a return path.
    pub fn is_declared(&self, route: &str) -> bool {
        if !valid_service_name(route) {
            return false;
        }
        self.exact.iter().any(|exact| exact == route)
            || self
                .prefixes
                .iter()
                .any(|prefix| route.starts_with(prefix.as_str()))
    }

    /// The refused entries, each message naming the entry and why, in the order of the list.
    pub fn refusals(&self) -> Vec<String> {
        self.refusals.clone()
    }
}

fn refusal(entry: &str) -> Option<String> {
    if entry == WILDCARD {
        return Some("a bare '*' would declare every route".to_string());
    }
    let star = entry.find(WILDCARD);
    if let Some(at) = star {
        if at != entry.len() - 1 {
            return Some("'*' may only end an entry".to_string());
        }
    }
    let fixed = match star {
        Some(at) => &entry[..at],
        None => entry,
    };
    // a prefix may end on a whole segment (order.status.*), so its trailing separator is not part
    // of the name
    let name = match star {
        Some(_) => fixed.strip_suffix('.').unwrap_or(fixed),
        None => fixed,
    };
    if !valid_service_name(name) || !name.contains('.') {
        return Some(format!("'{name}' is not a valid route name"));
    }
    None
}

/// Java `Utility.validServiceName`: lowercase letters, digits, `.`, `_` and `-`, neither starting
/// nor ending with a separator, and no `..`.
fn valid_service_name(text: &str) -> bool {
    if text.is_empty()
        || text.starts_with(['.', '_', '-'])
        || text.ends_with(['.', '_', '-'])
        || text.contains("..")
    {
        return false;
    }
    text.bytes()
        .all(|b| b.is_ascii_digit() || b.is_ascii_lowercase() || matches!(b, b'.' | b'_' | b'-'))
}

#[cfg(test)]
mod tests {
    //! The declared return paths (Java `ReturnPathsTest`): a prefix entry ends with `*`, any other
    //! entry is one route, and an entry that would declare too much, or nothing a route could be,
    //! is refused by name.

    use super::ReturnPaths;

    #[test]
    fn a_prefix_entry_matches_whole_segments_and_an_exact_entry_one_route() {
        let paths = ReturnPaths::new("order.status.*, progress.relay, kafka.progress.publisher");
        assert!(paths.refusals().is_empty(), "{:?}", paths.refusals());
        assert!(paths.is_declared("order.status.kafka"));
        assert!(paths.is_declared("order.status.mail.v2"));
        assert!(
            !paths.is_declared("order.status"),
            "the prefix itself is not a route in the family"
        );
        assert!(
            !paths.is_declared("order.statuses"),
            "the dot before the '*' ends the prefix on a segment"
        );
        assert!(paths.is_declared("progress.relay"));
        assert!(
            !paths.is_declared("progress.relay.v2"),
            "an exact entry declares one route"
        );
        assert!(paths.is_declared("kafka.progress.publisher"));
        assert!(!paths.is_declared("audit.store"));
    }

    #[test]
    fn a_route_that_is_not_a_route_name_is_never_declared() {
        let paths = ReturnPaths::new("order.status.*");
        // Java's null route has no Rust form; the empty name is the closest
        assert!(!paths.is_declared(""));
        assert!(!paths.is_declared("order.status."));
        assert!(!paths.is_declared("ORDER.status.kafka"));
        assert!(!paths.is_declared("order.status.kafka@another-app"));
    }

    #[test]
    fn an_entry_that_declares_too_much_or_nothing_is_refused_by_name() {
        let paths =
            ReturnPaths::new("*, order.*.status, order*, Order.Status, order.status.*, good.route");
        assert_eq!(
            paths.refusals(),
            vec![
                "return.path.routes entry '*' refused - a bare '*' would declare every route",
                "return.path.routes entry 'order.*.status' refused - '*' may only end an entry",
                "return.path.routes entry 'order*' refused - 'order' is not a valid route name",
                "return.path.routes entry 'Order.Status' refused - 'Order.Status' is not a valid route name",
            ]
        );
        assert!(
            !paths.is_declared("order.kafka"),
            "a refused entry declares nothing"
        );
        assert!(
            paths.is_declared("order.status.kafka"),
            "the valid entries still declare their routes"
        );
        assert!(paths.is_declared("good.route"));
    }

    #[test]
    fn an_empty_list_declares_nothing() {
        assert!(!ReturnPaths::new("").is_declared("any.route"));
        assert!(!ReturnPaths::new("  ,  ").is_declared("any.route"));
    }
}
