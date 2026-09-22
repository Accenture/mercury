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

//! Regression guard for the OpenTelemetry forwarding switch (the Java
//! composable-example's `OtelForwardingSwitchTest` twin).
//!
//! This application links the `mercury-opentelemetry-forwarder` crate but ships
//! `otel.forwarding: false` - the shape the switch exists to make safe: a team
//! can carry the dependency and let DevOps decide, per environment, whether
//! traces leave the process. Without the `#[optional_service("otel.forwarding")]`
//! gate the linked crate alone would register the route and start exporting.
//! That is what this test pins. The opposite direction (switch on, spans
//! reaching a collector) is covered in the forwarder crate's own suites.

use platform_core::{overrides, AppConfigReader, AutoStart, Platform};

#[allow(dead_code)]
#[path = "../src/main.rs"]
mod app;

const FORWARDER_ROUTE: &str = "distributed.trace.forwarder";

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_forwarder_is_not_registered_when_forwarding_is_off() {
    overrides::set("rest.server.port", "0");
    AutoStart::main(vec![]).await.expect("app lifecycle");
    let config = AppConfigReader::get_instance();
    assert_eq!(
        config.get_property("otel.forwarding").as_deref(),
        Some("false"),
        "this example ships with forwarding off - the 'dependency present, feature off' case"
    );
    assert!(
        !Platform::get_instance().has_route(FORWARDER_ROUTE),
        "otel.forwarding=false must skip registration entirely, not merely make the forwarder a \
         no-op: the crate is linked, so only the optional_service gate keeps the route from existing"
    );
}
