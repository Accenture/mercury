# The Mercury Family

**Advanced software foundations for human–AI collaboration** — Accenture open source,
Apache-2.0.

Mercury is a family of open-source foundations for building software *with* AI, not merely
faster: composable engines whose applications are assembled from self-contained functions and
YAML-configured event flows — a shape that human developers and AI agents read equally well —
plus a shared memory layer that keeps every human and every AI agent on a project oriented on
the same intent and state.

## Members

| Member | Role | Repository | Documentation |
| --- | --- | --- | --- |
| **agent-memory** | Vendor-neutral shared AI memory + cognitive loop — no-code, markdown-only. Gives any repository one committed `memory/` that every AI agent and every contributor shares, with deterministic decay, supersession and a human-gated intent trace. Joined the family September 10, 2026. | [Accenture/mercury-go](https://github.com/Accenture/mercury-go) | [accenture.github.io/mercury-go](https://accenture.github.io/mercury-go/) |
| **mercury-composable** | The Java engine — composable, event-driven applications in three layers: Platform Core, Event Script, Active Knowledge Graph. | [Accenture/mercury-composable](https://github.com/Accenture/mercury-composable) | [accenture.github.io/mercury-composable](https://accenture.github.io/mercury-composable/) |
| **mercury** | The Rust engine — a twin implementation of mercury-composable: same three layers, same flow YAML (flow files port unchanged). | [Accenture/mercury](https://github.com/Accenture/mercury) | [accenture.github.io/mercury](https://accenture.github.io/mercury/) |
| **mercury-python** | Python language pack — write composable functions in Python and call them from flows and knowledge graphs. | [Accenture/mercury-python](https://github.com/Accenture/mercury-python) | [accenture.github.io/mercury-python](https://accenture.github.io/mercury-python/) |
| **mercury-nodejs** | Node.js language pack — write composable functions in Node.js and call them from flows and knowledge graphs. | [Accenture/mercury-nodejs](https://github.com/Accenture/mercury-nodejs) | [accenture.github.io/mercury-nodejs](https://accenture.github.io/mercury-nodejs/) |

## How the pieces fit

- The **engines** (mercury-composable, mercury) run the application: functions addressed by
  route name exchange immutable events, YAML flows sequence them, and a knowledge graph
  provides the semantic layer.
- The **language packs** (mercury-python, mercury-nodejs) extend the engines so functions can
  be written in Python or Node.js and called from the same flows and graphs.
- **agent-memory** carries the *project's* memory — Vision, Blueprint, continuity, session
  records, architecture decisions — so the humans and AI agents building on the engines
  orient on purpose and state, not only on API shape. Every Mercury repository is itself
  AI-enabled with agent-memory: the family uses its own memory layer.

## Using the brand

- **Short form:** *the Mercury family — advanced software foundations for human–AI collaboration.*
- **Member sentence** (for a member repository's README or site footer):

> Part of the **Mercury family** — Accenture's open-source foundations for human–AI
> collaboration: [agent-memory](https://accenture.github.io/mercury-go/) ·
> [mercury-composable](https://accenture.github.io/mercury-composable/) ·
> [mercury](https://accenture.github.io/mercury/) ·
> [mercury-python](https://accenture.github.io/mercury-python/) ·
> [mercury-nodejs](https://accenture.github.io/mercury-nodejs/).

All members are published by Accenture under the Apache License 2.0.
