# Graph Edge Crossing Minimization Spec

**Status:** Approved for implementation by the user's 2026-07-30 fix request
**Run:** `edge-crossing-20260730-01`
**Classification:** Non-durable presentation change inside the existing Graph View boundary
**Serves:** `vision-mercury-composable` — graph execution remains inspectable and explainable

## 1. Delivery contract

When a graph is loaded or refreshed, a graph author or viewer should get a stable
left-to-right layout that removes avoidable line crossings and keeps long edges out
of intermediate node bodies wherever the layered topology permits it.

“Minimize” is intentionally best-effort. Non-planar graphs, cycles, and layouts changed
manually after rendering cannot be guaranteed to have zero crossings.

### Acceptance criteria

- **AC1 — Adjacent-layer crossings:** tutorial-3 and representative planar fork/join
  graphs render with no avoidable edge-edge crossings.
- **AC2 — Long-edge corridors:** tutorial-4's `less-than -> end` edge does not pass
  through the non-incident `decision` node. Long edges participate in ordering at
  every intermediate rank.
- **AC3 — Compatibility:** every input node and connection is preserved one-for-one,
  including endpoints, relation labels, arrows, authoring handles, disconnected
  component ordering, segregated orphan rows, and cycle/back-edge routing.
- **AC4 — Determinism:** repeated transforms and reordered equivalent node/connection
  arrays produce the same node positions. Tie-breaking is explicit.
- **AC5 — Render-time cost:** optimization uses a fixed number of passes, has no
  convergence loop, and remains practical for a 1,000-node / 1,900-edge synthetic graph.

### Non-goals

- Proving that every arbitrary graph is planar or rendering every graph with zero crossings.
- Adding or inferring semantic graph connections from node property text.
- Persisting automatic or user-dragged positions.
- Dynamically re-running layout after a user drags or resizes a node.
- Replacing React Flow's Bezier edge renderer with a new obstacle-routing component.
- Changing graph JSON, REST, WebSocket, or command contracts.

## 2. Pre-change behavior and evidence

`GraphView.tsx` derives React Flow nodes and edges by calling `transformGraphData`
when `graphData` changes. Before this change, `graphTransformer.ts`:

1. classifies connected flow nodes and segregated orphans;
2. finds cycle back-edges;
3. splits and semantically orders connected components;
4. assigns longest-path-like left-to-right levels;
5. sorts aliases alphabetically inside every level;
6. sorts each node's handles by peer Y position; and
7. emits built-in React Flow Bezier edges.

Alphabetical level ordering ignores the connections between levels. The measured
pre-change baselines are:

| Fixture | Conflict |
|---|---:|
| `tutorial-3.json` | 2 edge-edge crossings |
| `tutorial-4.json` | 1 edge-through-node intrusion |
| `tutorial-5.json` | 4 edge-edge crossings |
| `tutorial-6.json` | 3 edge-edge crossings |
| `tutorial-9.json` | 1 edge-edge crossing |
| `tutorial-12.json` | 4 edge-edge crossings |

The current transform takes a median 5.94 ms and a measured maximum 7.74 ms over
eight runs for a synthetic 1,000-node / 1,900-edge adjacent-rank graph. This is a
comparison baseline, not a contractual latency threshold.

## 3. Selected design

Keep the existing ranking, component, orphan, cycle, handle, and Bezier policies.
Replace alphabetical-only within-rank placement with a deterministic layered
crossing-reduction pass:

1. Sort traversal adjacency so cycle classification does not depend on payload order.
2. Keep real nodes at their existing levels.
3. Expand each forward edge spanning multiple levels into a chain of presentation-only
   virtual slots, one per intermediate level, while the expanded graph stays within a
   deterministic 10,000-segment work budget.
4. Run a fixed number of downward and upward barycentric sweeps. Each slot is ordered
   by the average position of its adjacent-rank neighbors, with previous order and a
   stable key as tie-breakers.
5. For small candidates, run a depth-two, evaluation-capped local insertion search so
   a barycentric plateau does not leave a simple avoidable crossing.
6. Score candidates lexicographically by actual forward-Bezier/node intrusions and
   then adjacent-rank inversions. Geometry work uses a precomputed list capped at
   2,000 eligible edge-node pairs; shared sources and targets are not inversions.
7. Give virtual slots normal node-height clearance during Y assignment, but do not
   emit them as React Flow nodes. If virtual-slot optimization would introduce more
   node intrusions than the real-node baseline, retain the baseline.
