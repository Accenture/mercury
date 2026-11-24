---
title: Flow Configuration Schema
summary: Precise field-level reference for flow YAML — every flow/task field, execution type, and
  data-mapping namespace.
layer: event-script
audience: [developer, reference]
keywords: [flow schema, reference, task fields, execution types, data mapping, yaml]
---

# Flow Configuration Schema Reference

*Reference: Complete field-level reference for Mercury Composable Event Script flow configuration YAML files.*

> **At a glance**
>
> - **What** — every field, execution type, and data-mapping namespace available in flow YAML,
>   documented at the field level.
> - **Why it matters** — flows are validated at compile time, so a precise field reference is how
>   you author flows the engine will accept.
> - **For** developers authoring Event Script flows (pair with the tutorial in [Event Script Syntax](event-script/syntax.md)).
> - **The flow YAML syntax is identical to the Java engine's** — flow files port between the two
>   engines unchanged. Where this engine differs, a note marked *Rust port* says so.

> **Note**: Due to the dynamic nature of data mapping expressions, a formal JSON Schema is
> not provided. Validation of flow configurations is performed at compile time by the Mercury
> flow engine.

This page documents every field, namespace, and syntax element available in flow
configuration files. For a tutorial-style introduction with worked examples, see
[Event Script Syntax](event-script/syntax.md). For the rules at a glance, see the
[flow grammar](event-script/flow-grammar.md). For the `#[preload]` attribute used to register
task functions, see the [Macros Reference](macros-reference.md).

---

## Flow list file (`flows.yaml`)

The flow list file is an index that tells the engine which flow configuration files to load.
It is identified by the `yaml.flow.automation` application property.

### `flows`

| Type | Required |
|---|---|
| list of strings | Yes |

File names of individual flow configuration YAML files to load. Only `.yml`/`.yaml` names are
accepted (anything else is logged and ignored); the list is de-duplicated and loaded in sorted order.

### `location`

| Type | Required |
|---|---|
| string | No |

Base directory for resolving file names. Default: `classpath:/flows/`.

Multiple flow list files can be specified as a comma-separated list:

```yaml
# application.properties
yaml.flow.automation=classpath:/flows.yaml, classpath:/more-flows.yaml
```

Example `flows.yaml`:

```yaml
location: 'classpath:/flows/'
flows:
  - 'get-profile.yml'
  - 'create-profile.yml'
  - 'delete-profile.yml'
```

**Load-time failure semantics** (resilient by design): an unreadable list file is a warning
and that location is skipped; an invalid flow is an error and that flow is skipped; an invalid
*data mapping* drops only the offending **task** while the rest of the flow loads — the
runtime then fails fast if the flow ever reaches the missing task.

---

## Flow-level fields

These fields appear at the root of every individual flow configuration file.

### `flow.id`

| Type | Required |
|---|---|
| string | **Yes** |

Unique identifier for this flow. Referenced by REST config (`flow` field in `rest.yaml`) and subflow calls (`flow://flow-id`). Must be unique across all loaded flows: a duplicate id rejects the later file.

### `flow.description`

| Type | Required |
|---|---|
| string | **Yes** |

Human-readable description of the flow's purpose. Validated at compile time.

### `flow.ttl`

| Type | Required |
|---|---|
| duration | **Yes** |

Maximum wall-clock time for the flow to complete. Minimum 1 second. Accepted units: `s` (seconds), `m` (minutes), `h` (hours), `d` (days); a bare number is seconds. Example: `30s`.

### `flow.exception`

| Type | Required |
|---|---|
| string | No |

Route name of the global exception handler function for this flow. Receives the error dataset when any task throws an unhandled exception.

### `first.task`

| Type | Required |
|---|---|
| string | **Yes** |

Route name (or task `name`) of the first task to execute when the flow starts.

### `external.state.machine`

| Type | Required |
|---|---|
| string | Conditional |

Route name (or `flow://flow-id`) of an external state machine service. **Required** when any task uses the `ext:` output namespace.

### `tasks`

| Type | Required |
|---|---|
| list | **Yes** |

