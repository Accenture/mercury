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

//! `rest.yaml` parsing and URL matching — Rust port of the Java `RoutingEntry`
//! (`org.platformlambda.automation.config.RoutingEntry`), following the
//! authoritative grammar in the Java project's
//! `docs/guides/rest-automation/rest-grammar.md`.
//!
//! Increment-6 scope is **function binding**: `service` is a composable
//! function route. Deferred (see the design doc §5e/§7): flow binding
//! (`http.flow.adapter`), HTTP(S) relay (`url_rewrite`/`trust_all_cert`),
//! A/B dual service, multipart upload, and `static-content`.

use std::collections::HashMap;
use std::time::Duration;

use crate::function::AppError;
use crate::util::config_reader::ConfigReader;
use crate::util::multi_level_map::ConfigValue;

const ALLOWED_METHODS: [&str; 6] = ["GET", "PUT", "POST", "DELETE", "HEAD", "PATCH"];
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const MIN_TIMEOUT: Duration = Duration::from_secs(1);
const MAX_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

/// One `rest:` endpoint entry (Java `RouteInfo`).
#[derive(Clone, Debug)]
pub struct RouteInfo {
    pub service: String,
    pub methods: Vec<String>,
    /// URL segments as authored; `{param}` captures and a trailing `*` wildcard.
    pub url: String,
    pub timeout: Duration,
    pub cors: Option<CorsInfo>,
    pub headers: Option<HeaderInfo>,
    /// Simple form only: a function route that authorizes the request.
    pub authentication: Option<String>,
    pub tracing: bool,
    /// Per-endpoint impedance overrides (grammar: `trace.id.header`,
    /// `correlation.id.header`).
    pub trace_id_header: Option<String>,
    pub correlation_id_header: Option<String>,
    segments: Vec<Segment>,
}

#[derive(Clone, Debug)]
enum Segment {
    /// Case-insensitive literal (stored lowercase).
    Literal(String),
    /// `{param}` capture with its name.
    Param(String),
    /// Trailing `*` — matches the remainder.
    Wildcard,
}

/// A reusable `cors:` block (Java `CorsInfo`).
#[derive(Clone, Debug, Default)]
pub struct CorsInfo {
    pub id: String,
    /// headers for the preflight (OPTIONS) response
    pub options: Vec<(String, String)>,
    /// headers added to normal responses
    pub headers: Vec<(String, String)>,
}

/// A reusable `headers:` block (Java `HeaderInfo`): request/response
/// add / drop / keep transforms.
#[derive(Clone, Debug, Default)]
pub struct HeaderInfo {
    pub id: String,
    pub request: HeaderTransform,
    pub response: HeaderTransform,
}

#[derive(Clone, Debug, Default)]
pub struct HeaderTransform {
    pub add: Vec<(String, String)>,
    pub drop: Vec<String>,
    pub keep: Vec<String>,
}

impl HeaderTransform {
    /// Apply this transform to a header map (names matched case-insensitively;
    /// grammar: `keep` retains only the listed headers, then `drop` removes,
    /// then `add` appends).
    pub fn apply(&self, headers: &mut HashMap<String, String>) {
        if !self.keep.is_empty() {
            let keep: Vec<String> = self.keep.iter().map(|k| k.to_lowercase()).collect();
            headers.retain(|name, _| keep.contains(&name.to_lowercase()));
        }
        for name in &self.drop {
            let name = name.to_lowercase();
            headers.retain(|existing, _| existing.to_lowercase() != name);
        }
        for (name, value) in &self.add {
            headers.insert(name.clone(), value.clone());
        }
    }
}

/// The `static-content` block (Java `RoutingEntry` + `SimpleHttpFilter`):
/// no-cache pages and an optional request filter for static assets.
#[derive(Clone, Debug)]
pub struct StaticContent {
    /// Pages served with no-cache headers instead of the etag protocol
    /// (default `["/", "/index.html"]` — entry pages must never be cached,
    /// e.g. for SSO redirection).
    pub no_cache_pages: Vec<String>,
    pub filter: Option<SimpleHttpFilter>,
}

impl Default for StaticContent {
    fn default() -> Self {
        StaticContent {
            no_cache_pages: vec!["/".to_string(), "/index.html".to_string()],
            filter: None,
        }
    }
}

