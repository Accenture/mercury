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

//! Latency summary — Rust port of the Java `Stats` record: exact nearest-rank
//! percentiles over a sorted copy, mean / population stddev, and the same
//! log-spaced histogram bins, so Rust and Java reports chart identically.

/// Upper edges (ms) of the log-spaced histogram bins; the last absorbs the rest.
pub const EDGES_MS: [f64; 19] = [
    0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1000.0,
    2000.0, 5000.0, 10000.0,
];

/// Human-readable labels for [`EDGES_MS`].
pub const EDGE_LABELS: [&str; 19] = [
    "10µs", "20µs", "50µs", "0.1ms", "0.2ms", "0.5ms", "1ms", "2ms", "5ms", "10ms", "20ms", "50ms",
    "100ms", "200ms", "500ms", "1s", "2s", "5s", "10s+",
];

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub count: u64,
    pub min_ms: f64,
    pub mean_ms: f64,
    pub stddev_ms: f64,
    pub p50: f64,
    pub p90: f64,
    pub p99: f64,
    pub p999: f64,
    pub p9999: f64,
    pub max_ms: f64,
    pub bin_counts: Vec<u64>,
}

impl Stats {
    /// Compute from raw nanosecond samples (consumed; sorted in place).
    pub fn compute(mut ns: Vec<u64>) -> Stats {
        let n = ns.len();
        if n == 0 {
            return Stats {
                bin_counts: vec![0; EDGES_MS.len()],
                ..Stats::default()
            };
        }
        ns.sort_unstable();
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        let mut bins = vec![0u64; EDGES_MS.len()];
        for &sample in &ns {
            let ms = sample as f64 / 1e6;
            sum += ms;
            sum_sq += ms * ms;
            let mut bin = 0;
            while bin < EDGES_MS.len() - 1 && ms > EDGES_MS[bin] {
                bin += 1;
            }
            bins[bin] += 1;
        }
        let mean = sum / n as f64;
        let variance = (sum_sq / n as f64 - mean * mean).max(0.0);
        Stats {
            count: n as u64,
            min_ms: ns[0] as f64 / 1e6,
            mean_ms: mean,
            stddev_ms: variance.sqrt(),
            p50: pct(&ns, 50.0),
            p90: pct(&ns, 90.0),
            p99: pct(&ns, 99.0),
            p999: pct(&ns, 99.9),
            p9999: pct(&ns, 99.99),
            max_ms: ns[n - 1] as f64 / 1e6,
            bin_counts: bins,
        }
    }
}

/// Nearest-rank percentile (ms) from an ascending-sorted nanosecond slice.
fn pct(sorted: &[u64], p: f64) -> f64 {
    let n = sorted.len();
    let rank = ((p / 100.0 * n as f64).ceil() as usize).clamp(1, n);
    sorted[rank - 1] as f64 / 1e6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_rank_percentiles() {
        // 1..=100 ms in nanoseconds
        let ns: Vec<u64> = (1..=100u64).map(|ms| ms * 1_000_000).collect();
        let s = Stats::compute(ns);
        assert_eq!(s.count, 100);
        assert_eq!(s.p50, 50.0);
        assert_eq!(s.p90, 90.0);
        assert_eq!(s.p99, 99.0);
        assert_eq!(s.max_ms, 100.0);
        assert_eq!(s.min_ms, 1.0);
        assert!((s.mean_ms - 50.5).abs() < 1e-9);
        // bins sum to count
        assert_eq!(s.bin_counts.iter().sum::<u64>(), 100);
    }

    #[test]
    fn empty_is_zeroed() {
        let s = Stats::compute(Vec::new());
        assert_eq!(s.count, 0);
        assert_eq!(s.bin_counts.len(), EDGES_MS.len());
    }
}
