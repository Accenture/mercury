# mercury-agent-loop (`llm.run`)

The agent loop of RFC-0011, the Rust twin of the Java engine's `extensions/agent-loop` (design spec:
`draft-design-specs/llm-helper-agent-loop.md` in the Java repository). A graph node calls `llm.run` with a tool
catalog and a prompt. The loop sends each model turn to `llm.chat`, the LLM helper of a language pack (mercury-python
or mercury-nodejs). When the model asks for a tool, the loop calls the route the catalog maps the tool's name to, and
sends every result of the turn back in one message. It repeats until the model answers, a person must approve a call,
the model refuses, or a limit stops it. The graph routes on the envelope it returns.

The crate is opt-in: linking it registers `llm.run`. It links no provider SDK, no MCP client and no store client. The
transcript stays in the language pack's transcript store, and the loop holds only a reference to it.

```toml
[dependencies]
mercury-agent-loop = "x.y.z"
```

Use the Mercury release your application builds with in place of `x.y.z`. Reference the crate from `main.rs` (for
example `use agent_loop as _;`) so the linker keeps its registration inventory.

The application also needs `llm.chat`. Deploy a language pack's LLM helper with a transcript store configured
(`llm.transcript.store`), and declare `llm.chat` in `yaml.event.over.http`. A tool route is any composable function:
a local one, or one a language pack serves (an MCP binding, for example) over Event-over-HTTP.

## A loop node

The catalog is a skill-less dataset node under the graph's `graph.island`, so it is certified with the graph and is
never a step of the run. The settings are constant input mappings on the loop's node.

```text
create node knowledge
with type Island
with properties
skill=graph.island

create node credit-tools
with type ToolCatalog
with properties
tools='''[ { "name": "credit_limit", "route": "credit.limit.lookup", "strict": true,
             "description": "A customer's credit limit and its currency.",
             "input_schema": { "type": "object", "properties": { "id": { "type": "string" } },
                               "required": ["id"], "additionalProperties": false } },
           { "name": "create_issue", "route": "issue.create", "write": true,
             "description": "Open an issue for a credit officer to review.",
             "input_schema": { "type": "object", "properties": { "title": { "type": "string" } },
                               "required": ["title"] } } ]'''

create node investigate
with type Task
with properties
skill=graph.task
task=llm.run
ttl=60
input[]=f:json(credit-tools.tools) -> tools
input[]=text(You are a credit officer's assistant.) -> system
input[]=input.body.question -> prompt
input[]=text(write) -> mode
input[]=int(6) -> max_turns
input[]=text(55s) -> timeout
```

`ttl=60` gives the node a minute, and `timeout=55s` makes the loop stop and answer before the node does.

### The catalog

| Field | Meaning |
| --- | --- |
| `name` | The name the model sees: `^[a-zA-Z0-9_-]{1,128}$`, unique. A route name has a dot, so a row maps an API-safe name to it. |
| `description` | What the model reads about the tool, written on the graph. Required. |
| `input_schema` | The schema the model fills and the loop validates against, of `type` `object` (the vocabulary below). |
| `route` | The composable-function route the loop calls. The model never sees it. Required, except on a row that asks a person. |
| `strict` | `true` asks the pack for grammar-constrained arguments. Optional. |
| `write` | `true` marks a tool that writes; the node's mode decides whether it waits for a person. Optional. |
| `ask` | `true` makes the row a question for a person, with no route (see below). Optional. |

Any other field is refused, so a misspelt `write` can never let a write run. Before the first model turn the loop
checks the whole catalog, and that every route is served: a local function or an Event-over-HTTP target (the Rust
port has no Kafka service mesh, which the Java engine also accepts). A failure answers with status 400 before a token
is spent. A name the model invents is different: it goes back to the model as that call's error, and the model picks
another tool or answers.

**The input-schema vocabulary** is a closed subset of JSON Schema: `type` (a name, or a list of names, from `object`,
`array`, `string`, `number`, `integer`, `boolean` and `null`), `properties`, `required`, `additionalProperties`,
`items`, `enum`, `const`, `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum`, `minLength`, `maxLength`,
`pattern`, `minItems` and `maxItems`. `title`, `description`, `default`, `examples` and `format` are documentary. Any
other keyword is refused, and so is a keyword that cannot apply to the declared type, because a constraint the loop
ignored would teach that an unflagged schema is safe. A `pattern` stays inside the regular-expression subset that
both engines share: no lookaround, atomic group, backreference or possessive quantifier. An invalid input goes back
to the model as that call's error, naming each violation, and the route is not called.