/// A composable function that inspects HTTP requests for configured static
/// paths (Java `SimpleHttpFilter`) — e.g. a backend handling SSO redirection
/// for a UI bundle. Patterns: exact, `prefix*`, or `*suffix`.
#[derive(Clone, Debug)]
pub struct SimpleHttpFilter {
    pub path_list: Vec<String>,
    pub exclusion_list: Vec<String>,
    pub service: String,
}

/// Match a path against exact / `prefix*` / `*suffix` patterns
/// (Java `HttpRouter.matchedElement`).
pub fn matched_element(elements: &[String], path: &str) -> bool {
    elements.iter().any(|pattern| {
        if let Some(prefix) = pattern.strip_suffix('*') {
            path.starts_with(prefix)
        } else if let Some(suffix) = pattern.strip_prefix('*') {
            path.ends_with(suffix)
        } else {
            path == pattern
        }
    })
}

/// A valid filter pattern has at most one `*`, and only at the start or end
/// (Java `invalidFilterParameters`, inverted).
fn valid_patterns(patterns: &[String]) -> bool {
    patterns.iter().all(|p| {
        !p.is_empty()
            && match p.matches('*').count() {
                0 => true,
                1 => p.starts_with('*') || p.ends_with('*'),
                _ => false,
            }
    })
}

/// The parsed routing table (Java `RoutingEntry`).
pub struct RoutingTable {
    routes: Vec<RouteInfo>,
    static_content: StaticContent,
}

/// A matched route with its extracted `{param}` path variables.
pub struct AssignedRoute<'a> {
    pub info: &'a RouteInfo,
    pub path_params: HashMap<String, String>,
}

impl RoutingTable {
    /// Load and validate a `rest.yaml` (Java `RoutingEntry.load`). Invalid
    /// entries fail the load — the grammar's parser invariants.
    pub fn load(reader: &ConfigReader) -> Result<Self, AppError> {
        let map = reader.get_map();
        // reusable blocks first, so entry references can be validated
        let cors_blocks = parse_cors_blocks(map.get_element("cors"))?;
        let header_blocks = parse_header_blocks(map.get_element("headers"))?;
        let Some(ConfigValue::List(entries)) = map.get_element("rest") else {
            return Err(AppError::new(400, "rest.yaml has no 'rest' section"));
        };
        let mut routes = Vec::new();
        for (index, entry) in entries.iter().enumerate() {
            let ConfigValue::Map(entry) = entry else {
                return Err(AppError::new(400, format!("rest[{index}] is not a map")));
            };
            routes.push(parse_route(index, entry, &cors_blocks, &header_blocks)?);
        }
        let static_content = parse_static_content(reader)?;
        Ok(RoutingTable {
            routes,
            static_content,
        })
    }

    /// The `static-content` configuration (defaults applied when absent).
    pub fn static_content(&self) -> &StaticContent {
        &self.static_content
    }

    /// Build a routing table from YAML text (used for the built-in default
    /// endpoints — same parser, same invariants).
    pub fn from_yaml_text(yaml: &str) -> Result<Self, AppError> {
        let value: serde_yaml::Value =
            serde_yaml::from_str(yaml).map_err(|e| AppError::new(400, e.to_string()))?;
        match ConfigValue::from_yaml(&value) {
            ConfigValue::Map(map) => {
                let reader = ConfigReader::from_map(map);
                Self::load(&reader)
            }
            _ => Err(AppError::new(400, "rest.yaml text must be a YAML mapping")),
        }
    }

    pub fn routes(&self) -> &[RouteInfo] {
        &self.routes
    }

    /// Whether any entry already claims this URL (used by the default-endpoint
    /// merge: user entries always win — Java `default-rest.yaml` semantics).
    pub fn has_url(&self, url: &str) -> bool {
        self.routes.iter().any(|r| r.url.eq_ignore_ascii_case(url))
    }

    pub(crate) fn add_route(&mut self, route: RouteInfo) {
        self.routes.push(route);
    }

