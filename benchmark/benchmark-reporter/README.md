# benchmark-reporter (Rust)

The Rust port of the mercury-composable `benchmark-reporter`: a self-contained,
single-process end-to-end performance harness for **platform-core**. It registers one echo
service (with `bench.consumers` worker instances) and runs the same **six-scenario suite**
as the Java original, then writes a **self-contained HTML report** (inline SVG histogram +
percentile plot + a statistics table + environment metadata — no external files).

**Normal operation (no back-pressure)** — arrival rate ≤ capacity; the ElasticQueue stays
within its 20-event in-memory buffer:
- **RPC 1 → C** — one publisher, in-flight = 1: baseline request/reply round-trip.
- **RPC C → C** — balanced: throughput reaches the single-route dispatch ceiling.
- **Callback C → C (paced)** — async at a sustainable rate: latency ≈ service time.

**Overload (back-pressure engaged)** — backlog exceeds the buffer and spills to disk;
the system stays stable and loss-free:
- **RPC 2C → C** — over-subscribed 2:1: back-pressure via oversubscription.
- **Callback flood → C** — open-loop, in-flight ≫ 20: latency becomes queue-bounded.

**Mixed workload (latency isolation under load)**:
- **Latency probe under background flood** — a paced RPC on a *separate* route measured
  *while* a flood hammers the worker's ElasticQueue. The per-route manager-task dispatch
  keeps the spill off the shared executor, so the probe stays fast.

## Run

```bash
# release build matters — this is a benchmark
cargo run -r -p benchmark-reporter
# parameters are -D runtime arguments (the JVM -D analog), AFTER `--`
cargo run -r -p benchmark-reporter -- -Dbench.report=/tmp/report.html -Dbench.ops=50000
```

The report is written to `bench.report` (default `/tmp/benchmark-report.html`) and a
summary prints to stdout; the process exits when done. Committed reference records live in
[`analysis/`](analysis/README.md).

### Parameters (all optional; `-D` runtime arguments)

| property | default | meaning |
|---|---|---|
| `bench.ops` | 200000 | timed operations per scenario |
| `bench.warmup` | 20000 | warm-up operations per path (discarded) |
| `bench.payload` | 256 | request body size in bytes |
| `bench.consumers` | 50 | worker instances `C` |
| `bench.callback.pacing.micros` | 1000 | per-publisher pause in the paced callback scenario † |
| `bench.callback.inflight` | 2000 | flood-scenario max in-flight |
| `bench.callback.producers` | 1 | flood-scenario producer tasks |
| `bench.probe.ops` | 3000 | mixed-workload probe requests |
| `bench.probe.pacing.micros` | 2000 | pause between probe requests |
| `bench.timeout` | 30000 | per-request timeout (ms) |
| `bench.report` | /tmp/benchmark-report.html | output HTML path |

† pacing rides tokio's timer (~1 ms granularity), so sub-millisecond pacing values pace
coarser than the Java `parkNanos` original — the paced scenario's throughput is
rate-limited by design; compare latency, not throughput, on that row.

## Scope

Single store (the file-backed ElasticQueue — the Berkeley DB fallback was deliberately not
ported), single process, in-process event bus only — no REST automation, no external load
generator.
