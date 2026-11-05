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

//! Rust port of the Java `AgentLoop` (`org.platformlambda.agentloop.AgentLoop`): the agent loop
//! (RFC-0011, section 4), one run of model turns to an outcome. Each turn goes to `llm.chat`, which
//! keeps the transcript in the language pack's store and returns either the outcome or the model's tool
//! calls; the loop resolves each call's name to a route in the node's catalog, validates the input,
//! applies the node's approval mode, calls the route, and sends every result of the turn back in one
//! message. It holds the conversation's reference, never the transcript.
//!
//! A model outcome - done, an approval, a refusal, a limit - is the envelope a decision node routes
//! on. Only the loop's own machinery fails: a bad input or catalog (before a token is spent), a resume
//! whose decisions do not match the waiting calls, or `llm.chat` failing after the pack's own retries.
//!
//! The values are the platform's own (`rmpv::Value`), so a map keeps the order its sender wrote, as
//! the Java engine's `LinkedHashMap` does, and the envelope lists its fields in the order the spec
//! does. A turn's tool calls run concurrently on the function's own task (Java issues every request
//! on the worker thread before awaiting any), so each request carries the `llm.run` span as its
//! parent; they are answered in call order.

use std::collections::{HashMap, HashSet};

use futures_util::stream::{FuturesOrdered, StreamExt};
use platform_core::{AppError, EventEnvelope};
use rmpv::Value;

use crate::gateway::LoopGateway;
use crate::json::compact;
use crate::platform_gateway::LLM_CHAT;
use crate::settings::{
    Defaults, LoopSettings, MANUAL, MAX_TURNS_INPUT, MODE_INPUT, RESULT_LIMIT_INPUT,
    RETENTION_INPUT, TIMEOUT_INPUT, TRANSCRIPT_TTL_INPUT, UNATTENDED, WRITE,
};
use crate::tool_catalog::ToolCatalog;
use crate::tool_schema::{Decimal, ToolSchema};
use crate::value::{
    and_list, contains_key, copy, entries, get, java_cmp, java_is_blank, java_trim, key_text,
    map_of, put, text, value_of,
};

/// Time kept back from the deadline so the envelope reaches the graph before the node's own timeout.
pub const RESERVE_MS: i64 = 1000;
/// The least time a model turn, or a round of tool calls, is started with.
pub const MIN_STEP_MS: i64 = 1000;
pub(crate) const TOOLS: &str = "tools";
const SYSTEM: &str = "system";
const PROMPT: &str = "prompt";
const MESSAGES: &str = "messages";
const SCHEMA: &str = "schema";
pub(crate) const MODEL: &str = "model";
const PARAMS: &str = "params";
pub(crate) const CONVERSATION: &str = "conversation";
pub(crate) const TURN: &str = "turn";
const APPROVAL: &str = "approval";
const DECISIONS: &str = "decisions";
pub(crate) const STATUS: &str = "status";
const TEXT: &str = "text";
const DATA: &str = "data";
const DECISION: &str = "decision";
pub(crate) const CAP: &str = "cap";
const REFUSAL: &str = "refusal";
pub(crate) const TURNS: &str = "turns";
pub(crate) const USAGE: &str = "usage";
const CALLS: &str = "calls";
const ID: &str = "id";
const NAME: &str = "name";
const INPUT: &str = "input";
const ROUTE: &str = "route";
const WAIT: &str = "wait";
const APPROVED: &str = "approved";
const NOTE: &str = "note";
const OUTPUT: &str = "output";
const ERROR: &str = "error";
const RESULTS: &str = "results";
const TTL: &str = "ttl";
const RETENTION: &str = "retention";
const TIMEOUT_MS: &str = "timeout_ms";
const DONE: &str = "done";
const TOOL_USE: &str = "tool_use";
const CAP_TURNS: &str = "turns";
const CAP_DEADLINE: &str = "deadline";
const OK: &str = "ok";
const UNKNOWN: &str = "unknown";
const INVALID: &str = "invalid";
const REFUSED: &str = "refused";
const ANSWERED: &str = "answered";
const ASK: &str = "ask";
const ANSWER: &str = "answer";
const USAGE_KEYS: [&str; 4] = [
    "input_tokens",
    "output_tokens",
    "cache_creation_input_tokens",
    "cache_read_input_tokens",
];
const INPUTS: [&str; 17] = [
    TOOLS,
    SYSTEM,
    PROMPT,
    MESSAGES,
    SCHEMA,
    MODEL,
    PARAMS,
    MODE_INPUT,
    MAX_TURNS_INPUT,
    RESULT_LIMIT_INPUT,
    TRANSCRIPT_TTL_INPUT,
    RETENTION_INPUT,
    TIMEOUT_INPUT,
    CONVERSATION,
    TURN,
    APPROVAL,
    DECISIONS,
];
const LLM_CHAT_FAILED: &str = "llm.chat failed - ";
const OUTSIDE_CONTRACT: &str = "llm.chat answered outside the loop contract - ";
const NO_LONGER_MATCHES: &str = "') no longer matches the tool catalog or the mode";
const DECISION_SHAPE: &str = " must be {id, approved, note}, or {id, answer} for a question";
const NOT_A_MAP: &str = "the input of llm.run must be a map of its inputs";
// progress events (RFC-0011 section 8.4): sent to the caller's return route, never content
const TYPE: &str = "type";
const PROGRESS: &str = "progress";
const EVENT: &str = "event";
const SEQ: &str = "seq";
const TIME: &str = "time";
const RUN_START: &str = "run.start";
const RUN_END: &str = "run.end";
const TURN_START: &str = "turn.start";
const TURN_END: &str = "turn.end";
const TOOL_START: &str = "tool.start";
const TOOL_END: &str = "tool.end";
const APPROVAL_WAIT: &str = "approval.wait";
const STOP_REASON: &str = "stop_reason";
const TOOL_COUNT: &str = "tools";
const MS: &str = "ms";

