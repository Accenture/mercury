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

//! The `graph.api.fetcher` skill — Rust port of `GraphApiFetcher` (512
//! lines): a fetcher node names data-dictionary nodes; each dictionary maps
//! required inputs (with `key:default` fallbacks) to a provider node whose
//! `url`/`method`/`input` describe the HTTP call, made through the
//! platform-core `async.http.request` client. Dictionary output mapping
//! copies `response.*` into the fetcher's `result.*`/`model.*`; identical
//! provider calls within one graph instance are served from the instance
//! cache. A `for_each` property fans the calls out with clamped concurrency
//! (fork-join).

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use event_script::conversions::display;
use event_script::mapping::{get_constant_value, get_lhs_element, get_lhs_or_constant};
use event_script::mlm::MultiLevelMap;
use event_script::util::str2int;
use platform_core::automation::AsyncHttpRequest;
use platform_core::graph::SimpleNode;
use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::common::{
    fill_fetcher_api_parameters, get_entries, get_for_each_mapping, get_model_array_size,
    get_model_ttl, get_next_model_param_set, invalid, map_http_input,
    perform_fetcher_output_mapping, substitute_var_if_any, ERROR, EXCEPTION, HEADER, MAP_TO, NEXT,
    NODE_NAME, RESULT, SKILL, STATUS, TARGET,
};
use crate::features::{self, HttpResponseView};
use crate::model::GraphInstance;

pub const ROUTE: &str = "graph.api.fetcher";
const WARNING_INTERVAL_MS: i64 = 30000;

/// Java `HostUri`: split a provider URL into target host and URI, tolerating
/// `{path}` brackets and spaces.
pub struct HostUri {
    pub host: String,
    pub uri: String,
}

impl HostUri {
    pub fn parse(url: &str) -> Result<Self, AppError> {
        let (scheme, rest) = if let Some(rest) = url.strip_prefix("http://") {
            ("http://", rest)
        } else if let Some(rest) = url.strip_prefix("https://") {
            ("https://", rest)
        } else {
            return Err(invalid(format!("Invalid provider URL {url}")));
        };
        match rest.find('/') {
            Some(slash) => Ok(HostUri {
                host: format!("{scheme}{}", &rest[..slash]),
                uri: rest[slash..].to_string(),
            }),
            None => Ok(HostUri {
                host: url.to_string(),
                uri: "/".to_string(),
            }),
        }
    }
}

struct ProviderMetadata {
    fetcher_name: String,
    dd: Arc<SimpleNode>,
    provider_name: String,
    before: Vec<Arc<dyn features::FeatureRunner>>,
    after: Vec<Arc<dyn features::FeatureRunner>>,
    break_on_exception: bool,
    target: HostUri,
    method: String,
    inputs: Vec<String>,
}

impl ProviderMetadata {
    fn new(
        fetcher: &Arc<SimpleNode>,
        dd: &Arc<SimpleNode>,
        provider: &Arc<SimpleNode>,
    ) -> Result<Self, AppError> {
        let url = provider.get_property("url");
        let method = provider.get_property("method");
        let (Some(url), Some(method)) = (url, method) else {
            return Err(invalid(format!(
                "Missing url or method in data provider {}",
                provider.get_alias()
            )));
        };
        let mut md = ProviderMetadata {
            fetcher_name: fetcher.get_alias().to_string(),
            dd: dd.clone(),
            provider_name: provider.get_alias().to_string(),
            before: Vec::new(),
            after: Vec::new(),
            break_on_exception: fetcher.get_property(EXCEPTION).is_none(),
            target: HostUri::parse(&display(&url))?,
            method: display(&method),
            inputs: get_entries(provider.get_property("input")),
        };
        for name in get_entries(provider.get_property("feature")) {
            match features::get_feature(&name) {
                Some(feature) => {
                    if feature.run_before() {
                        md.before.push(feature);
                    } else {
                        md.after.push(feature);
                    }
                }
                None => lack_of_skill_advice(&name),
            }
        }
        Ok(md)
    }
}

/// Throttled advisory for unimplemented features (Java `lackOfSkillAdvice`).
fn lack_of_skill_advice(feature: &str) {
    static WARNED: OnceLock<Mutex<HashMap<String, i64>>> = OnceLock::new();
    let warned = WARNED.get_or_init(|| Mutex::new(HashMap::new()));
    let now = chrono::Utc::now().timestamp_millis();
    let mut map = warned.lock().expect("feature advisories");
    let last = map.get(feature).copied().unwrap_or(0);
    if now - last > WARNING_INTERVAL_MS {
        map.insert(feature.to_string(), now);
        log::warn!(
            "'{feature}' not implemented. Please implement the FeatureRunner trait and \
             register it with features::register"
        );
    }
}