Ordered list of task definitions. See [Task-level fields](#task-level-fields). The list must not be
empty and must contain **at least one task with `execution: end`**, or the flow is rejected.

Minimal valid flow:

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

---

## Task-level fields

Each entry in the `tasks` list defines a step in the flow.

### `name`

| Type | Required | Default |
|---|---|---|
| string | Conditional | value of `process` |

Unique task identifier within this flow. **Required** when the same `process` route is used more than once; otherwise the `process` value is used as the name.

### `description`

| Type | Required | Default |
|---|---|---|
| string | **Yes** | — |

Human-readable description of what this task does. Validated at compile time.

### `input`

| Type | Required | Default |
|---|---|---|
| list of strings | **Yes** | — |

Input data mapping rules. Use `[]` for no input. See [Data mapping syntax](#data-mapping-syntax).

### `output`

| Type | Required | Default |
|---|---|---|
| list of strings | **Yes** | — |

Output data mapping rules. Use `[]` for no output. See [Data mapping syntax](#data-mapping-syntax).

The compiler enforces these rules on both lists:

- exactly one usable `->` per compiled rule; a source must differ from its destination;
- a task with any invalid mapping is **dropped** (error-logged) while the flow still loads;
  a rule with a dangling `.` or `:` fails the whole flow;
- the reserved state-machine keys — `model.cid`, `model.flow`, `model.instance`, `model.ttl`,
  `model.trace`, `model.none`, `model.run` (including nested paths beneath them), and
  `model.parent` / `model.root` as whole targets — must never be mapping destinations
  (writing *beneath* `model.parent.*` is the sub-flow shared-state mechanism and allowed;
  `model.run` is written only by the knowledge graph's `graph.resume` skill);
- the `decision` destination is only valid in a `decision` task.

### `process`

| Type | Required | Default |
|---|---|---|
| string | Conditional | — |

Route name of the composable function to call, or `flow://flow-id` to invoke a subflow. Either `name` or `process` must be present. Any other `scheme://` form is rejected, and when `name` is a `flow://` URI, `process` (if given) must be a `flow://` URI too.

### `execution`

| Type | Required | Default |
|---|---|---|
| string | **Yes** | — |

Task execution type. See [Execution types](#execution-types).

### `next`

| Type | Required | Default |
|---|---|---|
| list of strings | Conditional | — |

Route name(s) of subsequent task(s). Required for `sequential`, `parallel`, `fork`, `pipeline`. Not used for `end` or `sink`. Decision tasks: first entry = `true`/`1`, second = `false`/`2`, then `3`,`4`,… (1-based). The compiler holds `sequential` and `pipeline` to exactly one entry and `decision` to at least two; an entry may carry the [`@retry` keyword](#built-in-resilience-handler).

### `exception`

| Type | Required | Default |
|---|---|---|
| string | No | _(flow exception)_ |

Route name of a task-level exception handler. Overrides `flow.exception` for this task only.

### `delay`

| Type | Required | Default |
|---|---|---|
| string or int | No | — |

Delay before this task executes. Integer = milliseconds; string = model variable path (e.g. `model.wait_ms`). Must be less than `flow.ttl`. A numeric delay must be positive, and an `ms` suffix is cosmetic and stripped.

Applies to regular function tasks **and** subflow tasks (`process: 'flow://...'`). On a subflow task the launch itself is deferred and the child's TTL timer only starts on delivery, so the delay consumes the parent's budget first — keep `delay + ttl` under this flow's `ttl` or the subflow timeout may no longer be catchable (the engine logs a warning). Combined with the task `ttl` override, a delayed retry loop gains spacing between attempts (backoff).

### `ttl`

| Type | Required | Default |
|---|---|---|
| string | No | _(parent TTL propagated)_ |

Deadline override for a subflow task (`process: 'flow://...'`) — rejected on regular function tasks. Duration string in `flow.ttl` syntax (e.g. `8s`); must be less than this flow's `ttl`. By default a subflow inherits the parent's full TTL with a restarted timer, so the parent always times out first and a subflow timeout is never catchable. A shorter task `ttl` makes the subflow time out first, so this task's `exception` handler (or the flow-level one) catches the 408 and can retry within the parent's remaining budget:

```yaml
- process: 'flow://payment-processor'
  execution: sequential
  ttl: 8s                        # subflow times out first (must be < flow.ttl)
  exception: 'v1.payment.retry'  # catches the 408; may route back for a retry
  # ... name / input / output / description / next as usual
```

The timeout surfaces on the parent's handler as status `408` with the message
`Flow timeout for {ms} ms`. Note that a TTL abort **bypasses the child's own
`flow.exception`** — the child cannot swallow its timeout — and the built-in
`resilience.handler` treats a missing `model.attempt` (first-ever failure) as `0`.

### `pipeline`

| Type | Required | Default |
|---|---|---|
| list of strings | Conditional | — |

Ordered list of task route names. **Required** when `execution` is `pipeline`.

### `loop`

| Type | Required | Default |
|---|---|---|
| object | No | — |

Loop configuration for pipeline tasks. See [Pipeline configuration](#pipeline-configuration).

### `join`

| Type | Required | Default |
|---|---|---|
| string | Conditional | — |

Name of the task that collects fork results. **Required** when `execution` is `fork`.

### `source`

| Type | Required | Default |
|---|---|---|
| string | No | — |

Model variable path (e.g. `model.items`) containing a list to iterate over in a dynamic fork. The fork creates one branch per list item.

> `name`, `process`, and `description` are validated: task name must not be empty, and
> `description` must not be blank.

---

## Execution types

!!! note "Rust port"
    *Forbidden* in the tables below means the field does not belong to that execution type. This
    engine's compiler does not reject every such field: `join`, `pipeline` and `loop` on another
    execution type, and `next` on `end` or `sink`, are ignored.

### `sequential`

Executes the function and then passes control to exactly one next task.

| Required | Optional | Forbidden |
|----------|----------|-----------|
| `next` (exactly 1 entry) | `delay`, `exception`, `ttl` *(sub-flow tasks only)* | — |

```yaml
- name: 'step.one'
  input:
    - 'input.body -> *'
  process: 'v1.my.service'
  description: 'Process the request body'
  output:
    - 'result -> model.data'
  execution: sequential
  next:
    - 'step.two'
```

---

### `parallel`

Fans out to multiple next tasks concurrently. Each task receives the same model state. There
is no join point — all branches run independently to their own `end` or `sink`.

| Required | Optional | Forbidden |
|----------|----------|-----------|
| `next` (2+ entries) | `delay`, `exception`, `ttl` *(sub-flow tasks only)* | `join`, `pipeline`, `loop` |

```yaml
- input:
    - 'model.data -> *'
  process: 'v1.router'
  description: 'Fan out to parallel handlers'
  output: []
  execution: parallel
  next:
    - 'handler.a'
    - 'handler.b'
    - 'handler.c'
```

---

### `fork`

Fork-and-join pattern. All `next` tasks run concurrently; execution resumes at the `join`
task once all branches complete. Also supports iterating over a list via the `source` field.

| Required | Optional | Forbidden |
|----------|----------|-----------|
| `next` (1+ entries), `join` | `source`, `delay`, `exception`, `ttl` *(sub-flow tasks only)* | `pipeline`, `loop` |

```yaml
- input: []
  process: 'no.op'
  description: 'Fork into parallel branches'
  output: []
  execution: fork
  next:
    - 'branch.a'
    - 'branch.b'
  join: 'collect.results'
```

**Dynamic fork** — iterates over a model list (`source`). Must have exactly one `next` task.
Inside that task, `model.<source>.ITEM` holds the current item and `model.<source>.INDEX`
holds the zero-based index:

```yaml
- input: []
  process: 'no.op'
  description: 'Fork over list items'
  output: []
  execution: fork
  source: 'model.items'
  next:
    - 'process.item'
  join: 'collect.results'

- name: 'process.item'
  input:
    - 'model.items.ITEM -> item'
    - 'model.items.INDEX -> idx'
  process: 'v1.item.processor'
  description: 'Process one list item'
  output: []
  execution: sink
```

---

### `decision`

Branches to one of the `next` tasks based on a value mapped to `decision` in the output.

- **Boolean decision** (two-way): `true` routes to the **first** `next` entry (`next[0]`), `false` to
  the **second** (`next[1]`).
- **Numeric decision** (multi-way, like a `switch`): the integer is **1-based** — `1` routes to `next[0]`,
  `2` to `next[1]`, `N` to `next[N-1]` — so a flow can branch to more than two tasks.

(`true` is equivalent to `1` and `false` to `2`; both select `next[0]` and `next[1]` respectively.)

| Required | Optional | Forbidden |
|----------|----------|-----------|
| `next` (2+ entries) | `delay`, `exception`, `ttl` *(sub-flow tasks only)* | `join`, `pipeline`, `loop`, `source` |

```yaml
- name: 'check.status'
  input:
    - 'model.status_code -> status'
  process: 'v1.status.checker'
  description: 'Decide next step based on status'
  output:
    - 'result -> decision'
  execution: decision
  next:
    - 'handle.success'   # decision true  (or 1)
    - 'handle.failure'   # decision false (or 2)
```

The decision value `true` or `1` routes to `handle.success` (the first `next` entry); `false` or `2`
routes to `handle.failure` (the second).

A missing value, or an integer above the number of `next` entries, aborts the flow with status 500;
`0`, a negative integer and a non-numeric value are converted to an integer clamped to a minimum of 1,
so they **silently select the first branch**.

---

### `response`

Sends an HTTP response to the caller immediately, then continues executing subsequent tasks
asynchronously. Useful for long-running background processing after acknowledging a request.

| Required | Optional | Forbidden |
|----------|----------|-----------|
| `next` (exactly 1 entry) | `delay`, `exception`, `ttl` *(sub-flow tasks only)* | `join`, `pipeline`, `loop`, `source` |

```yaml
- input:
    - 'input.body -> *'
  process: 'v1.create.record'
  description: 'Create record and respond immediately'
  output:
    - 'result -> output.body'
    - 'int(201) -> output.status'
  execution: response
  next:
    - 'v1.send.notification'
```

---

### `end`

Terminates the flow. Any output mappings set the final HTTP response.

| Required | Optional | Forbidden |
|----------|----------|-----------|
| — | `delay`, `exception`, `ttl` *(sub-flow tasks only)* | `next`, `join`, `pipeline`, `loop`, `source` |

```yaml
- input:
    - 'model.profile -> *'
  process: 'v1.get.profile'
  description: 'Fetch and return the profile'
  output:
    - 'text(application/json) -> output.header.content-type'
    - 'result -> output.body'
  execution: end
```

---

### `sink`

A terminal task with no outbound connection. Used as a branch endpoint in `parallel` or
`fork` patterns. No response is sent.

| Required | Optional | Forbidden |
|----------|----------|-----------|
| — | `delay`, `exception`, `ttl` *(sub-flow tasks only)* | `next`, `join`, `pipeline`, `loop`, `source` |

```yaml
- name: 'persist.audit'
  input:
    - 'model.event -> payload'
  process: 'v1.audit.writer'
  description: 'Persist audit record (fire and forget)'
  output: []
  execution: sink
```

---

### `pipeline`

Executes the tasks listed in `pipeline` sequentially within this task's context, then passes
control to the single `next` task. Supports optional `for` and `while` loop control. See
[Pipeline configuration](#pipeline-configuration) for loop syntax.

| Required | Optional | Forbidden |
|----------|----------|-----------|
| `next` (exactly 1 entry), `pipeline` | `loop`, `delay`, `exception`, `ttl` *(sub-flow tasks only)* | `join`, `source` |

```yaml
- input:
    - 'int(0) -> model.n'
    - 'int(5) -> model.limit'
  process: 'no.op'
  description: 'Run pipeline loop'
  output: []
  execution: pipeline
  loop:
    statement: 'for (model.n = 0; model.n < model.limit; model.n++)'
  pipeline:
    - 'step.a'
    - 'step.b'
  next:
    - 'step.final'
```

---

## Data mapping syntax

Every `input` and `output` entry is a mapping rule string in the form:

```
'source -> destination'
```

The `->` operator assigns the value of `source` to `destination`. Source and destination
must be different.

### Whole-object mapping (`*`)

Use `*` as the destination to pass the entire source object as the function's input body:

```yaml
input:
  - 'input.body -> *'
  - 'text(json) -> header.type'  # additional input headers; must come BEFORE *
```

> `*` is a destination of an input mapping only. It is not a source, and a rule's source must
> differ from its destination, so `'* -> *'` is refused; the whole `model` namespace is not a source
> either (map `model.<key>`).

### Array elements and append mode

The dot-bracket form addresses list elements on either side of `->`:

```yaml
- 'text(world) -> model.hello[0]'   # set the first element (creates the list)
- 'text(next) -> model.hello[]'     # APPEND: [] on the destination adds after the last element
- 'input.body.items[1] -> model.second'  # read the second element of a source list
```

Append mode (`[]` on the destination) is how a list is built up rule by rule — e.g. three
`text(...) -> model.list[]` rules produce a three-element list. To create a list — or any
JSON dataset — in a single statement, parse it with the
[`f:json` plugin](#plugin-functions-f-prefix):

```yaml
- 'f:json(text([])) -> model.my_empty_list'                              # empty list
- 'f:json(text({"hello": [1, 2, {"nested": "demo"}]})) -> model.data'   # nested dataset
```

### Three-part mapping

A three-part rule decomposes into two rules at compile time:

```
'LHS -> model.var -> RHS'
```
is equivalent to:
```
'LHS -> model.var'
'model.var -> RHS'
```

Useful for applying a type conversion and storing the intermediate value:

```yaml
- 'boolean(true) -> !model.bool -> negate_value'
# compiled as:
# - 'boolean(true) -> model.bool:!'
# - 'model.bool -> negate_value'
```

### Type conversion suffixes

> *Deprecated*: the suffixes are superseded by [plugin functions](#plugin-functions-f-prefix) and
> documented for backward compatibility. They still parse, and the compiler converts each one to its
> `f:` plugin call with a warning, so existing flows work unchanged. **New flows use the plugins**
> (`f:text(model.count)` instead of `model.count:text`).

Append `:qualifier` to any source reference to convert the value before mapping:

| Suffix | Converts to | Example |
|--------|-------------|---------|
| `:text` | String | `model.count:text -> label` |
| `:int` | int | `model.str:int -> count` |
| `:long` | long | `model.str:long -> ts` |
| `:float` | float | `model.str:float -> price` |
| `:double` | double | `model.str:double -> ratio` |
| `:boolean` | boolean | `model.str:boolean -> flag` |
| `:binary` | byte[] | `model.text:binary -> bytes` |
| `:b64` | Base64 string | `model.bytes:b64 -> encoded` |
| `:!` | negated boolean | `model.flag:! -> inverted` |
| `:uuid` | UUID string | `model.none:uuid -> id` |
| `:length` | int (size) | `model.list:length -> count` |
| `:substring(start)` | String | `model.text:substring(6) -> tail` |
| `:substring(start, end)` | String | `model.text:substring(0, 5) -> head` |
| `:concat(args...)` | String | `model.a:concat(text( ), model.b) -> full` |
| `:and(model.key)` | boolean | `model.p:and(model.q) -> both` |
| `:or(model.key)` | boolean | `model.p:or(model.q) -> either` |
| `:boolean(value=true)` | boolean | `model.str:boolean(yes=true) -> flag` |

> `model.none` is the built-in null constant. Any mapping from `model.none` clears the destination.

---

### Input namespaces (LHS of `input` mappings)

| Namespace | Description | Example |
|-----------|-------------|---------|
| `input` | Entire HTTP input dataset as a map | `input -> *` |
| `input.body` | Parsed request body (JSON object → Map, JSON array → List, text → String) | `input.body -> *` |
| `input.body.<field>` | Specific field from the request body | `input.body.user_id -> id` |
| `input.header` | All request headers (lowercase keys) | `input.header -> headers` |
| `input.header.<name>` | Specific request header | `input.header.authorization -> token` |
| `input.query` | All query parameters | `input.query -> params` |
| `input.query.<name>` | Specific query parameter | `input.query.page -> page` |
| `input.path_parameter.<name>` | URL path parameter | `input.path_parameter.id -> id` |
| `input.cookie.<name>` | HTTP cookie value | `input.cookie.session -> session` |
| `input.method` | HTTP method string | `input.method -> method` |
| `input.ip` | Remote IP address | `input.ip -> client_ip` |
| `input.stream` | Input stream route ID (file uploads) | `input.stream -> stream_id` |
| `input.filename` | Filename (multipart uploads) | `input.filename -> name` |
| `input.session.<key>` | Authenticated session value | `input.session.user_id -> uid` |
| `input.uri` | Request URI path | `input.uri -> uri` |
| `model.<key>` | State machine variable for this flow | `model.profile -> *` |
| `model.<key>.<field>` | Nested field in a state variable | `model.user.name -> name` |
| `model.<key>[n]` | Indexed element of a list | `model.items[0] -> first` |
| `model.<key>[model.n]` | Dynamically-indexed element | `model.items[model.n] -> item` |
| `model.parent.<key>` | Parent flow's state machine (in subflows) | `model.parent.token -> token` |
| `model.root.<key>` | Alias for `model.parent.<key>` | `model.root.user -> user` |
| `model.none` | Null constant (clears the destination) | `model.none -> model.old_key` |
| `model.trace` | Current distributed trace ID (read-only metadata) | `model.trace -> trace_id` |
| `model.flow` | ID of the flow configuration (read-only metadata) | `model.flow -> flow_id` |
| `model.instance` | ID of this flow instance (read-only metadata) | `model.instance -> instance` |
| `model.cid` | Business correlation-id of the inbound request (read-only metadata) | `model.cid -> header.cid` |
| `model.ttl` | Flow instance TTL in milliseconds (read-only metadata) | `model.ttl -> ttl` |
| `model.{model.pointer}` | Dynamic model key (resolved at runtime) | `model.{model.pointer} -> value` |
| `error.task` | Route name of the task that threw (exception handlers) | `error.task -> failed_task` |
| `error.code` | HTTP status code of the error | `error.code -> status` |
| `error.message` | Error message string | `error.message -> message` |
| `error.stack` | Stack trace (if available) | `error.stack -> stack` |
| `$.path` | JSONPath expression | `$.input.body.list[*].id -> ids` |

!!! note "Rust port"
    The flow input dataset of this engine carries `method`, `uri`, `header`, `path_parameter`,
    `query`, `body` and `ip`. Cookies, upload streams, file names and session values are not passed to
    a flow, so `input.cookie`, `input.stream`, `input.filename` and `input.session` are accepted in a
    mapping and resolve null. This engine produces **no stack trace**: `error.stack`, present on the
    Java engine when a stack trace exists, is accepted and resolves null.

---

### Function result sources (LHS of `output` mappings)

These namespaces are only valid on the left-hand side of `output` mapping rules.

| Namespace | Description | Example |
|-----------|-------------|---------|
| `result` | Entire function return value | `result -> output.body` |
| `result.<field>` | Specific field from return value | `result.count -> model.n` |
| `status` | HTTP status code from `EventEnvelope` | `status -> output.status` |
| `header` | All response headers from `EventEnvelope` | `header -> output.header` |
| `header.<name>` | Specific response header | `header.x-request-id -> model.request_id` |
| `datatype` | Class name of a PoJo result on the Java engine; this engine has no PoJo classes, so it is accepted and resolves null | `datatype -> output.header.x-type` |
| `model.<key>` | Current state machine variable | `model.cached -> output.body` |
| `input` | The flow's input dataset, as an input mapping reads it | `input -> model.saved` |
| `input.<field>` | Specific field of the flow's input dataset | `input.body.id -> model.id` |
| `$.path` | JSONPath expression on the result | `$.result.items[*].id -> ids` |

---

### Output destinations (RHS of `output` mappings)

| Namespace | Description | Example |
|-----------|-------------|---------|
| `output.body` | HTTP response body | `result -> output.body` |
| `output.body.<field>` | Set a specific field in the response body | `model.name -> output.body.user` |
| `output.header.<name>` | Set an HTTP response header | `text(application/json) -> output.header.content-type` |
| `output.header` | Set the entire header map | `header -> output.header` |
| `output.status` | Set the HTTP response status code (integer) | `int(201) -> output.status` |
| `model.<key>` | Store in flow's state machine | `result -> model.profile` |
| `model.<key>[]` | Append to a list in the state machine | `result.id -> model.id_list[]` |
| `model.parent.<key>` | Store in parent flow's state machine | `result -> model.parent.child_result` |
| `decision` | Set the decision value (decision tasks only) | `result -> decision` |
| `file(<path>)` | Write to a file at the given path | `result -> file(/tmp/output.json)` |
| `file(append:<path>)` | Append to a file | `text(line) -> file(append:/tmp/log.txt)` |
| `ext:<key>` | Write to external state machine | `result -> ext:/session/data` |

---

### Constant types (LHS sources)

Use these functions on the left-hand side of any input mapping to inject a literal value.

| Syntax | Value type | Example |
|--------|------------|---------|
| `text(value)` | text | `text(application/json) -> output.header.content-type` |
| `text(Bearer {model.token})` | text with interpolation | `text(Bearer {model.token}) -> headers.Authorization` |
| `int(value)` | integer | `int(201) -> output.status` |
| `long(value)` | long | `long(9223372036854775807) -> max` |
| `float(value)` | float | `float(3.14) -> pi` |
| `double(value)` | double | `double(3.14159265) -> pi` |
| `boolean(value)` | boolean | `boolean(true) -> enabled` |
| `map(k=v, k2=v2, ...)` | map of text values | `map(type=json, ver=2) -> config` |
| `map(app.config.key)` | the configured value (a primitive, a map or a list) | `map(my.config.section) -> settings` |
| `file(text:<path>)` | text | `file(text:/tmp/template.txt) -> template` |
| `file(json:<path>)` | map or list | `file(json:/tmp/config.json) -> config` |
| `file(binary:<path>)` | bytes | `file(binary:/tmp/image.png) -> bytes` |
| `classpath(text:<path>)` | text | `classpath(text:templates/email.txt) -> body` |
| `classpath(json:<path>)` | map or list | `classpath(json:defaults.json) -> defaults` |
| `classpath(binary:<path>)` | bytes | `classpath(binary:certs/key.der) -> cert` |

> **Template interpolation in `text()`**: `{model.key}` is replaced at runtime with the
> current value of `model.key`. Also supports `${ENV_VAR}` for environment variables and
> `${app.property}` for application properties.

---

### JSONPath expressions

Use `$.` to apply a JSONPath expression to the input or result. The grammar is
[RFC 9535](https://www.rfc-editor.org/rfc/rfc9535), the IETF standard, on both engines:

```yaml
input:
  - '$.input.body.users[*].id -> user_ids'   # a wildcard yields a list

output:
  - '$.result.items[*].name -> model.names'
```

> A wildcard search (e.g. `[*]`) yields a list of the matched values. Several such lists mapped under one
> model key (`model.list.id`, `model.list.name`) form a map of lists; use `f:listOfMap()` to turn it
> into a list of maps.
>
> A path outside the standard is refused when the flow is compiled, and a reference in it (`{model.key}`,
> `[model.i]`) is checked where it lands. See [JSON Path](event-script/syntax.md#json-path) for the grammar, the
> result shape, the checks and the replacements for Jayway-only forms.

---

## HTTP input dataset

When a flow is triggered by an HTTP request, the following fields are available under the
`input.` namespace:

| Field | Type | Description |
|-------|------|-------------|
| `method` | String | HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS) |
| `uri` | String | Request URI path |
| `header` | Map | Request headers (lowercase keys) |
| `cookie` | Map | HTTP cookies (lowercase keys) — Java engine only |
| `path_parameter` | Map | URL path parameters (as defined in `rest.yaml`) |
| `query` | Map | Query string parameters (lowercase keys) |
| `body` | Map / List / Object | Parsed request body; JSON object → Map, JSON array → List (map it with `*` as the function's whole input), text → String |
| `stream` | String | Stream route ID for file upload or large body streaming — Java engine only |
| `ip` | String | Remote IP address |
| `filename` | String | Original filename (multipart file upload) — Java engine only |
| `session` | Map | Session key-values set by an authentication function — Java engine only |

The four keys marked *Java engine only* are absent from this engine's dataset; see the *Rust port*
note under [Input namespaces](#input-namespaces-lhs-of-input-mappings).

---

## Pipeline configuration

The `loop` sub-object controls iteration for `pipeline` tasks.

### `loop.statement`

| Type | Required |
|---|---|
| string | **Yes** (if `loop` present) |

Loop control statement. Must be `for (...)` or `while (...)`.

### `loop.condition`

| Type | Required |
|---|---|
| string or list | No |

Break or continue condition(s). One or more `if (model.key) break\|continue` strings.

### For loop

```
for (<init>; <comparator>; <sequencer>)
```

| Part | Syntax | Operators |
|------|--------|-----------|
| init (optional) | `model.var = <integer>` | assignment |
| comparator | `model.var < model.max` | `<`, `<=`, `>`, `>=` |
| sequencer | `model.var++` or `model.var--` | `++`, `--` |

The comparator compares `model.*` keys and/or integers, and the sequencer's key must appear in it.

```yaml
loop:
  statement: 'for (model.n = 0; model.n < model.limit; model.n++)'
  condition:
    - 'if (model.quit) break'
    - 'if (model.skip) continue'
pipeline:
  - 'step.a'
  - 'step.b'
```

### While loop

```
while (model.key)
```

The loop continues while `model.key` is truthy. `model.key` must be a simple model variable
(no operators).

```yaml
loop:
  statement: 'while (model.running)'
  condition: 'if (model.abort) break'
pipeline:
  - 'step.a'
  - 'step.b'
```

### Loop conditions

Each condition has the form `if (model.key) break` or `if (model.key) continue`. Multiple
conditions can be a YAML list. Conditions are evaluated before each pipeline iteration.

---

## Subflows

A task can invoke another flow as a subroutine using the `flow://` protocol in `process`:

```yaml
- name: 'call.sub.flow'
  input:
    - 'model.data -> input.body'
    - 'model.token -> input.header.authorization'
  process: 'flow://child-flow-id'
  description: 'Delegate to child flow'
  output:
    - 'result -> model.child_result'
  execution: sequential
  next:
    - 'next.step'
```

| Aspect | Detail |
|--------|--------|
| Syntax | `process: 'flow://flow-id'` |
| Input | Data is mapped into `input.body` and `input.header.*` of the child flow |
| Output | The child flow's final `output.body` is available as `result` in the parent |
| Shared state | Parent state accessible via `model.parent.<key>` or `model.root.<key>` in child tasks |
| TTL | By default the parent's **full** `flow.ttl` value is copied to the subflow and the subflow's timer restarts at launch — so the parent's deadline fires first and a subflow timeout is not catchable. An optional task-level [`ttl`](#ttl) overrides this with a shorter subflow deadline, making its 408 catchable by this flow's exception handler |
| Nesting | Subflows can themselves call subflows |

> When the task name is omitted and `process` is a `flow://` URI, the `process` value
> (`flow://flow-id`) is the task name.

---

## Exception handling

### Flow-level exception handler

Declared with `flow.exception`. Catches any unhandled exception from any task in the flow.
The error dataset is available through the `error.*` namespace.

```yaml
flow:
  id: 'my-flow'
  description: 'My flow'
  ttl: 30s
  exception: 'v1.exception.handler'

# ... other fields ...

tasks:
  # ... other tasks ...

  - input:
      - 'error.code -> status'
      - 'error.message -> message'
      - 'error.task -> failed_task'
    process: 'v1.exception.handler'
    description: 'Handle flow-level exceptions'
    output:
      - 'result.status -> output.status'
      - 'result -> output.body'
    execution: end
```

`error.task` is the name of the task that failed, `error.code` the status code and `error.message`
the message (a nested sub-flow error is unwrapped to its message). There is no `error.stack` on this
engine (see the *Rust port* note under [Input namespaces](#input-namespaces-lhs-of-input-mappings)).

If the flow-level handler itself throws, the flow aborts rather than looping (exception-loop
guard). Without any handler, the flow aborts and the caller receives the error as
`{type: "error", status, message}` with the matching HTTP status. The built-in
`simple.exception.handler` can serve as a ready-made handler: it logs the error context and
echoes that same shape for the response mapping.

### Task-level exception handler

Declared with the `exception` field on an individual task. Overrides the flow-level handler
for that task only.

```yaml
- name: 'risky.task'
  input:
    - 'model.payload -> *'
  process: 'v1.risky.service'
  description: 'Risky operation with its own error handler'
  output:
    - 'result -> model.result'
  execution: sequential
  exception: 'v1.task.exception.handler'
  next:
    - 'next.step'
```

### Built-in resilience handler

The `resilience.handler` function implements retry-with-backoff. Use it as a task-level
exception handler combined with a `decision` execution type:

```yaml
- name: 'my.task'
  process: 'v1.service'
  description: 'Service call with retry'
  output:
    - 'result -> model.result'
  execution: end
  exception: 'resilience.handler'

- input:
    - 'error.code -> status'
    - 'error.message -> message'
    - 'model.attempt -> attempt'
    - 'int(3) -> max_attempts'
  process: 'resilience.handler'
  description: 'Retry decision'
  output:
    - 'result.attempt -> model.attempt'
    - 'result.decision -> decision'
  execution: decision
  next:
    - 'my.task | @retry'
    - 'abort.handler'
```

The `@retry` keyword in a `next` entry re-executes the original task. The `|` pipe operator
provides a fallback task name if `@retry` is not triggered.

The handler answers with `decision` **1** (retry), **2** (abort) or **3** (alternative path), so the
full `next` list is `[retry-target, abort-task, alternative-task]`; the third entry is needed only
when `alternative` is mapped. Its input contract (all optional except the failure context), from the
canonical `resilience-demo` fixture (`crates/event-script/tests/resources/flows/resilience-demo.yml`):

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

---

## Plugin functions (`f:` prefix)

Plugins are used on the **left-hand side** of input and output mapping rules. They compute a value
from their arguments and place it at the destination.

Syntax: `f:<name>(arg1, arg2, ...) -> destination`

Arguments can be model variables or constant types. Nested plugin calls are **not**
supported — the engine deliberately ignores an `f:` expression inside another plugin's
argument list (loop prevention); compute the inner value into a model variable first.
The argument tokenizer splits on **top-level commas only**, so commas inside a nested
constant are safe — e.g. `f:json(text({"a": 1, "b": 2}))` passes the whole JSON as one
argument.

### Arithmetic

| Function | Description | Example |
|----------|-------------|---------|
| `f:add(a, b, ...)` | Sum of arguments | `f:add(model.x, model.y) -> sum` |
| `f:subtract(a, b, ...)` | `a` minus remaining args | `f:subtract(model.total, model.fee) -> net` |
| `f:multiply(a, b)` | Product | `f:multiply(model.price, model.qty) -> total` |
| `f:div(a, b)` | Division (`a / b`) | `f:div(model.total, model.n) -> avg` |
| `f:mod(a, b)` | Remainder (`a % b`) | `f:mod(model.n, int(2)) -> rem` |
| `f:increment(a)` | `a + 1` | `f:increment(model.count) -> next_count` |
| `f:decrement(a)` | `a - 1` | `f:decrement(model.count) -> prev_count` |

**Exact decimals.** `f:add` and its family compute in `long` or `double`, which is right for counts and wrong for money. The
`decimal` family computes in exact decimal arithmetic with the same rules as the `DECIMAL` statement of `graph.math`
([skills reference](knowledge-graph/skills-reference.md#math-decimal)) and answers a **canonical decimal string**: plain
notation, the computed scale kept, a zero of any scale written `"0"`. A string survives `graph.suspend`, every event hop and
a JSON response unchanged, where a number would be rounded by the parser. An operand is a whole number, a string that is a
canonical number, or a double (taken through the shortest decimal text it prints as — **send money as strings**); anything
else, a boolean or a `null` included, is an error naming the plugin. The `f:add` family is unchanged.

| Function | Description | Example |
|----------|-------------|---------|
| `f:decimalAdd(a, b, ...)` | Exact sum; the scale is the largest operand scale | `f:decimalAdd(model.price, model.fee) -> total` |
| `f:decimalSubtract(a, b, ...)` | `a` minus the remaining args, exactly | `f:decimalSubtract(model.total, model.fee) -> net` |
| `f:decimalMultiply(a, b, ...)` | Exact product; the scale is the sum of the scales | `f:decimalMultiply(model.price, model.qty) -> amount` |
| `f:decimalDiv(a, b)` | Never truncates: the exact quotient when it terminates, otherwise 34 significant digits, half-even; division by zero is an error | `f:decimalDiv(model.total, model.n) -> share` |
| `f:decimalMod(a, b)` | Exact remainder; division by zero is an error | `f:decimalMod(model.cents, int(100)) -> rem` |
| `f:decimalRound(x, scale, mode)` | Rounding is always explicit: `mode` is `text(HALF_UP)`, `HALF_EVEN`, `HALF_DOWN`, `UP`, `DOWN`, `CEILING` or `FLOOR`, and the result has exactly `scale` places; the one- and two-argument forms are refused | `f:decimalRound(model.fee, int(2), text(HALF_UP)) -> fee` |
| `f:decimalCompare(a, b)` | `-1`, `0` or `1` by numeric value (`2.0` equals `2.00`) | `f:decimalCompare(model.balance, text(0)) -> sign` |

### Logical and comparison

| Function | Description | Example |
|----------|-------------|---------|
| `f:eq(a, b, ...)` | `a == b` → boolean; optional `text(ignoreCase)` and/or `text(ignoreType)` modifiers compare case-insensitively / by text form (`"123" == 123`, `"true" == true`) | `f:eq(model.status, text(ok), text(ignoreCase)) -> is_ok` |
| `f:ne(a, b, ...)` | `a != b` → boolean; same optional modifiers as `f:eq` | `f:ne(model.status, text(deleted)) -> is_live` |
| `f:gt(a, b)` | `a > b` → boolean | `f:gt(model.n, int(10)) -> over_limit` |
| `f:lt(a, b)` | `a < b` → boolean | `f:lt(model.n, int(0)) -> negative` |
| `f:and(a, b, ...)` | Logical AND | `f:and(model.ready, model.valid) -> ok` |
| `f:or(a, b, ...)` | Logical OR | `f:or(model.err1, model.err2) -> has_error` |
| `f:not(a)` | Logical NOT | `f:not(model.flag) -> inverted` |
| `f:ternary(cond, a, b)` | `cond ? a : b` | `f:ternary(model.ok, text(pass), text(fail)) -> label` |
| `f:isNull(a)` | `a == null` → boolean | `f:isNull(model.opt) -> is_missing` |
| `f:notNull(a)` | `a != null` → boolean | `f:notNull(model.opt) -> is_present` |

### Type conversion

| Function | Description | Example |
|----------|-------------|---------|
| `f:text(a)` | Convert to String | `f:text(model.n) -> label` |
| `f:int(a)` | Convert to int | `f:int(model.str) -> count` |
| `f:long(a)` | Convert to long | `f:long(model.str) -> ts` |
| `f:float(a)` | Convert to float | `f:float(model.str) -> price` |
| `f:double(a)` | Convert to double | `f:double(model.str) -> ratio` |
| `f:boolean(a)` | Convert to boolean | `f:boolean(model.str) -> flag` |
| `f:binary(a)` | Convert to byte[] | `f:binary(model.text) -> bytes` |
| `f:b64(a)` | Base64 encode/decode | `f:b64(model.bytes) -> encoded` |
| `f:json(a)` | Parse JSON text — object `{...}` → map, array `[...]` → list | `f:json(text([])) -> empty_list` |

### Decision table

| Function | Description | Example |
|----------|-------------|---------|
| `f:lookup(table, value, default)` | Name of the first rule of a decision table whose list contains the value (text comparison, case-insensitive); the optional default on a miss, null when omitted. `table.keys` lists the rules in priority order, each rule field lists its values — lists or JSON arrays as text, or the whole table as JSON text | `f:lookup(model.rules, input.body.state, text(unknown)) -> rule` |
| `f:rangeLookup(table, value, default)` | Name of the first band of a range table whose range contains the value; the optional default on a miss or a null value, null when omitted. Each band field is one range as text: `min..max` from `min` inclusive up to but excluding `max`, an open end omitted. A bound is a canonical number, quoted text (single or double quotes, compared as written and case-sensitively) or unquoted text (compared without case; text orders by Unicode code point); numeric bounds compare the value exactly as a decimal (a non-number fails by name); the value must be a number or text (a boolean, a list or a map fails by name); a table mixing kinds, or a band given as a list, is refused | `f:rangeLookup(model.tiers, input.body.annual_spend, text(unknown)) -> tier` |
| `f:minRange(range, default)` | Lower bound of one range written as `min..max`, `min..` or `..max`, typed by the notation: a number written without a decimal point, within the 64-bit range, a long, any other number its exact decimal as text (`f:double` converts it for a floating-point output), a text bound as written, a quoted one without its quotes; an open end answers the optional default, null when omitted; a null range answers null, never the default. A malformed, list, mixed or empty range is refused as `f:rangeLookup` refuses the band | `f:minRange(model.tiers.gold) -> gold_from` |
| `f:maxRange(range, default)` | Upper bound of one range — the first value the range leaves out — otherwise as `f:minRange` | `f:maxRange(model.tiers.gold) -> gold_to` |

### String operations

| Function | Description | Example |
|----------|-------------|---------|
| `f:concat(a, b, ...)` | Concatenate strings | `f:concat(model.first, text( ), model.last) -> name` |
| `f:substring(str, start)` | Substring from start | `f:substring(model.text, int(6)) -> tail` |
| `f:substring(str, start, end)` | Substring range | `f:substring(model.text, int(0), int(5)) -> head` |
| `f:length(a)` | Length of string or list | `f:length(model.items) -> count` |

> **Cross-engine note (portable flows):** string length and substring indexes count
> **Unicode scalar values** in this Rust engine and future ports (the
> [contract rule](registration-metadata-contract.md#capabilities)) and UTF-16 code units in
> the Java engine (`String.length()` — a JVM legacy). Identical for all
> Basic-Multilingual-Plane text — English, Chinese, JSON keys, typical enterprise
> payloads; only supplementary-plane characters (e.g. emoji) differ: 1 scalar value here,
> 2 code units in Java.

### Collection operations

| Function | Description | Example |
|----------|-------------|---------|
| `f:listOfMap(a)` | Normalize JSONPath wildcard result (`Map<String,List>`) to `List<Map>` | `f:listOfMap(model.rows) -> rows` |
| `f:updateListOfMap(list, extra)` | Merge additional fields into each map in the list | `f:updateListOfMap(model.rows, model.extra) -> augmented` |
| `f:removeKey(map, key)` | Remove a key from a map or each map in a list | `f:removeKey(model.obj, text(secret)) -> clean` |

### Key normalization

| Function | Description | Example |
|----------|-------------|---------|
| `f:camelCase(mapOrList)` | Normalize every key of a map (recursively, incl. maps inside lists) to camelCase | `f:camelCase(input.body) -> normalized` |
| `f:snakeCase(mapOrList)` | Normalize every key of a map (recursively, incl. maps inside lists) to snake_case | `f:snakeCase(input.body.list) -> normalized` |

Legacy systems — often XML-to-JSON transformations — deliver key formats that vary per
source: `MyExampleKey`, `My_Example_key` and `my_example_Key` are the same logical key.
Both plugins segmentize each key and re-case the segments: underscore, hyphen and dot are
separators; a lower-case-or-digit to upper-case transition starts a new segment; and an
upper-case run followed by a lower-case letter splits before its last upper-case letter
(the acronym rule — `myXMLKey` becomes `myXmlKey` / `my_xml_key`). Digits ride with their
segment (`address1Line`). Values are never touched — only keys, at every nesting depth.
Two distinct source keys can normalize to the same target (`MyKey` and `my_key` both
become `myKey`); the later entry in map order wins. Normalization is idempotent, and a
key with no letter or digit segments at all (e.g. `"___"`) is kept as-is. Both engines
ship the identical algorithm and error messages (portable-flow contract).

Beyond legacy cleanup, the plugins also do **impedance matching** between systems with
different key conventions: an incoming camelCase request payload can be transformed to
snake_case for processing and forwarding to a downstream system that expects it — one
mapping statement at either boundary, no per-field mapping.

### Generators

| Function | Description | Example |
|----------|-------------|---------|
| `f:uuid()` | Generate a random UUID string | `f:uuid() -> id` |
| `f:now()` | Current UTC time — ISO-8601 by default, `text(ms)` for epoch milliseconds, `text(local)` for local date-time | `f:now(text(ms)) -> ts` |
| `f:dateTime(format?)` | Current date-time with local timezone, optionally formatted | `f:dateTime(text(yyyy-MM-dd)) -> today` |

### Date arithmetic

A date is an ISO-8601 calendar date written exactly as `YYYY-MM-DD`, with no time and no time zone. A date of another
shape, a day the calendar lacks, a fractional number of days, months or years, a null argument and a result outside the
years 0000 to 9999 are errors naming the plugin. Calendar months and years keep the day of the month, and count a month
when that day is reached; the three rules are in the [syntax page](event-script/syntax.md#simple-plugins) (*Date
arithmetic plugins*).

| Function | Description | Example |
|----------|-------------|---------|
| `f:addDays(date, days)` | The date a whole number of days later (earlier for a negative number), as `YYYY-MM-DD`: `(2027-01-15, 15)` gives `2027-01-30`; the days may be numeric text | `f:addDays(input.body.invoice_date, int(30)) -> due_date` |
| `f:daysBetween(from, to)` | The whole days between two dates as an integer, the earlier date excluded and the later one included: `(2027-04-10, 2027-05-11)` gives `31`, the same date twice `0`; the order of the arguments does not matter, so it is never negative | `f:daysBetween(model.due_date, input.body.paid_on) -> days_apart` |
| `f:addMonths(date, months)` | The date a whole number of calendar months later (earlier for a negative number), as `YYYY-MM-DD`, with the same day of the month; a day the resulting month lacks becomes its last day: `(2027-03-15, 12)` gives `2028-03-15`, `(2027-01-31, 1)` gives `2027-02-28` | `f:addMonths(input.body.start_date, int(12)) -> renewal_date` |
| `f:monthsBetween(from, to)` | The whole months completed between two dates as an integer, a month being complete when the day of the month is reached: `(2026-06-01, 2027-05-31)` gives `11`, `(2027-01-31, 2027-02-28)` gives `0`; the order of the arguments does not matter, so it is never negative | `f:monthsBetween(input.body.start_date, input.body.as_of) -> months_used` |
| `f:addYears(date, years)` | The date a whole number of calendar years later (earlier for a negative number), as `YYYY-MM-DD`, with the same month and day; 29 February becomes 28 February in a year that lacks it: `(2028-02-29, 1)` gives `2029-02-28` | `f:addYears(input.body.hired_on, int(5)) -> vesting_date` |
| `f:yearsBetween(from, to)` | The whole years completed between two dates as an integer, a year being complete when the month and day are reached: `(1990-07-04, 2027-07-03)` gives `36`; the order of the arguments does not matter, so it is never negative | `f:yearsBetween(input.body.born_on, input.body.as_of) -> age` |

### Configuration

| Function | Description | Example |
|----------|-------------|---------|
| `f:setConfig(key, value)` | Set or override a configuration parameter (stored as a system property; key = non-empty string, value = any object converted to text) | `f:setConfig(text(my.parameter), model.secret) -> updated` |

> `f:setConfig` changes run-time state for the whole application instance, not just the calling
> flow. The typical use case is hydrating secrets from a secret manager at start-up; components
> that already consumed a parameter will not see a later override.

> A plugin is a plain synchronous function declared with `#[simple_plugin]` and collected at link
> time; the loader checks at application startup that every built-in arrived. The Java engine
> enforces the design goal — every simple plugin executes in sub-millisecond time with no side
> effects — with a package allowlist that scans each plugin class. In Rust the signature is
> synchronous by design, so the rule is a discipline: keep the body pure, with no I/O, no network
> call and no blocking. See [Simple Plugins](event-script/syntax.md#simple-plugins).

---

## External state machine

When `external.state.machine` is declared at the flow level, tasks can read from and write to
a separate service that manages cross-flow or cross-instance state.

The `ext:` namespace is used in output mappings to write to the external state machine.
To read from it, invoke the external state machine service directly via a `sequential` task.

```yaml
flow:
  id: 'session-flow'
  description: 'Flow with external session store'
  ttl: 30s
  external.state.machine: 'v1.session.store'

# ...

  # Write to external state machine
  - input:
      - 'input.path_parameter.user -> ext:/session/user'
      - 'input.body -> ext:/session/payload'
      - 'model.none -> ext:/session/old_key'        # delete key
    process: 'no.op'
    description: 'Persist session data'
    output: []
    execution: sequential
    next:
      - 'next.step'
```

| Syntax | Action |
|--------|--------|
| `value -> ext:<key>` | Write value to external state at `<key>` |
| `model.none -> ext:<key>` | Delete `<key>` from external state |

> `external.state.machine` can be a service route (`v1.session.store`) or a subflow
> (`flow://session-flow`).

---

## Built-in special variables

These `model.*` variables are set by the framework automatically. The flow-instance metadata
(`model.flow`, `model.instance`, `model.trace`, `model.cid`, `model.ttl`, `model.run`) and the
`model.none` null constant are READ only — use them as mapping sources; the flow compiler rejects
any data mapping that overwrites these reserved keys (and the engine rejects a dynamic
`model.{model.pointer}` target that resolves to one at runtime).

| Variable | Description |
|----------|-------------|
| `model.trace` | Current distributed trace ID |
| `model.flow` | ID of the flow configuration |
| `model.instance` | Unique ID of this flow instance |
| `model.cid` | Business correlation-id of the inbound request |
| `model.ttl` | Flow instance TTL in milliseconds |
| `model.run` | Run condition set by the knowledge graph's `graph.resume` skill: `resume` when a suspended workflow record was restored, `fresh` when none existed — absent outside suspend/resume graphs |
| `model.none` | Always `null` (enforced); use to clear model keys or delete file/ext destinations |
| `model.<source>.ITEM` | Current item in a dynamic fork iteration |
| `model.<source>.INDEX` | Zero-based index in a dynamic fork iteration |

---

## Built-in service routes

| Route | Purpose |
|-------|---------|
| `no.op` | No-operation function; passes input through unchanged. Useful as a routing-only task. |
| `resilience.handler` | Built-in retry-with-backoff handler. Use as `exception:` on a task. |
| `simple.exception.handler` | Default exception handler; returns error details as HTTP response. |
| `async.http.request` | Built-in HTTP client. Set `url`, `method`, `body`, `headers.*`, `parameters.query.*` as input. See [the async HTTP client](actuators-and-http-client.md#the-async-http-client-asynchttprequest). |
| `http.flow.adapter` | The HTTP-to-flow bridge used by the [`rest.yaml` flow binding](rest-automation.md#flow) — not called from tasks. |

---

## Complete annotated example

A realistic flow that creates a user profile, responds immediately, then asynchronously
encrypts sensitive fields.

```yaml
# Flow-level metadata
flow:
  id: 'create-profile'
  description: 'Create a new user profile and encrypt sensitive fields'
  ttl: 30s                               # Total time budget for the flow
  exception: 'v1.exception.handler'      # Catches any unhandled task exception

first.task: 'v1.create.profile'          # Entry point

tasks:

  # Step 1: Store the profile
  - input:
      - 'input.body -> *'                # Map the entire request body as function input
    process: 'v1.create.profile'
    description: 'Persist the new profile to the data store'
    execution: response                  # Send HTTP response immediately, then continue
    output:
      - 'result -> model.profile'        # Store result in flow state machine
      - 'result -> output.body'
      - 'int(201) -> output.status'
      - 'text(application/json) -> output.header.content-type'
    next:
      - 'v1.encrypt.fields'              # Continue asynchronously

  # Step 2: Encrypt sensitive fields (runs after response is sent)
  - input:
      - 'model.profile -> *'             # Pass stored profile as function input
    process: 'v1.encrypt.fields'
    description: 'Encrypt address and telephone in the persisted record'
    output:
      - 'result -> model.encrypted'
    execution: sequential
    next:
      - 'v1.save.encrypted'

  # Step 3: Save the encrypted profile
  - input:
      - 'model.encrypted -> *'
    process: 'v1.save.encrypted'
    description: 'Persist the encrypted profile back to the data store'
    output: []
    execution: end                       # Terminate the flow; no response (already sent)

  # Exception handler task (referenced by flow.exception)
  - input:
      - 'error.code -> status'         # HTTP status from the thrown exception
      - 'error.message -> message'       # Error message
      - 'error.task -> failed_task'      # Which task failed
    process: 'v1.exception.handler'
    description: 'Return a structured error response'
    output:
      - 'result.status -> output.status'
      - 'result -> output.body'
    execution: end
```

---

## See also

- [Event Script Syntax](event-script/syntax.md) — tutorial-style walkthrough with worked examples
- [Flow grammar](event-script/flow-grammar.md) — the same schema as compact rules, with the
  data-mapping mini-language
- [REST Automation](rest-automation.md) — `rest.yaml` configuration for HTTP endpoint routing
- [Configuration Reference](configuration-reference.md) — `application.properties` keys
  including `yaml.flow.automation`

---

*Adapted from the mercury-composable guide `docs/guides/flow-schema-reference.md`; behavior
verified against this repository's source.*
