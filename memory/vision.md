# Vision — mercury

> The north star: the target future state of this repo. **Set by the maintainer at enable
> (2026-07-15)** in direct response to the greenfield Vision prompt — not inferred. Treated
> as `core` (never decays) but re-confirmed on the invariant-verification cadence (a vision
> can go stale). The **Blueprint** (Open Threads tagged `(blueprint)` in `continuity.md`)
> tracks the gap from Current State to here; Designs and Implementations trace back to this
> `id`. See `DECAY.md` §12.
>
> <!-- id: vision-mercury | created: 2026-07-15 | last_used: 2026-07-15 | uses: 1 | tier: core -->

## Elevator statement

A **Rust port of `mercury-composable`** — Accenture's event-driven, composable application
platform, whose canonical implementation is in Java — carrying the **same product vision**,
re-delivered in Rust **step by step from the foundation up to the user interface**.

## Current-state context

> Refreshed 2026-07-21 at the invariant re-verify ceremony (maintainer-confirmed; the
> north star below is unchanged). Original enable-time context kept for the record.

Greenfield at enable 2026-07-15 (only a placeholder `README.md`), delivered from a
**private prototyping repo** chosen deliberately to keep rapid, noisy iteration away from
public readers. That stage is complete: all **three core layers are ported and
milestone-closed** (platform-core, event-script, active knowledge graph + Playground), and
the work **GRADUATED to the official repo `github.com/Accenture/mercury` on 2026-07-20**.
Forward runway: continue foundation → user interface (`bp-foundation-to-ui`) and the
lightweight-connectors backlog (see Non-goals).

The **authoritative, detailed vision is the official `mercury-composable` (Java) project** —
this repo inherits that vision, scoped to a Rust re-implementation. We *map, don't mirror*:
treat the Java project as canonical rather than copying its docs here.

- The ultimate purpose it inherits from mercury-composable: **AI-assisted Semantic
  Application Development** — where the **Active Knowledge Graph *is* the application**
  (intent + knowledge + behavior as one executable model), with Event Script and Platform
  Core as the foundation beneath it. mercury re-reaches that destination in Rust.

## What it should become

- A **faithful Rust re-implementation of `mercury-composable`**, preserving its purpose and
  behavior, built **bottom-up: foundation → user interface**.
- Specifically, the **three core layers** ported to Rust (names + roles per
  mercury-composable; each layer's authoritative spec lives in the Java project — map, don't
  mirror):
  1. **platform-core** — the event-driven foundation. Self-contained **functions** (actors)
     addressed only by **route name**, exchanging immutable **`EventEnvelope`** messages over
     an **in-memory event bus**; a **`PostOffice`** RPC/messaging client; a **`Platform`**
     registry. No direct calls between functions (decoupling is the point). Java uses Vert.x +
     Java 21 virtual threads; the Rust port uses its own async runtime (foundation increment,
     `bp-platform-core`).
  2. **event-script** — composable orchestration: a **YAML DSL** that sequences functions for
     a transaction (a per-transaction state-machine `model`, `input/output` data mapping,
     task execution types), so orchestration is configuration, not code.
  3. **active knowledge graph** — the semantic layer where a **graph model executes behavior**
     (skills embedded on nodes during traversal); zero imperative code for the common case.
- **Ready to graduate to the official Accenture repo** once the foundation is sufficient.

## For whom

- The Accenture team porting mercury-composable to Rust — rapid-prototyping in this private
  repo now, the wider audience once it moves to the official repo.
- Ultimately the same audience mercury-composable (Java) serves: developers building
  composable, event-driven applications on the platform.

## Success criteria

- The three layers (platform-core → event-script → active knowledge graph) are ported to
  Rust and behave **faithfully to the Java original**, within the reduced scope below.
- Delivery is **incremental and traceable** — foundation first, then upward — each increment
  building on a solid base (VBDI: intent → design → implementation, with low drift).
- The foundation is solid enough to **move the repo to the official Accenture repo**.

## Non-goals (what it must never become)

- **The Kafka service mesh is NOT ported** — deliberately out of scope, for simplicity: the
  service-discovery and sync-over-Kafka layer, i.e. the whole `connectors/` tree
  (`cloud-connector`, `service-monitor`, `kafka-connector`, `kafka-presence`). The core stays
  single-runtime on the in-memory event bus. **Refined by the maintainer 2026-07-20:**
  `minimalist-kafka` and `twin-kafka` are *lightweight, cloud-native connectors* — NOT part of
  the mesh exclusion — and will be ported in future iterations, along with `sync-over-async`
  (tracked as backlog in `continuity.md`).
- Inherited from mercury-composable (confirm against the Java project as the port matures):
  - **Never couples functions directly** — coupling stays route-name + `EventEnvelope` only.
    (This is an Architectural Invariant, tracked in `continuity.md`.)
  - **Not** a general-purpose graph database / OLAP engine — the graph drives *execution and
    decisioning*, not storage-scale querying.
  - **Not** a heavyweight runtime — stays lightweight; no mandatory framework in the core.
  - **Not** a "no code ever" dogma — zero-code is the default; Event Script + custom skills
    remain the escape hatch.

## Mental model

> The same destination as mercury-composable (Java), re-reached in Rust — built bottom-up
> (platform-core → event-script → active knowledge graph → UI), minus the Kafka service mesh.
> Prototyped privately until the foundation was ready; now living at the official Accenture
> repo (`github.com/Accenture/mercury`, since 2026-07-20) — one product vision, two languages.