/// The skill entry point (Java `handleEvent`).
pub async fn handle(
    platform: &Platform,
    headers: HashMap<String, String>,
    _event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    if headers.get("type").map(String::as_str) != Some("execute") {
        return Err(invalid("Type must be EXECUTE"));
    }
    let po = PostOffice::new(platform);
    let node_name = headers.get("node").map(String::as_str).unwrap_or("none");
    po.annotate_trace("node", node_name);
    let in_id = headers.get("in").map(String::as_str).unwrap_or("none");
    let instance = crate::common::get_graph_instance(in_id)?;
    let fetcher = crate::common::get_node(node_name, &instance.graph)?;
    let skill = fetcher.get_property(SKILL).map(|v| display(&v));
    if skill.as_deref() != Some(ROUTE) {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have skill - {ROUTE}"
        )));
    }
    let dictionary = get_entries(fetcher.get_property("dictionary"));
    let dictionary_nodes = get_dictionary_nodes(&dictionary, node_name, &instance)?;
    let for_each = get_entries(fetcher.get_property("for_each"));
    // reset results so execution is idempotent, resolve ttl and for-each
    let (timeout, for_each_size) = {
        let mut state = instance.state.lock().expect("graph state machine");
        state.remove_element(&format!("{node_name}.{RESULT}"));
        state.remove_element(&format!("{node_name}.{HEADER}"));
        let timeout = get_model_ttl(&mut state);
        if for_each.is_empty() {
            (timeout, None)
        } else {
            let mappings = get_for_each_mapping(node_name, &for_each, &mut state)?;
            if mappings.is_empty() {
                return Err(invalid(format!(
                    "{NODE_NAME}{node_name} - No data mapping resolved from 'for_each' entries. \
                     LHS must be a list."
                )));
            }
            // stage one parameter set per iteration into {node}.each.*[]
            let mapping = get_entries(fetcher.get_property("input"));
            let size = get_model_array_size(&mappings);
            for i in 0..size {
                for (key, value) in get_next_model_param_set(&mappings, i) {
                    state.set_element(&key, value).map_err(invalid)?;
                }
                for entry in &mapping {
                    fill_fetcher_api_parameters(node_name, entry, &mut state, true)?;
                }
            }
            (timeout, Some(size))
        }
    };
    let next = match for_each_size {
        None => execute_providers(&po, &instance, &fetcher, &dictionary_nodes, timeout).await?,
        Some(size) => {
            execute_providers_with_fork_join(
                &po,
                &instance,
                &fetcher,
                &dictionary_nodes,
                timeout,
                size,
            )
            .await?
        }
    };
    EventEnvelope::new().set_body(next)
}

fn get_dictionary_nodes(
    dictionary: &[String],
    node_name: &str,
    instance: &GraphInstance,
) -> Result<Vec<Arc<SimpleNode>>, AppError> {
    if dictionary.is_empty() {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} - missing dictionary"
        )));
    }
    let mut nodes = Vec::with_capacity(dictionary.len());
    for dict in dictionary {
        match instance.graph.find_node_by_alias(dict)? {
            Some(node) => nodes.push(node),
            None => {
                return Err(invalid(format!(
                    "{NODE_NAME}{node_name} - data dictionary node '{dict}' does not exist"
                )));
            }
        }
    }
    Ok(nodes)
}

