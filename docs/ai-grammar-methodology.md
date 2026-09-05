# Intent-Driven Development and the Architecture of Human-AI Collaboration

*The Mercury Story: from event-driven systems to AI co-authorship*

*A white paper · Eric Law with Claude Code and GitHub Copilot · 2026-09-04 · the Rust engine's
edition of the shared product story — this repository is itself the methodology's founding
proof point.*

> Prefer slides? The Mercury story within this paper has a
> **[presentation deck](presentations/ai-grammar-story.html)**.

---

## Abstract

Mercury Composable began as an event-driven framework. Over eight years, it became a journey in
progressively reducing coupling, moving behavior from code into configuration and knowledge, and
ultimately revealing a new model for Human-AI collaboration.

Its evolution exposed a recurring pattern. The most successful AI-assisted projects were not the
ones with the cleverest prompts. They were the ones with the clearest intent.

Humans define purpose, constraints, priorities, and judgment. AI partners help refine those ideas
and translate them into designs, models, flows, tests, and implementation artifacts. Shared memory
preserves continuity. AI Grammar makes systems legible to machines. Validation and governance keep
generated artifacts aligned with their intended purpose.

This paper argues that Human-AI collaboration is becoming an engineering discipline.
**Intent-Driven Development** provides the collaboration model. **Mercury** provides a working
reference implementation.

Together, they demonstrate a simple idea:

> Productivity does not come from generating more code. It comes from reducing the distance
> between human intent and a working, governed system.

## The Thesis in One View

The first generation of AI-assisted development focused on prompts.

The second focused on context.

The next stage focuses on intent.

> **Prompts tell AI what to do. Context helps AI understand. Intent explains what matters.**

|  | Prompt engineering | Context engineering | Intent-driven development |
|---|---|---|---|
| **Human provides** | Instructions | Instructions, knowledge, constraints, and project-specific information | Vision, goals, boundaries, priorities, and judgment |
| **AI contributes** | Responses | Better responses | Collaboration in analysis, design, implementation, and refinement |

Intent-Driven Development treats intent as the organizing force of delivery. Humans remain
responsible for purpose, boundaries, trade-offs, and judgment. AI partners contribute analysis,
critique, translation, implementation, and refinement. Memory preserves continuity. Grammar makes
capabilities discoverable. Validation and governance keep generated artifacts aligned with
approved outcomes.

The architecture can be summarized as:

```text
Intent
  ↓
Vision and boundaries
  ↓
Shared memory and blueprint
  ↓
AI-readable grammar
  ↓
Models, flows, and functions
  ↓
Validation and human certification
  ↓
Governed execution
  ↓
Feedback into intent
```

Mercury did not begin with this model. It discovered it gradually through eight years of
architectural evolution.

## 1 · Origin: Composability Before AI

Mercury's story begins in 2018, before AI pair programmers became part of everyday development.
The original problem was familiar: enterprise systems tend to become coupled at their joints.

Components call one another directly. Shared structures spread across boundaries. Orchestration
logic becomes entangled with business logic. Each shortcut makes the next change harder.

Mercury adopted a strict architectural invariant from the actor-model tradition [2]: **functions
know nothing about one another** — information hiding [1], applied without exception. A function
is addressed by a route name and exchanges immutable event envelopes. It has no direct reference
to another function and no shared object through which hidden dependencies can grow.

That principle created the foundation for everything that followed:

> You can only compose what you never coupled.

The early cost was ergonomics. Event-driven development often required callbacks and reactive
chains. Lightweight concurrency later made sequential programming practical over the event-driven
core — Java virtual threads on the canonical engine, tokio async/await in this repository —
allowing the architecture to retain its decoupling without forcing application developers into a
reactive style.

The importance of this history is not merely technical. Human-AI collaboration benefits from the
same qualities that make systems composable: explicit boundaries, stable contracts, small units
of capability, and limited hidden state.

## 2 · The Ascent: Code to Configuration to Knowledge

Mercury evolved through three layers. Each layer removed a class of imperative code and made
system behavior easier to understand, compose, and govern.

### Layer 1: Event-Driven Functions

At the platform core, functions are independent units of work connected through an event bus. The
function contains capability, but it does not own the wiring.

