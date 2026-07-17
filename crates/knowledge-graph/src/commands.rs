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

//! The Playground command grammar — Rust port of `GraphCommandService`
//! (1,494 lines): the conversational interface shared by the React
//! workbench, a human on a raw websocket, and an AI companion through
//! `POST /api/companion/{session-id}`. Commands build a draft `MiniGraph`
//! per session (create/update/delete node, connect, list, describe, export/
//! import), instantiate a graph instance with mock data, execute single
//! nodes, `run` the traveler, `inspect` the state machine, and manage
//! collaborative sessions (subscribe/unsubscribe/reset).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use event_script::conversions::display;
use event_script::converter;
use event_script::mapping::get_constant_value;
use event_script::mlm::MultiLevelMap;
use event_script::util::{split, str2int};
use platform_core::graph::{MiniGraph, SimpleConnection, SimpleNode};
use platform_core::{AppConfigReader, AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::common::{get_model_ttl, initialize_with_node_properties, invalid};
use crate::model::{self, GraphInstance};
use crate::session::{self, GraphSession};

pub const ROUTE: &str = "graph.command.service";
pub const SINGLETON_COMMAND_HANDLER: &str = "graph.command.singleton";
const DEFAULT_TEMP_DIR: &str = "/tmp/graph";
const DEFAULT_DEPLOY_DIR: &str = "classpath:/graph";
const INVALID_GRAPH_NAME: &str = "Invalid filename - must be a-z, A-Z, 0-9 with optional hyphen";
const TRY_HELP: &str = "Please try 'help' for details";
const NODE_NAME: &str = "node ";
const NOT_FOUND: &str = " not found";
const SESSION_TAG: &str = "Session ";
const UNTYPED: &str = "untyped";
const EXPIRY_MS: i64 = 20 * 1000;
const MAX_BUFFER_SIZE: usize = 62 * 1024;
const MAPPING_PROPERTIES: &[&str] = &["mapping", "input", "output", "for_each"];

/// The Playground temp folder (Java constructor validations).
pub fn temp_dir() -> &'static PathBuf {
    static TEMP: OnceLock<PathBuf> = OnceLock::new();
    TEMP.get_or_init(|| {
        let config = AppConfigReader::get_instance();
        let mut location = config.get_property_or("location.graph.temp", DEFAULT_TEMP_DIR);
        if location.starts_with("classpath:") {
            log::error!(
                "location.graph.temp must use local file system because of read/write requirements"
            );
            location = DEFAULT_TEMP_DIR.to_string();
        }
        if let Some(stem) = location.strip_prefix("file:") {
            location = stem.to_string();
        }
        if location.contains(':') || split(&location, "/").len() < 2 {
            log::error!("location.graph.temp is invalid. Fallback to {DEFAULT_TEMP_DIR}");
            location = DEFAULT_TEMP_DIR.to_string();
        }
        let dir = PathBuf::from(&location);
        if !dir.exists() && std::fs::create_dir_all(&dir).is_ok() {
            log::info!("Created temp folder {location}");
        }
        log::info!("Playground temp folder (location.graph.temp) - {location}");
        dir
    })
}

fn deployed_location() -> String {
    let config = AppConfigReader::get_instance();
    let location = config.get_property_or("location.graph.deployed", DEFAULT_DEPLOY_DIR);
    if location.starts_with("file:") || location.starts_with("classpath:") {
        location
    } else {
        DEFAULT_DEPLOY_DIR.to_string()
    }
}

/// Start the temp-graph housekeeping sweep (Java: 10s periodic).
pub fn start_housekeeping() {
    static STARTED: OnceLock<()> = OnceLock::new();
    STARTED.get_or_init(|| {
        housekeeping();
        tokio::spawn(async {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                housekeeping();
            }
        });
    });
}

fn housekeeping() {
    let now = session::now_ms();
    let Ok(entries) = std::fs::read_dir(temp_dir()) else {
        return;
    };
    let mut n = 0;
    for entry in entries.flatten() {
        if is_expired(&entry, now) && std::fs::remove_file(entry.path()).is_ok() {
            n += 1;
        }
    }
    if n > 0 {
        log::info!(
            "Removed {n} expired temp graph{}",
            if n == 1 { "" } else { "s" }
        );
    }
}

fn is_expired(entry: &std::fs::DirEntry, now: i64) -> bool {
    let file_name = entry.file_name().to_string_lossy().to_string();
    let Some(name) = file_name.strip_suffix(".json") else {
        return false;
    };
    let parts = split(name, "-");
    if !(parts.len() == 3
        && parts[0] == "ws"
        && parts[1].chars().all(|c| c.is_ascii_digit())
        && parts[2].chars().all(|c| c.is_ascii_digit()))
    {
        return false;
    }
    let modified = entry
        .metadata()
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as i64)
        .unwrap_or(now);
    now - modified > EXPIRY_MS
}

/// Duplicate-command suppression (Java `ManagedCache("last.ws.message", 1000)`).
fn last_message_cache() -> &'static Mutex<HashMap<String, (String, i64)>> {
    static CACHE: OnceLock<Mutex<HashMap<String, (String, i64)>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn is_duplicate(in_route: &str, command: &str) -> bool {
    let now = session::now_ms();
    let mut cache = last_message_cache().lock().expect("message cache");
    let duplicate = matches!(cache.get(in_route),
        Some((last, at)) if last == command && now - at < 1000);
    cache.insert(in_route.to_string(), (command.to_string(), now));
    duplicate
}

fn counter() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::SeqCst) + 1
}

fn random_counter() -> String {
    let now = session::now_ms().to_string();
    let tail = &now[now.len().saturating_sub(3)..];
    format!("{tail}-{}", counter())
}

async fn say(po: &PostOffice, out_route: &str, message: impl Into<String>) {
    let _ = po
        .send(
            EventEnvelope::new()
                .set_to(out_route)
                .set_raw_body(Value::from(message.into().as_str())),
        )
        .await;
}

async fn say_value(po: &PostOffice, out_route: &str, value: Value) {
    let _ = po
        .send(EventEnvelope::new().set_to(out_route).set_raw_body(value))
        .await;
}

/// The service entry point (Java `handleEvent` + `handleRequest`).
pub async fn handle(
    platform: &Platform,
    _headers: HashMap<String, String>,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let po = PostOffice::new(platform);
    let Value::Map(entries) = event.body() else {
        return EventEnvelope::new().set_body("ignored");
    };
    let get = |key: &str| -> Option<String> {
        entries
            .iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .map(|(_, v)| display(v))
    };
    let kind = get("type").unwrap_or_default();
    let in_route = get("in").unwrap_or_default();
    let out_route = get("out").unwrap_or_default();
    let message = get("message").unwrap_or_default();
    let forwarded = get("forwarded").map(|v| v == "true").unwrap_or(false);
    let outcome = handle_request(
        platform, &po, &kind, &in_route, &out_route, &message, forwarded,
    )
    .await;
    if let Err(e) = outcome {
        if !out_route.is_empty() {
            say(&po, &out_route, format!("ERROR: {}", e.message())).await;
        }
    }
    EventEnvelope::new().set_body("done")
}

