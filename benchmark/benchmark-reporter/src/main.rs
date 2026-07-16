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

//! benchmark-reporter — Rust port of the Java `BenchmarkApp`: a
//! self-contained, single-process end-to-end performance harness for the
//! mercury platform-core. It registers one echo worker route (with
//! `bench.consumers` instances) and runs the same six-scenario suite as the
//! Java original — normal operation, overload (back-pressure engaged), and
//! the mixed latency-isolation probe — then writes a self-contained HTML
//! report (inline SVG histogram + percentile plot + environment metadata).
//!
//! All parameters are `-Dkey=value` runtime arguments (the JVM `-D` analog):
//! `bench.ops`, `bench.warmup`, `bench.payload`, `bench.consumers`,
//! `bench.callback.pacing.micros`, `bench.callback.inflight`,
//! `bench.callback.producers`, `bench.probe.ops`, `bench.probe.pacing.micros`,
//! `bench.timeout`, `bench.report`.

mod html_report;
mod stats;
mod workload;

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use platform_core::{
    resources, trace, util::elastic_queue, AppConfigReader, AppError, AppStarter,
    ComposableFunction, EntryPoint, EventEnvelope, Platform, PostOffice,
};

use stats::Stats;
use workload::WorkloadResult;

const WORKER: &str = "benchmark.worker";
const PROBE: &str = "benchmark.probe";
const NORMAL: &str = "Normal operation (no back-pressure)";
const OVERLOAD: &str = "Overload (back-pressure engaged)";
const MIXED: &str = "Mixed workload (latency isolation under load)";

/// The echo worker (Java `Worker`): returns the request body untouched.
struct Worker;

#[async_trait]
impl ComposableFunction for Worker {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        Ok(EventEnvelope::new().set_raw_body(input.body().clone()))
    }
}

/// Benchmark parameters (Java system properties → `-D` runtime overrides).
#[derive(Clone)]
struct Params {
    ops: u64,
    warmup: u64,
    payload_bytes: usize,
    consumers: usize,
    callback_inflight: usize,
    callback_producers: usize,
    pacing_micros: u64,
    probe_ops: u64,
    probe_pace_micros: u64,
    timeout: Duration,
    report_path: String,
}

impl Params {
    fn load() -> Params {
        let config = AppConfigReader::get_instance();
        let get = |key: &str, default: u64| -> u64 {
            config
                .get_property_or(key, &default.to_string())
                .parse()
                .unwrap_or(default)
        };
        Params {
            ops: get("bench.ops", 200_000),
            warmup: get("bench.warmup", 20_000),
            payload_bytes: get("bench.payload", 256) as usize,
            consumers: get("bench.consumers", 50).max(1) as usize,
            callback_inflight: get("bench.callback.inflight", 2_000).max(1) as usize,
            callback_producers: get("bench.callback.producers", 1).max(1) as usize,
            pacing_micros: get("bench.callback.pacing.micros", 1_000),
            probe_ops: get("bench.probe.ops", 3_000),
            probe_pace_micros: get("bench.probe.pacing.micros", 2_000),
            timeout: Duration::from_millis(get("bench.timeout", 30_000)),
            report_path: config.get_property_or("bench.report", "/tmp/benchmark-report.html"),
        }
    }
}

struct BenchmarkApp;