The same function may serve different roles depending on how it is composed. It can be exposed as
a service, invoked as a task in a flow, or attached as a skill to a graph node. Mercury's
conventions name this **one atom, four roles**: the function is the single atom, and _service_,
_task_, and _skill_ only describe how it is wired.

Code remains the unit of work, not the unit of orchestration.

### Layer 2: Event Script

Orchestration written as code is a common place for coupling to return. Mercury therefore moved
orchestration into declarative YAML flows.

A flow names tasks by route, maps data between them, declares branches and execution patterns,
and defines its exception path. The result is both executable and explainable. A reader can see
the sequence, decisions, dependencies, and failure behavior without reconstructing them from
application code.

This shift moves a significant portion of an application from imperative implementation into
configuration.

### Layer 3: The Active Knowledge Graph

The third layer follows from a broader observation: much backend behavior is knowledge about the
business.

Fetch this information. Apply this rule. Transform that result. Select a path. Compose a
response. Suspend here and resume when a person or system provides new information.

Mercury represents that behavior as an Active Knowledge Graph executed by the MiniGraph engine.
Nodes can carry executable skills, and traversal turns the semantic model into a running
application.

For suitable use cases, the application becomes a model rather than a body of orchestration code.

This is not a claim that all code disappears. Custom functions and Event Script remain available
where the problem demands them. The layers compose downward:

```text
Graph → Flow → Function
```

The architectural direction is clear: place behavior at the highest level where it can remain
understandable and governed.

### Compile Before Run

A graph becomes available only after it passes the CompileGraph validation gate. Flows follow the
same compile-before-run discipline.

The principle is straightforward:

> A machine-authored artifact should not become executable merely because it looks plausible.

It must satisfy the platform contract, survive validation, and remain available for human
inspection and certification.

## 3 · The Turn: Why This Matters in the AI Era

AI-assisted engineering introduces two problems that meet directly in this architecture.

### The Governance Problem

AI can produce imperative code at high speed. That speed creates value, but the evidence for its
cost is measurable: AI-generated code reproduces known vulnerability patterns in a large fraction
of security-relevant scenarios [3]; developers assisted by AI write less secure code while
believing it more secure [4]; and industry-wide delivery research ties rising AI adoption to
reduced delivery stability, even as throughput improves [5].

The common response is to add more review after generation. Mercury suggests a complementary
strategy:

> **Change the artifact the AI authors.**

When an AI partner authors a flow or graph, it produces a bounded, declarative artifact.
Compilers can validate its structure and contracts. A human can examine its intended behavior.
Dry-runs and tests can challenge it before deployment.

This does not make AI deterministic. It separates creative authorship from authorized execution.

### The Context Problem

AI agents require context, and context is not free. Asking every agent in every session to
rediscover a platform from source code is slow, expensive, and inconsistent. Large context
windows do not eliminate the need for curation, discovery, and reliable contracts — models
measurably degrade at using information buried in the middle of long inputs [6].

A platform designed for AI collaboration must make itself legible at a cost that scales.

Mercury's declarative architecture addresses the governance problem. **AI Grammar** addresses the
context problem.

A third question remains:

> Who decides what the system should become?

That responsibility begins with intent.

## 4 · Human-AI Collaboration as an Engineering Discipline

The first generation of AI-assisted development treated AI primarily as a generator. A human
supplied a prompt. An AI produced an answer, a design, or a piece of code. Context engineering
improved the quality of those results by supplying knowledge, examples, instructions, and
constraints.

Those advances were important, but they left a central question unanswered:

> How do we keep increasingly capable AI systems aligned with what humans actually intend?

Without a disciplined answer, acceleration can amplify misunderstanding. A missing boundary
becomes an implementation assumption. An outdated instruction becomes a design decision. A
forgotten ruling is rediscovered inconsistently.

More output is produced, but not necessarily better outcomes.

> **Acceleration without direction is only faster drift.**

Intent-Driven Development addresses this challenge by making intent, rather than generated
output, the organizing force of delivery.

It does not assume that intent is complete at the beginning. Intent is expressed, examined,
refined, translated, implemented, evaluated, and corrected through a continuing Human-AI feedback
loop.

### Intent Is the Primary Human Artifact

A prompt describes the next action. Intent gives that action meaning.

Intent includes:

- the purpose of the system;
- the outcome the work should achieve;
- the boundaries that must be respected;
- the qualities that must be preserved;
- the trade-offs that are acceptable;
- the risks that require judgment;
- and the evidence by which an outcome will be accepted.

