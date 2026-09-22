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

//! The exporter against the in-process collector double (the Java
//! `OtlpComposableExportTest` + `OtlpExportRetryTest` +
//! `ExportFailureDiagnosticsTest` twins): the OTLP bytes round-trip with the
//! exact ids, the credential header arrives, transient failures are retried
//! and final ones are diagnosed, and the credential is re-read on every export.
//!
//! One test function: the platform boots once per process, and the scenarios
//! share it (the suite convention).

mod support;

use std::time::Duration;

use async_trait::async_trait;
use opentelemetry_forwarder::{
    config, Exporter, ForwarderSettings, Span, FORWARDER_ROUTE, INSTRUMENTATION_SCOPE,
};
use platform_core::{
    main_application, overrides, resources, AppConfigReader, AppError, AutoStart, EntryPoint,
    Platform,
};
use support::mock_collector::MockCollector;

const TRACE_ID: &str = "4bf92f3577b34da6a3ce929d0e0e4736";
const SPAN_ID: &str = "00f067aa0ba902b7";
const PARENT_SPAN_ID: &str = "a3ce929d0e0e4736";
const WAIT: Duration = Duration::from_secs(10);

/// The lifecycle needs an application entry point; this suite has nothing to start.
#[main_application]
struct TestApp;

#[async_trait]
impl EntryPoint for TestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

fn fast_backoff() -> Vec<Duration> {
    vec![Duration::from_millis(20); 4]
}

fn sample_span() -> Span {
    let dataset = serde_json::json!({
        "trace": {
            "id": TRACE_ID,
            "span_id": SPAN_ID,
            "parent_span_id": PARENT_SPAN_ID,
            "service": "hello.world",
            "path": "/api/hello",
            "from": "http.request",
            "origin": "node-1",
            "start": "2026-06-24T10:00:00Z",
            "exec_time": 12.5,
            "status": 200,
            "success": true,
        },
        "annotations": { "user": "alice" }
    });
    Span::from_dataset(&rmpv::ext::to_value(dataset).expect("dataset")).expect("a valid span")
}

fn exporter_for(collector: &MockCollector, path: &str, headers: Vec<(String, String)>) -> Exporter {
    Exporter::from_settings(
        ForwarderSettings::fixed(&collector.url(path), 5000, headers)
            .with_service_name("mercury-otel-demo"),
    )
    .expect("a valid endpoint")
    .with_backoff(fast_backoff())
}

