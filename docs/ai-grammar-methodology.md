# The Mercury Story — Origin, Evolution, and the AI Grammar Methodology

*A white paper · Eric Law with Claude Code · 2026-09-04 · written at the ai-enabled-repo-demo
milestone, where the AI grammar was proven end-to-end by fresh AI agents and the feedback loop
from those runs shipped to the field in v4.12.2. This is the Rust engine's edition of the shared
product story — and this repository is itself the methodology's founding proof point.*

> Prefer slides? This paper has a **[presentation deck](presentations/ai-grammar-story.html)**.

---

## Abstract

Mercury Composable is an eight-year-old event-driven framework that spent its first years
solving a people problem — teams coupling software at the joints — and, in doing so, quietly
built the substrate the AI era turns out to need. Its three-layer ascent moved application
behavior out of imperative code into **configuration** and then into **knowledge**: declarative
artifacts that a compiler validates, a human certifies, and a machine can author. This paper
tells that story, then articulates the discipline the milestone proved out: the **AI grammar
methodology** — engineering a codebase's documentation as a machine-consumable, CI-verified
contract, so an AI partner can build correct applications from the grammar alone, and so every
building block created on Mercury can compile a grammar of its own and stay legible to the next
project that depends on it.

---

## 1 · Origin — composability before it was needed

Mercury's copyright line starts in 2018, well before AI pair programmers existed. The problem it
was built against is older still: enterprise backends rot at the joints. Direct calls between
components, shared data structures, orchestration logic threaded through business code — every
one of these is a coupling that makes the next change more expensive than the last.

The founding decision came from the actor-model lineage [2] (Scala/Akka) carried onto the Eclipse
Vert.x event bus: **functions know nothing about each other** — information hiding [1], applied
without exception. A Mercury function is a
self-contained unit addressed by a route-name string, exchanging immutable event envelopes. There
is no other coupling surface — no shared objects, no direct references, nothing to rot. That rule
is the framework's first architectural invariant, and it has never changed.

For years the honest cost of this style was ergonomics: event-driven code asked developers to
think in callbacks and reactive chains. Java 21 virtual threads dissolved that tax. A synchronous
request in Mercury suspends a virtual thread instead of blocking a kernel thread, so plain
sequential code performs on par with reactive code. The event-driven core became free to use —
which mattered, because everything above it depends on it.

> You can only compose what you never coupled. The decade of discipline at layer one is the
> precondition for everything this paper describes.

## 2 · The ascent — code → configuration → knowledge

Each layer of Mercury removes a class of imperative code, and each removal was learned from
watching where coupling sneaks back in.

**Layer 1 — Event-driven (Platform Core).** Decoupled functions on the event bus. One atom, four
roles: the same function is called a *service* when `rest.yaml` maps it to an HTTP endpoint, a
*task* when a flow sequences it, a *skill* when a graph node carries it. Code remains the unit of
work — never the unit of wiring.

**Layer 2 — Composable (Event Script, introduced in v4).** Orchestration written as code is where
coupling returns, so orchestration became configuration: YAML flows that name tasks by route,
map data field-by-field between them, declare execution types, branches, and the exception path.
A flow file does two jobs code cannot do as well: it **communicates intent** — the sequence,
branches, and failure path are legible without reading Java — and it **manages dependencies**,
because the engine enforces the wiring. Roughly half of an application stops being code.

**Layer 3 — Semantic (the Active Knowledge Graph, executed by the MiniGraph engine).** The
observation behind the third layer: most backend behavior — fetch, decide, transform, respond —
is *knowledge about the business*, not engineering. So the model became the application: a
property graph whose nodes carry skills that execute during traversal. Data contracts, decision
logic, API composition, even long-running workflow suspension and resumption live in the model.
For the common case, **zero imperative code**.

The third layer is guarded the way production software must be: a graph deploys only through
**CompileGraph**, the mandatory validation gate — a model is compiled and listed, or its endpoint
answers 404 as if it never existed. Dry-run in the Playground, validate, then deploy: the same
compile-before-run discipline that code has always had, applied to knowledge.