AI can help refine intent by exposing contradictions, identifying assumptions, comparing
alternatives, and translating an emerging idea into a clearer Vision or Blueprint.

The AI participates in refining intent. It does not become its owner.

### Complementary Responsibility

Human-AI collaboration works when responsibilities remain clear.

#### Humans provide

- purpose and desired outcomes;
- accountability and ethical judgment;
- boundaries and priorities;
- architectural rulings and acceptable trade-offs;
- certification of consequential artifacts;
- and the final decision to place a system into use.

#### AI partners contribute

- synthesis and analysis;
- critique and alternative generation;
- translation of intent into structured artifacts;
- implementation of bounded tasks;
- consistency checking;
- test and documentation support;
- and identification of ambiguity or missing context.

#### Platforms provide

- contracts and constraints;
- structural and semantic validation;
- controlled execution;
- deployment gates;
- observability;
- and operational feedback.

This is not a model of autonomous replacement. It is a model of complementary responsibility.

**Humans provide direction. AI provides leverage. Governance provides trust.**

One practical refinement sharpens the AI side of this division: specialized reviewer personas
invoked on demand. One critiques architecture. Another examines security. Another checks
implementation consistency. Another assesses documentation clarity. These perspectives do not
replace human judgment; they give the human architect structured, independent viewpoints from
which better decisions are made.

### From Conversation to Governed Artifact

A valuable conversation may clarify an idea, but an unrecorded conversation is not a durable
engineering artifact. It cannot orient the next session, bind an implementation, or establish
what was approved.

Intent-Driven Development therefore moves collaboration through progressively more executable
artifacts:

```text
Intent
  ↓
Vision
  ↓
Blueprint
  ↓
Design and decisions
  ↓
Model, flow, contract, or function
  ↓
Validation and certification
  ↓
Execution and feedback
```

The Vision describes the desired future. The Blueprint measures the gap between that future and
the current state. Design records preserve important reasoning. Models, flows, and functions
realize the design. Compilers and tests evaluate conformance. Human certification evaluates
meaning, risk, and fitness. Runtime evidence returns to the next cycle of refinement.

The result is not a one-way handoff from human to machine. It is a cognitive loop that keeps
purpose connected to execution.

### Four Forms of Drift

Intent-Driven Development can be understood by the problems it helps control.

**Purpose drift.** The implementation gradually moves away from the outcome it was meant to
achieve. Vision, intent, and human judgment keep delivery connected to purpose.

**Context drift.** Documentation and instructions become disconnected from the implementation.
AI Grammar binds the explanatory layer to the platform through versioning, machine-readable
catalogs, manifests, and validation gates.

**Continuity drift.** Decisions and rationale are forgotten across sessions, participants, or
tools. Shared memory, Blueprints, continuity records, and architecture decisions preserve what
was decided and why.

**Implementation drift.** Generated artifacts violate approved designs, platform contracts, or
deployment rules. Compilers, tests, dry-runs, staged promotion, and human certification constrain
what becomes executable.

These controls reinforce one another.

> Grammar without intent can help AI build the wrong thing correctly. Intent without executable
> contracts remains aspiration. Validation without human judgment can prove conformance without
> proving value.

## 5 · Memory and Grammar

Human-AI collaboration requires both continuity and legibility. Shared memory and AI Grammar
address these needs, but they solve different problems.

### Shared Memory Preserves the Project

Memory is project-specific. It explains:

- why the project exists;
- what is currently true;
- what has been decided;
- which assumptions were rejected;
- what changed;
- and what should happen next.

Looking backward, memory preserves decisions, provenance, contradictions, and history. Looking
forward, it carries intent through Vision, Blueprint, design, implementation, and feedback.

Memory allows a new session or AI partner to continue the project rather than reconstruct it.

### AI Grammar Explains the Capability

Grammar is capability-specific. It explains:

- what a platform or building block can express;
- where authoritative guidance is found;
- how correct artifacts are authored;
- which contracts they must satisfy;
- and how conformance is verified.

An **AI Grammar** is the machine-consumable contract of a codebase: its discovery map, reference
guides, machine-readable catalogs, and validation gates. Together, these let an AI partner author
correct artifacts without repeatedly reading engine source. The test of a grammar is that system
behavior becomes **derivable rather than guessable**.

