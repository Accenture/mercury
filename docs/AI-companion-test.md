# AI-Companion Test Log

> A running record of the **human–AI collaboration** validation exercise: can a *fresh* AI agent,
> given only the canonical AI-agent documentation (no tutorial walkthroughs), operate the MiniGraph
> Playground and help a human build correct knowledge graphs? Each stage raises the cognitive
> demand. The point is not just pass/fail — it is to surface where `llms.txt`, the AI grammar
> (`command-reference.md` / `minigraph-commands.json`), and the human docs can be improved, and to
> harden the Rust port toward production quality along the way.
>
> Tracked as Open Thread `ot-companion-validation-sweep` in `memory/continuity.md`.

## Method

- **Fresh agent per test** — a new sub-agent with no prior context, briefed only with a task and a
  pointer to the canonical AI-agent docs:
  `mercury-composable/docs/llms.txt`, `docs/guides/knowledge-graph/ai-agent-guide.md`,
  `command-reference.md`, `minigraph-commands.json`. **Never** the interactive `help/*.md` or the
  tutorial walkthroughs.
- **The companion drives the live session** over the documented endpoints:
  `POST /api/companion/{id}` (one command per request), `GET /api/graph/session/{id}` (read the
  model), `GET /api/inspect/{id}/{key}` (read state after a run).
- **The human validates** in the Playground UI (screenshots) and via a behavioral **dry-run**.
- The orchestrator (Claude Code) may privately consult the canonical tutorial to *judge* correctness
  and assist if the companion is genuinely stuck, but never leaks it into the briefing.

## Validation ladder

| Level | Tutorials | What it tests | Briefing | Pause model | Acceptance |
|---|---|---|---|---|---|
| **L1 — mechanics + transcription** | 1–2 | drive the endpoints; look up exact syntax for a *known* graph | command list / syntax reference | step-wise | commands dispatch; graph built |
| **L2 — comprehension** | 3 | reconstruct a *specified* graph from the canonical docs alone (no walkthrough) | full node/connection plan; derive syntax | step-wise, screenshot each step | structural match to canonical + dry-run |
| **L3 — synthesis / problem-solving** | 4 | *design* a correct solution to a stated **problem + output contract**, with **no syntax hints** | problem statement only | build whole graph, pause for dry-run | **behavioral**: dry-run honors the contract on every branch |
| **L4+ …** | 5–13 | escalate further (composition, error paths, extensions, …) | TBD per tutorial | TBD | TBD |

---

## Stage records

### Tutorials 1–2 — L1 (mechanics + transcription) — PASSED
- **Method:** fresh companion agents derived the exact commands from `minigraph-commands.json` and
  built the tutorial graphs, one command at a time with human screenshot validation.
- **Result:** proved a fresh agent can (a) use the companion endpoint contract, (b) read the model
  back over the session endpoint, and (c) translate the command grammar into valid commands.
- **Doc/grammar insight:** the companion hint set should point at `docs/llms.txt` +
  `docs/guides/knowledge-graph/ai-agent-guide.md` (the AI-facing docs), not the interactive help.

### Tutorial 3 — L2 (comprehension) — PASSED
- **Task:** the Data-Dictionary method — source a person's name + address from an MDM Provider via
  two Dictionary entries + a Fetcher, keyed by `person_id`.
- **Method:** fresh companion, canonical docs only, **step-wise** with a screenshot pause per node
  and per connection.
- **Result:** built **7 nodes + 7 connections**, an **exact structural match** to the canonical
  `tutorial-3.json` (verified by an independent nodes/edges diff); dry-run with `person_id=100`
  returned `output.body = {name:"Peter", address:"100 World Blvd"}`.
- **Loose ends surfaced (engine/parity), all fixed (increments 30–32):**
  1. `@OptionalService` only gated `#[preload]` → extended to `#[websocket_service]` /
     `#[before_application]` / `#[main_application]`; the Playground was retired from programmatic
     registration to **declarative dev-gating** (increment 31, Java parity).
  2. The dev mock data providers (`mock.mdm.profile`, …) weren't ported → added, dev-gated
     (increment 30).
  3. A Dictionary node's bare `input[]=person_id` was wrongly rejected by the create/update console
     command → node-type-aware fix (Java first, then Rust).