This is deliberately **not** a "no code ever" dogma. Zero-code is the default, not a limit;
Event Script and custom functions remain the escape hatch for the demanding edge, and the three
layers compose cleanly — graph → flow → function — with no coupling anywhere in the stack.

## 3 · The turn — why this matters to the AI world

Two problems dominate serious AI-assisted engineering, and they meet exactly here.

**The governance problem.** AI writes imperative code fast, and ungoverned. Enterprises cannot
certify what they cannot read, and reviewing machine-authored code at machine speed does not
scale. The evidence is measurable: AI-generated code reproduces known vulnerability patterns in a
large fraction of security-relevant scenarios [3]; developers assisted by AI write less secure
code while believing it more secure [4]; and industry-wide delivery research ties rising AI
adoption to reduced delivery stability, even as throughput improves [5]. The industry's answer so
far is more review. Mercury's answer is different: **change the
artifact the AI authors.** An AI that authors a flow or a graph model is producing a declarative,
compiler-validated, human-legible artifact. CompileFlows and CompileGraph reject malformed intent
at build time; the Playground dry-runs it; a product owner can read and certify it. Mercury calls
this **governed nondeterminism** — the creativity of a model author, bounded by gates. Never a
determinism claim; a governance design.

**The context problem.** AI agents consume context expensively. The default way an agent learns
a dependency is to read its source — and that cost repeats for every dependency, every session,
every team. Nor is a huge context window a substitute for curation: models measurably degrade at using
information buried in the middle of long inputs [6]. A platform meant to be built *on* by AI
partners has to make itself legible at a price that scales.

The framework's decade of pushing behavior into declarations answers the first problem. The
**AI grammar** answers the second.

## 4 · The AI grammar — making a platform legible to machines

**Definition.** An AI grammar is the machine-consumable contract of a codebase: the discovery
map, reference guides, machine-readable catalogs, and validation gates that together let an AI
agent author correct artifacts **without reading engine source**. It is not documentation with an
AI sticker on it. It is documentation engineered to be *sufficient* (an agent guide may claim
"you can generate correct artifacts from this page alone" — and only agent guides may claim it),
*verified* (CI ties every claim to the code), and *cheap* (measured in tokens, not pages).

**What Mercury ships today:**

| Component | What it does |
|---|---|
| `llms.txt` discovery map | Routes an agent to the right page in one hop. On the Java engine it is an exhaustive site map with a CI **coverage gate** (a new guide cannot ship unlisted); on the Rust engine it is a deliberately curated agent map with a CI **link-integrity gate**. |
| Three DSL spec kits | For each authoring surface — `rest.yaml`, Event Script flows, MiniGraph commands — a grammar reference, a machine-readable JSON catalog, an AI agent guide, and a **CI drift test** binding them to the engine — the machine-readable-contract idea OpenAPI proved for HTTP APIs [7], applied to authoring surfaces. |
| `ai-contract-provider` | Serves the documentation set as a **version-matched contract**: discovery endpoint, per-file SHA-256 manifest, exportable as an offline Agent Skill. The agent's docs match the engine it is actually driving. |
| Registration Metadata Contract | The grammar of *declaring* functions — one metadata model with fixed boot semantics, carried by per-language idioms (Java annotations, Rust macros, future Python/Node decorators), proven by **golden vectors** shared verbatim between engines. |
| Engine parity | The Java engine is the reference implementation; the Rust engine ships in lock-step; flow YAML ports unchanged; python/node functions join flows as Event-over-HTTP peers speaking the same envelope. The grammar is language-neutral because the contracts are. |
| The shared memory layer | The *project's* grammar, distinct from the platform's: Vision, Blueprint, continuity, session logs — so every AI session starts oriented on intent and state, not just API shape. |

**The benchmark.** The maintainer's ruling, and the discipline behind every entry: **doc
discovery and token efficiency**. A grammar is useful only if the agent finds the right page in
one hop and the map stays cheap to read. Measured on the Rust engine this week: a ~2,500-token
map routes to ~46,000 tokens of source-verified reference — replacing a hunt through ~515,000
tokens of source. Completeness that costs discovery is a regression; the map is curated, dense
with the exact tokens an agent searches for, and gated so it cannot silently rot.