/// Sequential fetch, one call per dictionary node (Java `executeProviders`).
async fn execute_providers(
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    fetcher: &Arc<SimpleNode>,
    dictionary_nodes: &[Arc<SimpleNode>],
    timeout: i64,
) -> Result<String, AppError> {
    let node_name = fetcher.get_alias().to_string();
    let parameter_mapping = get_entries(fetcher.get_property("input"));
    for dd in dictionary_nodes {
        let provider_name = match dd.get_property("provider") {
            Some(v) => display(&v),
            None => {
                return Err(invalid(format!(
                    "Missing provider in data dictionary {}",
                    dd.get_alias()
                )));
            }
        };
        let Some(provider) = instance.graph.find_node_by_alias(&provider_name)? else {
            return Err(invalid(format!(
                "Data provider {provider_name} does not exist"
            )));
        };
        let md = ProviderMetadata::new(fetcher, dd, &provider)?;
        // build the request and check the per-instance provider cache
        let (request_value, params, cached) = {
            let mut state = instance.state.lock().expect("graph state machine");
            // fetcher input mapping stages parameters at {node}.fetch.*
            for entry in &parameter_mapping {
                fill_fetcher_api_parameters(&node_name, entry, &mut state, false)?;
            }
            let params = state
                .get_element(&format!("{node_name}.fetch"))
                .unwrap_or(Value::Map(vec![]));
            state.remove_element(&format!("{node_name}.fetch"));
            let params_map = value_to_pairs(&params);
            let required = get_entries(dd.get_property("input"));
            let mut params_map = params_map;
            if !required.is_empty() {
                fill_dictionary_api_parameters(
                    &node_name,
                    &mut state,
                    dd,
                    &required,
                    &mut params_map,
                )?;
            }
            state
                .set_element(
                    &format!("{node_name}.{TARGET}"),
                    Value::from(dd.get_alias()),
                )
                .map_err(invalid)?;
            let request = build_http_request(&md, &mut state)?;
            // the cache key is the DICTIONARY-scoped parameter map
            // ({node}.dd.{alias}.* — declared inputs only), never the whole
            // staged fetch map: equivalent dictionary requests must share a
            // cache entry regardless of extra fetcher-level staging (Java
            // makeRegularHttpCall; parity F6 fix, 2026-07-21)
            let params_value = state
                .get_element(&format!("{node_name}.dd.{}", dd.get_alias()))
                .unwrap_or(Value::Map(vec![]));
            let cached = get_cached_result(&md.provider_name, &state, &params_value);
            (request.to_value(), params_value, cached)
        };
        if let Some(cached) = cached {
            let mut state = instance.state.lock().expect("graph state machine");
            state
                .set_element(&format!("{node_name}.response"), cached)
                .map_err(invalid)?;
        } else {
            let request = AsyncHttpRequest::from_value(&request_value);
            let target = format!(
                "{}{}",
                request.target_host().unwrap_or_default(),
                request.finalized_url()
            );
            log::info!(
                "{} {}, with {:?}, ttl={}",
                request.method(),
                target,
                params_keys(&params),
                timeout
            );
            po.annotate_trace("parameters", format!("{:?}", params_keys(&params)));
            po.annotate_trace("url", format!("{} {}", request.method(), target));
            let response = call_http(po, request_value.clone(), timeout).await;
            let mut state = instance.state.lock().expect("graph state machine");
            run_after_features(&md, &response, &mut state, &node_name);
            state
                .set_element(
                    &format!("{node_name}.{STATUS}"),
                    Value::from(response.status),
                )
                .map_err(invalid)?;
            if response.status >= 400 {
                store_error(&md, instance, &mut state, &node_name, &response)?;
            } else {
                state
                    .set_element(&format!("{node_name}.response"), response.body.clone())
                    .map_err(invalid)?;
                // cache request+response for this graph instance
                state
                    .set_element(
                        &format!("cache.{}[]", md.provider_name),
                        Value::Map(vec![
                            (Value::from("input"), params.clone()),
                            (Value::from("output"), response.body.clone()),
                        ]),
                    )
                    .map_err(invalid)?;
            }
        }
        {
            let mut state = instance.state.lock().expect("graph state machine");
            let mapping = get_entries(dd.get_property("output"));
            perform_dictionary_output_mapping(
                &node_name,
                &mut state,
                dd.get_alias(),
                &mapping,
                false,
            )?;
        }
    }
    let mut state = instance.state.lock().expect("graph state machine");
    let output_mapping = get_entries(fetcher.get_property("output"));
    perform_fetcher_output_mapping(&node_name, &mut state, &output_mapping)?;
    // clear the temporary dataset
    state.remove_element(&format!("{node_name}.dd"));
    state.remove_element(&format!("{node_name}.response"));
    Ok(next_path(&node_name, fetcher, &state))
}

