---
title: Event Script flow grammar
summary: The authoritative, rule-based reference for the Event Script flow DSL (the YAML that
  defines event flows) — flow structure, task fields, the eight execution types, and the
  data-mapping mini-language. Designed so a human or an AI agent can author flows deterministically.
layer: event-script
audience: [developer, architect, ai-agent, reference]
keywords: [event script, flow, dsl, grammar, execution type, data mapping, deterministic, yaml]
related:
  - guides/event-script/syntax.md
  - guides/event-script/ai-agent-guide.md
  - guides/knowledge-graph/command-reference.md
---

# Event Script flow grammar

> **At a glance**
>
> - The **source of truth** for the Event Script flow DSL — the rules an author (human or AI)
>   needs to write a valid flow without inferring from examples.
> - Machine-readable form: [`event-script-flow.json`](event-script-flow.json). Authoring from an
>   agent? See the [AI agent guide](ai-agent-guide.md). Tutorial/worked examples and exhaustive
>   field docs: [Event Script Syntax](syntax.md).
> - A flow is compiled and validated by the engine; the rules below mirror that validation (the
>   `CompileFlows` port, `crates/event-script/src/compiler.rs`), so violating them fails compilation.
> - **The flow YAML syntax is identical to the Java engine's** — flow files port unchanged.

## Flow file structure {#structure}

A flow is a YAML file with these top-level keys:

| Key | Required | Meaning |
|---|---|---|
| `flow.id` | **yes** | unique flow id (referenced by `rest.yaml` and by sub-flows as `flow://{id}`) |
| `flow.description` | **yes** | human-readable purpose (validated non-blank) |
| `flow.ttl` | **yes** | time-to-live, e.g. `30s` (`s`/`m`/`h`; minimum 1s) |
| `flow.exception` | no | route of a global exception handler |
| `first.task` | **yes** | the entry-point task (route name or task `name`) |
| `external.state.machine` | conditional | route or `flow://id` of an external state machine — **required** if any task uses the `ext:` namespace |
| `tasks` | **yes** | the list of task definitions; **must include ≥1 task with `execution: end`** |

## Task fields {#task-fields}

Each entry under `tasks:` may have:

