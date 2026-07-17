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

//! Playground session state (Java `GraphSession` + the `sessions` /
//! `graphModels` registries on the `GraphLambdaFunction` base class). A
//! session is keyed by its websocket `.in` route; its public id swaps dots
//! for hyphens (`ws.123456.1.in` → `ws-123456-1`) so it can travel in URL
//! paths (the companion endpoint). A secondary session may subscribe to a
//! primary session for collaborative editing.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, RwLock};

use platform_core::graph::MiniGraph;

const IN_SUFFIX: &str = ".in";
const OUT_SUFFIX: &str = ".out";

/// One collaborative session (Java `GraphSession`).
pub struct GraphSession {
    session_id: String,
    target_id: Mutex<String>,
    subscribers: Mutex<HashMap<String, bool>>,
    pub start_time_ms: i64,
}

impl GraphSession {
    pub fn new(route: &str) -> Self {
        let id = GraphSession::session_id_of(route);
        GraphSession {
            session_id: id.clone(),
            target_id: Mutex::new(id),
            subscribers: Mutex::new(HashMap::new()),
            start_time_ms: now_ms(),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn target_id(&self) -> String {
        self.target_id.lock().expect("session target").clone()
    }

    pub fn is_primary(&self) -> bool {
        self.session_id == self.target_id()
    }

    pub fn set_target_id(&self, session_id: &str) {
        if !session_id.is_empty() {
            *self.target_id.lock().expect("session target") = session_id.to_string();
        }
    }

    pub fn subscribers(&self) -> Vec<String> {
        self.subscribers
            .lock()
            .expect("session subscribers")
            .keys()
            .cloned()
            .collect()
    }

    pub fn subscribe(&self, out_route: &str) {
        self.subscribers
            .lock()
            .expect("session subscribers")
            .insert(out_route.to_string(), true);
    }

    pub fn has_subscriber(&self, out_route: &str) -> bool {
        self.subscribers
            .lock()
            .expect("session subscribers")
            .contains_key(out_route)
    }

    pub fn unsubscribe(&self, out_route: &str) {
        self.subscribers
            .lock()
            .expect("session subscribers")
            .remove(out_route);
    }

    /// Java `getSessionId`: the public hyphenated id of a route.
    pub fn session_id_of(route: &str) -> String {
        let id = if let Some(stem) = route.strip_suffix(IN_SUFFIX) {
            stem
        } else if let Some(stem) = route.strip_suffix(OUT_SUFFIX) {
            stem
        } else {
            return route.to_string();
        };
        id.replace('.', "-")
    }

    /// Java `getInRoute`.
    pub fn in_route_of(session_id: &str) -> String {
        if session_id.ends_with(IN_SUFFIX) {
            session_id.to_string()
        } else if let Some(stem) = session_id.strip_suffix(OUT_SUFFIX) {
            format!("{stem}{IN_SUFFIX}")
        } else {
            format!("{}{IN_SUFFIX}", session_id.replace('-', "."))
        }
    }

    /// Java `getOutRoute`.
    pub fn out_route_of(session_id: &str) -> String {
        if session_id.ends_with(OUT_SUFFIX) {
            session_id.to_string()
        } else if let Some(stem) = session_id.strip_suffix(IN_SUFFIX) {
            format!("{stem}{OUT_SUFFIX}")
        } else {
            format!("{}{OUT_SUFFIX}", session_id.replace('-', "."))
        }
    }
}

pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

// ---- registries (Java GraphLambdaFunction statics) ----

fn sessions() -> &'static RwLock<HashMap<String, Arc<GraphSession>>> {
    static SESSIONS: OnceLock<RwLock<HashMap<String, Arc<GraphSession>>>> = OnceLock::new();
    SESSIONS.get_or_init(|| RwLock::new(HashMap::new()))
}

fn graph_models() -> &'static RwLock<HashMap<String, Arc<MiniGraph>>> {
    static MODELS: OnceLock<RwLock<HashMap<String, Arc<MiniGraph>>>> = OnceLock::new();
    MODELS.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn get_session(in_route: &str) -> Option<Arc<GraphSession>> {
    sessions()
        .read()
        .expect("sessions poisoned")
        .get(in_route)
        .cloned()
}

pub fn put_session(in_route: &str, session: Arc<GraphSession>) {
    sessions()
        .write()
        .expect("sessions poisoned")
        .insert(in_route.to_string(), session);
}

pub fn remove_session(in_route: &str) {
    sessions()
        .write()
        .expect("sessions poisoned")
        .remove(in_route);
}

pub fn get_graph_model(in_route: &str) -> Option<Arc<MiniGraph>> {
    graph_models()
        .read()
        .expect("graph models poisoned")
        .get(in_route)
        .cloned()
}

pub fn put_graph_model(in_route: &str, graph: Arc<MiniGraph>) {
    graph_models()
        .write()
        .expect("graph models poisoned")
        .insert(in_route.to_string(), graph);
}

pub fn remove_graph_model(in_route: &str) {
    graph_models()
        .write()
        .expect("graph models poisoned")
        .remove(in_route);
}

pub fn has_graph_model(in_route: &str) -> bool {
    graph_models()
        .read()
        .expect("graph models poisoned")
        .contains_key(in_route)
}

/// Java `getTempGraphName`: the draft-file name for a session route
/// (`ws.123456.1.in` → `ws-123456-1`).
pub fn temp_graph_name(in_route: &str) -> String {
    let name = match in_route.rfind('.') {
        Some(dot) => &in_route[..dot],
        None => in_route,
    };
    name.replace('.', "-")
}