/// Fork-join fetch over the staged for-each parameter arrays
/// (Java `executeProvidersWithForkJoin`).
async fn execute_providers_with_fork_join(
    po: &PostOffice,
    instance: &Arc<GraphInstance>,
    fetcher: &Arc<SimpleNode>,
    dictionary_nodes: &[Arc<SimpleNode>],
    timeout: i64,
    size: usize,
) -> Result<String, AppError> {
    let node_name = fetcher.get_alias().to_string();
    let given = str2int(&display(
        &fetcher.get_property("concurrency").unwrap_or(Value::Nil),
    ));
    let concurrency = if given < 0 { 3 } else { given }.clamp(1, 30) as usize;
    for dd in dictionary_nodes {
        let provider_name = match dd.get_property("provider") {
            Some(v) => display(&v),
            None => {
                return Err(invalid(format!(
                    "Data provider is not configured in dictionary {}",
                    dd.get_alias()
                )));
            }
        };
        let Some(provider) = instance.graph.find_node_by_alias(&provider_name)? else {
            return Err(invalid(format!(
                "Data provider {provider_name} does not exist"
            )));
        };
        let md = ProviderMetadata::new(fetcher, dd, &provider)?;
        // build one request per parameter set
        let (requests, parameter_names) = {
            let mut state = instance.state.lock().expect("graph state machine");
            let api_params = state
                .get_element(&format!("{node_name}.each"))
                .unwrap_or(Value::Map(vec![]));
            let param_arrays = value_to_pairs(&api_params);
            let mut requests: Vec<Value> = Vec::with_capacity(size);
            let mut names: Vec<String> = param_arrays.iter().map(|(k, _)| k.clone()).collect();
            names.sort();
            for i in 0..size {
                let mut params_map: Vec<(String, Value)> = Vec::new();
                for (key, values) in &param_arrays {
                    if let Value::Array(items) = values {
                        if let Some(value) = items.get(i) {
                            params_map.push((key.clone(), value.clone()));
                            state
                                .set_element(&format!("{node_name}.fetch.{key}"), value.clone())
                                .map_err(invalid)?;
                        }
                    }
                }
                let required = get_entries(dd.get_property("input"));
                if !required.is_empty() {
                    fill_dictionary_api_parameters(
                        &node_name,
                        &mut state,
                        dd,
                        &required,
                        &mut params_map,
                    )?;
                }
                requests.push(build_http_request(&md, &mut state)?.to_value());
            }
            state
                .set_element(
                    &format!("{node_name}.{TARGET}"),
                    Value::from(dd.get_alias()),
                )
                .map_err(invalid)?;
            (requests, names)
        };
        if let Some(first) = requests.first() {
            let request = AsyncHttpRequest::from_value(first);
            log::info!(
                "{} {}, for each {:?}, parallel={}, ttl={}",
                request.method(),
                request.finalized_url(),
                parameter_names,
                concurrency.min(requests.len()),
                timeout
            );
            po.annotate_trace("for_each", format!("{parameter_names:?}"));
            po.annotate_trace(
                "url",
                format!(
                    "{} {}{}",
                    request.method(),
                    request.target_host().unwrap_or_default(),
                    request.finalized_url()
                ),
            );
        }
        for batch in requests.chunks(concurrency) {
            let responses = run_http_batch(batch.to_vec(), timeout).await;
            let mut state = instance.state.lock().expect("graph state machine");
            for response in &responses {
                run_after_features(&md, response, &mut state, &node_name);
                state
                    .set_element(
                        &format!("{node_name}.{STATUS}"),
                        Value::from(response.status),
                    )
                    .map_err(invalid)?;
                if response.status >= 400 {
                    store_error(&md, instance, &mut state, &node_name, response)?;
                } else {
                    state
                        .set_element(&format!("{node_name}.response"), response.body.clone())
                        .map_err(invalid)?;
                }
                let mapping = get_entries(dd.get_property("output"));
                perform_dictionary_output_mapping(
                    &node_name,
                    &mut state,
                    dd.get_alias(),
                    &mapping,
                    true,
                )?;
            }
        }
    }
    let mut state = instance.state.lock().expect("graph state machine");
    let output_mapping = get_entries(fetcher.get_property("output"));
    perform_fetcher_output_mapping(&node_name, &mut state, &output_mapping)?;
    // clear the temporary dataset
    state.remove_element(&format!("{node_name}.fetch"));
    state.remove_element(&format!("{node_name}.each"));
    state.remove_element(&format!("{node_name}.dd"));
    state.remove_element(&format!("{node_name}.response"));
    Ok(next_path(&node_name, fetcher, &state))
}

