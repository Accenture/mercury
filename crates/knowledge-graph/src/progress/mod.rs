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

//! Progress events from a graph run (RFC-0011 section 8.4) — the Rust port of the Java
//! `com.accenture.minigraph.progress` package. A caller that watches a run gets progress events
//! while it goes on, then one terminal event, `result` or `error`:
//!
//! - [`return_paths`] — the routes an application declares as return paths
//!   (`return.path.routes`, Java `ReturnPaths`);
//! - [`return_channel`] — the way back to the caller of one run, framed for the entry's
//!   transport (Java `ReturnChannel`);
//! - `relay` — the relay route of one node in a watched run (Java `ProgressRelay`).

mod relay;
pub mod return_channel;
pub mod return_paths;

pub use return_channel::{ReturnChannel, PROGRESS, X_RETURN_PATH};
pub use return_paths::{ReturnPaths, RETURN_PATH_ROUTES};
