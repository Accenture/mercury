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

//! Integration tests for the elastic overflow buffer (increment 3) — the
//! two-tier FIFO, segment sealing, and immediate reclamation.
//!
//! The base directory and a tiny segment size (512 = the minimum, forcing
//! multi-segment spill) are injected through the process override registry
//! before the first `ElasticQueue` is constructed — the overrides are read at
//! construction/first-use, dogfooding the config layer.

use platform_core::util::elastic_queue::{base_dir, ElasticQueue, MEMORY_BUFFER};
use platform_core::{overrides, resources};
use std::sync::Once;

fn setup() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        resources::prepend_resource_root("tests/resources");
        let holding = std::env::temp_dir().join(format!("mercury-eq-test-{}", std::process::id()));
        overrides::set("transient.data.store", &holding.display().to_string());
        overrides::set("elastic.queue.segment.size.bytes", "512");
    });
}

fn payload(n: u64) -> Vec<u8> {
    // > 100 bytes so a 512-byte segment seals after a few records
    format!("event-{n}-{}", "x".repeat(100)).into_bytes()
}

fn segment_files_for(id_fragment: &str) -> Vec<String> {
    std::fs::read_dir(base_dir())
        .map(|entries| {
            entries
                .flatten()
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|name| {
                    name.starts_with("eq-") && name.contains(id_fragment) && name.ends_with(".dat")
                })
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn fifo_order_across_memory_and_disk_tiers() {
    setup();
    let total = MEMORY_BUFFER + 30; // spans the memory tier and several segments
    let mut queue = ElasticQueue::new("fifo.order.test");
    for n in 0..total {
        queue.write(&payload(n)).unwrap();
    }
    assert_eq!(queue.write_counter(), total);
    // spilled records exist on disk while the queue holds the overflow
    assert!(
        !segment_files_for("fifo.order.test").is_empty(),
        "expected overflow segments on disk"
    );
    for n in 0..total {
        assert_eq!(queue.read().unwrap(), payload(n), "FIFO order broke at {n}");
    }
    // caught up: the next read closes and resets the queue
    assert!(queue.read().unwrap().is_empty());
    assert!(queue.is_closed());
    assert!(
        segment_files_for("fifo.order.test").is_empty(),
        "all segments should be reclaimed after drain"
    );
}

#[test]
fn peek_holds_the_next_event_without_consuming() {
    setup();
    let mut queue = ElasticQueue::new("peek.test");
    queue.write(&payload(1)).unwrap();
    queue.write(&payload(2)).unwrap();
    assert_eq!(queue.peek().unwrap(), payload(1));
    assert_eq!(queue.peek().unwrap(), payload(1)); // still held
    assert_eq!(queue.read().unwrap(), payload(1)); // consumes the held event
    assert_eq!(queue.read().unwrap(), payload(2));
}

#[test]
fn queue_is_reusable_after_drain_with_a_new_generation() {
    setup();
    let mut queue = ElasticQueue::new("reuse.test");
    for n in 0..(MEMORY_BUFFER + 5) {
        queue.write(&payload(n)).unwrap();
    }
    while !queue.read().unwrap().is_empty() {}
    assert!(queue.is_closed());
    // second burst reuses the queue (Java: counters reset, generation++)
    for n in 100..(100 + MEMORY_BUFFER + 5) {
        queue.write(&payload(n)).unwrap();
    }
    for n in 100..(100 + MEMORY_BUFFER + 5) {
        assert_eq!(queue.read().unwrap(), payload(n));
    }
}

#[test]
fn sealed_consumed_segments_are_reclaimed_immediately() {
    setup();
    let mut queue = ElasticQueue::new("reclaim.test");
    let total = MEMORY_BUFFER + 40; // several 512-byte segments
    for n in 0..total {
        queue.write(&payload(n)).unwrap();
    }
    let during_burst = segment_files_for("reclaim.test").len();
    assert!(
        during_burst >= 2,
        "expected multiple sealed segments, saw {during_burst}"
    );
    // drain past the memory tier and most of the disk tier
    for _ in 0..(total - 5) {
        assert!(!queue.read().unwrap().is_empty());
    }
    let near_end = segment_files_for("reclaim.test").len();
    assert!(
        near_end < during_burst,
        "consumed segments should be deleted incrementally ({near_end} vs {during_burst})"
    );
    while !queue.read().unwrap().is_empty() {}
    assert!(segment_files_for("reclaim.test").is_empty());
}

#[test]
fn destroy_purges_stray_segment_files() {
    setup();
    let mut queue = ElasticQueue::new("destroy.test");
    for n in 0..(MEMORY_BUFFER + 20) {
        queue.write(&payload(n)).unwrap();
    }
    assert!(!segment_files_for("destroy.test").is_empty());
    queue.destroy();
    assert!(queue.is_closed());
    assert!(segment_files_for("destroy.test").is_empty());
}

#[test]
fn empty_writes_are_ignored() {
    setup();
    let mut queue = ElasticQueue::new("empty.write.test");
    queue.write(&[]).unwrap();
    assert!(queue.is_closed());
    assert!(queue.read().unwrap().is_empty());
}