async fn handle_request(
    platform: &Platform,
    po: &PostOffice,
    kind: &str,
    in_route: &str,
    out_route: &str,
    message: &str,
    forwarded: bool,
) -> Result<(), AppError> {
    start_housekeeping();
    match kind {
        "open" if !in_route.is_empty() => {
            session::put_session(in_route, Arc::new(GraphSession::new(in_route)));
            session::put_graph_model(in_route, Arc::new(MiniGraph::new()));
            Ok(())
        }
        "close" if !in_route.is_empty() => {
            if let Some(me) = session::get_session(in_route) {
                let out = GraphSession::out_route_of(me.session_id());
                reset_session(po, &me, &out).await;
            }
            session::remove_session(in_route);
            session::remove_graph_model(in_route);
            model::remove_instance(in_route);
            let file = temp_dir().join(format!("{}.json", session::temp_graph_name(in_route)));
            if file.exists() {
                let _ = std::fs::remove_file(file);
            }
            Ok(())
        }
        "command" if !in_route.is_empty() && !out_route.is_empty() => {
            let command = message.trim();
            if command.is_empty() {
                return Ok(());
            }
            handle_command(platform, po, command, in_route, out_route, forwarded).await
        }
        _ => Ok(()),
    }
}

async fn handle_command(
    platform: &Platform,
    po: &PostOffice,
    command: &str,
    in_route: &str,
    out_route: &str,
    forwarded: bool,
) -> Result<(), AppError> {
    if command.starts_with('{') && command.ends_with('}') {
        return handle_json_command(po, out_route, command).await;
    }
    if command.to_lowercase().starts_with("session") || forwarded {
        return single_or_multi_line(platform, po, command, in_route, out_route).await;
    }
    if is_duplicate(in_route, command) {
        log::debug!("Duplicated message - {command} for {in_route}");
        return Ok(());
    }
    let Some(me) = session::get_session(in_route) else {
        return Ok(());
    };
    if me.is_primary() {
        single_or_multi_line(platform, po, command, in_route, out_route).await?;
        for sub_out in me.subscribers() {
            let sub_in = GraphSession::in_route_of(&sub_out);
            let forward = command_body(&sub_in, &sub_out, command, true);
            let _ = po
                .send(EventEnvelope::new().set_to(ROUTE).set_raw_body(forward))
                .await;
        }
        Ok(())
    } else {
        // forward everything except session commands to the primary session
        let target_in = GraphSession::in_route_of(&me.target_id());
        let target_out = GraphSession::out_route_of(&me.target_id());
        let forward = command_body(&target_in, &target_out, command, false);
        let _ = po
            .send(EventEnvelope::new().set_to(ROUTE).set_raw_body(forward))
            .await;
        Ok(())
    }
}

fn command_body(in_route: &str, out_route: &str, message: &str, forwarded: bool) -> Value {
    let mut map = vec![
        (Value::from("type"), Value::from("command")),
        (Value::from("in"), Value::from(in_route)),
        (Value::from("out"), Value::from(out_route)),
        (Value::from("message"), Value::from(message)),
    ];
    if forwarded {
        map.push((Value::from("forwarded"), Value::from(true)));
    }
    Value::Map(map)
}

async fn handle_json_command(
    po: &PostOffice,
    out_route: &str,
    command: &str,
) -> Result<(), AppError> {
    let parsed: serde_json::Value =
        serde_json::from_str(command).map_err(|e| invalid(e.to_string()))?;
    match parsed.get("type").and_then(|v| v.as_str()) {
        Some("ping") => {
            say_value(
                po,
                out_route,
                Value::Map(vec![(Value::from("type"), Value::from("pong"))]),
            )
            .await;
        }
        Some("welcome") => say(po, out_route, "Welcome to MiniGraph Playground!").await,
        _ => {}
    }
    Ok(())
}

async fn single_or_multi_line(
    platform: &Platform,
    po: &PostOffice,
    command: &str,
    in_route: &str,
    out_route: &str,
) -> Result<(), AppError> {
    if command.contains('\n') {
        handle_multi_line(po, in_route, out_route, command, false).await
    } else {
        handle_single_line(platform, po, in_route, out_route, command).await
    }
}

/// Word split with the Java aliases (`start` → `instantiate`, `clear` → `delete`).
fn get_words(command: &str) -> Vec<String> {
    let mut words = split(command, " ");
    if !words.is_empty() {
        if words[0].eq_ignore_ascii_case("start") {
            words[0] = "instantiate".to_string();
        }
        if words[0].eq_ignore_ascii_case("clear") {
            words[0] = "delete".to_string();
        }
        if words.len() > 1 && words[0].eq_ignore_ascii_case("help") {
            if words[1].eq_ignore_ascii_case("start") {
                words[1] = "instantiate".to_string();
            }
            if words[1].eq_ignore_ascii_case("clear") {
                words[1] = "delete".to_string();
            }
        }
    }
    words
}

async fn handle_single_line(
    platform: &Platform,
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    command: &str,
) -> Result<(), AppError> {
    let words = get_words(command);
    say(po, out_route, format!("> {command}")).await;
    if !words.is_empty() && words[0].eq_ignore_ascii_case("help") {
        let help = get_help(&words);
        say(
            po,
            out_route,
            help.unwrap_or_else(|| format!("'{command}'{NOT_FOUND}")),
        )
        .await;
        Ok(())
    } else if words.len() > 1
        && (words[0].eq_ignore_ascii_case("create") || words[0].eq_ignore_ascii_case("instantiate"))
    {
        handle_multi_line(po, in_route, out_route, command, true).await
    } else if words.len() > 1 && words[0].eq_ignore_ascii_case("describe") {
        handle_describe(po, in_route, out_route, &words).await
    } else if words.len() == 2 && words[0].eq_ignore_ascii_case("inspect") {
        handle_inspect(po, in_route, out_route, &words[1]).await
    } else {
        handle_part_two(platform, po, in_route, out_route, &words).await
    }
}

async fn handle_part_two(
    platform: &Platform,
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    words: &[String],
) -> Result<(), AppError> {
    if words.len() > 2 && words[0].eq_ignore_ascii_case("connect") {
        handle_connect(po, in_route, out_route, words).await
    } else if words.len() > 1 && words[0].eq_ignore_ascii_case("delete") {
        handle_delete(po, in_route, out_route, words).await
    } else if words.len() == 4
        && words[0].eq_ignore_ascii_case("export")
        && words[1].eq_ignore_ascii_case("graph")
        && words[2].eq_ignore_ascii_case("as")
    {
        handle_export(po, in_route, out_route, &words[3]).await
    } else if words.len() == 4
        && words[0].eq_ignore_ascii_case("import")
        && words[1].eq_ignore_ascii_case("graph")
        && words[2].eq_ignore_ascii_case("from")
    {
        handle_import_graph(po, in_route, out_route, &words[3]).await
    } else if words.len() == 5
        && words[0].eq_ignore_ascii_case("import")
        && words[1].eq_ignore_ascii_case("node")
        && words[3].eq_ignore_ascii_case("from")
    {
        handle_import_node(po, in_route, out_route, &words[2], &words[4]).await
    } else if words.len() == 3
        && words[0].eq_ignore_ascii_case("edit")
        && words[1].eq_ignore_ascii_case("node")
    {
        handle_edit(po, in_route, out_route, &words[2]).await
    } else if words.len() == 2 && words[0].eq_ignore_ascii_case("list") {
        handle_list(po, in_route, out_route, &words[1]).await
    } else {
        handle_part_three(platform, po, in_route, out_route, words).await
    }
}

