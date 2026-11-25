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
//! A task's provider named on the node (RFC-0017 D16; Java `TaskServer`): `server.url` is the
//! Event-over-HTTP endpoint of the engine or language pack that hosts the function the node's
//! `task` names, and `server.header[]` adds transport headers to the call, `name: value`, such
//! as a gateway's token.
//!
//! A hot-deployed graph may need a custom composable function the engine does not hold, and the
//! engine's Event-over-HTTP map (`yaml.event.over.http`) is loaded once at startup — so the node
//! says where its function is, and the graph reaches a function the map has never heard of
//! without a restart. The property wins over a local function and over the map: explicit beats
//! implicit. A URL without a path gets `/api/event`; a URL with a path is used as written, which
//! is how an API gateway's own path reaches the target. The values are read from the compiled
//! model, so a `${...}` reference resolved when the graph was registered, once per instance, from
//! its own environment. A compiled model holds the property nested — the config reader that reads
//! a graph back normalizes a dotted key into `server: {url: ...}` — while a draft in the Playground
//! holds the flat key the author typed; both forms are read.
use std::collections::HashMap;
use std::time::Duration;

use platform_core::graph::SimpleNode;
use platform_core::{AppError, EventEnvelope, PostOffice};
use rmpv::Value;

use event_script::conversions::display;

/// The node property that names the provider's Event-over-HTTP endpoint.
pub const URL: &str = "server.url";
/// The node property whose entries are the call's transport headers, `name: value`.
pub const HEADER: &str = "server.header";
const SERVER: &str = "server";
const URL_LEAF: &str = "url";
const HEADER_LEAF: &str = "header";
const DEFAULT_PATH: &str = "/api/event";

/// A task's provider: the endpoint the call goes to and the headers it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskServer {
    endpoint: String,
    headers: HashMap<String, String>,
}

impl TaskServer {
    /// The provider a node names, or `None` when the node has no `server.url`; an error names
    /// the malformed URL or header entry — the gate refuses such a graph, so this is reached by
    /// a draft in a dry run only.
    pub fn of(node: &SimpleNode) -> Result<Option<TaskServer>, String> {
        let Some(url) = url_of(node) else {
            return Ok(None);
        };
        check_url(&url)?;
        let header = header_of(node);
        check_headers(header.as_ref())?;
        Ok(Some(TaskServer {
            endpoint: normalize(display(&url).trim()),
            headers: headers_of(header.as_ref()),
        }))
    }

    /// The endpoint the call goes to, the default path supplied.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// The transport headers, possibly empty.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Call the function at the provider as a request-response, the deadline travelling as
    /// `X-TTL` as on every Event-over-HTTP call.
    pub async fn request(
        &self,
        po: &PostOffice,
        event: EventEnvelope,
        timeout: Duration,
    ) -> Result<EventEnvelope, AppError> {
        platform_core::automation::event_api::event_over_http_with_headers(
            po,
            &self.endpoint,
            event,
            timeout,
            true,
            &self.headers,
        )
        .await
    }
}

fn nested_or_flat(server: Option<Value>, leaf: &str, flat: Option<Value>) -> Option<Value> {
    if let Some(Value::Map(entries)) = server {
        if let Some((_, value)) = entries.iter().find(|(k, _)| k.as_str() == Some(leaf)) {
            if !value.is_nil() {
                return Some(value.clone());
            }
        }
    }
    flat
}

/// The node's `server.url`, nested or flat.
pub fn url_of(node: &SimpleNode) -> Option<Value> {
    nested_or_flat(node.get_property(SERVER), URL_LEAF, node.get_property(URL))
}

/// The node's `server.header[]`, nested or flat.
pub fn header_of(node: &SimpleNode) -> Option<Value> {
    nested_or_flat(
        node.get_property(SERVER),
        HEADER_LEAF,
        node.get_property(HEADER),
    )
}

/// Whether the node declares a provider at all, in either form — for the gate's placement rule.
pub fn declared_on(node: &SimpleNode) -> bool {
    node.get_property(SERVER).is_some()
        || node.get_property(URL).is_some()
        || node.get_property(HEADER).is_some()
}

