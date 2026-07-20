# Flow Schema Reference

The field-level reference for Event Script flow configuration files: every field the flow
compiler (`crates/event-script/src/compiler.rs`) accepts, and the validation it applies.
Flows are compiled and validated at application startup, so this page is how you author flows
the engine will accept. **The flow YAML syntax is identical to the Java engine's** — flow
files port between the two implementations unchanged.

This page is deliberately terse. The rules-at-a-glance summary is the
[flow grammar](event-script/flow-grammar.md); the tutorial with worked examples and the full
data-mapping catalog is [Event Script Syntax](event-script/syntax.md).

## The flow list (`flows.yaml`)

The engine discovers flows through one or more flow-list files, named by the
`yaml.flow.automation` application property (default `classpath:/flows.yaml`; a
comma-separated list loads several):

```yaml
yaml.flow.automation: 'classpath:/flows.yaml, classpath:/more-flows.yaml'
```

#### `flows`

| Type | Required |
|---|---|
| list | yes |

File names of the individual flow configuration files. Only `.yml`/`.yaml` names are accepted
(anything else is logged and ignored); the list is de-duplicated and loaded in sorted order.

#### `location`

| Type | Default |
|---|---|
| string | `classpath:/flows/` |

Base location for resolving the file names. The shipped `examples/hello-flow` list:

```yaml
flows:
  - 'hello-flow.yml'

location: 'classpath:/flows/'
```

**Load-time failure semantics** (resilient by design): an unreadable list file is a warning
and that location is skipped; an invalid flow is an error and that flow is skipped; an invalid
*data mapping* drops only the offending **task** while the rest of the flow loads — the
runtime then fails fast if the flow ever reaches the missing task.

## Flow-level fields

#### `flow.id`

| Type | Required |
|---|---|
| string | yes |

