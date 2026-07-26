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

//! Utility layer of platform-core — configuration management first
//! (`docs/design/platform-core-port.md`, increment 1).

pub mod app_config_reader;
pub mod config_reader;
pub mod elastic_queue;
pub mod feature;
pub mod multi_level_map;
pub mod overrides;
pub mod resources;
pub mod w3c_trace;