async fn handle_part_three(
    platform: &Platform,
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    words: &[String],
) -> Result<(), AppError> {
    if words.len() == 3
        && words[0].eq_ignore_ascii_case("upload")
        && words[1].eq_ignore_ascii_case("mock")
        && words[2].eq_ignore_ascii_case("data")
    {
        if model::get_instance(in_route).is_some() {
            let name = session::temp_graph_name(in_route);
            say(
                po,
                out_route,
                format!("You may upload JSON payload -> POST /api/mock/{name}"),
            )
            .await;
        }
        Ok(())
    } else if words.len() == 1 && words[0].eq_ignore_ascii_case("seen") {
        handle_seen(po, in_route, out_route).await
    } else if words[0].eq_ignore_ascii_case("session") {
        handle_session(po, in_route, out_route, words).await
    } else if words.len() > 1 && words[0].eq_ignore_ascii_case("execute") {
        handle_execute(platform, in_route, out_route, words).await
    } else if words.len() == 1 && words[0].eq_ignore_ascii_case("run") {
        // launch the traveler with the console as the reply path
        let cid = uuid::Uuid::new_v4().simple().to_string();
        let _ = po
            .send(
                EventEnvelope::new()
                    .set_to(crate::traveler::ROUTE)
                    .set_header("in", in_route)
                    .set_reply_to(out_route)
                    .set_correlation_id(&cid),
            )
            .await;
        Ok(())
    } else {
        say(po, out_route, TRY_HELP).await;
        Ok(())
    }
}

async fn handle_seen(po: &PostOffice, in_route: &str, out_route: &str) -> Result<(), AppError> {
    let instance = crate::common::get_graph_instance(in_route)?;
    let mut root = false;
    let mut end = false;
    let mut nodes: Vec<String> = Vec::new();
    for name in instance.node_seen.lock().expect("node seen").keys() {
        match name.as_str() {
            "root" => root = true,
            "end" => end = true,
            other => nodes.push(other.to_string()),
        }
    }
    nodes.sort();
    let mut result: Vec<String> = Vec::new();
    if root {
        result.push("root".to_string());
    }
    result.append(&mut nodes);
    if end {
        result.push("end".to_string());
    }
    say(
        po,
        out_route,
        format!(
            "Total {} node{} have been seen",
            result.len(),
            if result.len() == 1 { "" } else { "s" }
        ),
    )
    .await;
    say_value(
        po,
        out_route,
        Value::Array(result.into_iter().map(Value::from).collect()),
    )
    .await;
    Ok(())
}

// ---- session commands ----

async fn handle_session(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    words: &[String],
) -> Result<(), AppError> {
    let Some(me) = session::get_session(in_route) else {
        return Ok(());
    };
    if words.len() == 1 {
        show_session(po, &me, out_route).await;
    } else if words.len() == 2 {
        if words[1].eq_ignore_ascii_case("reset") {
            reset_session(po, &me, out_route).await;
            session::put_session(in_route, Arc::new(GraphSession::new(in_route)));
            session::put_graph_model(in_route, Arc::new(MiniGraph::new()));
            say(po, out_route, "Session restarted").await;
        } else if words[1].eq_ignore_ascii_case("unsubscribe") {
            if me.is_primary() {
                say(po, out_route, "Nothing to unsubscribe").await;
            } else {
                let target_id = me.target_id();
                unsubscribe_session(po, &me, out_route).await;
                me.set_target_id(me.session_id());
                say(
                    po,
                    out_route,
                    format!("Session unsubscribed from {target_id}"),
                )
                .await;
            }
        }
    } else if words.len() > 2 && words[1].eq_ignore_ascii_case("subscribe") {
        subscribe_session(po, &me, in_route, out_route, &words[2]).await?;
    } else {
        say(po, out_route, "Invalid session command").await;
    }
    Ok(())
}

async fn show_session(po: &PostOffice, me: &Arc<GraphSession>, out_route: &str) {
    let started = local_timestamp(me.start_time_ms);
    let mut status = format!("{SESSION_TAG}{} started since {started}\n", me.session_id());
    if !me.is_primary() {
        status.push_str(&format!("subscribed to {}\n", me.target_id()));
    }
    let subscribers = me.subscribers();
    if !subscribers.is_empty() {
        let names: Vec<String> = subscribers
            .iter()
            .map(|route| GraphSession::session_id_of(route))
            .collect();
        status.push_str(&format!("subscribed by {names:?}\n"));
    }
    say(po, out_route, status).await;
}

fn local_timestamp(ms: i64) -> String {
    use chrono::TimeZone;
    chrono::Local
        .timestamp_millis_opt(ms)
        .single()
        .map(|t| t.format("%Y-%m-%d %H:%M:%S%.3f").to_string())
        .unwrap_or_else(|| ms.to_string())
}

async fn reset_session(po: &PostOffice, me: &Arc<GraphSession>, out_route: &str) {
    if me.is_primary() {
        for subscriber_out in me.subscribers() {
            let target_id = GraphSession::session_id_of(&subscriber_out);
            if let Some(target) = session::get_session(&GraphSession::in_route_of(&target_id)) {
                target.set_target_id(&target_id);
                say(
                    po,
                    &subscriber_out,
                    format!("{SESSION_TAG}{} has closed", me.session_id()),
                )
                .await;
            }
        }
    } else {
        unsubscribe_session(po, me, out_route).await;
    }
}

async fn unsubscribe_session(po: &PostOffice, me: &Arc<GraphSession>, out_route: &str) {
    let target_id = me.target_id();
    if let Some(target) = session::get_session(&GraphSession::in_route_of(&target_id)) {
        if target.has_subscriber(out_route) {
            target.unsubscribe(out_route);
            let target_out = GraphSession::out_route_of(&target_id);
            say(
                po,
                &target_out,
                format!("{} unsubscribed from your session", me.session_id()),
            )
            .await;
        }
    }
}

async fn subscribe_session(
    po: &PostOffice,
    me: &Arc<GraphSession>,
    in_route: &str,
    out_route: &str,
    session_id: &str,
) -> Result<(), AppError> {
    if !me.is_primary() {
        say(
            po,
            out_route,
            format!(
                "You have already subscribed to {}\nPlease do 'session reset' before subscribing \
                 to another session",
                me.target_id()
            ),
        )
        .await;
        return Ok(());
    }
    let Some(target) = session::get_session(&GraphSession::in_route_of(session_id)) else {
        say(
            po,
            out_route,
            format!("{SESSION_TAG}{session_id}{NOT_FOUND}"),
        )
        .await;
        return Ok(());
    };
    if !target.is_primary() {
        say(
            po,
            out_route,
            format!("{session_id} is not a primary session"),
        )
        .await;
        return Ok(());
    }
    if me.session_id() == target.session_id() {
        say(po, out_route, "You cannot subscribe to yourself").await;
        return Ok(());
    }
    synchronize_graph(po, me, &target, in_route, out_route).await
}

async fn synchronize_graph(
    po: &PostOffice,
    me: &Arc<GraphSession>,
    target: &Arc<GraphSession>,
    in_route: &str,
    out_route: &str,
) -> Result<(), AppError> {
    let target_id = target.session_id().to_string();
    let target_in = GraphSession::in_route_of(&target_id);
    let target_out = GraphSession::out_route_of(&target_id);
    let source_graph = session::get_graph_model(in_route);
    let target_graph = session::get_graph_model(&target_in);
    let (Some(source_graph), Some(target_graph)) = (source_graph, target_graph) else {
        return Ok(());
    };
    let direct = if target_graph.is_empty() {
        if !source_graph.is_empty() {
            let data = source_graph.export_graph();
            target_graph
                .import_graph(&data)
                .map_err(|e| invalid(e.message()))?;
            // populate other subscribers of the target
            for subscriber in target.subscribers() {
                if subscriber != out_route {
                    let sub_in = GraphSession::in_route_of(&subscriber);
                    if let Some(sub_graph) = session::get_graph_model(&sub_in) {
                        let _ = sub_graph.import_graph(&data);
                    }
                }
            }
        }
        false
    } else {
        source_graph
            .import_graph(&target_graph.export_graph())
            .map_err(|e| invalid(e.message()))?;
        true
    };
    me.set_target_id(&target_id);
    target.subscribe(out_route);
    say(po, out_route, format!("Subscribed to {target_id}")).await;
    say(
        po,
        &target_out,
        format!("{} subscribed to your session", me.session_id()),
    )
    .await;
    if !source_graph.is_empty() {
        touch_node(po, &source_graph, in_route, out_route, direct).await;
    }
    Ok(())
}