**"Compiled" is meant literally.** What separates an AI grammar from ordinary documentation is
that CI binds it to the code: drift tests for the three DSLs, the coverage and link-integrity
gates on the maps, golden vectors for the cross-language contracts, the version-matched manifest.
When documentation can drift freely, agents rationally learn to distrust it and go read source —
which defeats the entire economics. The gates are what make guide-first behavior rational.

**The proof.** The ai-enabled-repo-demo exercises put the grammar under load: fresh AI agents —
no project context, no human hints — built and ran applications from the grammar alone, through
repeated rehearsals and a live demonstration. Every friction they hit became a fix within days,
shipped to the field in v4.12.2: a `json` simple plugin closing a data-mapping gap, an
export-guard correction in the Playground, recipe lines added to the agent guides where two
independent fresh agents drew the same wrong conclusion, and discovery gaps closed with CI gates
behind them. That loop — **agent friction → grammar or engine fix → field release** — is the
methodology working, not a promise that it might.

## 5 · The methodology — how a developer builds with an AI partner

The developer journey has three steps. Each is ordinary on its own; the compounding effect is
the point.

### Step 1 — AI-enable the project

Greenfield or existing, the first act is not code: install the shared memory layer, and write
the **Vision** with the AI partner — confirmed by the human, never fabricated by the machine.
From the Vision, derive the **Blueprint**: the measured gap between the current state and the
target. Plan implementation as increments that close it. Every subsequent session — any agent,
any vendor — starts oriented: *Vision → Blueprint → Design → Implementation*, with feedback
closing the loop.

The proof point is **this repository**: the Rust engine was AI-enabled **before its first line
of code**, so every increment was derived from stated intent rather than reconstructed after the
fact — the Vision and memory layer landed on 2026-07-15, the code followed. Roughly a hundred
increments later it ships in lock-step with the Java engine, with the same discipline intact.

### Step 2 — add mercury-composable to the session, and choose the path

The engine arrives carrying its own grammar — the `llms.txt` map, the version-matched contract,
the exportable Agent Skill — so the session needs no source archaeology. Then choose the path:

- **Recommended for user applications: Layer 3, the knowledge graph.** (Both engines carry
  the same grammar and the same layers — choose the engine, keep the methodology.) Express the service as a
  graph model; dry-run it in the Playground with the AI companion; deploy it behind the
  CompileGraph gate. The application *is* the model.
- **Layer 2, Event Script**, when a flow is the natural shape — and beneath any graph that
  composes onto flows.
- **Layer 1, custom functions**, where code is genuinely the unit of work.

The path is a dial, not a wall. The layers compose downward without coupling, so a project can
start at the top and reach down exactly as far as the problem demands.

### Step 3 — build the application — or a building block

Two kinds of things get built on Mercury, and the methodology treats them differently on one
crucial point.

**A user application** goes intent → model (plus flows where needed) → certify → deploy. Its
behavior changes by refining the model — the Vision's core promise.

**A building block** — a common library or utility that rides on Mercury, the pattern of the
Java engine's own Kafka adapter family — is implemented with **layer-2 and layer-3 patterns**:
composable functions and reusable flows and skills, coupled to nothing, addressable by route
name like everything else. And then the step that makes the methodology recursive:

> **Compile an AI grammar into the building block's own repository.** Its guide, its map
> entries, its machine-readable catalog where it has an authoring surface, its drift gates.
> The block becomes legible to AI partners the same way Mercury is.

A new application session then loads Mercury's grammar **plus each building block's grammar**,
alongside the application's own memory layer. Grammar composes the way dependencies compose —
transitively, and at near-constant token cost per block, because each grammar routes the agent
to exactly what it needs instead of handing it source.

