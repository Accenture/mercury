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

//! Full pipeline (the Java `OtlpTracePipelineTest` twin): a traced RPC chain
//! `fun.1 -> fun.2 -> fun.3` makes the engine emit three linked telemetry
//! datasets; the REAL forwarder — registered by configuration alone
//! (`otel.forwarding: true`) — maps and exports each one to the collector
//! double over OTLP. The test asserts the spans arrived with the trace id the
//! test started, the parent-child lineage, the resource and scope, and the
//! configured credential header.

mod support;

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use opentelemetry_forwarder::{FORWARDER_ROUTE, INSTRUMENTATION_SCOPE};
use platform_core::{
    main_application, preload, resources, trace, AppConfigReader, AppError, AutoStart,
    ComposableFunction, EntryPoint, EventEnvelope, Platform, PostOffice,
};
use support::mock_collector::{DecodedSpan, MockCollector};

const CHAIN: [&str; 3] = ["fun.1", "fun.2", "fun.3"];

/// The lifecycle needs an application entry point; this suite has nothing to start.
#[main_application]
struct TestApp;

#[async_trait]
impl EntryPoint for TestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

/// The three chain functions: each annotates its span and calls the next.
#[preload(route = "fun.1", instances = 5)]
struct Fun1;

#[preload(route = "fun.2", instances = 5)]
struct Fun2;

#[preload(route = "fun.3", instances = 5)]
struct Fun3;

async fn step(
    next: Option<&str>,
    step: &str,
    input: EventEnvelope,
) -> Result<EventEnvelope, AppError> {
    let po = PostOffice::new(&Platform::get_instance());
    po.annotate_trace("step", step);
    match next {
        Some(route) => {
            // the trace propagates automatically: no trace fields set here
            po.request(
                EventEnvelope::new()
                    .set_to(route)
                    .set_raw_body(input.body().clone()),
                Duration::from_secs(5),
            )
            .await
        }
        None => Ok(EventEnvelope::new().set_raw_body(input.body().clone())),
    }
}

#[async_trait]
impl ComposableFunction for Fun1 {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        step(Some("fun.2"), "one", input).await
    }
}

#[async_trait]
impl ComposableFunction for Fun2 {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        step(Some("fun.3"), "two", input).await
    }
}

#[async_trait]
impl ComposableFunction for Fun3 {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        step(None, "three", input).await
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn traced_rpc_chain_produces_linked_spans() {
    let collector = MockCollector::start().await;
    std::env::set_var(
        "OTEL_FWD_TEST_ENDPOINT",
        collector.url("/api/v2/otlp/v1/traces"),
    );
    resources::prepend_resource_root("tests/resources");
    let _ = AppConfigReader::get_instance();
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();
    assert!(
        platform.has_route(FORWARDER_ROUTE),
        "otel.forwarding=true must register the forwarder by configuration alone"
    );

    let trace_id = trace::new_trace_id();
    let po = PostOffice::new(&platform);
    let reply = po
        .request(
            EventEnvelope::new()
                .set_to("fun.1")
                .set_trace(&trace_id, "TEST /api/trace/chain")
                .set_body(serde_json::json!({"hello": "world"}))
                .expect("body"),
            Duration::from_secs(10),
        )
        .await
        .expect("the chain answers");
    assert_eq!(reply.status(), 200);

    // collect the three spans of THIS trace, keyed by span name (other traces,
    // if any, are ignored)
    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    let mut by_name: HashMap<String, (DecodedSpan, String)> = HashMap::new();
    let mut seen = 0usize;
    while by_name.len() < CHAIN.len() && tokio::time::Instant::now() < deadline {
        let captured = collector.wait_for(seen + 1, Duration::from_secs(15)).await;
        seen = captured.len();
        for request in &captured {
            for span in &request.spans {
                if span.trace_id == trace_id {
                    by_name.insert(
                        span.name.clone(),
                        (
                            span.clone(),
                            request.header("authorization").unwrap_or("").to_string(),
                        ),
                    );
                }
            }
        }
        if by_name.len() < CHAIN.len() {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
    let mut names: Vec<&str> = by_name.keys().map(String::as_str).collect();
    names.sort_unstable();
    assert_eq!(
        names, CHAIN,
        "the traced chain must forward exactly its three spans to the collector"
    );

    let (span1, _) = &by_name["fun.1"];
    let (span2, _) = &by_name["fun.2"];
    let (span3, _) = &by_name["fun.3"];
    assert_ne!(span1.span_id, span2.span_id);
    assert_ne!(span2.span_id, span3.span_id);
    // parent-child lineage: root(fun.1) <- fun.2 <- fun.3 - the exact ids the engine minted
    assert_eq!(
        span2.parent_span_id.as_deref(),
        Some(span1.span_id.as_str()),
        "fun.2's parent must be fun.1"
    );
    assert_eq!(
        span3.parent_span_id.as_deref(),
        Some(span2.span_id.as_str()),
        "fun.3's parent must be fun.2"
    );
    assert!(
        span1.parent_span_id.is_none(),
        "fun.1 is the root span, got parent {:?}",
        span1.parent_span_id
    );
    for (name, (span, authorization)) in &by_name {
        assert_eq!(span.kind, 1, "{name}: a route-to-route hop is INTERNAL");
        assert_eq!(span.status_code, 1, "{name}: success -> OK");
        assert_eq!(span.attribute("route"), Some(name.as_str()));
        assert!(span.end_unix_nano >= span.start_unix_nano, "{name}");
        assert_eq!(
            authorization, "Api-Token test-token",
            "{name}: the configured credential header reaches the collector"
        );
    }
    assert_eq!(span1.attribute("annotation.step"), Some("one"));
    assert_eq!(span2.attribute("annotation.step"), Some("two"));
    assert_eq!(span3.attribute("annotation.step"), Some("three"));
    let request = collector
        .captured()
        .into_iter()
        .find(|r| r.spans.iter().any(|s| s.trace_id == trace_id))
        .expect("a captured request of this trace");
    assert_eq!(
        request.header("content-type"),
        Some("application/x-protobuf")
    );
    assert_eq!(request.service_name.as_deref(), Some("mercury-otel-demo"));
    assert_eq!(request.scope_name.as_deref(), Some(INSTRUMENTATION_SCOPE));
    assert_eq!(
        request.scope_version.as_deref(),
        Some(env!("CARGO_PKG_VERSION"))
    );
}