#[async_trait]
impl EntryPoint for BenchmarkApp {
    async fn start(&self, _args: &[String]) -> Result<(), AppError> {
        let p = Params::load();
        let platform = Platform::get_instance();
        let payload: Vec<u8> = (0..p.payload_bytes).map(|i| (i & 0x7f) as u8).collect();
        let c = p.consumers;

        log::info!("Warming up…");
        run_rpc(
            &platform,
            &payload,
            &p,
            meta("warmup", "warmup", ""),
            c.min(16),
            p.warmup,
        )
        .await;
        run_callback(
            &platform,
            &payload,
            &p,
            meta("warmup", "warmup", ""),
            p.callback_producers,
            0,
            p.callback_inflight,
            p.warmup,
        )
        .await;

        let mut results = Vec::new();

        // --- Normal operation: backlog stays within the 20-event memory buffer ---
        log::info!("[1/6] RPC 1 → {c}");
        results.push(
            run_rpc(&platform, &payload, &p, meta(
                &format!("RPC · 1 → {c} consumers"),
                NORMAL,
                "One publisher, in-flight = 1: pure request/reply round-trip. Baseline latency; \
                 the ElasticQueue is idle.",
            ), 1, p.ops)
            .await,
        );

        log::info!("[2/6] RPC {c} → {c}");
        results.push(
            run_rpc(
                &platform,
                &payload,
                &p,
                meta(
                    &format!("RPC · {c} → {c} consumers"),
                    NORMAL,
                    &format!(
                        "Balanced (publishers = consumers): the backlog stays within the 20-event \
                     memory buffer, so it runs on the fast in-memory tier — throughput reaches \
                     the single-route dispatch ceiling (~an order of magnitude over 1→{c}) and \
                     latency stays sub-millisecond."
                    ),
                ),
                c,
                p.ops,
            )
            .await,
        );

        log::info!("[3/6] Callback {c} → {c} paced ~{}µs", p.pacing_micros);
        results.push(
            run_callback(
                &platform,
                &payload,
                &p,
                meta(
                    &format!("Callback · {c} → {c} consumers (paced)"),
                    NORMAL,
                    &format!(
                    "Async callback at a sustainable rate: {c} publishers spaced ~{}µs apart so \
                     consumers keep up. The backlog stays under the 20-event memory buffer — no \
                     disk spill, latency ≈ service time. Healthy steady-state async operation.",
                    p.pacing_micros
                ),
                ),
                c,
                p.pacing_micros,
                usize::MAX,
                p.ops,
            )
            .await,
        );

        // --- Overload: backlog exceeds the buffer and spills to disk ---
        log::info!("[4/6] RPC {} → {c} (over-subscribed 2:1)", 2 * c);
        results.push(
            run_rpc(
                &platform,
                &payload,
                &p,
                meta(
                    &format!("RPC · {} → {c} consumers", 2 * c),
                    OVERLOAD,
                    &format!(
                        "Over-subscribed 2:1: with {} publishers and {c} consumers, ~{c} requests \
                     queue behind the workers — above the 20-event memory buffer, so events \
                     spill to disk. Throughput drops to the disk-spill ceiling (≈ the flood \
                     below) and latency rises well beyond a naive 2×. This is back-pressure via \
                     oversubscription.",
                        2 * c
                    ),
                ),
                2 * c,
                p.ops,
            )
            .await,
        );

        log::info!(
            "[5/6] Callback flood → {c} (in-flight {})",
            p.callback_inflight
        );
        results.push(
            run_callback(
                &platform,
                &payload,
                &p,
                meta(
                    &format!("Callback · flood → {c} consumers"),
                    OVERLOAD,
                    &format!(
                    "Async callback with no pacing, up to {} in-flight — far above the 20-event \
                     memory buffer. Events spill through the ElasticQueue back-pressure buffer \
                     and latency becomes queue-bounded (Little's law), yet the system stays \
                     stable and loss-free.",
                    p.callback_inflight
                ),
                ),
                p.callback_producers,
                0,
                p.callback_inflight,
                p.ops,
            )
            .await,
        );

        // --- Mixed: latency probe on a separate route while flooding the worker ---
        log::info!("[6/6] Mixed: latency probe on {PROBE} while flooding {WORKER}");
        results.push(run_mixed(&platform, &payload, &p).await);

        let env = environment(&p);
        print_summary(&env, &results);
        let html = html_report::render(&env, &results);
        std::fs::write(&p.report_path, html)
            .map_err(|e| AppError::new(500, format!("Unable to write report: {e}")))?;
        println!("\nHTML report written to {}", p.report_path);
        Ok(())
    }
}