    /// Match a request (Java `getRouteInfo`): candidates must match every
    /// segment; precedence is more literal segments first, wildcard last.
    /// Method filtering happens here too — `OPTIONS` matches any entry (CORS
    /// preflight, auto-added per the grammar).
    pub fn find(&self, method: &str, path: &str) -> Option<AssignedRoute<'_>> {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut best: Option<(usize, bool, AssignedRoute)> = None;
        for info in &self.routes {
            if method != "OPTIONS" && !info.methods.iter().any(|m| m == method) {
                continue;
            }
            let Some(params) = info.match_path(&segments) else {
                continue;
            };
            let literals = info
                .segments
                .iter()
                .filter(|s| matches!(s, Segment::Literal(_)))
                .count();
            let wildcard = info.segments.iter().any(|s| matches!(s, Segment::Wildcard));
            let better = match &best {
                None => true,
                // prefer non-wildcard, then more literal segments
                Some((best_literals, best_wildcard, _)) => {
                    (!wildcard && *best_wildcard)
                        || (wildcard == *best_wildcard && literals > *best_literals)
                }
            };
            if better {
                best = Some((
                    literals,
                    wildcard,
                    AssignedRoute {
                        info,
                        path_params: params,
                    },
                ));
            }
        }
        best.map(|(_, _, assigned)| assigned)
    }
}

impl RouteInfo {
    /// Match URL segments against this entry (case-insensitive literals),
    /// returning the extracted `{param}` values on success.
    fn match_path(&self, request_segments: &[&str]) -> Option<HashMap<String, String>> {
        let mut params = HashMap::new();
        let mut request_index = 0;
        for segment in &self.segments {
            match segment {
                Segment::Wildcard => return Some(params), // consumes the rest (even empty)
                Segment::Literal(expected) => {
                    let actual = request_segments.get(request_index)?;
                    if actual.to_lowercase() != *expected {
                        return None;
                    }
                    request_index += 1;
                }
                Segment::Param(name) => {
                    let actual = request_segments.get(request_index)?;
                    params.insert(name.clone(), actual.to_string());
                    request_index += 1;
                }
            }
        }
        if request_index == request_segments.len() {
            Some(params)
        } else {
            None
        }
    }
}

/// Navigate a possibly-dotted key: config normalization nests dotted YAML keys
/// (`trace.id.header` → `trace: {id: {header: …}}`), exactly as the Java
/// ConfigReader does — so look up the direct key first, then walk the dots.
fn lookup<'a>(
    map: &'a std::collections::BTreeMap<String, ConfigValue>,
    key: &str,
) -> Option<&'a ConfigValue> {
    if let Some(value) = map.get(key) {
        return Some(value);
    }
    let mut parts = key.split('.');
    let mut current = map.get(parts.next()?)?;
    for part in parts {
        match current {
            ConfigValue::Map(nested) => current = nested.get(part)?,
            _ => return None,
        }
    }
    Some(current)
}

fn parse_route(
    index: usize,
    entry: &std::collections::BTreeMap<String, ConfigValue>,
    cors_blocks: &HashMap<String, CorsInfo>,
    header_blocks: &HashMap<String, HeaderInfo>,
) -> Result<RouteInfo, AppError> {
    let text = |key: &str| {
        lookup(entry, key)
            .and_then(|v| v.as_text())
            .map(str::to_string)
    };
    // invariant 1: service, methods, url are required
    let service = text("service")
        .ok_or_else(|| AppError::new(400, format!("rest[{index}] missing 'service'")))?;
    if service.starts_with("http://") || service.starts_with("https://") {
        return Err(AppError::new(
            400,
            format!("rest[{index}] HTTP relay is not yet ported (service '{service}')"),
        ));
    }
    let url =
        text("url").ok_or_else(|| AppError::new(400, format!("rest[{index}] missing 'url'")))?;
    let Some(ConfigValue::List(raw_methods)) = entry.get("methods") else {
        return Err(AppError::new(
            400,
            format!("rest[{index}] missing 'methods' list"),
        ));
    };
    // invariant 2: methods from the allowed set; OPTIONS is auto-added
    let mut methods = Vec::new();
    for method in raw_methods {
        let method = method
            .as_text()
            .map(str::to_uppercase)
            .ok_or_else(|| AppError::new(400, format!("rest[{index}] method must be text")))?;
        if !ALLOWED_METHODS.contains(&method.as_str()) {
            return Err(AppError::new(
                400,
                format!("rest[{index}] invalid method '{method}' (allowed: {ALLOWED_METHODS:?})"),
            ));
        }
        methods.push(method);
    }
    // segments: literal / {param} / trailing *
    let mut segments = Vec::new();
    let parts: Vec<&str> = url.split('/').filter(|s| !s.is_empty()).collect();
    for (position, part) in parts.iter().enumerate() {
        if let Some(name) = part.strip_prefix('{').and_then(|p| p.strip_suffix('}')) {
            segments.push(Segment::Param(name.to_string()));
        } else if *part == "*" {
            if position != parts.len() - 1 {
                return Err(AppError::new(
                    400,
                    format!("rest[{index}] wildcard '*' must be the trailing segment"),
                ));
            }
            segments.push(Segment::Wildcard);
        } else {
            segments.push(Segment::Literal(part.to_lowercase()));
        }
    }
    // invariant 3: cors/headers references must exist
    let cors =
        match text("cors") {
            Some(id) => Some(cors_blocks.get(&id).cloned().ok_or_else(|| {
                AppError::new(400, format!("rest[{index}] unknown cors id '{id}'"))
            })?),
            None => None,
        };
    let headers = match text("headers") {
        Some(id) => Some(header_blocks.get(&id).cloned().ok_or_else(|| {
            AppError::new(400, format!("rest[{index}] unknown headers id '{id}'"))
        })?),
        None => None,
    };
    let tracing = matches!(entry.get("tracing"), Some(ConfigValue::Bool(true)));
    Ok(RouteInfo {
        service,
        methods,
        url,
        timeout: parse_timeout(text("timeout").as_deref()),
        cors,
        headers,
        authentication: text("authentication"),
        tracing,
        trace_id_header: text("trace.id.header"),
        correlation_id_header: text("correlation.id.header"),
        segments,
    })
}

