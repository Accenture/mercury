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

//! End-to-end drive of the tutorial-14 purchase workflow — the Rust twin of
//! the Java example app's `SuspendResumeTutorialTest`: three human
//! checkpoints (order, approval, delivery release) expressed as four short
//! graph runs sharing one X-Correlation-Id, each run resuming past the
//! previous checkpoint through the REAL Redis client and the
//! `v1.redis.persist.model` / `v1.redis.retrieve.model` store functions,
//! with the final response carrying the state captured across all of them.
//!
//! The Java suite embeds a real redis-server binary; here the server side is the
//! in-process RESP2 test double of `tests/common/redis_double.rs` (the approach of the
//! `minigraph-state-redis` crate's contract tests) — the client wire path (TCP, RESP2
//! frames, SETEX/GETDEL) stays fully real.

use std::time::Duration;

use platform_core::automation::AsyncHttpRequest;
use platform_core::{automation, overrides, AutoStart, EventEnvelope, Platform, PostOffice};
use rmpv::Value;

// the RESP2 double shared with the agent graphs' test
#[path = "common/redis_double.rs"]
mod redis_double;
use redis_double::start_resp_double;

// The application under test is a BIN crate — include its source so the
// link-time inventory in this test binary carries the engine skills AND the
// Redis state-store functions the app links.
#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

const TIMEOUT: Duration = Duration::from_secs(8);

// ---- helpers ----

fn get_element<'a>(body: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = body;
    for key in path {
        let Value::Map(entries) = current else {
            return None;
        };
        current = &entries.iter().find(|(k, _)| k.as_str() == Some(*key))?.1;
    }
    Some(current)
}

fn text_of(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(text)) => text.as_str().unwrap_or_default().to_string(),
        Some(other) => format!("{other}"),
        None => String::new(),
    }
}

async fn run_graph(po: &PostOffice, target: &str, cid: &str, body: Value) -> EventEnvelope {
    let request = AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(target)
        .set_url("/api/graph/tutorial-14")
        .set_header("content-type", "application/json")
        .set_header("accept", "application/json")
        .set_header("x-correlation-id", cid)
        .set_body(body);
    po.request(
        EventEnvelope::new()
            .set_to("async.http.request")
            .set_raw_body(request.to_value()),
        TIMEOUT,
    )
    .await
    .expect("graph run reply")
}

fn stage(response: &EventEnvelope, cid: &str) -> Value {
    assert_eq!(200, response.status(), "run reply: {:?}", response.body());
    assert_eq!(
        cid,
        text_of(get_element(response.body(), &["cid"])),
        "every reply carries the business correlation id"
    );
    response.body().clone()
}

fn json_map(entries: &[(&str, Value)]) -> Value {
    Value::Map(
        entries
            .iter()
            .map(|(k, v)| (Value::from(*k), v.clone()))
            .collect(),
    )
}