/// The time in milliseconds, from which the deadline is measured.
pub type Clock<'a> = Box<dyn Fn() -> i64 + Send + Sync + 'a>;

/// The agent loop: one run of model turns to an outcome envelope.
pub struct AgentLoop<'a> {
    gateway: &'a dyn LoopGateway,
    defaults: Defaults,
    clock: Clock<'a>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Run,
    Wait,
    Error,
    Answer,
}

/// What a planned call needs of its catalog row.
#[derive(Clone, Debug)]
struct RowRef {
    route: Option<String>,
    ask: bool,
}

/// One call of a turn, planned: run it, wait for a person, answer it with an error, or answer it with
/// what a person said. `value` is the error text for `Error` and the person's answer for `Answer`.
#[derive(Debug)]
struct Planned {
    id: String,
    name: String,
    row: Option<RowRef>,
    input: Value,
    outcome: Outcome,
    label: &'static str,
    value: Value,
}

impl Planned {
    fn new(id: &str, name: &str, row: Option<RowRef>, input: Value, outcome: Outcome) -> Planned {
        Planned {
            id: id.to_string(),
            name: name.to_string(),
            row,
            input,
            outcome,
            label: "",
            value: Value::Nil,
        }
    }

    fn answered(mut self, label: &'static str, value: Value) -> Planned {
        self.label = label;
        self.value = value;
        self
    }

    fn route(&self) -> Option<&str> {
        self.row.as_ref().and_then(|row| row.route.as_deref())
    }

    fn asks(&self) -> bool {
        self.row.as_ref().is_some_and(|row| row.ask)
    }
}

/// The run's own state; the transcript is never part of it.
struct Run<'i> {
    settings: LoopSettings,
    input: &'i Value,
    deadline: i64,
    conversation: Option<String>,
    turn: i64,
    turns: i64,
    model: Option<String>,
    usage: [i64; 4],
    calls: Vec<Value>,
    seq: i64,
}

impl<'i> Run<'i> {
    fn new(settings: LoopSettings, input: &'i Value, deadline: i64) -> Run<'i> {
        Run {
            settings,
            input,
            deadline,
            conversation: None,
            turn: 0,
            turns: 0,
            model: None,
            usage: [0; 4],
            calls: Vec::new(),
            seq: 0,
        }
    }

    fn usage(&self) -> Value {
        Value::Map(
            USAGE_KEYS
                .iter()
                .zip(self.usage)
                .map(|(key, n)| (Value::from(*key), Value::from(n)))
                .collect(),
        )
    }
}

impl<'a> AgentLoop<'a> {
    /// `gateway` is how the loop reaches `llm.chat` and the tools' routes, `defaults` the global
    /// defaults of the settings, and `clock` the time in milliseconds, from which the deadline is
    /// measured.
    pub fn new(
        gateway: &'a dyn LoopGateway,
        defaults: Defaults,
        clock: Clock<'a>,
    ) -> AgentLoop<'a> {
        AgentLoop {
            gateway,
            defaults,
            clock,
        }
    }

    /// Run the loop over the node's input (a map) and answer the outcome envelope; an `Err` is a
    /// failure of the loop's own machinery.
    pub async fn run(&self, input: &Value) -> Result<Value, AppError> {
        if !matches!(input, Value::Map(_)) {
            return Err(AppError::new(400, NOT_A_MAP));
        }
        let start = (self.clock)();
        let (settings, catalog) = self
            .prepare(input)
            .map_err(|message| AppError::new(400, message))?;
        if !self.gateway.reachable(LLM_CHAT) {
            return Err(AppError::new(
                503,
                "llm.chat is not reachable - deploy a language pack's LLM helper and declare \
                 llm.chat in yaml.event.over.http",
            ));
        }
        let deadline = start + settings.timeout_ms;
        let mut run = Run::new(settings, input, deadline);
        let mut begin = Vec::new();
        if let Some(model) = text(get(input, MODEL)) {
            put(&mut begin, MODEL, Value::from(model));
        }
        put(
            &mut begin,
            TOOL_COUNT,
            Value::from(catalog.advertised().len() as i64),
        );
        put(
            &mut begin,
            MAX_TURNS_INPUT,
            Value::from(run.settings.max_turns),
        );
        self.emit(&mut run, RUN_START, Value::Map(begin)).await;
        match self.run_loop(&mut run, &catalog).await {
            Ok(envelope) => {
                let status = value_of(get(&envelope, STATUS));
                let data = ended(&run, &status);
                self.emit(&mut run, RUN_END, data).await;
                Ok(envelope)
            }
            Err(e) => {
                let data = ended(&run, ERROR);
                self.emit(&mut run, RUN_END, data).await;
                Err(e)
            }
        }
    }