/// The `server.url` of a node's property map as a model holds it, nested or flat — for a reader
/// of the compiled model, such as the deploy answer's advisory.
pub fn url_of_properties(properties: &Value) -> Option<Value> {
    let Value::Map(entries) = properties else {
        return None;
    };
    let get = |key: &str| {
        entries
            .iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .map(|(_, v)| v.clone())
    };
    nested_or_flat(get(SERVER), URL_LEAF, get(URL))
}

/// Whether a `server.url` value is an absolute http or https URL with a host.
pub fn check_url(url: &Value) -> Result<(), String> {
    let text = display(url);
    let text = text.trim();
    if text.is_empty() {
        return Err(format!(
            "'{URL}' is empty - a ${{...}} reference may be unresolved in this environment"
        ));
    }
    let lower = text.to_ascii_lowercase();
    let rest = lower
        .strip_prefix("http://")
        .or_else(|| lower.strip_prefix("https://"));
    let host = rest
        .map(|r| r.split('/').next().unwrap_or(""))
        .unwrap_or("");
    if host.is_empty() || host.contains(char::is_whitespace) {
        return Err(format!(
            "'{URL}' must be an absolute http or https URL, not '{text}'"
        ));
    }
    Ok(())
}

/// Whether every `server.header[]` entry is `name: value`.
pub fn check_headers(value: Option<&Value>) -> Result<(), String> {
    let Some(value) = value else {
        return Ok(());
    };
    let Value::Array(entries) = value else {
        return Err(format!("'{HEADER}' is a list of 'name: value' entries"));
    };
    for entry in entries {
        let text = display(entry);
        match text.find(':') {
            Some(colon) if colon > 0 && !text[..colon].trim().is_empty() => {}
            _ => return Err(format!("'{HEADER}' entry '{text}' is not 'name: value'")),
        }
    }
    Ok(())
}

fn headers_of(value: Option<&Value>) -> HashMap<String, String> {
    let mut result = HashMap::new();
    if let Some(Value::Array(entries)) = value {
        for entry in entries {
            let text = display(entry);
            if let Some(colon) = text.find(':') {
                result.insert(
                    text[..colon].trim().to_string(),
                    text[colon + 1..].trim().to_string(),
                );
            }
        }
    }
    result
}

/// A URL without a path (or with `/` alone) gets the Event API's default path.
fn normalize(url: &str) -> String {
    let scheme_end = url.find("://").map(|i| i + 3).unwrap_or(0);
    match url[scheme_end..].find('/') {
        Some(offset) if url[scheme_end + offset..] != *"/" => url.to_string(),
        Some(offset) => format!("{}{DEFAULT_PATH}", &url[..scheme_end + offset]),
        None => format!("{url}{DEFAULT_PATH}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_url_without_a_path_gets_the_default_path() {
        assert_eq!("http://h:1/api/event", normalize("http://h:1"));
        assert_eq!("http://h:1/api/event", normalize("http://h:1/"));
        assert_eq!(
            "https://gw/engine/api/event",
            normalize("https://gw/engine/api/event")
        );
        assert_eq!("https://gw/x", normalize("https://gw/x"));
    }

    #[test]
    fn the_url_must_be_absolute_with_a_host() {
        assert!(check_url(&Value::from("http://127.0.0.1:8086")).is_ok());
        assert!(check_url(&Value::from("/api/event")).is_err());
        assert!(check_url(&Value::from("")).is_err());
        assert!(check_url(&Value::from("ftp://h/api/event")).is_err());
    }

    #[test]
    fn a_header_entry_is_name_colon_value() {
        let good = Value::Array(vec![
            Value::from("authorization: Bearer x"),
            Value::from("x-a: b"),
        ]);
        assert!(check_headers(Some(&good)).is_ok());
        let bad = Value::Array(vec![Value::from("authorization")]);
        assert!(check_headers(Some(&bad)).is_err());
        assert!(check_headers(Some(&Value::from("not a list"))).is_err());
        assert_eq!(
            Some(&"Bearer x".to_string()),
            headers_of(Some(&good)).get("authorization")
        );
    }
}
