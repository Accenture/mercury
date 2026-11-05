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

//! The Rust twin of the Java `LlmRunVectorsTest`: runs the shared vector file `llm-run-vectors.json`
//! (byte-identical with the Java repository) against the loop, with a scripted `llm.chat` and
//! scripted tool routes behind the gateway and a clock the script advances. No token is spent and
//! nothing waits on real time.
//!
//! Every case runs, and every failing case is reported by name: a scripted-side surprise (an
//! unscripted turn or call, a request that differs) is recorded by the gateway and reported before
//! the outcome, as the Java harness's `AssertionError` would be.

use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use agent_loop::json::compact;
use agent_loop::{json_equals, AgentLoop, Defaults, LoopGateway, ToolSchema, LLM_CHAT};
use async_trait::async_trait;
use platform_core::{AppError, EventEnvelope};
use rmpv::Value;

const VECTOR_FILE: &str = include_str!("resources/vectors/llm-run-vectors.json");
const LOOP_CASES: usize = 105;
const SCHEMA_CASES: usize = 29;

fn vectors() -> &'static Value {
    static VECTORS: OnceLock<Value> = OnceLock::new();
    // parsed into the platform's value type, so every map keeps the order the file writes
    VECTORS.get_or_init(|| serde_json::from_str(VECTOR_FILE).expect("llm-run-vectors.json"))
}

fn field<'a>(map: &'a Value, key: &str) -> Option<&'a Value> {
    match map {
        Value::Map(entries) => entries
            .iter()
            .find(|(name, _)| name.as_str() == Some(key))
            .map(|(_, value)| value),
        _ => None,
    }
}

fn owned(map: &Value, key: &str) -> Value {
    field(map, key).cloned().unwrap_or(Value::Nil)
}

fn list<'a>(map: &'a Value, key: &str) -> &'a [Value] {
    match field(map, key) {
        Some(Value::Array(items)) => items,
        _ => &[],
    }
}

fn name_of(case: &Value) -> String {
    field(case, "name")
        .and_then(Value::as_str)
        .unwrap_or("?")
        .to_string()
}

#[test]
fn the_file_names_its_contract() {
    let vectors = vectors();
    assert_eq!(
        field(vectors, "contract").and_then(Value::as_str),
        Some("llm-run-loop")
    );
    assert_eq!(field(vectors, "version").and_then(Value::as_i64), Some(1));
    assert_eq!(list(vectors, "cases").len(), LOOP_CASES, "loop cases");
    assert_eq!(
        list(vectors, "schema_cases").len(),
        SCHEMA_CASES,
        "schema cases"
    );
    let mut names = HashSet::new();
    for case in list(vectors, "cases")
        .iter()
        .chain(list(vectors, "schema_cases"))
    {
        let name = name_of(case);
        assert!(names.insert(name.clone()), "duplicate case name {name}");
    }
}