/// Parse a duration like `30s`, `500ms`, `2m` (default 30 s; clamped 1 s–5 m,
/// Java parity).
fn parse_timeout(value: Option<&str>) -> Duration {
    let parsed = value.and_then(|text| {
        let text = text.trim().to_lowercase();
        if let Some(ms) = text.strip_suffix("ms") {
            ms.trim().parse::<u64>().ok().map(Duration::from_millis)
        } else if let Some(minutes) = text.strip_suffix('m') {
            minutes
                .trim()
                .parse::<u64>()
                .ok()
                .map(|m| Duration::from_secs(m * 60))
        } else if let Some(seconds) = text.strip_suffix('s') {
            seconds.trim().parse::<u64>().ok().map(Duration::from_secs)
        } else {
            text.parse::<u64>().ok().map(Duration::from_secs)
        }
    });
    parsed
        .unwrap_or(DEFAULT_TIMEOUT)
        .clamp(MIN_TIMEOUT, MAX_TIMEOUT)
}

fn parse_cors_blocks(section: Option<&ConfigValue>) -> Result<HashMap<String, CorsInfo>, AppError> {
    let mut blocks = HashMap::new();
    let Some(ConfigValue::List(entries)) = section else {
        return Ok(blocks);
    };
    for entry in entries {
        let ConfigValue::Map(map) = entry else {
            continue;
        };
        let Some(id) = map.get("id").and_then(|v| v.as_text()) else {
            return Err(AppError::new(400, "cors block missing 'id'"));
        };
        let mut info = CorsInfo {
            id: id.to_string(),
            ..CorsInfo::default()
        };
        info.options = parse_header_lines(map.get("options"), id, "options")?;
        info.headers = parse_header_lines(map.get("headers"), id, "headers")?;
        blocks.insert(info.id.clone(), info);
    }
    Ok(blocks)
}

/// Each cors line must be `Access-Control-*: value` (grammar invariant).
fn parse_header_lines(
    list: Option<&ConfigValue>,
    id: &str,
    kind: &str,
) -> Result<Vec<(String, String)>, AppError> {
    let mut out = Vec::new();
    if let Some(ConfigValue::List(lines)) = list {
        for line in lines {
            let Some(line) = line.as_text() else { continue };
            let Some((name, value)) = line.split_once(':') else {
                return Err(AppError::new(
                    400,
                    format!("cors '{id}' {kind} line '{line}' is not 'name: value'"),
                ));
            };
            let name = name.trim();
            if !name.to_lowercase().starts_with("access-control-") {
                return Err(AppError::new(
                    400,
                    format!("cors '{id}' {kind} line '{name}' must be an Access-Control-* header"),
                ));
            }
            out.push((name.to_string(), value.trim().to_string()));
        }
    }
    Ok(out)
}

fn parse_header_blocks(
    section: Option<&ConfigValue>,
) -> Result<HashMap<String, HeaderInfo>, AppError> {
    let mut blocks = HashMap::new();
    let Some(ConfigValue::List(entries)) = section else {
        return Ok(blocks);
    };
    for entry in entries {
        let ConfigValue::Map(map) = entry else {
            continue;
        };
        let Some(id) = map.get("id").and_then(|v| v.as_text()) else {
            return Err(AppError::new(400, "headers block missing 'id'"));
        };
        blocks.insert(
            id.to_string(),
            HeaderInfo {
                id: id.to_string(),
                request: parse_transform(map.get("request")),
                response: parse_transform(map.get("response")),
            },
        );
    }
    Ok(blocks)
}