| Field | Required | Meaning |
|---|---|---|
| `process` | conditional | route of the composable function, or `flow://{flow-id}` for a sub-flow (either `process` or `name` required) |
| `name` | conditional | unique task id; **defaults to `process`** — required only when the same `process` is used more than once |
| `description` | **yes** | non-blank purpose |
| `input` | **yes** | list of `source -> target` mapping rules (use `[]` for none) |
| `output` | **yes** | list of `source -> target` mapping rules (use `[]` for none) |
| `execution` | **yes** | one of the [execution types](#execution-types) below |
| `next` | conditional | next task(s) — required/shaped by execution type |
| `join` | conditional | the join task — **required** for `fork` |
| `pipeline` | conditional | ordered task list — **required** for `pipeline` |
| `loop` | no | loop control (`statement`/`condition`) for a `pipeline` |
| `source` | no | a `model.*` list to iterate for a dynamic `fork` |
| `delay` | no | ms (int) or a `model.*` var; must be < `flow.ttl` |
| `exception` | no | task-level exception handler route (overrides `flow.exception`) |

## Execution types {#execution-types}

The eight values `execution:` may take (authoritative set — `EXECUTION_TYPES` in
`crates/event-script/src/compiler.rs`). `next` requirements are enforced at compile time:

| `execution` | `next` | also requires | forbids | does |
|---|---|---|---|---|
| `sequential` | exactly 1 | — | — | run, then go to the one next task |
| `decision` | ≥ 2 | output maps `-> decision` | `join`,`pipeline`,`source` | branch on the `decision` value (`true`→next[0]/`false`→next[1]; or numeric 1-indexed, multi-way) |
| `parallel` | ≥ 2 | — | `join` | fan out to all next tasks concurrently |
| `fork` | ≥ 1 | `join` | `pipeline` | run next tasks concurrently, resume at `join`; optional `source` to iterate |
| `pipeline` | exactly 1 | `pipeline` list | `join` | run the pipeline tasks in order (optional `loop`), then the one next task |
| `response` | exactly 1 | — | `join` | send the HTTP response now, then continue async to next |
| `end` | **none** | — | `next`,`join`,… | terminate; `output` mappings form the final response |
| `sink` | **none** | — | `next`,… | terminal branch (no response) — e.g. a fork/parallel leaf |

## Data-mapping mini-language {#mapping}

Every `input`/`output` entry is `'source -> target'` (one `->` per rule; a three-part
`'LHS -> model.var -> RHS'` is allowed and compiles to two).

**Source (LHS) namespaces**

| Namespace | Use |
|---|---|
| `input.*` | the HTTP request dataset (`input.body`, `input.header.*`, `input.query.*`, `input.path_parameter.*`, `input.method`, …) |
| `model.*` | flow-instance state (dot/bracket/`{model.key}` dynamic keys) |
| `model.parent.*` / `model.root.*` | parent-flow state (in sub-flows) |
| `error.*` | exception context in a handler (`error.task/.status/.message/.stack`) |
| `$.…` | a JSONPath expression |
| `result` / `input` / `header` / `status` / `datatype` | (in `output` rules) the function's result, the task input, response headers, status code, or result type name |
| constants | `text(…)`, `int(…)`, `long(…)`, `float(…)`, `double(…)`, `boolean(…)`, `map(k=v,…)`, `file(text:/json:/binary:path)`, `classpath(…)` |
| `f:fn(args)` | a simple plugin function |

**Target (RHS) namespaces**

| Namespace | Use |
|---|---|
| `output.*` | the HTTP response (`output.body`, `output.header.*`, `output.status`) |
| `model.*` (+ `model.parent.*`) | store in flow / parent state (`[]` appends to a list) |
| `decision` | (in a `decision` task) the branch selector |
| `file(path)` / `file(append:path)` | write/append a file |
| `ext:…` | external state machine (needs `external.state.machine`) |

**Input vs. output targets (this trips up authors).** In an **`input`** mapping the right-hand side
is the **function's input body** — written with **no namespace** (e.g. `model.order -> order` puts
`order` into the function's input body), or `header.*` for input headers. In an **`output`** mapping
the right-hand side uses the namespaces above. On the **left** of an `output` mapping, `result`
(bare) is the **whole** function result, `result.x` a field, and `status`/`header`/`datatype` other
function outputs. `model.*` always needs a **specific key** — no whole-`model` access.

**Type-conversion suffixes — DEPRECATED; do not generate.** The legacy "simple type matching"
suffixes on a source (`:text` `:int` `:long` `:float` `:double` `:boolean` `:binary` `:b64` `:!`
`:uuid` `:length` `:substring(a[,b])` `:concat(…)` `:and(model.k)` `:or(model.k)`) still parse for
backward compatibility with existing flows, but are **deprecated** — author new flows without them
(shape values in the function itself, or use an `f:` simple-plugin function).

**Special tokens:** `->` (map), `[]` (append), `*` (whole-object map), `{model.key}` (runtime
interpolation in `text()`), `.ITEM`/`.INDEX` (dynamic-fork iteration). The full catalog with
examples is in [Event Script Syntax](syntax.md#tasks-and-data-mapping).

## Triggering, chaining & exceptions {#flow}

- Tasks are referenced — in `first.task`, `next`, `join`, and `pipeline` — by their **`name`**
  (which defaults to `process`).
- The engine runs `first.task`, then chains by execution type (`next`/`join`/`pipeline`).
- A `decision` task routes by the value its `output` maps into `decision`.
- `flow.exception` catches unhandled task errors; a task's own `exception` overrides it; the handler
  reads `error.*`. The built-in `resilience.handler` adds retry/backoff.

## Invariants {#invariants}

Compile-time rules — violate them and the flow won't load:

1. `flow.id`, `flow.description`, `flow.ttl`, `first.task`, and `tasks` are all present; `ttl ≥ 1s`.
2. Every task has `description`, `input`, `output`, and a valid `execution`.
3. There is **≥1 `end`** task.
4. `decision` needs **≥2** `next`; `sequential`/`response`/`pipeline` need **exactly 1**; `end`/`sink`
   have **no** `next`.
5. `fork` requires `join`; `pipeline` requires a `pipeline` list.
6. A `name` is required when the same `process` appears in more than one task.
7. `ext:` targets require `external.state.machine` to be declared.

## See also {#see-also}

- [`event-script-flow.json`](event-script-flow.json) — machine-readable form of this grammar.
- [AI agent guide](ai-agent-guide.md) — authoring flows deterministically from an agent.
- [Event Script Syntax](syntax.md) — worked examples and exhaustive field docs.
- [Function AI agent guide](../event-driven/ai-agent-guide.md) — writing the functions tasks call (Rust API).