8. Continue sorting rendered handles by final peer Y position, adding deterministic
   peer/index tie-breakers.

The virtual slots exist only during `transformGraphData`. The server graph JSON remains
the sole source of truth, and the output still contains exactly the original nodes and
connections.

### Complexity

Let `S` be the number of adjacent-rank edge segments after long-edge expansion. With a
fixed sweep count, time is bounded by sorting and scoring the ranks and segments rather
than an open-ended optimizer; memory is `O(V + S)`. Expansion falls back to real-node
ordering above 10,000 segments. Bezier/body scoring consumes at most 2,000 precomputed
pairs, and depth-two local search is enabled only for at most 64 slots / 256 segments
with 512 candidate evaluations. Component-local connections are bucketed once instead
of rescanning the full edge list per disconnected component.

## 4. Alternatives

### A. Neighbor-aware sort without virtual slots

This is the lightest code change and fixes tutorial-3, but a long edge remains invisible
to intermediate ranks, so it does not satisfy AC2.

### B. Existing transformer plus layered sweeps and virtual slots — selected

This adds no dependency or rendering component, preserves project-specific component and
cycle policies, and addresses both screenshots through one presentation-only algorithm.

### C. Dagre/ELK plus routed custom edges

This offers a larger layout/routing system but adds a runtime dependency, bundle weight,
edge component wiring, and adaptation work for the existing orphan rows, component order,
resizing, authoring handles, and back-edge policy. It is disproportionate to this fix.

The selected option is Pareto-preferred here on compatibility risk, dependency count,
review surface, and preservation of existing behavior. Option C should be revisited only
if future requirements include orthogonal obstacle routing after arbitrary user movement.

## 5. Boundary, lifecycle, and failure behavior

### Source of truth

| Concept | Owner | Derived copy / synchronization |
|---|---|---|
| Nodes and relations | fetched `MinigraphGraphData` | React Flow arrays recomputed by `useMemo` |
| Automatic positions | `transformGraphData` result | replaced whenever upstream graph data changes |
| User drag/resize state | React Flow local state | intentionally reset on graph refresh |
| Virtual slots | one transform invocation | discarded before output |

No API, persistence, process, async, or security boundary changes. Untrusted graph payloads
continue through the existing fetch/type guard and Graph View error boundary.

### Failure matrix

| Case | Required behavior |
|---|---|
| Empty graph | Existing empty state; no layout work |
| Orphan-only graph | Existing segregated rows |
| Disconnected flows | Existing root/default/end component order |
| Cycle/self-edge | Exclude DFS back-edge from ranking; still emit it on reverse-side handles |
| Parallel/shared-endpoint edges | Preserve all; do not count a common endpoint as a crossing |
| Non-planar topology | Deterministic lowest candidate found; never claim zero |
| Reordered payload | Stable positions and cycle classification |
| Large graph | Fixed passes terminate; expansion/geometry/local-search budgets degrade deterministically to best-effort ordering |
| Invalid endpoint | Existing transform error path remains unchanged |
| User drag/resize | No automatic re-layout until upstream graph refresh |

### Rollout and rollback

There is no feature flag because this is a local, reversible rendering correction with no
data or public contract change. Rollback reverts the transformer, its focused tests, and
this spec. The tracked production bundle is regenerated separately when the maintainer
next runs the release build.

## 6. Implementation slices

1. Add output-geometry and preservation tests for tutorial-3, tutorial-4, synthetic
   fork/join, local-minimum, long-edge, cycle, shuffled-order, parallel-edge,
   adversarial-budget, and unavoidable-crossing cases.
2. Add deterministic virtual-slot expansion, fixed barycentric sweeps, bounded local
   search, actual Bezier/body scoring, and guarded fallback inside `graphTransformer.ts`.
3. Run focused/full webapp tests and type checking. Per the user's “不需要跑build”
   direction, do not regenerate the tracked production bundle in this run.

## 7. Verification matrix

| Claim | Evidence |
|---|---|
| AC1 tutorial-3 crossing removal | Public transformed geometry test |
| AC2 tutorial-4 node clearance | Public transformed Bezier/node-rectangle test |
| AC3 semantic preservation | Exact node/edge endpoint/relation/handle assertions |
| AC3 cycle/component/orphan compatibility | Focused topology tests plus full suite |
| AC4 deterministic layout | repeat and shuffled-input normalized-position tests |
| AC5 bounded practical cost | explicit work caps plus adversarial chain/skip-edge regression |
| Type/test integration | `npm run typecheck` and full `npm test` |
| Generated backend assets | intentionally deferred at the user's direction; no stale bundle included |
| User-visible result | output-based XYFlow Bezier crossing/body-intersection assertions |