fn parse_transform(section: Option<&ConfigValue>) -> HeaderTransform {
    let mut transform = HeaderTransform::default();
    let Some(ConfigValue::Map(map)) = section else {
        return transform;
    };
    if let Some(ConfigValue::List(add)) = map.get("add") {
        for line in add {
            if let Some((name, value)) = line.as_text().and_then(|l| l.split_once(':')) {
                transform
                    .add
                    .push((name.trim().to_string(), value.trim().to_string()));
            }
        }
    }
    for (key, target) in [("drop", &mut transform.drop), ("keep", &mut transform.keep)] {
        if let Some(ConfigValue::List(names)) = map.get(key) {
            for name in names {
                if let Some(name) = name.as_text() {
                    target.push(name.to_string());
                }
            }
        }
    }
    transform
}

/// Parse the optional `static-content` block (Java `getNoCacheConfig` +
/// `getStaticContentFilter`). Missing block → defaults; an invalid filter is
/// an error (the grammar's fail-on-invalid discipline).
fn parse_static_content(reader: &ConfigReader) -> Result<StaticContent, AppError> {
    let mut result = StaticContent::default();
    if let Some(ConfigValue::List(pages)) = reader
        .get_map()
        .get_element("static-content.no-cache-pages")
    {
        let list: Vec<String> = pages
            .iter()
            .filter_map(|v| v.as_text().map(str::to_string))
            .collect();
        if valid_patterns(&list) && !list.is_empty() {
            result.no_cache_pages = list;
        } else {
            return Err(AppError::new(
                400,
                "static-content.no-cache-pages has invalid syntax",
            ));
        }
    }
    let map = reader.get_map();
    if map.key_exists("static-content.filter") {
        let Some(ConfigValue::List(paths)) = map.get_element("static-content.filter.path") else {
            return Err(AppError::new(
                400,
                "static-content.filter.path must be a list",
            ));
        };
        let path_list: Vec<String> = paths
            .iter()
            .filter_map(|v| v.as_text().map(str::to_string))
            .collect();
        let service = map
            .get_element("static-content.filter.service")
            .and_then(|v| v.as_text())
            .map(str::to_string)
            .ok_or_else(|| AppError::new(400, "static-content.filter.service is required"))?;
        let exclusion_list: Vec<String> = match map.get_element("static-content.filter.exclusion") {
            Some(ConfigValue::List(items)) => items
                .iter()
                .filter_map(|v| v.as_text().map(str::to_string))
                .collect(),
            _ => Vec::new(),
        };
        if path_list.is_empty() || !valid_patterns(&path_list) || !valid_patterns(&exclusion_list) {
            return Err(AppError::new(
                400,
                "static-content.filter path/exclusion has invalid syntax",
            ));
        }
        log::info!("static-content.filter loaded: {path_list:?} -> {service}, exclusion {exclusion_list:?}");
        result.filter = Some(SimpleHttpFilter {
            path_list,
            exclusion_list,
            service,
        });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(yaml: &str) -> Result<RoutingTable, AppError> {
        let dir = std::env::temp_dir().join(format!("pc-rest-{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("rest.yaml");
        std::fs::write(&file, yaml).unwrap();
        let reader = ConfigReader::load(&format!("file:{}", file.display()))
            .map_err(|e| AppError::new(400, e.to_string()))?;
        let result = RoutingTable::load(&reader);
        std::fs::remove_dir_all(&dir).ok();
        result
    }

    const VALID: &str = r#"
rest:
  - service: "greeting.api"
    methods: ['GET']
    url: "/api/greeting/{user}"
    timeout: 10s
    cors: cors_1
    headers: header_1
    tracing: true
  - service: "catch.all"
    methods: ['GET', 'POST']
    url: "/api/files/*"
  - service: "exact.match"
    methods: ['GET']
    url: "/api/greeting/system"
cors:
  - id: cors_1
    options:
      - "Access-Control-Allow-Origin: *"
      - "Access-Control-Allow-Methods: GET, POST, OPTIONS"
    headers:
      - "Access-Control-Allow-Origin: *"
headers:
  - id: header_1
    request:
      drop: ['x-secret']
    response:
      add: ["x-served-by: mercury"]
"#;

    #[test]
    fn parses_valid_rest_yaml() {
        let table = table(VALID).unwrap();
        assert_eq!(table.routes().len(), 3);
        let route = &table.routes()[0];
        assert_eq!(route.service, "greeting.api");
        assert_eq!(route.timeout, Duration::from_secs(10));
        assert!(route.tracing);
        assert!(route.cors.is_some());
        assert!(route.headers.is_some());
    }

    #[test]
    fn match_precedence_exact_then_param_then_wildcard() {
        let table = table(VALID).unwrap();
        // exact literal wins over {param}
        let hit = table.find("GET", "/api/greeting/system").unwrap();
        assert_eq!(hit.info.service, "exact.match");
        // {param} extraction (value case preserved; literals case-insensitive)
        let hit = table.find("GET", "/API/Greeting/Eric").unwrap();
        assert_eq!(hit.info.service, "greeting.api");
        assert_eq!(hit.path_params["user"], "Eric");
        // wildcard consumes the remainder
        let hit = table.find("POST", "/api/files/a/b/c").unwrap();
        assert_eq!(hit.info.service, "catch.all");
        // method filtering
        assert!(table.find("DELETE", "/api/greeting/eric").is_none());
        // OPTIONS matches any entry (CORS preflight)
        assert!(table.find("OPTIONS", "/api/greeting/eric").is_some());
        // no match
        assert!(table.find("GET", "/api/unknown").is_none());
    }

    #[test]
    fn parser_invariants_are_enforced() {
        // invalid method
        assert!(table("rest:\n  - service: x.y\n    methods: ['FETCH']\n    url: /a\n").is_err());
        // missing cors reference
        assert!(table(
            "rest:\n  - service: x.y\n    methods: ['GET']\n    url: /a\n    cors: nope\n"
        )
        .is_err());
        // relay not yet ported
        assert!(table(
            "rest:\n  - service: 'https://example.com'\n    methods: ['GET']\n    url: /a\n"
        )
        .is_err());
        // non-trailing wildcard
        assert!(
            table("rest:\n  - service: x.y\n    methods: ['GET']\n    url: '/a/*/b'\n").is_err()
        );
        // cors line must be Access-Control-*
        assert!(table(
            "rest:\n  - service: x.y\n    methods: ['GET']\n    url: /a\ncors:\n  - id: c1\n    options:\n      - 'X-Other: 1'\n"
        )
        .is_err());
    }

    #[test]
    fn timeout_parse_and_clamp() {
        assert_eq!(parse_timeout(Some("10s")), Duration::from_secs(10));
        assert_eq!(parse_timeout(Some("2m")), Duration::from_secs(120));
        assert_eq!(parse_timeout(Some("1500ms")), Duration::from_millis(1500));
        assert_eq!(parse_timeout(Some("500ms")), MIN_TIMEOUT); // clamped up to 1s
        assert_eq!(parse_timeout(None), DEFAULT_TIMEOUT);
        assert_eq!(parse_timeout(Some("0s")), MIN_TIMEOUT); // clamp low
        assert_eq!(parse_timeout(Some("30m")), MAX_TIMEOUT); // clamp high
        assert_eq!(parse_timeout(Some("garbage")), DEFAULT_TIMEOUT);
    }

    #[test]
    fn header_transform_keep_drop_add() {
        let transform = HeaderTransform {
            add: vec![("x-served-by".into(), "mercury".into())],
            drop: vec!["X-Secret".into()],
            keep: vec![],
        };
        let mut headers: HashMap<String, String> = HashMap::from([
            ("x-secret".into(), "shh".into()),
            ("accept".into(), "*/*".into()),
        ]);
        transform.apply(&mut headers);
        assert!(!headers.contains_key("x-secret"));
        assert_eq!(headers["x-served-by"], "mercury");
        assert_eq!(headers["accept"], "*/*");
        let keep_only = HeaderTransform {
            keep: vec!["Accept".into()],
            ..HeaderTransform::default()
        };
        let mut headers: HashMap<String, String> = HashMap::from([
            ("accept".into(), "*/*".into()),
            ("x-noise".into(), "1".into()),
        ]);
        keep_only.apply(&mut headers);
        assert_eq!(headers.len(), 1);
        assert!(headers.contains_key("accept"));
    }
}