fn credential() -> Vec<(String, String)> {
    vec![(
        "Authorization".to_string(),
        "Api-Token test-secret".to_string(),
    )]
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn exporter_end_to_end() {
    // the collector first: its random port must exist before the configuration
    // snapshot resolves ${OTEL_FWD_TEST_ENDPOINT}
    let collector = MockCollector::start().await;
    std::env::set_var(
        "OTEL_FWD_TEST_ENDPOINT",
        collector.url("/api/v2/otlp/v1/traces"),
    );
    std::env::remove_var("OTEL_FWD_TEST_HEADERS");
    resources::prepend_resource_root("tests/resources-exporter");
    let _ = AppConfigReader::get_instance();
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();
    assert!(
        !platform.has_route(FORWARDER_ROUTE),
        "the switch is absent in this suite's configuration - nothing auto-registers"
    );

    // --- Scenario 1: the ids survive the wire, the credential arrives ----------
    for path in ["/api/v2/otlp/v1/traces", "/v2/trace/otlp"] {
        collector.clear();
        let exporter = exporter_for(&collector, path, credential());
        exporter.export(&sample_span()).await.expect("export");
        let captured = collector.wait_for(1, WAIT).await;
        let request = &captured[0];
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, path);
        assert_eq!(
            request.header("content-type"),
            Some("application/x-protobuf")
        );
        assert_eq!(
            request.header("authorization"),
            Some("Api-Token test-secret"),
            "the backend credential header must reach the collector"
        );
        assert_eq!(request.service_name.as_deref(), Some("mercury-otel-demo"));
        assert_eq!(request.scope_name.as_deref(), Some(INSTRUMENTATION_SCOPE));
        assert_eq!(
            request.scope_version.as_deref(),
            Some(env!("CARGO_PKG_VERSION"))
        );
        assert_eq!(request.spans.len(), 1, "one span per export");
        let span = &request.spans[0];
        assert_eq!(span.trace_id, TRACE_ID);
        assert_eq!(span.span_id, SPAN_ID);
        assert_eq!(span.parent_span_id.as_deref(), Some(PARENT_SPAN_ID));
        assert_eq!(span.name, "hello.world");
        assert_eq!(span.kind, 2, "from=http.request -> SERVER");
        assert_eq!(span.status_code, 1, "success -> OK");
        assert_eq!(span.flags, 0x101, "sampled, local");
        assert_eq!(span.start_unix_nano, 1_782_295_200_000_000_000);
        assert_eq!(span.end_unix_nano - span.start_unix_nano, 12_500_000);
        assert_eq!(span.attribute("route"), Some("hello.world"));
        assert_eq!(span.attribute("path"), Some("/api/hello"));
        assert_eq!(span.attribute("from"), Some("http.request"));
        assert_eq!(span.attribute("origin"), Some("node-1"));
        assert_eq!(span.attribute("status"), Some("200"));
        assert_eq!(span.attribute("exec_time_ms"), Some("12.5"));
        assert_eq!(span.attribute("annotation.user"), Some("alice"));
    }

    // --- Scenario 2: transient statuses are retried to success ---------------
    let flaky = MockCollector::start_with(vec![(503, "busy"), (429, "slow down")], 0).await;
    exporter_for(&flaky, "/v1/traces", Vec::new())
        .export(&sample_span())
        .await
        .expect("a 503 then a 429 must be retried, not dropped");
    assert_eq!(
        flaky.requests(),
        3,
        "two rejected attempts, then the one that landed"
    );

    // --- Scenario 3: a killed connection is retried ---------------------------
    // the collector reads the request and closes without answering: the client
    // sees EOF where the status line should be (the failure that dropped a span
    // in the Java module's CI until its retry predicate was widened)
    let killer = MockCollector::start_with(Vec::new(), 1).await;
    exporter_for(&killer, "/v1/traces", Vec::new())
        .export(&sample_span())
        .await
        .expect("the second attempt lands on a healthy connection");
    assert!(
        killer.connections() >= 2,
        "success requires a SECOND connection (got {})",
        killer.connections()
    );
    assert_eq!(
        killer.requests(),
        1,
        "exactly one request reached the HTTP layer"
    );

    // --- Scenario 4: final rejections are not retried and name their cause ----
    let rejecting = MockCollector::start_with(
        vec![
            (404, ""),
            (401, "Token Authentication failed"),
            (
                403,
                "User is missing required permission: openpipeline:traces:ingest",
            ),
            (400, &"x".repeat(5000)),
        ],
        0,
    )
    .await;
    let exporter = exporter_for(&rejecting, "/api/v2/otlp/traces", credential());
    let e404 = exporter
        .export(&sample_span())
        .await
        .expect_err("404 fails");
    assert_eq!(e404.attempts, 1, "a 404 is final");
    assert!(e404.detail.starts_with("HTTP 404"), "{e404}");
    assert!(e404.detail.contains("signal path"), "{e404}");
    assert!(e404.detail.contains("/v1/traces"), "{e404}");
    let e401 = exporter
        .export(&sample_span())
        .await
        .expect_err("401 fails");
    assert!(
        e401.detail
            .starts_with("HTTP 401 - Token Authentication failed"),
        "{e401}"
    );
    assert!(e401.detail.contains("otel.exporter.otlp.headers"), "{e401}");
    assert!(!e401.detail.contains("signal path"), "{e401}");
    let e403 = exporter
        .export(&sample_span())
        .await
        .expect_err("403 fails");
    assert!(e403.detail.contains("openpipeline:traces:ingest"), "{e403}");
    assert!(e403.detail.contains("lacks permission"), "{e403}");
    let e400 = exporter
        .export(&sample_span())
        .await
        .expect_err("400 fails");
    assert!(
        e400.detail.len() < 600,
        "bounded body: {}",
        e400.detail.len()
    );
    assert!(e400.detail.ends_with("..."), "{e400}");
    assert_eq!(
        rejecting.requests(),
        4,
        "one attempt each - no retry on a final status"
    );

    // --- Scenario 5: the credential is re-read on every export ----------------
    // the production path: headers resolve through application configuration,
    // whose ${OTEL_FWD_TEST_HEADERS} is unset at boot. A credential a bootstrap
    // publishes later arrives as a runtime override (`overrides::set`, the
    // `-D` / System.setProperty analog, consulted first on every lookup) - the
    // environment itself is resolved once, when the configuration loads.
    let configured = Exporter::from_settings(ForwarderSettings::from_config(
        AppConfigReader::get_instance(),
    ))
    .expect("configured endpoint")
    .with_backoff(fast_backoff());
    assert_eq!(
        configured.endpoint(),
        collector.url("/api/v2/otlp/v1/traces")
    );
    assert_eq!(configured.service_name(), "mercury-otel-demo");
    assert!(configured.header_names().is_empty(), "no credential yet");
    collector.clear();
    configured
        .export(&sample_span())
        .await
        .expect("export without credential");
    let without = collector.wait_for(1, WAIT).await;
    assert_eq!(
        without[0].header("authorization"),
        None,
        "unset -> no header"
    );
    overrides::set(config::HEADERS, "Authorization=Api-Token published-later");
    collector.clear();
    configured
        .export(&sample_span())
        .await
        .expect("export with the late credential");
    let with = collector.wait_for(1, WAIT).await;
    assert_eq!(
        with[0].header("authorization"),
        Some("Api-Token published-later"),
        "a credential published after construction must be picked up without a rebuild"
    );
    assert_eq!(configured.header_names(), vec!["Authorization".to_string()]);
    overrides::clear(config::HEADERS);
    assert!(
        configured.header_names().is_empty(),
        "the override was the only source"
    );

    // --- Scenario 6: a misconfigured endpoint is refused at construction -------
    let Err(err) =
        Exporter::from_settings(ForwarderSettings::fixed("localhost:4318", 1000, Vec::new()))
    else {
        panic!("not an http(s) URL - must be refused");
    };
    assert_eq!(err.status(), 400);
    assert!(
        err.message().contains("otel.exporter.otlp.endpoint"),
        "{}",
        err.message()
    );
}