    /// The checks before a token is spent: the input names, the settings, the catalog and the run
    /// inputs, in that order.
    fn prepare(&self, input: &Value) -> Result<(LoopSettings, ToolCatalog), String> {
        check_input_names(input)?;
        let settings = LoopSettings::resolve(input, &self.defaults)?;
        let catalog =
            ToolCatalog::parse(get(input, TOOLS), &|route| self.gateway.reachable(route))?;
        check_run_inputs(input)?;
        Ok((settings, catalog))
    }

    async fn run_loop(&self, run: &mut Run<'_>, catalog: &ToolCatalog) -> Result<Value, AppError> {
        let mut results = None;
        if let Some(conversation) = text(get(run.input, CONVERSATION)) {
            run.conversation = Some(conversation.to_string());
            run.turn = int_of(get(run.input, TURN));
            let planned = self.plan_resume(run, catalog)?;
            if run.turn >= i64::from(run.settings.max_turns) {
                return Ok(cap(run, CAP_TURNS));
            }
            if self.time_left(run) < MIN_STEP_MS {
                return Ok(cap(run, CAP_DEADLINE));
            }
            results = Some(self.dispatch(run, &planned).await);
        }
        self.turns(run, catalog, results).await
    }

    async fn turns(
        &self,
        run: &mut Run<'_>,
        catalog: &ToolCatalog,
        answers: Option<Vec<Value>>,
    ) -> Result<Value, AppError> {
        let mut results = answers;
        loop {
            let budget = self.time_left(run);
            if budget < MIN_STEP_MS {
                return Ok(cap(run, CAP_DEADLINE));
            }
            let Some(reply) = self
                .model_turn(run, catalog, results.take(), budget)
                .await?
            else {
                return Ok(cap(run, CAP_DEADLINE));
            };
            if let Some(envelope) = outcome(run, &reply)? {
                return Ok(envelope);
            }
            if run.turn >= i64::from(run.settings.max_turns) {
                return Ok(cap(run, CAP_TURNS));
            }
            let planned = plan_turn(run, catalog, get(&reply, CALLS))?;
            if planned.iter().any(|p| p.outcome == Outcome::Wait) {
                return Ok(self.approval(run, &planned).await);
            }
            if self.time_left(run) < MIN_STEP_MS {
                return Ok(cap(run, CAP_DEADLINE));
            }
            results = Some(self.dispatch(run, &planned).await);
        }
    }

    /// One model turn's reply, or `None` when the turn timed out: the loop's deadline, named as a cap.
    async fn model_turn(
        &self,
        run: &mut Run<'_>,
        catalog: &ToolCatalog,
        results: Option<Vec<Value>>,
        budget: i64,
    ) -> Result<Option<Value>, AppError> {
        let request = chat_request(run, catalog, results, budget);
        let next = map_of(vec![(TURN, Value::from(run.turn + 1))]);
        self.emit(run, TURN_START, next).await;
        let response = self
            .gateway
            .chat(request, budget + RESERVE_MS / 2)
            .await
            .unwrap_or_else(|e| failure(&e));
        if response.status() == 408 {
            return Ok(None);
        }
        if response.has_error() {
            return Err(AppError::new(
                response.status(),
                format!("{LLM_CHAT_FAILED}{}", error_text(&response)),
            ));
        }
        let reply = response.body().clone();
        if !matches!(reply, Value::Map(_)) {
            return Err(AppError::new(
                502,
                format!("{OUTSIDE_CONTRACT}the reply is not a map"),
            ));
        }
        note_turn(run, &reply)?;
        let stop_reason = match get(&reply, STOP_REASON) {
            Some(reason @ Value::String(_)) => reason.clone(),
            _ => copy(&reply, STATUS),
        };
        let end = map_of(vec![
            (TURN, Value::from(run.turn)),
            (STOP_REASON, stop_reason),
            (USAGE, turn_usage(&reply)),
        ]);
        self.emit(run, TURN_END, end).await;
        Ok(Some(reply))
    }

    async fn approval(&self, run: &mut Run<'_>, planned: &[Planned]) -> Value {
        let mut calls = Vec::new();
        for p in planned {
            let mut call = Vec::new();
            put(&mut call, ID, Value::from(p.id.as_str()));
            put(&mut call, NAME, Value::from(p.name.as_str()));
            if let Some(route) = p.route() {
                put(&mut call, ROUTE, Value::from(route));
            }
            put(&mut call, INPUT, p.input.clone());
            put(&mut call, WAIT, Value::Boolean(p.outcome == Outcome::Wait));
            if p.asks() {
                put(&mut call, ASK, Value::Boolean(true));
            }
            calls.push(Value::Map(call));
        }
        let waiting: Vec<Value> = planned
            .iter()
            .filter(|p| p.outcome == Outcome::Wait)
            .map(|p| Value::Map(call_names(p)))
            .collect();
        self.emit(
            run,
            APPROVAL_WAIT,
            map_of(vec![(CALLS, Value::Array(waiting))]),
        )
        .await;
        let block = map_of(vec![(CALLS, Value::Array(calls))]);
        envelope(run, APPROVAL, vec![(APPROVAL, block)])
    }