AI Grammar is not documentation with an AI label. It must be:

- **sufficient**, so an AI partner can act from it;
- **verified**, so its claims remain bound to the implementation;
- **discoverable**, so the right guidance is found quickly;
- **version-matched**, so the guidance corresponds to the running capability;
- and **economical**, so context consumption scales with the task rather than the dependency tree.

Within Intent-Driven Development, AI Grammar is the translation contract between project intent
and platform capability. It does not decide what should be built. It makes the available building
language explicit and verifiable.

The distinction is concise:

> **Memory explains the project. Grammar explains the platform.**

Memory without grammar provides direction without a dependable means of execution. Grammar
without memory provides capability without purpose or continuity.

## 6 · What Mercury Ships

Mercury makes the AI Grammar concrete through a small set of mutually reinforcing mechanisms.

### Discovery Maps

`llms.txt` routes an AI partner to the relevant guidance in one hop. The Java engine uses an
exhaustive map with a CI coverage gate — a new guide cannot ship unlisted. This repository uses a
deliberately curated agent map with a CI link-integrity gate.

The goal is not to place all knowledge in the map. The goal is to make authoritative knowledge
easy to find.

### DSL Specification Kits

Each major authoring surface — REST bindings, Event Script flows, MiniGraph commands — has a
specification kit:

- a grammar reference;
- a machine-readable catalog;
- an AI agent guide;
- and a CI drift test binding the guidance to the engine.

The approach applies the idea of machine-readable contracts to the surfaces through which
applications are authored.

### Version-Matched Contracts

The `ai-contract-provider` serves the documentation set as a version-matched contract: a
discovery endpoint, a per-file SHA-256 manifest, and an export for offline agent use.

The AI partner receives guidance for the engine it is actually driving.

### Cross-Language Registration Contracts

A shared registration metadata model defines how functions declare themselves and how they behave
during startup. Language-specific forms, including Java annotations and Rust macros, carry the
same contract.

Golden vectors test common contracts across engines so independent implementations can prove
compatible behavior.

### Engine and Language-Pack Parity

The Java engine is the reference implementation, and this repository — the Rust engine — tracks
the same contracts. Flow YAML can move across the two engines. Python and Node.js functions can
participate as Event-over-HTTP peers using the same envelope.

The grammar remains language-neutral because the contracts remain language-neutral.

### Project Memory

The project's memory is distinct from the platform's grammar. Vision, Blueprint, continuity,
session records, and architecture decisions orient each AI session on purpose and state rather
than only API shape.

Together, these mechanisms make guide-first behavior rational. If documentation can drift freely,
an AI partner learns to distrust it and returns to source. Verification gates preserve trust in
the explanatory layer.

The obligation runs both ways. The AI partner does not merely consume the explanatory layer; it
helps maintain it. In Mercury's working practice, an agent that had to fall back to source
records the guide gap in its session log, and the gap is closed in a follow-up documentation
change.

## 7 · The Methodology: Building with an AI Partner

Intent-Driven Development becomes practical through a simple journey.

### Step 1: Establish Intent and Memory

The first act is not code.

Human and AI partner define the Vision together. The human confirms it. From the Vision, they
derive a Blueprint that describes the current state, desired state, and meaningful gaps between
them.

Implementation proceeds through increments that close those gaps. Each new session begins with
the same orientation:

```text
Current state → Vision → Blueprint → Design → Implementation → Feedback
```

Memory preserves continuity, but judgment remains a shared human responsibility.

> Mechanize the arithmetic. Do not mechanize the judgment.

The proof point is **this repository**: the Rust engine was AI-enabled **before its first line of
code** — the Vision and memory layer landed on 2026-07-15, the code followed — so every increment
was derived from stated intent rather than reconstructed after the fact. Roughly a hundred
increments later it ships in lock-step with the Java engine, with the same discipline intact.

### Step 2: Load the Capability Grammar

The platform arrives with its own AI Grammar: discovery map, version-matched contract, reference
guides, catalogs, and validation rules. Both engines carry the same grammar and the same layers —
choose the engine, keep the methodology.

The team then chooses the highest appropriate authoring layer:

1. **Active Knowledge Graph** for behavior naturally expressed as knowledge and traversal;
2. **Event Script** when a flow is the clearest shape;
3. **Custom functions** where code is genuinely the unit of work.

