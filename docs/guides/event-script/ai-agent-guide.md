---
title: AI agent guide — authoring Event Script flows
summary: The authoritative context an AI agent needs to author Event Script flow YAML
  deterministically — the compile contract, a pre-write checklist, a recipe, and a worked example.
layer: event-script
audience: [ai-agent, developer]
keywords: [event script, flow, authoring, context engineering, deterministic, yaml, compile]
related:
  - guides/event-script/flow-grammar.md
  - guides/event-script/syntax.md
---

# AI agent guide — authoring Event Script flows

> **At a glance**
>
> - **Read this if you are an AI agent** asked to write or modify an Event Script flow. It is the
>   context you need — you should not have to read the engine source.
> - **Generate from rules.** The [flow grammar](flow-grammar.md) and its machine-readable form
>   [`event-script-flow.json`](event-script-flow.json) are the source of truth. Validate against them.
> - A flow is a **YAML file compiled and validated by the engine** (the `CompileFlows` port,
>   `crates/event-script/src/compiler.rs`). Your job is to produce one that passes — the
>   [invariants](flow-grammar.md#invariants) are the contract.
> - **The flow YAML syntax is identical to the Java engine's** — flow files port between the two
>   unchanged. Only the composable *functions* a flow calls are written differently (Rust — see the
>   [function AI agent guide](../event-driven/ai-agent-guide.md)).

## How flows are deployed & triggered {#deploy}

A flow is not called directly:

1. Write the flow YAML under the application's `resources/flows/` folder (an app is a standalone
   `examples/<name>/` workspace crate with `resources/` beside its `Cargo.toml`).
2. Register its file in the **`resources/flows.yaml`** manifest (config key `yaml.flow.automation`,
   default `classpath:/flows.yaml`).
3. Map an HTTP endpoint to it in **`rest.yaml`** (`flow: {flow-id}`) — or trigger it by event.

The engine compiles every registered flow at startup; a flow that violates the grammar fails to
load. So correctness is checkable *before* runtime.

## Generate deterministically {#deterministic}

Use [`event-script-flow.json`](event-script-flow.json) to look up the exact execution types, task
fields, and mapping namespaces; use [`flow-grammar.md`](flow-grammar.md) for the rules. Then verify:

> **Pre-write checklist**
> - [ ] `flow.id`, `flow.description`, `flow.ttl` (≥1s), `first.task`, and `tasks` are present.
> - [ ] Every task has `description`, `input`, `output`, and a valid `execution`.
> - [ ] There is **≥1 task with `execution: end`**.
> - [ ] `next` matches the execution type: `decision` ≥2; `sequential`/`response`/`pipeline` exactly 1;
>       `end`/`sink` none.
> - [ ] `fork` has a `join`; `pipeline` has a `pipeline` list; a `decision` task's `output` maps a value
>       into `decision`.
> - [ ] If the same `process` is used twice, each such task has a unique `name`.
> - [ ] Every `source -> target` uses valid namespaces (see the grammar's mapping section);
>       `ext:` targets require `external.state.machine`.

## Recipe {#recipe}

1. **Header:** set `flow.id`, `description`, `ttl`, optional `exception`; pick `first.task`.
2. **Tasks:** for each, set `process` (or `flow://`), `description`, `input`, `output`, `execution`,
   and the `next`/`join`/`pipeline` its type requires.
3. **Terminate:** ensure at least one `end` task whose `output` builds the response.
4. **Wire up:** register in `flows.yaml`; add a `rest.yaml` mapping if HTTP-facing.

## Worked example {#example}

A minimal flow: receive a POST, call a `greeting` function, return its result.

```yaml
flow:
  id: 'greeting-flow'
  description: 'Greet a caller by name'
  ttl: 10s
first.task: 'greeting.service'
tasks:
  - input:
      - 'input.body.name -> name'
    process: 'greeting.service'
    output:
      - 'result.message -> output.body.message'
    description: 'Build the greeting'
    execution: end
```

A decision example (branch on a function result):

```yaml
  - input:
      - 'input.body.amount -> amount'
    process: 'risk.scorer'
    output:
      - 'result.high_risk -> decision'
    description: 'Score and branch'
    execution: decision
    next:
      - 'review.task'       # decision = true (high_risk) -> next[0] (first)
      - 'approve.task'      # decision = false            -> next[1] (second)
```

A working end-to-end app lives at `examples/hello-flow/` — two functions, one flow YAML, a
`rest.yaml` binding, and a decision branch (`lang=fr`).

## See also {#see-also}

- [Event Script flow grammar](flow-grammar.md) + [`event-script-flow.json`](event-script-flow.json) — the source of truth.
- [Event Script Syntax](syntax.md) — worked examples and the full data-mapping catalog.
- [Function AI agent guide](../event-driven/ai-agent-guide.md) — writing the composable functions a flow's tasks call (Rust API).