// One test function on purpose (the repo convention): the app boots ONCE per
// process, so all scenarios run sequentially against the same server.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn tutorial_14_purchase_workflow() {
    let redis_port = start_resp_double().await;
    // the state-store connection is lazy and reads configuration at first
    // use, so an override set before the first suspension is honored
    overrides::set("redis.host", "127.0.0.1");
    overrides::set("redis.port", &redis_port.to_string());
    overrides::set("rest.server.port", "0");
    let holding = test_support::temp_path("tutorial-14");
    overrides::set("transient.data.store", &holding.display().to_string());
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let port = automation::server_address().expect("server started").port();
    let target = format!("http://127.0.0.1:{port}");
    let po = PostOffice::new(&Platform::get_instance());

    // --- the purchase workflow: three checkpoints, four runs, ONE cid
    let cid = uuid::Uuid::new_v4().simple().to_string();
    // run 1: the customer orders a laptop - suspend for the store manager
    let ordered = stage(
        &run_graph(
            &po,
            &target,
            &cid,
            json_map(&[
                ("item", Value::from("laptop")),
                ("amount", Value::from(2000)),
            ]),
        )
        .await,
        &cid,
    );
    assert!(
        text_of(get_element(&ordered, &["stage"])).starts_with("order-submitted"),
        "run 1: {ordered:?}"
    );
    assert_eq!(
        "fresh",
        text_of(get_element(&ordered, &["run"])),
        "run 1 is a fresh transaction"
    );
    // run 2: the store manager approves - suspend for the delivery department
    let approved = stage(
        &run_graph(
            &po,
            &target,
            &cid,
            json_map(&[
                ("decision", Value::from("approved")),
                ("manager", Value::from("store-88")),
            ]),
        )
        .await,
        &cid,
    );
    assert!(
        text_of(get_element(&approved, &["stage"])).starts_with("approved"),
        "run 2: {approved:?}"
    );
    // run 3: the delivery department releases the shipment
    let released = stage(
        &run_graph(
            &po,
            &target,
            &cid,
            json_map(&[
                ("release", Value::from(true)),
                ("courier", Value::from("express")),
            ]),
        )
        .await,
        &cid,
    );
    assert!(
        text_of(get_element(&released, &["stage"])).starts_with("released"),
        "run 3: {released:?}"
    );
    // run 4: shipment confirmation - the workflow completes with full history
    let shipped = stage(
        &run_graph(
            &po,
            &target,
            &cid,
            json_map(&[("tracking", Value::from("TRK-12345"))]),
        )
        .await,
        &cid,
    );
    assert_eq!("shipped", text_of(get_element(&shipped, &["stage"])));
    assert_eq!(
        "resume",
        text_of(get_element(&shipped, &["run"])),
        "runs 2-4 are resumed continuations"
    );
    // state captured across all four runs survived every suspension
    assert_eq!("laptop", text_of(get_element(&shipped, &["order", "item"])));
    assert_eq!(
        Some(&Value::from(2000)),
        get_element(&shipped, &["order", "amount"])
    );
    assert_eq!(
        "approved",
        text_of(get_element(&shipped, &["approval", "decision"]))
    );
    assert_eq!(
        "store-88",
        text_of(get_element(&shipped, &["approval", "manager"]))
    );
    assert_eq!(
        Some(&Value::from(true)),
        get_element(&shipped, &["delivery", "release"])
    );
    assert_eq!(
        "express",
        text_of(get_element(&shipped, &["delivery", "courier"]))
    );
    assert_eq!(
        "TRK-12345",
        text_of(get_element(&shipped, &["shipment", "tracking"]))
    );
    log::info!("tutorial-14 purchase workflow shipped for cid {cid}");

    // --- a transaction that never suspended simply flows through the resume
    // node (fresh correlation id runs from the start)
    let fresh_cid = uuid::Uuid::new_v4().simple().to_string();
    let fresh = run_graph(
        &po,
        &target,
        &fresh_cid,
        json_map(&[("item", Value::from("mouse"))]),
    )
    .await;
    assert_eq!(200, fresh.status());
    assert!(
        text_of(get_element(fresh.body(), &["stage"])).starts_with("order-submitted"),
        "fresh run: {:?}",
        fresh.body()
    );

    // --- input validation: an approval decision for a transaction that was
    // never submitted (or has expired) must be rejected - submission first
    let unknown_cid = uuid::Uuid::new_v4().simple().to_string();
    let rejected = run_graph(
        &po,
        &target,
        &unknown_cid,
        json_map(&[("decision", Value::from("approved"))]),
    )
    .await;
    assert_eq!(404, rejected.status(), "gate: {:?}", rejected.body());
    assert_eq!("rejected", text_of(get_element(rejected.body(), &["type"])));
    assert_eq!(
        "fresh",
        text_of(get_element(rejected.body(), &["run"])),
        "the rejection advises the UI of the fresh condition"
    );
    assert!(
        text_of(get_element(rejected.body(), &["message"])).contains("Submit the order"),
        "unexpected rejection message: {:?}",
        rejected.body()
    );

    // --- the manager rejects instead: the check-approval decision routes to
    // the terminal rejection, which reports the manager's reason with the
    // original order, and the workflow ends - no further checkpoints
    let reject_cid = uuid::Uuid::new_v4().simple().to_string();
    let ordered = stage(
        &run_graph(
            &po,
            &target,
            &reject_cid,
            json_map(&[
                ("item", Value::from("monitor")),
                ("amount", Value::from(300)),
            ]),
        )
        .await,
        &reject_cid,
    );
    assert!(
        text_of(get_element(&ordered, &["stage"])).starts_with("order-submitted"),
        "reject-path run 1: {ordered:?}"
    );
    let declined = stage(
        &run_graph(
            &po,
            &target,
            &reject_cid,
            json_map(&[
                ("decision", Value::from("rejected")),
                ("reason", Value::from("budget exceeded")),
            ]),
        )
        .await,
        &reject_cid,
    );
    assert_eq!("rejected", text_of(get_element(&declined, &["stage"])));
    assert_eq!(
        "budget exceeded",
        text_of(get_element(&declined, &["reason"])),
        "the manager's reason is reported"
    );
    assert_eq!(
        "resume",
        text_of(get_element(&declined, &["run"])),
        "the rejection run is a resumed continuation"
    );
    assert_eq!(
        "monitor",
        text_of(get_element(&declined, &["order", "item"])),
        "the reply echoes the original order"
    );
    // the record was consumed on resume and nothing re-suspended: the workflow
    // is over, so a further request under the same correlation id is rejected
    // as a fresh transaction
    let after = run_graph(
        &po,
        &target,
        &reject_cid,
        json_map(&[("release", Value::from(true))]),
    )
    .await;
    assert_eq!(
        404,
        after.status(),
        "a rejected workflow cannot be continued: {:?}",
        after.body()
    );

    // --- an invalid or missing decision keeps the workflow waiting: the
    // check-approval decision routes it to await-decision, which re-suspends
    // and loops its continuation back to the decision (RESET clears the loop
    // nodes' seen marks on every pass, so the loop is stable across resumes)
    let wait_cid = uuid::Uuid::new_v4().simple().to_string();
    let ordered = stage(
        &run_graph(
            &po,
            &target,
            &wait_cid,
            json_map(&[
                ("item", Value::from("keyboard")),
                ("amount", Value::from(80)),
            ]),
        )
        .await,
        &wait_cid,
    );
    assert!(
        text_of(get_element(&ordered, &["stage"])).starts_with("order-submitted"),
        "wait-path run 1: {ordered:?}"
    );
    // run 2: no decision at all (e.g. a stale-record replay) - re-suspends
    let waiting = stage(
        &run_graph(
            &po,
            &target,
            &wait_cid,
            json_map(&[("note", Value::from("no decision here"))]),
        )
        .await,
        &wait_cid,
    );
    assert!(
        text_of(get_element(&waiting, &["stage"])).starts_with("awaiting-decision"),
        "an invalid decision re-suspends: {waiting:?}"
    );
    assert_eq!("resume", text_of(get_element(&waiting, &["run"])));
    // run 3: an unrecognized decision loops the same way - the wait loop is
    // stable across suspensions
    let still_waiting = stage(
        &run_graph(
            &po,
            &target,
            &wait_cid,
            json_map(&[("decision", Value::from("maybe"))]),
        )
        .await,
        &wait_cid,
    );
    assert!(
        text_of(get_element(&still_waiting, &["stage"])).starts_with("awaiting-decision"),
        "an unrecognized decision keeps waiting: {still_waiting:?}"
    );
    // run 4: the manager finally approves - the workflow exits the wait loop
    let approved_after_wait = stage(
        &run_graph(
            &po,
            &target,
            &wait_cid,
            json_map(&[
                ("decision", Value::from("approved")),
                ("manager", Value::from("store-88")),
            ]),
        )
        .await,
        &wait_cid,
    );
    assert!(
        text_of(get_element(&approved_after_wait, &["stage"])).starts_with("approved"),
        "a valid decision exits the wait loop: {approved_after_wait:?}"
    );
}