    /// Section 7.1: the calls come from the restored approval block, never from the resume request,
    /// so the call that runs is the call that was shown. Every waiting call needs one decision;
    /// anything else fails.
    fn plan_resume(&self, run: &Run<'_>, catalog: &ToolCatalog) -> Result<Vec<Planned>, AppError> {
        let shown: &[Value] = match get(run.input, APPROVAL).and_then(|a| get(a, CALLS)) {
            Some(Value::Array(list)) => list,
            _ => &[],
        };
        let mut waiting: Vec<(String, &Value)> = Vec::new();
        let mut ids = HashSet::new();
        for (i, item) in shown.iter().enumerate() {
            let id = shown_call(item, i)?;
            if !ids.insert(id.clone()) {
                return Err(shape_error(i));
            }
            if matches!(get(item, WAIT), Some(Value::Boolean(true))) {
                waiting.push((id, item));
            }
        }
        let decisions = decisions(run, &waiting)?;
        shown
            .iter()
            .enumerate()
            .map(|(i, call)| resume_call(run, catalog, i, call, &decisions))
            .collect()
    }

    /// Run the turn's calls concurrently, each bounded by the time left, and answer every call in call
    /// order.
    async fn dispatch(&self, run: &mut Run<'_>, planned: &[Planned]) -> Vec<Value> {
        let budget = self.time_left(run);
        let started = (self.clock)();
        let gateway = self.gateway;
        let clock = &self.clock;
        // every request is issued from this task before any is awaited: one stream, polled together
        let mut pending = FuturesOrdered::new();
        for p in planned.iter().filter(|p| p.outcome == Outcome::Run) {
            self.emit(run, TOOL_START, Value::Map(tool_data(run, p)))
                .await;
            let route = p.route().unwrap_or_default();
            let input = p.input.clone();
            pending.push_back(async move {
                let response = gateway
                    .call(route, input, budget)
                    .await
                    .unwrap_or_else(|e| failure(&e));
                (response, clock())
            });
        }
        let mut results = Vec::new();
        for p in planned {
            let mut result = vec![(Value::from(ID), Value::from(p.id.as_str()))];
            let (label, ended) = if p.outcome == Outcome::Run {
                // in call order, so a watcher sees a turn's results the way the model gets them
                let (response, ended) = pending
                    .next()
                    .await
                    .unwrap_or_else(|| (failure(&AppError::new(500, "no response")), started));
                (ran(&run.settings, &response, &mut result), Some(ended))
            } else {
                (answered(&run.settings, p, &mut result), None)
            };
            results.push(Value::Map(result));
            let mut entry = call_names(p);
            put(&mut entry, STATUS, Value::from(label));
            run.calls.push(Value::Map(entry));
            if let Some(ended) = ended {
                let mut end = tool_data(run, p);
                put(&mut end, STATUS, Value::from(label));
                put(&mut end, MS, Value::from((ended - started).max(0)));
                self.emit(run, TOOL_END, Value::Map(end)).await;
            }
        }
        results
    }

    /// One progress event: names, routes, counts and statuses only, numbered from 1 for this call.
    async fn emit(&self, run: &mut Run<'_>, event: &str, data: Value) {
        run.seq += 1;
        let progress = map_of(vec![
            (TYPE, Value::from(PROGRESS)),
            (EVENT, Value::from(event)),
            (SEQ, Value::from(run.seq)),
            (TIME, Value::from(iso_time((self.clock)()))),
            (DATA, data),
        ]);
        self.gateway.progress(progress).await;
    }

    fn time_left(&self, run: &Run<'_>) -> i64 {
        run.deadline - (self.clock)() - RESERVE_MS
    }
}

fn ended(run: &Run<'_>, status: &str) -> Value {
    map_of(vec![
        (STATUS, Value::from(status)),
        (TURNS, Value::from(run.turns)),
        (USAGE, run.usage()),
    ])
}

/// The envelope for a model outcome, or `None` for a tool_use turn.
fn outcome(run: &Run<'_>, reply: &Value) -> Result<Option<Value>, AppError> {
    let status = get(reply, STATUS);
    match text(status) {
        Some(DONE) => Ok(Some(done(run, reply))),
        Some(REFUSAL) => Ok(Some(envelope(
            run,
            REFUSAL,
            vec![(REFUSAL, copy(reply, REFUSAL))],
        ))),
        Some(CAP) => Ok(Some(envelope(run, CAP, vec![(CAP, copy(reply, CAP))]))),
        Some(TOOL_USE) => Ok(None),
        _ => Err(AppError::new(
            502,
            format!("{OUTSIDE_CONTRACT}status '{}'", value_of(status)),
        )),
    }
}

// ------------------------------------------------------------------ inputs

fn check_input_names(input: &Value) -> Result<(), String> {
    let mut unknown: Vec<String> = entries(input)
        .iter()
        .map(|(key, _)| key_text(key))
        .filter(|key| !INPUTS.contains(&key.as_str()))
        .collect();
    unknown.sort_by(|a, b| java_cmp(a, b));
    unknown.dedup();
    if unknown.is_empty() {
        return Ok(());
    }
    Err(format!(
        "not an input of llm.run: {} - the inputs are {}",
        unknown.join(", "),
        and_list(&INPUTS)
    ))
}

fn check_run_inputs(input: &Value) -> Result<(), String> {
    if get(input, SYSTEM).is_some_and(|v| !matches!(v, Value::String(_))) {
        return Err("system must be text".to_string());
    }
    if get(input, SCHEMA).is_some_and(|v| !matches!(v, Value::Map(_))) {
        return Err("schema must be a JSON schema object".to_string());
    }
    if get(input, MODEL).is_some_and(|v| !v.as_str().is_some_and(|m| !java_is_blank(m))) {
        return Err("model must be a model id".to_string());
    }
    let params = get(input, PARAMS);
    if params.is_some_and(|v| !matches!(v, Value::Map(_))) {
        return Err("params must be a map".to_string());
    }
    if let Some(params) = params {
        if contains_key(params, MODEL) {
            return Err("params.model is not set here - use the model input".to_string());
        }
        if contains_key(params, TIMEOUT_MS) {
            return Err(
                "params.timeout_ms is set by the loop from its deadline - use the timeout input"
                    .to_string(),
            );
        }
    }
    if get(input, CONVERSATION).is_none() {
        check_first_turn(input)
    } else {
        check_resume(input)
    }
}

