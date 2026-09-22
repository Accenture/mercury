//! The E0 twin in this Playground: the support-triage graph (its AI node is a function on a
//! python/node wrapper host) compiles at boot, and the `/api/llm/stream` relay teaches
//! instead of hopping into nothing when no wrapper host is mapped.
use std::time::Duration;

use async_trait::async_trait;
use platform_core::automation::AsyncHttpRequest;
use platform_core::{
    automation, main_application, overrides, AppError, AutoStart, EntryPoint, EventEnvelope,
    Platform, PostOffice,
};
use rmpv::Value;

/// The test's own entry point; naming the library's route constant links its
/// annotation inventory (the relay) into this test binary.
#[main_application]
struct TestApp;

#[async_trait]
impl EntryPoint for TestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        log::info!(
            "relay under test: {}",
            minigraph_playground::LLM_STREAM_RELAY_ROUTE
        );
        Ok(())
    }
}

fn body_text(body: &Value) -> String {
    match body {
        Value::String(s) => s.as_str().unwrap_or_default().to_string(),
        Value::Binary(bytes) => String::from_utf8_lossy(bytes).to_string(),
        other => format!("{other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn support_triage_compiles_and_the_relay_teaches_when_unmapped() {
    overrides::set("rest.server.port", "0");
    // no peer map in this test: the relay's teaching failure, not a hop into nothing
    overrides::set("yaml.event.over.http", "classpath:/no-such-peer-map.yaml");
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let graphs = knowledge_graph::graphs::get_all_graphs();
    assert!(
        graphs.iter().any(|g| g == "support-triage"),
        "the support-triage graph compiles at boot - graphs: {graphs:?}"
    );
    let port = automation::server_address().expect("server started").port();
    let po = PostOffice::new(&Platform::get_instance());
    let request = AsyncHttpRequest::new()
        .set_method("POST")
        .set_target_host(&format!("http://127.0.0.1:{port}"))
        .set_url("/api/llm/stream")
        .set_header("content-type", "application/json")
        .set_header("accept", "text/event-stream")
        .set_body(Value::Map(vec![(
            Value::from("prompt"),
            Value::from("hello"),
        )]));
    let response = po
        .request(
            EventEnvelope::new()
                .set_to("async.http.request")
                .set_raw_body(request.to_value()),
            Duration::from_secs(20),
        )
        .await
        .expect("relay reply");
    let text = body_text(response.body());
    assert!(
        text.contains("not configured"),
        "the relay names the missing configuration - status {} body {text}",
        response.status()
    );
}