The layers are a dial, not a wall. A project can start with knowledge and reach down only as far
as the problem requires.

### Step 3: Produce Governed Artifacts

A user application moves from intent to model, from model to validation, and from validation to
certification and deployment.

A reusable building block requires one additional step: it should publish its own AI Grammar.

Its repository should explain what the block provides, how it is discovered, how it is used,
which contracts apply, and how those contracts remain verified. The block then becomes legible to
the next application and the next AI partner.

This makes the methodology recursive:

> Dependencies compose. Their grammars must compose too.

The recursion is already live inside the framework, and it has already supplied its own
cautionary tale. On the Java engine, twin-kafka is a building block built on a building block — a
second Kafka cluster on top of the minimalist Kafka library. While its entry was missing from the
discovery map, the module was effectively invisible to AI partners, who fell back to reading
source. The entry landed, with a CI gate behind it, and one hop of discovery replaced the hunt.

To an AI partner, undocumented capability is absent capability.

## 8 · Governed Nondeterminism

Intent-Driven Development does not assume deterministic AI output.

Two capable AI partners may suggest different designs from the same Vision. The same model may
produce different implementations in separate runs. That variation can be useful because it
supports exploration, alternative generation, and critique.

The goal is not deterministic authorship. The goal is governed execution.

1. AI partners propose and author within defined boundaries.
2. Machine-readable contracts constrain the available forms.
3. Compilers and tests reject invalid artifacts.
4. Humans evaluate meaning, risk, and fitness.
5. Promotion gates determine what becomes operational.
6. Runtime evidence informs the next refinement.

This is **governed nondeterminism**:

> **Variation in exploration. Discipline in execution.**

Governance is not a review step added after AI generation. It is part of the architecture through
which intent becomes executable.

## 9 · Proof Through Fresh-Agent Evaluation

A methodology for AI collaboration should be tested with AI collaborators.

The `ai-enabled-repo-demo` exercises gave fresh AI agents no project history and no human hints
beyond the available grammar. The agents were asked to discover the platform, author
applications, and run them.

The value of the exercise was not that every first attempt succeeded. The value was that friction
became evidence.

Repeated misunderstandings indicated defects in guidance, discovery, or platform behavior. Those
findings led to concrete changes: a JSON data-mapping plugin, a Playground export correction,
clearer recipe lines in agent guides, and the retirement of a fire-and-forget companion endpoint
that hid errors from its callers — shipped in v4.12.2. Stronger discovery gates followed in
v4.12.3: a coverage gate binding the Java engine's exhaustive map to the documentation tree, and
a link-integrity gate on this repository's curated map.

The evaluation loop was:

```text
Agent friction
  ↓
Grammar or engine diagnosis
  ↓
Verified correction
  ↓
Field release
  ↓
Fresh-agent re-evaluation
```

This is the methodology working as intended. The AI partner is not only a consumer of the
platform. Its friction helps improve the platform's ability to explain itself.

## 10 · Standing on Recognized Practice

Intent-Driven Development and AI Grammar build on established ideas rather than replacing them.

- **Information hiding and the actor model** provide the decoupled architectural substrate [1][2].
- **Event-driven architecture** provides independent units of capability and message-based
  composition.
- **Docs-as-code** establishes documentation as a versioned engineering artifact [9].
- **OpenAPI and machine-readable contracts** demonstrate the value of specifications that both
  people and systems can consume [7].
- **Architecture Decision Records** preserve durable rationale [10].
- **Consumer-driven contract testing** provides a model for shared, executable expectations
  across implementations [11][12].
- **`llms.txt` and agent-instruction conventions** improve machine-oriented discovery and
  guidance [8][13].
- **Spec-driven development** treats the specification as a primary artifact from which AI can
  build [14].
- **Human-in-the-loop governance** keeps consequential decisions subject to oversight [15][16][17].
- **Evaluation culture** replaces anecdotal confidence with repeatable evidence [18][19].

The contribution lies in the synthesis:

> A Human-AI collaboration model supported by shared memory, a CI-verified and token-conscious
> capability grammar, governed artifacts, and an executable composable runtime.

## 11 · What This Unlocks

### Enterprise-Grade AI Development