fn check_first_turn(input: &Value) -> Result<(), String> {
    for key in [TURN, APPROVAL, DECISIONS] {
        if get(input, key).is_some() {
            return Err(format!(
                "{key} needs conversation - a run continues only from an approval"
            ));
        }
    }
    let has_prompt = text(get(input, PROMPT)).is_some_and(|p| !java_is_blank(p));
    let has_messages = matches!(get(input, MESSAGES), Some(Value::Array(list)) if !list.is_empty());
    if !has_prompt && !has_messages {
        return Err("prompt or messages is required on a run's first turn".to_string());
    }
    Ok(())
}

fn check_resume(input: &Value) -> Result<(), String> {
    if !text(get(input, CONVERSATION)).is_some_and(is_reference) {
        return Err("conversation must be the 32-hex reference a run returned".to_string());
    }
    if int_of(get(input, TURN)) < 1 {
        return Err("turn must be the turn count the approval returned".to_string());
    }
    let calls = get(input, APPROVAL)
        .filter(|a| matches!(a, Value::Map(_)))
        .and_then(|a| get(a, CALLS));
    if !matches!(calls, Some(Value::Array(list)) if !list.is_empty()) {
        return Err(
            "approval must be the approval block the run returned, {calls: [...]} - \
                    a run continues only from an approval"
                .to_string(),
        );
    }
    if !matches!(get(input, DECISIONS), Some(Value::Array(_))) {
        return Err(
            "decisions must be a list of {id, approved, note}, one per waiting call".to_string(),
        );
    }
    Ok(())
}

/// `[0-9a-f]{32}`, matched whole.
fn is_reference(text: &str) -> bool {
    text.len() == 32 && text.bytes().all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f'))
}

/// A whole number from a number, or from text of one to nine ASCII digits; -1 otherwise.
fn int_of(value: Option<&Value>) -> i64 {
    match value {
        Some(Value::String(s)) => {
            let text = java_trim(s.as_str().unwrap_or_default());
            if (1..=9).contains(&text.len()) && text.bytes().all(|c| c.is_ascii_digit()) {
                text.parse().unwrap_or(-1)
            } else {
                -1
            }
        }
        Some(number) => Decimal::of(number)
            .and_then(|n| n.whole())
            .filter(|n| n.abs() <= i64::from(i32::MAX))
            .unwrap_or(-1),
        None => -1,
    }
}

/// `Number.longValue()`: an integer as it is, a float truncated toward zero.
fn long_value(value: &Value) -> Option<i64> {
    match value {
        Value::Integer(n) => n.as_i64().or_else(|| n.as_u64().map(|v| v as i64)),
        Value::F64(f) => Some(*f as i64),
        Value::F32(f) => Some(*f as i64),
        _ => None,
    }
}

// ------------------------------------------------------------------ the model turn

fn chat_request(
    run: &Run<'_>,
    catalog: &ToolCatalog,
    results: Option<Vec<Value>>,
    budget: i64,
) -> Value {
    let input = run.input;
    let mut request = Vec::new();
    if let Some(system) = get(input, SYSTEM) {
        put(&mut request, SYSTEM, system.clone());
    }
    if run.conversation.is_none() {
        if let Some(prompt) = get(input, PROMPT) {
            put(&mut request, PROMPT, prompt.clone());
        }
        if let Some(messages) = get(input, MESSAGES) {
            put(&mut request, MESSAGES, messages.clone());
        }
    }
    if let Some(schema) = get(input, SCHEMA) {
        put(&mut request, SCHEMA, schema.clone());
    }
    put(&mut request, TOOLS, Value::Array(catalog.advertised()));
    let mut params = Vec::new();
    if let Some(given @ Value::Map(_)) = get(input, PARAMS) {
        for (key, value) in entries(given) {
            put(&mut params, &key_text(key), value.clone());
        }
    }
    if let Some(model) = get(input, MODEL) {
        put(&mut params, MODEL, model.clone());
    }
    put(&mut params, TIMEOUT_MS, Value::from(budget));
    put(&mut request, PARAMS, Value::Map(params));
    if let Some(conversation) = &run.conversation {
        put(
            &mut request,
            CONVERSATION,
            Value::from(conversation.as_str()),
        );
        put(&mut request, TURN, Value::from(run.turn));
        put(
            &mut request,
            RESULTS,
            results.map(Value::Array).unwrap_or(Value::Nil),
        );
    }
    put(
        &mut request,
        TTL,
        Value::from(run.settings.transcript_ttl.as_str()),
    );
    put(
        &mut request,
        RETENTION,
        Value::from(run.settings.retention.as_str()),
    );
    Value::Map(request)
}

fn turn_usage(reply: &Value) -> Value {
    let mut usage = Vec::new();
    if let Some(given @ Value::Map(_)) = get(reply, USAGE) {
        for key in USAGE_KEYS {
            let n = get(given, key).and_then(long_value).unwrap_or(0);
            put(&mut usage, key, Value::from(n));
        }
    }
    Value::Map(usage)
}

