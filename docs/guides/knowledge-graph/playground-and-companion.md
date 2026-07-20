# Playground & AI Companion

The **MiniGraph Playground** is a browser workbench for building, dry-running, and inspecting
active knowledge graphs interactively — and the **companion endpoints** let an AI agent drive
the same live session over HTTP while humans watch every step in real time. Together they are
the platform's human–AI collaboration surface: both parties work on the *same* model, not on
code.

Both are **dev-only**: the workbench (WebSocket UI, command service, companion endpoints)
registers only when `app.env=dev`, and there is no authentication — never expose them beyond
a trusted development host. Deployed graphs still run in production through
`POST /api/graph/{graph-id}`.

## The workbench

```bash
cargo run -p minigraph-playground
```

Open <http://127.0.0.1:8100>. The page is split into two panels:

- **The console** (left) — every command and its output, with a command input at the bottom.
  Enter sends; Shift+Enter inserts a new line, and pasting a multi-line block works too. The
  input keeps a command history and offers suggestions as you type.
- **The model** (right) — a **Graph** tab rendering the current model live as commands change
  it, and a **Graph Data** tab showing the same model as JSON.

Around them: a **Save** button bookmarks the current graph under a name in your browser
(with a menu to reload or delete saved graphs), the **Workspace** sidebar collects clipped
nodes for reuse, the **Tools** menu switches between the MiniGraph and JSON-Path playgrounds,
and the `?` button (or ``Ctrl+` ``) opens a help panel serving the same pages as the `help`
command — including the [thirteen tutorials](build-your-first-graph.md#where-to-go-next).

## Sessions

Each open Playground connection is a **session**. The console's welcome message carries its
public id and, conveniently, the companion URL an agent will need:

```
session ws-100001-1 started
Companion endpoint: /api/companion/ws-100001-1
```

The `session` command shows the current status at any time:

```
> session
Session ws-100001-1 started since 2026-07-19 10:20:32.054
subscribed by [ws-485844-4]
```

### Multi-party viewing

Sessions can be shared, so several people follow (and drive) the same graph together. From
*another* browser session, subscribe to the primary one:

```
session subscribe ws-100001-1
```

Commands then mirror both ways — input from either session runs in both, keeping the models
in sync. On subscribe the graphs are aligned: an empty primary receives your draft; otherwise
the primary's graph replaces it. You can subscribe only to a **primary** session (one that
has not itself subscribed) and never to yourself. `session unsubscribe` detaches, keeping
your current graph; `session reset` restarts the session and disconnects its subscribers.

Note that `session reset` resets *subscriptions*, not your work: it does **not** clear the
draft graph — the UI restores the draft on reconnect. To start clean, delete the nodes
explicitly. The full rules are in the [command grammar](command-reference.md#session).

## The companion endpoints

The companion endpoints let an HTTP client — a script, a test harness, or an AI agent —
dispatch a Playground command into an **already-open** session, as if it were typed into the
console. Two variants:

| Endpoint | Behavior |
|---|---|
| `POST /api/companion/{session-id}/sync` | **Synchronous** — returns the command's outcome in-band as a JSON envelope. The preferred endpoint for AI agents. |
| `POST /api/companion/{session-id}` | **Fire-and-forget** — returns an immediate `{"status": "accepted", …}` acknowledgment; the outcome streams to the WebSocket console only, so the caller is blind to errors. Kept for compatibility. |

Both take `Content-Type: text/plain` with exactly **one** command in the body (multi-line
commands are one command). Status codes: `200` executed (read the body), `400`
missing/empty/non-text body, `404` no active session for that id. To read the current model
of a live session: `GET /api/graph/session/{session-id}`.

!!! note "Rust port"
    The synchronous `/sync` endpoint originated in this Rust port and has been merged into
    the Java engine upstream — it is available in **both** engines with a byte-identical
    REST contract, so companion clients are engine-neutral.

### The `/sync` envelope

```bash
SESSION="ws-100001-1"   # from the console welcome message

curl -sS -X POST "http://127.0.0.1:8100/api/companion/${SESSION}/sync" \
  -H 'Content-Type: text/plain' \
  --data-binary $'create node root\nwith type Root\nwith properties\npurpose=demo'
```

```json
{
  "ok": true,
  "id": "ws-100001-1",
  "command": "create node root...",
  "output": ["> create node root...", "node root created"]
}
```

`ok`
:   Whether the command succeeded, derived from the console output with whole-output
    context (a benign fallback, like `import` finding the model in the deployed location
    after missing the temp folder, is correctly `true`).

`output`
:   The console lines the command produced — the same lines a human sees in the browser.

`error`
:   The first failing line — present only when `ok` is `false`.

`result`
:   Structured data when the command yields it — for example, a `run`'s traversal outcome
    including `output.body`, so the caller can verify the response payload directly.

Null fields are **omitted** from the wire: a success carries no `error` key, and no `result`
key unless the command produced data — treat absent as null. Because every response carries
`ok`/`error`/`result`, an agent can **self-correct in-band**: read the error, fix the
command, and re-issue, with no human relaying the console. The full contract is specified in
the [AI agent guide](ai-agent-guide.md#contract).

### The live tee — humans watch agents work

Everything a companion does is **also teed to the session's browser console** — and, through
the session fan-out, to every subscribed session. A person watches the graph take shape in
real time while the agent builds it, sees the same errors the agent sees, and can step in on
the same console at any point. The human steers and certifies; the AI drafts and refines;
both work on one live model. (One operator at a time: don't POST while a human is typing in
the same instant.)

### Companions are read-only on session topology

A companion is an *assistant to* the session in the URL — not a WebSocket session of its own.
Both companion endpoints therefore **reject** `session subscribe`, `session unsubscribe`, and
`session reset`; only the read-only `session` status query is allowed. The refusal is
returned in-band (and teed to the console, so watchers see the attempt). Subscriptions are
managed from browser-connected sessions only.

## Discovery

Two read-only commands make the server's deployable assets self-describing — useful to
humans, and essential to agents composing [delegation](composing-the-layers.md) without an
out-of-band brief:

```
list graphs        # deployed graph models, each with its root node's purpose
list flows         # Event Script flows, each with its description
```

!!! note "Discovery in both engines"
    The discovery commands originated in this Rust port and were contributed upstream — the
    Java engine has them too
    ([mercury-composable#199](https://github.com/Accenture/mercury-composable/pull/199)).

## Where this leads

This page is the seed of the platform's collaboration vision made concrete: a fresh AI agent,
given only the [engine-verified AI documentation](index.md#in-this-section), built and ran
all thirteen Playground tutorials through `/sync` — self-correcting from in-band errors while
maintainers watched live through subscribed sessions. If an agent builds graphs for you,
point it at the [AI agent guide](ai-agent-guide.md); everything it needs, including the
[command grammar](command-reference.md) and its machine-readable form, is linked from there.

## See also

- [Build Your First Graph](build-your-first-graph.md) — the command loop in a full
  walkthrough.
- [Composing the Layers](composing-the-layers.md) — what to build once two parties share a
  session: delegation, tasks, and discovery.
- [AI agent guide](ai-agent-guide.md) — the companion contract in full, with a canonical
  build recipe.

---

*Adapted from the mercury-composable guide `knowledge-graph/playground-and-companion.md`; behavior verified against this repository's engine-verified AI documentation.*
