---
title: Test Report — The LLM helper, certified end to end through both engines
summary: Permanent record of the live certification of the LLM helper app (llm.chat, llm.stream,
  llm.health on the Anthropic SDK) in the Python and Node.js function hosts, driven by the Java and
  Rust engines through all three layers with real Claude calls - what was tested, the evidence that
  progressive rendering is never buffered, the error contract on the real SDKs, and what the
  drive found.
layer: reference
audience: [developer, architect]
keywords: [llm, claude, anthropic, llm helper, ai node, streaming, sse, event over http, certification, test report]
---

# Test Report — The LLM helper, certified end to end through both engines

*Live validation of the LLM helper app — `llm.chat`, `llm.stream` and `llm.health`, on the official Anthropic SDK —
hosted by the Python ([mercury-python](https://github.com/Accenture/mercury-python)) and Node.js
([mercury-nodejs](https://github.com/Accenture/mercury-nodejs)) packs and driven by the Java
([mercury-composable](https://github.com/Accenture/mercury-composable)) and Rust
([mercury](https://github.com/Accenture/mercury)) engines through Event-over-HTTP, conducted 2026-10-01 (UTC) with
real Claude calls. It closes the item the OpenTelemetry certification left open in
[Scenario 9](otel-dynatrace-certification.md#scenario-9-one-connected-tree-per-request-the-round-trip-span-the-parented-client-leg-head-and-tail-stream-tracing-2026-09-22):
"a real-provider token stream of the same shape follows whenever quota returns". The earlier drives ran on Gemini, whose
quota and availability decided which calls succeeded, and their token-bearing runs used a mock. This one ran on a paid
Claude key with nothing throttled.*

## Result in brief

- **Four pairs, three layers, no failed check.** Java and Rust, each in front of the Python and the Node.js helper, were driven
  through a direct service wired to a streaming REST entry (Layer 1), an Event Script flow (Layer 2), and the shipped
  `support-triage` graph plus a generic chat graph (Layer 3). 40 results per pair, 124 model calls, 22,596 input and 7,248
  output tokens: **$0.22** at list prices.
- **Progressive rendering is never buffered.** Across 8 streams (two models, four pairs), every batch the helper forwarded
  reached the engine's HTTP edge as its own frame — 50 to 101 per stream, none merged, none dropped, the text identical —
  within a few milliseconds and without drift. [Scenario 4](#scenario-4-layer-1-progressive-rendering-never-buffered).
- **The cadence a viewer sees is the API's, and it depends on the model.** Measured against the API with no SDK in the path,
  Haiku 4.5 streams continuously (a batch every 25 ms); Opus 5.5 delivers bursts about every 600 ms. The helper and the
  engines add nothing to either.
- **The error contract holds on the real SDKs**, with the same status and the same message from both helpers, from both
  engines, on every layer: no credential, an invalid key, an unknown model, a model that rejects a parameter, a deadline, a
  malformed request. [Scenario 6](#scenario-6-failure-paths-on-the-real-sdks).
- **One connected trace tree per request.** All 32 traces that touched a helper rebuild as one tree from the six processes'
  own logs; none is broken. [Scenario 7](#scenario-7-one-connected-tree-per-request).
- **The engines held no credential.** The credential reached the two helpers only; each engine process was started without
  one (verified by counting the variable in each process's environment: 1 for each helper, 0 for each engine).

## What was under test

| Component | Build under test |
|---|---|
| Java engine | MiniGraph Playground 4.12.20, built 2026-10-01 from `main` at `a52ac747` plus this branch's wording updates; OpenJDK 27 (the project builds for 21) |
| Rust engine | `minigraph-playground` 4.12.20, debug build of 2026-10-01 from `main` at `7201c95d` plus the same updates; rustc 1.98.1 |
| Python helper | mercury-python `examples/llm-helper` on package 4.12.15 (`main` at `e084fd3` plus the helper change); Python 3.14.5, `anthropic` 1.11.0 |
| Node.js helper | mercury-nodejs `examples/llm-helper` on package 4.12.15 (`main` at `da2bb12` plus the helper change); Node v22.12.0, `@anthropic-ai/sdk` 0.131.0 |
| Model | `claude-opus-5-5` (the helpers' default); `claude-haiku-4-5` where a request names it |

Ports: the helpers on 8086 (Python) and 8087 (Node.js); Java on 8085 (peer Python) and 8095 (peer Node.js); Rust on 8090
(peer Python) and 8091 (peer Node.js). The peer is chosen with `-Dllm.peer.port`; the engines read the map once per process,
so each pair is its own process. Every application ran with `-Dlog.format=compact`, and the helpers with
`-Dllm.log.batches=true`.

**The three layers, as reached.** The playground's REST automation serves a streaming entry with a direct service — the
relay `llm.stream.relay` on `POST /api/llm/stream`, a Layer 1 function on a `stream: true` entry that forwards its reply
lane into the Event-over-HTTP mapped `llm.stream`. The same hop is taken by a flow task and by a graph task that simply name
the mapped route. `support-triage` and the engine's `POST /api/graph/{graph_id}` were already in the playground; the generic
chat graph (`llm-chat-probe`, Layer 3) and the chat flow (`llm-chat`, Layer 2) were deployed beside them, with no rebuild,
through the extra-manifest lane (`graph.model.automation` and `yaml.flow.automation` take a list; the later manifest wins).
They are in [Appendix A](#appendix-a-the-deploy-folder). The Rust engine reads one REST location, not Java's merged list, so
its REST entry for the flow went into a single combined `rest.yaml`.

**The method.** A small harness drove the stack: it launched the processes, posted every scenario and checked each outcome
(a failed check was listed and set the exit code), read each Server-Sent Events response line by line and stamped every
frame on arrival, and two analysers rebuilt the traces and attributed edge frames to helper batches from the processes'
own logs. It is a scratch tool of the drive and ships in no repository; every request in it is reproducible with the `curl`
commands in [Reproduce](#reproduce).

## Scenario 1 — Layer 3: the shipped `support-triage` graph, real verdicts

The E0 graph, byte-identical in the two engines, ran against Claude for the first time on this key: its `classify` node is a
`graph.task` calling `llm.chat` with a JSON schema built in mapping lines, and the graph — not the model — routes on the
verdict. Each of the three requests below was sent to every pair; every verdict was the expected one and every route followed it.

| Request | Verdict | The graph's route | Java → Python | Java → Node.js | Rust → Python | Rust → Node.js |
|---|---|---|---|---|---|---|
| "The app crashes with an error dialog every time I upload a file larger than 5 MB." | `bug` | `bug-filed` | 3896 ms | 3191 ms | 1637 ms | 1490 ms |
| "How do I export my monthly report as a PDF file?" | `question` | `answered` | 4808 ms | 2345 ms | 3223 ms | 2346 ms |
| "Please add a dark mode to the dashboard, my eyes hurt at night." | `feature` | `feature-routed` | 1533 ms | 1498 ms | 1416 ms | 1282 ms |
| no text (rejected before any model call) | — | `rejected`, HTTP 400 | 8 ms | 8 ms | 3 ms | 3 ms |

Usage surfaced through `model.*` into the response (for the bug request: 320 input and 51 output tokens), and the model
reported itself as `claude-opus-5-5`. The rejected request spent no tokens: the graph's own check ran first.

## Scenario 2 — Layer 3: every outcome of the contract, live

The generic chat graph passes the request body to `llm.chat` and returns the helper's reply, or — through one shared failure
handler anchored from an island — the provider's status and message. Twelve probes, each run on every pair; the outcome was the
same on all four, and the error bodies were identical to the character (the request id aside).

| Probe | Outcome |
|---|---|
| plain prompt | 200 — `text: OK`, `stop_reason: end_turn`, 20 input and 4 output tokens, a `request_id` |
| conversation with a system prompt | 200 — the reply uses the earlier turn |
| structured output (a schema) | 200 — `data` is the parsed object, `label: bug` |
| `params.effort: low` | 200 |
| `params.stop_sequences: ["5"]` on a counting prompt | 200 — `stop_reason: stop_sequence`, text `1 2 3 4 ` |
| `max_tokens: 1` | 200 — text `O`, `stop_reason: max_tokens`: partial text is returned with its reason |
| structured output with `max_tokens: 1` | **422** — `LLM reply is not valid JSON for the requested schema - stop_reason=max_tokens (the reply was cut off - raise params.max_tokens)` |
| `params.model: claude-haiku-4-5` | 200 — `model: claude-haiku-4-5-20251001`: the override works with no thinking, effort or fallbacks parameter sent |
| `effort` on Haiku 4.5 | **400** — `LLM provider error - 400 invalid_request_error: This model does not support the effort parameter. (request_id …)` |
| an unknown model | **404** — `LLM provider error - 404 not_found_error: model: claude-no-such-model-9 (request_id …)` |
| `params.timeout_ms: 50` | **408** — `LLM request timed out after 50 ms` (about 60 ms at the caller) |
| `params.temperature` | **400** — `unsupported params: temperature - supported: provider, model, max_tokens, timeout_ms, effort, stop_sequences` |

## Scenario 3 — Layer 2: an Event Script flow

`POST /api/llm/chat` runs the flow `llm-chat`, whose one task is `process: 'llm.chat'` with `input.body -> *`: the route is
not local and resolves through `event-over-http.yaml`, exactly like the declarative demo in the
[Event over HTTP guide](../guides/event-over-http.md). On all four pairs a plain prompt answered `OK` (2172 to 4478 ms),
a structured request returned `data.label: feature` (2553 to 3965 ms), and an unknown model returned the provider's 404 as
the HTTP status with its message, the error body the flow adapter renders.

## Scenario 4 — Layer 1: progressive rendering, never buffered

The requirement is that a token batch reaches the caller as it is produced, never gathered and sent as one message. Four
things were checked, because each hop could break it.

**The helper's own code path.** A unit test in each pack makes the fake model refuse to produce batch *k* until the caller
already holds batches 0 to *k*-1. A mutation that gathers the batches and sends them at the end fails that test, and the
vector for tokens delivered before a mid-stream error, in both packs; the correct code passes all of them.

**Live: every batch the helper forwarded reached the edge as its own frame.** Each stream carried its own trace id; the
helper logged each batch's number, size and arrival time for that trace (`llm.log.batches`), and the harness stamped each frame
at the edge. The offset is the edge's arrival time minus the helper's forward time for the same batch.

| Pair | Model | Helper batches | Edge frames | Text identical | Offset, median | Offset, spread | Longest gap, helper / edge |
|---|---|---|---|---|---|---|---|
| Java → Python | Opus 5.5 | 67 | 67 | yes | 12 ms | 10 ms | 1201 / 1206 ms |
| Java → Python | Haiku 4.5 | 101 | 101 | yes | 7 ms | 4 ms | 191 / 191 ms |
| Rust → Python | Opus 5.5 | 50 | 50 | yes | 4 ms | 9 ms | 922 / 922 ms |
| Rust → Python | Haiku 4.5 | 82 | 82 | yes | 3 ms | 5 ms | 228 / 227 ms |
| Java → Node.js | Opus 5.5 | 51 | 51 | yes | 11 ms | 5 ms | 924 / 925 ms |
| Java → Node.js | Haiku 4.5 | 81 | 81 | yes | 10 ms | 6 ms | 181 / 178 ms |
| Rust → Node.js | Opus 5.5 | 55 | 55 | yes | 5 ms | 5 ms | 1125 / 1124 ms |
| Rust → Node.js | Haiku 4.5 | 96 | 96 | yes | 3 ms | 4 ms | 303 / 302 ms |

The counts match, the characters match, and the offset stays inside a band of 4 to 10 ms for the whole stream; a hop that
held batches back would show fewer frames than batches, or an offset that grows. The engine's own count agrees as a third
witness: the terminal record's `frames` annotation, counted by the reply lane, is 67 on the Java → Python Opus stream, the same
67 (the terminal records are in [Scenario 7](#scenario-7-one-connected-tree-per-request)). The long gaps are the same at both ends: they are not the
edge's.

**Where the cadence comes from.** The first Opus stream arrived in bursts: groups of several frames with 0 ms between them,
then a pause of about 600 ms. The same prompt was measured with the SDK straight to the API (no helper, no engine) and then on
a raw TLS socket with no SDK and no HTTP library, with no proxy variable in the environment. The raw socket shows the API itself
delivering the batches that way, so it is neither the SDK, nor the helper, nor an engine. The pace depends on the model:

| Model, measured on the raw API (2 runs each) | Tokens per batch | Distinct arrival times, of the batches | Typical pacing |
|---|---|---|---|
| Haiku 4.5 | 2.2 to 2.4 | 73 and 76, of 85 and 88 | median gap 25 ms, 0 or 1 gap over 150 ms |
| Sonnet 5.5, effort low | 3.8 to 4.1 | 26 and 33, of 65 and 67 | bursts about every 350 ms |
| Opus 5.5, effort low | 5.3 to 5.4 | 7 and 8, of 45 and 49 | bursts of about a dozen batches, every 510 to 1490 ms |
| Opus 5.5, model default effort | 5.0 to 5.6 | 7 and 8, of 46 and 53 | the same |

So the certification drives both ends. **Haiku 4.5** is the continuous case, and its check is strict: at least 30 frames,
distinct arrival times for at least 60% of them, a median gap of 60 ms or less and no gap over 600 ms. On all four pairs the
median gap was 24 to 25 ms and the longest 178 to 302 ms, with 69 to 88 distinct arrival times for 81 to 101 frames.
**Opus 5.5**, the helpers' default, is the case that inherits the API's bursts: 7 to 19 distinct arrival times for 50 to 67
frames, spread over 3.4 to 4.4 seconds (Haiku's streams spanned 2.0 to 2.5). If smooth rendering matters more than the model,
set `llm.model` (or `params.model`).

**The terminal event** carried everything it should on every stream: `model`, `stop_reason: end_turn`, `usage`, a
`request_id`, the helper's `language` (`python` or `node.js`), the caller's business correlation id (`X-Correlation-Id`)
and the trace id the caller supplied (`X-Trace-Id`), both echoed unchanged. A stream cut at one token delivered exactly one
frame and a terminal with `stop_reason: max_tokens`; a stream for an unknown model failed before any head with the provider's
404 as the HTTP status.

## Scenario 5 — Concurrency

Twelve `support-triage` requests and four streams were sent at once to each pair. Every chat answered with the expected
verdict and route, and every stream delivered frames and a terminal.

| Pair | Wall time | Median | Slowest |
|---|---|---|---|
| Java → Python | 5226 ms | 2502 ms | 5215 ms |
| Java → Node.js | 4432 ms | 2593 ms | 4425 ms |
| Rust → Python | 4404 ms | 2294 ms | 4393 ms |
| Rust → Node.js | 3413 ms | 2392 ms | 3403 ms |

The paid key was never throttled: no 429 appeared in any drive.

## Scenario 6 — Failure paths on the real SDKs

The vectors pin the error contract against a fake of each SDK; this scenario proves it against the real ones, without a valid
credential and so without spend. Four helper variants ran beside the real ones — each language with no credential, and each
with a deliberately invalid placeholder — and were called both through engines and directly from a pack's own client.

| Helper | Called through | Layer 3 graph | Layer 2 flow | Layer 1 relay |
|---|---|---|---|---|
| Python, **no credential** | Java | 503 | 503 | 503 |
| Python, no credential | Rust | 503 | — | — |
| Node.js, **invalid key** | Rust | 401 | 401 | 401 |
| Node.js, invalid key | Java | 401 | — | — |

Called directly with the Python pack's client (no engine), all four variants answered `llm.chat` with the same text as above:
**503** `LLM provider credential missing - set ANTHROPIC_API_KEY in the environment` for each language with no credential,
**401** `LLM provider error - 401 authentication_error: invalid x-api-key (request_id …)` for each with an invalid key, and
**400** `unsupported params: temperature - …` for a malformed request. The 401 carries Anthropic's own request id. A malformed
request answered 400 through every layer too: an unsupported param through the graph, a missing prompt through the flow, a
`schema` on the streaming route through the relay, a bad `effort` through the Rust graph, and an unknown provider through the
Rust relay.

`/health` tells the credential states apart without a call: a helper with a credential reports `UP` and names its backend and
model; one without reports `400` (its `llm.health` dependency answered 503). A helper with an invalid key reads as healthy,
because telling it apart takes a network call and a health probe must never spend a token.

## Scenario 7 — One connected tree per request

The traces were rebuilt from the six processes' own compact logs by the Scenario 9 method: group every distributed-trace
record by trace id, take the records whose parent is absent as roots, and require exactly one root — the edge's `http.request`
round-trip span — and no record whose parent was never exported. The logs held 156 traces; **32 touched a helper, and all 32
form one connected tree**.

```text
Java → Python   eaf5e393…  (the caller supplied X-Trace-Id and no traceparent)
http.request [java-py]  5600.5 ms, status 200                                   ← the root, the edge's round trip
└── llm.stream.relay [java-py]  0.2 ms
    ├── async.http.request [java-py]  3.0 ms
    └── llm.stream [py]  5590.6 ms  (model=claude-opus-5-5, stop_reason=end_turn, output_tokens=302)
        ├── async.http.response.stream.0 [java-py]  the head
        └── async.http.response.stream.0 [java-py]  the terminal  (frames=67)

Rust → Node.js  a979c78b…
http.request [rust-node]  4804.1 ms, status 200
└── llm.stream.relay [rust-node]  0.1 ms
    ├── async.http.request [rust-node]  1222.3 ms
    └── llm.stream [node]  4802.1 ms  (model=claude-opus-5-5, stop_reason=end_turn, output_tokens=261)
        ├── async.http.response.stream.0 [rust-node]  the head
        └── async.http.response.stream.0 [rust-node]  the terminal  (frames=55)
```

The helper's usage rides its span as annotations (`llm_model`, `llm_stop_reason`, `llm_input_tokens`, `llm_output_tokens`,
`llm_request_id`), so a backend shows what a call cost. Failed requests are trees too: a stream the helper failed before its
first token (no credential, an unknown model) is a root `http.request` carrying the provider's status and message over the
relay, the client leg, the helper's span and a terminal with `frames=0`. A helper's application log lines carry the caller's
business correlation id and the trace id in their `context` block, and the completed-call line holds the model, the
`stop_reason`, the token counts, the request id and the elapsed time — never a prompt or a completion.

## Findings

1. **The cadence of progressive rendering belongs to the API and the model, not to the pipeline.** Opus 5.5's batches arrive in
   bursts about every 600 ms and Haiku 4.5's in a continuous trickle. Neither the helper nor either engine changes the pattern:
   the batches and the frames are in one-to-one correspondence, with a constant offset. A product that wants smooth rendering
   chooses the model for it; the helper README says so, and `llm.log.batches` is the tool for telling a bursty source from a
   hop that holds batches back.
2. **Thinking tokens count against `max_tokens`, and the helper's rule caught it live.** Opus 5.5 thinks before it answers.
   Twice, unprovoked, in the first Rust drives, a stream with a 120-token budget spent all of it on thinking and produced no
   text; both helpers answered `422 LLM reply is empty - stop_reason=max_tokens, output_tokens=120 (raise params.max_tokens
   or lower params.effort)` instead of an empty 200 stream — the same trap the Gemini drives hit with a 200-token budget, now an
   explicit error with its remedy. It is stochastic, so it is pinned deterministically by the vectors, not scripted live; the
   concurrency scenario's budget was raised to 400 tokens afterwards and the final drives ran clean. The demos now ship
   generous budgets for the same reason: the `support-triage` graph and the Layer 1 stream example ask for 2000 tokens (they
   asked for 512 and 300). A probe of 48 live Opus 5.5 calls with those prompts found no empty result at the old or the new
   budget: the two-sentence stream used 100 to 127 output tokens, thinking included, and the triage call 40 to 49, so the old
   caps held a margin of between two and ten times, and only a tight budget (the 120 above) or a heavier prompt uses it up.
   Opus 5.5 stays the default model, deliberately: it is the most capable, and a project that wants the smooth token trickle
   of Haiku 4.5 sets `llm.model: claude-haiku-4-5` on the helper.
3. **Real SDK facts the helpers had to meet.** A missing credential is not an API error: the Python SDK raises a bare
   `TypeError` and the Node SDK a plain `Error` (an `AnthropicError` on a stream), before anything is sent; both helpers map
   it to a 503 by its text. Python SDK 1.x takes no `temperature`, `top_p` or `top_k` at all, and the current models reject
   them, so the contract excludes sampling parameters. The API accepts the server-side refusal fallback (`fallbacks: "default"`
   with its beta header) on Opus 5.5.
4. **The Rust REST automation is a single location.** The Java engine merges a comma-separated list of REST files; the Rust
   engine reads one. It only mattered for adding a flow's REST entry without a rebuild, solved with one combined file per engine.
5. **A helper's own span says `status 200` for a stream that failed in-band.** The function returned normally; the failure is
   the terminal. The edge's `http.request` span carries the failure's status and message, so a backend should read failures
   from the edge span.
6. **Engine parity held without adjustment.** The same graph JSON, byte for byte, ran on both engines; the relay's contract
   and error shapes were identical; the only visible difference is cosmetic, the order in which each engine serializes a
   response map's keys.

## Defects found and fixed

The round surfaced one product defect, found while building the second helper from the first: in the Python helper a body
`{"messages": "hello"}` (not a list) passed the first check and then raised a `KeyError` on the missing `prompt`, a 500; only a
non-empty list now counts as turns, in both helpers, with two vectors for it. The harness had two faults of its own, fixed
before the final drives: its SSE reader stripped every leading space after `data:` instead of one, as the specification says,
and its concurrency scenario used a budget small enough to meet Finding 2.

## Reproduce

Start a helper with the credential exported in its environment, and each engine without one:

```bash
# Python helper (8086) and Node.js helper (8087), from their repositories
mercury-serve examples/llm-helper/llm_helper.py -Dlog.format=compact -Dllm.log.batches=true
node dist/src/cli.js examples/llm-helper/llm-helper.mjs -Dlog.format=compact -Dllm.log.batches=true

# the engines, with the deploy folder of Appendix A (replace $DEPLOY; Rust takes one REST location)
java -Drest.server.port=8085 -Dllm.peer.port=8086 -Dlog.format=compact \
  -Dgraph.model.automation="classpath:/graphs.yaml, file:$DEPLOY/graphs.yaml" \
  -Dyaml.flow.automation="classpath:/flows.yaml, file:$DEPLOY/flows.yaml" \
  -Dyaml.rest.automation="file:$DEPLOY/rest-java.yaml" \
  -jar examples/minigraph-playground/target/minigraph-playground-4.12.20.jar
cargo run -p minigraph-playground -- -Drest.server.port=8090 -Dllm.peer.port=8086 -Dlog.format=compact \
  -Dgraph.model.automation="classpath:/graphs.yaml, file:$DEPLOY/graphs.yaml" \
  -Dyaml.flow.automation="classpath:/flows.yaml, file:$DEPLOY/flows.yaml" \
  -Dyaml.rest.automation="file:$DEPLOY/rest-rust.yaml"
```

Then, against either engine's port:

```bash
# Layer 3 - the shipped graph, then the generic chat graph
curl -s -X POST -H 'content-type: application/json' -d '{"text":"The app crashes when I upload a file."}' \
  http://127.0.0.1:8085/api/graph/support-triage
curl -s -X POST -H 'content-type: application/json' \
  -d '{"prompt":"Reply with exactly one word: OK","params":{"model":"claude-haiku-4-5"}}' \
  http://127.0.0.1:8085/api/graph/llm-chat-probe

# Layer 2 - the flow
curl -s -X POST -H 'content-type: application/json' -d '{"prompt":"Reply with exactly one word: OK"}' \
  http://127.0.0.1:8085/api/llm/chat

# Layer 1 - the relay, progressive rendering (a smooth stream: Haiku)
curl -N -X POST -H 'accept: text/event-stream' -H 'content-type: application/json' \
  -H 'X-Correlation-Id: biz-demo-1' -H 'X-Trace-Id: 0123456789abcdef0123456789abcdef' \
  -d '{"prompt":"Write about 120 words on why event-driven systems decouple producers from consumers.","params":{"model":"claude-haiku-4-5","max_tokens":400}}' \
  http://127.0.0.1:8085/api/llm/stream
```

Without a credential the same commands return the 503 of Scenario 6, which is the quickest check that the wiring is right.

## Appendix A — the deploy folder

The two assets deployed beside the playground, and the REST entry that serves the flow. No engine code changed.

**`graphs.yaml`** and **`flows.yaml`** each list one id and carry their own `location` (a `file:` folder). **`llm-chat-probe`**
is six nodes: `root` → `ask` (a `graph.task` on `llm.chat` with `exception=failed`; `input[]` maps `input.body.prompt`,
`messages`, `system`, `schema` and `params` to the same names, and a null source is simply not supplied, so one graph serves a
prompt, a conversation and a structured request; `output[]` is `result -> model.reply`) → `answer` (`model.reply ->
output.body`) → `end`; and `root` → `island` (`graph.island`) → `failed` (a mapper: `error.code -> output.status`,
`error.source`, `error.code` and `error.message` into `output.body`) → `end`.

**`flows/llm-chat.yml`**

```yaml
flow:
  id: 'llm-chat'
  description: 'Ask the LLM helper from an Event Script task (Layer 2), reached through Event-over-HTTP'
  ttl: 45s

first.task: 'llm.chat'

tasks:
  - input:
      - 'input.body -> *'
    process: 'llm.chat'
    output:
      - 'text(application/json) -> output.header.content-type'
      - 'result -> output.body'
    description: 'Single-shot completion on the LLM helper, resolved through event-over-http.yaml'
    execution: end
```

**The REST entry**, appended to each engine's own `rest.yaml` in a combined copy:

```yaml
  - service: 'http.flow.adapter'
    methods: ['POST']
    url: '/api/llm/chat'
    flow: 'llm-chat'
    timeout: 50s
    tracing: true
```
