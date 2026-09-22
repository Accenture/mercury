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

//! The home page follows the dev-mode gate. `get.index.html` serves the Playground
//! web app ONLY when `app.env=dev` (the same gate as every Playground service); any
//! other value serves the plain service page, so a production deployment never
//! shows the Playground UI. The Playground page lives outside the static folder, and
//! the static `resources/public/index.html` is the same plain page — so an
//! application that does not route `get.index.html` never serves the Playground by
//! accident either (Java `GetIndexHtml` parity).
//!
//! Its own test binary on purpose: it flips the process-wide `app.env` override.

use knowledge_graph::rest::{get_index_html, PLAIN_PAGE, PLAYGROUND_PAGE};
use platform_core::{overrides, EventEnvelope};

const PLAYGROUND_TITLE: &str = "<title>Minigraph Playground</title>";
const PLAIN_TITLE: &str = "<h2>MiniGraph Service</h2>";

fn resources() {
    platform_core::resources::append_resource_root(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources"
    ));
}

async fn home_page() -> String {
    let page = get_index_html(EventEnvelope::new())
        .await
        .expect("home page");
    assert_eq!(
        page.headers().get("Content-Type").map(String::as_str),
        Some("text/html; charset=utf-8")
    );
    page.body().as_str().expect("html text").to_string()
}

#[tokio::test]
async fn home_page_follows_the_dev_mode_gate() {
    resources();
    // dev: the Playground web app
    overrides::set("app.env", "dev");
    let page = home_page().await;
    assert!(
        page.contains(PLAYGROUND_TITLE),
        "dev mode must serve the Playground page: {page}"
    );
    // any other environment: the plain service page
    overrides::set("app.env", "prod");
    let page = home_page().await;
    assert!(
        page.contains(PLAIN_TITLE),
        "prod must serve the plain page: {page}"
    );
    assert!(!page.contains(PLAYGROUND_TITLE));
    overrides::clear("app.env");
}

#[test]
fn playground_page_is_outside_the_static_folder() {
    resources();
    // the Playground page is reachable only through get.index.html ...
    assert!(PLAYGROUND_PAGE.starts_with("/template/"));
    assert!(platform_core::resources::resolve_classpath(PLAYGROUND_PAGE).is_some());
    assert!(platform_core::resources::resolve_classpath("/public/playground.html").is_none());
    // ... and the static index.html is the plain page, never the web app
    let static_page = platform_core::resources::resolve_classpath("/public/index.html")
        .and_then(|p| std::fs::read_to_string(p).ok())
        .expect("static public/index.html");
    assert!(static_page.contains(PLAIN_TITLE));
    assert!(!static_page.contains(PLAYGROUND_TITLE));
    let plain_page = platform_core::resources::resolve_classpath(PLAIN_PAGE)
        .and_then(|p| std::fs::read_to_string(p).ok())
        .expect("template/index.html");
    assert!(plain_page.contains(PLAIN_TITLE));
}