fn note_turn(run: &mut Run<'_>, reply: &Value) -> Result<(), AppError> {
    let Some(reference) = text(get(reply, CONVERSATION)).filter(|r| !r.is_empty()) else {
        return Err(AppError::new(
            502,
            format!("{OUTSIDE_CONTRACT}no conversation"),
        ));
    };
    let turn = int_of(get(reply, TURN));
    if turn < 1 {
        return Err(AppError::new(
            502,
            format!("{OUTSIDE_CONTRACT}no turn count"),
        ));
    }
    run.conversation = Some(reference.to_string());
    run.turn = turn;
    run.turns += 1;
    if let Some(model) = text(get(reply, MODEL)) {
        run.model = Some(model.to_string());
    }
    if let Some(usage @ Value::Map(_)) = get(reply, USAGE) {
        for (i, key) in USAGE_KEYS.iter().enumerate() {
            if let Some(n) = get(usage, key).and_then(long_value) {
                run.usage[i] = run.usage[i].wrapping_add(n);
            }
        }
    }
    Ok(())
}

fn done(run: &Run<'_>, reply: &Value) -> Value {
    let mut fields = Vec::new();
    if contains_key(reply, DATA) {
        let data = copy(reply, DATA);
        // the business decision of section 7.2, lifted when the node's output schema declares it
        let declared = get(run.input, SCHEMA)
            .filter(|schema| matches!(schema, Value::Map(_)))
            .and_then(|schema| get(schema, "properties"))
            .is_some_and(|props| matches!(props, Value::Map(_)) && contains_key(props, DECISION));
        let lifted = declared && matches!(data, Value::Map(_)) && contains_key(&data, DECISION);
        let decision = lifted.then(|| copy(&data, DECISION));
        fields.push((DATA, data));
        if let Some(decision) = decision {
            fields.push((DECISION, decision));
        }
    } else {
        fields.push((TEXT, copy(reply, TEXT)));
    }
    envelope(run, DONE, fields)
}

// ------------------------------------------------------------------ the calls of a turn

fn plan_turn(
    run: &Run<'_>,
    catalog: &ToolCatalog,
    raw: Option<&Value>,
) -> Result<Vec<Planned>, AppError> {
    let calls = match raw {
        Some(Value::Array(calls)) if !calls.is_empty() => calls,
        _ => {
            return Err(AppError::new(
                502,
                format!("{OUTSIDE_CONTRACT}tool_use without calls"),
            ))
        }
    };
    let mut planned = Vec::new();
    let mut ids = HashSet::new();
    for item in calls {
        let id = match text(get(item, ID)) {
            Some(id) if !id.is_empty() && ids.insert(id.to_string()) => id,
            _ => {
                return Err(AppError::new(
                    502,
                    format!("{OUTSIDE_CONTRACT}a call without a unique id"),
                ))
            }
        };
        let name = value_of(get(item, NAME));
        planned.push(evaluate(run, catalog, id, &name, copy(item, INPUT)));
    }
    Ok(planned)
}

/// The steps of section 4 for one call: resolve the name, validate the input, check approval.
fn evaluate(run: &Run<'_>, catalog: &ToolCatalog, id: &str, name: &str, input: Value) -> Planned {
    let Some(row) = catalog.get(name) else {
        return Planned::new(id, name, None, input, Outcome::Error)
            .answered(UNKNOWN, Value::from(format!("unknown tool '{name}'")));
    };
    let reference = Some(RowRef {
        route: row.route.clone(),
        ask: row.ask,
    });
    let mut violations = Vec::new();
    row.schema.validate(INPUT, &input, &mut violations);
    if !violations.is_empty() {
        let message = format!(
            "invalid input for '{name}': {}",
            ToolSchema::report(&violations)
        );
        return Planned::new(id, name, reference, input, Outcome::Error)
            .answered(INVALID, Value::from(message));
    }
    let mode = run.settings.mode.as_str();
    if row.ask {
        // a question for a person waits in every mode that has a person to ask
        return if mode == UNATTENDED {
            Planned::new(id, name, reference, input, Outcome::Error).answered(
                REFUSED,
                Value::from(format!(
                    "'{name}' asks a person, and this run has nobody to ask"
                )),
            )
        } else {
            Planned::new(id, name, reference, input, Outcome::Wait)
        };
    }
    if row.write && mode == UNATTENDED {
        return Planned::new(id, name, reference, input, Outcome::Error).answered(
            REFUSED,
            Value::from(format!("'{name}' is a write, and this run is unattended")),
        );
    }
    if mode == MANUAL || (row.write && mode == WRITE) {
        return Planned::new(id, name, reference, input, Outcome::Wait);
    }
    Planned::new(id, name, reference, input, Outcome::Run)
}

/// A shown call keeps its shape: `{id, name, route, input, wait}` with the ask mark optional. Answers
/// the call's id.
fn shown_call(item: &Value, i: usize) -> Result<String, AppError> {
    let id = text(get(item, ID));
    let shaped = matches!(item, Value::Map(_))
        && id.is_some()
        && matches!(get(item, WAIT), Some(Value::Boolean(_)))
        && text(get(item, NAME)).is_some()
        && matches!(get(item, ASK), None | Some(Value::Boolean(_)));
    match id {
        Some(id) if shaped => Ok(id.to_string()),
        _ => Err(shape_error(i)),
    }
}