#[tokio::test]
async fn loop_cases() {
    let cases = list(vectors(), "cases");
    let mut failures = Vec::new();
    for case in cases {
        if let Err(message) = run_case(case).await {
            failures.push(format!("{}: {message}", name_of(case)));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} loop cases failed:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}

#[test]
fn schema_cases() {
    let cases = list(vectors(), "schema_cases");
    let mut failures = Vec::new();
    for case in cases {
        if let Err(message) = run_schema_case(case) {
            failures.push(format!("{}: {message}", name_of(case)));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} schema cases failed:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}

async fn run_case(case: &Value) -> Result<(), String> {
    let clock = Arc::new(AtomicI64::new(0));
    let gateway = ScriptedGateway::new(case, clock.clone());
    let tick = clock.clone();
    let agent = AgentLoop::new(
        &gateway,
        defaults(field(case, "config")),
        Box::new(move || tick.load(Ordering::SeqCst)),
    );
    let input = owned(case, "input");
    let expect = owned(case, "expect");
    let outcome = agent.run(&input).await;
    if let Some(failure) = gateway.failure() {
        return Err(failure);
    }
    match field(&expect, "error") {
        Some(error) => {
            let status = field(error, "status").and_then(Value::as_i64).unwrap_or(-1);
            let message = field(error, "message")
                .and_then(Value::as_str)
                .unwrap_or("");
            match outcome {
                Ok(envelope) => {
                    return Err(format!(
                        "expected the error {status} '{message}', got the envelope {}",
                        compact(&envelope)
                    ))
                }
                Err(e) if i64::from(e.status()) != status || e.message() != message => {
                    return Err(format!(
                        "error\n expected {status} {message}\n      got {} {}",
                        e.status(),
                        e.message()
                    ))
                }
                Err(_) => {}
            }
        }
        None => {
            let expected = owned(&expect, "envelope");
            match outcome {
                Err(e) => {
                    return Err(format!(
                        "expected the envelope {}, got the error {} {}",
                        compact(&expected),
                        e.status(),
                        e.message()
                    ))
                }
                Ok(envelope) if !json_equals(&expected, &envelope) => {
                    return Err(format!(
                        "envelope\n expected {}\n      got {}",
                        compact(&expected),
                        compact(&envelope)
                    ))
                }
                Ok(_) => {}
            }
        }
    }
    if let Some(expected) = field(&expect, "events") {
        let events = Value::Array(gateway.events());
        if !json_equals(expected, &events) {
            return Err(format!(
                "events\n expected {}\n      got {}",
                compact(expected),
                compact(&events)
            ));
        }
    }
    gateway.verify_used()
}

/// The configuration keys a case names, over the built-in defaults.
fn defaults(config: Option<&Value>) -> Defaults {
    let mut defaults = Defaults::built_in();
    let Some(config) = config else {
        return defaults;
    };
    let text = |key: &str, fallback: &mut String| {
        if let Some(value) = field(config, key).and_then(Value::as_str) {
            *fallback = value.to_string();
        }
    };
    text(Defaults::MAX_TURNS_KEY, &mut defaults.max_turns);
    text(Defaults::RESULT_LIMIT_KEY, &mut defaults.result_limit);
    text(Defaults::TRANSCRIPT_TTL_KEY, &mut defaults.transcript_ttl);
    text(Defaults::RETENTION_KEY, &mut defaults.retention);
    text(Defaults::TIMEOUT_KEY, &mut defaults.timeout);
    defaults
}

fn run_schema_case(case: &Value) -> Result<(), String> {
    let schema = owned(case, "schema");
    if let Some(expected) = field(case, "error") {
        let expected = expected.as_str().unwrap_or_default();
        return match ToolSchema::compile("schema", &schema) {
            Ok(_) => Err(format!("compiled, expected the error '{expected}'")),
            Err(message) if message != expected => {
                Err(format!("error\n expected {expected}\n      got {message}"))
            }
            Err(_) => Ok(()),
        };
    }
    let compiled = ToolSchema::compile("schema", &schema)
        .map_err(|message| format!("does not compile: {message}"))?;
    let mut violations = Vec::new();
    compiled.validate("input", &owned(case, "input"), &mut violations);
    let expected: Vec<String> = list(case, "violations")
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_string())
        .collect();
    if violations != expected {
        return Err(format!(
            "violations\n expected {expected:?}\n      got {violations:?}"
        ));
    }
    if let Some(report) = field(case, "report").and_then(Value::as_str) {
        let got = ToolSchema::report(&violations);
        if got != report {
            return Err(format!("report\n expected {report}\n      got {got}"));
        }
    }
    Ok(())
}

/// The scripted side of a case: `llm.chat` answers each turn in order after checking the request the
/// loop sent, and each route call is matched by route and body among the calls the script expects
/// before the next turn.
struct ScriptedGateway {
    routes: Vec<String>,
    llm_chat: bool,
    clock: Arc<AtomicI64>,
    script: Mutex<Script>,
}

#[derive(Default)]
struct Script {
    turns: VecDeque<Value>,
    pending: Vec<Value>,
    events: Vec<Value>,
    turn_number: usize,
    failure: Option<String>,
}

impl ScriptedGateway {
    fn new(case: &Value, clock: Arc<AtomicI64>) -> ScriptedGateway {
        let mut script = Script {
            turns: list(case, "turns").iter().cloned().collect(),
            ..Script::default()
        };
        if script
            .turns
            .front()
            .is_some_and(|turn| field(turn, "before").is_some())
        {
            let before = script.turns.pop_front().unwrap_or(Value::Nil);
            script.pending = list(&before, "before").to_vec();
        }
        ScriptedGateway {
            routes: list(case, "routes")
                .iter()
                .filter_map(|route| route.as_str().map(str::to_string))
                .collect(),
            llm_chat: !matches!(field(case, "llm_chat"), Some(Value::Boolean(false))),
            clock,
            script: Mutex::new(script),
        }
    }

    fn script(&self) -> std::sync::MutexGuard<'_, Script> {
        self.script.lock().expect("script")
    }

    /// Record the first surprise and answer with an error, so the run ends and the case reports it.
    fn fail(script: &mut Script, message: String) -> Result<EventEnvelope, AppError> {
        script.failure.get_or_insert(message.clone());
        Err(AppError::new(599, message))
    }

    fn failure(&self) -> Option<String> {
        self.script().failure.clone()
    }

    fn events(&self) -> Vec<Value> {
        self.script().events.clone()
    }

    fn answer(scripted: &Value) -> EventEnvelope {
        match field(scripted, "error") {
            Some(error @ Value::Map(_)) => {
                let status = field(error, "status")
                    .and_then(Value::as_i64)
                    .unwrap_or(500);
                EventEnvelope::new()
                    .set_status(status as i32)
                    .set_raw_body(owned(error, "message"))
            }
            _ => EventEnvelope::new()
                .set_status(200)
                .set_raw_body(owned(scripted, "reply")),
        }
    }

    fn verify_used(&self) -> Result<(), String> {
        let script = self.script();
        if !script.turns.is_empty() {
            return Err(format!(
                "{} scripted turn(s) never reached",
                script.turns.len()
            ));
        }
        if !script.pending.is_empty() {
            return Err(format!(
                "scripted call(s) never made: {}",
                compact(&Value::Array(script.pending.clone()))
            ));
        }
        Ok(())
    }
}