### Inputs

| Input | Meaning |
| --- | --- |
| `tools` | The catalog: a list of rows, or its JSON text. |
| `system` | The system prompt. Optional. |
| `prompt` or `messages` | The first turn. A resume carries neither: the transcript holds the conversation. |
| `schema` | The output schema. The outcome is then `data`, and a `decision` it declares is lifted into the envelope. |
| `model` | The model. Pinned by the pack at the first turn. Optional. |
| `params` | Other `llm.chat` parameters, such as `max_tokens` and `effort`. The loop sets `timeout_ms`. |
| `conversation`, `turn`, `approval`, `decisions` | A resume after an approval (see below). |

Any other input is refused with status 400 that lists the inputs, so a misspelt setting never falls back to its default.

### Settings

| Input | Global default | Meaning |
| --- | --- | --- |
| `mode` | `write` | The approval mode, which governs the use of tools. |
| `max_turns` | `llm.run.max.turns=10` | The conversation's turn limit, a proxy for the input tokens the run accumulates. A resume spends from the same budget. |
| `result_limit` | `llm.run.result.limit=65536` | The largest tool result, in characters of JSON text. A larger one is cut and marked as cut. |
| `transcript_ttl` | `llm.run.transcript.ttl=7d` | How long a running or waiting transcript lives. At least the longest approval wait. |
| `retention` | `llm.run.transcript.retention=0` | How long a finished run's transcript is kept for audit. `0` deletes it at the end. |
| `timeout` | `llm.run.timeout=25s` | The loop's deadline, below the node's `ttl` (the graph's `model.ttl` is 30 seconds by default). |

A duration is seconds, or a number with `s`, `m`, `h` or `d`. `worker.instances.llm.run` sets how many loops run at
once (default 50).

## The envelope

| Field | Meaning |
| --- | --- |
| `status` | `done`, `approval`, `refusal` or `cap`. |
| `text` or `data` | The outcome, when `done`: `data` when the node declares an output schema. |
| `decision` | Lifted from `data.decision` when the output schema declares it. |
| `approval` | When `status` is `approval`: `{calls: [{id, name, route, input, wait, ask}]}`; `ask: true` marks a question. |
| `cap` | When `status` is `cap`: `turns`, `deadline`, `context` or `max_tokens`. |
| `refusal` | When `status` is `refusal`: `{category}`. |
| `conversation`, `turn` | The transcript's reference and turn count, for a resume. |
| `turns` | The model turns this call took. |
| `usage` | The tokens this call spent: input, output, cache creation and cache read. |
| `calls` | `[{id, name, route, status}]` for each call answered: `ok`, `error`, `unknown`, `invalid`, `refused` or `answered`. No arguments. |
| `model` | The model that served the turns. |

A decision node routes on `status` first, then on `decision`. Only the loop's own machinery fails, so the node's
`exception=` route handles it: a bad input or catalog (400, before a token is spent), `llm.chat` unreachable (503), a
resume whose decisions do not match the waiting calls (400), `llm.chat` failing after the pack's own retries (its
status), or a reply outside the loop contract (502). A tool route that fails or times out goes back to the model as
that call's error, carrying its status and message, never a stack trace.

## Approval

The mode governs the use of tools:

| `mode` | A row marked `write: true` | Any other row |
| --- | --- | --- |
| `write` (default) | waits for a person | runs |
| `auto` | runs | runs |
| `manual` | waits for a person | waits for a person |
| `unattended` | goes back to the model as a refusal, never waits | runs |

A node runs a write without a person only when the graph says `mode=auto`, which is visible on the graph and
certified with it; a node that sets no mode holds its writes. In `auto`, a write is guarded by the fixed catalog, the
validated input, the narrow loop, the call log in the envelope and on the span, and any decision the graph routes
after the loop. A tool result can still carry instructions that steer a write, and that is the risk a node accepts
by declaring `auto`.

When a turn holds a call that must wait, the loop runs none of the turn's calls: it returns `status: approval` with
every call of the turn and its arguments, `wait: true` for each call that needs a decision. A call that could never
run (an unknown name, an invalid input) does not wait.

The loop node keeps what a resume needs in `model.*`, so the graph's suspension persists it with the suspend store,
and a decision node after it routes on the status:

