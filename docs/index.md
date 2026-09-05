# Mercury Composable for Rust

Mercury Composable lets you build backend applications as **composable, event-driven systems** —
and, increasingly, as **Active Knowledge Graphs** you evolve by *editing a model* rather than
rewriting code. Functions are fully decoupled — they know each other only by route name, wired
through event envelopes — so orchestration is **configuration, not code**. This is the official
**Rust implementation**: the event-driven core descends from the **Scala/Akka actor model**,
realized here on **tokio** async/await, with the canonical
[Java engine](https://accenture.github.io/mercury-composable/) as the behavior specification —
same three layers, same flow YAML, behavior-synced release by release.

[Get started](guides/getting-started.md){ .md-button .md-button--primary }
[Read the white paper](https://accenture.github.io/mercury/ai-grammar-methodology/){ .md-button }
[View the deck](https://accenture.github.io/mercury/presentations/ai-grammar-story.html){ .md-button }

*The white paper — **Intent-Driven Development and the Architecture of Human-AI Collaboration** —
presents the collaboration model the AI era needs and the Mercury story that proved it: shared
memory, AI grammar, and governed execution; the deck tells the same story in slides. This
repository is the methodology's founding proof point: AI-enabled before its first line of code.*

## A layered ascent

Mercury grew in three layers. Each builds on the one beneath it, and you can mix them in a single
application — drop down a layer exactly where you need more control, and no further.

| Layer | You express behavior as… | What you write |
|:------|:--------------------------|:---------------|
| **Event-driven**<br>[Platform Core](guides/event-driven/index.md) | decoupled functions reacting<br>to events | Rust functions,<br>addressed by route name |
| **Composable**<br>[Event Script](guides/event-script/index.md) | YAML flows that choreograph<br>functions | ~50% config,<br>50% code |
| **Semantic**<br>[Active Knowledge Graph](guides/knowledge-graph/index.md) | a graph whose nodes *execute*<br>during traversal | a model —<br>little or no code |

## Knowledge Graph as application

The newest layer is a paradigm shift: model business intent, enterprise knowledge, and system
behavior as **one executable [Active Knowledge Graph](guides/knowledge-graph/index.md)**. Behavior
runs as the graph is traversed, so changing what a system *does* means refining the model,
certifying it, and deploying the updated model — not rewriting and redeploying code. Humans and AI
companions co-author the same model in a live
[Playground](guides/knowledge-graph/playground-and-companion.md).

## Why a Rust port

The same destination as the Java engine, re-reached in Rust: **AI-assisted Semantic
Application Development**, on a lightweight, fast foundation
(the ported event bus benchmarks at ~155K RPC ops/s at 6 µs round-trip). The port is
**faithful by design** — the Java project remains the canonical behavior specification
(*map, don't mirror*) — with deliberate, documented divergences where the platform differs
(tokio instead of virtual threads, compile-time registration instead of classpath scanning,
no Kafka service mesh, no Spring).

## Building with an AI agent

Mercury's DSLs ship **agent-ready specifications** — a rule-based grammar plus a machine-readable
catalog — so an AI agent can generate correct artifacts *deterministically*, without inferring from
examples or reading engine source:

- [MiniGraph commands](guides/knowledge-graph/ai-agent-guide.md) — build graphs via the companion endpoint.
- [Event Script flows](guides/event-script/ai-agent-guide.md) — author flow YAML.
- [Composable functions](guides/event-driven/ai-agent-guide.md) — the Rust authoring contract.

A machine-readable map of the whole site lives at [`llms.txt`](llms.txt) — engine-verified; a
fresh agent can build graphs from it with zero out-of-band context.

## Explore the docs

- **Get started** — [Getting Started](guides/getting-started.md)
- **Guides** — [Event-driven Functions](guides/event-driven/index.md) ·
  [Event Script Flows](guides/event-script/index.md) ·
  [REST Automation](guides/rest-automation.md) · [Event over HTTP](guides/event-over-http.md) ·
  [Polyglot Functions](guides/polyglot-functions.md) · [Observability](guides/observability.md)
- **Knowledge Graph** — [Knowledge Graph as Application](guides/knowledge-graph/index.md) ·
  [Build Your First Graph](guides/knowledge-graph/build-your-first-graph.md) ·
  [Workflow Suspension](guides/knowledge-graph/workflow-suspension.md) ·
  [Playground & AI Companion](guides/knowledge-graph/playground-and-companion.md)
- **Concepts** — [Methodology](guides/methodology.md) · [Architecture Overview](guides/architecture.md) ·
  [Port Scope & Fidelity](background/port-scope.md) ·
  [Architecture Decision Records](arch-decisions/ADR.md)
- **Reference** — [Macros](guides/macros-reference.md) ·
  [Configuration](guides/configuration-reference.md) · [Event Envelope](guides/event-envelope-reference.md) ·
  [Flow Schema](guides/flow-schema-reference.md) · [API Overview](guides/api-overview.md)

## Project

- **Source:** [github.com/Accenture/mercury](https://github.com/Accenture/mercury)
- **Release notes:** [CHANGELOG](https://github.com/Accenture/mercury/blob/main/CHANGELOG.md)
- **Contributing:** [CONTRIBUTING](https://github.com/Accenture/mercury/blob/main/CONTRIBUTING.md)
  · [Code of Conduct](https://github.com/Accenture/mercury/blob/main/CODE_OF_CONDUCT.md)
- **Java version:** Mercury's canonical Java implementation — same three layers, same flow
  YAML, behavior-synced with this engine:
  [github.com/Accenture/mercury-composable](https://github.com/Accenture/mercury-composable)
  · [documentation](https://accenture.github.io/mercury-composable/)
- **Polyglot functions:** write functions in **Python** or **Node.js** and call them from
  flows and graphs — see [Polyglot Functions](guides/polyglot-functions.md):
  [Composable for Python](https://accenture.github.io/mercury-python/)
  · [Composable for Node.js](https://accenture.github.io/mercury-nodejs/)

!!! note "Rust port"
    Throughout this site, boxes like this mark the places where the Rust port deliberately
    differs from the Java original — no silent divergence.
