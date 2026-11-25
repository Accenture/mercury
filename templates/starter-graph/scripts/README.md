# Playground scripts

## ai-companion-broker.mjs — the AI companion broker

Lets an AI agent **host** a MiniGraph Playground session instead of borrowing a human's
browser session. The broker opens one WebSocket session against a running
playground app, keeps it alive with the same ping cadence the web UI uses, and
exposes a tiny localhost control API. Zero dependencies; Node.js >= 22.

This folder holds the only copy: the `minigraph-playground` and `distributed-cache-example`
examples run it from here, and a project copied from this template carries it along.

Use it instead of hand-rolling a WebSocket client: the session contract includes
a keep-alive (ping every ~20 s), and a hand-rolled client that misses it appears
to work, then dies silently at the server's idle timeout — typically in the
middle of the collaboration it was hosting.

```bash
node scripts/ai-companion-broker.mjs --target http://127.0.0.1:8303 --port 8765
```

Both flags are runtime overrides: `--target` points the broker at whatever host and port the
app serves — this starter's default is `8303` (`rest.server.port`), while the broker's built-in
default is `http://127.0.0.1:8085` (the minigraph-playground example's port), so pass your
app's actual port explicitly; `--port` moves the broker's own control API (default `8765`).

| Call | Effect |
|---|---|
| `GET /session` | Current `sessionId`, connection state, prior session ids, `subscribedTo` (the primary session this one mirrors, or `null`) and `subscribers` (the sessions that mirror this one) |
| `GET /console?lines=50` | Recent console lines teed to this session (ping/pong filtered) |
| `POST /subscribe` | Body `{"sessionId": "ws-..."}`: this session subscribes to that primary session, as `session subscribe <id>` typed in a browser would; answers `{sessionId, subscribedTo}`, or the engine's refusal |
| `POST /start` | Close any current session, open a fresh one, return the new id |
| `POST /stop` | Close the session; auto-reconnect stays off until `/start` |
| `POST /shutdown` | Stop the broker process |

### The collaboration pattern (recommended)

1. The human starts the MiniGraph application in a terminal, so they can see its
   application log.
2. The human asks the agent to start a persistent WebSocket connection with the AI
   companion broker. The agent starts it, reads the session id from `GET /session`
   and hands the human `session subscribe <id>` to type in the Playground's console.
   Session sync is **symmetric**: every command (except `session` topology commands)
   propagates to the primary and all subscribers, so AI and humans are equal
   co-authors of one shared model.
3. Once the human has subscribed (`GET /session` lists them under `subscribers`),
   they collaborate: the human explains the intent of the business use case, and the
   agent prototypes it through the companion endpoint
   (`POST /api/companion/{session-id}/sync`) while the human watches the graph grow in
   the Playground; then they dry-run it together. Every output line is teed to all
   subscribed consoles live.

Because the **agent** holds the primary session, the session never times out. A browser
that throttles a background tab stops the web UI's keep-alive pings, and the server closes
that tab's session at its idle timeout (`websocket.idle.timeout`, 60 seconds by default);
the human reloads the page and types `session subscribe <id>` again, and the
work-in-progress graph syncs back in full. Nothing is lost.

### When the human's session came first

If the human opened the Playground first and handed their session id to the agent, the
agent starts the broker and calls `POST /subscribe` with that id. The broker's session
then mirrors the human's and holds a copy of the shared draft; the agent drives `/sync`
through the **broker's** session id (a subscriber forwards each command to the primary).
If the human's browser closes, the engine frees the broker's session as a primary that
keeps the work-in-progress graph: the broker logs it and `GET /session` shows
`subscribedTo: null`. The agent hands its own session id to the human, who subscribes
from a new tab and recovers the work.

Both paths work; the first is easier for both the agent and the human.

### Caveats

- **Keep-alive protects against idle, not restarts.** Restarting the app
  destroys the session and any unexported graph. The broker auto-reconnects and
  captures the **new** session id (`previousSessionIds` records the old ones),
  but subscribers must re-subscribe and unexported work is gone — export before
  any restart.
- **Dev-only.** The Playground (and therefore this broker) is gated by
  `app.env=dev`; the control API binds `127.0.0.1` only. Never expose either
  beyond a trusted dev host.
- The broker works unchanged against the Java and Rust playground engines —
  both announce `session ws-NNNNNN-N started` on open, answer the same
  ping/pong keep-alive frames and report subscriptions in the same words.

## deploy-graph-set.mjs and undeploy-graph-set.mjs — hot deployment without the Playground

Deploy a certified graph set to a running MiniGraph engine without a restart, and undeploy it by
name (RFC-0017), for an engine whose authoring UI is off - the production path. Zero dependencies;
Node.js 26 LTS (minimum 22). This folder holds the only copy, as it does for the broker.

```bash
node scripts/deploy-graph-set.mjs --engine https://gateway.example.com/engine --file settlement.pack \
     --sha256 9f2c... --header "authorization: Bearer $DEPLOY_TOKEN"
node scripts/undeploy-graph-set.mjs --engine https://gateway.example.com/engine --graph settlement
node scripts/undeploy-graph-set.mjs --engine https://gateway.example.com/engine --list
node scripts/deploy-graph-set.mjs --file settlement.pack --digest
```

| Flag | Meaning |
|---|---|
| `--engine <url>` | The engine's base URL, or the gateway's path in front of it; the entry's path is appended as given, so a gateway prefix such as `/engine` reaches the engine |
| `--file <path>` | The graph set, `<set>.pack`, named after its main graph |
| `--sha256 <hex>` | The digest the field's ledger holds, sent as `X-Graph-Set-SHA256`; the script checks the file against it before the upload, and the engine refuses other bytes |
| `--header 'name: value'` | A transport header, repeatable - a gateway's token taken from the shell's environment, so it never sits in a command history as a literal |
| `--digest` | Print the package's SHA-256 and exit: the ledger step, without an upload |
| `--list` | Print the deployed sets the engine records and exit (either script) |
| `--graph <id>` | The main graph's id to undeploy, which is the set's name |

The answer prints as the engine gives it, one JSON document on stdout. Exit codes: `0` on success; `1`
on a refusal, with the engine's status and reasons on stderr (`refused (409): graph id x is owned by ...`;
a 404 `Resource not found` means the entry is not served - hot deployment is off on this engine, `graph.hot.deploy`
and `graph.hot.undeploy`, or the engine is older than the entries, or the base URL is wrong - and a 503 that a switch
is on but the engine refused its folder for this run); `2` on a usage error.
No prompt: a script in a pipeline runs unattended. Authentication is the gateway's: the scripts carry
whatever headers they are given and nothing else.