fn next_path(node_name: &str, fetcher: &Arc<SimpleNode>, state: &MultiLevelMap) -> String {
    let process_status = state.get_element(&format!("{node_name}.{STATUS}"));
    let result_error = state.get_element(&format!("{node_name}.{ERROR}"));
    let error_handler = fetcher.get_property(EXCEPTION);
    if let (Some(Value::Integer(_)), Some(_), Some(handler)) =
        (&process_status, &result_error, &error_handler)
    {
        display(handler)
    } else {
        NEXT.to_string()
    }
}

/// Fill dictionary-required inputs from the staged parameters, honoring
/// `key:default` fallbacks (Java `fillDictionaryApiParameters` +
/// `fillDefaultApiParameter`). Values land at `{node}.dd.{dict}.{key}`.
fn fill_dictionary_api_parameters(
    node_name: &str,
    state: &mut MultiLevelMap,
    dd: &Arc<SimpleNode>,
    required: &[String],
    parameters: &mut Vec<(String, Value)>,
) -> Result<(), AppError> {
    let dd_name = dd.get_alias().to_string();
    for input in required {
        let key = input.trim();
        match key.split_once(':') {
            None => {
                if let Some((_, value)) = parameters.iter().find(|(k, _)| k == key) {
                    state
                        .set_element(&format!("{node_name}.dd.{dd_name}.{key}"), value.clone())
                        .map_err(invalid)?;
                }
            }
            Some((dk, dv)) => {
                let dk = dk.trim();
                let dv = dv.trim();
                if let Some((_, value)) = parameters.iter().find(|(k, _)| k == dk) {
                    let value = value.clone();
                    state
                        .set_element(&format!("{node_name}.dd.{dd_name}.{dk}"), value)
                        .map_err(invalid)?;
                } else {
                    // the default may itself map a value from the state machine
                    let mapped = get_lhs_or_constant(dv, state).map_err(invalid)?;
                    let resolved = mapped.unwrap_or_else(|| Value::from(dv));
                    state
                        .set_element(&format!("{node_name}.dd.{dd_name}.{dk}"), resolved.clone())
                        .map_err(invalid)?;
                    parameters.push((dk.to_string(), resolved));
                }
            }
        }
    }
    Ok(())
}

/// Dictionary output mapping (Java `performDictionaryOutputMapping` +
/// `setDictionaryOutputEntry`): `response.*` LHS reads the provider response
/// under the fetcher's namespace; `result.*` RHS lands under the fetcher
/// (`[]`-appended per fork-join response).
fn perform_dictionary_output_mapping(
    node_name: &str,
    state: &mut MultiLevelMap,
    dictionary_name: &str,
    mapping: &[String],
    is_array: bool,
) -> Result<(), AppError> {
    for output in mapping {
        let text = output.trim();
        let Some(sep) = text.rfind(MAP_TO) else {
            return Err(invalid(format!(
                "{NODE_NAME}{node_name} - invalid output mapping: {text}"
            )));
        };
        let mut lhs = substitute_var_if_any(text[..sep].trim(), state)?;
        let rhs = text[sep + MAP_TO.len()..].trim();
        let constant = get_constant_value(&lhs);
        if constant.is_none() && !lhs.starts_with("f:") {
            if lhs.starts_with("response.") || lhs == "response" {
                lhs = format!("{node_name}.{lhs}");
            } else if let Some(rest) = lhs.strip_prefix("$.response") {
                lhs = format!("$.{node_name}.response{rest}");
            } else if !lhs.starts_with("model.") && !lhs.starts_with("$.model.") {
                return Err(invalid(format!(
                    "Invalid output data mapping in data dictionary {dictionary_name} - \
                     LHS must start with 'model.' or 'response.' namespace"
                )));
            }
        }
        let value = match constant {
            Some(c) => Some(c),
            None => get_lhs_element(&lhs, state).map_err(invalid)?,
        };
        if let Some(value) = value {
            let target = if rhs.starts_with("result.") {
                format!("{node_name}.{rhs}{}", if is_array { "[]" } else { "" })
            } else if rhs.starts_with("model.") {
                rhs.to_string()
            } else {
                return Err(invalid(format!(
                    "Invalid output data mapping in data dictionary {node_name} - \
                     RHS must start with 'model.' or 'result.' namespace"
                )));
            };
            state.set_element(&target, value).map_err(invalid)?;
        }
    }
    Ok(())
}