**The recursion is already live inside the framework.** On the Java engine, twin-kafka is a
building block built on a building block — a second Kafka cluster on top of minimalist-kafka — and this week supplied
the cautionary tale that makes the premise concrete: while twin-kafka's entry was missing from
the discovery map, the module was effectively invisible to AI partners, who fell back to source.
The entry landed, with a CI gate behind it, and one hop of discovery replaced the hunt.

**To an AI partner, undocumented capability is absent capability.** That sentence is the whole
methodology in eight words.

## 6 · Industry context — standing on recognized practice

The AI grammar is not a private invention; it is a hardening of practices the industry already
trusts, assembled into one discipline and pointed at AI collaboration:

- **`llms.txt`** [8] — the emerging convention for machine-readable site maps, proposed on the
  same premise as our benchmark: context is finite, so hand agents a curated, token-efficient
  map. Mercury adopts it and
  then does what conventions alone cannot: gates it in CI (coverage on the exhaustive map,
  link integrity on the curated one) and holds it to a measured token-efficiency benchmark.
- **Docs-as-code** [9] — documentation versioned, reviewed, and built like software. Mercury extends
  it to *docs-as-contract*: drift tests fail the build when a guide and its engine disagree.
- **Architecture Decision Records** (Nygard, 2011) [10] — Mercury's ADR ledger holds the durable rationale;
  the memory layer's facts point at ADRs, so agents inherit the *why*, not just the what.
- **Consumer-driven contract testing** [11] — the golden-vector suites (envelope wire format,
  registration metadata) are contract tests shared verbatim between independent engine
  implementations, the same trust mechanism Pact-style testing [12] brought to service boundaries.
- **Agent-instruction conventions** (`AGENTS.md` and kin) [13] — Mercury layers a routing shim on
  top: contributors are directed into the memory protocol, consumers into the version-matched
  contract, so each audience gets its own grammar.
- **Spec-driven development** [14] — the industry's 2025 turn toward the specification as the
  primary artifact AI builds from, with generation validated against it. The AI grammar applies
  the same principle one level down: the *platform's* contract, versioned and gated, is what the
  AI builds *with*.
- **Event-driven architecture and the actor model** [2] — the substrate itself is orthodox EDA;
  Mercury's contribution is carrying its decoupling discipline up into configuration and
  knowledge.
- **Human-in-the-loop governance** — dry-run before deploy, compile gates, human certification,
  and staged promotion mirror the review-and-release controls enterprises already run in their
  delivery pipelines — and the oversight posture that AI risk frameworks and regulation now
  require of consequential AI systems [15][16][17]; the graph lifecycle applies them to model
  artifacts.
- **Evaluation culture** [18][19] — the demo's fresh-agent rehearsals are evals for
  documentation; the memory smoke test is an eval for project memory. Both run on cadence, both
  produced fixes — the same systematic-measurement stance the LLM evaluation literature brought
  to models themselves.

The synthesis is the contribution: each practice is known; **binding them into a single,
CI-enforced, token-budgeted contract that an AI partner can build from is the AI grammar.**

## 7 · What this unlocks

- **Enterprise-grade AI development.** The AI authors models and configuration; compilers and
  humans gate; the certified artifact deploys. Nondeterministic authorship, governed outcome.
- **Sustainable context economics.** An agent's context budget scales with the task, not with
  the dependency tree — because every dependency worth using carries a grammar.
- **Human–AI co-authorship as a first-class capability.** Playground sessions an AI can host
  with humans joining as equal co-authors; a synchronous companion endpoint; suspend/resume as
  the human-in-the-loop primitive inside a running workflow.
- **Language independence.** The envelope and registration contracts make the grammar polyglot:
  Java and Rust engines in lock-step, python and node functions joining the same flows as peers.

## 8 · Where it goes

The public Blueprint continues the same line: **AI agent orchestration on the graph runtime** —
bounded-agency decision graphs where LLM reasoning and tools join as nodes, with the first
experiment already run end-to-end (a support-triage graph driving live LLM verdicts through the
engine under one distributed trace); a **pluggable AI companion backend** maturing the
collaboration layer; and the **enterprise governance lifecycle** — dry-run → certify → stage →
approve → production — so models promote to production as standard endpoints.