```text
output[]=result.status -> model.agent.status
output[]=result.conversation -> model.agent.conversation
output[]=result.turn -> model.agent.turn
output[]=result.approval -> model.agent.approval
```

The resume runs the same node again (`RESET`) with the conversation, the turn and the approval block from `model.*`,
and one decision per waiting call from the resume request. On the first run these sources are absent, so the inputs
are not supplied:

```text
input[]=model.agent.conversation -> conversation
input[]=model.agent.turn -> turn
input[]=model.agent.approval -> approval
input[]=input.body.decisions -> decisions
```

A decision is `{id, approved, note}`. The loop runs the calls from the approval block, never from the request, so
the call that runs is the call that was shown. An approved call and every call with `wait: false` run; a refused call
goes back to the model as an error carrying the reviewer's note. A waiting call without a decision, a decision for any
other id, an id decided twice, or a call that no longer matches the catalog or the mode answers with status 400.

### A question for a person

When the model should recommend options and a person chooses, and the model must carry on with the choice in the
same conversation, the catalog holds a row that asks a person: `ask: true` and no route. The model calls it like any
tool, with an input the row's schema shapes, for example a question and its options:

```json
{ "name": "ask_officer", "ask": true,
  "description": "Ask the credit officer to choose one of the options you propose. The answer is the option's number.",
  "input_schema": { "type": "object",
                    "properties": { "question": { "type": "string" },
                                    "options": { "type": "array", "items": { "type": "string" } } },
                    "required": ["question", "options"] } }
```

The call waits for a person in every mode except `unattended`, where it goes back to the model as "nobody to ask", so
the turn is held as for any waiting call and the approval block shows it with `ask: true` and no route. Its decision
is `{id, answer}`: the person's answer, text or any JSON value, goes back to the model as the call's result, and the
call log reports it as `answered`. A question's decision carries no `approved` or `note`, and a tool call's decision
carries no `answer`. An ask row has no `write` mark.

When the options are the actions the graph already has branches for, the business-decision pattern needs no question
row: the output schema declares the options (an `enum`) and a recommended `decision`, the graph suspends, and a
decision node routes the person's choice. A question row is for a choice that must return into the same
conversation.

## Progress events

When its caller passes `x-return-path`, the loop sends its progress events there, in order and before its own
response. A caller that passes none gets none. The Java graph engine passes it to a `graph.task` node that carries
`progress=<text>` (the MiniGraph skills reference, *Progress events*); the Rust graph engine does not relay progress
events yet, so here a caller names its own return route.

| `event` | `data` |
| --- | --- |
| `run.start` | `model` (when the node names one), `tools` (a count), `max_turns` |
| `turn.start` | `turn` |
| `turn.end` | `turn`, `stop_reason`, `usage` |
| `tool.start` | `turn`, `id`, `name`, `route` |
| `tool.end` | `turn`, `id`, `name`, `route`, `status`, `ms` |
| `approval.wait` | `calls`: `[{id, name, route}]` for the calls that wait; the arguments stay in the envelope |
| `run.end` | `status` (the envelope's, or `error` when the loop fails), `turns`, `usage` |

Each event is `{type: progress, event, seq, time, data, cid}`, numbered from 1 for each call, and carries names,
routes, counts and statuses only; `cid` is the business correlation id, when the call has one. A call that never runs
(an unknown name, an invalid input, a question, a refusal) has no tool events, and a turn that times out has no
`turn.end`. The events are not traced (`PostOffice::send_untraced`), so a run adds no span per event.

## Telemetry

The `llm.run` span parents every `llm.chat` turn and every tool call. Its annotations carry the status, the turn
counts, the model and the usage, never a prompt, a completion, a tool's input or a tool's result, and neither does
its log line.

## Tests

`tests/llm_run_vectors.rs` runs the shared vector file `tests/resources/vectors/llm-run-vectors.json` against the
loop: a scripted `llm.chat` checks every request the loop sends and answers it, scripted tool routes answer the calls,
and a clock the script advances drives the deadline. No token is spent. The file carries 105 loop cases, seven of
them pinning the progress events, and 29 schema cases, byte-identical with the Java repository's copy.
`tests/llm_run_platform.rs` runs `llm.run` through the platform: registered by the preload inventory, with real
routes, a route that fails, and a route that times out.

```bash
cargo test -p mercury-agent-loop
```
