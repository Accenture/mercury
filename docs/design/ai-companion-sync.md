# Design — Synchronous AI-companion feedback

> **Status:** DRAFT (2026-07-18). Rust R&D prototype first; upstream to Java after it proves out.
> **Serves:** `vision-mercury` (faithful delivery; a fresh agent orients + *operates* from the docs
> and the tool's own responses). **Blueprint thread:** `bp-companion-sync` in `continuity.md`.
> **Origin:** the AI-companion validation sweep (`docs/AI-companion-test.md`), Tutorial-4.

## The gap

The AI-companion HTTP surface is a **write-only command bus with side-channel output**, not a tool:

- **`POST /api/companion/{id}`** dispatches the command fire-and-forget and returns `{status:"accepted"}`.
  The real result — success text *and errors* — is streamed via `say()` to the session's WebSocket
  `{route}.out`, which an HTTP/AI caller never sees.
- To learn the effect, the caller must **poll** `GET /api/graph/session/{id}` (shape) and
  `GET /api/inspect/{id}/{key}` (state) — and a **rejected** command leaves the model unchanged with
  no error, so polling can't even distinguish "no-op" from "rejected".

**Evidence (Tutorial-4).** A capable agent sent an invalid `graph.math` node, received **HTTP 200
"accepted"**, and was blind to the engine's `node compare does not have if:, then: or else:` →
*Graph traversal aborted* (WS-only). It only inferred failure by polling `inspect` and finding empty
state. We worked around it by hand-building a Node WS subscriber (`session subscribe`) purely so the
orchestrator could see the console — a workaround standing in for a missing feature.

Net: the agent operates **write-then-guess**. A true companion needs **synchronous, self-describing
feedback**: send a command → get back what happened.

## Goals / non-goals

**Goals**
- A single call returns the command's **outcome in-band**: `ok`, the console `output`, and the exact
  `error` on failure.
- For `run` / `execute` / `inspect`, fold the **result** (traversal outcome / `output.body`) into the
  response so the poll dance disappears for the common cases.
- **Additive** — the existing fire-and-forget endpoint and the human WebSocket console are unchanged.

**Non-goals**
- Not an MCP tool server (deferred — heavier, and it forks the shared human/AI text surface into an
  AI-only one; the sync response gets ~80% of the value in-band while keeping one surface).
- Not a replacement for the graph-shape GET — the full model still lives behind
  `GET /api/graph/session/{id}`; sync responses carry *outcomes*, not the whole model, to stay lean.
- The human WS console keeps streaming exactly as today.

## Design

### Surface (additive)
A sibling route — **`POST /api/companion/{id}/sync`** (service `post.companion.command.sync`) —
identical body (`text/plain`, one command), returning a structured envelope:

```json
{
  "ok": false,
  "id": "ws-336199-2",
  "command": "create node compare\nwith type Compare\n…",
  "output": ["> create node compare …"],
  "error": "node compare does not have if:, then: or else:",
  "run": null
}
```

- `ok` — did the command succeed (no `ERROR:` line, no aborted traversal)?
- `output` — the console lines the command produced (the same text the WS would show), in order.
- `error` — the first error line, lifted out for convenience (`null` on success).
- `run` — for `run`/`execute`: `{ completed: bool, output_body: {…}, ms: N }` (or `null`). For
  `inspect`: the inspected value. Populated from the same stream; absent otherwise.

The existing `POST /api/companion/{id}` is untouched (parity with Java until Java adopts this).

### Mechanism
The command pipeline already routes **all** output through `say(po, out_route, …)`. The sync path
supplies a **private capture route** as `out` instead of the session's WS `.out`, then awaits
completion and returns what was captured:

1. Validate (id, non-empty command, session exists) — same as today.
2. Mint a unique capture route `companion.sync.{uuid}` and register a one-instance capture function
   that appends each received body to a per-call buffer.
3. Dispatch the command to `graph.command.singleton` with `out = companion.sync.{uuid}` using
   **`PostOffice::request` (RPC)** — `handle()` awaits `handle_request()` (all `say()` calls
   complete) before returning its `"done"` ack, so the RPC resolving marks the command finished.
4. **Completion signaling.** `say()` is fire-and-forget (`send` enqueues to the route mailbox), so
   after the RPC resolves the last lines may still be in the capture route's mailbox. The pipeline
   is FIFO per route, so the sync path enqueues a **sentinel** to the capture route *after* the RPC
   resolves; the capture function signals done when it sees the sentinel (deterministic — no
   arbitrary sleep). *(Prototype may use a short bounded settle; production uses the sentinel.)*
5. Drain the buffer → classify (`ok`/`error`), parse `run`/`inspect` results, build the envelope.
6. Deregister the capture route; return the JSON.

> **Tee to both (v1 — real-time human+AI collaboration).** The capture sink also **fire-and-forget
> forwards** each line (except the sentinel) to the session's real WebSocket `.out` route. So the
> caller gets the structured response *and* a human watching the Playground UI sees the same output
> live — and, because the command service already re-dispatches a primary's command to each
> subscriber's own `.out` (`handle_command`), any **subscribed** session (e.g. a product owner who
> ran `session subscribe`) watches too. This is the headline goal: an architect + AI draft the graph
> while others watch/participate on the same live session; suspend/resume across sprints via
> `export`/`import`. *(Without the tee, sync output went only to the caller — a watching human saw a
> quiet console even as the model changed; observed live during verification.)*

> **Why capture-route rather than threading a sink through `say()`?** The command runs in the
> command-service task (reached over the event bus), not the endpoint's task, so an in-process
> task-local buffer can't cross the boundary. A private route *is* the correlation, and it reuses
> existing primitives (`Platform::register`, `PostOffice::request`) with no signature churn in the
> ~dozen command functions.

### Alternative considered — handler returns the transcript
Have `handle()` accumulate its `say()` output and return it as the RPC reply (no capture route).
Cleaner conceptually, but `say()` is threaded through ~a dozen functions by `out_route`; adding a
sink parameter to all of them is invasive. Deferred; the capture-route approach is behaviorally
equivalent and localized. If we later add a first-class output-sink abstraction, revisit.

## Java parity plan
The gap exists in Java too (`PostCompanionCommand` is fire-and-forget). Prove the design in the Rust
R&D repo, then **PR the design upstream** to `Accenture/mercury-composable` (a `/command` sibling
route + the same envelope), keeping the two engines aligned — same as the graph.math grammar fix
(PR #187).

## Open questions
- Envelope field names / shape — settle before the Java PR (this is the cross-vendor contract).
- Should `run`'s `output_body` always be inlined, or gated by size (large payloads already spill to
  `GET /api/inspect/...`)? Lean: inline under a threshold, else a pointer — mirror the existing
  large-payload rule in the command handler.
- Multi-command batch in one call? Deferred — one command per call keeps it simple and mirrors the
  existing contract.