fn shape_error(i: usize) -> AppError {
    AppError::new(
        400,
        format!(
            "approval.calls[{i}] must be a call the run returned: {{id, name, route, input, wait}}, \
             its id unique"
        ),
    )
}

fn mismatch(i: usize, id: &str) -> AppError {
    AppError::new(
        400,
        format!("approval.calls[{i}] ('{id}{NO_LONGER_MATCHES}"),
    )
}

/// A shown call as it runs on resume: checked against the catalog and the mode, then decided.
fn resume_call(
    run: &Run<'_>,
    catalog: &ToolCatalog,
    i: usize,
    call: &Value,
    decisions: &HashMap<String, &Value>,
) -> Result<Planned, AppError> {
    let id = text(get(call, ID)).unwrap_or_default();
    let name = text(get(call, NAME)).unwrap_or_default();
    let row = catalog.get(name);
    let route = get(call, ROUTE);
    let shown_ask = matches!(get(call, ASK), Some(Value::Boolean(true)));
    let same_row = match row {
        None => route.is_none() && !shown_ask,
        Some(row) => same_route(row.route.as_deref(), route) && row.ask == shown_ask,
    };
    let fresh = evaluate(run, catalog, id, name, copy(call, INPUT));
    let wait = matches!(get(call, WAIT), Some(Value::Boolean(true)));
    if !same_row || wait != (fresh.outcome == Outcome::Wait) {
        return Err(mismatch(i, id));
    }
    if !wait {
        return Ok(fresh);
    }
    let decision = decisions.get(id).copied().unwrap_or(&Value::Nil);
    let input = copy(call, INPUT);
    if fresh.asks() {
        return Ok(
            Planned::new(id, name, fresh.row.clone(), input, Outcome::Answer)
                .answered(ANSWERED, copy(decision, ANSWER)),
        );
    }
    if matches!(get(decision, APPROVED), Some(Value::Boolean(true))) {
        return Ok(Planned::new(
            id,
            name,
            fresh.row.clone(),
            input,
            Outcome::Run,
        ));
    }
    let note = match text(get(decision, NOTE)) {
        Some(note) if !java_is_blank(note) => format!(": {note}"),
        _ => String::new(),
    };
    Ok(
        Planned::new(id, name, fresh.row.clone(), input, Outcome::Error).answered(
            REFUSED,
            Value::from(format!("refused by the reviewer{note}")),
        ),
    )
}

/// Java `Objects.equals(row.route(), route)`: both absent, or the same text.
fn same_route(row_route: Option<&str>, shown: Option<&Value>) -> bool {
    match (row_route, shown) {
        (None, None) => true,
        (Some(route), Some(shown)) => shown.as_str() == Some(route),
        _ => false,
    }
}

fn decisions<'v>(
    run: &Run<'v>,
    waiting: &[(String, &Value)],
) -> Result<HashMap<String, &'v Value>, AppError> {
    let given: &[Value] = match get(run.input, DECISIONS) {
        Some(Value::Array(list)) => list,
        _ => &[],
    };
    let mut by_id: HashMap<String, &'v Value> = HashMap::new();
    for (i, decision) in given.iter().enumerate() {
        let place = format!("decisions[{i}]");
        let Some(id) = text(get(decision, ID)).filter(|_| matches!(decision, Value::Map(_))) else {
            return Err(AppError::new(400, format!("{place}{DECISION_SHAPE}")));
        };
        let Some((_, call)) = waiting.iter().find(|(waiting_id, _)| waiting_id == id) else {
            return Err(AppError::new(
                400,
                format!("{place}.id '{id}' is not a call waiting for approval"),
            ));
        };
        if matches!(get(call, ASK), Some(Value::Boolean(true))) {
            check_answer(&place, id, decision)?;
        } else {
            check_approval(&place, decision)?;
        }
        if by_id.insert(id.to_string(), decision).is_some() {
            return Err(AppError::new(
                400,
                format!("{place}.id '{id}' is decided twice"),
            ));
        }
    }
    for (id, _) in waiting {
        if !by_id.contains_key(id) {
            return Err(AppError::new(
                400,
                format!("no decision for the waiting call '{id}'"),
            ));
        }
    }
    Ok(by_id)
}

/// A question's decision is exactly `{id, answer}`: the person's answer is the call's result.
fn check_answer(place: &str, id: &str, decision: &Value) -> Result<(), AppError> {
    if get(decision, ANSWER).is_none()
        || contains_key(decision, APPROVED)
        || contains_key(decision, NOTE)
    {
        return Err(AppError::new(
            400,
            format!("{place} answers the question in call '{id}': {{id, answer}}"),
        ));
    }
    Ok(())
}

/// A tool call's decision is `{id, approved, note}`: approved true or false, the note optional text.
fn check_approval(place: &str, decision: &Value) -> Result<(), AppError> {
    if !matches!(get(decision, APPROVED), Some(Value::Boolean(_))) || contains_key(decision, ANSWER)
    {
        return Err(AppError::new(
            400,
            format!("{place} must be {{id, approved, note}} with approved true or false"),
        ));
    }
    if get(decision, NOTE).is_some_and(|note| !matches!(note, Value::String(_))) {
        return Err(AppError::new(400, format!("{place}.note must be text")));
    }
    Ok(())
}