/// Build the provider HTTP request and run before-features
/// (Java `buildHttpRequest`).
fn build_http_request(
    md: &ProviderMetadata,
    state: &mut MultiLevelMap,
) -> Result<AsyncHttpRequest, AppError> {
    let mut request = AsyncHttpRequest::new()
        .set_method(&md.method)
        .set_target_host(&md.target.host)
        .set_url(&md.target.uri);
    if !md.inputs.is_empty() {
        request = map_http_input(
            request,
            &md.fetcher_name,
            md.dd.get_alias(),
            state,
            &md.inputs,
        )?;
    }
    for feature in &md.before {
        feature.execute(Some(&mut request), None, state, &md.fetcher_name);
    }
    Ok(request)
}

fn run_after_features(
    md: &ProviderMetadata,
    response: &HttpResponseView,
    state: &mut MultiLevelMap,
    node_name: &str,
) {
    for feature in &md.after {
        feature.execute(None, Some(response), state, node_name);
    }
}

fn store_error(
    md: &ProviderMetadata,
    _instance: &Arc<GraphInstance>,
    state: &mut MultiLevelMap,
    node_name: &str,
    response: &HttpResponseView,
) -> Result<(), AppError> {
    state
        .set_element(&format!("{node_name}.{ERROR}"), response.body.clone())
        .map_err(invalid)?;
    if md.break_on_exception {
        state
            .set_element("output.body", response.body.clone())
            .map_err(invalid)?;
        state
            .set_element("output.header", headers_value(&response.headers))
            .map_err(invalid)?;
        state
            .set_element("output.status", Value::from(response.status))
            .map_err(invalid)?;
    }
    Ok(())
}

fn headers_value(headers: &[(String, String)]) -> Value {
    Value::Map(
        headers
            .iter()
            .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
            .collect(),
    )
}

/// Per-instance provider cache lookup: identical parameters reuse the
/// previous response (Java `getCachedResult`).
fn get_cached_result(provider: &str, state: &MultiLevelMap, params: &Value) -> Option<Value> {
    if let Some(Value::Array(entries)) = state.get_element(&format!("cache.{provider}")) {
        for entry in entries {
            if let Value::Map(map) = &entry {
                let input = map
                    .iter()
                    .find(|(k, _)| k.as_str() == Some("input"))
                    .map(|(_, v)| v);
                // parameter maps are built by the same code path, so map
                // ordering is stable and direct equality matches Java's
                // order-insensitive Map.equals in practice
                if input == Some(params) {
                    return map
                        .iter()
                        .find(|(k, _)| k.as_str() == Some("output"))
                        .map(|(_, v)| v.clone());
                }
            }
        }
    }
    None
}

fn value_to_pairs(value: &Value) -> Vec<(String, Value)> {
    match value {
        Value::Map(entries) => entries
            .iter()
            .filter_map(|(k, v)| k.as_str().map(|key| (key.to_string(), v.clone())))
            .collect(),
        _ => Vec::new(),
    }
}

fn params_keys(params: &Value) -> Vec<String> {
    match params {
        Value::Map(entries) => entries
            .iter()
            .filter_map(|(k, _)| k.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    }
}

/// One HTTP call through `async.http.request`; errors normalize into the
/// response view (Java `po.request(event, timeout, false)`).
async fn call_http(po: &PostOffice, request: Value, ttl: i64) -> HttpResponseView {
    let event = EventEnvelope::new()
        .set_to(platform_core::automation::ASYNC_HTTP_REQUEST)
        .set_raw_body(request);
    match po
        .request(event, Duration::from_millis(ttl.max(0) as u64))
        .await
    {
        Ok(response) => HttpResponseView {
            status: response.status(),
            headers: response
                .headers()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            body: response.body().clone(),
        },
        Err(e) => HttpResponseView {
            status: e.status(),
            headers: Vec::new(),
            body: Value::from(e.message()),
        },
    }
}

/// One fork-join batch of HTTP calls, responses in request order.
async fn run_http_batch(requests: Vec<Value>, ttl: i64) -> Vec<HttpResponseView> {
    let mut handles = Vec::with_capacity(requests.len());
    for request in requests {
        handles.push(tokio::spawn(async move {
            let platform = Platform::get_instance();
            let po = PostOffice::new(&platform);
            call_http(&po, request, ttl).await
        }));
    }
    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.await.unwrap_or_else(|_| HttpResponseView {
            status: 500,
            headers: Vec::new(),
            body: Value::from("HTTP join failure"),
        }));
    }
    results
}