Unique identifier across all loaded flows (a duplicate id rejects the later file). Referenced
by `rest.yaml` ([`flow:` binding](rest-automation.md#flow)), by sub-flow tasks as
`flow://{id}`, and by knowledge-graph delegation (`extension=flow://{id}`). The Playground's
`list flows` command shows each deployed id with its description.

#### `flow.description`

| Type | Required |
|---|---|
| string | yes |

Human-readable purpose of the flow — displayed next to the id by `list flows`.

#### `flow.ttl`

| Type | Required |
|---|---|
| duration | yes |

Maximum wall-clock lifetime of a flow instance; on expiry the instance is aborted. Written
with an `s`/`m`/`h`/`d` suffix (`30s`, `2m`) or as bare seconds; the minimum is 1 second.

#### `flow.exception`

| Type | Required |
|---|---|
| string | no |

Route name of the global exception handler for this flow. See
[Exception handling](#exception-handling).

#### `first.task`

| Type | Required |
|---|---|
| string | yes |

The task where execution starts, referenced by task name (which defaults to `process`).

#### `external.state.machine`

| Type | Required |
|---|---|
| string | conditional |

Route name (or `flow://{id}`) of an external state-machine service. **Required** whenever any
task maps to the `ext:` namespace — the compiler rejects the flow otherwise. See
[Event Script Syntax](event-script/syntax.md) for the `ext:` write/delete/append forms.

#### `tasks`

| Type | Required |
|---|---|
| list | yes |

The task definitions. The list must be non-empty and must contain **at least one task with
`execution: end`**, or the flow is rejected.

A minimal valid flow:

```yaml
flow:
  id: 'my-flow'
  description: 'Example flow'
  ttl: 30s

first.task: 'my.service'

tasks:
  - input:
      - 'input.body -> *'
    process: 'my.service'
    description: 'Call my service'
    output:
      - 'result -> output.body'
    execution: end
```

## Task fields

#### `process`

| Type | Required |
|---|---|
| string | conditional |

Route name of the composable function this task runs, or `flow://{flow-id}` to invoke a
[sub-flow](#sub-flows). Either `process` or `name` must be present. Any other `scheme://`
form is rejected, and when `name` is a `flow://` URI, `process` (if given) must be a
`flow://` URI too.

#### `name`

| Type | Default |
|---|---|
| string | value of `process` |

Unique task identifier within the flow — this is the name used by `first.task`, `next`,
`join`, and `pipeline` references. Required only when the same `process` route appears in
more than one task.

#### `description`

| Type | Required |
|---|---|
| string | yes |

Non-blank purpose of the task; validated at compile time.

#### `input` / `output`

| Type | Required |
|---|---|
| list | yes |

Data-mapping rules, each a `'source -> destination'` string (use `[]` for none). A three-part
rule `'LHS -> model.var -> RHS'` compiles into two rules. The namespace catalog —
`input.*`, `model.*`, `error.*`, constants, `f:` plugins, JSONPath on the left; the function
input body, `output.*`, `model.*`, `file(…)`, `ext:` on the right — lives in the
[flow grammar](event-script/flow-grammar.md#mapping) and
[Event Script Syntax](event-script/syntax.md#tasks-and-data-mapping); this page does not
repeat it. The compiler enforces:

- exactly one usable `->` per compiled rule; a source must differ from its destination;
- a task with any invalid mapping is **dropped** (error-logged) while the flow still loads;
  a rule with a dangling `.` or `:` fails the whole flow;
- the reserved state-machine keys — `model.cid`, `model.flow`, `model.instance`, `model.ttl`,
  `model.trace`, `model.none` (including nested paths beneath them), and `model.parent` /
  `model.root` as whole targets — must never be mapping destinations (writing *beneath*
  `model.parent.*` is the sub-flow shared-state mechanism and allowed);
- the `decision` destination is only valid in a `decision` task, and legacy `:type`
  conversion suffixes still parse but are deprecated — they are auto-converted to `f:` plugin
  calls with a warning.

#### `execution`

| Type | Required |
|---|---|
| string | yes |

One of the eight [execution types](#execution-types):

```text
sequential  decision  parallel  fork  pipeline  response  end  sink
```

#### `next`

| Type | Required |
|---|---|
| list | conditional |

The subsequent task name(s). Required for every type except `end` and `sink` (where it is
ignored); `sequential` and `pipeline` take exactly one entry; `decision` needs at least two.
In a `decision` task the list order **is** the routing table — see
[`decision`](#decision) — and an entry may carry the
[`@retry` keyword](#the-built-in-resilience-handler).

#### `exception`

| Type | Required |
|---|---|
| string | no |

Route of a task-level exception handler, overriding `flow.exception` for this task only.

#### `delay`

| Type | Required |
|---|---|
| int or string | no |

Delay before the task executes: an integer in milliseconds (an `ms` suffix is cosmetic and
stripped), which must be positive and **less than `flow.ttl`** — or a `model.*` variable
resolved at run time.

#### `join`

| Type | Required |
|---|---|
| string | conditional |

The task where execution resumes once all branches complete. **Required** when `execution`
is `fork`.

#### `source`

| Type | Required |
|---|---|
| string | no |

Turns a `fork` into a **dynamic fork**: a `model.*` path holding a list to iterate, creating
one branch per element. Requires exactly **one** `next` task; inside it,
`model.<source>.ITEM` is the current element and `model.<source>.INDEX` the zero-based index.

#### `pipeline`

| Type | Required |
|---|---|
| list | conditional |

Ordered list of task names to run within this task's context. **Required** when `execution`
is `pipeline`.

#### `loop`

| Type | Required |
|---|---|
| map | no |

Iteration control for a `pipeline` task, with two keys:

`statement`
:   `'for (<init>; <comparator>; <sequencer>)'` — the init (`model.n = 0`) is optional; the
    comparator uses `<`, `<=`, `>`, `>=` between `model.*` keys and/or integers; the
    sequencer is `model.n++` or `model.n--` and its key must appear in the comparator.
    Or `'while (model.key)'` — a single model key, no operators or assignment.

`condition`
:   One string or a list, each `'if (model.key) break'` or `'if (model.key) continue'`,
    evaluated before each iteration.

```yaml
loop:
  statement: 'for (model.n = 0; model.n < model.limit; model.n++)'
  condition: 'if (model.quit) break'
pipeline:
  - 'step.a'
  - 'step.b'
```

## Execution types

Worked examples for every type are in
[Event Script Syntax — task types](event-script/syntax.md#task-types); the compile-time shape
of each is:

#### `sequential`

Runs the function, then passes control to exactly one `next` task.

#### `decision`

Branches on the value its `output` maps to `decision`. `true` (or `1`) routes to the first
`next` entry, `false` (or `2`) to the second; an integer `N` is **1-based** — `N` routes to
the Nth entry, so a decision can fan to more than two tasks. Requires at least two `next`
entries and the `-> decision` output mapping; a value that is missing or out of range aborts
the flow with status 500.

#### `parallel`

Fans out to two or more `next` tasks concurrently, all sharing the same state machine. There
is no join — every branch runs to its own `end` or `sink`.

#### `fork`

Runs its `next` tasks concurrently and resumes at the mandatory [`join`](#join) task once
all branches complete. Add [`source`](#source) to iterate a model list (dynamic fork).

#### `pipeline`

Runs the tasks in the [`pipeline`](#pipeline) list sequentially within this task's context
— optionally under [`loop`](#loop) control — then passes to exactly one `next` task.

#### `response`

Sends the HTTP response immediately from its `output` mappings, then continues to its one
`next` task asynchronously — acknowledge now, keep working.

#### `end`

Terminates the flow; its `output` mappings form the final response (unless a `response` task
already sent one). Every flow must contain at least one `end` task.

#### `sink`

A terminal task with no outbound connection and no response — the leaf of a `parallel` or
`fork` branch.

## Sub-flows

A task whose `process` is `flow://{flow-id}` invokes another flow as a subroutine: the task's
input mapping feeds the child's `input.*` dataset, the child's final response becomes the
task's `result`, and the child can reach the parent's state machine through
`model.parent.<key>` (`model.root.<key>` is an alias of the same object). See
[Event Script Syntax — hierarchy of flows](event-script/syntax.md#hierarchy-of-flows).

## Exception handling

`flow.exception` names a global handler that catches any unhandled task error; a task's own
`exception` field overrides it for that task. A handler is an ordinary task (usually
`execution: end`) whose input mapping reads the error dataset:

```yaml
- input:
    - 'error.code -> status'
    - 'error.message -> message'
  process: 'v1.exception.handler'
  description: 'Return a structured error response'
  output:
    - 'result.status -> output.status'
    - 'result -> output.body'
  execution: end
```

`error.task`
:   Name of the task that failed.

`error.code`
:   The status code of the error.

`error.message`
:   The error message (a nested sub-flow error is unwrapped to its message).

!!! note "Rust port"
    The Java reference documents these keys as `error.status` and `error.stack`. The engine's
    error dataset — here exactly as in the canonical fixture flows — uses **`error.code`**
    for the status code, and the Rust engine produces **no stack trace**: an `error.stack`
    mapping is tolerated but resolves null.

If the flow-level handler itself throws, the flow aborts rather than looping (exception-loop
guard). Without any handler, the flow aborts and the caller receives the error as
`{type: "error", status, message}` with the matching HTTP status. The built-in
`simple.exception.handler` can serve as a ready-made handler: it logs the error context and
echoes that same shape for the response mapping.

### The built-in resilience handler

`resilience.handler` implements retry with backoff as a `decision` task. Wire it as a task's
`exception:` handler; it answers with `decision` **1** (retry), **2** (abort), or **3**
(alternative path), so its `next` list is `[retry-target, abort-task, alternative-task]`.
In the retry slot, the keyword `@retry` re-executes the task that failed;
`'my.task | @retry'` falls back to `my.task` when the failed task cannot be resolved.
Condensed from the canonical `resilience-demo` fixture (`crates/event-script/tests/resources/flows/resilience-demo.yml`):

```yaml
- input:
    - 'error.code -> status'
    - 'error.message -> message'
    - 'model.attempt -> attempt'
    - 'int(3) -> max_attempts'
    - 'int(500) -> delay'
    - 'text(401, 403-404) -> alternative'
  process: 'resilience.handler'
  description: 'Retry, abort, or reroute'
  output:
    - 'result.attempt -> model.attempt'
    - 'result.decision -> decision'
  execution: decision
  next:
    - 'my.task | @retry'
    - 'abort.request'
    - 'alternative.task'
```

Its input contract (all optional except the failure context):

`status` / `message`
:   The failure being handled (map from `error.code` / `error.message`). A `status` of 200
    is gatekeeper mode — proceed immediately with decision 1.

`attempt` / `max_attempts`
:   Retry bookkeeping: map `result.attempt` back to `model.attempt`; when the attempt count
    exceeds `max_attempts` the handler decides 2 (abort) with the original status and message.

`delay`
:   Milliseconds between retries (floor 10; the first attempt is immediate).

`alternative`
:   Status codes or ranges (`'401, 403-404'`) that route to decision 3 instead of a retry.

`cumulative` + `backoff_trigger` / `backoff_seconds` / `backoff`
:   Backoff control: when the cumulative failure count exceeds the trigger, the handler
    decides 2 with status 503 and a `result.backoff` timestamp; map `result.cumulative` and
    `result.backoff` to durable storage (the fixture uses `file(…)`) and feed them back in.

## Built-in service routes

| Route | Purpose |
|---|---|
| `no.op` | Pass-through echo function — a routing-only task (500 instances by default; tune with `worker.instances.no.op`). |
| `simple.exception.handler` | Logs the error context and returns `{type, status, message}`. |
| `resilience.handler` | Retry / abort / reroute decisions, as above. |
| `async.http.request` | The [built-in HTTP client](actuators-and-http-client.md#the-async-http-client-asynchttprequest) as a flow task. |
| `http.flow.adapter` | The HTTP-to-flow bridge used by [`rest.yaml` flow binding](rest-automation.md#flow) — not called from tasks. |

## See also

- [Flow grammar](event-script/flow-grammar.md) — the same schema as compact rules, with the
  data-mapping mini-language.
- [Event Script Syntax](event-script/syntax.md) — the tutorial: worked examples for every
  execution type and mapping form.
- [REST Automation](rest-automation.md) — binding HTTP endpoints to flows.

---

*Adapted from the mercury-composable guide `docs/guides/flow-schema-reference.md`; behavior
verified against this repository's source.*