- **Doc/grammar insight (increment 32):** the `inspect` command docs used `{…}` as a placeholder in
  the *syntax* line but **repeated the braces in the examples**, so the companion sent
  `inspect {output.body}` — which both engines resolve to the key `{output`→`body}` = empty. Fixed:
  **examples unbraced, braces only in syntax lines, + a placeholder-convention note** in
  `command-reference.md`, `ai-agent-guide.md`, `help inspect.md`, `minigraph-commands.json`.
  A first pass also changed the webapp autocomplete *template* — reverted, because a `template`
  field is a fill-in placeholder (`{variable_name}` is correct there), not an example. **Lesson: in
  the docs, `{…}` must appear only where a value is substituted, never in text meant to be typed
  verbatim.**

### Tutorial 4 — L3 (synthesis / problem-solving) — FAILED → doc fix → RE-VERIFIED PASS
- **Task (problem only, no syntax hints):** input has two parameters `a` and `b`; build a graph that
  compares them and branches two ways (`a >= b` and `a < b`). Output contract:
  `{ "message": <describes the outcome>, "less_than": <bool>, "sum": <a + b> }`. Tutorial-4's focus
  is the **decision node**.
- **Method:** fresh companion, canonical docs only, **no** node/type/mapping/`input.body.*` hints;
  built the whole graph then paused for the dry-run (step-wise pauses dropped — mechanics proven at L2).
