# Playground Webapp

A lightweight, browser-based developer tool for interacting with Mercury Composable backend services over WebSocket. Each **playground** is a dedicated page for a specific backend endpoint — you write commands, send payloads, and observe live responses in real time.

---

## Table of Contents

- [Getting Started](#getting-started)
- [Layout Overview](#layout-overview)
- [Connection Bar](#connection-bar)
- [Console Output (Left Panel — Top)](#console-output-left-panel--top)
- [Command Input (Left Panel — Bottom)](#command-input-left-panel--bottom)
- [Payload Editor (Right Panel)](#payload-editor-right-panel)
- [Navigation & Quick Links](#navigation--quick-links)
- [Keyboard Reference](#keyboard-reference)
- [Persistence](#persistence)
- [Available Playgrounds](#available-playgrounds)
- [Adding a New Playground](#adding-a-new-playground)

---

## Getting Started

### Development server

```bash
npm install
npm run dev
```

Open [http://localhost:5173](http://localhost:5173) in your browser. The dev server proxies WebSocket connections to `ws://localhost:3000`.

### Production release (deploy to backend)

```bash
npm run release
```

This builds the app and reloads the output into the backend's `src/main/resources/public/` directory. The Spring Boot / Mercury Composable server then serves the static files.

Once the backend JAR is running:

```
java -jar target/minigraph-playground-{version}.jar
```

visit [http://127.0.0.1:8085](http://127.0.0.1:8085).

---

## Layout Overview

```
┌─────────────────────────────────────────────────────────────────┐
│  Title                                                          │
│  ● Disconnected   ws://…   [Start]    Tools: …   Quick Links:   │  ← Header
├────────────────────────────┬────────────────────────────────────┤
│                            │                                    │
│   Console Output           │   JSON/XML Payload                 │
│   ┌────────────────────┐   │   ┌────────────────────────────┐   │
│   │  (message list)    │   │   │  (textarea)                │   │
│   └────────────────────┘   │   └────────────────────────────┘   │
│                            │   Quick load: JSON: … XML: …       │
│   Command  [Multiline □]   │                                    │
│   [textarea]   [Send]      │                                    │
│   Enter to send · …        │                                    │
├────────────────────────────┤────────────────────────────────────┤
│       Left Panel           │         Right Panel                │
│      (resizable)    ◀ ▶    │         (resizable)                │
└────────────────────────────┴────────────────────────────────────┘
```

The two panels are **resizable** — drag the divider between them. On viewports narrower than 768 px the panels stack vertically.

---

## Connection Bar

Located in the header below the page title. Shows the current WebSocket state and controls.

| State | Indicator | Button |
|---|---|---|
| **Disconnected** | 🔴 red dot | **Start** — opens the connection |
| **Connecting** | 🟡 yellow dot (pulsing) | **Connecting…** — disabled while handshake is in progress |
| **Connected** | 🟢 green dot (pulsing) | **Stop Service** — closes the connection |

The WebSocket URL is displayed between the status label and the button, e.g. `ws://localhost:3000/ws/graph/playground`.

---

## Console Output (Left Panel — Top)

Displays all WebSocket traffic in chronological order. The console holds up to **200 messages**; the oldest is evicted when the limit is reached.

### Message types

| Icon | Type | Meaning |
|---|---|---|
| ℹ️ | `info` | Lifecycle events — connected, disconnected |
| 👋 | `welcome` | Server greeting on open |
| 📝 | `raw` | Plain-text response (not JSON-structured) |
| ❌ | `error` | Server or client-side error |

When a message body is valid JSON, it is rendered as a **collapsible tree** (via `react-json-view-lite`) with the first two levels expanded by default. Plain text falls back to a preformatted span.

### Toolbar buttons

| Button | Action |
|---|---|
| **Disable AutoScroll / Enable AutoScroll** | Toggles whether the console automatically scrolls to the newest message. Useful when reviewing older output while a connection is active. |
| **Copy Output** | Copies all visible messages to the clipboard (one raw message per line). |
| **Clear** | Removes all messages from the console (does not disconnect). |

---

## Command Input (Left Panel — Bottom)

Type commands here and send them to the connected backend service.

### Single-line mode (default)

- **Enter** — sends the command and clears the input; focus returns to the textarea.
- **Shift+Enter** — inserts a newline (expands the textarea) without sending.
- **↑ / ↓ Arrow keys** — navigate command history (last **50** commands, persisted across sessions).

### Multiline mode

Check the **Multiline** checkbox to switch to a 5-row textarea.

- **Ctrl+Enter** (or **⌘+Enter** on macOS) — sends.
- **Enter** — inserts a newline.
- **Shift+Enter** — also inserts a newline.
- **↑ / ↓ Arrow keys** — navigate history.

### Send button

The **Send** button is disabled when:
- Not connected, **or**
- The command input is empty / whitespace-only.

In single-line mode the Send button sits to the right of the textarea. In multiline mode it appears full-width below it.

### `load` command

Typing `load` and sending triggers a two-step sequence:
1. The literal string `load` is sent to the server.
2. The contents of the **Payload Editor** are sent as a second frame.

This populates the backend's working context with your JSON/XML document. The command is rejected client-side (with an error message in the console) if:
- The payload textarea is empty, or
- The payload exceeds **64 000 characters**.

---

## Payload Editor (Right Panel)

A persistent textarea for the JSON or XML document you want to work with.

### Validation indicators

Shown in the label row as you type:

| Indicator | Meaning |
|---|---|
| `JSON` badge (green) | Content is valid JSON |
| `XML` badge (green) | Content is valid XML |
| ✅ | Payload is valid |
| ❌ | Payload is invalid |
| `0 / 64000` counter | Character count versus the 64 000 limit |

An inline error message appears below the textarea when the content is malformed.

### Format button

Prettifies the payload with 2-space indentation. Only enabled for valid JSON (XML formatting is not supported).

### Quick load samples

Buttons below the textarea load pre-built sample documents instantly:

| Group | Samples |
|---|---|
| **JSON** | simple · nested · array |
| **XML** | simple · nested · array |

Clicking a sample replaces the textarea contents immediately.

> The payload is **always editable** — you can paste and edit before, during, or after a connection. It is not sent automatically; it is only used when the `load` command is issued.

---

## Navigation & Quick Links

The navigation bar in the header has two sections:

**Tools** — links to each configured playground. The active playground link is highlighted.

**Quick Links** — shortcuts to backend info endpoints (open in a new tab):

| Label | URL |
|---|---|
| INFO | `/info` |
| LIBRARIES | `/info/lib` |
| SERVICES | `/info/routes` |
| HEALTH | `/health` |
| ENVIRONMENT | `/env` |

---

## Keyboard Reference

| Key | Context | Action |
|---|---|---|
| **Enter** | Single-line command | Send command |
| **Shift+Enter** | Single-line command | New line (no send) |
| **Ctrl/⌘+Enter** | Multiline command | Send command |
| **Enter** | Multiline command | New line |
| **↑** | Command input | Recall previous command from history |
| **↓** | Command input | Move forward in history (↓ past index 0 clears the field) |

---

## Persistence

All data is stored in **`localStorage`** per playground — nothing is sent to the server on page load.

| Data | Storage key (example) | Limit |
|---|---|---|
| Payload textarea | `minigraph-last-payload` | — |
| Command history | `minigraph-command-history` | 50 entries |
| Panel split ratio | `<path>-panel-split` | — |

Each playground has its own independent storage keys, so switching between playgrounds never overwrites the other's data.

---

## Available Playgrounds

| Route | Title | WebSocket endpoint | Purpose |
|---|---|---|---|
| `/json-path` | JSON-Path Playground | `/ws/json/path` | Evaluate JSONPath expressions against a loaded document |
| `/minigraph` | Minigraph Playground | `/ws/graph/playground` | Create and query a Knowledge Graph |

### Minigraph quick start

1. Click **Start** to connect.
2. Type `help` in the command input and press **Enter** to list all available commands.
3. Paste a JSON document into the Payload Editor (or click a **Quick load** sample).
4. Type `load` and press **Enter** — the document is sent and stored in the `response` node.
5. Type `response` to retrieve the full loaded object.
6. Use dot-bracket notation for simple retrieval: `response.hello` returns `"world"` given `{ "hello": "world" }`.
7. Use JSONPath for richer queries: `$.response.hello` — see e.g. [SmartBear JSONPath docs](https://support.smartbear.com/alertsite/docs/monitors/api/endpoint/jsonpath.html).

### JSON-Path quick start

1. Click **Start** to connect.
2. Paste any JSON document into the Payload Editor and send `load`.
3. Enter a JSONPath expression (starting with `$`) in the command input and press **Enter**.

---

## Adding a New Playground

1. Open `src/config/playgrounds.ts`.
2. Add a new entry to the `PLAYGROUND_CONFIGS` array:

```ts
{
  path:              '/my-tool',
  label:             'My Tool',           // shown in the nav bar
  title:             'My Tool Playground', // shown as the page heading
  wsPath:            '/ws/my/endpoint',   // backend WebSocket path
  storageKeyPayload: 'mytool-last-payload',
  storageKeyHistory: 'mytool-command-history',
}
```

3. That's it — the route, navigation link, and storage keys are all generated automatically. No changes to `App.tsx` or `Navigation.tsx` are needed.

To add new **Quick load** samples, add entries to the `SAMPLE_DATA` object in the same file using the `json_<label>` or `xml_<label>` key convention.

---

## Tech Stack

| Library | Version | Role |
|---|---|---|
| React | 19 | UI framework |
| TypeScript | 5.9 | Type safety |
| Vite | 6 | Dev server + build |
| react-router-dom | 7 | Client-side routing |
| react-resizable-panels | 4.6 | Draggable split-panel layout |
| react-json-view-lite | 2.5 | Collapsible JSON tree in console |