/// Update or re-create a node so the UI repopulates the graph view
/// (Java `touchNode`).
async fn touch_node(
    po: &PostOffice,
    graph: &Arc<MiniGraph>,
    in_route: &str,
    out_route: &str,
    direct: bool,
) {
    let touch = match graph.find_node_by_alias("root") {
        Ok(Some(root)) => Some(root),
        _ => graph.get_nodes().into_iter().next(),
    };
    if let Some(node) = touch {
        let command = construct_node_update_command(&node);
        say(po, in_route, command.clone()).await;
        let body = command_body(in_route, out_route, &command, direct);
        let _ = po
            .send(EventEnvelope::new().set_to(ROUTE).set_raw_body(body))
            .await;
    }
}

// ---- list / describe / edit ----

async fn handle_list(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    kind: &str,
) -> Result<(), AppError> {
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    let text = if kind.eq_ignore_ascii_case("nodes") {
        list_nodes(&graph)?
    } else if kind.eq_ignore_ascii_case("connections") {
        list_connections(&graph)
    } else {
        "Please use 'list nodes' or 'list connections'".to_string()
    };
    say(po, out_route, text).await;
    Ok(())
}

fn types_text(node: &Arc<SimpleNode>) -> String {
    let mut types: Vec<String> = node.get_types().into_iter().collect();
    types.sort();
    format!("{types:?}")
}

fn list_nodes(graph: &Arc<MiniGraph>) -> Result<String, AppError> {
    let nodes = graph.get_nodes();
    if nodes.is_empty() {
        return Ok("There are no nodes in this graph".to_string());
    }
    let root = graph.get_root_node();
    let end = graph.get_end_node();
    let mut sb = String::new();
    sb.push_str(&match &root {
        Some(node) => format!("root {}\n", types_text(node)),
        None => "root (does not exist)\n".to_string(),
    });
    let mut listing: Vec<String> = Vec::new();
    for node in &nodes {
        let is_root = root
            .as_ref()
            .map(|r| r.get_id() == node.get_id())
            .unwrap_or(false);
        let is_end = end
            .as_ref()
            .map(|e| e.get_id() == node.get_id())
            .unwrap_or(false);
        if !is_root && !is_end {
            listing.push(format!("{} {}", node.get_alias(), types_text(node)));
        }
    }
    listing.sort();
    for line in listing {
        sb.push_str(&line);
        sb.push('\n');
    }
    sb.push_str(&match &end {
        Some(node) => format!("end {}\n", types_text(node)),
        None => "end (does not exist)\n".to_string(),
    });
    Ok(sb)
}

fn list_connections(graph: &Arc<MiniGraph>) -> String {
    let connections = graph.get_connections();
    if connections.is_empty() {
        return "There are no connections in this graph".to_string();
    }
    let mut root_list: Vec<String> = Vec::new();
    let mut end_list: Vec<String> = Vec::new();
    let mut regular: Vec<String> = Vec::new();
    for connection in connections {
        let source = connection.get_source().get_alias().to_string();
        let target = connection.get_target().get_alias().to_string();
        let relations = connection.get_relations();
        let line = if relations.is_empty() {
            format!("{source} --> {target}")
        } else {
            let mut names: Vec<String> =
                relations.iter().map(|r| r.get_type().to_string()).collect();
            names.sort();
            // Java `ArrayList.toString()`: `[first]` / `[first, second]` (no quotes)
            format!("{source} -[{}]-> {target}", names.join(", "))
        };
        if source.eq_ignore_ascii_case("root") {
            root_list.push(line);
        } else if target.eq_ignore_ascii_case("end") {
            end_list.push(line);
        } else {
            regular.push(line);
        }
    }
    root_list.sort();
    regular.sort();
    end_list.sort();
    let mut sb = String::new();
    for line in root_list.into_iter().chain(regular).chain(end_list) {
        sb.push_str(&line);
        sb.push('\n');
    }
    sb
}

async fn handle_describe(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    words: &[String],
) -> Result<(), AppError> {
    if words.len() > 1 && words[1].eq_ignore_ascii_case("graph") {
        describe_graph(po, in_route, out_route).await
    } else if words.len() == 3 && words[1].eq_ignore_ascii_case("skill") {
        let command = format!("help {}", words[2].replace('.', "-"));
        let help = get_help(&split(&command, " "));
        say(
            po,
            out_route,
            help.unwrap_or_else(|| format!("'{command}'{NOT_FOUND}")),
        )
        .await;
        Ok(())
    } else if words.len() == 3 && words[1].eq_ignore_ascii_case("node") {
        describe_node(po, in_route, out_route, &words[2]).await
    } else if words.len() == 5
        && words[1].eq_ignore_ascii_case("connection")
        && words[3].eq_ignore_ascii_case("and")
    {
        describe_connection(po, in_route, out_route, &words[2], &words[4]).await
    } else {
        say(po, out_route, TRY_HELP).await;
        Ok(())
    }
}

async fn describe_graph(po: &PostOffice, in_route: &str, out_route: &str) -> Result<(), AppError> {
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    let filename = session::temp_graph_name(in_route);
    let file = temp_dir().join(format!("{filename}.json"));
    std::fs::write(&file, graph.to_json()).map_err(|e| invalid(e.to_string()))?;
    let size = graph.get_nodes().len();
    say(
        po,
        out_route,
        format!(
            "Graph with {size} node{} described in /api/graph/model/{filename}/{}",
            if size == 1 { "" } else { "s" },
            random_counter()
        ),
    )
    .await;
    Ok(())
}

async fn describe_node(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    node_name: &str,
) -> Result<(), AppError> {
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    match graph.find_node_by_alias(node_name)? {
        Some(node) => {
            let mut map: Vec<(Value, Value)> = Vec::new();
            let forward: Vec<Value> = graph
                .get_forward_links(node_name)?
                .iter()
                .map(|n| Value::from(n.get_alias()))
                .collect();
            let backward: Vec<Value> = graph
                .get_backward_links(node_name)?
                .iter()
                .map(|n| Value::from(n.get_alias()))
                .collect();
            if !forward.is_empty() {
                map.push((Value::from("to"), Value::Array(forward)));
            }
            if !backward.is_empty() {
                map.push((Value::from("from"), Value::Array(backward)));
            }
            map.push((Value::from("node"), node_to_value(&node)));
            say_value(po, out_route, Value::Map(map)).await;
        }
        None => say(po, out_route, format!("{NODE_NAME}{node_name}{NOT_FOUND}")).await,
    }
    Ok(())
}

fn node_to_value(node: &Arc<SimpleNode>) -> Value {
    let mut types: Vec<String> = node.get_types().into_iter().collect();
    types.sort();
    let mut properties: Vec<(String, Value)> = node.get_properties().into_iter().collect();
    properties.sort_by(|a, b| a.0.cmp(&b.0));
    Value::Map(vec![
        (Value::from("alias"), Value::from(node.get_alias())),
        (Value::from("id"), Value::from(node.get_id())),
        (
            Value::from("types"),
            Value::Array(types.into_iter().map(Value::from).collect()),
        ),
        (
            Value::from("properties"),
            Value::Map(
                properties
                    .into_iter()
                    .map(|(k, v)| (Value::from(k.as_str()), v))
                    .collect(),
            ),
        ),
    ])
}

