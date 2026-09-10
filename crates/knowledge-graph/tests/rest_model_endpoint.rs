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

//! Pins the temp graph-model endpoint's not-found contract: a draft that does
//! not exist (never described, or expired by housekeeping) answers **404** as
//! if it is not there — the deployed-graph "compiled or 404" precedent. The
//! `{sequence}` path parameter is an artificial cache-buster, deliberately not
//! part of the resource identity (Java `DescribeGraph` parity).

use knowledge_graph::rest::show_graph_model;
use platform_core::EventEnvelope;
use rmpv::Value;

fn model_request(graph_id: &str, sequence: &str) -> EventEnvelope {
    EventEnvelope::new().set_raw_body(Value::Map(vec![
        (
            Value::from("parameters"),
            Value::Map(vec![(
                Value::from("path"),
                Value::Map(vec![
                    (Value::from("graph_id"), Value::from(graph_id)),
                    (Value::from("sequence"), Value::from(sequence)),
                ]),
            )]),
        ),
        (Value::from("method"), Value::from("GET")),
    ]))
}

#[tokio::test]
async fn missing_draft_answers_404() {
    let err = show_graph_model(model_request("no-such-draft", "1"))
        .await
        .expect_err("a nonexistent draft must not answer 200");
    assert_eq!(err.status(), 404);
    assert_eq!(err.message(), "Draft graph 'no-such-draft' does not exist");
}
