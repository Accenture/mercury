# Playground scripts

## playground-session-broker.mjs

Lets an AI agent **host** a Playground session instead of borrowing a human's
browser session. The broker opens one WebSocket session against a running
playground app, keeps it alive with the same ping cadence the web UI uses, and
exposes a tiny localhost control API. Zero dependencies; Node.js >= 22.

```bash
node scripts/playground-session-broker.mjs --target http://127.0.0.1:8085 --port 8765
```

Both flags are runtime overrides: `--target` points the broker at whatever host and port the
playground app serves (default `http://127.0.0.1:8085` — pass your app's actual port, e.g.
`--target http://127.0.0.1:8087`); `--port` moves the broker's own control API (default `8765`).

| Call | Effect |
|---|---|
| `GET /session` | Current `sessionId`, connection state, prior session ids |
| `GET /console?lines=50` | Recent console lines teed to this session (ping/pong filtered) |
| `POST /start` | Close any current session, open a fresh one, return the new id |
| `POST /stop` | Close the session; auto-reconnect stays off until `/start` |
| `POST /shutdown` | Stop the broker process |

### The collaboration pattern

1. The agent starts the broker and reads the session id from `GET /session`.
2. The agent hands the id to the humans; each opens the Playground in a browser
   and types `session subscribe <id>`. Session sync is **symmetric**: every
   command (except `session` topology commands) propagates to the primary and
   all subscribers, so AI and humans are equal co-authors of one shared model.
3. The agent drives its work through the companion endpoint
   (`POST /api/companion/{session-id}/sync`); every output line is teed to all
   subscribed consoles live.
4. Because the **agent** holds the primary session, a human who drops
   (backgrounded tab, idle timeout) just re-subscribes — the work-in-progress
   graph syncs back to them. Nothing is lost.

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
  both announce `session ws-NNNNNN-N started` on open and answer the same
  ping/pong keep-alive frames.
