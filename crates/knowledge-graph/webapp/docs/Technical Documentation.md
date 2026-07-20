# Minigraph Playground — Codebase Walkthrough & Knowledge Transfer

**Audience:** Engineers inheriting or maintaining this webapp  
**Branch:** `feature/playground`  
**Last updated:** June 1, 2026

> **Note on line-number citations:** All `[Lxx]` anchors in this document are approximate and reflect the state of the codebase at the last documentation update. The codebase continues to grow; use the cited symbols and function names to locate code rather than relying on line numbers literally.

---

## Table of Contents

1. [What This App Does](#1-what-this-app-does)
2. [Technology Stack & Tooling](#2-technology-stack--tooling)
3. [High-Level Features](#3-high-level-features)
   - 3.1 [Multi-Playground Architecture](#31-multi-playground-architecture)
   - 3.2 [Navigation-Persistent WebSocket Connections](#32-navigation-persistent-websocket-connections)
   - 3.3 [Interactive Console (REPL-like)](#33-interactive-console-repl-like)
   - 3.4 [Graph Visualisation](#34-graph-visualisation)
   - 3.5 [Auto-Graph Refresh](#35-auto-graph-refresh)
   - 3.6 [Bundled Help Panel (Developer Guides)](#36-bundled-help-panel-developer-guides)
   - 3.7 [Large Payload Handling](#37-large-payload-handling)
   - 3.8 [JSON-Path ↔ Minigraph Cross-Playground Routing](#38-json-path--minigraph-cross-playground-routing)
   - 3.9 [Two-Step Payload Upload (REST Handshake)](#39-two-step-payload-upload-rest-handshake)
   - 3.10 [Saved Graph Bookmarks](#310-saved-graph-bookmarks)
   - 3.11 [Keep-Alive Pings](#311-keep-alive-pings)
   - 3.12 [Mock-Data Upload Modal](#312-mock-data-upload-modal)
   - 3.13 [Save-Name Management](#313-save-name-management)
   - 3.14 [Protocol Kernel (Centralized Message Classification)](#314-protocol-kernel-centralized-message-classification)
   - 3.15 [Clipboard Panel](#315-clipboard-panel)
   - 3.16 [UI Node Authoring (Create Node)](#316-ui-node-authoring-create-node)
4. [Repository Layout](#4-repository-layout)
5. [Architecture Overview](#5-architecture-overview)
6. [Layer-by-Layer Walkthrough](#6-layer-by-layer-walkthrough)
   - 6.1 [Entry Point & Routing](#61-entry-point--routing)
   - 6.2 [Playground Configuration](#62-playground-configuration)
   - 6.3 [WebSocket State — Context + Hook](#63-websocket-state--context--hook)
   - 6.4 [Playground.tsx — The Orchestrator](#64-playgroundtsx--the-orchestrator)
   - 6.5 [Left Panel — Console & Command Input](#65-left-panel--console--command-input)
   - 6.6 [Right Panel — Tabs](#66-right-panel--tabs)
   - 6.7 [Graph Pipeline](#67-graph-pipeline)
   - 6.8 [Protocol Kernel Layer](#68-protocol-kernel-layer)
   - 6.9 [Automation Hooks](#69-automation-hooks)
   - 6.10 [Utilities](#610-utilities)
   - 6.11 [Navigation Bar](#611-navigation-bar)
   - 6.12 [Saved Graphs](#612-saved-graphs)
   - 6.13 [Graph Authoring Layer](#613-graph-authoring-layer)
7. [Key Engineering Decisions](#7-key-engineering-decisions)
8. [Data & Message Flows](#8-data--message-flows)
   - 8.1 [User Sends a Command](#81-user-sends-a-command)
   - 8.2 [User Pins a Graph](#82-user-pins-a-graph)
   - 8.3 [Auto-Refresh After Mutation](#83-auto-refresh-after-mutation)
   - 8.4 [Large Payload Flow](#84-large-payload-flow)
   - 8.5 [REST Upload Handshake](#85-rest-upload-handshake)
   - 8.6 [Mock-Data Upload Flow](#86-mock-data-upload-flow)
   - 8.7 [Save-Name Lifecycle](#87-save-name-lifecycle)
   - 8.8 [Create Node UI Flow](#88-create-node-ui-flow)
9. [State Ownership Map](#9-state-ownership-map)
10. [Build, Dev & Deploy](#10-build-dev--deploy)
11. [Extending the App](#11-extending-the-app)
12. [Pitfalls & Gotchas](#12-pitfalls--gotchas)

---

## 1. What This App Does

The **Minigraph Playground** is a React-based developer tool that communicates with a Java Spring Boot backend over **WebSocket** and **REST**. It provides two interactive playgrounds:

| Playground | URL | Purpose |
|---|---|---|
| **Minigraph** | `/` | CRUD operations on a node-and-edge graph model via text commands |
| **JSON-Path** | `/json-path` | Evaluate JSON-Path expressions against a JSON payload |

The UI offers:
- A **live WebSocket console** (like a REPL) where commands are sent and responses printed
- A **ReactFlow graph visualiser** that renders the current graph model
- A **bundled help panel** — `help` commands and their topics are served from files bundled at build time; the panel is a resizable right-side overlay toggled by a header button or `Ctrl + \``
- A **JSON/XML payload editor** for the JSON-Path playground
- **Saved graph bookmarks** (export/import graph snapshots by name)
- **Large payload handling** — payloads exceeding 64 KB are automatically fetched via REST and displayed inline
- **Mock-data upload modal** — when the server responds to a command with an upload invitation, a modal dialog opens automatically so the user can paste, drag-and-drop, or browse a JSON file and POST it to the provided endpoint
- **Clipboard panel** — a resizable sidebar (Minigraph only) for clipping nodes from the graph and pasting their create/update command templates into the console input; persisted in IndexedDB and synchronised across open tabs via BroadcastChannel
- **UI node authoring** — Minigraph-only create-node workflow with an empty-graph entry point and a graph-pane context menu entry point; the modal serialises form values into the same raw text command grammar used by the console

The built output (`dist/`) is copied into the Java application's `src/main/resources/public/` directory and served as a static SPA by the same Spring Boot server that provides the WebSocket and REST endpoints.

---

## 2. Technology Stack & Tooling

| Concern | Choice | Notes |
|---|---|---|
| Framework | **React 19** | Uses the new React Compiler (`babel-plugin-react-compiler`) |
| Language | **TypeScript 5.9** | Strict, no implicit any |
| Build tool | **Vite 6** | Fast HMR in dev; Rollup-based production bundle |
| Routing | **react-router-dom v7** | `BrowserRouter` + `<Routes>` |
| Graph renderer | **@xyflow/react v12** | ReactFlow — nodes, edges, minimap, controls |
| Panel layout | **react-resizable-panels v4** | Draggable left/right split |
| Markdown | **react-markdown + remark-gfm** | GitHub-flavoured Markdown |
| JSON viewer | **react-json-view-lite** | Collapsible inline JSON tree |
| SVG components | **vite-plugin-svgr** | SVG files imported with `?react` are transformed into React components; plain `?url` / asset imports are unaffected |
| Styling | **CSS Modules** | Per-component `.module.css` files; global resets in `index.css` |
| State persistence | **localStorage** | Via custom `useLocalStorage` hook |
| Testing | **Vitest 3** | `npm run test` / `npm run test:watch`; globals enabled; built-in `node` environment (tests are pure logic — no DOM or rendering library needed) |
| Dev proxy | **Vite server.proxy** | `/ws`, `/api`, `/health`, `/info`, `/env` → `localhost:8085` |

### React Compiler

The project opts in to the **React Compiler** (`babel-plugin-react-compiler`), which automatically inserts memoisation. This means you will rarely see manual `useMemo` or `useCallback` calls needed purely for performance — they are present only where the semantics require a stable reference (e.g. a ref updated in an effect, an empty-dep-array `useCallback`). Do not add gratuitous memoisation; let the compiler do its job.

---

## 3. High-Level Features

### 3.1 Multi-Playground Architecture
The app supports multiple independent playgrounds configured entirely in one file (`src/config/playgrounds.ts`). Adding a new playground requires no changes to routing, navigation, or state management — only a new entry in `PLAYGROUND_CONFIGS`.

**Key code locations:**
- `src/config/playgrounds.ts` [L33–L55](../src/config/playgrounds.ts#L33) — `PlaygroundConfig` interface defining every per-playground property
- `src/config/playgrounds.ts` [L57–end](../src/config/playgrounds.ts#L57) — `PLAYGROUND_CONFIGS` array (the only file to edit for a new playground)
- `src/App.tsx` [L21–L22](../src/App.tsx#L21) — routes auto-generated: `PLAYGROUND_CONFIGS.map((cfg) => <Route … element={<Playground config={cfg} />} />)`
- `src/components/Navigation.tsx` [L103–end](../src/components/Navigation.tsx#L103) — nav links and status dots also auto-generated from the same array

---

### 3.2 Navigation-Persistent WebSocket Connections
Switching between the Minigraph and JSON-Path playgrounds does **not** close either WebSocket connection. Each connection is owned by `WebSocketContext` which lives above `<Routes>` in the tree, so both sockets stay alive simultaneously. The navigation bar shows a live status dot per playground.

**Key code locations:**
- `src/App.tsx` [L14–L29](../src/App.tsx#L14) — `<WebSocketProvider>` wraps `<BrowserRouter>`, not the other way around
- `src/contexts/WebSocketContext.tsx` [L1–L10](../src/contexts/WebSocketContext.tsx#L1) — explains the "above Routes" contract in the module JSDoc
- `src/contexts/WebSocketContext.tsx` [L30](../src/contexts/WebSocketContext.tsx#L30) — `WsPhase = 'idle' | 'connecting' | 'connected'`
- `src/contexts/WebSocketContext.tsx` [L147–L157](../src/contexts/WebSocketContext.tsx#L147) — `useReducer`-managed `slots` map keyed by `wsPath`; `wsRefs` / `pingRefs` / `msgIdRefs` live in `useRef` outside the reducer, not inside `SlotState`
- `src/contexts/WebSocketContext.tsx` [L206–L270](../src/contexts/WebSocketContext.tsx#L206) — `connect()`: opens socket, sends `{"type":"welcome"}`, starts ping interval
- `src/components/Navigation.tsx` [L21–L28](../src/components/Navigation.tsx#L21) — `aggregateDotStatus()` + `phaseToDotStatus()` derive nav dot colours

---

### 3.3 Interactive Console (REPL-like)
- Messages are printed in real time with type-based icons (ℹ️ ❌ 👋)
- JSON messages are rendered as a collapsible tree (`react-json-view-lite`)
- Graph-link messages (🕸️) are clickable to pin the graph
- Plain-text messages (📌) are clickable to pin them to the Markdown Preview
- Per-row copy button; command history navigation with ↑/↓ arrow keys
- **History-based autocomplete dropup** — as the user types, matching history entries appear in a dropup above the textarea; Tab or Enter accepts the highlighted suggestion; Escape dismisses it
- **Command palette** — a terminal-icon button opens a popover listing all known commands with descriptions; clicking a row inserts the command template into the textarea
- **Auto-grow textarea** — the input is a single auto-growing textarea; Shift+Enter inserts a newline, Enter sends; there is no separate multiline toggle mode

**Key code locations:**
- `src/components/Console/ConsoleMessage.tsx` [L37–L54](../src/components/Console/ConsoleMessage.tsx#L37) — per-message classification: reads from `classificationMap` to derive `isGraphLink`, `isLargePayload`, `isMockUpload`, `mockUploadPath`; `isPinnable` guard; `canSendToJsonPath`
- `src/components/Console/ConsoleMessage.tsx` [L98–L170](../src/components/Console/ConsoleMessage.tsx#L98) — JSX: `<JsonView>` for JSON, plain `<span>` for text, copy button, ➡️ send-to-JSON-Path button
- `src/components/Console/ConsoleMessage.tsx` [L42–L47](../src/components/Console/ConsoleMessage.tsx#L42) — `events` looked up from `classificationMap.get(msgId)`; `isGraphLink` / `isLargePayload` / `isMockUpload` derived via `events.some(e => e.kind === ...)` instead of direct parser calls
- `src/hooks/useWebSocket.ts` [L145–L167](../src/hooks/useWebSocket.ts#L145) — `sendCommand()`: sends text, pushes to history, handles special `load` command
- `src/hooks/useWebSocket.ts` [L168–L191](../src/hooks/useWebSocket.ts#L168) — `handleKeyDown()`: ↑/↓ history navigation with in-progress command save/restore (`ENTER_HISTORY` / `EXIT_HISTORY` / `SET_HISTORY_INDEX` reducer actions)
- `src/hooks/useWebSocket.ts` [L14–L32](../src/hooks/useWebSocket.ts#L14) — `LocalState` + `LocalAction` types; local command state saves in-progress input on first ↑ press
- `src/hooks/useHistoryAutocomplete.ts` — `useHistoryAutocomplete(history, command)`: pure filtering + dedup in a single `useMemo`; returns `suggestions`, `isOpen`, `activeIndex`, `navigate`, `accept`, `onTab`, `dismiss`, `onCommandChange`
- `src/components/CommandInput/CommandInput.tsx` [L5](../src/components/CommandInput/CommandInput.tsx#L5) — imports `useHistoryAutocomplete`; [L43–L71](../src/components/CommandInput/CommandInput.tsx#L43) — command palette open/close + outside-click handler; [L62](../src/components/CommandInput/CommandInput.tsx#L62) — auto-grow `useEffect` (always rows=1, height derived from `scrollHeight`); [L82–L175](../src/components/CommandInput/CommandInput.tsx#L82) — `handleKeyDown`: Tab → accept suggestion; Enter → accept or send; Escape → dismiss dropup; ↑/↓ → navigate dropup or history
- `src/utils/commandSuggestions.ts` — `QuickstartEntry` now has `template: string` (text inserted on palette click) and optional `multiline?: boolean` (informational metadata indicating a multi-line command)

---

### 3.4 Graph Visualisation
- Fetched via REST (`GET /api/graph/model/{graph_id}/{sequence}`) after the server emits a graph-link in the WebSocket stream
- Rendered with **ReactFlow**: nodes are colour-coded by type (`Root`, `End`, `Fetcher`, `mapper`, `Math`, `JavaScript`, `Provider`, `Dictionary`, `Join`, `Extension`, `Island`, `Decision`), edges show relation labels
- Nodes are **resizable** (NodeResizer) and can be re-arranged interactively
- A **minimap** provides navigation for large graphs
- A **refreshing overlay** (spinner) is displayed during background re-fetches without clearing the existing graph
- The **graph toolbar** displays a resolved graph name (root node name → save-name fallback → "Untitled"), with node/connection counts revealed on hover
- **Orphan node segregation** — nodes not participating in any connection are classified by type and placed in horizontal rows below the main flow: Dictionary → Provider → Module → Entity → unknown catch-all
- **Cycle detection & back-edge routing** — cycles in the graph are detected via iterative DFS; back-edges (edges pointing from a deeper level to a shallower one) are excluded from BFS level assignment so the layout doesn't loop infinitely. Back-edges are still rendered: they exit from the **left** side of the source node and enter the **right** side of the target node (the reverse of forward edges), producing a natural backward-arcing bezier curve. Handles on each side are sorted by peer y-position and interleaved (forward + back-edge) to prevent crossing

**Live-session REST shortcut**

The backend also exposes `GET /api/graph/session/{id}` for the active WebSocket session id. This returns the same exported graph structure directly from the in-memory session model, without issuing `describe graph`, without writing a temp file, and without depending on a graph-link message.

This endpoint is additive. The current frontend graph-loading flow still depends on the file-backed graph-link contract:
- the server emits `/api/graph/model/{graph_id}/{sequence}` from `describe graph`
- the frontend extracts only `/api/graph/model/...` links and pins that path for graph loading

So `/api/graph/session/{id}` is currently a backend shortcut for direct consumers, not a replacement for the existing WebSocket-driven graph refresh flow.

**Key code locations:**
- `src/utils/graphTypes.ts` [L1–L47](../src/utils/graphTypes.ts#L1) — `MinigraphGraphData`, `MinigraphNode`, `MinigraphConnection` types + `isMinigraphGraphData()` type guard
- `src/hooks/useGraphData.ts` [L7–L20](../src/hooks/useGraphData.ts#L7) — `normalizeRightTab()`: validates the persisted tab value against the current playground's `tabs` list and migrates stale entries (for example legacy `"preview"`) to a safe fallback before render
- `src/hooks/useGraphData.ts` [L66–L144](../src/hooks/useGraphData.ts#L66) — normalized right-tab state + initial-load path: reads `storedRightTab` from localStorage, derives a safe `rightTab`, writes the normalized value back when migration is needed, then `fetch(pinnedGraphPath)` → `setGraphData(json)` → `setRightTab('graph')`; clears `graphData` to `null` on path change
- `src/hooks/useGraphData.ts` [L149–L189](../src/hooks/useGraphData.ts#L149) — `refetchGraph()`: overlay-mode re-fetch; does NOT clear `graphData` (stale graph stays visible), sets `isRefreshing = true`
- `../../src/main/java/com/accenture/minigraph/services/GraphCommandService.java` [L234–L238](../../src/main/java/com/accenture/minigraph/services/GraphCommandService.java#L234) — `downloadGraph(id)`: resolves the public session id to the in-memory `inRoute` and returns `graph.exportGraph()`
- `../../src/main/java/com/accenture/minigraph/rest/GetLiveGraph.java` [L34–L47](../../src/main/java/com/accenture/minigraph/rest/GetLiveGraph.java#L34) — thin REST adapter for `GET /api/graph/session/{id}`
- `../../src/main/resources/rest.yaml` [L31–L43](../../src/main/resources/rest.yaml#L31) — REST route definitions for both the existing file-backed graph model endpoint and the new live-session endpoint
- `src/utils/graphTransformer.ts` — `NODE_ACCENT` colour map + `nodeStyle()` applying `--node-accent` CSS custom property
- `src/utils/graphTransformer.ts` — `classifyNode()`: classifies each node into a `LayoutCategory` — connected nodes always go to `'flow'`; orphaned nodes are segregated into `'Dictionary'`, `'Provider'`, `'Module'`, `'Entity'`, or `'__unknown__'`
- `src/utils/graphTransformer.ts` — `computeLayout()`: BFS topological layout with DFS cycle detection and segregated rows for orphan nodes; returns `{ positions, levelOf }`
- `src/utils/graphTransformer.ts` — `transformGraphData()`: converts `MinigraphGraphData` → ReactFlow `nodes[]` + `edges[]`; classifies edges as forward or backward using `levelOf`, builds per-side handle arrays sorted by peer y-position with forward and back-edge handles interleaved
- `src/components/GraphView/NodeTypes.tsx` [L9–L22](../src/components/GraphView/NodeTypes.tsx#L9) — `TYPE_META` icon/label map per node type
- `src/components/GraphView/NodeTypes.tsx` — `MinigraphNode`: renders as `<Fragment>` (no wrapper div); `NodeResizer` + forward target handles (left) + back-edge source handles (left) + content + forward source handles (right) + back-edge target handles (right) as siblings
- `src/components/GraphView/NodeTypes.tsx` — `nodeTypes` export map used by `<ReactFlow nodeTypes={nodeTypes}>`
- `src/components/GraphView/GraphView.tsx` — `<ReactFlow>` with `fitView`, `<Controls>`, `<MiniMap>`, and `isRefreshing` overlay; accepts `graphName` prop and threads it to `GraphToolbar`
- `src/components/GraphToolbar/GraphToolbar.tsx` — displays `graphName` prominently with hover-reveal node/connection stats; accepts `graphName?` prop
- `src/components/Playground.tsx` — `graphDisplayName` memo: resolves root node's `name` property → `graphSaveName` fallback; threaded via `RightPanel` → `GraphView` / `GraphDataView` → `GraphToolbar`

---

### 3.5 Auto-Graph Refresh
After any graph-mutating command (`create node`, `delete node`, `connect`, etc.), the graph automatically re-fetches and re-renders without user interaction. If no graph was previously loaded, the app issues `describe graph` silently, receives the graph-link, and opens the Graph tab automatically. This is debounced at 300 ms to collapse rapid-fire commands.

The hook subscribes to `graph.mutation`, `graph.link`, and `session.reset` events on the `ProtocolBus` — it no longer scans the raw message array directly.

Each mutation also emits a toast notification visible in the playground's toast stack:
- `import-graph` mutation → `'Graph imported — refreshing view…'` (immediate, no debounce)
- `node-mutation` with an existing graph → `'Graph updated — refreshing…'` (after debounce)
- `node-mutation` with no graph yet → `'Graph updated — opening Graph tab…'` (after debounce)

**Collaborative session gate-arm timing.** In a collaborative session the backend forwards the primary's `describe graph` response (as a `graph.link`) to all subscribers immediately after the primary processes it. This forwarded response can arrive at a subscriber's client during the 300 ms debounce window — before the subscriber would normally arm `waitingForDescribeRef`. To accept this forwarded link, `waitingForDescribeRef` is set to `true` at **mutation-detection time** (synchronously, before the debounce timer starts), not only inside the debounce callback. The assignment inside the callback is kept as a re-arm for the case where an early forwarded `graph.link` was already accepted and reset the gate to `false` before the debounce fired.

**Session reset handling.** When the user runs `session reset`, the backend sends `"Session restarted"`. The classifier emits a `session.reset` event (Rule 7c). The hook's `session.reset` handler cancels any in-flight debounce, resets `waitingForDescribeRef`, and calls `setPinnedGraphPath(null)`. Because `useGraphData` clears `graphData` when `pinnedGraphPath` becomes null, the GraphView is immediately cleared.

**Key code locations:**
- `src/utils/messageParser.ts` [L279–L306](../src/utils/messageParser.ts#L279) — `detectMutation()`: classifies a raw message as `'node-mutation'`, `'import-graph'`, or `null`; includes the critical `startsWith('node ')` prefix guard
- `src/protocol/classifier.ts` — Rule 7: calls `detectMutation()` and emits `graph.mutation` event; Rule 7c: emits `session.reset` for `"Session restarted"` via `SESSION_RESTARTED_MSG` constant
- `src/hooks/useAutoGraphRefresh.ts` [L32–L36](../src/hooks/useAutoGraphRefresh.ts#L32) — five key refs: `debounceTimerRef`, `waitingForDescribeRef`, `pinnedGraphPathRef`, `connectedRef`, `sendRawTextRef`
- `src/hooks/useAutoGraphRefresh.ts` [L44–L53](../src/hooks/useAutoGraphRefresh.ts#L44) — disconnect guard: clears `waitingForDescribeRef` and cancels any pending debounce on socket close
- `src/hooks/useAutoGraphRefresh.ts` [L55–L64](../src/hooks/useAutoGraphRefresh.ts#L55) — `bus.on('graph.link')`: consumes pending graph-link when `waitingForDescribeRef` is true → calls `setPinnedGraphPath(event.apiPath)`
- `src/hooks/useAutoGraphRefresh.ts` [L65–L110](../src/hooks/useAutoGraphRefresh.ts#L65) — `bus.on('graph.mutation')`: arms `waitingForDescribeRef` immediately at mutation-detection time; fires immediate `describe graph` for `import-graph`; debounced (300 ms) for `node-mutation`, with re-arm inside the timeout; `bus.on('session.reset')`: clears debounce, resets gate, calls `setPinnedGraphPath(null)`
- `src/hooks/useWebSocket.ts` [L286](../src/hooks/useWebSocket.ts#L286) — `sendRawText()`: sends without echoing to console or history (used exclusively by automation hooks)

---

### 3.6 Bundled Help Panel (Developer Guides)
Help content (`help.md`, `help create.md`, `help tutorial 1.md`, …) is bundled into the webapp at build time via `import.meta.glob`. When the user types `help` or `help <topic>` while connected, the command is **handled locally** by `handleLocalCommand` in `useWebSocket` — no WebSocket round-trip occurs. The console receives a local echo (`> help …`) that is classified identically to a server-echoed command, so `useAutoHelpNavigate` opens the help panel automatically.

Access while disconnected: the header `?` button and the `Ctrl + \`` hotkey always open the help panel regardless of connection state.

`describe …` responses remain **console-driven**; they are not captured by the help panel.

**Key code locations:**
- `src/data/helpContent.ts` — `getHelpContent(topic)`: looks up the bundled markdown string; `HELP_TOPIC_KEYS`: ordered list of all valid topic keys
- `src/utils/helpTopic.ts` — `extractHelpTopic(commandText)`: strips `"help "` prefix → bare topic key (lowercased)
- `src/utils/localHelpCommand.ts` — `resolveBundledHelpTopic(commandText, supportsHelp)`: returns the topic key when the command should be handled locally, or `null` when it should go to the backend
- `src/hooks/useWebSocket.ts` — `handleLocalCommand` option: called inside `sendCommand` before the remote-send path; return `true` to intercept
- `src/hooks/useAutoHelpNavigate.ts` — subscribes to `command.helpOrDescribe` on the bus; opens the help panel from server-echoed **and** locally-appended echoes (both classify identically)
- `src/components/HelpBrowser/HelpBrowser.tsx` — the rendered help panel component; `activeTopic` + `onNavigate` props drive the current topic view

---

### 3.7 Large Payload Handling
When the server reports a payload exceeding 64 KB (`"Large payload (N) -> GET /api/inspect/…"`), the hook fetches it via REST and appends the result directly to the console as a collapsible JSON row — identical in appearance to small payloads.

The hook subscribes to `payload.large` events on the `ProtocolBus`. An `isFetchingRef` re-entrancy guard prevents concurrent fetches.

**Key code locations:**
- `src/utils/messageParser.ts` [L148–L168](../src/utils/messageParser.ts#L148) — `extractLargePayloadLink()`: regex parses byte size and API path from the server notification; `isLargePayloadMessage()` ([L167](../src/utils/messageParser.ts#L167)) is the boolean predicate
- `src/protocol/classifier.ts` [L65–L77](../src/protocol/classifier.ts#L65) — Rule 3: calls `extractLargePayloadLink()` and emits `payload.large` event
- `src/hooks/useLargePayloadDownload.ts` [L24–L25](../src/hooks/useLargePayloadDownload.ts#L24) — two key refs: `abortRef` (AbortController for in-flight cancellation), `isFetchingRef` (re-entrancy guard)
- `src/hooks/useLargePayloadDownload.ts` [L50–L87](../src/hooks/useLargePayloadDownload.ts#L50) — `bus.on('payload.large')`: guards with `isFetchingRef`, fetches the endpoint, pretty-prints JSON, calls `appendMessage(content)`
- `src/config/playgrounds.ts` [L27](../src/config/playgrounds.ts#L27) — `MAX_BUFFER = 63_488` — the 62 KB send limit that makes large-payload handling necessary

---

### 3.8 JSON-Path ↔ Minigraph Cross-Playground Routing
A JSON response in the Minigraph console can be sent directly to the JSON-Path Playground payload editor with one click (➡️ button), navigating automatically and pre-filling the editor.

There are three branches depending on the JSON-Path slot phase:

1. **`'connected'`** — deposit `json` into the context via `ctx.setPendingPayload(wsPath, json)` and `navigate(jsonPathConfig.path)` immediately.
2. **`'connecting'`** — overwrite `pendingJsonTransferRef` with the new payload; do **not** call `ctx.connect()` again (already in progress); toast `'Updated pending JSON transfer — latest payload will open when connected'`. Last-write-wins.
3. **`'idle'`** — arm `pendingJsonTransferRef`, call `ctx.connect(jsonPathConfig.wsPath, addToast)`, and toast `'Connecting to JSON-Path Playground…'`. A `useEffect` in `useSendToJsonPath` watches the JSON-Path slot phase; once it reaches `'connected'`, the deposit + navigate executes and `pendingJsonTransferRef` is cleared.

**Key code locations:**
- `src/components/Console/ConsoleMessage.tsx` — `canSendToJsonPath = !!onSendToJsonPath && jsonCheck.isJSON` — button only shown on JSON messages when the callback is wired
- `src/hooks/useSendToJsonPath.ts` — `handleSendToJsonPathInner`: three-branch phase switch; `pendingJsonTransferRef` is the single-slot last-write-wins mailbox
- `src/contexts/WebSocketContext.tsx` — `setPendingPayload` / `takePendingPayload` interface; backed by `useState` (not `useRef`) so depositing triggers a re-render in the consuming playground
- `src/components/Playground.tsx` — receiving side: `payloadOverride` state initialised from `ctx.takePendingPayload(wsPath)` at mount

---

### 3.9 Two-Step Payload Upload (REST Handshake)
The JSON-Path Playground supports uploading large JSON payloads over REST:
1. The user clicks **Upload** → the hook sends `"upload"` over the WebSocket
2. The server responds with `"Please upload XML/JSON text to /api/json/content/{id}"`
3. The hook detects that URL via `upload.contentPath` event on the ProtocolBus (or legacy message scanning when bus is not provided) and fires `POST /api/json/content/{id}` with the payload

**Key code locations:**
- `src/utils/messageParser.ts` [L118–L127](../src/utils/messageParser.ts#L118) — `extractUploadPath()`: regex extracts `/api/json/content/{id}` from the server's reply
- `src/protocol/classifier.ts` [L93–L102](../src/protocol/classifier.ts#L93) — Rule 5: calls `extractUploadPath()` and emits `upload.contentPath` event
- `src/hooks/useWebSocket.ts` [L125](../src/hooks/useWebSocket.ts#L125) — `pendingUploadRef = useRef(false)` — arms the two-step handshake
- `src/hooks/useWebSocket.ts` [L272–L283](../src/hooks/useWebSocket.ts#L272) — `uploadPayload()`: sets `pendingUploadRef.current = true`, sends `"upload"` over the socket
- `src/hooks/useWebSocket.ts` [L194–L228](../src/hooks/useWebSocket.ts#L194) — bus-based `upload.contentPath` subscription: when `pendingUploadRef` is true, clears flag synchronously before `fetch()`, fires `POST` with the payload
- `src/hooks/useWebSocket.ts` [L230–L269](../src/hooks/useWebSocket.ts#L230) — legacy messages-watch fallback (active when bus is NOT provided): checks `extractUploadPath` on last message

---

### 3.10 Saved Graph Bookmarks
The Minigraph playground lets users save a named graph snapshot. The name is stored in localStorage and sent to the server via `export graph as {name}`. Loading re-issues `import graph from {name}`, and the auto-refresh hook re-renders the graph.

**Save flow (connection required):** The save button is disabled when disconnected — `GraphSaveButton` enforces `disabled={disabled || !connected}`. When connected, `handleSaveGraph(name)` in `useSavedGraphWorkflow` does **not** write to localStorage immediately. Instead it arms `pendingSaveRef.current = { graphName: name, timeoutId }` and sends `export graph as {name}` over the WebSocket. Four resolution paths:
- `graph.exported` event with matching `graphName` → `savedGraphs.saveGraph(name)` + `setLastSavedName(name)` + success toast (server-side file is guaranteed to exist for future import)
- `graph.export.failed` event → typed error toast (invalid filename or root-name mismatch); no bookmark written
- 10-second timeout → error toast; no bookmark written
- Disconnect while in-flight → pending ref cleared + failure toast; no bookmark written

**Key code locations:**
- `src/hooks/useSavedGraphs.ts` — `UseSavedGraphsReturn` interface: `savedGraphs`, `saveGraph`, `deleteGraph`, `hasGraph`; implementation uses `useLocalStorage<SavedGraphsMap>` backing store; `useMemo` sorted newest-first
- `src/hooks/useSavedGraphWorkflow.ts` — `handleSaveGraph(name)`: guards against disconnected calls (error toast + early return); when connected, arms `pendingSaveRef.current = { graphName, timeoutId }` and sends `export graph as ${name}`; four `useEffect`-based resolution paths: `graph.exported` (name-matched) → bookmark + success; `graph.export.failed` → typed error; 10 s timeout → error; disconnect → error
- `src/hooks/useSavedGraphWorkflow.ts` — `handleLoadGraph(name)`: guards `!connected`; sends `import graph from ${name}` — auto-refresh hook takes it from there
- `src/components/GraphSaveButton/GraphSaveButton.tsx` — self-contained inline save form (open/close, pre-filled name, overwrite warning, Enter/Escape keyboard handling); `disabled={disabled || !connected}` prevents the form from opening while disconnected
- `src/components/SavedGraphsMenu/SavedGraphsMenu.tsx` — dropdown list reusing `NavMenu`; Load/Delete actions per entry

---

### 3.11 Keep-Alive Pings
Every 20 seconds the client sends `{"type":"ping","message":"keep alive"}` over each open socket. Pong responses and outgoing pings are silently filtered — they never appear in the console.

**Key code locations:**
- `src/config/playgrounds.ts` [L30](../src/config/playgrounds.ts#L30) — `PING_INTERVAL = 20_000` — the only place to change the frequency
- `src/contexts/WebSocketContext.tsx` [L227–L231](../src/contexts/WebSocketContext.tsx#L227) — `setInterval` started in `ws.onopen`; fires `ws.send(eventWithTimestamp('ping', 'keep alive'))`
- `src/contexts/WebSocketContext.tsx` [L192–L205](../src/contexts/WebSocketContext.tsx#L192) — `isKeepAliveMessage()`: checks `parsed.type === 'ping' || 'pong'`; messages passing this check are dropped before `dispatch(MESSAGE_RECEIVED)` ([L240](../src/contexts/WebSocketContext.tsx#L240))

---

### 3.12 Mock-Data Upload Modal
When the server responds to a command (e.g. `upload mock data`) with the message `"You may upload JSON payload -> POST /api/mock/{id}"`, the app automatically opens a modal dialog. The user can paste JSON directly, drag-and-drop a `.json` file onto a drop zone, or click "Browse file…" to open the system file-picker. Submitting POSTs the payload to the provided URL.

The console row for the invitation displays a ⬆️ icon and an "⬆️ Upload JSON…" re-open button so the modal can be recalled after it has been dismissed. A ✅ badge appears on the row after a successful upload (session-only).

**Two triggers, one modal:** The modal is opened either (a) automatically by `useAutoMockUpload` when the server message arrives, or (b) manually via the re-open button on the console row. Both paths call the same `handleOpenUploadModal` callback in `Playground.tsx`.

**Key code locations:**
- `src/utils/messageParser.ts` [L182–L193](../src/utils/messageParser.ts#L182) — `extractMockUploadPath()` / `isMockUploadMessage()`: regex extracts `/api/mock/{id}` from the server's invitation message
- `src/hooks/useAutoMockUpload.ts` — subscribes to `bus.on('upload.invitation')`; `pendingModalRef` ([L22](../src/hooks/useAutoMockUpload.ts#L22)) prevents duplicate modal opens per batch; a `modalOpen: boolean` prop clears `pendingModalRef` via a separate effect ([L26](../src/hooks/useAutoMockUpload.ts#L26)) when the modal closes, so the next invitation can trigger normally
- `src/hooks/useMockUpload.ts` — owns the `fetch` lifecycle: `AbortController` per attempt, `isUploading` state, `upload()` / `cancel()` functions
- `src/components/MockUploadModal/MockUploadModal.tsx` — native `<dialog>` with `.showModal()`; textarea + drop zone + "Browse file…" button; `Ctrl+Enter` / `⌘+Enter` submit shortcut; inline JSON validation via `tryParseJSON`
- `src/components/MockUploadModal/MockUploadModal.module.css` — amber accent theming; drop zone active state; spinner; `--warning-color` token for file/validation errors
- `src/components/Playground.tsx` [L135–L144](../src/components/Playground.tsx#L135) — `modalUploadPath` / `modalTriggerRef` / `successfulUploadPaths` state
- `src/components/Playground.tsx` [L185–L215](../src/components/Playground.tsx#L185) — `handleOpenUploadModal`, `handleCloseUploadModal`, `handleUploadSuccess`, `handleUploadError` callbacks + `useAutoMockUpload` invocation
- `src/components/Console/ConsoleMessage.tsx` [L41–L50](../src/components/Console/ConsoleMessage.tsx#L41) — `isMockUpload`, `mockUploadPath`, `canUploadMock`, `uploadSucceeded` derived flags; `isPinnable` guard (`&& !isMockUpload`) prevents a nested-interactive-element accessibility violation
- `src/hooks/useAutoHelpNavigate.ts` — subscribes to `command.helpOrDescribe`; the `isMockUploadMessage` guard in `ConsoleMessage` ensures upload invitation rows are never mistaken for help-command echoes

---

### 3.13 Save-Name Management
When the user opens the **💾 Save Graph** inline form, the input is pre-filled according to a strict priority chain:

| Priority | Source | Condition |
|---|---|---|
| 1 | `lastSavedName` | The working graph was previously saved this session |
| 2 | `importedName` | The graph was loaded via `import graph from {name}` |
| 3 | `untitled-{n}` | Fallback — monotonically incrementing per-playground counter |

A new import always supersedes a previous save: when a new `import graph from {name}` echo is detected in the message stream, `lastSavedName` is cleared to `null` immediately, ensuring the imported name wins on the next form open.

**Untitled counter semantics.** The counter only advances when the current `untitled-{n}` slot has actually been *used as a save name*. Clearing the console without ever saving reuses the same slot — so `untitled-2` will never appear in storage unless `untitled-1` was saved first. The counter is persisted in localStorage (keyed per playground) so it survives page refreshes.

**Key code locations:**
- `src/hooks/useGraphSaveName.ts` [L9–L37](../src/hooks/useGraphSaveName.ts#L9) — `UseGraphSaveNameReturn` interface with documented priority order and counter invariant
- `src/hooks/useGraphSaveName.ts` [L75](../src/hooks/useGraphSaveName.ts#L75) — `useLocalStorage<number>(storageKey, 1)` — counter persisted, starts at 1
- `src/hooks/useGraphSaveName.ts` [L82](../src/hooks/useGraphSaveName.ts#L82) — `untitledSlotConsumedRef = useRef(false)` — tracks whether the current slot has been used; stored as a ref (not state) because it never drives a re-render
- `src/hooks/useGraphSaveName.ts` [L99–L103](../src/hooks/useGraphSaveName.ts#L99) — import-echo subscription: `bus.on('command.importGraph')` clears `lastSavedName` to `null` whenever a new `import graph from {name}` echo arrives, so the imported name takes over immediately
- `src/hooks/useGraphSaveName.ts` [L107–L115](../src/hooks/useGraphSaveName.ts#L107) — `setLastSavedName(name)`: sets the saved name and marks `untitledSlotConsumedRef = true` only when the name equals the current `untitled-{n}` fallback
- `src/hooks/useGraphSaveName.ts` [L117–L127](../src/hooks/useGraphSaveName.ts#L117) — `resetName()`: clears both names; advances counter only when `untitledSlotConsumedRef.current` is `true`, then resets the flag
- `src/hooks/useGraphSaveName.ts` [L130–L133](../src/hooks/useGraphSaveName.ts#L130) — derived `defaultName`: `lastSavedName ?? importedName ?? \`untitled-${untitledCounter}\``
- `src/components/Playground.tsx` [L234–L236](../src/components/Playground.tsx#L234) — hook instantiated with `bus` instead of `ws.messages`; key `\`${storageKeySavedGraphs}-untitled-counter\`` keeps the counter isolated per playground
- `src/utils/messageParser.ts` [L243–L254](../src/utils/messageParser.ts#L243) — `extractImportGraphName()`: parses `> import graph from {name}` echoes; returns the trimmed name or `null`

---

### 3.14 Protocol Kernel — Centralised Message Classification

The Protocol Kernel is a classify-once, emit-to-many layer that sits between the raw WebSocket message stream and all consuming hooks/components. Instead of each automation hook scanning every message independently (duplicated regex work, duplicated watermark bookkeeping), a single `useProtocolKernel` hook:

1. **Maintains one watermark** for the entire message log
2. **Classifies each new message once** via the pure `classifyMessage()` function
3. **Emits typed events** to a `ProtocolBus` — hooks subscribe to only the event kinds they care about
4. **Builds a `classificationMap`** (`Map<number, ProtocolEvent[]>`) keyed by message ID — components read pre-computed classifications instead of re-parsing at render time

This architecture eliminates O(hooks × messages) repeated parsing and provides a single extension point (the classifier's rule list) for new message types.

**Modules:**

| File | Purpose | Lines |
|---|---|---|
| `src/protocol/events.ts` | Discriminated union type (`ProtocolEvent`) for graph, upload, docs, create-node, lifecycle, and session-reset messages | ~151 |
| `src/protocol/classifier.ts` | Pure `classifyMessage(msgId, raw)` → `ProtocolEvent[]` rule pipeline | ~243 |
| `src/protocol/bus.ts` | `ProtocolBus` — lightweight typed event emitter (~45 lines) | 45 |
| `src/protocol/useProtocolKernel.ts` | React hook: watermark + classify + memoised map + bus emission | 73 |
| `src/protocol/index.ts` | Barrel re-export | 18 |

**Event kinds:** `graph.link`, `graph.mutation`, `minigraph.createNode.textResult`, `graph.exported`, `graph.export.failed`, `payload.large`, `upload.invitation`, `upload.contentPath`, `command.echo`, `command.helpOrDescribe`, `command.importGraph`, `docs.response`, `json.response`, `lifecycle`, `session.reset`, `unclassified`.

**Key code locations:**
- `src/protocol/events.ts` [L103](../src/protocol/events.ts#L103) — `export type ProtocolEvent` discriminated union
- `src/protocol/classifier.ts` [L27](../src/protocol/classifier.ts#L27) — `export function classifyMessage(msgId, raw)`: returns an array (one message may match multiple rules, e.g. a create-node success is both `graph.mutation` and `minigraph.createNode.textResult`)
- `src/protocol/bus.ts` [L16](../src/protocol/bus.ts#L16) — `export class ProtocolBus`: `on(kind, handler)` returns an unsubscribe function; `emit(event)` fires all registered handlers for that `event.kind`; `clear()` removes all listeners
- `src/protocol/useProtocolKernel.ts` [L33](../src/protocol/useProtocolKernel.ts#L33) — hook entry; [L40](../src/protocol/useProtocolKernel.ts#L40) — watermark init effect; [L47](../src/protocol/useProtocolKernel.ts#L47) — `classificationMap = useMemo(...)` over the full messages array; [L56](../src/protocol/useProtocolKernel.ts#L56) — watermark-gated bus emission effect
- `src/components/Playground.tsx` [L88](../src/components/Playground.tsx#L88) — `busRef = useRef(new ProtocolBus())`; [L94](../src/components/Playground.tsx#L94) — `useProtocolKernel({ messages, bus })`

**Test infrastructure:**
- `src/protocol/__tests__/classifier.test.ts` — golden transcript tests using fixture JSON files
- `src/protocol/__tests__/bus.test.ts` — ProtocolBus unit tests
- `src/protocol/__tests__/fixtures/*.json` — 12 fixture files covering all event kinds, including `multi-event.json` (tests a single message matching multiple rules simultaneously) and `negative-cases.json` (tests inputs that must NOT produce specific event kinds); not a strict one-per-event-kind mapping
- `vitest.config.ts` — Vitest 3 + built-in `node` environment; `globals: true`

---

### 3.15 Clipboard Panel
The Minigraph playground includes a resizable **Clipboard** sidebar (enabled when `supportsClipboard: true` in the playground config). It allows engineers to clip individual nodes from the current graph and later paste their create/update command templates into the console input.

**Clipboard features:**
- Right-click any node in the graph → "Clip to Clipboard" menu item
- Items are persisted in **IndexedDB** and survive page reloads
- **BroadcastChannel** keeps all open browser tabs in sync automatically
- Duplicate detection: if an item with the same alias already exists, a `ClipboardDuplicateDialog` asks whether to replace it
- **Paste** button on each clipboard item builds a `create`/`update` command from the node data and inserts it into the console's command input; whether to create or update is determined by checking the current graph for an existing node with the same alias

**UI entry points:**
- A **Clipboard** toggle button appears in the playground header when `supportsClipboard` is true; it shows the item count and persists its open/closed state in localStorage
- The sidebar is a third resizable panel in the `react-resizable-panels` `<Group>` — only mounted when `clipboardOpen === true`

**Key code locations:**
- `src/contexts/ClipboardContext.tsx` — context + `useReducer` + IndexedDB hydration + BroadcastChannel sync; `ClipboardProvider` lives at the app level (wraps `BrowserRouter`)
- `src/clipboard/db.ts` — IndexedDB schema + CRUD helpers
- `src/clipboard/channel.ts` — `createClipboardChannel()`: wraps `BroadcastChannel` for cross-tab sync
- `src/clipboard/commandBuilder.ts` — `buildNodeCommand(verb, node)`: builds the command string from a clipped node
- `src/components/ClipboardSidebar/ClipboardSidebar.tsx` — sidebar container: item list + paste buttons + empty state
- `src/components/ClipboardSidebar/ClipboardDuplicateDialog.tsx` — dialog for resolving duplicate-alias clips
- `src/components/Playground.tsx` — `handleClipNode` / `handlePasteToInput` callbacks; `clipboardOpen` via `useLocalStorage`; `duplicateDialogState` via `useState`

---

### 3.16 UI Node Authoring (Create Node)
The Minigraph playground exposes a UI path for creating nodes without requiring the user to hand-write the `create node ...` command. The feature is enabled by `supportsAuthoring: true` in the Minigraph playground config and is not mounted for JSON-Path.

**Entry points:**
- **Empty graph state** — when no graph data is loaded or the loaded graph has no nodes, the Graph tab shows a "Create Node" button. This opens the modal with `alias = root` and `nodeType = Root` as editable starting values.
- **Existing graph pane** — right-clicking empty space in the ReactFlow pane opens a pointer-positioned context menu with "Create Node". This opens the same modal with blank alias/type fields.

**Authoring model:**
- The modal edits create-node form state: `alias`, optional `nodeType`, flat single-line property rows, and the source entry point.
- Frontend validation checks the supported authoring surface before any raw text is sent: alias required, token format, reserved aliases, duplicate alias against the current graph snapshot, property key requirements, single-line property values, and final command size.
- On submit, `buildCreateNodeCommand()` serialises the form values into the backend's existing multiline command grammar:

```text
create node {alias}
with type {nodeType}
with properties
key=value
```

There is deliberately no JSON command payload. The command sent over WebSocket is raw text, identical in shape to a console-entered command.

**Result handling:**
- The modal does not optimistically mutate `graphData`.
- `useGraphAuthoring` sends through `GraphAuthoringExecutor`, which delegates to `useWebSocket.sendRawText()`. This avoids adding the command to console history or echoing an extra local command row.
- The Protocol Kernel classifies backend text results into `minigraph.createNode.textResult` events using `parseCreateNodeTextResult()`.
- Accepted result (`"node {alias} created"`) closes the modal. Rejected duplicate result (`"node {alias} already exists"`) and generic `ERROR: ...` responses keep the modal open and show the backend message.
- A 10-second timeout returns the modal to editing with an "outcome unknown" message; disconnect while open locks the fields and asks the user to refresh/check the graph before retrying.

**Key code locations:**
- `src/config/playgrounds.ts` — `supportsAuthoring: true` enables the feature for Minigraph only
- `src/components/Playground.tsx` — creates `GraphAuthoringExecutor`, owns `useGraphAuthoring`, renders `GraphAuthoringModals`, and threads `onCreateNode` to `RightPanel`
- `src/components/RightPanel/RightPanel.tsx` — forwards `supportsAuthoring`, `isConnected`, and `onCreateNode` to `GraphView`
- `src/components/GraphView/GraphView.tsx` — renders the empty-state create button and the pane context-menu trigger
- `src/components/GraphView/GraphContextMenu.tsx` — pointer-positioned menu for pane-level graph actions
- `src/components/GraphAuthoring/useGraphAuthoring.ts` — create-node lifecycle state machine, validation invocation, submit timer, ProtocolBus result correlation, and disconnect handling
- `src/components/GraphAuthoring/GraphAuthoringModals.tsx` — maps hook state to modal props
- `src/components/NodeDialog/NodeDialog.tsx` — presentational modal for editing the create-node form
- `src/graphActions/*` — pure form types, row helpers, validation, command builder, and executor adapter
- `src/utils/messageParser.ts` — `parseCreateNodeTextResult()`
- `src/protocol/classifier.ts` / `src/protocol/events.ts` — `minigraph.createNode.textResult` event

---

## 4. Repository Layout

```
webapp/
├── src/
│   ├── main.tsx                  # React root mount
│   ├── App.tsx                   # Router + WebSocketProvider + ClipboardProvider bootstrap
│   ├── index.css                 # Global resets / CSS variables
│   ├── config/
│   │   └── playgrounds.ts        # ★ SINGLE source of truth for all playgrounds
│   ├── hooks/
│   │   ├── useWebSocket.ts       # Per-playground WS + command input logic (handleLocalCommand extension)
│   │   ├── useGraphData.ts       # REST fetch + graph state management
│   │   ├── useAutoGraphRefresh.ts# Mutation detection → auto re-fetch (bus-based)
│   │   ├── useAutoHelpNavigate.ts# help/describe echo → auto-open help panel (bus-based)
│   │   ├── useLargePayloadDownload.ts # Large payload REST fetch → console (bus-based)
│   │   ├── useAutoMockUpload.ts  # Mock-upload invitation → auto-open modal (bus-based)
│   │   ├── useMockUpload.ts      # POST fetch lifecycle for mock-upload modal
│   │   ├── useSavedGraphs.ts     # localStorage graph bookmark CRUD
│   │   ├── useSavedGraphWorkflow.ts   # Save/load workflow — export-domain protocol events
│   │   ├── useGraphSaveName.ts   # Save-form pre-fill name (bus-based; priority: saved > imported > untitled-n)
│   │   ├── usePinnedGraphPath.ts # Session-bound graph API path (module-scope Map)
│   │   ├── useSendToJsonPath.ts  # Cross-playground JSON transfer (last-write-wins mailbox)
│   │   ├── useLocalStorage.ts    # Generic localStorage hook
│   │   ├── useToast.ts           # Toast notification queue
│   │   ├── useCopyToClipboard.ts # Clipboard write with copied state
│   │   ├── useMediaQuery.ts      # Responsive breakpoint detection
│   │   ├── useHistoryAutocomplete.ts  # History-based autocomplete for CommandInput dropup
│   │   └── useAutocomplete.ts    # Template-based autocomplete (dormant — not currently wired to CommandInput)
│   ├── protocol/                  # ★ Protocol Kernel — centralised message classification
│   │   ├── events.ts             # ProtocolEvent discriminated union (incl. create-node, graph export, upload events)
│   │   ├── classifier.ts         # classifyMessage() pure function
│   │   ├── bus.ts                # ProtocolBus typed event emitter
│   │   ├── useProtocolKernel.ts  # React hook: watermark + classify + map + emit
│   │   ├── index.ts              # Barrel re-export
│   │   └── __tests__/
│   │       ├── classifier.test.ts  # Golden transcript tests
│   │       ├── bus.test.ts         # Bus unit tests
│   │       └── fixtures/           # 12 JSON fixture files covering all event kinds
│   ├── graphActions/              # ★ Create-node authoring helpers (form state → validation → raw command)
│   │   ├── graphAuthoringExecutor.ts # Adapter boundary over useWebSocket.sendRawText()
│   │   ├── minigraphCommandBuilder.ts# Serialises form state to multiline create-node command text
│   │   ├── nodeAuthoringTypes.ts  # Create-node form / PropertyRow / source types
│   │   ├── propertyRows.ts        # Default form values + property row id helpers
│   │   ├── validation.ts          # Frontend validation for supported authoring surface
│   │   └── __tests__/
│   ├── clipboard/                 # ★ Clipboard module (IndexedDB + BroadcastChannel)
│   │   ├── db.ts                 # IndexedDB schema + CRUD helpers
│   │   ├── channel.ts            # BroadcastChannel wrapper for cross-tab sync
│   │   ├── commandBuilder.ts     # Builds create/update command strings from clipped nodes
│   │   ├── helpers.ts            # Shared clipboard utilities
│   │   └── __tests__/
│   ├── contexts/
│   │   ├── WebSocketContext.tsx  # Shared multi-socket state (above Routes)
│   │   └── ClipboardContext.tsx  # Clipboard items — IndexedDB-backed, cross-tab synced
│   ├── components/
│   │   ├── Playground.tsx        # ★ Top-level orchestrator per route
│   │   ├── Navigation.tsx        # Header nav bar (Tools + Quick Links menus)
│   │   ├── Toast.tsx             # Toast container + item
│   │   ├── LeftPanel/            # Console + CommandInput wrapper
│   │   ├── RightPanel/           # Tab switcher (payload/preview/graph/graph-data)
│   │   ├── Console/              # Message list + ConsoleMessage renderer
│   │   ├── CommandInput/         # Text input + send button
│   │   ├── GraphAuthoring/       # Create-node lifecycle hook wrapper + modal mount
│   │   ├── GraphView/            # ReactFlow canvas + NodeTypes + context menus + ErrorBoundary
│   │   ├── GraphDataView/        # Raw JSON viewer for graph model
│   │   ├── GraphToolbar/         # Graph name display + copy toolbar for graph panel
│   │   ├── GraphSaveButton/      # Inline save-form button in header
│   │   ├── SavedGraphsMenu/      # Dropdown list of saved graph bookmarks
│   │   ├── MockUploadModal/      # Modal dialog for mock-data JSON upload
│   │   ├── NodeDialog/           # Create-node modal (presentational form editor)
│   │   ├── ClipboardSidebar/     # Resizable clipboard panel (ClipboardSidebar + ClipboardItem + ClipboardDuplicateDialog)
│   │   ├── HelpBrowser/          # Bundled help panel renderer (react-markdown)
│   │   ├── PayloadEditor/        # Textarea + validation + SampleButtons
│   │   └── NavMenu/              # Generic accessible dropdown menu
│   ├── icons/
│   │   ├── CloseIcon.svg         # Local SVG imported via ?react for close/delete controls
│   │   └── TerminalPromptIcon.svg
│   └── utils/
│       ├── messageParser.ts      # ★ Message classification & pattern matching (incl. create-node text results)
│       ├── localHelpCommand.ts   # resolveBundledHelpTopic — pure local-help resolver
│       ├── graphTransformer.ts   # Backend JSON → ReactFlow nodes + edges (BFS layout, cycle detection, orphan segregation, back-edge routing)
│       ├── graphTypes.ts         # TypeScript interfaces + type guard
│       ├── validators.ts         # JSON/XML payload validation + formatting
│       ├── urls.ts               # WebSocket & HTTP URL construction
│       └── commandSuggestions.ts # Autocomplete command list
├── docs/
│   ├── Technical Documentation.md
│   ├── SPEC-protocol-kernel.md
│   ├── SPEC-auto-graph-refresh.md
│   ├── SPEC-large-payload-inline.md
│   └── SPEC-mock-data-upload-modal.md
├── vitest.config.ts              # Vitest 3 + node environment test configuration
├── vite.config.ts
├── tsconfig.json
└── package.json
```

---

## 5. Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                      App.tsx                        │
│  ┌──────────────────────────────────────────────┐  │
│  │           WebSocketProvider                  │  │
│  │   (lives above Routes — sockets persist)     │  │
│  │  ┌────────────────────────────────────────┐  │  │
│  │  │         ClipboardProvider              │  │  │
│  │  │  (app-level — IndexedDB + cross-tab)   │  │  │
│  │  │  ┌──────────────────────────────────┐  │  │  │
│  │  │  │         BrowserRouter            │  │  │  │
│  │  │  │  Route /json-path → Playground A │  │  │  │
│  │  │  │  Route /         → Playground B  │  │  │  │
│  │  │  └──────────────────────────────────┘  │  │  │
│  │  └────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘

Each Playground instance:

  Playground.tsx (orchestrator)
  ├── useWebSocket          ← command input + history + WS actions (handleLocalCommand for local help)
  ├── useProtocolKernel     ← classifies messages once → ProtocolBus + classificationMap
  ├── useGraphData          ← REST fetch + graph state
  ├── useAutoGraphRefresh   ← bus: graph.link + graph.mutation + session.reset → silent describe graph + toast; clears graph on reset
  ├── useAutoHelpNavigate   ← bus: command.helpOrDescribe → auto-open help panel
  ├── useSavedGraphWorkflow ← bus: graph.exported / graph.export.failed → export-domain save correlation
  ├── useLargePayloadDownload ← bus: payload.large → REST fetch → console (inline)
  ├── useAutoMockUpload     ← bus: upload.invitation → auto-open modal
  ├── useSavedGraphs        ← localStorage bookmark CRUD
  ├── useGraphSaveName      ← bus: command.importGraph → save-form pre-fill name
  ├── useClipboardContext   ← IndexedDB items + clipNode + confirmReplace
  ├── useGraphAuthoring     ← create-node modal lifecycle + raw command submit + result correlation
  │
  ├── LeftPanel
  │   ├── Console           ← message list (ConsoleMessage per row)
  │   │                       classificationMap threaded down for render-time lookups
  │   └── CommandInput      ← textarea + send button + history nav
  │
  ├── RightPanel (tabbed: payload / graph / graph-data; receives graphName)
  │   ├── PayloadEditor     ← textarea + validation (JSON-Path only)
  │   ├── GraphView         ← ReactFlow canvas (receives graphName; clip/create-node context actions)
  │   ├── GraphDataView     ← raw JSON tree of graph model (receives graphName)
  │   └── HelpBrowser       ← bundled help panel (resizable overlay — not a tab)
  │
  ├── MockUploadModal       ← native <dialog>; JSON paste / drop / browse; POST
  ├── GraphAuthoringModals  ← create-node modal mount (NodeDialog)
  │
  └── ClipboardSidebar      ← conditional third panel (only when supportsClipboard && clipboardOpen)

  Message flow:
    WebSocket → messages[] → useProtocolKernel
                              ├── classificationMap (Map<msgId, ProtocolEvent[]>)
                              │     └── ConsoleMessage reads at render time
                              └── ProtocolBus.emit(event)
                                    ├── useAutoGraphRefresh.on('graph.link' | 'graph.mutation' | 'session.reset')
                                    ├── useAutoHelpNavigate.on('command.helpOrDescribe')
                                    ├── useSavedGraphWorkflow.on('graph.exported' | 'graph.export.failed')
                                    ├── useLargePayloadDownload.on('payload.large')
                                    ├── useAutoMockUpload.on('upload.invitation')
                                    ├── useGraphSaveName.on('command.importGraph')
                                    ├── useGraphAuthoring.on('minigraph.createNode.textResult')
                                    └── useWebSocket.on('upload.contentPath')  [when bus provided]
```

**Key design principle:** State flows top-down through props; side-effects are encapsulated in hooks; components are dumb renderers. `Playground.tsx` is the only component that coordinates between hooks.

---

## 6. Layer-by-Layer Walkthrough

### 6.1 Entry Point & Routing

**`src/main.tsx`** — Standard React 19 root mount. Wraps `<App>` in `<StrictMode>`.

**`src/App.tsx`** — Two responsibilities:
1. Wraps the entire tree in `<WebSocketProvider>` *outside* `<BrowserRouter>` so connections survive route changes. `<ClipboardProvider>` is also at this level, between `WebSocketProvider` and `BrowserRouter`.
2. Generates one `<Route>` per entry in `PLAYGROUND_CONFIGS` — adding a playground requires only a config entry, never a code change here.

A catch-all `<Route path="*">` redirects unknown paths to `PLAYGROUND_CONFIGS[0].path`. Currently JSON-Path (`/json-path`) is index 0 and therefore the default route; Minigraph (`/`) is index 1.

---

### 6.2 Playground Configuration

**`src/config/playgrounds.ts`** — The single source of truth for everything that varies per playground:

```typescript
interface PlaygroundConfig {
  path:                  string;   // URL route, e.g. "/json-path"
  label:                 string;   // Nav bar label
  title:                 string;   // Page heading
  wsPath:                string;   // WebSocket endpoint, e.g. "/ws/json/path"
  storageKeyPayload:     string;   // localStorage key for payload
  storageKeyHistory:     string;   // localStorage key for command history
  storageKeyTab:         string;   // localStorage key for right-panel tab
  storageKeySavedGraphs?: string;  // present → enables saved-graph bookmarks
  supportsUpload?:       boolean;  // present → enables REST upload handshake
  supportsClipboard?:    boolean;  // true → enables node clip/paste sidebar (Minigraph only)
  supportsHelp?:         boolean;  // true → enables bundled help panel
  supportsAuthoring?:    boolean;  // true → enables create-node UI authoring (Minigraph only)
  storageKeyHelpTopic?:  string;   // localStorage key for the last help topic
  tabs:                  RightTab[]; // ordered list of right-panel tabs to show
}
```

The file also exports shared runtime constants used across the app:

| Constant | Value | Purpose |
|---|---|---|
| `MAX_ITEMS` | 200 | Console message ring buffer size |
| `MAX_HISTORY` | 50 | Command history entries in localStorage |
| `MAX_AUTOCOMPLETE_SUGGESTIONS` | 8 | Maximum history suggestions shown in the command input dropup |
| `MAX_BUFFER` | 63,488 | WebSocket send character limit (safely under 64 KB) |
| `PING_INTERVAL` | 20,000 ms | Keep-alive ping frequency |

The file also exports `SAMPLE_DATA: Record<string, string>` — a set of quick-load JSON/XML payloads for the `PayloadEditor`. Key format: `"<type>_<label>"` where the type prefix (`json` or `xml`) groups buttons by row and the label becomes button text (underscores become spaces). Adding a new entry causes a new button to appear automatically in the `PayloadEditor`.

> **Maintainer tip:** To add a new playground, add one object to `PLAYGROUND_CONFIGS`. The route, nav link, connection dot, localStorage namespace, and right-panel tabs are all derived automatically.

`supportsAuthoring` should be enabled only for a backend that accepts the Minigraph multiline command grammar. The UI authoring layer does not define a new protocol; it serialises to raw command text and sends it through the existing WebSocket command path.

---

### 6.3 WebSocket State — Context + Hook

The WebSocket layer is split across two files with distinct responsibilities:

#### `src/contexts/WebSocketContext.tsx` — shared, navigation-persistent state

Holds a `useReducer`-managed map of `SlotState` objects (`AllSlots`), one per `wsPath`. Each `SlotState` contains:
- `phase: 'idle' | 'connecting' | 'connected'`
- `messages: { id: number; raw: string }[]` (ring-buffered at `MAX_ITEMS`)

> **⚠️ Gotcha:** An exported `WsSlot` interface also exists on this file (L32) but is **not** what the reducer uses internally — it is a public type for potential external consumers. The actual per-slot reducer state is `SlotState` (L76). Do not confuse these.

WebSocket instances (`wsRefs`), ping interval handles (`pingRefs`), and message ID counters (`msgIdRefs`) live in `useRef` Records (outside the reducer, L153–L155) so they never cause re-renders.

**`connect(wsPath, onToast)`** ([L206](../src/contexts/WebSocketContext.tsx#L206)):
1. Dispatches `CONNECTING`
2. Opens `new WebSocket(makeWsUrl(wsPath))`
3. On `open`: dispatches `CONNECTED`, sends `{"type":"welcome"}` (plain `JSON.stringify`, not `eventWithTimestamp`), starts the ping interval
4. On `message`: filters out keep-alive ping/pong frames via `isKeepAliveMessage()` ([L192](../src/contexts/WebSocketContext.tsx#L192)), dispatches `MESSAGE_RECEIVED`
5. On `close`/`error`: clears ping interval, dispatches `DISCONNECTED`

Auto-connect on startup: on mount the provider silently opens a socket for every entry in `PLAYGROUND_CONFIGS` ([L288](../src/contexts/WebSocketContext.tsx#L288)). React StrictMode double-invoke is handled safely by the `onclose` ref guard.

**`send(wsPath, data)`** ([L313](../src/contexts/WebSocketContext.tsx#L313)): Writes directly to `wsRef.current` — no React re-render.

**`pendingPayloads`** is a `useState` (not `useRef`) map so that depositing a payload via `setPendingPayload` triggers a re-render in consuming components (specifically the JSON-Path Playground's `useEffect` polling for cross-playground payload transfer).

#### `src/hooks/useWebSocket.ts` — per-playground UI state

A thin delegation layer that reads its slot from the context and adds purely local concerns:

- **Command input + history** via a local `useReducer` (not shared — intentionally resets on remount)
- **Auto-scroll** via a `consoleRef` and a `useEffect` watching `messages`
- **History navigation** with ↑/↓ arrow key handling; saves and restores the in-progress command when entering/exiting history mode
- **`sendCommand()`**: sends the command, pushes to history, and — for the special `load` command — also sends the payload as a second WebSocket message
- **Two-step upload handshake**: a `pendingUploadRef` is armed by `uploadPayload()` ([L272](../src/hooks/useWebSocket.ts#L272)). When a `ProtocolBus` is provided via the optional `bus?` prop ([L40](../src/hooks/useWebSocket.ts#L40)), the hook subscribes to `upload.contentPath` events ([L194](../src/hooks/useWebSocket.ts#L194)) for responsive detection. Otherwise, a legacy `useEffect` watches messages for the `extractUploadPath` pattern ([L230](../src/hooks/useWebSocket.ts#L230)) and fires the `fetch` POST.
- **`sendRawText(text)`** ([L286](../src/hooks/useWebSocket.ts#L286)): sends without echoing to history. Used by automation hooks for silent commands like `describe graph` and by the create-node authoring executor for modal-submitted raw command text. Returns `false` when the socket is not open, allowing callers to keep their UI state instead of assuming the send succeeded.

---

### 6.4 Playground.tsx — The Orchestrator

`Playground.tsx` is the most important file to understand. It does **no rendering logic** — it wires together all hooks and passes their outputs as props to dumb layout components.

Key responsibilities:

| Concern | How handled |
|---|---|
| Protocol Kernel | `busRef = useRef(new ProtocolBus())` keeps a single stable bus instance; `useProtocolKernel({ messages, bus })` returns `classificationMap` |
| Payload persistence | `useLocalStorage(storageKeyPayload)` — survives navigation and refresh |
| Cross-playground payload | Separate `useState(null)` (`payloadOverride`) initialised from `ctx.takePendingPayload` at mount; never written to localStorage; wins over stored value when set |
| Payload validation | `useMemo(() => validatePayload(payload))` — synchronous, no extra render |
| Toast notifications | `useToast()` — queue-based, auto-removes after timeout |
| Graph path pinning | `usePinnedGraphPath(wsPath)` — module-scope `Map<string, string | null>` keyed by wsPath; cleared on disconnect via transition-based `useEffect` in Playground.tsx |
| Preview message pinning | removed — `MarkdownPreview` and `pinnedMessageId` are deleted |
| Panel split persistence | `useDefaultLayout` from `react-resizable-panels` — keyed per route path (`config.path + '-panel-split'`) |
| Responsive layout | `useMediaQuery('(max-width: 768px)')` → vertical panel stacking on mobile |
| Clear messages | Clears `pinnedGraphPath`, `successfulUploadPaths`, and `graphData`; does **not** clear `modalUploadPath` (modal stays open if active during clear) |
| Cross-playground routing | Three branches: immediate deposit + navigate when JSON-Path is `'connected'`; overwrite `pendingJsonTransferRef` + toast when `'connecting'` (last-write-wins); `pendingJsonTransferRef` + `ctx.connect()` when `'idle'` |
| Mock-upload modal path | `useState<string \| null>(null)` — `null` = closed; non-null = open for that specific POST endpoint |
| Modal trigger element | `useRef<HTMLElement \| null>(null)` — captures `document.activeElement` before open; `.focus()` restored on close via `setTimeout` |
| Successful upload paths | `useState<Set<string>>(new Set())` — keyed by POST path; drives ✅ badge on invitation rows; session-only (cleared with `clearMessages`) |
| Graph display name | `graphDisplayName = useMemo(...)` — resolves root node's `name` property → `graphSaveName` fallback; threaded as `graphName` prop to `RightPanel` → `GraphView` / `GraphDataView` → `GraphToolbar` |
| Graph authoring | `createGraphAuthoringExecutor(ws.sendRawText)` + `useGraphAuthoring({ bus, connected, graphData, executor })`; modal state rendered through `GraphAuthoringModals`; `openCreateNode` threaded to `RightPanel` → `GraphView` |
| Graph save-form name | `useGraphSaveName(storageKey, bus)` — provides `defaultName`, `setLastSavedName`, `resetName`; see §3.13 |
| Deferred graph export | `pendingSaveRef = useRef<PendingSave | null>(null)` where `PendingSave = { graphName, timeoutId }` — armed when connected save is sent; confirmed by `graph.exported` (name-matched), rejected by `graph.export.failed`, timeout (10 s), or disconnect |
| Deferred JSON-Path send | `pendingJsonTransferRef = useRef<{ wsPath, json } | null>(null)` — last-write-wins mailbox; overwritten on each send while JSON-Path is `'connecting'`; consumed by a `useEffect` watching the slot once `phase === 'connected'` |
| Clipboard integration | `useClipboardContext()` for `clipNode` / `confirmReplace`; `clipboardOpen` via `useLocalStorage`; `duplicateDialogState` via `useState`; guarded by `supportsClipboard` config flag |
| Command history for autocomplete | `ws.history` (string array, newest-first) passed as `commandHistory` to `LeftPanel` → `CommandInput` for the history dropup |
| Classification map threading | `classificationMap` passed to `LeftPanel` → `Console` → `ConsoleMessage` |

**Pinning logic** (`handleGraphLinkMessage`):
- Reads `classificationMap.get(msg.id)` to find pre-classified events
- If any event has `kind === 'graph.link'` → reads `event.apiPath` to set `pinnedGraphPath` (triggers `useGraphData`) and highlight the row

---

### 6.5 Left Panel — Console & Command Input

**`LeftPanel.tsx`** — a thin layout shell that positions `Console` above `CommandInput`.

**`Console.tsx`** — renders a scrollable `role="log"` div. Each message is wrapped in a `ConsoleErrorBoundary` so a rendering crash in one row does not take down the whole list. It also has copy-all and clear-all toolbar buttons. Receives `classificationMap` as a prop and passes it through to each `ConsoleMessage`.

**`ConsoleMessage.tsx`** ([L37](../src/components/Console/ConsoleMessage.tsx#L37)) — the most visually complex component. Receives `msgId` and `classificationMap` props ([L10–L11](../src/components/Console/ConsoleMessage.tsx#L10)). For each message it:
1. Calls `parseMessage()` to extract `{type, message, time}`
2. Calls `tryParseJSON()` — JSON messages render as a collapsible `<JsonView>` tree
3. Reads pre-computed events from `classificationMap.get(msgId)` ([L42](../src/components/Console/ConsoleMessage.tsx#L42)) — `isGraphLink`, `isLargePayload` ([L44](../src/components/Console/ConsoleMessage.tsx#L44)), `isMockUpload` are derived via `events.some(e => e.kind === ...)` instead of calling parser functions at render time
4. Conditionally renders:
   - A **pin button** (📌 for plain-text, 🕸️ for graph-links) — clickable rows with full keyboard support (`role="button"`, `tabIndex`, `Enter`/`Space` handlers). Mock-upload and large-payload rows are **explicitly excluded** from `isPinnable` via `&& !isMockUpload && !isLargePayload` ([L54](../src/components/Console/ConsoleMessage.tsx#L54)) to prevent nested-interactive-element accessibility violations
   - A **copy button** (📄 → ✅) with scoped `copied` state
   - A **send-to-JSON-Path button** (➡️) on JSON messages when the callback is wired
   - A **"⬆️ Upload JSON…" re-open button** on mock-upload invitation rows when `onUploadMockData` is wired; shows a ✅ badge when `successfulUploadPaths` contains the row's POST path

**`CommandInput.tsx`** — a single auto-growing textarea with a Send button. No separate multiline toggle exists; the textarea grows naturally as the user types. Key behaviours:
- **Auto-grow:** a `useEffect` watching `command` sets `el.style.height` from `scrollHeight` on every value change — covers both user typing and programmatic history-navigation updates
- **History dropup:** powered by `useHistoryAutocomplete(history, command)`; the dropup (`role="listbox"`, `id="history-dropup"`) is always in the DOM (toggled via `hidden`) so `aria-controls` always resolves; items show the matched prefix in `<strong>` and a `↵` badge for multi-line entries
- **Command palette:** a terminal-prompt icon button (`aria-expanded`, `aria-controls="command-palette"`) opens a popover `role="listbox"` listing all `COMMAND_QUICKSTART` entries; clicking a row inserts the entry's `template` string and closes the palette
- **ARIA combobox pattern:** the textarea has `role="combobox"`, `aria-haspopup="listbox"`, `aria-controls="history-dropup"`, and `aria-activedescendant` pointing to the active item; focus never leaves the textarea
- **Key handling** (`handleKeyDown`): Tab accepts active suggestion (or first if none highlighted); Enter accepts highlighted suggestion or sends; Shift+Enter lets the browser insert a newline; Escape dismisses the dropup; ↑/↓ navigates the dropup when open, otherwise delegates to the history handler in `useWebSocket` with a `requestAnimationFrame` caret-to-end fix
- **Outside-click:** a `mousedown` listener on `document` closes the palette when focus moves outside `paletteWrapperRef`

---

### 6.6 Right Panel — Tabs

**`RightPanel.tsx`** — renders only the tabs listed in the playground's `tabs` config. Uses `useId()` for accessible `aria-controls` / `role="tab"` associations. Active tab is driven by `rightTab` state from `useGraphData` (persisted in localStorage).

`useGraphData` normalizes the persisted tab value against the current `tabs` array before `RightPanel` renders. This prevents legacy localStorage entries from older UI versions from blanking the panel after a tab is removed. Example: an old Minigraph value of `preview` is migrated to `graph` because Minigraph now exposes only `graph` and `graph-data`.

When `supportsAuthoring` is enabled, `RightPanel` forwards `isConnected` and `onCreateNode` into `GraphView`. The right panel does not own authoring state; it only passes the graph-tab affordance down to the renderer.

The three possible tabs:

| Tab key | Component | When shown |
|---|---|---|
| `'payload'` | `PayloadEditor` | JSON-Path playground |
| `'graph'` | `GraphView` | Both playgrounds (receives `graphName` prop) |
| `'graph-data'` | `GraphDataView` | Both playgrounds (receives `graphName` prop) |

The help panel is **not** a tab. It is a resizable overlay rendered inside `RightPanel` when `supportsHelp` is true and `helpOpen` is set. Its split position and maximised state are persisted in `localStorage`.

---

### 6.7 Graph Pipeline

The journey from a WebSocket message to a rendered graph has four stages:

```
1. Server emits graph-link message in WebSocket stream
   e.g. "Graph described in /api/graph/model/ws-123-1"

2. messageParser.extractGraphApiPath() → "/api/graph/model/ws-123-1"
   (either by user clicking the 🕸️ row, or automatically via useAutoGraphRefresh)

3. Playground.tsx sets pinnedGraphPath → useGraphData fires REST fetch
   GET /api/graph/model/ws-123-1
   Response: { nodes: [...], connections: [...] }

4. graphTransformer.transformGraphData(data)
   → ReactFlow nodes[]  (with computed layout positions + per-type styles)
   → ReactFlow edges[]  (with relation labels)

5. GraphView renders <ReactFlow> with the computed nodes/edges
```

#### `src/utils/graphTypes.ts` — Type definitions & type guard

Defines `MinigraphGraphData`, `MinigraphNode`, `MinigraphConnection`, and `MinigraphRelation`. The exported `isMinigraphGraphData(value)` type guard is called on every REST response before setting state — it guards against malformed JSON without throwing.

#### `src/utils/graphTransformer.ts` — Layout algorithm & edge routing

The transformer performs two major jobs: **layout** (assigning pixel positions) and **edge routing** (assigning per-connection handle IDs with correct side placement).

**Layout — `computeLayout()`** returns `{ positions, levelOf }`:

1. **Classify & partition:** Every node is classified via `classifyNode()` into a `LayoutCategory`. Connected nodes (participating in at least one edge) always go to `'flow'`. Orphaned nodes are segregated by type: `'Dictionary'`, `'Provider'`, `'Module'` (has `graph.math` / `graph.js` skill), `'Entity'` (no skill), or `'__unknown__'` catch-all.
2. **Cycle detection (iterative DFS):** Before BFS, a DFS from all seed/flow nodes identifies back-edges (edges to an ancestor in the DFS tree). These are recorded in a `backEdges` set and excluded from the BFS queue to prevent infinite looping on cycles.
3. **BFS level assignment:** Seeds (in-degree 0 or `entry_point` typed) start at level 0. BFS assigns each flow node a column, skipping back-edges. A node's level is always > its predecessor's level.
4. **Vertical stacking:** Within each level, flow nodes are stacked vertically centred at y = 0 with per-node heights preventing overlap.
5. **Segregated rows:** After computing the main flow bounding box, each non-flow category gets its own horizontal row below the flow in order: Dictionary → Provider → Module → Entity → __unknown__. Nodes within each row are sorted alphabetically for visual stability.

**Edge routing — inside `transformGraphData()`:**

After layout, each connection is classified as forward or backward by comparing source/target levels from `levelOf`. Forward edges exit the **right** side of the source and enter the **left** side of the target (normal left-to-right flow). Back-edges reverse this: they exit the **left** side of the source and enter the **right** side of the target, producing a natural backward-arcing bezier curve.

Handles on each node side (left and right) are collected, sorted by the y-position of the connected peer node, and assigned interleaved handle IDs (forward + back-edge mixed together). This y-sorting prevents edge crossing: connections to higher peers get higher handle slots.

The `GraphNodeData` interface carries four handle arrays: `sourceHandles` (right, forward out), `targetHandles` (left, forward in), `backSourceHandles` (left, back-edge out), and `backTargetHandles` (right, back-edge in).

Node styles are applied via `node.style` (not CSS classes) using the CSS custom property `--node-accent` for per-type accent colours. The full set of recognised type names is `Root`, `End`, `Fetcher`, `mapper`, `Math`, `JavaScript`, `Provider`, `Dictionary`, `Join`, `Extension`, `Island`, `Decision` — unknown types fall back to a neutral grey accent. This is the pattern required by ReactFlow's `NodeResizer` — having no inner wrapper div means the RF wrapper element *is* the visual shell.

#### `src/components/GraphView/NodeTypes.tsx` — Custom node

`MinigraphNode` renders as a **React Fragment** (no wrapper div). Top-level siblings in order: `NodeResizer`, forward target handles (left), back-edge source handles (left), content div, forward source handles (right), back-edge target handles (right). This avoids all the sizing workarounds that a nested div structure requires. The `nodeTypes` map exported here is passed directly to `<ReactFlow nodeTypes={nodeTypes}>`.

#### `src/components/GraphToolbar/GraphToolbar.tsx` — Graph name & stats

Displays a resolved `graphName` prop prominently (falls back to "Untitled"). Node and connection counts are shown as hover-reveal stats (CSS transition: opacity + max-width on `.nameGroup:hover`). The `graphName` prop is threaded from `Playground.tsx` → `RightPanel` → `GraphView` / `GraphDataView` → `GraphToolbar`.

#### `src/components/GraphView/GraphView.tsx` — Clipboard context menu

When `onClipNode` is wired (i.e. `supportsClipboard` is true in the playground config), right-clicking a node opens a context menu with a "Clip to Clipboard" option. This calls `onClipNode(node, connections)` which is threaded from `Playground.tsx` → `RightPanel` → `GraphView`.

#### `src/components/GraphView/GraphView.tsx` — Create-node entry points

When `supportsAuthoring`, `onCreateNode`, and `isConnected` are all truthy, the graph view exposes create-node actions:
- Empty graph: the empty state renders a "Create Node" button. Clicking it calls `onCreateNode('empty-graph')`.
- Existing graph: right-clicking the pane calls `onPaneContextMenu`, records `event.clientX` / `event.clientY`, and opens `GraphContextMenu` at the pointer location. Clicking "Create Node" calls `onCreateNode('pane-context-menu')`.

The node context menu remains reserved for clipboard actions. Pane-level create-node and node-level clipboard clipping are separate menus so a right-click on a node never accidentally opens a create-node flow.

#### `src/components/GraphView/GraphContextMenu.tsx`

A lightweight, pointer-positioned menu for graph-pane actions. It focuses its first item on open, closes on outside pointer down or Escape, and does not trap focus. It is intentionally not centered like a modal; the `x`/`y` coordinates come from the user’s right-click.

#### `src/components/GraphView/GraphViewErrorBoundary.tsx`

A class-based error boundary that resets when its `key` prop changes. `GraphView` passes a `boundaryKey` derived from `graphData.nodes.map(n => n.alias)` — a corrected graph after a previous render failure renders cleanly without a page reload.

---

### 6.8 Protocol Kernel Layer

This layer centralises message classification so that hooks subscribe to typed events instead of scanning raw messages themselves. See §3.14 for the feature overview.

#### `src/protocol/events.ts` — Event type definitions

A discriminated union type `ProtocolEvent`. Each variant carries a `kind` tag, the originating `msgId`, plus kind-specific payloads (e.g. `GraphLinkEvent` includes `apiPath`; `LargePayloadEvent` includes `byteSize`, `apiPath`, and `filename` — the last path segment of `apiPath` with `.json` appended, reserved for a future download action). Create-node result correlation uses `CreateNodeTextResultEvent` with `kind: 'minigraph.createNode.textResult'`, `status`, `alias`, and the raw backend message. Session lifecycle uses `SessionResetEvent` with `kind: 'session.reset'` — emitted when the backend sends `"Session restarted"` after a `session reset` command. `ProtocolEventKind` is automatically derived as `ProtocolEvent['kind']` — adding a new variant to the union automatically updates the kind set.

#### `src/protocol/classifier.ts` — Pure classification function

`classifyMessage(msgId: number, raw: string): ProtocolEvent[]` ([L27](../src/protocol/classifier.ts#L27)) returns an **array** because a single message may match multiple rules (e.g. `"node root created"` is both a `graph.mutation` and a `minigraph.createNode.textResult`). The fallback `unclassified` rule fires only when no other rule matched.

The classifier delegates to `messageParser.ts` predicates (`extractGraphApiPath`, `extractLargePayloadLink`, `extractMockUploadPath`, `extractUploadPath`, `isHelpOrDescribeCommand`, `detectMutation`, `parseCreateNodeTextResult`, `extractImportGraphName`, `isGraphLinkMessage`, `isMarkdownCandidate`, `tryParseJSON`) — it adds no new parsing logic, only orchestrates existing parsers into a single classification pass. The module-level constant `SESSION_RESTARTED_MSG = 'Session restarted'` is the single source of truth for the session-reset wire string used by Rule 7c; both the classifier and its tests import from this constant.

#### `src/protocol/bus.ts` — Typed event emitter

`ProtocolBus` ([L16](../src/protocol/bus.ts#L16)) is a ~45-line synchronous event emitter. Listeners are stored in a `Map<ProtocolEventKind, Set<(event: ProtocolEvent) => void>>`. Key methods:
- `on(kind, handler)` ([L22](../src/protocol/bus.ts#L22)) — returns an unsubscribe function (for `useEffect` cleanup)
- `emit(event)` ([L31](../src/protocol/bus.ts#L31)) — fires all registered handlers for `event.kind` in Set insertion order
- `clear()` ([L42](../src/protocol/bus.ts#L42)) — removes all listeners (used in tests; not called in production code)

The bus is created **once** via `useRef` in `Playground.tsx` and never changes identity. It is deliberately *not* React Context — it is an imperative coordination mechanism between effects, not a source of rendered state.

#### `src/protocol/useProtocolKernel.ts` — React integration hook

`useProtocolKernel({ messages, bus })` ([L33](../src/protocol/useProtocolKernel.ts#L33)):

1. **Effect 1** ([L40](../src/protocol/useProtocolKernel.ts#L40)): On mount, initialises `watermarkRef` to the last message ID in the existing log — prevents replaying historical messages
2. **`classificationMap`** ([L47](../src/protocol/useProtocolKernel.ts#L47)): A `useMemo` that iterates the full messages array and builds `Map<number, ProtocolEvent[]>` via `classifyMessage()`. This is the **render-path** output consumed by `ConsoleMessage`
3. **Effect 2** ([L56](../src/protocol/useProtocolKernel.ts#L56)): Watermark-gated loop that emits only **new** events to the bus. This is the **effect-path** output consumed by automation hooks

The two outputs are deliberately separate: the map is stable across re-renders (same `messages` array → same map reference), while the bus emission is a one-shot side-effect.

---

### 6.9 Automation Hooks

All automation hooks now subscribe to the `ProtocolBus` instead of scanning messages directly. The common pattern:

```
useEffect (runs once at mount):
  const unsub = bus.on('event.kind', (event) => {
    // act on event
  });
  return () => unsub();
```

Because `bus` is a stable `useRef` object, the subscription effect runs only once. Hooks read mutable state through refs (e.g. `connectedRef`, `sendRawTextRef`, `pinnedGraphPathRef`) to avoid stale closures without adding dependencies.

#### `useAutoGraphRefresh`

Subscribes to:
- `bus.on('graph.link')` — when `waitingForDescribeRef` is true, consumes the graph path and calls `setPinnedGraphPath`
- `bus.on('graph.mutation')` — arms `waitingForDescribeRef` **immediately at mutation-detection time** (before the debounce timer starts), then arms a 300 ms debounce timer; when the timer fires it re-arms `waitingForDescribeRef` (in case an early forwarded `graph.link` accepted and reset the gate before the debounce fired), sends `describe graph` silently, and emits an `addToast` notification
- `bus.on('session.reset')` — cancels any in-flight debounce, resets `waitingForDescribeRef` to false, and calls `setPinnedGraphPath(null)` to clear the stale graph view

**Why gate-arm timing matters in collaborative sessions:** the backend forwards the primary's `describe graph` response to all subscriber sessions. This forwarded `graph.link` arrives during the 300 ms debounce window — before the subscriber's debounce fires and before the gate would previously have been armed. The immediate arm ensures the forwarded link is accepted. The re-arm inside the debounce callback handles the inverse: if the forwarded link arrived early and already consumed the gate, the debounce re-arms it so the session's own `describe graph` response is also accepted.

**Stale-closure fix**: `pinnedGraphPath` is read via `pinnedGraphPathRef` inside the debounce timer callback — if it were read directly from closure it would be stale after subsequent renders. Same pattern for `connectedRef` and `sendRawTextRef`.

**Auto-refresh always uses the initial-load path**: the hook sends `describe graph` via `sendRawText`, which triggers a new graph-link event and updates `pinnedGraphPath`. This triggers the **initial-load** path in `useGraphData` (graph clears to `null`, re-fetches). The overlay-spinner path (`refetchGraph`) is only triggered imperatively by direct user actions, not by auto-refresh.

Disconnect cleanup: when `connected` flips to false, pending debounce timers are cleared and `waitingForDescribeRef` is reset.

#### `useAutoHelpNavigate`

Subscribes to `bus.on('command.helpOrDescribe')`. When the echo is a bundled `help` topic, calls `onTabSwitch()` to open the help panel and sets `activeHelpTopic` via `setHelpTopic`. Both server-echoed and locally-appended help command echoes classify identically, so the hook works for both connected and disconnected local-help paths.

`useAutoMarkdownPin` has been removed. `describe` responses are console-driven and are not captured by the help panel.

#### `useLargePayloadDownload`

Subscribes to `bus.on('payload.large')`. An `isFetchingRef` ([L25](../src/hooks/useLargePayloadDownload.ts#L25)) re-entrancy guard prevents concurrent fetches. When the event fires, retrieves `event.apiPath`, fetches the endpoint, pretty-prints JSON, and calls `appendMessage(content)` to inject the result into the console.

#### `useAutoMockUpload`

Subscribes to `bus.on('upload.invitation')` ([L31](../src/hooks/useAutoMockUpload.ts#L31)). Uses `pendingModalRef` ([L22](../src/hooks/useAutoMockUpload.ts#L22)) as a first-match guard — only the first invitation in a batch opens the modal. A `modalOpen: boolean` prop ([L8](../src/hooks/useAutoMockUpload.ts#L8)) tracks the modal's open/closed state; a dedicated clearing effect ([L26](../src/hooks/useAutoMockUpload.ts#L26)) resets `pendingModalRef` to `false` when `modalOpen` flips from `true` to `false`, so the next server invitation will trigger the modal normally.

Note: the `connected` prop is still accepted for interface symmetry with peer hooks but has no effect on the hook's behaviour — its JSDoc describes exactly why the watermark pattern was **not** reset on disconnect (replaying stale invitations on reconnect would be worse than missing a new one).

#### `useGraphSaveName`

Subscribes to `bus.on('command.importGraph')` ([L99](../src/hooks/useGraphSaveName.ts#L99)) — clears `lastSavedName` to `null` when a new `import graph from {name}` echo arrives, so the imported name takes over immediately. See §3.13 for the full priority logic.

---

### 6.10 Utilities

#### `src/utils/messageParser.ts`

The low-level classification engine for WebSocket messages. These functions are now called by the Protocol Kernel's `classifier.ts` instead of being invoked directly by automation hooks (though they remain exported for use in `ConsoleMessage` rendering). Key exports:

| Function | Purpose |
|---|---|
| `parseMessage(raw)` | Parse JSON or return raw as `{type:'raw'}` |
| `tryParseJSON(str)` | Returns `{isJSON, data}` — only true for objects/arrays, not primitives |
| `isMarkdownCandidate(raw)` | True for non-JSON strings; false for **any** valid JSON object or array |
| `isGraphLinkMessage(raw)` | True when the message contains `/api/graph/model/...` |
| `isLargePayloadMessage(raw)` | True for `"Large payload (N) -> GET ..."` |
| `isMockUploadMessage(raw)` | True for `"You may upload … -> POST /api/mock/..."` upload invitations |
| `isHelpOrDescribeCommand(raw)` | True for `"> help ..."` and `"> describe <non-graph>"` echoes |
| `detectMutation(raw)` | Returns `'node-mutation'`, `'import-graph'`, or `null` |
| `parseCreateNodeTextResult(raw)` | Parses create-node backend text into `{status, alias, message}` for modal result correlation |
| `extractGraphApiPath(raw)` | Regex extracts `/api/graph/model/{id}` |
| `extractUploadPath(raw)` | Regex extracts `/api/json/content/{id}` (JSON-Path upload handshake) |
| `extractMockUploadPath(raw)` | Regex extracts `/api/mock/{id}` from the mock-upload invitation |
| `extractLargePayloadLink(raw)` | Parses the byte size, API path, and a derived `filename` (last path segment + `.json`, e.g. `"input.body.json"`) from the large-payload notification; the `filename` field is reserved for a future "save to disk" action |

**`detectMutation` matching rules** (important to understand for maintenance):

```
'import-graph'   if lower includes the backend import-graph success phrase
'node-mutation'  if lower includes ' -> ' AND 'removed'         (connection delete)
'node-mutation'  if lower.startsWith('node ') AND:
                    includes ' created'
                    includes ' updated'
                    includes ' deleted'
                    includes ' connected to '
                    includes ' imported from '
                    includes ' overwritten by node from '
```

The `startsWith('node ')` prefix guard is critical — it prevents false positives from `"Graph instance created"` (`instantiate graph`) and `"Root node created because it does not exist"` (`export graph`).

#### `src/utils/graphTransformer.ts`

Converts `MinigraphGraphData` to ReactFlow `nodes[]` and `edges[]`. Includes node classification (`classifyNode`), BFS layout with DFS cycle detection, orphan-node segregated rows, and back-edge handle routing. See §6.7 for the full layout algorithm and edge routing detail.

#### `src/utils/urls.ts`

Single source of truth for URL construction:

```typescript
makeWsUrl(wsPath)
  // dev  → ws://localhost:3000{wsPath}   (Vite proxy on port 3000)
  // prod → ws://{window.location.host}{wsPath}  (same origin)
```

HTTP API paths are always **relative** — the Vite proxy forwards them in dev; the same-origin server handles them in production.

#### `src/utils/validators.ts`

`validatePayload(text)` tries `JSON.parse`, then `DOMParser` XML parse. Returns `{valid, error, type}`. `formatJSON` pretty-prints JSON. Used in `PayloadEditor` for live validation feedback.

---

### 6.11 Navigation Bar

**`Navigation.tsx`** reads all playground configs and renders two dropdown menus built on `NavMenu`:

**Tools menu** — contains a **Connect All / Disconnect All** batch action at the top, followed by per-playground entries:

- **Connect All / Disconnect All** button at the top of the menu:
  - `anyConnecting` → "Connecting…" (disabled)
  - `allConnected` → "Disconnect All"
  - otherwise → "Connect All"
- Per-playground entries: a `<NavLink>` (navigates) with a status dot, plus a separate **Start/Stop** button (connects/disconnects without navigating)

The aggregate dot status across all playgrounds uses `aggregateDotStatus()`:
- All connected → green
- All idle → grey
- Any connecting → pulsing yellow
- Mixed → partial (some connected)

**Quick Links menu** — static links to backend `/info`, `/health`, `/env`, etc.

**`NavMenu.tsx`** — a reusable accessible dropdown:
- `Escape` closes it
- Click outside closes it (`useEffect` + `mousedown` listener)
- `aria-expanded` / `aria-haspopup` for screen readers

---

### 6.12 Saved Graphs

**`useSavedGraphs(storageKey)`** manages a `Record<string, SavedGraphEntry>` in localStorage. The data model stores only the graph **name** (a string), not the graph data itself — the server holds the actual file.

**Save flow (connection required, deferred confirmation):**
```
User clicks Save → GraphSaveButton inline form
  (button disabled when !connected — GraphSaveButton enforces disabled={disabled || !connected})

  handleSaveGraph(name):     [in useSavedGraphWorkflow]
    if (!connected):
      addToast('Save failed: connection required to export graph', 'error')
      return                 // no bookmark written; no state advanced

    pendingSaveRef.current = { graphName: name, timeoutId }  // arm — no localStorage write yet
    ws.sendRawText(`export graph as ${name}`)  // server writes {name}.json

    Resolution paths (bus effects):
      → bus.on('graph.exported', name-matched)
              → clearTimeout(timeoutId)
              → savedGraphs.saveGraph(name)       // localStorage bookmark written HERE
              → setLastSavedName(name)            // in useGraphSaveName
              → addToast(`Graph saved as "${name}"`, 'success')
      → bus.on('graph.export.failed')
              → clearTimeout(timeoutId)
              → addToast(typed error message, 'error')   // NO bookmark written
      → 10-second timeout fires
              → addToast('Save failed: export confirmation timed out', 'error')
      → disconnect while in-flight (connected flips to false)
              → clearTimeout(timeoutId)
              → addToast('Save failed: connection closed before export confirmation', 'error')
```

**Load flow:**
```
User clicks Load in SavedGraphsMenu →
  handleLoadGraph(name):
    ws.sendRawText(`import graph from ${name}`)  // server reads {name}.json
    → server responds with a mutation success message
    → useAutoGraphRefresh detects import-graph
    → fires describe graph automatically
    → graph renders
```

**Save-form pre-fill (`useGraphSaveName`):**

The name pre-filled in the save form is managed entirely by `useGraphSaveName` — `GraphSaveButton` receives only the already-computed `defaultName` string and a `setLastSavedName` callback; it knows nothing about the priority logic.

```
Priority (highest → lowest):

1. lastSavedName   set by setLastSavedName(name) after a successful save
2. importedName    extracted from "> import graph from {name}" echo in WS stream
3. untitled-{n}    persisted localStorage counter, starts at 1, never skips
```

The counter increment rule: `untitled-{n}` only advances to `untitled-{n+1}` when `untitled-{n}` was actually saved (i.e. `setLastSavedName` was called with a name matching the current untitled fallback). Clearing the console without ever saving reuses the same slot. This guarantees `untitled-2` never appears unless `untitled-1` was saved first.

`importedName` always supersedes `lastSavedName` for a new import: the `bus.on('command.importGraph')` subscription calls `setLastSavedNameState(null)` immediately when a fresh `> import graph from {name}` arrives — `lastSavedName` is cleared in the same callback that sets `importedName`, so the priority chain reflects the new state in the very next render.

See §3.13 for the full feature description and all code locations.

---

### 6.13 Graph Authoring Layer

The create-node UI is split into a pure domain layer, a lifecycle hook, and presentational components.

#### `src/graphActions/*` — Pure authoring helpers

This folder is intentionally independent of React. It contains:
- `nodeAuthoringTypes.ts` — create-node form model, `PropertyRow`, and the source union (`'empty-graph' | 'pane-context-menu'`)
- `propertyRows.ts` — deterministic default form values and row IDs used only for React rendering/error keys
- `validation.ts` — frontend validation for the supported authoring surface; keeps the token rules aligned with backend name validation and checks final command size against `MAX_BUFFER`
- `minigraphCommandBuilder.ts` — serialises valid form values into raw multiline Minigraph command text
- `graphAuthoringExecutor.ts` — a thin interface and production adapter over `sendRawText`

The builder is the single serialisation boundary. UI components should never concatenate the backend command directly.

#### `src/components/GraphAuthoring/useGraphAuthoring.ts`

Owns the create-node state machine:
- `closed`
- `open/editing`
- `open/sending`

It keeps the current form values, validation errors, the pending submit metadata (`alias`, `command`, `sentAt`), the backend/server message, and a `connectionLost` flag. It validates before building, builds before sending, and sends through `GraphAuthoringExecutor.execute(command)`.

Result matching is best-effort and text-based: the hook subscribes to `bus.on('minigraph.createNode.textResult')` and matches accepted/rejected events by alias. This works because the backend already returns stable text responses for create-node outcomes:
- `node {alias} created` → accepted
- `node {alias} already exists` → rejected
- `ERROR: ...` → error while a submit is pending

The hook does not mutate `graphData` directly. Graph refresh is delegated to the existing auto-refresh path, because the accepted backend message also classifies as `graph.mutation`.

Disconnect handling: a connected→disconnected transition clears the pending timer, keeps the dialog mounted, disables all fields, and shows a refresh/check-graph message.

#### `src/components/NodeDialog/NodeDialog.tsx`

A presentational modal that edits the create-node form and delegates submit/close/change intents upward. Important UI details:
- The modal uses a fixed overlay element, not a native `<dialog>`, so pointer events are absorbed before underlying `react-resizable-panels` handles can drag.
- Escape and backdrop click close only while not sending.
- When a property row is added, focus moves to the new row's key input.
- Remove-property and close controls use a local `CloseIcon.svg` imported through `?react`; CSS controls the hover/focus states, not the SVG itself.
- When `lockReason === 'disconnected'`, all editable controls are disabled and the dialog shows the connection-lost message from the hook.

#### `src/components/GraphAuthoring/GraphAuthoringModals.tsx`

The mount adapter between hook state and modal props. It converts hook state into the modal's `lockReason` (`'sending'`, `'disconnected'`, or `null`) and returns `null` when the authoring state is closed.

---

## 7. Key Engineering Decisions

### 7.1 WebSocket Context Above the Router

**Decision:** `WebSocketProvider` wraps `<BrowserRouter>`, not the other way around.

**Why:** If the provider were inside a route, it would unmount and re-mount on navigation — closing the socket. By living above routes, all playground sockets share a single provider instance that never unmounts during normal navigation.

**Trade-off:** The context holds state for all playgrounds simultaneously, which is a small memory overhead. Given the bounded `MAX_ITEMS = 200` ring buffer per slot, this is negligible.

---

### 7.2 Refs Outside the Reducer for Socket Instances

**Decision:** `WebSocket` instances, ping interval handles, and message ID counters live in `useRef` (not `useState` or inside the reducer).

**Why:** These are imperative handles that must be accessible synchronously in event callbacks. Putting them in state would cause render cycles on every incoming message. The reducer owns only the serialisable view of state (phase + messages array) that React needs to diff and render.

---

### 7.3 Monotonic Message IDs Instead of Array Indices

**Decision:** Each message has a stable `id: number` that increments globally per slot, never recycled.

**Why:** Array indices shift when the ring buffer drops old messages. A pinned message identified by index would shift to a different message after the buffer rotates. The stable ID ensures `pinnedGraphPath` and console message lookups via `classificationMap` always refer to the correct row.

---

### 7.4 Message Watermark Pattern

**Decision:** The `useProtocolKernel` hook centralises the watermark — a single `watermarkRef` ([L37](../src/protocol/useProtocolKernel.ts#L37)) that gates bus emission so that only messages newer than the watermark are emitted. Individual automation hooks no longer maintain their own watermarks.

**Why:** Without the watermark, mounting `useProtocolKernel` into an existing message log would emit every historical message as a new event — triggering spurious mutations, auto-pins, or payload downloads. Previously each of the five automation hooks maintained its own watermark; centralising this into one watermark in `useProtocolKernel` eliminates redundancy and ensures all hooks agree on what has already been processed.

---

### 7.5 `sendRawText` vs `sendCommand`

**Decision:** Two separate send functions.

- `sendCommand()` — writes to history, echoes the command, handles the `load` special case
- `sendRawText(text)` — sends silently, used exclusively by automation hooks

**Why:** Automation hooks must not pollute the command history or produce `> describe graph` echo entries in the console (those would trigger `useAutoHelpNavigate`, since `describe` is a triggering word). `sendRawText` bypasses all that.

---

### 7.6 Node Styles via `node.style` + CSS Custom Properties

**Decision:** ReactFlow node visual styling is applied via `node.style` on the node data object, not via a wrapper `<div className>` inside the component.

**Why:** When `MinigraphNode` renders as a `<Fragment>` (no inner wrapper), the ReactFlow-managed wrapper element *is* the visible shell. `node.style` is the only way to style it. The per-type accent colour is passed as `--node-accent` CSS custom property so the CSS module can reference it for header background, badge colour, and border — without any JS–CSS coupling in the component itself.

---

### 7.7 Two-Path `useGraphData` Design

The hook has two distinct code paths:

- **Initial-load path** (triggered by `pinnedGraphPath` changing): if `pinnedGraphPath` is non-null, clears `graphData` to `null` (clean loading state for new graphs), fetches, and auto-switches the tab to `'graph'` on success. If `pinnedGraphPath` changes to `null` (e.g. on session reset), `graphData` is also set to `null` immediately — the invariant is "no pinned path means no graph data." Uses a `useEffect`-managed `AbortController`.
- **Auto-refresh path** (triggered by `refetchGraph()` call): does *not* clear `graphData` (stale graph stays visible under a spinner overlay), does *not* switch the tab (user is not interrupted). Uses a `useCallback`-stable imperative function with its own abort ref.

**Why separate paths?** A first-time load of a new graph needs visual loading feedback and should switch context to the Graph tab. A background refresh should be invisible to the user — just a spinner on the existing graph.

> **Note:** `useAutoGraphRefresh` currently triggers **only the initial-load path** — it always issues `describe graph` via `sendRawText`, which sends a new graph-link event and updates `pinnedGraphPath`, causing a clean re-fetch. The auto-refresh path (overlay spinner) is structurally available but not invoked by any automated hook; it exists for future imperative use.

---

### 7.8 `useLargePayloadDownload` Re-entrancy Guard

**Decision:** `isFetchingRef` is checked at the top of the main effect.

**Why:** After `appendMessage(content)` injects the fetched payload into the console, `messages` changes, triggering the effect again. Without the guard, the newly appended message would be scanned — `extractLargePayloadLink` would return null for it, but the watermark advance logic uses `+ 1` as a belt-and-suspenders measure to ensure the appended message is treated as seen even before React re-renders.

---

### 7.9 `localStorage` for Persistence, Not Global State

**Decision:** All persistence (payload, command history, right-panel tab, saved graphs) uses `useLocalStorage` — a hook that wraps `useState` with read/write effects and a `storage` event listener.

**Why not Redux or Zustand?** The app's cross-component state needs are modest: the WebSocket context covers the only truly global mutable state. Everything else is either local to a component or derivable from props. Adding a global state library would be over-engineering. The `storage` event listener handles multi-tab synchronisation as a bonus.

---

### 7.10 Payload Override Pattern

**Decision:** `payloadOverride: string | null` is a separate `useState` that wins over `storedPayload` when non-null.

**Why:** Large payloads fetched via REST should not be written to localStorage (they can be megabytes). Instead the override is held in memory. Any manual edit in the textarea calls `setPayload` which calls `setPayloadOverride(null)` first — the user always gets back in control.

---

### 7.11 Protocol Bus — Why Not React Context?

**Decision:** The `ProtocolBus` is a plain class stored in a `useRef`, not a React Context provider.

**Why:** The bus connects effects to effects — none of its state drives rendering. Putting it in Context would cause every consumer component to re-render whenever the bus’s internal listener map changes. As a `useRef`, the bus has a stable identity for the lifetime of the `Playground` component, and subscriptions are managed entirely in `useEffect` cleanup functions.

**Classify-once, emit-to-many:** Because `classifyMessage` is a pure function and `useProtocolKernel` classifies each message exactly once, all downstream hooks receive the same event objects. This prevents subtle bugs where two hooks would classify the same message differently (e.g. due to regex flag differences or parser updates that landed in one hook but not another).

**Ref-wrapping for stable subscriptions:** Automation hooks read mutable state through refs (`connectedRef`, `sendRawTextRef`, `pinnedGraphPathRef`, callback refs) so their bus subscription callbacks never close over stale values. This is critical because the `bus.on()` subscription runs only once (the bus reference never changes), so the callback must be able to read current state without being a React dependency.

---

### 7.12 Create-Node UI Uses the Existing Raw Command Protocol

**Decision:** The create-node UI serialises form state into the existing multiline Minigraph command grammar and sends it with `sendRawText()`. It does not introduce a JSON command API.

**Why:** The backend already owns command semantics in `GraphCommandService`. Reusing the command grammar keeps the UI authoring path behaviorally aligned with the console path and avoids a second protocol that would need separate backend validation, parsing, and error handling.

**Single serialization boundary:** `minigraphCommandBuilder.ts` is the only file that turns form state into raw command text. `NodeDialog` edits form values only; `useGraphAuthoring` validates and calls the builder; the executor sends the final string.

**No optimistic graph mutation:** The frontend waits for the backend text result and lets `useAutoGraphRefresh` fetch the updated graph after the accepted response classifies as `graph.mutation`. This avoids trying to locally mirror backend graph rules.

**Custom overlay instead of native dialog:** `NodeDialog` uses a fixed overlay element to absorb pointer events before underlying `react-resizable-panels` handles can drag. A native `<dialog>` backdrop was not sufficient for this interaction in the existing panel layout.

---

## 8. Data & Message Flows

### 8.1 User Sends a Command

```
User types in CommandInput → ws.command state
User presses Enter or Send button → ws.sendCommand()
  → ctx.send(wsPath, text)           — fires WebSocket.send()
  → pushes to history in localStorage
  → if text === 'load': also sends payload as second WS message
  → dispatch CLEAR_COMMAND
Server echoes: "> <command>"          — arrives via ws.onmessage
  → dispatch MESSAGE_RECEIVED
  → ConsoleMessage renders it as plain text
Server sends response (JSON or plain text)
  → dispatch MESSAGE_RECEIVED
  → ConsoleMessage renders:
      JSON object/array  → <JsonView>
      graph-link text    → 🕸️ pinnable row
      plain text         → plain text row
```

### 8.2 User Pins a Graph

```
User clicks 🕸️ row in Console → handlePinMessage(msg)
  → classificationMap.get(msg.id) → events[]
  → find event with kind === 'graph.link' → event.apiPath
  → setPinnedGraphPath(event.apiPath)
  → setPinnedMessageId(msg.id)   — highlights the row
  → useGraphData effect fires:
      fetch(event.apiPath)
      → transformGraphData(json)  — BFS layout + ReactFlow nodes/edges
      → setGraphData(result)
      → setRightTab('graph')      — tab switches automatically
```

### 8.3 Auto-Refresh After Mutation

```
User sends "create node foo" → ws.sendCommand()
Server echoes "> create node foo"   — ignored by classifier (command.echo kind)
Server responds "Node foo created"  — classifier emits graph.mutation event
  → useAutoGraphRefresh.on('graph.mutation'):
      waitingForDescribeRef = true  ← armed immediately at mutation-detection time
      arms 300 ms debounce timer
  → (300 ms passes, no further mutations)
  → waitingForDescribeRef = true  ← re-armed inside debounce callback
  → addToast('Graph updated — refreshing…')          [or '…opening Graph tab…' if no graph yet]
  → sendRawTextRef.current('describe graph')   — silent, no history entry

Server emits graph-link "Graph described in /api/graph/model/ws-123-2"
  → classifier emits graph.link event
  → useAutoGraphRefresh.on('graph.link'):
      waitingForDescribeRef === true
      → setPinnedGraphPath(event.apiPath)
      → waitingForDescribeRef = false

  → useGraphData initial-load path:
      fetch("/api/graph/model/ws-123-2")
      → setGraphData(result)
      → setRightTab('graph')
```

### 8.4 Large Payload Flow

```
User sends a command that returns a large payload
Server responds: "Large payload (254922) -> GET /api/inspect/ws-563-1/input.body"
  → useLargePayloadDownload detects via extractLargePayloadLink
  → isFetchingRef = true
  → fetch("/api/inspect/ws-563-1/input.body")
  → appendMessage(JSON.stringify(parsedJson, null, 2))
  → ConsoleMessage renders it as a <JsonView>
  → ➡️ button available: user can send to JSON-Path Playground
```

### 8.5 REST Upload Handshake

```
User pastes JSON in PayloadEditor, clicks Upload
  → ws.uploadPayload():
      pendingUploadRef = true
      ctx.send(wsPath, 'upload')

Server responds: "Please upload XML/JSON text to /api/json/content/ws-123-1"
  → classifier emits upload.contentPath event
  → useWebSocket.on('upload.contentPath'):  [bus-based path]
      pendingUploadRef === true
      → pendingUploadRef = false  (cleared synchronously before fetch)
      → fetch POST /api/json/content/ws-123-1  body: payload JSON
      → addToast('Payload uploaded successfully')

  (If bus is not provided, a legacy useEffect watches messages
   for extractUploadPath and follows the same logic)
```

### 8.6 Mock-Data Upload Flow

```
User sends "upload mock data" → ws.sendCommand()
Server responds: "You may upload JSON payload -> POST /api/mock/ws-417669-24"
  → classifier emits upload.invitation event
  → useAutoMockUpload.on('upload.invitation'):
      pendingModalRef guard check
      → extractMockUploadPath → "/api/mock/ws-417669-24"
      → handleOpenUploadModal("/api/mock/ws-417669-24")
          modalTriggerRef.current = document.activeElement   (command input)
          setModalUploadPath("/api/mock/ws-417669-24")        → modal mounts + showModal()

ConsoleMessage renders the invitation row with ⬆️ icon + "⬆️ Upload JSON…" button

User provides JSON (paste, drag-and-drop, or Browse file…):
  a. Paste / type → inline validation on every keystroke (tryParseJSON)
  b. Drop .json file onto drop zone → validateFileType → readFileAsText →
       tryParseJSON → formatJSON → textarea (focus restored)
  c. Click "Browse file…" → hidden <input type="file"> → same path as (b)

User clicks "Upload ▶" (or presses Ctrl+Enter / ⌘+Enter):
  → useMockUpload.upload()
      isUploading = true
      fetch POST /api/mock/ws-417669-24  Content-Type: application/json
      → 2xx:  isUploading = false → onSuccess(body)
                → setSuccessfulUploadPaths (adds path → ✅ badge on console row)
                → setModalUploadPath(null)  → modal unmounts
                → setTimeout: modalTriggerRef.current?.focus()
                → addToast('Mock data uploaded successfully ✓', 'success')
      → non-2xx: isUploading = false → onError('HTTP 400 — <body>')
                → setUploadError (inline banner) + addToast (error toast)
                → modal stays open for retry

User dismisses modal (Escape / Cancel / backdrop click):
  → handleClose → cancel() (AbortController.abort()) → onClose()
  → setModalUploadPath(null) → modal unmounts
  → setTimeout: focus restored to modalTriggerRef

User clicks "⬆️ Upload JSON…" re-open button on the console row:
  → handleOpenUploadModal(mockUploadPath) → modal re-opens for same endpoint
```

---

### 8.7 Save-Name Lifecycle

```
── Fresh session ─────────────────────────────────────────────────────
localStorage has no counter yet
  → useLocalStorage initialises untitledCounter = 1
  → defaultName = "untitled-1"

── User saves as "untitled-1" (connected) ───────────────────────────
handleSaveGraph("untitled-1"):   [in useSavedGraphWorkflow]
  pendingSaveRef.current = { graphName: "untitled-1", timeoutId }  // arm — no localStorage write yet
  ws.sendRawText("export graph as untitled-1") // server writes file
  → (server responds with graph.exported event, graphName === "untitled-1")
  → bus.on('graph.exported') fires (name-matched):
      clearTimeout(timeoutId)
      savedGraphs.saveGraph("untitled-1")      // localStorage bookmark written HERE
      setLastSavedName("untitled-1")           // in useGraphSaveName
        → setLastSavedNameState("untitled-1")
        → "untitled-1" === `untitled-${1}` → untitledSlotConsumedRef = true
      addToast(`Graph saved as "untitled-1"`)
  → defaultName = "untitled-1"                 // lastSavedName wins

── User clears console (slot was consumed) ───────────────────────────
handleClearMessages() → resetSaveName() → resetName():
  setImportedName(null)
  setLastSavedNameState(null)
  untitledSlotConsumedRef.current === true
    → setUntitledCounter(1 + 1 = 2)            // advances: slot was used
  untitledSlotConsumedRef.current = false
  → defaultName = "untitled-2"

── User clears console WITHOUT saving ────────────────────────────────
resetName():
  setImportedName(null)
  setLastSavedNameState(null)
  untitledSlotConsumedRef.current === false
    → counter stays at 2                        // slot NOT advanced: never used
  → defaultName = "untitled-2"                 // same slot reused

── User saves as "my-graph" (renaming away from untitled) ────────────
setLastSavedName("my-graph"):
  setLastSavedNameState("my-graph")
  "my-graph" !== `untitled-${2}` → flag stays false
  → defaultName = "my-graph"                   // lastSavedName wins
  → counter is still 2; "untitled-2" was never used

── User imports "other-graph" (supersedes lastSavedName) ─────────────
WS echo: "> import graph from other-graph"
  → extractImportGraphName → "other-graph"
  → setImportedName("other-graph")
  → setLastSavedNameState(null)               // lastSavedName cleared
  → defaultName = "other-graph"              // importedName wins
```

---

### 8.8 Create Node UI Flow

```
Entry point A: empty graph state
  GraphView renders "Create Node"
  → user clicks button
  → onCreateNode('empty-graph')
  → useGraphAuthoring.openCreateNode()
      create default form values for 'empty-graph'
        alias = "root"
        nodeType = "Root"
        properties = [empty row]

Entry point B: existing graph pane
  User right-clicks empty ReactFlow pane
  → GraphView.onPaneContextMenu(event)
      event.preventDefault()
      setPaneMenu({ x: event.clientX, y: event.clientY })
  → GraphContextMenu renders at pointer position
  → user clicks "Create Node"
  → onCreateNode('pane-context-menu')
  → useGraphAuthoring.openCreateNode()
      create default form values for 'pane-context-menu'
        alias = ""
        nodeType = ""
        properties = [empty row]

User edits modal fields
  → NodeDialog reports the updated form values
  → useGraphAuthoring stores the latest form values
      clears validation errors and server message

User clicks Create Node
  → useGraphAuthoring.submit()
      validate form values with current graph data
        checks alias, node type, flat property rows, duplicate alias, command budget
      buildCreateNodeCommand(formValues)
        create node {alias}
        with type {nodeType}
        with properties
        key=value
      executor.execute(command)
        → ws.sendRawText(command)
        → ctx.send(wsPath, command)
      state.phase = "sending"
      pendingSubmit = { alias, command, sentAt }
      start 10 s timeout

Backend response arrives over WebSocket
  → WebSocketContext dispatches MESSAGE_RECEIVED
  → useProtocolKernel.classifyMessage()
      "node root created"
        → graph.mutation
        → minigraph.createNode.textResult { status: "accepted", alias: "root" }

Accepted result
  → useGraphAuthoring bus handler matches alias
  → clear timeout
  → close modal
  → useAutoGraphRefresh sees graph.mutation
  → sends silent "describe graph"
  → graph.link response updates pinnedGraphPath
  → useGraphData fetches and renders the updated graph

Rejected/error result
  → useGraphAuthoring clears pendingSubmit
  → state.phase = "editing"
  → serverMessage shown in NodeDialog
  → user can edit and resubmit

Disconnect while modal is open
  → useGraphAuthoring detects connected → disconnected
  → clear timeout
  → connectionLost = true
  → all fields/buttons disabled except close/cancel
  → message tells the user to refresh/check graph before trying again
```

## 9. State Ownership Map

| State | Owner | Persistence | Notes |
|---|---|---|---|
| WS phase per slot | `WebSocketContext` reducer | Memory only | Resets on page reload |
| Messages per slot | `WebSocketContext` reducer | Memory only | Ring-buffered at 200 |
| WS refs (socket, ping, msgId) | `useRef` in Context | Memory only | Not in reducer (no renders) |
| Pending payload (cross-playground) | `useState` in Context | Memory only | `useState` (not `useRef`) for re-render trigger |
| Command input | `localReducer` in `useWebSocket` | Memory only | Resets on unmount — intentional |
| Command history | `useLocalStorage` in `useWebSocket` | `localStorage` | Keyed per playground |
| Payload text | `useLocalStorage` in `Playground` | `localStorage` | Keyed per playground |
| Payload override (large) | `useState` in `Playground` | Memory only | Never written to localStorage |
| Pinned message id | `useState` in `Playground` | Memory only | |
| Pinned graph path | `usePinnedGraphPath` hook (module-scope `Map` + `useState`) | Memory only (survives remount, resets on page reload) | Cleared on disconnect (Playground.tsx effect) and on `session.reset` (useAutoGraphRefresh handler) |
| Modal upload path | `useState` in `Playground` | Memory only | `null` = closed; non-null = modal open |
| Modal trigger element | `useRef` in `Playground` | Memory only | Captures active element before open for focus restore |
| Successful upload paths | `useState` in `Playground` | Memory only | `Set<string>`; cleared on `clearMessages` |
| `pendingSaveRef` | `useRef` in `useSavedGraphWorkflow` | Memory only | `PendingSave = { graphName, timeoutId }` while awaiting server confirmation; cleared on `graph.exported` (matched), `graph.export.failed`, timeout (10 s), or disconnect |
| `pendingJsonTransferRef` | `useRef` in `useSendToJsonPath` | Memory only | Last-write-wins mailbox; armed when ➡️ is clicked while JSON-Path is connecting; consumed once slot reaches `connected` |
| Clipboard items | `ClipboardContext` reducer | IndexedDB | Hydrated at mount; cross-tab sync via BroadcastChannel |
| `clipboardOpen` | `useLocalStorage` in `Playground` | `localStorage` | Key `'clipboard-sidebar-open'`; persists sidebar open/closed state |
| Duplicate dialog state | `useState` in `Playground` | Memory only | `null` = closed; non-null = `{ pendingItem, existingItem }` |
| Graph display name | `useMemo` in `Playground` | Memory only | Derived: root node `name` property → `graphSaveName` fallback; threaded as `graphName` prop |
| Graph authoring state | `useState` in `useGraphAuthoring` | Memory only | Closed/open modal state, form values, phase, pending submit, server message, connection-lost flag |
| Graph authoring validation errors | `useState` in `useGraphAuthoring` | Memory only | Field-keyed errors; cleared on form edit and accepted submit |
| Graph authoring submit timer | `useRef` in `useGraphAuthoring` | Memory only | 10-second timeout while a create-node submit is pending |
| Property row focus refs | `useRef` in `NodeDialog` | Memory only | Focuses the new key input after Add Property; not persisted or sent |
| Graph data | `useState` in `useGraphData` | Memory only | |
| Right panel tab | `useLocalStorage` in `useGraphData` | `localStorage` | Keyed per playground; normalized against the current `tabs` array before render so removed legacy values are migrated to a valid tab |
| Is refreshing | `useState` in `useGraphData` | Memory only | |
| Saved graph names | `useLocalStorage` in `useSavedGraphs` | `localStorage` | Keyed per playground |
| Untitled counter | `useLocalStorage` in `useGraphSaveName` | `localStorage` | Keyed per playground; only increments when slot was consumed by a save |
| Last-saved name | `useState` in `useGraphSaveName` | Memory only | Cleared on import or reset |
| Imported graph name | `useState` in `useGraphSaveName` | Memory only | Set from WS echo; cleared on reset |
| Untitled slot consumed | `useRef` in `useGraphSaveName` | Memory only | Write-only flag; never drives a re-render |
| Toasts | `useReducer` in `useToast` | Memory only | Auto-expires |
| Panel split ratio | `react-resizable-panels` | `localStorage` | Keyed per route path |
| Autocomplete dropup state | `useState` in `useHistoryAutocomplete` | Memory only | `isOpen`, `activeIndex`; resets on unmount — intentional |
| ProtocolBus | `useRef` in `Playground` | Memory only | Stable identity for component lifetime; never triggers re-renders |
| Classification map | `useMemo` in `useProtocolKernel` | Memory only | `Map<number, ProtocolEvent[]>` — re-derived when `messages` changes |
| Protocol watermark | `useRef` in `useProtocolKernel` | Memory only | Single centralised watermark; replaces per-hook watermarks |
| Pending modal ref | `useRef` in `useAutoMockUpload` | Memory only | First-match guard; reset by `modalOpen` clearing effect |

---

## 10. Build, Dev & Deploy

### Development

```bash
# Start the Java backend (in examples/minigraph-playground)
java -jar target/minigraph-playground-4.3.83.jar   # port 8085

# Start the Vite dev server (in webapp/)
npm run dev   # port 3000, proxies /ws and /api to 8085
```

Open `http://localhost:3000`. Hot module replacement is active.

### Testing

```bash
# Run all tests once
npm run test

# Run with watch mode during development
npm run test:watch
```

Tests use Vitest 3 with the built-in `node` environment and `globals: true` (no explicit imports needed for `describe`/`it`/`expect`). The tests are pure TypeScript logic with no DOM or React rendering dependency, so no external testing library is required. Test files live alongside source in `__tests__/` directories. See `vitest.config.ts` for configuration.

### Production release

```bash
npm run release
# Equivalent to:
npm run clean
npm run build    # → dist/
npm run deploy   # → copies dist/ → ../src/main/resources/public/
```

The Java jar is then rebuilt and serves the SPA from the same origin as the WebSocket and REST endpoints — no CORS, no proxy needed.

### Bundle splitting (`vite.config.ts`)

Manual chunks keep each heavy vendor isolated:

| Chunk | Content |
|---|---|
| `vendor-xyflow` | `@xyflow/react` (largest dependency) |
| `vendor-router` | `react-router-dom` |
| `vendor-markdown` | `react-markdown` + `remark-gfm` |
| `vendor-json-view` | `react-json-view-lite` |
| `vendor-panels` | `react-resizable-panels` |

Source maps are enabled for production (`sourcemap: true`).

---

## 11. Extending the App

### Add a new playground

1. Add an entry to `PLAYGROUND_CONFIGS` in `src/config/playgrounds.ts`
2. That's it. The route, nav link, localStorage namespace, right-panel tabs, and connection management are all derived automatically.

### Add a new right-panel tab type

1. Add the new key to the `RightTab` union in `RightPanel.tsx`
2. Add the tab button and panel in `RightPanel.tsx`
3. Reference it in `tabs:` arrays in `PLAYGROUND_CONFIGS`

### Add a new node type to the graph

1. Add the type string to `NODE_ACCENT` in `graphTransformer.ts` (gives it an accent colour)
2. Add the type to `TYPE_META` in `NodeTypes.tsx` (gives it an icon and label)
3. Add the type to the `nodeTypes` export in `NodeTypes.tsx` (maps it to `MinigraphNode`)
4. (Optionally) add it to the MiniMap `colorMap` in `GraphView.tsx`
5. If the new type should be **segregated when orphaned** (not connected to any edge), add it to `SEGREGATED_ROW_ORDER` in `graphTransformer.ts` and handle it in `classifyNode()`. Connected nodes of any type always participate in the main BFS flow regardless of classification

### Add a new mutation command to auto-refresh

Edit `detectMutation()` in `messageParser.ts`. Follow the existing patterns — always guard with `!isMarkdownCandidate` (skip JSON), `!raw.startsWith('> ')` (skip echoes), and `!isGraphLinkMessage` (skip graph links). Add the new success message pattern as a return case.

The Protocol Kernel's `classifier.ts` calls `detectMutation()` internally, so no changes to the classifier are needed for new mutation patterns — they will automatically emit `graph.mutation` events.

### Add a new message type to the Protocol Kernel

1. Add a new event variant to the `ProtocolEvent` union in `src/protocol/events.ts`. `ProtocolEventKind` is `ProtocolEvent['kind']` — it updates automatically; no separate step needed.
2. Add a classification rule in `src/protocol/classifier.ts` (before Rule 12, the unclassified fallback). If the rule matches a static string, export a constant for the wire value from `classifier.ts` so tests import the same string.
3. Add a test fixture JSON file in `src/protocol/__tests__/fixtures/`
4. Add golden transcript test cases in `src/protocol/__tests__/classifier.test.ts`
5. Subscribe to the new event kind in the appropriate hook via `bus.on('new.kind', handler)`

### Extend the create-node UI surface

1. Add the new field to the create-node form model in `src/graphActions/nodeAuthoringTypes.ts`
2. Initialise it in the default form-value helper in `propertyRows.ts`
3. Add frontend validation in `validation.ts`
4. Update `buildCreateNodeCommand()` in `minigraphCommandBuilder.ts` to serialise it into the backend's accepted command grammar
5. Render/edit the field in `NodeDialog.tsx`
6. Add or update `graphActions/__tests__` coverage for validation and command serialization

Do not add backend-command string concatenation inside `NodeDialog` or `useGraphAuthoring`; keep `minigraphCommandBuilder.ts` as the single serialization boundary.

---

## 12. Pitfalls & Gotchas

### P1 — Do not move `WebSocketProvider` inside `<BrowserRouter>`
It must stay *above* the router. Inside the router it would unmount on every navigation, closing all sockets. See §7.1.

### P2 — Do not put WS instances in React state
`WebSocket` instances are imperative handles that must be available synchronously. In state they would cause render cycles on every incoming message. They belong in `useRef`. See §7.2.

### P3 — Message IDs are not array indices
The ring buffer drops old messages. Always identify pinned/processed messages by their stable `id`, never by their position in the array. See §7.3.

### P4 — `detectMutation` requires the `startsWith('node ')` prefix
Without this guard, `"Graph instance created"` and `"Root node created because..."` trigger false-positive auto-refreshes. Do not remove the prefix check. See §3.5 and §6.10.

### P5 — `sendRawText` must be used for silent commands, not `sendCommand`
`sendCommand` pushes to history and echoes the command — the echo would trigger `useAutoHelpNavigate`'s `describe` guard and open the help panel unexpectedly. `sendRawText` bypasses all that. Create-node UI authoring also uses `sendRawText`; keep checking its boolean return so the modal can remain open when the socket is unavailable. See §7.5 and §7.12.

### P6 — The centralised watermark in `useProtocolKernel` must initialise before the emission effect
The watermark init effect ([L40](../src/protocol/useProtocolKernel.ts#L40)) must run *before* the bus emission effect ([L56](../src/protocol/useProtocolKernel.ts#L56)) at mount. React guarantees effects in a component run in declaration order on the first render, so declaring the watermark init first works. Merging them into a single effect would process historical messages as new events. See §7.4.

### P7 — `payloadOverride` must be cleared on manual edits
The `setPayload` callback does `setPayloadOverride(null)` before calling `setStoredPayload`. If you add a new code path that modifies the payload, ensure it also clears the override so the user's edit is not silently discarded. See §7.10.

### P8 — `refetchGraph` has an intentionally empty dependency array
It reads `pinnedGraphPath` via `pinnedGraphPathRef` to avoid stale closures. Adding `pinnedGraphPath` to its dep array would break the "stable reference" contract that lets automation hooks include it in their own dep arrays safely. See §7.7.

### P9 — Large payloads must never be written to localStorage
The payload override pattern exists precisely for this reason. Never call `setStoredPayload` (the localStorage-backed setter) with a large blob. See §7.10.

### P10 — ReactFlow node type must be in `nodeTypes` map
If the backend sends a node whose `types[0]` value is not a key in the `nodeTypes` map in `NodeTypes.tsx`, ReactFlow will render a default node without the custom styling. Add new types to all four places listed in §11.

### P11 — `isPinnable` must exclude mock-upload and large-payload rows
The upload invitation is plain text, so `isMarkdownCandidate` returns `true` for it. Without the `&& !isMockUpload && !isLargePayload` guard in the `isPinnable` derivation in `ConsoleMessage.tsx` ([L51](../src/components/Console/ConsoleMessage.tsx#L51)), the row would receive `role="button"` and `onClick` *alongside* the "⬆️ Upload JSON…" re-open button — creating nested interactive elements that violate WCAG. Always keep both exclusions. See §3.12 and §6.5.

### P12 — `docs.response` classifier rule must exclude all non-docs message types
The `docs.response` classification rule in `classifier.ts` excludes echoes, graph-links, mock-upload invitations, large-payload notices, JSON responses, and lifecycle messages. If a new message type is added that is plain text, it must also be excluded from `docs.response` to prevent it from being stolen as an auto-pin target. See §6.8 and §6.9.

> **Note on coexistence:** the `session.reset` event (Rule 7c) and `docs.response` (Rule 11) intentionally fire together for the `"Session restarted"` message — the console still renders it as text, and the hook receives the `session.reset` event to act on it. This dual-event pattern is consistent with how `graph.mutation` and `minigraph.createNode.textResult` coexist for the same message.

### P13 — Bus subscription callbacks must read mutable state through refs
Because `bus.on()` registers a callback only once (the bus `useRef` identity never changes), any React state read inside the callback would be stale after subsequent renders. All automation hooks use ref-wrapping (`connectedRef`, `sendRawTextRef`, `pinnedGraphPathRef`, callback refs) to access current values. Adding a new bus subscriber that reads state directly from closure will produce stale-closure bugs. See §7.11.

### P14 — `classificationMap` identity changes on every new message
The `classificationMap` is a `useMemo` that re-derives when `messages` changes. Components that receive it as a prop will see a new `Map` reference on every new WebSocket message. This is intentional — React Compiler handles memoisation — but avoid using `classificationMap` as a `useEffect` dependency without wrapping the effect in additional guards, as it will fire on every message.

### P15 — Never allow saves while disconnected
`useSavedGraphs` stores only the graph **name**, not the graph data. A bookmark without a matching server-side `{name}.json` export file cannot be loaded — `import graph from {name}` will fail after reconnect. `useSavedGraphWorkflow.handleSaveGraph` guards against disconnected calls with an early-return error toast; `GraphSaveButton` enforces `disabled={disabled || !connected}` at the UI level. If you add any new path that calls `saveGraph(name)` directly, ensure the caller is always connected first. See §3.10 and §6.12.

### P16 — Keep create-node serialization out of UI components
`NodeDialog` must stay a typed form editor. If a future field is added, update the form model, validation, and `buildCreateNodeCommand()` together. String-building command text in JSX or in event handlers will bypass validation, command-size checks, and tests. See §6.13 and §7.12.

### P17 — Create-node result matching is text-based and alias-scoped
`useGraphAuthoring` closes the modal only when a `minigraph.createNode.textResult` accepted/rejected event matches the pending alias. If backend success/error wording changes, update `parseCreateNodeTextResult()` and its classifier tests. Do not make the hook infer success from graph refresh alone; graph refresh is a downstream side effect, not submit confirmation. See §3.16 and §8.8.

### P18 — The create-node modal must block panel resizing underneath
The fixed overlay's `onPointerDown` prevents underlying `react-resizable-panels` handles from receiving drag gestures while the modal is open. If the modal structure changes, verify that resize handles under the overlay cannot move. See §6.13.

---
