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

//! One workload's outcome (Java `WorkloadResult`).

use crate::stats::Stats;

pub struct WorkloadResult {
    pub name: String,
    pub category: String,
    pub description: String,
    /// Ordered display parameters (insertion order preserved — Java LinkedHashMap).
    pub params: Vec<(String, String)>,
    /// Attempted operations (Java parity); informational — the report derives
    /// counts from `stats.count` and `failures`.
    #[allow(dead_code)]
    pub total: u64,
    pub failures: u64,
    pub elapsed_sec: f64,
    pub stats: Stats,
}

impl WorkloadResult {
    /// Successful operations per second over the wall-clock window.
    pub fn throughput(&self) -> f64 {
        if self.elapsed_sec > 0.0 {
            self.stats.count as f64 / self.elapsed_sec
        } else {
            0.0
        }
    }
}
