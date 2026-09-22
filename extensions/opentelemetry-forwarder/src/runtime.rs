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

//! Process-wide state: the exporter the startup hook validated and installed,
//! shared by the forwarder's worker instances.

use std::sync::{Arc, OnceLock, RwLock};

use crate::export::Exporter;

fn slot() -> &'static RwLock<Option<Arc<Exporter>>> {
    static SLOT: OnceLock<RwLock<Option<Arc<Exporter>>>> = OnceLock::new();
    SLOT.get_or_init(|| RwLock::new(None))
}

/// Install the shared exporter (the startup hook; a test may install its own).
pub fn install(exporter: Arc<Exporter>) {
    *slot().write().expect("exporter slot poisoned") = Some(exporter);
}

/// The installed exporter, if any.
pub fn exporter() -> Option<Arc<Exporter>> {
    slot().read().expect("exporter slot poisoned").clone()
}

/// Remove the installed exporter (test hygiene).
pub fn clear() {
    *slot().write().expect("exporter slot poisoned") = None;
}