async fn describe_connection(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    node_a: &str,
    node_b: &str,
) -> Result<(), AppError> {
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    if !valid_source_and_target(po, out_route, &graph, node_a, node_b).await? {
        return Ok(());
    }
    let mut sb = String::new();
    let forward = graph.find_connection(node_a, node_b)?;
    let backward = graph.find_connection(node_b, node_a)?;
    if let Some(connection) = &forward {
        sb.push_str(&format!(
            "{node_a} -{:?}-> {node_b}\n",
            relation_names(connection)
        ));
    }
    if let Some(connection) = &backward {
        sb.push_str(&format!(
            "{node_b} -{:?}-> {node_a}\n",
            relation_names(connection)
        ));
    }
    if forward.is_none() && backward.is_none() {
        sb.push_str(&format!("{node_a} is not connected to {node_b}\n"));
    }
    say(po, out_route, sb).await;
    Ok(())
}

fn relation_names(connection: &Arc<SimpleConnection>) -> Vec<String> {
    let mut names: Vec<String> = connection
        .get_relations()
        .iter()
        .map(|r| r.get_type().to_string())
        .collect();
    names.sort();
    names
}

async fn valid_source_and_target(
    po: &PostOffice,
    out_route: &str,
    graph: &Arc<MiniGraph>,
    node_a: &str,
    node_b: &str,
) -> Result<bool, AppError> {
    if node_a == node_b {
        say(
            po,
            out_route,
            "source and target node names cannot be the same",
        )
        .await;
        return Ok(false);
    }
    if graph.find_node_by_alias(node_a)?.is_none() {
        say(po, out_route, format!("{NODE_NAME}{node_a}{NOT_FOUND}")).await;
        return Ok(false);
    }
    if graph.find_node_by_alias(node_b)?.is_none() {
        say(po, out_route, format!("{NODE_NAME}{node_b}{NOT_FOUND}")).await;
        return Ok(false);
    }
    Ok(true)
}

async fn handle_edit(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    node_name: &str,
) -> Result<(), AppError> {
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    let Some(node) = graph.find_node_by_alias(node_name)? else {
        return Err(invalid(format!("{NODE_NAME}{node_name}{NOT_FOUND}")));
    };
    say(po, out_route, construct_node_update_command(&node)).await;
    Ok(())
}

/// Java `constructNodeUpdateCommand`: render a node as an editable
/// `update node` command (properties flattened; multi-line values wrapped in
/// triple quotes; array indexes zero-filled for sorting).
fn construct_node_update_command(node: &Arc<SimpleNode>) -> String {
    let mut sb = format!("update node {}\n", node.get_alias());
    let types = node.get_types();
    let mut sorted: Vec<String> = types.into_iter().collect();
    sorted.sort();
    let first = sorted
        .first()
        .cloned()
        .unwrap_or_else(|| UNTYPED.to_string());
    sb.push_str(&format!("with type {first}\n"));
    sb.push_str("with properties\n");
    let properties = node.get_properties();
    if !properties.is_empty() {
        sb.push_str(&raw_properties(&properties));
    }
    sb
}

fn raw_properties(properties: &HashMap<String, Value>) -> String {
    let mut flat: Vec<(String, Value)> = Vec::new();
    for (key, value) in properties {
        flatten(key, value, &mut flat);
    }
    // zero-fill index keys to 3 digits to guarantee correct sorting order
    let mut normalized: Vec<(String, String, Value)> = flat
        .into_iter()
        .map(|(key, value)| (zero_fill_key(&key), strip_index_key(&key), value))
        .collect();
    normalized.sort_by(|a, b| a.0.cmp(&b.0));
    let mut sb = String::new();
    for (_, display_key, value) in normalized {
        let text = display(&value);
        let lines = split(&text, "\n");
        if lines.is_empty() {
            continue;
        }
        sb.push_str(&display_key);
        sb.push('=');
        if lines.len() == 1 {
            sb.push_str(&lines[0]);
            sb.push('\n');
        } else {
            sb.push_str("'''\n");
            for line in &lines {
                sb.push_str(line);
                sb.push('\n');
            }
            sb.push_str("'''\n");
        }
    }
    sb
}

fn flatten(prefix: &str, value: &Value, target: &mut Vec<(String, Value)>) {
    match value {
        Value::Map(entries) => {
            for (k, v) in entries {
                flatten(
                    &format!("{prefix}.{}", k.as_str().unwrap_or_default()),
                    v,
                    target,
                );
            }
        }
        Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                flatten(&format!("{prefix}[{i}]"), v, target);
            }
        }
        leaf => target.push((prefix.to_string(), leaf.clone())),
    }
}

fn zero_fill_key(key: &str) -> String {
    let (Some(open), Some(close)) = (key.find('['), key.rfind(']')) else {
        return key.to_string();
    };
    let index = str2int(&key[open + 1..close]);
    if index < 0 {
        return key.to_string();
    }
    format!("{}{:03}{}", &key[..open + 1], index, &key[close..])
}

fn strip_index_key(key: &str) -> String {
    let (Some(open), Some(close)) = (key.find('['), key.rfind(']')) else {
        return key.to_string();
    };
    format!("{}{}", &key[..open + 1], &key[close..])
}

// ---- inspect / execute ----

async fn handle_inspect(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    key: &str,
) -> Result<(), AppError> {
    let instance = crate::common::get_graph_instance(in_route)?;
    let value = {
        let state = instance.state.lock().expect("graph state machine");
        state.get_element(key).unwrap_or(Value::Map(vec![]))
    };
    if matches!(value, Value::Map(_) | Value::Array(_)) {
        let text = event_script::conversions::to_json_string(&value);
        if text.len() > MAX_BUFFER_SIZE {
            let name = session::temp_graph_name(in_route);
            say(
                po,
                out_route,
                format!(
                    "Large payload ({}) -> GET /api/inspect/{name}/{key}",
                    text.len()
                ),
            )
            .await;
            return Ok(());
        }
    }
    say_value(
        po,
        out_route,
        Value::Map(vec![
            (Value::from("inspect"), Value::from(key)),
            (Value::from("outcome"), value),
        ]),
    )
    .await;
    Ok(())
}

async fn handle_execute(
    platform: &Platform,
    in_route: &str,
    out_route: &str,
    words: &[String],
) -> Result<(), AppError> {
    let node_name = if words.len() == 2 {
        words[1].clone()
    } else if words.len() == 3 && words[1].eq_ignore_ascii_case("node") {
        words[2].clone()
    } else {
        return Err(invalid(
            "Invalid command. Please try EXECUTE NODE {name} or EXECUTE {node-name}",
        ));
    };
    let instance = crate::common::get_graph_instance(in_route)?;
    let timeout = {
        let mut state = instance.state.lock().expect("graph state machine");
        get_model_ttl(&mut state)
    };
    let Some(node) = instance.graph.find_node_by_alias(&node_name)? else {
        return Err(invalid(format!("{NODE_NAME}{node_name}{NOT_FOUND}")));
    };
    let Some(skill) = node.get_property("skill") else {
        return Err(invalid(format!(
            "{NODE_NAME}{node_name} does not have a skill property"
        )));
    };
    let skill_route = display(&skill);
    if !platform.has_route(&skill_route) {
        return Err(invalid(format!(
            "{NODE_NAME} is invalid - Skill '{skill_route}' does not exist"
        )));
    }
    instance
        .node_seen
        .lock()
        .expect("node seen")
        .insert(node_name.clone(), true);
    // fire-and-collect on a background task so the console stays responsive
    let platform = platform.clone();
    let in_route = in_route.to_string();
    let out_route = out_route.to_string();
    tokio::spawn(async move {
        let po = PostOffice::new(&platform);
        let request = EventEnvelope::new()
            .set_to(&skill_route)
            .set_header("in", &in_route)
            .set_header("type", "execute")
            .set_header("node", &node_name);
        match po
            .request(
                request,
                std::time::Duration::from_millis(timeout.max(0) as u64),
            )
            .await
        {
            Ok(response) if response.status() < 400 => {
                let spent = response.exec_time().unwrap_or(0.0);
                say(
                    &po,
                    &out_route,
                    format!(
                        "{NODE_NAME}{node_name} run for {spent} ms with exit path '{}'",
                        display(response.body())
                    ),
                )
                .await;
            }
            Ok(response) => say_value(&po, &out_route, response.body().clone()).await,
            Err(e) => say(&po, &out_route, e.message().to_string()).await,
        }
    });
    Ok(())
}