- **Design produced (companion's own — a *valid alternative* to the canonical `Decision` node):**
  `root → compare (graph.math: sum + branch) → {ge-path | lt-path} (graph.data.mapper) → end
  (graph.data.mapper, assembles the contract)`. 5 nodes, 5 connections, all commands HTTP 200 —
  the build succeeded and read back exactly as intended.
- **Dry-run: FAILED.** `a=5,b=3` seeded fine (`input.body={a:5,b:3}`) but the run aborted at the
  `compare` node with the WS-console error:
  > `Walk to root` → `Walk to compare` → **`node compare does not have if:, then: or else:`** →
  > `Graph traversal aborted`
  Read-back confirmed total failure: `model = {}`, `output.body = Not found` (not even `COMPUTE`
  ran). The companion is **blind** to this — its POSTs returned HTTP 200 "accepted"; the parser
  error only reached the WS console, and its read-back endpoints just showed empty state.
- **Root cause — an AI-grammar gap.** The companion invented `IF: … / BEGIN / NEXT: <relation> /
  END`. The engine's real `graph.math` decision grammar is the multi-line triad
  `IF: <bool>` / `THEN: <node|next>` / `ELSE: <node|next>`. Every wrong guess was one the docs
  *caused*: `NEXT:` actually takes a **node name** (not a relation label); `BEGIN`/`END` are
  **`for_each` iteration** delimiters (not IF-block braces); and **`THEN:`/`ELSE:` appear nowhere in
  the AI-facing docs**. The AI docs (`command-reference.md`, `minigraph-commands.json`) *listed* the
  keywords (`COMPUTE`/`IF`/`MAPPING`/`EXECUTE`/`RESET` + `NEXT:`/`BEGIN`/`END`) but gave **no
  statement syntax** — the full grammar lived only in the interactive `help graph-math.md`, which the
  companion was (by design) forbidden to read. A keyword inventory without syntax was *worse than
  silence*: `BEGIN`/`END`/`NEXT:` were misleading breadcrumbs.
- **Doc fix (this session):** promoted the `graph.math` / `graph.js` **statement grammar** into the
  AI-facing docs — the `IF`/`THEN`/`ELSE` multi-line form (with a worked branch example),
  `NEXT:` = node-name, `BEGIN`/`END` = `for_each` scope, `COMPUTE` → `result` namespace,
  `MAPPING`/`EXECUTE`/`RESET`. Files: `command-reference.md` (new "statement grammar" subsection) +
  `minigraph-commands.json` (`graph.math` `statement_syntax` + notes). _(Java-canonical only — the
  Rust repo doesn't carry these guides; its `help graph-math.md` already had the full grammar.)_
- **Verification — PASSED (fix proven).** A *second* fresh companion (no knowledge of the failure or
  the doc change), same problem, now read the documented grammar and built a **runnable decision**:
  `compare` (`graph.math`) with `COMPUTE: sum -> a+b`, `COMPUTE: less_than -> a<b`, and the real
  triad `IF: a>=b / THEN: ge-path / ELSE: lt-path`; two `graph.data.mapper` branches filling the
  contract from `compare.result.*`. Dry-run on the primary session honored the contract on **all
  three** cases (types preserved — `less_than` a real boolean, `sum` a real number `8.0` in
  graph.math double precision):
  - `a=5,b=3` → `{message:"a is greater than or equal to b", less_than:false, sum:8.0}`
  - `a=3,b=5` → `{message:"a is less than b", less_than:true, sum:8.0}`
  - `a=4,b=4` → `{message:"a is greater than or equal to b", less_than:false, sum:8.0}` — the
    boundary handled **more correctly than canonical tutorial-4** (which branches on `a>b`, so
    `a==b` would wrongly take the "<" path); by computing `less_than=a<b` and branching on `a>=b`
    the companion nails the spec.
- **Additional (smaller) doc frictions from the retry, still open:** (a) boolean/float/list constant
  syntax (`boolean(...)` etc.) isn't reachable from the four AI docs (deferred to the Event Script
  page) — the companion avoided it by *computing* the boolean; consider inlining the constant-type
  table; (b) no example of one node's mapper reading another node's `.result` (bare form,
  `compare.result.sum`); (c) whether an `IF` `THEN:`/`ELSE:` jump requires a matching `connect`
  (and whether the relation label matters to routing) is unspecified; (d) a skill-less "terminate
  only" `End` is only implied. None blocked the solution; all are polish.
- **Session-management feature validated in passing.** For this run the human's primary
  (`ws-145417-6`) and a companion-side subscriber (`ws-452385-7`, a zero-dependency Node-22
  built-in-`WebSocket` `.mjs`) were linked via `session subscribe`. Content synced both ways (both
  showed 5 nodes / 5 connections) and the mirrored console stream gave the orchestrator **live
  visibility into run output** — directly mitigating finding #4: it let the orchestrator
  self-diagnose a *curl-quoting* bug of its own (zsh `$"…"` vs `$'…'`) from the console alone,
  without the human relaying it.
- **Trivial aside:** the first (failed) build showed as "untitled-1" — the root node had no `name`
  property (sets the graph title / export name); not the focus.

---

## Findings → documentation & grammar improvements (rollup)

| # | From | Insight | Action |
|---|---|---|---|
| 1 | Tut 1–2 | AI agents need the AI-facing docs, not interactive help | hint set = `llms.txt` + `ai-agent-guide.md` |
| 2 | Tut 3 | `{…}` mixed as placeholder *and* in examples misleads a literal reader/AI | unbrace examples; placeholder-convention note (increment 32) |
| 3 | Tut 4 | AI docs *list* `graph.math` statement keywords but give **no statement syntax** (no `IF`/`THEN`/`ELSE`, `NEXT:`/`BEGIN`/`END` semantics unstated) → a capable agent invents wrong-but-plausible branching that fails silently | **DONE + verified** — documented the `graph.math`/`graph.js` **statement grammar** in `command-reference.md` + `minigraph-commands.json` (IF/THEN/ELSE, NEXT:=node-name, BEGIN/END=for_each, COMPUTE→result); a fresh companion then solved Tut-4 from the problem alone |
| 4 | Tut 4 | companion is **blind to run-time errors** — POST returns 200, the parser error only reaches the WS console, read-back shows empty state | (candidate) surface command/run errors through the companion/inspect response; **interim mitigation proven** — a `session subscribe` link + a Node WS subscriber gives the orchestrator live console visibility |
| 5 | Tut 4 | boolean/float/list **constant syntax** (`boolean(...)`, etc.) isn't reachable from the four AI docs (deferred to the Event Script page) | (candidate) inline the constant-type table into the knowledge-graph grammar |
| 6 | Tut 4 | no example of a mapper reading another node's `.result`; `IF` jump ↔ `connect` relationship unspecified; skill-less terminal `End` only implied | (candidate) add small examples/notes to the skills/command reference |