## 8. Implementation evidence

Artifact revision `r6` implements the selected design in `graphTransformer.ts` without
changing the React Flow edge type or adding a dependency.

- Eighteen focused geometry/compatibility cases pass. Tutorial-3/5/6/9/12 and the
  connected synthetic inversion now have zero measured edge crossings and node
  intrusions; tutorial-4, `hello`, and `unit-test-task-4` have no non-incident
  node intrusion.
- Reviewer-supplied regressions cover a barycentric local minimum, two direct-Bezier
  node intrusions, reordered seedless cycles, parallel edges, and a 300-node chain
  with 597 skip/adjacent edges. The adversarial case completes in about 27 ms in the
  focused test run and does not allocate unbounded virtual corridors.
- The K3,3 negative control retains crossings while preserving all nodes and edges,
  so the suite does not equate “best effort” with an impossible universal zero.
- Repeated and reordered equivalent payloads, including a seedless cycle, produce
  identical normalized positions and back-edge classification.
- The full frontend suite passes 170 tests, `npm run typecheck` passes, and three
  independent re-review lenses report no remaining blocker/high/medium finding.
- The earlier `r4` timing for the 1,000-node / 1,900-edge adjacent-rank graph was
  12.91 ms median and 22.85 ms maximum over eight runs. A 250-node / 294-edge
  long-skip-edge graph is 6.90 ms median and 8.43 ms maximum over five runs.
  Final hardening adds explicit caps so pathological expansion and geometry scoring
  degrade instead of growing without bound.

## Appendix - UI Loop Engineer Planning Artifact

**Phase 0 - Domain calibration**
- Domain mix: Frontend UI/rendering 45%; algorithm/pure logic 45%; product workflow/UX 10%.
- Dominant failure mode: An over-complex layout algorithm or weak geometric invariants could trade visible crossings for jitter, excessive whitespace, or render-time cost.
- Pre-mortem watch-items: algorithm overshoot; regression of the prior component/cycle/handle policies; fixing edge-edge inversions while leaving long edges through nodes.
- Calibration: Emphasize compatibility, public-output geometry, deterministic ordering, and per-render performance.

**Step 1 - Requirement and journeys**
- Actor / trigger / success signal: Graph author/viewer loads or refreshes a graph and sees fewer avoidable crossings without lost semantics.
- Primary journeys: tutorial-3 fork/join; tutorial-4 unequal path length; cyclic/non-planar graph; refreshed equivalent payload; empty/disconnected graph.
- Non-goals: Zero crossings for every graph, inferred semantic edges, persisted positions, post-drag routing, public contract changes.

**Step 2 - Research and primitive budget**
- Framework/platform sources: `GraphView.tsx:129-169,280-340`; `graphTransformer.ts:193-676`; `NodeTypes.tsx:37-120`; `@xyflow/system` Bezier implementation; tutorial JSON fixtures.
- Applicable primitive levels: external Dagre/ELK; existing React Flow renderer plus custom layered layout; local neighbor sort; static/manual positions; do nothing.
- Lightest workable primitive: Neighbor-aware within-rank sort, rejected because it cannot reserve a long-edge corridor.

**Step 3 - Anti-anchoring and compatibility**
- Silent assumptions: left-to-right ranks, connected-node flow classification, root/default/end component order, orphan rows, reverse-side cycle handles all remain valid; alphabetical within-rank order is discarded.
- Fresh-team delta: A fresh team might adopt a complete graph-layout library; this repo retains its custom layout because it already owns domain-specific grouping and authoring behavior.
- Existing behavior contract: Node/edge semantics, components, orphans, cycles, handles, dragging, refresh, and render errors remain unchanged; only initial ordering/spacing is extended.

**Step 4 - Three architectures**
- Option A - root primitive: Local barycentric sort of real nodes only.
- Option B - root primitive: Existing transformer with virtual-slot layered sweeps.
- Option C - root primitive: Dagre/ELK layout plus routed custom edges.
- Picked: Option B.
- Axis spread verified: Options vary by dependency, abstraction level, long-edge representation, and rendering strategy.

