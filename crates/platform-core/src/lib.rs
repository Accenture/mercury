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

//! # platform-core (Rust port)
//!
//! Rust port of mercury-composable's `platform-core` — the event-driven
//! foundation layer. Canonical behavior spec: the Java project
//! (`system/platform-core`, v4.8.6) — see `docs/design/platform-core-port.md`.
//!
//! **Increment 1 — configuration management:** the `resources/` folder
//! convention, [`MultiLevelMap`] composite keys, [`ConfigReader`] with `${...}`
//! substitution, and the [`AppConfigReader`] base-config singleton.
//!
//! **Increment 2 — event-bus foundation:** the immutable [`EventEnvelope`],
//! the [`ComposableFunction`] contract (+ [`TypedFunction`]/[`TypedAdapter`]),
//! the [`Platform`] registry with per-route worker pools, and the
//! [`PostOffice`] messaging client (send + RPC). Functions are addressed only
//! by route name and never call each other directly.

pub mod actuator;
pub mod app_starter;
pub mod automation;
pub mod envelope;
pub mod function;
pub mod graph;
pub(crate) mod inbox;
pub mod logging;
pub mod platform;
pub mod post_office;
pub mod registry;
pub mod telemetry;
pub mod trace;
pub mod util;

pub use app_starter::{AppStarter, AutoStart, EntryPoint};
// the annotation layer (Java @PreLoad/@BeforeApplication/@MainApplication/@ZeroTracing)
pub use platform_macros::{before_application, main_application, preload};
// re-exported so the macros' generated `submit!` resolves without the user
// adding `inventory` as a direct dependency
pub use inventory;

/// Generate the application `main()` — the Java `AutoStart.main(args)`
/// one-liner. Expands **in the application crate**, so the app's own
/// `resources/` folder (next to its `Cargo.toml`) joins the resource roots:
///
/// ```ignore
/// platform_core::auto_start_main!();
/// ```
#[macro_export]
macro_rules! auto_start_main {
    () => {
        fn main() -> ::core::result::Result<(), $crate::AppError> {
            // the application's own resources/ folder (compile-time path of
            // the invoking crate)
            $crate::resources::prepend_resource_root(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources"
            ));
            $crate::AutoStart::run()
        }
    };
}
pub use envelope::EventEnvelope;
pub use function::{AppError, ComposableFunction, TypedAdapter, TypedFunction};
pub use graph::{MiniGraph, SimpleConnection, SimpleNode, SimpleRelationship};
pub use platform::{FunctionOptions, Platform};
pub use post_office::PostOffice;
pub use util::app_config_reader::AppConfigReader;
pub use util::config_reader::{ConfigError, ConfigReader};
pub use util::multi_level_map::{ConfigValue, MultiLevelMap};
pub use util::{overrides, resources};