/// Put a call's response into its result: its output, or its error with the status. The returned
/// label is the call's status.
fn ran(
    settings: &LoopSettings,
    response: &EventEnvelope,
    result: &mut Vec<(Value, Value)>,
) -> &'static str {
    if response.has_error() {
        let error = format!("error {}: {}", response.status(), error_text(response));
        put(result, ERROR, Value::from(error));
        return ERROR;
    }
    put(
        result,
        OUTPUT,
        capped(response.body(), settings.result_limit),
    );
    OK
}

/// Put a call that did not run into its result: a person's answer, or the error the call was planned
/// with. The returned label is the call's status.
fn answered(
    settings: &LoopSettings,
    p: &Planned,
    result: &mut Vec<(Value, Value)>,
) -> &'static str {
    if p.outcome == Outcome::Answer {
        put(result, OUTPUT, capped(&p.value, settings.result_limit));
    } else {
        put(result, ERROR, p.value.clone());
    }
    p.label
}

fn call_names(p: &Planned) -> Vec<(Value, Value)> {
    let mut names = Vec::new();
    put(&mut names, ID, Value::from(p.id.as_str()));
    put(&mut names, NAME, Value::from(p.name.as_str()));
    if let Some(route) = p.route() {
        put(&mut names, ROUTE, Value::from(route));
    }
    names
}

fn tool_data(run: &Run<'_>, p: &Planned) -> Vec<(Value, Value)> {
    let mut data = vec![(Value::from(TURN), Value::from(run.turn))];
    data.extend(call_names(p));
    data
}

/// A result over the limit is cut and marked as cut. Text is measured as it is; anything else as its
/// compact JSON text. Characters are code points; a nil result is empty text.
pub fn capped(body: &Value, limit: usize) -> Value {
    if body.is_nil() {
        return Value::from("");
    }
    let text = match body.as_str() {
        Some(text) => text.to_string(),
        None => compact(body),
    };
    let length = text.chars().count();
    if length <= limit {
        return body.clone();
    }
    let cut: String = text.chars().take(limit).collect();
    Value::from(format!(
        "{cut}\n[the result was cut at {limit} of {length} characters]"
    ))
}

/// An error response's message as text; a structured error as its compact JSON.
fn error_text(response: &EventEnvelope) -> String {
    match response.body().as_str() {
        Some(text) => text.to_string(),
        None => compact(response.body()),
    }
}

/// A request that failed without a response, as an error response: its status (408 for a timeout,
/// 404 for a route nothing serves) and its message.
pub fn failure(e: &AppError) -> EventEnvelope {
    EventEnvelope::new()
        .set_status(e.status())
        .set_raw_body(Value::from(e.message()))
}

// ------------------------------------------------------------------ the envelope

fn cap(run: &Run<'_>, reason: &str) -> Value {
    envelope(run, CAP, vec![(CAP, Value::from(reason))])
}

/// The outcome envelope of section 6, its fields in the order the spec lists them: the status, the
/// outcome fields, then the run's references and counts.
fn envelope(run: &Run<'_>, status: &str, fields: Vec<(&str, Value)>) -> Value {
    let mut map = vec![(Value::from(STATUS), Value::from(status))];
    for (key, value) in fields {
        put(&mut map, key, value);
    }
    if let Some(conversation) = &run.conversation {
        put(&mut map, CONVERSATION, Value::from(conversation.as_str()));
        put(&mut map, TURN, Value::from(run.turn));
    }
    put(&mut map, TURNS, Value::from(run.turns));
    put(&mut map, USAGE, run.usage());
    put(&mut map, CALLS, Value::Array(run.calls.clone()));
    if let Some(model) = &run.model {
        put(&mut map, MODEL, Value::from(model.as_str()));
    }
    Value::Map(map)
}

/// `yyyy-MM-dd'T'HH:mm:ss.SSS'Z'` in UTC, from milliseconds since the epoch.
fn iso_time(ms: i64) -> String {
    let seconds = ms.div_euclid(1000);
    let millis = ms.rem_euclid(1000);
    let (year, month, day) = civil_from_days(seconds.div_euclid(86_400));
    let of_day = seconds.rem_euclid(86_400);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{millis:03}Z",
        of_day / 3600,
        of_day % 3600 / 60,
        of_day % 60
    )
}

/// The proleptic Gregorian date of a day count from 1970-01-01 (H. Hinnant's `civil_from_days`).
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_time_is_iso_8601_utc_with_milliseconds() {
        assert_eq!(iso_time(0), "1970-01-01T00:00:00.000Z");
        assert_eq!(iso_time(3_000), "1970-01-01T00:00:03.000Z");
        assert_eq!(iso_time(1_700_000_000_123), "2023-11-14T22:13:20.123Z");
        assert_eq!(iso_time(951_782_400_000), "2000-02-29T00:00:00.000Z");
        assert_eq!(iso_time(-1), "1969-12-31T23:59:59.999Z");
    }

    #[test]
    fn whole_numbers_from_numbers_or_digits() {
        assert_eq!(int_of(Some(&Value::from(3))), 3);
        assert_eq!(int_of(Some(&Value::F64(2.0))), 2);
        assert_eq!(int_of(Some(&Value::F64(2.5))), -1);
        assert_eq!(int_of(Some(&Value::from(" 12 "))), 12);
        assert_eq!(int_of(Some(&Value::from("1234567890"))), -1);
        assert_eq!(int_of(Some(&Value::from(i64::from(i32::MAX) + 1))), -1);
        assert_eq!(int_of(Some(&Value::Boolean(true))), -1);
        assert_eq!(int_of(None), -1);
    }
}