struct Meta {
    name: String,
    category: String,
    description: String,
}

fn meta(name: &str, category: &str, description: &str) -> Meta {
    Meta {
        name: name.to_string(),
        category: category.to_string(),
        description: description.to_string(),
    }
}

/// Closed-loop RPC: `publishers` tokio tasks each block on `request()`
/// (the Java virtual-thread client analog). Latencies collected per task and
/// merged after the join — no shared hot-path state.
async fn run_rpc(
    platform: &Platform,
    payload: &[u8],
    p: &Params,
    meta: Meta,
    publishers: usize,
    total_ops: u64,
) -> WorkloadResult {
    let per = (total_ops / publishers as u64).max(1);
    let total = per * publishers as u64;
    let started = Instant::now();
    let mut handles = Vec::with_capacity(publishers);
    for _ in 0..publishers {
        let platform = platform.clone();
        let payload = payload.to_vec();
        let timeout = p.timeout;
        handles.push(tokio::spawn(async move {
            let po = PostOffice::new(&platform);
            let mut samples = Vec::with_capacity(per as usize);
            let mut failures = 0u64;
            for _ in 0..per {
                let s = Instant::now();
                let event = EventEnvelope::new()
                    .set_to(WORKER)
                    .set_raw_body(rmpv_binary(&payload));
                match po.request(event, timeout).await {
                    Ok(_) => samples.push(s.elapsed().as_nanos() as u64),
                    Err(_) => failures += 1,
                }
            }
            (samples, failures)
        }));
    }
    let mut all = Vec::with_capacity(total as usize);
    let mut failures = 0u64;
    for handle in handles {
        let (samples, failed) = handle.await.expect("publisher task panicked");
        all.extend(samples);
        failures += failed;
    }
    let elapsed = started.elapsed().as_secs_f64();
    let params = vec![
        ("publishers (in-flight)".to_string(), publishers.to_string()),
        ("consumers".to_string(), p.consumers.to_string()),
        ("operations".to_string(), total.to_string()),
        ("payload".to_string(), format!("{} bytes", payload.len())),
    ];
    WorkloadResult {
        name: meta.name,
        category: meta.category,
        description: meta.description,
        params,
        total,
        failures,
        elapsed_sec: elapsed,
        stats: Stats::compute(all),
    }
}

/// Open-loop callback: `publishers` tasks fire requests without blocking on
/// the reply, sharing one global in-flight window (a semaphore). A per-fire
/// pause (`pacing_micros`; 0 = flood) sets the offered rate.
#[allow(clippy::too_many_arguments)]
async fn run_callback(
    platform: &Platform,
    payload: &[u8],
    p: &Params,
    meta: Meta,
    publishers: usize,
    pacing_micros: u64,
    max_inflight: usize,
    total_ops: u64,
) -> WorkloadResult {
    let per = (total_ops / publishers as u64).max(1);
    let total = per * publishers as u64;
    let permits = Arc::new(tokio::sync::Semaphore::new(
        max_inflight.min(tokio::sync::Semaphore::MAX_PERMITS),
    ));
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Result<u64, ()>>();
    let started = Instant::now();
    for _ in 0..publishers {
        let platform = platform.clone();
        let payload = payload.to_vec();
        let timeout = p.timeout;
        let permits = permits.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let po = PostOffice::new(&platform);
            for _ in 0..per {
                let permit = permits.clone().acquire_owned().await.expect("semaphore");
                let po = po.clone();
                let payload = payload.clone();
                let tx = tx.clone();
                let s = Instant::now();
                tokio::spawn(async move {
                    let event = EventEnvelope::new()
                        .set_to(WORKER)
                        .set_raw_body(rmpv_binary(&payload));
                    let outcome = match po.request(event, timeout).await {
                        Ok(_) => Ok(s.elapsed().as_nanos() as u64),
                        Err(_) => Err(()),
                    };
                    let _ = tx.send(outcome);
                    drop(permit);
                });
                if pacing_micros > 0 {
                    tokio::time::sleep(Duration::from_micros(pacing_micros)).await;
                }
            }
        });
    }
    drop(tx);
    let mut samples = Vec::with_capacity(total as usize);
    let mut failures = 0u64;
    while let Some(outcome) = rx.recv().await {
        match outcome {
            Ok(ns) => samples.push(ns),
            Err(()) => failures += 1,
        }
    }
    let elapsed = started.elapsed().as_secs_f64();
    let mut params = vec![
        ("publishers".to_string(), publishers.to_string()),
        ("consumers".to_string(), p.consumers.to_string()),
    ];
    if pacing_micros > 0 {
        params.push((
            "pacing".to_string(),
            format!("{pacing_micros} µs/publisher"),
        ));
    } else {
        params.push(("max in-flight".to_string(), max_inflight.to_string()));
    }
    params.push(("operations".to_string(), total.to_string()));
    params.push(("payload".to_string(), format!("{} bytes", payload.len())));
    WorkloadResult {
        name: meta.name,
        category: meta.category,
        description: meta.description,
        params,
        total,
        failures,
        elapsed_sec: elapsed,
        stats: Stats::compute(samples),
    }
}

