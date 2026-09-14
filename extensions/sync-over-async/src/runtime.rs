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

//! Process-wide holder for the running [`ReturnRouteCoordinator`] — Rust port
//! of the Java `SyncRuntime`, populated once at application startup.
//! Composable facades obtain the coordinator from here; observers use the
//! operation delegates ([`active_streams`], [`pending_count`]) rather than
//! obtaining the coordinator whose lifecycle this holder owns (the Java
//! PR #376 lesson: expose operations, not resources).
//!
//! Integration suites that boot several isolated pods in one process build
//! and hold their own coordinators instead — this holder is for the one
//! coordinator a real application runs.

use std::sync::{Arc, RwLock};

use platform_core::AppError;

use crate::config::SyncOverAsyncConfig;
use crate::connection::RedisSettings;
use crate::coordinator::ReturnRouteCoordinator;

static COORDINATOR: RwLock<Option<Arc<ReturnRouteCoordinator>>> = RwLock::new(None);

/// Connect, start the subscriber, and install the coordinator as the
/// process-wide instance — the application-startup convenience (the analog of
/// the Java `SyncOverAsyncAutoStart` body). Returns the installed handle.
pub async fn init(
    settings: &RedisSettings,
    origin: &str,
    config: SyncOverAsyncConfig,
) -> Result<Arc<ReturnRouteCoordinator>, AppError> {
    let coordinator = Arc::new(ReturnRouteCoordinator::connect(settings, origin, config).await?);
    coordinator.start().await?;
    set(coordinator.clone());
    Ok(coordinator)
}

/// Install a running coordinator as the process-wide instance (replacing any
/// previous one, which keeps running for handles already cloned out).
pub fn set(coordinator: Arc<ReturnRouteCoordinator>) {
    *COORDINATOR.write().expect("sync runtime poisoned") = Some(coordinator);
}

/// The running coordinator, or `None` if sync-over-async was not enabled at
/// startup.
pub fn coordinator() -> Option<Arc<ReturnRouteCoordinator>> {
    COORDINATOR.read().expect("sync runtime poisoned").clone()
}

/// Diagnostic: the number of open streaming rendezvous on this pod (`0` when
/// sync-over-async is not enabled).
pub fn active_streams() -> usize {
    coordinator().map_or(0, |c| c.active_streams())
}

/// Diagnostic: the number of in-flight one-shot requests on this pod (`0`
/// when sync-over-async is not enabled).
pub fn pending_count() -> usize {
    coordinator().map_or(0, |c| c.pending_count())
}

/// Stop the subscriber and release the process-wide instance (idempotent).
/// Facades holding cloned handles drop the engine when the last clone goes.
pub fn shutdown() {
    if let Some(coordinator) = COORDINATOR.write().expect("sync runtime poisoned").take() {
        coordinator.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The holder's empty-state contract: every observer answers the
    /// not-enabled shape instead of panicking.
    #[test]
    fn empty_holder_reports_not_enabled() {
        shutdown();
        assert!(coordinator().is_none());
        assert_eq!(0, active_streams());
        assert_eq!(0, pending_count());
        shutdown(); // idempotent
    }
}
