# Composable Orchestration

Layer 2 — **Event Script**, a YAML DSL that describes an end-to-end transaction as a **flow**
choreographing composable functions. Orchestration moves *out of code and into
configuration*: instead of writing Rust that calls one function after another, you describe
the sequence in a flow file. Roughly half the work becomes configuration; the half that stays
code is the functions themselves — unchanged from
[Layer 1](../event-driven/index.md), referenced only by route name.

## The mental model

- A **flow** is an ordered set of **tasks** with one entry point (`first.task`) and at least
  one terminal task. A transaction — an API call, a batch job, a real-time event — runs one
  flow instance.
- Each **task** runs a function (named by its route); **data mapping** rules move values
  between the flow and the function's input/output.
- A per-transaction **state machine** (`model`) holds intermediate results across the
  stateless functions, isolated per flow instance.
- A task's **execution type** decides what happens next: `sequential`, `decision`,
  `parallel`, `fork`, `pipeline`, `response`, `end`, or `sink` (the rules live in the
  [flow grammar](flow-grammar.md)).
- A **flow adapter** drives the flow from outside: REST automation binds an HTTP endpoint to
  a flow with one line of `rest.yaml`.

The engine compiles and validates every flow at application startup (a before-application
hook at sequence 5, the Java `CompileFlows` analog) — a flow that violates the grammar fails
to load, so orchestration errors surface before runtime, not during a transaction.

!!! note "Rust port"
    The **flow YAML syntax is identical to the Java engine's** — flow files port between the
    two implementations unchanged, and the same grammar governs both. What differs is around
    the flows: the functions they call are written in Rust, and the only flow adapter is
    HTTP via REST automation (the Java engine's Kafka flow adapter belongs to the
    out-of-scope service mesh).

## Write your first flow

The shipped [`examples/hello-flow`](https://github.com/Accenture/mercury/blob/main/examples/hello-flow)
application is a complete Layer-2 transaction: greet a user in their language. Two small
functions, one flow file, one endpoint — and the two functions never reference each other.

### 1. The flow

`resources/flows/hello-flow.yml` *is* the transaction:

```yaml
flow:
  id: 'hello-flow'
  description: 'Greet a user in their language'
  ttl: 10s

first.task: 'language.router'

tasks:
  - input:
      - 'input.query.lang -> lang'
    process: 'language.router'
    output:
      - 'result -> decision'
    description: 'Choose the greeting language'
    execution: decision
    next:
      - 'greet.french'
      - 'greet.english'

  - name: 'greet.french'
    input:
      - 'input.path_parameter.user -> user'
      - 'text(Bonjour) -> greeting'
      - 'model.cid -> cid'
    process: 'greeting.composer'
    output:
      - 'text(application/json) -> output.header.content-type'
      - 'result -> output.body'
      - 'status -> output.status'
    description: 'French greeting'
    execution: end

  - name: 'greet.english'
    input:
      - 'input.path_parameter.user -> user'
      - 'text(Hello) -> greeting'
      - 'model.cid -> cid'
    process: 'greeting.composer'
    output:
      - 'text(application/json) -> output.header.content-type'
      - 'result -> output.body'
      - 'status -> output.status'
    description: 'English greeting'
    execution: end
```

Reading it top to bottom:

- **`first.task`** enters at `language.router`, a **`decision`** task. Its input mapping
  takes the HTTP query parameter `lang` from the flow input; its output maps the function's
  boolean `result` into the reserved `decision` variable. `true` selects the first entry in
  `next`, `false` the second — branching as configuration.
- **Both greeting tasks run the same function**, `greeting.composer`, so each carries a
  unique `name` for the flow to address. Their input mappings assemble the function's input
  from three *different sources*: the HTTP path parameter (`input.path_parameter.user`), a
  constant (`text(Bonjour)`), and the per-transaction state machine (`model.cid` — the
  business correlation id, seeded at the HTTP edge).
- **`execution: end`** terminates the transaction and shapes the HTTP response: the
  function's `result` becomes `output.body`, its status becomes `output.status`, and a
  response header is set from a constant. Every flow must have at least one `end` task.

### 2. The functions

The functions are ordinary Layer-1 composable functions
([`examples/hello-flow/src/main.rs`](https://github.com/Accenture/mercury/blob/main/examples/hello-flow/src/main.rs))
— they know nothing about the flow, the HTTP endpoint, or each other:

```rust
/// Decision task: French when `lang=fr`, English otherwise.
#[preload(route = "language.router")]
struct LanguageRouter;

#[async_trait]
impl ComposableFunction for LanguageRouter {
    async fn handle_event(
        &self,
        _headers: HashMap<String, String>,
        input: EventEnvelope,
        _instance: usize,
    ) -> Result<EventEnvelope, AppError> {
        let body: serde_json::Value = input.body_as().unwrap_or(serde_json::Value::Null);
        EventEnvelope::new().set_body(body["lang"].as_str() == Some("fr"))
    }
}
```

`greeting.composer` (10 lines, same shape) formats `"{greeting}, {user}!"` from whatever the
flow mapped into its input. Either function could be reused tomorrow by another flow — or a
knowledge-graph skill — without modification.

### 3. The wiring

The flow manifest `resources/flows.yaml` registers the flow file (config key
`yaml.flow.automation`, default `classpath:/flows.yaml`):

```yaml
flows:
  - 'hello-flow.yml'

location: 'classpath:/flows/'
```

And `resources/rest.yaml` binds the endpoint to the **flow** instead of a function — the
`flow:` line is the entire integration:

```yaml
rest:
  - service: "http.flow.adapter"
    methods: ['GET']
    url: "/api/hello/{user}"
    flow: 'hello-flow'
    timeout: 10s
    tracing: true
```

REST automation routes the request to the built-in flow adapter with the flow id attached;
the adapter reshapes the HTTP request into the flow's input dataset (`input.query.*`,
`input.path_parameter.*`, `input.body`, headers), launches the flow, and the `end` task's
`output` mappings complete the HTTP response.

### 4. Run it

```bash
cargo run -p hello-flow
curl 'http://127.0.0.1:8086/api/hello/eric?lang=fr'
```

```json
{
  "message": "Bonjour, eric!",
  "cid": "…",
  "handled_by_instance": 1,
  "served_by": "hello-flow"
}
```

Change `lang=fr` to `lang=en` (or drop it) and the decision task takes the English branch.
To change the transaction — add a step, a retry, a different response shape — you edit the
YAML, not the functions.

## Going deeper

- **[Event Script Syntax](syntax.md)** — the complete DSL reference: flow files, every
  execution type, the data-mapping mini-language, type matching, and worked examples.
- **[Flow grammar](flow-grammar.md)** — the rule-based schema the compiler enforces (with a
  machine-readable form, [`event-script-flow.json`](event-script-flow.json)).
- **[AI agent guide](ai-agent-guide.md)** — the agent-facing companion of this section:
  the compile contract, a pre-write checklist, and a deterministic recipe, engine-verified so
  an AI agent can author valid flows from the documentation alone. If an agent writes your
  flows, point it here first.
- **[Event-Driven Foundation](../event-driven/index.md)** — Layer 1: writing the functions
  that tasks call.

---

*Adapted from the mercury-composable guide `docs/guides/event-script/index.md`; behavior verified against this repository's source.*