/// Mixed workload: a paced latency probe on the separate PROBE route measured
/// while a background flood hammers WORKER's ElasticQueue. The reported stats
/// are the probe's — whether the background spill leaks into a
/// latency-sensitive path.
async fn run_mixed(platform: &Platform, payload: &[u8], p: &Params) -> WorkloadResult {
    let stop = Arc::new(AtomicBool::new(false));
    let bg_ops = Arc::new(AtomicU64::new(0));
    let permits = Arc::new(tokio::sync::Semaphore::new(p.callback_inflight));
    for _ in 0..4 {
        let platform = platform.clone();
        let payload = payload.to_vec();
        let timeout = p.timeout;
        let stop = stop.clone();
        let bg_ops = bg_ops.clone();
        let permits = permits.clone();
        tokio::spawn(async move {
            let po = PostOffice::new(&platform);
            while !stop.load(Ordering::Relaxed) {
                let permit = permits.clone().acquire_owned().await.expect("semaphore");
                let po = po.clone();
                let payload = payload.clone();
                let bg_ops = bg_ops.clone();
                tokio::spawn(async move {
                    let event = EventEnvelope::new()
                        .set_to(WORKER)
                        .set_raw_body(rmpv_binary(&payload));
                    let _ = po.request(event, timeout).await;
                    bg_ops.fetch_add(1, Ordering::Relaxed);
                    drop(permit);
                });
            }
        });
    }
    // let the backlog build past the memory buffer and start spilling
    tokio::time::sleep(Duration::from_millis(750)).await;

    let po = PostOffice::new(platform);
    let mut samples = Vec::with_capacity(p.probe_ops as usize);
    let mut failures = 0u64;
    let started = Instant::now();
    for _ in 0..p.probe_ops {
        let s = Instant::now();
        let event = EventEnvelope::new()
            .set_to(PROBE)
            .set_raw_body(rmpv_binary(payload));
        match po.request(event, p.timeout).await {
            Ok(_) => samples.push(s.elapsed().as_nanos() as u64),
            Err(_) => failures += 1,
        }
        if p.probe_pace_micros > 0 {
            tokio::time::sleep(Duration::from_micros(p.probe_pace_micros)).await;
        }
    }
    let elapsed = started.elapsed().as_secs_f64();
    stop.store(true, Ordering::Relaxed);

    let params = vec![
        (
            "probe".to_string(),
            format!("{PROBE} (paced RPC, in-flight 1)"),
        ),
        (
            "probe pacing".to_string(),
            format!("{} µs", p.probe_pace_micros),
        ),
        (
            "background".to_string(),
            format!("flood → {WORKER}, up to {} in-flight", p.callback_inflight),
        ),
        (
            "background ops (concurrent)".to_string(),
            bg_ops.load(Ordering::Relaxed).to_string(),
        ),
        ("consumers".to_string(), p.consumers.to_string()),
    ];
    WorkloadResult {
        name: "Latency probe under background flood".to_string(),
        category: MIXED.to_string(),
        description: format!(
            "The real production question: a latency-sensitive probe — a paced RPC on a SEPARATE \
             route ({PROBE}) — measured WHILE a background callback flood hammers {WORKER}'s \
             ElasticQueue. The manager-task dispatch runs the spill OFF the shared executor \
             threads, so the probe should stay fast. Watch the probe's p99.9/max."
        ),
        params,
        total: p.probe_ops,
        failures,
        elapsed_sec: elapsed,
        stats: Stats::compute(samples),
    }
}

