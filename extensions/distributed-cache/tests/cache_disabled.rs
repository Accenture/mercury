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

//! "Dependency present, feature off": an application that links this crate
//! but does not set `redis.cache.enabled` registers NEITHER function — the
//! `#[optional_service]` gate the Java module expresses with
//! `@OptionalService("redis.cache.enabled")`, pinned by the Java example.
//! A separate process from the contract suite, because the platform boots
//! once per process.

use async_trait::async_trait;
use distributed_cache::{CACHE_ROUTE, HEALTH_ROUTE};
use platform_core::{main_application, overrides, AppError, AutoStart, EntryPoint, Platform};

#[main_application]
struct CacheDisabledTestApp;

#[async_trait]
impl EntryPoint for CacheDisabledTestApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn nothing_registers_while_the_cache_is_not_enabled() {
    platform_core::resources::prepend_resource_root("tests/resources");
    overrides::clear("redis.cache.enabled");
    AutoStart::main(vec![]).await.expect("lifecycle");
    let platform = Platform::get_instance();
    assert!(
        !platform.has_route(CACHE_ROUTE),
        "{CACHE_ROUTE} must stay unregistered without redis.cache.enabled=true"
    );
    assert!(
        !platform.has_route(HEALTH_ROUTE),
        "{HEALTH_ROUTE} is gated by the same switch"
    );
}