AI partners can author models and configuration while compilers and humans govern what advances.
Creativity remains available, but operational outcomes stay bounded by contracts and approval.

### Sustainable Context Economics

An AI partner can navigate from a compact discovery map to authoritative guidance rather than
repeatedly reading an entire dependency's source. Context consumption can follow the task instead
of the full dependency tree.

The effect is measurable. On this repository's documentation map (measured 2026-09-04, v4.12.3),
a roughly 2,500-token discovery map routes an agent to about 46,000 tokens of source-verified
reference, replacing a hunt through roughly 515,000 tokens of engine source. Completeness that
costs discovery is a regression: the map must stay small, dense with the exact terms an agent
searches for, and gated so it cannot silently rot.

### Human-AI Co-Authorship

Humans and AI partners can work on the same durable artifacts: Vision, Blueprint, decisions,
models, flows, tests, and explanatory contracts. Collaboration becomes part of the engineering
system rather than an informal conversation outside it.

### Language Independence

Shared envelope and registration contracts allow functions from multiple languages and engines
to participate in the same composable architecture.

### Reusable AI-Legible Building Blocks

A building block can carry its own grammar, allowing capability and understanding to compose
transitively.

### Governed Knowledge-Driven Applications

The Active Knowledge Graph allows business behavior to be examined, validated, and changed as
knowledge rather than being buried entirely in imperative code.

## 12 · Where It Goes

The next step is AI agent orchestration on the graph runtime.

In this direction, LLM reasoning and tools become bounded nodes in a decision graph. The graph
controls where reasoning is invited, which tools are available, what context is supplied, and how
results are validated or escalated. The first experiment has already run end-to-end: a
support-triage graph driving live LLM verdicts through the engine under one distributed trace.

A pluggable AI companion can support collaboration without binding the methodology to one model
vendor. An enterprise governance lifecycle can move models through dry-run, certification,
staging, approval, and production as standard endpoints.

The north star does not change:

> **The Active Knowledge Graph is the application.**

Humans define and refine intent. AI partners help translate it into models, flows, functions,
tests, and explanations. Shared memory preserves direction and decisions. AI Grammar makes the
platform and its building blocks legible. Compiler and human gates govern what becomes
executable. The event-driven runtime carries certified artifacts into operation, and operational
evidence returns to the next cycle of refinement.

Changing behavior then means more than generating or editing code. It means refining knowledge
while preserving the chain from purpose to execution.

## Conclusion: The Real Contribution

The story presented here is not ultimately about Mercury.

Mercury is the journey through which these ideas became visible and the environment in which they
were proven together.

The larger lesson is that Human-AI collaboration works best when responsibilities are explicit
and artifacts remain connected.

Humans provide intent.

AI provides leverage.

Memory provides continuity.

Grammar provides understanding.

Governance provides trust.

The runtime provides execution.

As AI systems become more capable, successful organizations will not be distinguished only by
their access to models or their ability to produce code quickly. They will be distinguished by
their ability to express intent, preserve knowledge, govern change, and collaborate effectively
with AI partners.

That is the promise of Intent-Driven Development:

> **Human intent provides direction. AI partnership provides leverage. Governed systems turn
> their shared work into durable outcomes.**

And the working posture follows from it:

> **Do not just prompt. Do not just vibe code. Do not expect magic. Express intent. Define
> boundaries. Build living context. Guide your AI partners. Review the work. Improve the
> system.**

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

*This paper succeeds and extends two earlier pieces: "From Context Engineering to Intent-Driven
Development" (Eric Law, August 2026) and the Rust edition of "The Mercury Story — Origin,
Evolution, and the AI Grammar Methodology" (this repository, September 2026), whose Mercury
narrative it carries forward. Sources: this repository's `docs/guides/` and
`docs/arch-decisions/ADR.md` (with the canonical Java engine's guides as the behavior
specification); the shared memory layer (`memory/vision.md`, `memory/continuity.md`); records of
the ai-enabled-repo-demo exercises; measurements taken 2026-09-04 on this repository's
documentation map. All figures are reproducible from the cited artifacts.*

*Mercury Composable is an official Accenture open-source project; this repository is its official
Rust implementation (github.com/Accenture/mercury). agent-memory is a lightweight, vendor-neutral
shared-memory and cognitive-loop framework for human-AI collaboration, published under
Apache-2.0.*
