---
title: Test Report — The agent demo graphs driven live on the Rust engine
summary: Permanent record of the first live drive of the Playground's five agent graphs on this engine - order-assistant,
  diagram-assistant, site-notes, document-ingest and document-query, the Java example's models compiled unchanged - through
  the Python language pack on Claude Haiku 4.5, with Redis, Neo4j and a public Mermaid MCP server - what was tested, what
  it found, and how it compares with the Java engine's drives.
layer: reference
audience: [developer, architect]
keywords: [agent loop, llm.run, order-assistant, diagram-assistant, site-notes, document-ingest, document-query, agent tools,
  MCP binding, knowledge graph, suspend, resume, Rust engine, test report]
---

# Test Report — The agent demo graphs driven live on the Rust engine

*The Rust Playground example now ships the five agent graphs of the Java Playground - the same JSON models, compiled
unchanged by this engine (Increment 205) - with the example's own tool functions ported (`src/order_book.rs`,
`src/diagram.rs`) and the agent loop crate linked. This drive ran every graph end to end on 2026-11-18 (UTC+8) through
the Python pack of mercury-python (`main`, after its #67): the LLM helper on Claude Haiku 4.5, the built-in agent tools
and the MCP binding, with Redis 7 and Neo4j 5 Community on Colima and the public Mermaid Chart MCP server. The Java
engine's drives of the same graphs are the reference: `docs/test-reports/agent-loop-live-drive.md`,
`agent-tool-use-live-drive.md` and `agent-tools-live-drive.md` in mercury-composable.*

## Result in brief

- **Every graph answered on the Rust engine, with no change to any model.** The order assistant held its refund for a
  supervisor, re-advised on a resume without decisions and on a bad decision, ran the approved refund, read the order
  book's 409 on a second attempt and offered a replacement after a refusal; the diagram assistant validated a flowchart
  through the pack's MCP binding and stored the image out of band; the four documents of the corpus went into the
  knowledge graph with every node carrying its source and nothing doubled on a second ingest; two questions were
  answered from the graph with sources; the research graph crawled this project's published documentation, waited for
  a person before writing the notes, and wrote them after approval.
- **The protocol is the Java engine's.** The same correlation id carried a transaction across its suspension; a
  resume without decisions answered in under a millisecond of engine time; a decision for a call that was not waiting
  was refused by name before any token was spent and the transaction kept; a watched run streamed the node's text and
  the loop's events (`run.start`, `turn.start`, `tool.start`, `tool.end`, `turn.end`, `run.end`) before the result.
- **The same finding as the Java drive, in the same place.** On the travel policy the loop's schema check refused one
  `graph_ingest` call as invalid and the model corrected the shape on its next turn; the call log reads `invalid` then
  `ok`, and the graph was loaded.
- **The graph's counts equal the Java drive's, label for label:** Document 4, Party 4, Obligation 15, Rule 12,
  Deadline 8; PARTY_TO 8, STATED_IN 35; no node without a `source`. The document node did not double on the re-ingest,
  the key rule the Java drive added to the prompt holding.
- The Python helper served 43 model turns; no token, prompt or page text reached a log of any process; the engine
  logged no error.

## What was under test

| Piece | Version | Role |
| --- | --- | --- |
| Rust engine, `examples/minigraph-playground` | this branch (Increment 205) on 4.12.30, release build | the five agent graphs, the order book's three tools, `diagram.render`, the REST entries `/api/graph/{graph}` and `/api/stream/graph/{graph}`, the Redis state store |
| `extensions/agent-loop` | 4.12.30 | `llm.run`: the loop, the catalog check, the modes, the approval and resume, the progress events |
| mercury-python `extensions/llm-helper` | `main` after #67, port 8086 | `llm.chat` with the loop contract, `-Dllm.model=claude-haiku-4-5`, the credential in its environment only |
| mercury-python `extensions/agent-tools` | `main` after #67, port 8089 | `file.extract`, `graph.ingest`, `graph.schema`, `graph.query`, `web.crawl`, `file.write`, with `tools.file.root` a copy of the example's `corpus/`, `tools.web.allow=accenture.github.io`, `tools.graph.uri=bolt://127.0.0.1:7687`, `tools.graph.write=true` |
| mercury-python `extensions/mcp-binding` | `main` after #67, port 8088 | `mermaid.render`, bound to the public Mermaid Chart MCP server's tool with the pinned digest |
| Redis 7, Neo4j 5 Community (containers) | `redis:7-alpine`, `neo4j:5-community` | the graphs' suspend store; the agent's knowledge graph |
| Model | `claude-haiku-4-5-20251001` | the helper's `llm.model`; the graphs name none |

The engine ran from `examples/minigraph-playground` as `../../target/release/minigraph-playground -Dllm.peer.port=8086
-Dmcp.peer.port=8088 -Dtools.peer.port=8089`; its startup log showed the seven graphs compiled, `llm.run with 50
instances started` and fifteen Event-over-HTTP targets. Requests were `POST /api/graph/{graph}` with one correlation id
per transaction; the graph was counted through the database.

## The scenarios

| Scenario | Graph | Outcome |
| --- | --- | --- |
| A1 - a delivered order | order-assistant | `type: answer` in one request (12.0 s, the session's first model call): "delivered … to your front desk on November 8, 2026 … signed for"; calls `order_lookup` ok, `shipment_status` ok; 2 turns |
| A2 - a lost shipment, the approval protocol | order-assistant | run 1 `type: approval` at turn 2 in 3.1 s: `issue_refund` waiting with `order_id: ORD-1002, amount: 174.0` and the carrier's loss as the reason, the two lookups answered; run 2 (`{"note": "checking in"}`) re-advised the same call, `run: resume`, under a millisecond; run 3 (a decision for `toolu_does_not_exist`) refused by the loop and re-advised, the same call still waiting; run 4 (approved) `type: answer` at turn 3 in 1.3 s: "I've issued a full refund of $174.00", `issue_refund` ok |
| A3 - the same order again | order-assistant | a second transaction reached the approval; approved, the order book answered 409 and the model read it: "ORD-1002 has already been refunded (RF-8A8C004C) … no further action is needed"; the call log `issue_refund: error` |
| A4 - a refused decision | order-assistant | ORD-1004 reached the approval in 3.1 s; refused with the note that the customer had asked for a replacement; the model answered in 1.3 s offering "a replacement shipment of the 2 standing desk mats instead of a refund"; the call log `issue_refund: refused` |
| A5 - a watched run | order-assistant | `POST /api/stream/graph/order-assistant` with `accept: text/event-stream` for ORD-1003: 22 lines in 2.3 s - `node.start` ("Investigating the order"), then `run.start`, `turn.start`, `turn.end`, `tool.start`, `tool.end`, `turn.start`, `turn.end`, `run.end`, then the `result` ("in transit with tracking number BP-00912-4471") and `done` |
| B1 - a diagram through the MCP binding | diagram-assistant | `type: answer` in 17.5 s, 2 turns, 1,812 input and 317 output tokens; `render_diagram` → `diagram.render` ok; the binding logged `Handled mermaid.render status=200 exec_time=3593ms`; the answer carries the validated flowchart and the reference; `/tmp/rendered_diagrams/<reference>.svg` (50,062 bytes) and its `.html` page stored by the engine's function |
| C1 - the four documents | document-ingest | answers at turns 3, 3, 4 and 3 (10 to 15 s each); the pack logged `graph.ingest 200 12 nodes, 11 relationships` three times and `11 nodes, 10 relationships` once (the travel policy has three obligations); on the travel policy one `graph_ingest` was `invalid` then `ok` |
| C2 - the supplier agreement again | document-ingest | `type: answer` at turn 3; the graph unchanged: 43 nodes, 43 relationships |
| D1 - "Which deadlines does the supplier agreement set?" | document-query | turn 4, `graph_schema` then two queries (2 and 2 rows): "December 31, 2026 - the first annual price review … December 31, 2027 - the term ends unless the parties renew it", each with `(source: supplier-agreement.md)` |
| D2 - "Which obligations bind Fabrikam Hosting?" | document-query | turn 5, the schema then three queries (8, 1 and 0 rows): the four obligations of the data retention policy and the provider's obligations of the service level agreement, each with its source |
| E1 - the research graph | site-notes | the section `…/guides/knowledge-graph/` crawled in one call (`web.crawl accenture.github.io 200 5 pages, 5 fetched (cut)`), the write asked for at turn 2 and held: `notes/workflow-suspension.md`, 2,228 characters, shown to the person; approved under the same correlation id, `file.write 200 2228 bytes (create)`, `type: answer` at turn 3 in 1.5 s; the file in the workspace cites five pages |

## The graph, counted

| After | Document | Party | Obligation | Rule | Deadline | PARTY_TO | STATED_IN | Nodes without `source` |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| C1, the four documents | 4 | 4 | 15 | 12 | 8 | 8 | 35 | 0 |
| C2, the supplier agreement again | 4 | 4 | 15 | 12 | 8 | 8 | 35 | 0 |

The label counts are the Java drive's table (`agent-tools-live-drive.md`, after its A1), read from a fresh database;
the document node that doubled once in the Java drive, before the ingest prompt spelled its key rule, stayed single
here.

## Reading

- **The models are shared, not ported.** The five JSON files are byte-identical copies of the Java example's; the
  Rust compiler accepted every node type, mapping and skill they use (`graph.resume`, `graph.suspend`, `graph.math`
  with `RESET:` and probes, `f:concat` with four arguments, `llm.run` with `mode`, `progress`, `exception`). What the
  Rust example needed was its own tool functions and the `agent-loop` crate as a dependency - the Java example's
  `extensions/agent-loop` jar - and `diagram.store.folder` in its configuration.
- **The packs do not know which engine calls them.** The same Python helper, tools and binding that served the Java
  drives served this one, with the same configuration; the tools' logs read the same (`graph.ingest 200 12 nodes, 11
  relationships`, `file.write 200 … (create)`).
- **A re-advise costs nothing.** The Rust engine answered a resume without decisions and a refused decision from the
  restored state with no model call, as the Java engine does.
- **Not driven here:** the WebSocket entry (`graph.websocket.enabled`) and the Node.js pack - the Java drives covered
  both, and nothing in this example is pack-specific beyond the peer ports.

## Reproduce

```bash
# the stores (Colima or Docker Desktop)
docker run -d --name mercury-redis -p 6379:6379 redis:7-alpine
GRAPH_PASSWORD=... extensions/agent-tools/dev/graph-up.sh                # in mercury-python
# the pack (mercury-python shown; mercury-nodejs is the same with node dist/src/cli.js and ports 8087, 8089, 8090)
mercury-serve extensions/llm-helper/llm_helper.py -Dllm.model=claude-haiku-4-5
GRAPH_PASSWORD=... mercury-serve extensions/agent-tools/agent_tools.py \
  -Dtools.file.root=<a copy of examples/minigraph-playground/corpus> -Dtools.web.allow=accenture.github.io \
  -Dtools.graph.uri=bolt://127.0.0.1:7687 -Dtools.graph.write=true
mercury-serve extensions/mcp-binding/mcp_binding.py
# the engine, from this example's folder so resources/ is found
cargo build -p minigraph-playground --release
cd examples/minigraph-playground && ../../target/release/minigraph-playground \
  -Dllm.peer.port=8086 -Dmcp.peer.port=8088 -Dtools.peer.port=8089
# the drives
curl -s -X POST http://127.0.0.1:8085/api/graph/order-assistant -H 'content-type: application/json' \
  -H 'x-correlation-id: order-1002-case' -d '{"question": "My order ORD-1002 never arrived. What can you do?"}'
# then approve with the same correlation id: {"turn": <the reply's turn>, "decisions": [{"id": "<the call id>", "approved": true}]}
curl -s -X POST http://127.0.0.1:8085/api/graph/document-ingest -H 'content-type: application/json' \
  -H 'x-correlation-id: ingest-1' -d '{"document": "supplier-agreement.md"}'
curl -s -X POST http://127.0.0.1:8085/api/graph/document-query -H 'content-type: application/json' \
  -H 'x-correlation-id: query-1' -d '{"question": "Which deadlines does the supplier agreement set?"}'
```

The token-free twin of this drive is `examples/minigraph-playground/tests/agent_graphs.rs`: the same five graphs on a
scripted model, stand-ins for the tool routes and the Mermaid binding, and a RESP2 Redis double.