The north star does not move:

> **The Active Knowledge Graph is the application.** Humans and AI co-author the model, the
> event-driven runtime executes it, and changing behavior means editing knowledge — not
> shipping code.


## References

1. D. L. Parnas, *On the Criteria To Be Used in Decomposing Systems into Modules*, Communications
   of the ACM 15(12), 1972. <https://dl.acm.org/doi/10.1145/361598.361623>
2. C. Hewitt, P. Bishop, R. Steiger, *A Universal Modular ACTOR Formalism for Artificial
   Intelligence*, IJCAI 1973. <https://www.ijcai.org/Proceedings/73/Papers/027B.pdf>
3. H. Pearce et al., *Asleep at the Keyboard? Assessing the Security of GitHub Copilot's Code
   Contributions*, IEEE Symposium on Security and Privacy, 2022. <https://arxiv.org/abs/2108.09293>
4. N. Perry, M. Srivastava, D. Kumar, D. Boneh, *Do Users Write More Insecure Code with AI
   Assistants?*, ACM CCS 2023. <https://arxiv.org/abs/2211.03622>
5. DORA / Google Cloud, *Accelerate State of DevOps Report 2024* and *State of AI-assisted
   Software Development 2025* — AI adoption correlated with reduced delivery stability (−7.2%,
   2024); throughput positive but stability still negative (2025). <https://dora.dev>
6. N. F. Liu et al., *Lost in the Middle: How Language Models Use Long Contexts*, Transactions of
   the ACL 12, 2024. <https://aclanthology.org/2024.tacl-1.9/>
7. OpenAPI Initiative, *OpenAPI Specification*. <https://spec.openapis.org/oas/latest.html>
8. J. Howard, *The /llms.txt file* — proposal, Answer.AI, September 2024. <https://llmstxt.org>
9. Write the Docs community, *Docs as Code*. <https://www.writethedocs.org/guide/docs-as-code/>
10. M. Nygard, *Documenting Architecture Decisions*, 2011.
    <https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions>
11. I. Robinson, *Consumer-Driven Contracts: A Service Evolution Pattern*, martinfowler.com, 2006.
    <https://www.martinfowler.com/articles/consumerDrivenContracts.html>
12. Pact — consumer-driven contract testing. <https://docs.pact.io>
13. *AGENTS.md — an open format for guiding coding agents* (donated to the Agentic AI Foundation,
    Linux Foundation, 2025). <https://agents.md>
14. GitHub, *Spec-driven development with AI: get started with a new open source toolkit*, 2025.
    <https://github.blog/ai-and-ml/generative-ai/spec-driven-development-with-ai-get-started-with-a-new-open-source-toolkit/>
15. NIST, *Artificial Intelligence Risk Management Framework (AI RMF 1.0)*, NIST AI 100-1,
    January 2023. <https://nvlpubs.nist.gov/nistpubs/ai/nist.ai.100-1.pdf>
16. Regulation (EU) 2024/1689 (EU AI Act), Article 14 — Human oversight.
    <https://artificialintelligenceact.eu/article/14/>
17. ISO/IEC 42001:2023, *Artificial intelligence — Management system*.
    <https://www.iso.org/standard/42001>
18. P. Liang et al., *Holistic Evaluation of Language Models*, 2022 (arXiv:2211.09110).
    <https://arxiv.org/abs/2211.09110>
19. C. E. Jimenez et al., *SWE-bench: Can Language Models Resolve Real-World GitHub Issues?*,
    ICLR 2024. <https://arxiv.org/abs/2310.06770>

---

*Sources: this repository's `docs/guides/` and `docs/arch-decisions/ADR.md` (with the canonical
Java engine's guides as the behavior specification); the shared memory layer (`memory/vision.md`,
`memory/continuity.md`); records of the ai-enabled-repo-demo exercises; measurements taken
2026-09-04 on this repository's documentation map. All figures are reproducible from the cited
artifacts.*