// ---- create / update / delete / connect ----

async fn handle_multi_line(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    command: &str,
    shown: bool,
) -> Result<(), AppError> {
    let lines = split(command, "\n");
    if !shown {
        say(po, out_route, format!("> {}...", lines[0])).await;
    }
    let words = get_words(&lines[0]);
    if words.len() > 2
        && words[0].eq_ignore_ascii_case("create")
        && words[1].eq_ignore_ascii_case("node")
    {
        handle_create_node(po, in_route, out_route, &words[2], &lines).await
    } else if words.len() > 2
        && words[0].eq_ignore_ascii_case("update")
        && words[1].eq_ignore_ascii_case("node")
    {
        handle_update_node(po, in_route, out_route, &words[2], &lines).await
    } else if words.len() == 2
        && words[0].eq_ignore_ascii_case("instantiate")
        && words[1].eq_ignore_ascii_case("graph")
    {
        handle_instantiate(po, in_route, out_route, &lines).await
    } else {
        say(po, out_route, TRY_HELP).await;
        Ok(())
    }
}

async fn handle_create_node(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    node_name: &str,
    lines: &[String],
) -> Result<(), AppError> {
    let mut key_values = node_properties(lines)?;
    let notice = convert_mapping_properties(&mut key_values)?;
    let node_type = node_type_of(lines);
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    if graph.find_node_by_alias(node_name)?.is_some() {
        say(
            po,
            out_route,
            format!("{NODE_NAME}{node_name} already exists"),
        )
        .await;
        return Ok(());
    }
    let node = graph
        .create_node(node_name, node_type.as_deref().unwrap_or(UNTYPED))
        .map_err(|e| invalid(e.message()))?;
    apply_properties(&node, &key_values)?;
    let message = format!("{NODE_NAME}{node_name} created");
    say(
        po,
        out_route,
        match notice {
            Some(notice) => format!("{message}\n\n{notice}"),
            None => message,
        },
    )
    .await;
    Ok(())
}

async fn handle_update_node(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    node_name: &str,
    lines: &[String],
) -> Result<(), AppError> {
    let mut key_values = node_properties(lines)?;
    let notice = convert_mapping_properties(&mut key_values)?;
    let node_type = node_type_of(lines);
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    let Some(node) = graph.find_node_by_alias(node_name)? else {
        say(po, out_route, format!("{NODE_NAME}{node_name}{NOT_FOUND}")).await;
        return Ok(());
    };
    node.reset_types(node_type.as_deref().unwrap_or(UNTYPED))
        .map_err(|e| invalid(e.message()))?;
    node.clear_properties();
    apply_properties(&node, &key_values)?;
    let message = format!("{NODE_NAME}{node_name} updated");
    say(
        po,
        out_route,
        match notice {
            Some(notice) => format!("{message}\n\n{notice}"),
            None => message,
        },
    )
    .await;
    Ok(())
}

fn apply_properties(node: &Arc<SimpleNode>, key_values: &MultiLevelMap) -> Result<(), AppError> {
    if let Value::Map(entries) = key_values.to_value() {
        for (key, value) in entries {
            node.add_property(key.as_str().unwrap_or_default(), value)
                .map_err(|e| invalid(e.message()))?;
        }
    }
    Ok(())
}

async fn handle_delete(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    words: &[String],
) -> Result<(), AppError> {
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    if words.len() == 3 && words[1].eq_ignore_ascii_case("node") {
        let node_name = &words[2];
        if graph.find_node_by_alias(node_name)?.is_none() {
            say(po, out_route, format!("{NODE_NAME}{node_name}{NOT_FOUND}")).await;
        } else {
            graph.remove_node(node_name)?;
            say(po, out_route, format!("{NODE_NAME}{node_name} deleted")).await;
        }
        Ok(())
    } else if words.len() == 5
        && words[1].eq_ignore_ascii_case("connection")
        && words[3].eq_ignore_ascii_case("and")
    {
        let (node_a, node_b) = (&words[2], &words[4]);
        if valid_source_and_target(po, out_route, &graph, node_a, node_b).await? {
            let mut sb = String::new();
            if graph.find_connection(node_a, node_b)?.is_some() {
                graph.remove_connection(node_a, node_b)?;
                sb.push_str(&format!("{node_a} -> {node_b} removed\n"));
            }
            if graph.find_connection(node_b, node_a)?.is_some() {
                graph.remove_connection(node_b, node_a)?;
                sb.push_str(&format!("{node_b} -> {node_a} removed\n"));
            }
            if sb.is_empty() {
                sb.push_str(&format!("{node_a} has no connections with {node_b}\n"));
            }
            say(po, out_route, sb).await;
        }
        Ok(())
    } else if words.len() == 2 && words[1].eq_ignore_ascii_case("cache") {
        if let Some(instance) = model::get_instance(in_route) {
            instance
                .state
                .lock()
                .expect("graph state machine")
                .remove_element("cache");
            say(po, out_route, "cache cleared").await;
        }
        Ok(())
    } else {
        say(po, out_route, TRY_HELP).await;
        Ok(())
    }
}

async fn handle_connect(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    words: &[String],
) -> Result<(), AppError> {
    if words.len() == 6
        && words[2].eq_ignore_ascii_case("to")
        && words[4].eq_ignore_ascii_case("with")
    {
        let (node_a, node_b, relation) = (&words[1], &words[3], &words[5]);
        let Some(graph) = session::get_graph_model(in_route) else {
            return Ok(());
        };
        if valid_source_and_target(po, out_route, &graph, node_a, node_b).await? {
            graph.connect(node_a, node_b)?.add_relation(relation);
            say(
                po,
                out_route,
                format!("{NODE_NAME}{node_a} connected to {node_b}"),
            )
            .await;
        }
        Ok(())
    } else {
        say(
            po,
            out_route,
            "Syntax: connect {node-A} to {node-B} with {relation}",
        )
        .await;
        Ok(())
    }
}

// ---- export / import / instantiate ----

fn valid_graph_file_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