**Step 5 - Source of truth and boundaries**
- Source-of-truth inventory: Server graph JSON owns semantics; transformer owns ephemeral automatic positions; React Flow owns post-render interaction state.
- Boundary map: Data-model-to-presentation boundary only; no new API, process, persistence, async, trust, or ownership boundary.

**Step 6 - Contracts**
- Boundary-crossing contracts: `MinigraphGraphData` input and React Flow node/edge output remain unchanged.
- Shared constants / schema strategy: Existing TypeScript interfaces and handle helpers remain the compile-time contract; virtual slots do not escape.

**Step 7 - Complexity budget**
- Budget exceptions and justification: None; zero new runtime dependencies and zero new protocols.
- Feature-disable and per-N results: React Flow remains used for its existing graph interaction/rendering role; fixed sweeps replace any convergence loop. Expansion, geometry pairs, and local search each have deterministic work caps. Baseline 1,000/1,900 transform median is 5.94 ms.
- Authority-claim audit: Claims are limited to inspected local source and measured fixtures; red-flag phrase scan is clean.

**Step 8 - Purity discipline**
- Pure components claimed: Layout ordering and `transformGraphData`.
- Mechanical enforcement or not applicable: No time, random, network, storage, DOM, or mutable module state; repeat/shuffle tests enforce determinism.

**Step 9 - Primitive fit and composition**
- Under-use / over-use findings: React Flow supplies paths and interaction but no node layout/obstacle inspection; retaining the existing pure transformer is the fitting boundary.
- Composed primitives: Transformer positions/handles + React Flow Bezier rendering + node drag/resize.
- Overlaps: Initial geometry can become stale after manual movement.
- Guardrails: Scope and tests cover initial/refresh layout only; no hidden post-drag re-layout.

**Step 10 - From-scratch comparison**
- Materially simpler?: No. A full layout library reduces local algorithm code but increases dependency, adapter, regression, and bundle surfaces.
- If yes, redesign adopted: Not applicable.

**Step 11 - Failure and lifecycle**
- Failure matrix summary: Empty, orphan, disconnected, cyclic, parallel, non-planar, reordered, large, invalid, and manually moved graphs are covered above.
- Lifecycle data-access findings: Layout reads immutable fetched graph data during `useMemo`; output replaces React Flow state in the existing effect; virtual state ends with the call.

**Step 12 - Spike / performance**
- Spike required?: Yes, because layout runs synchronously per graph-data render and input size is not explicitly bounded.
- Result or reason not required: Baseline 1,000 nodes / 1,900 edges: median 5.94 ms, max 7.74 ms over eight measured transforms; post-change comparison is required.
- Per-N cost: Fixed passes over `V` and expanded adjacent-rank segments `S`; no per-node subscription, timer, or DOM observer is added.

**Step 13 - Security / trust**
- Input boundaries: Existing fetched graph JSON only.
- Validation / auth / encoding: Existing fetch/type guard and render-error handling remain; the change performs no evaluation, HTML generation, authorization, or data exposure.

**Step 14 - Observability / rollout**
- Debuggability: Existing `GraphView` transform error callback remains; focused fixtures make geometry regressions reproducible.
- Rollout / rollback: Source/tests/spec are reversible together; generated-bundle refresh is deferred at the user's direction.

**Step 15 - Verification**
- Verification mapping summary: Adaptive output geometry, semantic preservation, deterministic transforms, topology regressions, bounded-work regression, full tests, and typecheck.
- Design cross-reference complete: Yes; every AC maps to Section 7 evidence.

**Step 16 - Implementation / consumer readiness**
- Implementation slices: Tests; pure layout algorithm; integrated test/type verification. Bundle refresh intentionally deferred.
- Review questions / decision log: Verify crossing metric exclusions, virtual-slot clearance, cycle determinism, dense-long-edge cost, and no unintended authoring regression. Revisit a layout library only for dynamic obstacle routing.

**Step 17 - Pre-draft self-check**
- All self-check answers yes?: Yes (11/11).
- Enumeration completeness: Slot creation/mutation/discard, ordering/uniqueness/immutability/cardinality assumptions, and empty/error/no-op feedback are explicit.
- Cross-iteration regression: Preserve prior cycle detection, component sorting, peer-Y handle spreading, resize behavior, and hardened source/target authoring handles; discard only alphabetical rank order. No prior review finding is reintroduced.