fn rmpv_binary(payload: &[u8]) -> rmpv::Value {
    rmpv::Value::Binary(payload.to_vec())
}

fn environment(p: &Params) -> Vec<(String, String)> {
    let config = AppConfigReader::get_instance();
    vec![
        ("Generated".to_string(), trace::iso8601_utc_now()),
        ("Rust".to_string(), env!("BENCH_RUSTC_VERSION").to_string()),
        (
            "Memory management".to_string(),
            "ownership / borrow checker — no GC, no GC pause".to_string(),
        ),
        (
            "OS".to_string(),
            format!("{} / {}", std::env::consts::OS, std::env::consts::ARCH),
        ),
        (
            "Available processors".to_string(),
            std::thread::available_parallelism()
                .map(|n| n.get().to_string())
                .unwrap_or_else(|_| "?".to_string()),
        ),
        (
            "ElasticQueue store".to_string(),
            "file (the only store in the Rust port)".to_string(),
        ),
        (
            "Dispatch".to_string(),
            "per-route manager task (off-loop by design)".to_string(),
        ),
        (
            "transient.data.store".to_string(),
            config.get_property_or("transient.data.store", "/tmp/reactive"),
        ),
        (
            "mercury (Rust) version".to_string(),
            env!("CARGO_PKG_VERSION").to_string(),
        ),
        (
            "bench.timeout".to_string(),
            format!("{} ms", p.timeout.as_millis()),
        ),
    ]
}

fn print_summary(env: &[(String, String)], results: &[WorkloadResult]) {
    let mut out = String::from("\n===== benchmark-reporter (Rust) =====\n");
    for (key, value) in env {
        out.push_str(&format!("  {key:<24} {value}\n"));
    }
    let mut last_category = "";
    for r in results {
        if r.category != last_category {
            out.push_str(&format!("\n-- {} --\n", r.category));
            last_category = &r.category;
        }
        let s = &r.stats;
        out.push_str(&format!("\n{}\n", r.name));
        out.push_str(&format!(
            "  throughput={:.0} ops/s  ok={}  failures={}  elapsed={:.2}s\n",
            r.throughput(),
            s.count,
            r.failures,
            r.elapsed_sec
        ));
        out.push_str(&format!(
            "  latency ms: mean={:.3} p50={:.3} p90={:.3} p99={:.3} p99.9={:.3} p99.99={:.3} max={:.3}\n",
            s.mean_ms, s.p50, s.p90, s.p99, s.p999, s.p9999, s.max_ms
        ));
    }
    out.push_str("==============================\n");
    print!("{out}");
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    resources::prepend_resource_root(concat!(env!("CARGO_MANIFEST_DIR"), "/resources"));
    platform_core::logging::init();
    let p = Params::load();
    AppStarter::new()
        .preload(WORKER, Arc::new(Worker), p.consumers)
        .preload(PROBE, Arc::new(Worker), p.consumers.min(8))
        .main_application(1, Arc::new(BenchmarkApp))
        .run(std::env::args().collect())
        .await?;
    elastic_queue::shutdown_cleanup();
    Ok(())
}