async fn handle_export(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    filename: &str,
) -> Result<(), AppError> {
    if !valid_graph_file_name(filename) {
        say(po, out_route, INVALID_GRAPH_NAME).await;
        return Ok(());
    }
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    let file = temp_dir().join(format!("{filename}.json"));
    let root = match graph.get_root_node() {
        Some(root) => {
            let name = root.get_property("name").map(|v| display(&v));
            if name.as_deref() != Some(filename) && file.exists() {
                say(
                    po,
                    out_route,
                    format!(
                        "Expect root node name={filename}, Actual: {}\nUpdate root node to \
                         overwrite existing graph model",
                        name.unwrap_or_else(|| "null".to_string())
                    ),
                )
                .await;
                return Ok(());
            }
            root
        }
        None => {
            let root = graph.create_root_node()?;
            root.add_type("root").map_err(|e| invalid(e.message()))?;
            say(po, out_route, "Root node created because it does not exist").await;
            root
        }
    };
    root.add_property("name", Value::from(filename))
        .map_err(|e| invalid(e.message()))?;
    std::fs::write(&file, graph.to_json()).map_err(|e| invalid(e.to_string()))?;
    say(
        po,
        out_route,
        format!(
            "Graph exported to {}\nDescribed in /api/graph/model/{filename}/{}",
            file.display(),
            random_counter()
        ),
    )
    .await;
    Ok(())
}

async fn handle_import_graph(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    filename: &str,
) -> Result<(), AppError> {
    if !valid_graph_file_name(filename) {
        say(po, out_route, INVALID_GRAPH_NAME).await;
        return Ok(());
    }
    if session::get_graph_model(in_route).is_none() {
        return Ok(());
    }
    let file = temp_dir().join(format!("{filename}.json"));
    if file.exists() {
        let json = std::fs::read_to_string(&file).map_err(|e| invalid(e.to_string()))?;
        import_graph_as_draft(po, in_route, out_route, &json).await
    } else {
        say(
            po,
            out_route,
            format!("Graph model not found in {}", file.display()),
        )
        .await;
        match deployed_graph_as_text(filename) {
            Some(json) => {
                say(
                    po,
                    out_route,
                    format!(
                        "Found deployed graph model in {}\nPlease export an updated version and \
                         re-import to instantiate an instance model",
                        deployed_location()
                    ),
                )
                .await;
                import_graph_as_draft(po, in_route, out_route, &json).await
            }
            None => Ok(()),
        }
    }
}

fn deployed_graph_as_text(filename: &str) -> Option<String> {
    let location = deployed_location();
    if let Some(path) = location.strip_prefix("classpath:") {
        let resource = format!("{}/{filename}.json", path.trim_end_matches('/'));
        let resolved = platform_core::resources::resolve_classpath(&resource)?;
        return std::fs::read_to_string(resolved).ok();
    }
    if let Some(path) = location.strip_prefix("file:") {
        let file = PathBuf::from(path).join(format!("{filename}.json"));
        if file.exists() {
            return std::fs::read_to_string(file).ok();
        }
    }
    None
}

async fn import_graph_as_draft(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    json: &str,
) -> Result<(), AppError> {
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    let parsed: serde_json::Value =
        serde_json::from_str(json).map_err(|e| invalid(e.to_string()))?;
    let model = event_script::conversions::from_json(&parsed);
    graph
        .import_graph(&model)
        .map_err(|e| invalid(e.message()))?;
    if model::get_instance(in_route).is_some() {
        say(po, out_route, "Graph instance cleared").await;
        model::remove_instance(in_route);
    }
    say(po, out_route, "Graph model imported as draft").await;
    Ok(())
}

async fn handle_import_node(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    node_name: &str,
    filename: &str,
) -> Result<(), AppError> {
    if !valid_graph_file_name(filename) {
        say(po, out_route, INVALID_GRAPH_NAME).await;
        return Ok(());
    }
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    let file = temp_dir().join(format!("{filename}.json"));
    if !file.exists() {
        return Ok(());
    }
    let json = std::fs::read_to_string(&file).map_err(|e| invalid(e.to_string()))?;
    let parsed: serde_json::Value =
        serde_json::from_str(&json).map_err(|e| invalid(e.to_string()))?;
    let another_graph = MiniGraph::new();
    another_graph
        .import_graph(&event_script::conversions::from_json(&parsed))
        .map_err(|e| invalid(e.message()))?;
    let Some(another_node) = another_graph.find_node_by_alias(node_name)? else {
        say(
            po,
            out_route,
            format!("{NODE_NAME}{node_name} does not exist in {filename}"),
        )
        .await;
        return Ok(());
    };
    let mut types: Vec<String> = another_node.get_types().into_iter().collect();
    types.sort();
    let first = types
        .first()
        .cloned()
        .unwrap_or_else(|| UNTYPED.to_string());
    let overwritten = match graph.find_node_by_alias(node_name)? {
        Some(node) => {
            node.reset_types(&first).map_err(|e| invalid(e.message()))?;
            node.clear_properties();
            for (key, value) in another_node.get_properties() {
                node.add_property(&key, value)
                    .map_err(|e| invalid(e.message()))?;
            }
            true
        }
        None => {
            let node = graph.create_node(another_node.get_alias(), &first)?;
            for (key, value) in another_node.get_properties() {
                node.add_property(&key, value)
                    .map_err(|e| invalid(e.message()))?;
            }
            false
        }
    };
    say(
        po,
        out_route,
        format!(
            "{NODE_NAME}{node_name} {} {filename}",
            if overwritten {
                "overwritten by node from"
            } else {
                "imported from"
            }
        ),
    )
    .await;
    Ok(())
}

async fn handle_instantiate(
    po: &PostOffice,
    in_route: &str,
    out_route: &str,
    lines: &[String],
) -> Result<(), AppError> {
    let Some(graph) = session::get_graph_model(in_route) else {
        return Ok(());
    };
    model::remove_instance(in_route);
    let filename = session::temp_graph_name(in_route);
    let file = temp_dir().join(format!("{filename}.json"));
    std::fs::write(&file, graph.to_json()).map_err(|e| invalid(e.to_string()))?;
    // reload through the config reader so ${...} references resolve
    let model_value =
        crate::compiler::load_raw_graph(&format!("file:{}", temp_dir().display()), &filename)
            .map_err(invalid)?;
    let instance = Arc::new(GraphInstance::new(&format!(
        "playground-{}",
        uuid::Uuid::new_v4().simple()
    )));
    instance
        .graph
        .import_graph(&model_value)
        .map_err(|e| invalid(e.message()))?;
    let node_count = initialize_with_node_properties(&instance)?;
    let mut count = 0;
    {
        let root = instance.graph.get_root_node();
        let end = instance.graph.get_end_node();
        if root.is_none() {
            return Err(invalid("Did you forget to create a root node?"));
        }
        if end.is_none() {
            return Err(invalid("Did you forget to create an end node?"));
        }
        let mut state = instance.state.lock().expect("graph state machine");
        for line in lines.iter().skip(1) {
            let Some(sep) = line.rfind("->") else {
                return Err(invalid(
                    "Invalid data mapping entry. e.g. 'source -> target'",
                ));
            };
            let lhs = line[..sep].trim();
            let rhs = line[sep + 2..].trim();
            let Some(constant) = get_constant_value(lhs) else {
                return Err(invalid(format!("LHS '{lhs}' does not resolve to a value")));
            };
            if rhs.starts_with("input.header")
                || rhs.starts_with("input.body")
                || rhs.starts_with("model.")
            {
                state.set_element(rhs, constant).map_err(invalid)?;
                count += 1;
            } else {
                return Err(invalid(format!(
                    "RHS must use input.body, input.header or model namespace. Actual: {rhs}"
                )));
            }
        }
        if !state.exists("input.body") {
            state
                .set_element("input.body", Value::Map(vec![]))
                .map_err(invalid)?;
        }
        state
            .set_element("output", Value::Map(vec![]))
            .map_err(invalid)?;
    }
    let timeout = {
        let mut state = instance.state.lock().expect("graph state machine");
        get_model_ttl(&mut state)
    };
    log::info!("Instantiate graph with {node_count} nodes, model.ttl = {timeout} ms");
    model::add_instance(in_route, instance);
    say(
        po,
        out_route,
        format!(
            "Graph instance created. Loaded {count} mock {}, model.ttl = {timeout} ms",
            if count == 1 { "entry" } else { "entries" }
        ),
    )
    .await;
    Ok(())
}