#[async_trait]
impl LoopGateway for ScriptedGateway {
    fn reachable(&self, route: &str) -> bool {
        if route == LLM_CHAT {
            self.llm_chat
        } else {
            self.routes.iter().any(|r| r == route)
        }
    }

    async fn chat(&self, request: Value, _timeout_ms: i64) -> Result<EventEnvelope, AppError> {
        let mut script = self.script();
        script.turn_number += 1;
        let number = script.turn_number;
        if !script.pending.is_empty() {
            let pending = compact(&Value::Array(script.pending.clone()));
            return Self::fail(
                &mut script,
                format!("turn {number} began before these calls were made: {pending}"),
            );
        }
        let Some(turn) = script.turns.pop_front() else {
            return Self::fail(
                &mut script,
                format!(
                    "an unscripted llm.chat turn {number}: {}",
                    compact(&request)
                ),
            );
        };
        let expected = owned(&turn, "request");
        if !json_equals(&expected, &request) {
            return Self::fail(
                &mut script,
                format!(
                    "turn {number} request\n expected {}\n      got {}",
                    compact(&expected),
                    compact(&request)
                ),
            );
        }
        if let Some(elapsed) = field(&turn, "elapsed_ms").and_then(Value::as_i64) {
            self.clock.fetch_add(elapsed, Ordering::SeqCst);
        }
        script.pending = list(&turn, "calls").to_vec();
        Ok(Self::answer(&turn))
    }

    async fn call(
        &self,
        route: &str,
        body: Value,
        _timeout_ms: i64,
    ) -> Result<EventEnvelope, AppError> {
        let mut script = self.script();
        let found = script.pending.iter().position(|expected| {
            field(expected, "route").and_then(Value::as_str) == Some(route)
                && json_equals(&owned(expected, "body"), &body)
        });
        match found {
            Some(index) => {
                let expected = script.pending.remove(index);
                Ok(Self::answer(&expected))
            }
            None => Self::fail(
                &mut script,
                format!("an unscripted call to {route} with {}", compact(&body)),
            ),
        }
    }

    async fn progress(&self, event: Value) {
        self.script().events.push(event);
    }
}
