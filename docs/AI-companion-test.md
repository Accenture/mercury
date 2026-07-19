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
  pointer to the canonical AI-agent docs. **As of 2026-07-18 these are ported into THIS repo** so a
  test references the Rust port's own docs (which reflect its reality — e.g. `graph.js` retired, the
  synchronous companion endpoint): `docs/llms.txt`, `docs/guides/knowledge-graph/ai-agent-guide.md`,
  `command-reference.md`, `minigraph-commands.json`, `skills-reference.md`. **Never** the interactive
  `help/*.md` or the tutorial walkthroughs. (Tutorials 1–4 used the Java upstream's copies.)
  **From tutorial 6 on, the allowed set is `docs/llms.txt` + every guide it maps** — including the
  layer-1/2 AI docs (`docs/guides/event-script/`, `docs/guides/event-driven/`, ported 2026-07-19)
  for tutorials whose extensions use flows and tasks. In-band engine lookups (`help`,
  `describe skill`) remain fair game but each one is **recorded as a doc gap** — the grammar's goal
  is that agents never need them.
- **The companion drives the live session** over the documented endpoints. AI agents use the
  **synchronous** `POST /api/companion/{id}/sync` (outcome in-band `{ok, output, error, result}`;
  also teed to the human's WS console); `GET /api/graph/session/{id}` reads the model shape.
- **The human validates** in the Playground UI (screenshots) and via a behavioral **dry-run**.
- The orchestrator (Claude Code) may privately consult the canonical tutorial to *judge* correctness
  and assist if the companion is genuinely stuck, but never leaks it into the briefing.

## Validation ladder

| Level | Tutorials | What it tests | Briefing | Pause model | Acceptance |
|---|---|---|---|---|---|
| **L1 — mechanics + transcription** | 1–2 | drive the endpoints; look up exact syntax for a *known* graph | command list / syntax reference | step-wise | commands dispatch; graph built |
| **L2 — comprehension** | 3 | reconstruct a *specified* graph from the canonical docs alone (no walkthrough) | full node/connection plan; derive syntax | step-wise, screenshot each step | structural match to canonical + dry-run |
| **L3 — synthesis / problem-solving** | 4 | *design* a correct solution to a stated **problem + output contract**, with **no syntax hints** | problem statement only | build whole graph, pause for dry-run | **behavioral**: dry-run honors the contract on every branch |
| **L4 — composition** | 5 | *compose* mechanisms (parallel fan-out + join barrier + data sourcing + list assembly) from a problem + contract, no syntax hints | problem statement only | build whole graph, pause for dry-run | **behavioral** + demonstrable parallelism (traversal-log timing) |
| **L5 — data-driven iteration** | 6 | chained multi-step fetch where the fan-out set is **runtime data** (`for_each` inside a fetcher), incl. a POST provider | problem statement only | build whole graph, pause for dry-run | **behavioral**: every element of the runtime list fetched + assembled; nothing hardcoded |
| **L6 — mapping-language depth** | 7 | the data-mapping mini-language end-to-end: constants, nested extraction, ordered array assembly, execution-time `f:` plugin values — pure transformation, no backends | problem statement only | build whole graph, pause for dry-run | **behavioral**: exact reshaped contract incl. a genuinely runtime-generated value |
| **L7 — reshaping toolbox** | 8 | choose the right transformation tool (JSONPath wildcard extraction, `f:listOfMap`, `f:removeKey`) for a data-driven list-reshaping contract | problem statement only | build whole graph, pause for dry-run | **behavioral**: contract met for any list length; internal field never exposed |
| **L8+ …** | 9–13 | escalate further (error paths, extensions, sub-graphs, …) | TBD per tutorial | TBD | TBD |

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

### Tutorial 5 — L4 (composition: parallel fan-out + join + data sourcing) — PASSED (first attempt)
- **Task (problem only, no syntax hints):** fetch the profiles of TWO people **in parallel** (both API
  calls at the same time) and return one combined response. Input: `{person1: <id>, person2: <id>}`
  (mock ids 100/200). Profile API: `GET /api/mdm/profile/{id}` on the same server (the dev mock).
  Output contract: `output.body = { "profile": [ <both profiles> ] }`, each entry carrying at least
  name and address; order irrelevant. Tutorial-5's focus is **parallelism + the `graph.join` barrier**.
- **Method:** fresh companion; **this repo's own AI docs for the first time** (`docs/llms.txt` + the
  four `docs/guides/knowledge-graph/` files — tutorials 1–4 used the Java upstream's copies); build
  whole graph → dry-run; allowed to probe the profile endpoint directly and to use anything the
  engine says **in-band**; drove the human's live UI session (`ws-976371-7`) entirely via `/sync`.
- **Result: PASSED on the first run — 18/18 commands `ok:true`, zero failed commands** (a first for
  the sweep at L3+; tut-4 needed a doc fix + retry). Design: `root` forks to `fetch-one`/`fetch-two`
  (`graph.api.fetcher`, per-fetcher `model.profile1`/`model.profile2` scratch), both converge on
  `join-profiles` (`graph.join`), and `end` (`graph.data.mapper`) assembles
  `model.profile1 -> output.body.profile[0]`, `model.profile2 -> output.body.profile[1]`; one
  shared Dictionary (`person-profile`, `response.profile -> result.profile`) + Provider
  (`mdm-profile`, `person_id -> path_parameter.id`). Dry-run (verified independently by the
  orchestrator re-running it):
  `{"profile":[{"address":"100 World Blvd","id":"100","name":"Peter"},{"address":"200 World Blvd","id":"200","name":"Mary"}]}`.
  **Parallelism proven from the traversal log:** both `Walk to fetch-*` lines precede either
  `Executed`; the two fetches (~4.5–5 ms each) complete out-of-order inside a ~6 ms total traversal;
  the join executes twice (sink, then release). Exported as `parallel-profile-fetch`.
- **Judged against canonical `tutorial-5.json` — semantically equivalent, three deliberate
  divergences, all valid and two arguably better:**
  1. **One** Dictionary returning the whole `response.profile` object vs canonical's two
     field-level entries (`person-name`, `person-address`) — fewer nodes for the same contract
     (each profile also carries `id`, a superset).
  2. **Deterministic post-join assembly** at `end` (`profile[0]`/`profile[1]`) vs canonical's
     per-fetcher append (`model.fetcher-N -> output.body.profile[]`, order undetermined) — avoids
     the concurrent-append hazard entirely and pins order (canonical's own walkthrough mentions the
     indexed variant as the order-guaranteeing option).
  3. No organizational Island node — the docs mark it optional; Provider/Dictionary stayed
     unconnected per the documented config-node exception.
  Notably the companion derived tutorial-5's **key lesson** — parallel branches must write to
  disjoint `model.*` scratch keys — on its own, with no hint.
- **KEY method finding — the five AI docs alone were *not* sufficient; in-band help bridged the
  gap.** Provider URL `{name}` placeholders and the Dictionary node's **bare** `input[]` (a parameter
  name, not a `source -> target` mapping) are documented nowhere in the five docs — the page both
  guides delegate to (`composing-the-layers.md#data-dictionary`) **doesn't exist in this repo**. The
  companion resolved both *before* authoring — `describe skill graph.api.fetcher` (documented) told it
  to run `help data-dictionary` (undocumented but discoverable), whose in-band text carries the full
  recipe — and that is why it never issued a failing command. Legitimate under the rules (in-band is
  fair game, and the `/sync` channel is exactly this feedback loop), but it means doc completeness was
  rescued by the engine's own help surface.
- **Doc frictions recorded by the companion (rollup #9–#13 — all fixed in the AI grammar the same
  day, maintainer-directed):** the dangling
  links (`composing-the-layers.md`, `build-your-first-graph.md`, `playground-and-companion.md`,
  `index.md`, `../event-script/syntax.md` — the latter owed the **full constant-type set**, tut-4
  friction (a) again); `help {topic}` absent from the grammar docs; ai-agent-guide prose still calls
  the endpoint "`/command`" post-rename; the success envelope **omits** `error`/`result` (increment-33
  null-omission) though the docs write `error: null`, and carries an undocumented `id` field; fan-out
  concurrency (multiple outgoing connections = parallel branches) implied only by `graph.join`'s notes.
- **Engine finding by the orchestrator (not the companion): `session subscribe` issued via
  `/sync` corrupts the subscriber registration.** The command registers its reply route as the
  subscriber — through `/sync` that is the **ephemeral per-request capture route**
  (`companion.sync.<uuid>`, released when the POST returns), not the session's real WS `.out`.
  Engine state confirmed it: the primary reported `subscribed by ["companion.sync.3da2143d…"]`
  while the watcher session believed `subscribed to ws-976371-7`. *(Post-mortem correction: the
  watcher's mirror feed dying mid-test was **user-confirmed collateral of an accidental browser
  restart** that closed/reopened the primary — not proven to be caused by this defect. The
  capture-route-as-subscriber state remains objectively wrong and worth fixing.)* Java upstream
  mirrors the same `/sync` mechanism (#189) — check both ports. Workaround: issue
  `session subscribe` over the WS connection itself. Tracked as an Open Thread in
  `memory/continuity.md`.
- **RETEST (2026-07-19, after grammar fixes #9–#13 and #17–#22) — PASSED on the tightened
  criterion: ZERO in-band lookups.** A fresh companion, same problem-only brief, session
  `ws-876960-4`: **20/20 commands `ok:true`, zero failures, first-attempt dry-run pass** — and
  where the first run had to pull `describe skill` + `help data-dictionary` for Provider/Dictionary
  authoring, this run needed **no in-band help at all**: the docs alone sufficed (that is exactly
  what fixes #9–#13 claimed). Bonus validations: the companion **wired the Island knowledge layer
  unprompted** (`root -[contains]-> knowledge -[data]-> person-profile -[provider]-> mdm-profile`
  — the #22 mandate, applied from the grammar with no hint in the brief), cited the documented
  fork/state-safety rules (#13) in its design rationale, and produced an even cleaner design than
  the first run (each fetcher's result read directly from its own `{node}.result` namespace —
  no scratch keys needed — with deterministic post-join `profile[0]/[1]` assembly). Independently
  re-verified by the orchestrator (the island sinks in 0.014 ms; both fetchers overlap; contract
  honored). Two **inference-only** frictions noted (worked first try, but the docs never show
  them): array-index mapping targets (`-> output.body.profile[0]`) and whole-subtree Dictionary
  output (`response.profile -> result.profile`) — rollup #23/#24. Exported as
  `parallel-profile-fetch-v2`.
- **Session-hygiene aside (cause understood):** the human's browser restarted accidentally before
  the test; the primary session closed, a new one (`ws-976371-7`) was handed over — and it arrived
  carrying a complete draft of this very exercise (the UI restores the local draft into the
  reconnected session; `session reset` resets subscriptions, not the draft graph). The orchestrator
  deleted all 7 nodes before briefing the companion, or the exercise would have been contaminated.
  Lesson for the method: **always verify the primary is empty before briefing.**

### Tutorial 6 — L5 (data-driven iteration: for_each + chained fetch) — PASSED (first attempt)
- **Task (problem only, no syntax hints):** given `{person_id: <id>}`, return the person's profile
  AND the details of **every** account they hold: `output.body = {name, address, accounts:[detail…]}`.
  Two live services: `GET /api/mdm/profile/{id}` (profile + the account-number list) and
  `POST /api/account/details` (JSON body `person_id`+`account_id` → one account's details). The
  account list is **runtime data** — nothing hardcoded. Tutorial-6's focus is **`for_each`
  iterative fetching** (chained fetchers, parallel batches).
- **Method:** fresh companion on the human's live session (`ws-876960-4`), driven via `/sync`;
  **docs = `docs/llms.txt` + the guides it maps** (first run on the post-#9–#13 grammar and the
  newly ported layer-1/2 docs); in-band lookups allowed but each recorded as a doc gap; the
  orchestrator's headless watcher subscribed **over the WS connection** (`subscribed by
  ["ws-759054-5"]` — the #194 read-only companion rule + WS-side subscribe validated live; the
  mirror held for the entire run).
- **Result: PASSED on the first dry-run — 18 observed POSTs, all `ok:true`, zero failed commands**
  (the companion reported 16 — it didn't count its two `describe graph` read-backs). Design:
  `root → profile-fetcher (person-profile Dictionary → GET provider; name/address/accounts into
  disjoint model.* scratch) → accounts-fetcher (`for_each[]=profile-fetcher.result.accounts ->
  model.account_id`, explicit `concurrency=3`, account-detail Dictionary → **POST provider** with
  `body.*` targets) → end (mapper assembles the contract)`. Dry-run returned all **5** account
  details + name/address for person 100; independently re-verified by the orchestrator (chained
  traversal ~15 ms; iteration inside the second fetcher — no join barrier needed, correctly).
- **Judged vs canonical `tutorial-6.json` — semantically equivalent; economical divergences:** one
  whole-profile Dictionary vs canonical's three field-level entries (name/address/accounts); scratch
  `model.*` + deterministic `end`-mapper assembly vs canonical's direct-to-`output.body` writes; no
  organizational Island; no `feature[]` flags (the mock needs none — canonical shows `oauth2-bearer`
  as a placeholder). The **key mechanism — a cross-node runtime array
  (`profile-fetcher.result.accounts`) driving `for_each` iteration with `model.*` per-element input
  wiring — was derived correctly**, with exactly **one** in-band lookup.
- **Grammar-sufficiency check (the tightened criterion from tut-5's planned retest, applied here):**
  the tut-5 fixes **held** — Provider/Dictionary authoring (incl. the previously-undocumented POST
  `body.*` form) and everything else came **straight from the AI docs, zero lookups**. The single
  in-band `help graph-api-fetcher` was for the **`for_each` idiom**, whose authoring detail the AI
  docs still lack (rollup #17) — one gap class per tutorial, each narrower than the last.
- **Doc frictions recorded by the companion (rollup #17–#21 — all fixed same day,
  maintainer-directed, engine-verified; plus the #22 island mandate):** `for_each` idiom absent from the AI
  grammar (bare `for_each[]=<array> -> model.<var>` syntax only — no cross-node source example, no
  input-wiring pattern); **aggregation semantics undocumented** (per-iteration `result.{key}` values
  collect into an array — confirmed only by experiment; ordering unspecified though observed
  input-order under `concurrency=3`); `output[]` **required-vs-optional contradiction** for
  `graph.api.fetcher` (AI matrix says required, engine help says optional); POST-body Provider
  authoring was example-free; and the KG grammar's constants section reads as a **closed set** while
  `f:` simple-plugin calls (and `$.` JSONPath) are in fact resolvable in graph mappings — the graph
  skills route through the shared Event Script mapping engine (code-confirmed:
  `skills.rs` → `get_lhs_or_constant` → `f:` dispatch); the in-band help even recommends `f:` as the
  `:type` replacement.

### Tutorial 7 — L6 (mapping-language depth) — PASSED (first attempt; the cleanest run yet)
- **Task (problem only, no syntax hints):** a **pure transformation** graph, no backends. Input
  `{profile: {name, address1, address2}}`; output contract
  `{hello: "world" (literal), name, address: [address1, address2] (ordered), time: <current local
  date-time generated at execution — not a constant>}`. Tutorial-7's focus is the **data-mapping
  mini-language**: constants, nested extraction, array assembly, plugin functions.
- **Pre-flight (maintainer prompt):** the tutorial needs the `f:now()` simple plugin and not all
  Java plugins were assumed converted — verified **present and correct** in the port (registry +
  `plugins_e8.rs` + a live scratch-session probe: `iso`/`local`/`ms` all working in a real graph
  mapping) before briefing; no engine work needed. Also noted: the tutorial help pages carry
  known errors (backlog #16) — the canonical walkthrough itself recommends direct addressing over
  its own append+clear (`model.none`) sequence.
- **Result: PASSED — 8/8 commands `ok:true`, zero failures, ZERO in-band lookups, first-attempt
  dry-run pass** (session `ws-783755-2`). Output verbatim:
  `{"address":["100 World Blvd","New York"],"hello":"world","name":"Peter","time":"2026-07-19 08:54:28.815"}`;
  independently re-run by the orchestrator (fresh timestamp proved execution-time generation).
- **The updated grammar visibly drove the design:** indexed `address[0]/[1]` chosen **with the
  documented determinism-vs-append rationale** (#23/#25 in action); `f:now(text(local))`
  discovered by following the #21 pointer from the KG grammar into the **event-script plugin
  catalog** (the cross-layer llms.txt map worked); nested seed lines
  (`text(Peter) -> input.body.profile.name`) inferred from the composite-key rule; and **no
  cargo-cult island** — the companion correctly reasoned the #22 mandate doesn't apply to a graph
  with no config/data-entity nodes.
- **Judged vs canonical `tutorial-7.json`:** semantically equivalent and **more economical** —
  2 nodes (`root` → `end`-as-mapper, per the docs' own recipe) vs canonical's 3 (separate mapper +
  skill-less end); the companion independently landed on the walkthrough's own "better solution"
  (direct addressing, no `model.none` clearing needed).
- **Frictions (inference-only, both fixed same day → rollup #26/#27):** the KG grammar named no
  example `f:` plugin (an agent must realize the timestamp answer lives a layer down) → generator
  examples now inline; nested composite keys in `instantiate` seed lines were implied but never
  shown → nested-seed example added.

### Tutorial 8 — L7 (reshaping toolbox: JSONPath + transformation plugins) — PASSED (first attempt)
- **Task (problem only, no syntax hints):** pure transformation — input
  `{profile: {name, account: [{id, amount, description, type}, …]}}`; output
  `{name, account: [same accounts, same order, each WITHOUT the internal "description" field]}`;
  list length data-driven, nothing hardcoded per element. Tutorial-8's teaching set: JSONPath
  wildcard extraction (`$.…account[*].type` → map-of-lists), `f:listOfMap` consolidation, and
  `f:removeKey` — the walkthrough's own "indeed easier" single-source route.
- **Result: PASSED — 17/17 commands `ok:true`, zero failures, ZERO in-band lookups, first-attempt
  dry-run pass** (session `ws-783755-2`). Output verbatim:
  `{"account":[{"amount":18000.3,"id":"100","type":"C/D"},{"amount":62050.8,"id":"200","type":"Saving"}],"name":"Peter"}`
  — order preserved, `description` gone; independently re-run by the orchestrator.
- **Technique chosen:** the one-mapping `f:removeKey(input.body.profile.account, text(description))`
  route — the canonical's own recommendation for a single source; the invocation form was
  **inferred** from the generic plugin pattern (→ #28). JSONPath/`f:listOfMap` were not needed by
  the winning design (the JSONPath machinery itself remains covered by the canonical fixture
  suite); the companion's unused fallback plan was a `graph.math` `for_each` loop (→ #29).
- **Emergent knowledge-layer behavior (beyond the #22 mandate):** with **no config nodes at all**,
  the companion still built an island with two **data-entity nodes** (`person-profile`, `account`)
  whose purposes document the domain — including "description is internal-only and never exposed".
  The graph-as-living-documentation idea is now shaping designs unprompted; it also flagged that
  the mandate's scope for pure-transformation graphs could be stated crisply (→ #30, maintainer
  question).
- **Frictions:** #28 (`f:removeKey` had no syntax line/example anywhere — fixed same day,
  code-verified: map or list-of-maps, N key args, returns a copy) and #29 (`graph.math`
  `for_each[]`/`BEGIN`/`END` thinly specified — candidate; likely better addressed when a later
  tutorial exercises it). Verified-by-use with no issues: nested/indexed `instantiate` seeds
  (#27's fix), `double(…)` constants, the `/sync` envelope contract.

---

## Findings → documentation & grammar improvements (rollup)

| # | From | Insight | Action |
|---|---|---|---|
| 1 | Tut 1–2 | AI agents need the AI-facing docs, not interactive help | hint set = `llms.txt` + `ai-agent-guide.md` |
| 2 | Tut 3 | `{…}` mixed as placeholder *and* in examples misleads a literal reader/AI | unbrace examples; placeholder-convention note (increment 32) |
| 3 | Tut 4 | AI docs *list* `graph.math` statement keywords but give **no statement syntax** (no `IF`/`THEN`/`ELSE`, `NEXT:`/`BEGIN`/`END` semantics unstated) → a capable agent invents wrong-but-plausible branching that fails silently | **DONE + verified** — documented the `graph.math`/`graph.js` **statement grammar** in `command-reference.md` + `minigraph-commands.json` (IF/THEN/ELSE, NEXT:=node-name, BEGIN/END=for_each, COMPUTE→result); a fresh companion then solved Tut-4 from the problem alone |
| 4 | Tut 4 | companion is **blind to run-time errors** — POST returns 200, the parser error only reaches the WS console, read-back shows empty state | **DONE (Rust prototype, verified end-to-end)** — additive synchronous `POST /api/companion/{id}/sync` returns `{ok, output, error, result}` in-band (design: `docs/design/ai-companion-sync.md`; blueprint `bp-companion-sync`). Re-verified: a fresh companion rebuilt tutorial-4 **fully autonomously** via `/command` (built + ran + self-validated all 3 cases; explicitly *never needed the WS console or a GET fallback*); errors return in-band (`ok:false` + `error`). Java upstream PR still to follow. |
| 5 | Tut 4 | boolean/float/list **constant syntax** (`boolean(...)`, etc.) isn't reachable from the four AI docs (deferred to the Event Script page) | (candidate) inline the constant-type table into the knowledge-graph grammar |
| 6 | Tut 4 | no example of a mapper reading another node's `.result`; `IF` jump ↔ `connect` relationship unspecified; skill-less terminal `End` only implied | (candidate) add small examples/notes to the skills/command reference |
| 7 | Tut-4 live demo | **`graph.js` retired at runtime but still listed as available** in the AI docs → a fresh companion wasted three commands trying it (though it self-corrected each time via the in-band error) | **DONE** — AI docs **ported into this Rust repo** (`docs/guides/knowledge-graph/` + `docs/llms.txt`) and `graph.js` marked retired everywhere (command-reference, minigraph-commands.json, skills-reference); future tests reference the Rust repo's docs |
| 8 | Tut-4 live demo | `graph.math`'s dialect is narrower than "JS-like" implies (no bitwise ops, no function calls) and serializes integers as floats (`8.0`) with no in-grammar coercion | **DONE** — documented in the ported command-reference + skills-reference |
| 9 | Tut 5 | Provider URL `{name}` placeholder syntax and the Dictionary node's **bare** `input[]` shape (incl. `:` = default value **only**) were in none of the five AI docs — the page they delegated to (`composing-the-layers.md#data-dictionary`) doesn't exist in this repo; the recipe lived only in in-band `help data-dictionary` | **DONE** (same day) — new [Provider & Dictionary](guides/knowledge-graph/command-reference.md#provider-dictionary) section in `command-reference.md`, enriched `config_nodes` in `minigraph-commands.json`, `skills-reference.md` retargeted; **all dangling links removed** and the **closed constant set** inlined (`#constants`; deprecated `:type` suffix explicitly excluded — maintainer confirmed) |
| 10 | Tut 5 | `help {topic}` is the engine's own discovery surface (in-band `describe skill` even points to it) but was absent from the AI grammar docs | **DONE** (same day) — `help` documented as a command in `command-reference.md` + `minigraph-commands.json` (topics, aliases) |
| 11 | Tut 5 | `ai-agent-guide.md` prose still called the synchronous endpoint "the `/command` endpoint" after the `/command` → `/sync` rename (tables/examples were correct) | **DONE** (same day) — prose swept to `/sync` |
| 12 | Tut 5 | the `/sync` success envelope **omits** `error`/`result` (increment-33 serializer null-omission) though the docs wrote `error: null | "..."`, and carries an undocumented `id` field — a strict parser trips | **DONE** (same day) — envelope documented truthfully (absent ⇒ null, `id` field) in `ai-agent-guide.md` + a `sync_envelope` object in `minigraph-commands.json` |
| 13 | Tut 5 | fan-out concurrency (multiple outgoing connections fork **parallel** branches) was implied only by `graph.join`'s notes | **DONE** (same day) — explicit fork/join/state-safety rules in `command-reference.md#connect`, `skills-reference.md#join`, and a `traversal` object in `minigraph-commands.json` |
| 14 | Tut 5 (orchestrator) | **engine defect, both ports:** `session subscribe` via `/sync` registers the ephemeral `companion.sync.<uuid>` capture route as a durable subscriber → dangling subscriber + asymmetric session state. (The mirror death observed mid-test was later user-confirmed as collateral of an accidental browser restart — the wrong-registration defect stands on the engine-state evidence alone.) | **DONE (Rust, same day; maintainer decision):** both companion endpoints now limit `session` to the **read-only status query** — `subscribe`/`unsubscribe`/`reset` are rejected before dispatch (a companion is an *assistant to* a session, not a WS session of its own); refusal returned in-band (`ok:false` on `/sync`, 400 on fire-and-forget) **and** teed to the live console; deterministic test in `graph_runtime.rs`; AI docs updated. **Java fix MERGED** ([Accenture/mercury-composable#194](https://github.com/Accenture/mercury-composable/pull/194)): same guard in PostCompanionCommand/-Sync + shared statics in GraphCommandService, byte-identical refusal text, test `companionEndpointsLimitSessionCommandToReadOnly`, 65-test module suite green — the read-only rule is live in **both** engines. |
| 15 | Tut 5 (orchestrator) | `session reset` resets subscriptions but does **not** clear the draft graph, and the UI restores the local draft into a reconnected session — a "fresh" session can carry a stale (here: exercise-contaminating) draft | (candidate) UX note in `help session.md`; consider a `clear graph` affordance; method rule: verify-empty before briefing |
| 16 | Tut 5 (maintainer) | the interactive `help/*.md` pages are written for **human operators** and double as the engine's in-band reference; after tut-5 the AI grammar is self-sufficient, so the help pages deserve a dedicated human-UX rewrite (clarity, structure, the #15 `session reset` note) | **Backlog** — Open Thread `ot-help-pages-rewrite` in `memory/continuity.md`; separate session; coordinate with the Java upstream (the pages are verbatim ports) |
| 17 | Tut 6 | the **`for_each` authoring idiom** was absent from the AI grammar — only the bare `for_each[]=<array> -> model.<var>` syntax appeared; no cross-node array source (`{fetcher}.result.{key}`), no `input[]=model.<var> -> {param}` wiring pattern — the companion's single in-band lookup (`help graph-api-fetcher`) was for exactly this | **DONE** (2026-07-19, maintainer-directed) — new [Iterative fetching](guides/knowledge-graph/command-reference.md#for-each) section (source, wiring, `concurrency`, worked example) + `for_each` object in `minigraph-commands.json` + `skills-reference.md#api-fetcher` |
| 18 | Tut 6 | **`for_each` result aggregation was undocumented everywhere** (incl. the engine help): per-iteration `result.{key}` values collect into a single array; ordering under `concurrency` unspecified — the companion confirmed by experiment | **DONE** — engine-verified (`fetcher.rs`: batches execute in input order, responses join in request order, results `[]`-append per response ⇒ **order deterministically follows the source list**) and documented as a guarantee in all three docs |
| 19 | Tut 6 | `output[]` contradiction for `graph.api.fetcher`: the AI skill matrix + `minigraph-commands.json` said **required**; the engine help says **optional** ("use another data mapper") | **DONE** — engine-verified (only `dictionary[]` is hard-required; the result always lands at `{node}.result`): matrix + JSON now say `dictionary[]` required, `input[]` conditional (when dictionaries declare parameters), `output[]` optional. The engine help contradicts *itself* on this — folded into the help-rewrite backlog (#16) |
| 20 | Tut 6 | **POST-body Provider authoring was example-free** — `body.{key}` targets + `content-type` header were stated only as a target list; the only worked example was GET/path_parameter | **DONE** — second worked example (POST + `body.*` + `content-type`) in `command-reference.md#provider-dictionary` |
| 21 | Tut 6 | the KG grammar's constants section read as a **closed set**, but `f:` simple-plugin calls and `$.` JSONPath are resolvable in graph mappings (shared Event Script mapping engine; code-confirmed) — and the engine help recommends `f:` as the `:type` replacement | **DONE** — "non-constant source forms" table added under `#constants` (pointing at the event-script plugin catalog) + `_non_constant_sources` note in `minigraph-commands.json` |
| 22 | Tut 6 (maintainer) | **`graph.island` is required, not optional**: islands are isolated from traversal but they link data entities and dictionaries into the graph's **entity-relationship diagram** — the graph is living documentation of enterprise knowledge (a new joiner discovers the domain model from the connected dictionaries/entities). The tut-6 companion left its four config nodes floating (the docs said "optionally group") | **DONE** (2026-07-19, maintainer decision) — the AI grammar now mandates **no node left unconnected**: `command-reference.md#island` (new section), invariants + node-types + `#provider-dictionary` reworded, `graph.island` rewritten in `skills-reference.md`, pre-send checklist + recipe step in `ai-agent-guide.md`, `minigraph-commands.json` (`_role`, invariants, island notes). **Verified by the tut-5 retest**: a fresh companion wired the island unprompted |
| 23 | Tut-5 retest | **array-index mapping targets were never shown** — the lexical rules said keys "may be composite (dot-bracket)" but no doc had a worked example of building a JSON list in a mapper target (`… -> output.body.profile[0]`), the natural post-join assembly idiom; the companion inferred it correctly | **DONE** (2026-07-19) — "Composite keys & arrays" block in `command-reference.md#namespaces` + indexed-assembly example in `skills-reference.md#data-mapper` + `composite_keys` lexical note in `minigraph-commands.json` |
| 24 | Tut-5 retest | **whole-subtree Dictionary output mapping was only implied** — examples mapped leaf paths (`response.profile.name -> result.name`); that an interior path (`response.profile -> result.profile`) yields the entire subtree was never stated; the companion inferred it correctly | **DONE** (2026-07-19) — leaf-vs-interior rule + examples in `command-reference.md#provider-dictionary` (and the composite-keys block) + the Dictionary `output` note in `minigraph-commands.json` |
| 25 | maintainer | the **`[]` array-append target** (`… -> output.body.profile[]` appends an element, creating the list with the first element when absent) was absent from the KG grammar — and the parallel state-safety wording ("write to disjoint keys") over-restricted: data mapping is **thread-safe**, so concurrent appends carry no racing risk (element order follows completion order) | **DONE** (2026-07-19, engine-verified: `mlm.rs` `appendIndex` parity; the state mutex serializes mapping ops) — append + thread-safety + ordering guidance in the composite-keys block, `#connect` state-safety refined, `graph.join` gotcha aligned, `array_append` note in `minigraph-commands.json` |
| 26 | Tut 7 | the KG grammar named **no example `f:` plugin** — an agent needing an execution-time value (timestamp, uuid) must realize the answer lives a layer down in the event-script plugin catalog; the pointer existed but carried no scent | **DONE** (same day) — generator/arithmetic/logic examples (`f:now(text(local))`, `f:uuid()`, …) inlined in the `#constants` non-constant-sources row + `minigraph-commands.json` |
| 27 | Tut 7 | **nested composite keys in `instantiate` seed lines** were implied by the global lexical rule but never shown — every example seeded flat keys | **DONE** (same day) — nested-seed example (`text(Peter) -> input.body.profile.name`) in `command-reference.md#instantiate` + a note in `minigraph-commands.json` |
| 28 | Tut 8 | **`f:removeKey` had no syntax line or worked example anywhere** (the plugin table only said "remove one or more keys from a map or list of maps") — the companion inferred `f:removeKey(source, text(key))` from the generic pattern; this is the natural idiom for hiding internal fields | **DONE** (same day, code-verified in `plugins_e8.rs`: map or list-of-maps, N key args, non-map list elements pass through, returns a copy) — syntax in the plugin table + a worked *removeKey* example in `syntax.md`; `removeKey`/`listOfMap` added to the KG grammar's `f:` examples + JSONPath wildcard note in `minigraph-commands.json` |
| 29 | Tut 8 | **`graph.math` `for_each[]`/`BEGIN`/`END` is thinly specified** — no worked example, loop-variable binding / iteration order / interaction with the "MAPPING-only rejected" rule unstated (the fetcher's `for_each` got full rules in #17/#18; the math skill's didn't) | (candidate) document with engine verification — best addressed when a later tutorial exercises `graph.math` iteration |
| 30 | Tut 8 | **island-scope crispness for graphs with *no* config nodes:** the recipe said wire an island "if the graph has Dictionary/Provider (or data-entity) nodes" — for a pure-transformation graph, is a knowledge layer expected? (The tut-8 companion volunteered one with data-entity nodes documenting the domain) | **DONE** (maintainer ruling, 2026-07-19): island **required** whenever config/data-entity nodes exist; **encouraged** otherwise as ER documentation — wording landed in `command-reference.md#island`, `ai-agent-guide.md` recipe, `minigraph-commands.json`. Bonus context captured in `syntax.md`: `listOfMap`/JSONPath serve **impedance matching** between external (3rd-party) and internal API contracts |
