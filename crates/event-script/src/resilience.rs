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

//! The resilience handler — Rust port of
//! `com.accenture.services.Resilience4Flow` (`resilience.handler`, an event
//! interceptor). Used as a decision task in flows: given the failure context
//! it answers with `decision` 1 (retry / proceed), 2 (abort) or 3
//! (alternative path), plus attempt/cumulative/backoff bookkeeping the flow
//! maps back into its state (or the temp file system, as the resilience-demo
//! fixture shows). A retry after the first attempt is delayed via
//! `send_later` (the `delay` input, ms, floor 10).

use std::collections::HashMap;
use std::time::Duration;

use platform_core::{AppError, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

use crate::conversions::display;
use crate::mlm::MultiLevelMap;
use crate::util::{str2int, str2long};

pub const ROUTE: &str = "resilience.handler";

fn get_int(input: &MultiLevelMap, key: &str, default: i64) -> i64 {
    input
        .get_element(key)
        .map(|v| str2long(&display(&v)))
        .filter(|n| *n != -1 || default == -1)
        .unwrap_or(default)
}

/// Alternative-path routing: a comma list of status codes and ranges
/// (`401, 403-404`); only codes above 200 participate (Java parity).
fn need_reroute(codes: &str, status: i64) -> bool {
    for item in codes.split(',') {
        let s = item.trim();
        if let Some((a, b)) = s.split_once('-') {
            let n1 = str2int(a.trim()) as i64;
            let n2 = str2int(b.trim()) as i64;
            if n1 > 200 && n2 > 200 {
                let (low, high) = if n2 > n1 { (n1, n2) } else { (n2, n1) };
                if status >= low && status <= high {
                    return true;
                }
            }
        } else {
            let code = str2int(s) as i64;
            if code > 200 && code == status {
                return true;
            }
        }
    }
    false
}

/// The interceptor body: evaluates backoff, retry, abort or alternative
/// routing and replies manually with the decision map.
pub async fn handle(
    platform: &Platform,
    _headers: HashMap<String, String>,
    event: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let (Some(reply_to), Some(cid)) = (event.reply_to(), event.correlation_id()) else {
        return EventEnvelope::new().set_body("ignored");
    };
    if !matches!(event.body(), Value::Map(_)) {
        return EventEnvelope::new().set_body("ignored");
    }
    let input = MultiLevelMap::from_value(event.body().clone());
    let mut cumulative = get_int(&input, "cumulative", 0).max(0);
    let mut result = MultiLevelMap::new();
    let now_ms = chrono::Utc::now().timestamp_millis();
    // still inside the backoff period?
    if input.key_exists("backoff") {
        let last_backoff = get_int(&input, "backoff", 0);
        if now_ms < last_backoff {
            let diff = ((last_backoff - now_ms) / 1000).max(1);
            let plural = if diff == 1 { "second" } else { "seconds" };
            set(&mut result, "decision", Value::from(2))?;
            set(&mut result, "status", Value::from(503))?;
            set(
                &mut result,
                "message",
                Value::from(format!(
                    "Service temporarily not available - please try again in {diff} {plural}"
                )),
            )?;
            set(&mut result, "backoff", Value::from(last_backoff))?;
            return send_result(platform, reply_to, cid, result, 0).await;
        }
        // backoff period ended — reset the cumulative failure counter
        cumulative = 0;
    }
    let status = get_int(&input, "status", 200).max(200);
    // status 200 = gatekeeper mode: proceed to the user function immediately
    if status == 200 {
        set(&mut result, "decision", Value::from(1))?;
        set(&mut result, "cumulative", Value::from(cumulative))?;
        return send_result(platform, reply_to, cid, result, 0).await;
    }
    // trigger a new backoff period?
    if input.key_exists("backoff_trigger") && input.key_exists("backoff_seconds") {
        cumulative += 1;
        let trigger = get_int(&input, "backoff_trigger", 1).max(1);
        let seconds = get_int(&input, "backoff_seconds", 1).max(1);
        if cumulative > trigger {
            let plural = if seconds == 1 { "second" } else { "seconds" };
            set(&mut result, "decision", Value::from(2))?;
            set(&mut result, "status", Value::from(503))?;
            set(
                &mut result,
                "message",
                Value::from(format!(
                    "Service temporarily not available - please try again in {seconds} {plural}"
                )),
            )?;
            set(&mut result, "backoff", Value::from(now_ms + seconds * 1000))?;
            return send_result(platform, reply_to, cid, result, 0).await;
        }
    }
    let max_attempts = get_int(&input, "max_attempts", 1).max(1);
    let attempt = get_int(&input, "attempt", 0).max(0) + 1;
    let mut delay = get_int(&input, "delay", 10).max(10);
    set(&mut result, "attempt", Value::from(attempt))?;
    set(&mut result, "cumulative", Value::from(cumulative))?;
    if attempt > max_attempts {
        delay = 0;
        let message = input
            .get_element("message")
            .map(|v| display(&v))
            .unwrap_or_else(|| "Runtime exception".to_string());
        // abort by executing the second task in the decision list
        set(&mut result, "decision", Value::from(2))?;
        set(&mut result, "status", Value::from(status))?;
        set(&mut result, "message", Value::from(message))?;
    } else {
        if attempt == 1 {
            delay = 0;
        }
        let alternative = input.get_element("alternative").map(|v| display(&v));
        if alternative
            .map(|codes| need_reroute(&codes, status))
            .unwrap_or(false)
        {
            // execute the alternative path (third task)
            set(&mut result, "decision", Value::from(3))?;
        } else {
            // retry the original task
            set(&mut result, "decision", Value::from(1))?;
        }
    }
    send_result(platform, reply_to, cid, result, delay).await
}

fn set(result: &mut MultiLevelMap, key: &str, value: Value) -> Result<(), AppError> {
    result
        .set_element(key, value)
        .map_err(|e| AppError::new(500, e))
}

async fn send_result(
    platform: &Platform,
    reply_to: &str,
    cid: &str,
    result: MultiLevelMap,
    delay_ms: i64,
) -> Result<EventEnvelope, AppError> {
    let po = PostOffice::new(platform);
    let response = EventEnvelope::new()
        .set_to(reply_to)
        .set_correlation_id(cid)
        .set_raw_body(result.to_value());
    if delay_ms > 0 {
        po.send_later(response, Duration::from_millis(delay_ms as u64));
    } else {
        po.send(response).await?;
    }
    EventEnvelope::new().set_body("ok")
}