// ---- property parsing (with type / with properties / triple quotes) ----

fn node_type_of(lines: &[String]) -> Option<String> {
    for line in lines {
        if line.to_lowercase().trim().starts_with("with type") {
            let words = split(line, " ");
            if words.len() > 2 {
                return Some(words[2].clone());
            }
        }
    }
    None
}

fn node_properties(lines: &[String]) -> Result<MultiLevelMap, AppError> {
    let mut result = MultiLevelMap::new();
    let mut property_lines: Vec<String> = Vec::new();
    let mut found = false;
    for line in lines {
        let lower = line.to_lowercase().trim().to_string();
        if lower.starts_with("with properties") {
            found = true;
        } else if found {
            if lower.starts_with("with type") {
                break;
            }
            property_lines.push(line.clone());
        }
    }
    if property_lines.is_empty() {
        return Ok(result);
    }
    let mut multiline = false;
    let mut ml_key: Option<String> = None;
    let mut sb = String::new();
    for line in &property_lines {
        if multiline {
            match line.find("'''") {
                Some(mark) => {
                    let value = line[..mark].trim();
                    if !value.is_empty() {
                        sb.push_str(value);
                        sb.push('\n');
                    }
                    if let Some(key) = &ml_key {
                        result
                            .set_element(key, Value::from(sb.trim()))
                            .map_err(invalid)?;
                    }
                    sb.clear();
                    multiline = false;
                }
                None => {
                    sb.push_str(line);
                    sb.push('\n');
                }
            }
        } else {
            let eq = line.find('=');
            let key = match eq {
                Some(eq) => line[..eq].trim().to_string(),
                None => line.clone(),
            };
            let value = match eq {
                Some(eq) => line[eq + 1..].trim().to_string(),
                None => String::new(),
            };
            match value
                .find("'''")
                .or(if eq.is_none() { line.find("'''") } else { None })
            {
                Some(mark) => {
                    multiline = true;
                    ml_key = Some(key);
                    let rest = value[(mark + 3).min(value.len())..].trim();
                    if !rest.is_empty() {
                        sb.push_str(rest);
                        sb.push('\n');
                    }
                }
                None => {
                    result
                        .set_element(&key, Value::from(value.as_str()))
                        .map_err(invalid)?;
                }
            }
        }
    }
    Ok(result)
}

/// Validate and auto-convert `mapping`/`input`/`output`/`for_each` entries
/// (Java `convertMappingProperties`): deprecated "simple type matching"
/// converts to plugin syntax with a deprecation notice; malformed syntax and
/// unknown `f:` plugins are rejected immediately.
fn convert_mapping_properties(key_values: &mut MultiLevelMap) -> Result<Option<String>, AppError> {
    let mut conversions: Vec<String> = Vec::new();
    for property in MAPPING_PROPERTIES {
        if let Some(Value::Array(entries)) = key_values.get_element(property) {
            let mut converted: Vec<Value> = Vec::new();
            for entry in &entries {
                let line = display(entry);
                let Some(sep) = line.rfind("->").filter(|sep| *sep > 0) else {
                    return Err(invalid(format!(
                        "Invalid '{property}' entry - syntax must be 'LHS -> RHS'. Actual: '{line}'"
                    )));
                };
                let lhs = line[..sep].trim();
                let rhs = line[sep + 2..].trim();
                if lhs.is_empty() || rhs.is_empty() {
                    return Err(invalid(format!(
                        "Invalid '{property}' entry - LHS and RHS must not be empty. Actual: '{line}'"
                    )));
                }
                let converted_line = converter::convert(&line);
                if converted_line != line {
                    conversions.push(format!("{line}  =>  {converted_line}"));
                }
                validate_plugin_reference(property, &converted_line)?;
                converted.push(Value::from(converted_line));
            }
            key_values
                .set_element(property, Value::Array(converted))
                .map_err(invalid)?;
        }
    }
    if conversions.is_empty() {
        return Ok(None);
    }
    let mut sb = String::from(
        "DEPRECATION NOTICE for AI agents and developers: 'simple type matching' syntax \
         (e.g. model.key:type) is deprecated. Please use 'simple plugin' syntax instead \
         (e.g. f:type(model.key)). The following ",
    );
    sb.push_str(if conversions.len() == 1 {
        "entry was"
    } else {
        "entries were"
    });
    sb.push_str(" automatically converted:\n");
    for c in &conversions {
        sb.push_str(&format!("  {c}\n"));
    }
    Ok(Some(sb.trim().to_string()))
}

fn validate_plugin_reference(property: &str, line: &str) -> Result<(), AppError> {
    let Some(sep) = line.rfind("->") else {
        return Ok(());
    };
    let lhs = line[..sep].trim();
    if let Some(call) = lhs.strip_prefix("f:") {
        let open = call.find('(');
        let close = call.rfind(')');
        let (Some(open), Some(close)) = (open, close) else {
            return Err(invalid(format!(
                "Invalid '{property}' entry - malformed plugin call. Actual: '{lhs}'"
            )));
        };
        if close < open {
            return Err(invalid(format!(
                "Invalid '{property}' entry - malformed plugin call. Actual: '{lhs}'"
            )));
        }
        let plugin_name = &call[..open];
        if !event_script::plugins::contains_simple_plugin(plugin_name) {
            return Err(invalid(format!(
                "Invalid '{property}' entry - unknown simple plugin 'f:{plugin_name}'. \
                 Actual: '{lhs}'"
            )));
        }
    }
    Ok(())
}

// ---- help ----

fn get_help(words: &[String]) -> Option<String> {
    let title = words
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
    let resolved = platform_core::resources::resolve_classpath(&format!("/help/{title}.md"))?;
    std::fs::read_to_string(resolved).ok()
}

// ---- companion / REST support (Java statics on GraphCommandService) ----

/// Java `hasSession`.
pub fn has_session(id: &str) -> bool {
    session::has_graph_model(&GraphSession::in_route_of(id))
}

/// Java `uploadContent`: load mock data into a live instance's `input.body`.
pub async fn upload_content(platform: &Platform, id: &str, content: Value) -> bool {
    let in_route = GraphSession::in_route_of(id);
    let out_route = GraphSession::out_route_of(id);
    let Some(instance) = model::get_instance(&in_route) else {
        return false;
    };
    {
        let mut state = instance.state.lock().expect("graph state machine");
        let _ = state.set_element("input.body", content);
    }
    let po = PostOffice::new(platform);
    say(
        &po,
        &out_route,
        "Mock data loaded into 'input.body' namespace",
    )
    .await;
    true
}

/// Java `downloadContent`: read a key from a live instance's state machine.
pub fn download_content(id: &str, key: &str) -> Option<Value> {
    let in_route = GraphSession::in_route_of(id);
    let instance = model::get_instance(&in_route)?;
    let state = instance.state.lock().expect("graph state machine");
    state.get_element(key)
}

/// Java `downloadGraph`: export a session's draft graph.
pub fn download_graph(id: &str) -> Option<Value> {
    let in_route = GraphSession::in_route_of(id);
    let graph = session::get_graph_model(&in_route)?;
    Some(graph.export_graph())
}
