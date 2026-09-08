# Epic 5 Visual Edge Connection UI Spec

**Audience:** frontend and full-stack engineers implementing visual connection authoring in the Minigraph playground
**Status:** implemented and verified
**Scope:** Minigraph playground graph UI only

---

## 1. Goal

Allow a user to create a graph connection directly from the visual graph:

1. Hover a node.
2. See a subtle, border-aligned connection outline on the node edge.
3. Drag a connection line from that node to another node.
4. Release on the target node.
5. Confirm the connection in a modal by selecting a relation from the app-supported connection list.

The implementation must use the existing React Flow graph primitive and existing WebSocket raw command path. It must not add a new backend API or optimistically persist graph state in the frontend.

---

## 2. User-Facing Requirements

### 2.1 Hover Connection Affordance

When graph authoring is supported and the WebSocket is connected:

- hovering a node shows a subtle connection outline that sits on the node border rather than outside the node shell;
- the outline must read as a connection affordance, not as node selection, resize state, or disabled state;
- the outline must not resize the node or shift node content;
- the outline must not cover node text in a way that makes it unreadable;
- the outline must not appear while the node is being resized;
- the outline must remain visually compatible with existing selected-node and edge styles;
- authoring drag hot zones may extend slightly past the left/right edge for hit testing, but their visible treatment must stay minimal: no broad translucent side blocks or large tinted overlay around the node.

When authoring is unsupported or disconnected:

- do not expose connection drag handles;
- do not show a connection outline;
- existing graph view, context menu, clipboard, pan, zoom, and resize behavior remain unchanged.

### 2.2 Drag To Connect

When the user drags from the connection affordance on one node to another node:

- React Flow shows its connection line while dragging;
- dropping on a valid target node opens the connection confirmation modal;
- dropping on empty canvas, the same node, or an invalid target cancels without opening a modal;
- no backend command is sent until the modal is confirmed;
- no optimistic edge is added before backend acceptance and graph refresh.

The connection is directional: the node where the drag starts is the source, and the node where the drag ends is the target.

### 2.3 Confirmation Modal

After a valid source-to-target drag:

- open a modal using the same overlay/focus-blocking interaction pattern as the existing node dialog;
- show source node alias as read-only;
- show target node alias as read-only;
- show exactly one editable field: relation;
- relation is a select list, not free text;
- relation must look editable when enabled: use the normal input surface/text treatment, not the muted disabled/read-only styling used for Source and Target;
- the primary action sends the connection command;
- Cancel, Escape, or backdrop click closes the modal without sending;
- the modal remains open and locked while sending;
- the modal surfaces backend rejection, timeout, send failure, and disconnect messages.

Modal fields:

```text
Source: {sourceAlias}
Target: {targetAlias}
Relation: [select]
```

Suggested title:

```text
Create Connection
```

Suggested primary action:

```text
Create Connection
```

### 2.4 Relation Select Options

The relation select options come from app code, not from the current graph.

Create a shared relation option constant, for example:

```ts
export const CONNECTION_RELATION_OPTIONS = [
  'fetch',
  'details',
  'ext-call',
  'mapping',
  'compute',
  'calculate',
  'evaluate',
  'fork',
  'join',
  'one',
  'two',
  'three',
  'more',
  'done',
  'complete',
  'finish',
  'positive',
  'negative',
] as const;
```

The current edge-color map in `webapp/src/utils/graphTransformer.ts` already knows this relation set. Move the shared list and color map into one module, then make both graph rendering and the connection modal consume that module.

Imported graphs or console-created graphs may still contain relation types outside this list. Rendering must continue to support unknown relation types with the existing fallback color behavior. The UI select only offers app-supported relation choices.

### 2.5 Success Feedback

On accepted backend result:

- close the modal;
- show the same class of user feedback used by node authoring success;
- rely on the existing graph mutation auto-refresh path to fetch and render the updated graph;
- do not add a temporary frontend edge.

Expected backend success text:

```text
node {sourceAlias} connected to {targetAlias}
```

---

## 3. Explicit Non-Goals

- No backend command grammar change.
- No REST endpoint for connection creation.
- No free-text custom relation input in the first implementation.
- No connection edit UI.
- No connection delete UI.
- No edge reconnect UI for existing edges.
- No optimistic graph mutation in frontend state.
- No persistent modal state in localStorage, sessionStorage, or IndexedDB.
- No new runtime dependencies.
- No broad redesign of graph layout.

---

## 4. Existing Code References

Relevant existing implementation points:

- `webapp/src/components/GraphView/GraphView.tsx:240` renders `<ReactFlow>` and owns graph event wiring.
- `webapp/src/components/GraphView/NodeTypes.tsx:19` renders custom Minigraph nodes.
- `webapp/src/components/GraphView/NodeTypes.tsx:25` renders existing edge handles.
- `webapp/src/components/GraphView/NodeTypes.module.css:103` hides existing edge handles and disables pointer events.
- `webapp/src/utils/graphTransformer.ts:67` defines relation-to-edge-color knowledge.
- `webapp/src/clipboard/commandBuilder.ts:62` already builds `connect {source} to {target} with {relation}`.
- `webapp/src/graphActions/minigraphCommandBuilder.ts:31` centralizes UI authoring command serialization for node actions.
- `webapp/src/components/GraphAuthoring/useGraphAuthoring.ts:123` owns authoring lifecycle, validation, send, timeout, and disconnect handling.
- `webapp/src/utils/messageParser.ts:385` detects connection creation text as a graph mutation.
- `src/main/java/com/accenture/minigraph/services/GraphCommandService.java:980` handles backend `connect` commands.
- `src/main/java/com/accenture/minigraph/services/GraphCommandService.java:1157` rejects same-node and missing-node connections.

---

## 5. Architecture

### 5.1 Selected Architecture

Use React Flow's native connection primitive:

- add dedicated connection-authoring handles to each Minigraph node;
- keep existing edge-spreading handles for rendered edges separate from authoring handles;
- wire `onConnect`, `onConnectStart`, `onConnectEnd`, and `isValidConnection` at the `<ReactFlow>` level;
- open a connection confirmation modal from `onConnect`;
- submit through the existing raw WebSocket command executor.

This keeps hit testing, connection line rendering, pointer capture, and pan/zoom coordinate handling inside React Flow.

### 5.2 Authoring Handles

Existing rendered-edge handles are generated per existing connection and are intentionally non-interactive. They must remain non-interactive.

Add a separate pair of authoring handles per node:

- one source handle for outbound connection drags;
- one target handle for inbound drops;
- the source handle may start but not receive a connection;
- the target handle may receive but not start a connection;
- stable handle IDs, for example `authoring-source` and `authoring-target`;
- rendered only when visual authoring is enabled, or rendered inert when disabled;
- visually styled as the hover connection affordance rather than tiny dots;
- keep the visible affordance tight to the node edge; avoid broad translucent side blocks that make the node look selected, disabled, or wrapped by a separate overlay.

The authoring handles must not change the existing handle IDs used for rendered edges, because existing edge rendering depends on `sourceHandle` and `targetHandle` from `transformGraphData`.

### 5.3 Validation

Frontend validation before opening or submitting the modal:

- source alias exists in current `graphData.nodes`;
- target alias exists in current `graphData.nodes`;
- source alias and target alias differ;
- selected relation is one of `CONNECTION_RELATION_OPTIONS`;
- WebSocket is connected;
- no other graph authoring action is pending.

Backend remains authoritative and may reject stale graph data.

### 5.4 Command Contract

Producer: frontend graph authoring command builder
Consumer: `GraphCommandService.handleConnectCommand`

```text
connect {sourceAlias} to {targetAlias} with {relation}
```

Rules:

- `{sourceAlias}` and `{targetAlias}` come from React Flow connection `source` and `target`.
- `{relation}` comes from `CONNECTION_RELATION_OPTIONS`.
- all three tokens must satisfy the existing node-name token rule: letters, numbers, underscore, and hyphen.
- command size is validated with the existing command size guard.

Expected backend responses:

```text
node {sourceAlias} connected to {targetAlias}
node {alias} not found
Source and target nodes must be different
Syntax: connect {node-A} to {node-B} with {relation}
ERROR: {message}
```

The exact same raw text should also continue to work from the console.

### 5.5 Parser And Protocol Events

Extend the existing node action result parsing to include connection authoring.

Add action type:

```ts
type NodeActionTextResultAction =
  | 'create-node'
  | 'edit-node'
  | 'delete-node'
  | 'create-connection'
  | null;
```

Parse:

```text
node {sourceAlias} connected to {targetAlias}
```

as:

```ts
{
  status: 'accepted',
  action: 'create-connection',
  alias: sourceAlias,
  targetAlias,
  message: rawText,
}
```

Pending connection success is resolved only when both `alias` and `targetAlias` match the submitted source and target. This prevents an unrelated same-source result from closing the modal. The existing graph mutation detector already treats `node ... connected to ...` as `node-mutation`, so auto-refresh remains the graph synchronization mechanism.

### 5.6 State Ownership

Backend MiniGraph is the source of truth for graph structure.

Frontend state:

- `graphData`: fetched projection of backend graph;
- React Flow `nodes` and `edges`: derived render state from `graphData`;
- pending connection draft: memory-only UI state containing source alias, target alias, selected relation, phase, and server message;
- relation options: code-defined constant;
- validation errors: memory-only modal state.

No graph structure is persisted from the frontend.

### 5.7 UI Composition Guardrails

Graph interactions already include node resize, node context menu, pane context menu, clipboard drag/drop, pan, zoom, minimap, and refresh overlay. The new connection flow must preserve those behaviors.

Guardrails:

- `NodeResizer` remains visible only on selected nodes and must not be hidden by the connection outline.
- connection handles must not activate while a node resize is in progress.
- node context menu continues to use right-click and is not triggered by connection drag.
- pane context menu remains right-click only.
- clipboard drag/drop still uses DataTransfer type guards and does not treat React Flow connection drag as clipboard drag.
- modal overlay absorbs pointer events so graph drag/resize cannot continue underneath.
- Escape closes the modal only when it is not sending.
- dropping a connection on invalid targets does not show an error toast unless the user had already opened the modal and submitted.

---

## 6. Detailed Implementation Plan

### Slice 1 - Shared Connection Relation Contract

Files:

- `webapp/src/graphActions/connectionAuthoringTypes.ts` or equivalent
- `webapp/src/graphActions/connectionRelations.ts` or equivalent
- `webapp/src/utils/graphTransformer.ts`
- `webapp/src/graphActions/minigraphCommandBuilder.ts`
- `webapp/src/graphActions/validation.ts`

Work:

- introduce `CONNECTION_RELATION_OPTIONS`;
- introduce `ConnectionFormState` with `sourceAlias`, `targetAlias`, and `relation`;
- move relation color lookup to use the shared relation registry;
- add `validateConnectionFormState`;
- add `buildCreateConnectionCommand`;
- keep fallback edge coloring for unknown imported relation types.

Tests:

- command builder emits `connect root to end with done`;
- builder rejects invalid aliases, self-connections, missing relation, and unsupported relation;
- relation options and known color keys stay in sync.

Rollback:

- remove the new files and restore `graphTransformer.ts` local color map.

### Slice 2 - Parser And Authoring Lifecycle

Files:

- `webapp/src/utils/messageParser.ts`
- `webapp/src/protocol/events.ts`
- `webapp/src/components/GraphAuthoring/useGraphAuthoring.ts`
- `webapp/src/components/GraphAuthoring/GraphAuthoringModals.tsx`

Work:

- extend parser action union with `create-connection`;
- parse backend success text with both source and target aliases;
- extend authoring lifecycle to open connection modal, submit, match both endpoints, timeout, and disconnect;
- preserve the single pending authoring action rule across node and connection actions.

Tests:

- parser recognizes `node root connected to mapper`;
- classifier emits both `graph.mutation` and `minigraph.nodeAction.textResult` for accepted connection creation;
- pending lifecycle handles accepted, rejected, timeout, disconnect, and send-false paths.
- a success for the same source but a different target does not resolve the pending connection.

Rollback:

- remove `create-connection` action branch; node authoring remains unchanged.

### Slice 3 - Connection Confirmation Modal

Files:

- `webapp/src/components/ConnectionDialog/ConnectionDialog.tsx`
- `webapp/src/components/ConnectionDialog/ConnectionDialog.module.css`
- `webapp/src/components/GraphAuthoring/GraphAuthoringModals.tsx`

Work:

- build a presentational modal for connection confirmation;
- show read-only source and target aliases;
- render a relation select from `CONNECTION_RELATION_OPTIONS`;
- style the enabled relation select as an editable control; reserve muted disabled styling for disconnected/sending locks and for the read-only Source/Target fields;
- focus the select on open;
- disable controls during sending or disconnect lock;
- render validation and server messages consistently with `NodeDialog`.

Tests:

- state-only tests for form update and submit intent where practical;
- manual browser QA for focus, Escape, backdrop, and disabled sending state.

Rollback:

- modal is only mounted by the new connection authoring branch.

### Slice 4 - React Flow Wiring And Node Handles

Files:

- `webapp/src/components/GraphView/GraphView.tsx`
- `webapp/src/components/GraphView/NodeTypes.tsx`
- `webapp/src/components/GraphView/NodeTypes.module.css`
- `webapp/src/utils/graphTransformer.ts`
- `webapp/src/components/Playground.tsx`

Work:

- pass `supportsConnectionAuthoring` or equivalent into graph node data;
- render authoring source/target handles separately from existing edge handles;
- allow drags to start only from source handles and end only on target handles;
- style hover outline and authoring handle area as a border-aligned affordance with minimal side indicators;
- add `onConnect`, `onConnectStart`, `onConnectEnd`, and `isValidConnection`;
- call `graphAuthoring.openCreateConnection(sourceAlias, targetAlias)` on valid connect;
- reject same-node and missing-node connections before modal open;
- disable connection authoring when disconnected or unsupported.

Tests:

- unit-level validation for same-node and unsupported relation paths;
- TypeScript build catches React Flow connection type wiring;
- manual browser QA covers hover outline, drag line, modal open, invalid drop, resize interaction, right-click menu, clipboard drop, pan, and zoom.

Rollback:

- remove authoring handles and React Flow connect callbacks; graph rendering remains read-only.

---

## 7. Failure Matrix

| Failure path | User-visible result | State cleanup |
|---|---|---|
| Same source and target | No modal opens, or modal blocks submit if stale state reaches it | Draft discarded or validation error retained |
| Source node missing from current graphData | No modal opens; optional toast says node is no longer available | Draft discarded |
| Target node missing from current graphData | No modal opens; optional toast says node is no longer available | Draft discarded |
| Relation not selected | Modal shows relation validation error | Modal remains editable |
| Unsupported relation value | Modal shows validation error | Modal remains editable |
| WebSocket disconnected before drag | No connection affordance | No draft |
| WebSocket disconnected while modal editing | Modal locks and shows disconnect message | Draft retained but not editable |
| WebSocket disconnected while pending | Outcome unknown message | Pending cleared |
| `sendRawText` returns false | Modal shows send failure | Modal remains editable |
| Backend says node not found | Modal or toast shows backend message | Pending cleared |
| Backend syntax or error response | Modal or toast shows backend message | Pending cleared |
| Timeout | Modal shows outcome unknown | Pending cleared |
| User cancels | Modal closes without send | Draft discarded |
| Drop on empty canvas | No modal, no toast | Draft discarded |
| Drop on invalid target | No modal, no toast | Draft discarded |
| Concurrent authoring action pending | Toast asks user to wait | Existing pending action remains owner |
| Unrelated success for the same source and a different target | No modal/state change | Existing pending action remains owner |

---

## 8. Security And Trust

Inputs:

- source alias from React Flow connection event;
- target alias from React Flow connection event;
- relation selected by user from app-defined options;
- backend text result from WebSocket.

Validation:

- source and target are checked against current `graphData`;
- self-connection is rejected;
- relation must be one of the shared option constants;
- command serialization is centralized in `minigraphCommandBuilder`;
- backend remains authoritative for stale graph data.

Encoding:

- command fields are token-like values only;
- no multiline input;
- no HTML rendering of user input except React text nodes;
- backend messages are displayed as text in modal/toast.

Authorization:

- unchanged from existing playground WebSocket session model.

Abuse:

- only one authoring action can be pending at a time;
- no looped retries;
- no polling introduced.

---

## 9. Observability And Rollback

Debugging surfaces:

- command echo in console, if backend echo is enabled;
- backend success/error text in console;
- `ProtocolBus` `graph.mutation` event for auto-refresh;
- `minigraph.nodeAction.textResult` event for pending authoring resolution;
- user-visible toast or modal message for failures.

No new metrics are required.

Rollout:

- gated by existing `supportsAuthoring` and `isConnected`;
- no backend rollout required;
- old clients and console commands continue to work;
- rollback is removing the UI path and leaving backend command behavior untouched.

Data migration:

- none.

---

## 10. Verification Plan

Automated tests:

- `webapp/src/graphActions/__tests__/minigraphCommandBuilder.test.ts`
  - `buildCreateConnectionCommand` success and validation failure cases.
- `webapp/src/graphActions/__tests__/validation.test.ts`
  - source/target existence, self-connection, missing relation, unsupported relation.
- `webapp/src/utils/__tests__/messageParser.createNodeTextResult.test.ts`
  - accepted connection creation parsing.
- `webapp/src/protocol/__tests__/classifier.test.ts`
  - connection success emits mutation and node-action result events.
- `webapp/src/components/GraphAuthoring/useGraphAuthoring.test.ts`
  - accepted/rejected, exact endpoint matching, timeout, disconnect, and send-false lifecycle paths.

Build checks:

```bash
npm run test
npm run typecheck
npm run build
```

Manual QA:

1. Hover a node and confirm the connection outline sits on the node border without layout shift or broad translucent side blocks.
2. Drag from source node to target node and confirm modal opens.
3. Try to start a drag from a target handle and confirm no modal opens.
4. Confirm the enabled Relation select uses normal editable text/background styling, while read-only Source/Target remain visually muted.
5. Select each relation category type and confirm command submit for at least one relation.
6. Drop connection on empty canvas and confirm no modal opens.
7. Try same-node connection and confirm it is blocked.
8. Confirm node resize still works.
9. Confirm right-click node context menu still works.
10. Confirm pane context menu still works.
11. Confirm clipboard drag/drop still works.
12. Confirm pan, zoom, minimap, and refresh overlay still work.
13. Disconnect WebSocket and confirm affordance disappears.
14. Disconnect while modal is open or pending and confirm user-visible failure state.

---

## 11. Decision Log

| Decision | Alternatives considered | Why selected | Revisit if |
|---|---|---|---|
| Use React Flow connection primitive | custom SVG overlay, click-source/click-target flow | avoids custom coordinate mapping and uses existing graph interaction model | React Flow handles conflict with resize in implementation |
| Use code-defined relation list | current graph relation extraction, free text | gives user a stable app-supported choice list | product needs custom relation authoring |
| Reuse raw WebSocket command path | new REST/API path | backend command already exists and console remains contract source | backend exposes typed graph mutation API |
| No optimistic edge | immediate temporary edge | backend is source of truth and auto-refresh already exists | backend returns typed mutation result with correlation |
| Single pending authoring action | separate node and connection queues | avoids mixed modal/command outcomes | UX requires queued batch authoring |

---

## Appendix - UI Loop Engineer Planning Artifact

**Phase 0 - Domain calibration**
- Domain mix: Frontend UI / rendering 45%, State management / data flow 25%, Product workflow / UX 20%, Network protocol / RPC 10%.
- Dominant failure mode: Primitive overshoot or composition bugs, especially React Flow connection drag overlapping resize, context menus, clipboard drag/drop, and modal focus handling.
- Pre-mortem watch-items: unguarded graph interaction composition; stale frontend `graphData` causing backend rejection; relation select drifting from edge rendering knowledge.
- Calibration: Emphasize Step 9 composition, Step 5 source of truth, Step 6 command contract, and Step 17 state cleanup.

**Step 1 - Requirement and journeys**
- Actor / trigger / success signal: graph authoring user hovers a node, drags to another node, confirms relation, and sees the edge after backend acceptance and graph refresh.
- Primary journeys: successful create; cancel modal; invalid drop; disconnected graph; backend stale-data rejection.
- Non-goals: backend changes, free-text relation input, connection edit/delete, optimistic edge, new dependencies.

**Step 3 - Anti-anchoring and compatibility**
- Silent assumptions: node alias is React Flow node id; backend graph is source of truth; `graphData` is a staleable projection; raw WebSocket command is current authoring boundary; graph mutation text triggers auto-refresh.
- Fresh-team delta: a fresh team might build a canvas overlay, but this codebase already uses React Flow handles and should keep that primitive.
- Existing behavior contract: graph rendering extended; command grammar unchanged; node authoring lifecycle extended; context menus unchanged; clipboard drag/drop unchanged; auto-refresh unchanged.

**Step 4 - Three architectures**
- Option A - root primitive: click-source/click-target state machine.
- Option B - root primitive: custom SVG/pointer overlay.
- Option C - root primitive: React Flow handles and `onConnect`.
- Picked: Option C.
- Axis spread verified: options differ by interaction primitive, coordinate/rendering ownership, and abstraction level.

**Step 5 - Source of truth and boundaries**
- Source-of-truth inventory: backend owns graph; frontend `graphData` is projection; React Flow elements are derived; connection draft is memory-only; relation options are code-defined constants.
- Boundary map: UI boundary through modal and graph events; WebSocket boundary through raw command; async boundary through pending result and timeout; trust boundary through frontend validation plus backend authority.

**Step 6 - Contracts**
- Boundary-crossing contracts: `connect {sourceAlias} to {targetAlias} with {relation}` command; backend text success/rejection; `minigraph.nodeAction.textResult` event; `graph.mutation` event.
- Shared constants / schema strategy: shared `CONNECTION_RELATION_OPTIONS` and relation color metadata; command builder owns serialization.

**Step 9 - Primitive fit and composition**
- Under-use / over-use findings: React Flow already provides connection primitives, so custom pointer hit testing is under-use of platform capability.
- Composed primitives: React Flow handles, NodeResizer, context menus, clipboard drag/drop, modal overlay, ProtocolBus.
- Overlaps: pointer drag vs resize; connection drag vs clipboard drag; modal overlay vs graph pointer events; result matching vs auto-refresh.
- Guardrails: separate authoring handles; graph-level `isValidConnection`; DataTransfer type guard; overlay pointer absorption; single pending authoring action.

**Step 10 - From-scratch comparison**
- Materially simpler?: No.
- If yes, redesign adopted: Not applicable.

**Step 11 - Failure and lifecycle**
- Failure matrix summary: validation failure, disconnect, send false, backend rejection, timeout, cancellation, invalid drop, duplicate pending action, and stale data all have user-visible outcomes.
- Lifecycle data-access findings: React Flow connection event supplies source/target after valid drop; refs/timers belong to hook cleanup; modal reads state after open, not during render side effects.

**Step 12 - Spike / performance**
- Spike required?: Yes, because authoring handles are per node.
- Result or reason not required: Existing largest fixture has 20 nodes and 9 connections; adding two authoring handles per node adds 40 elements compared with 18 existing edge handle elements.
- Per-N cost: O(N + E) DOM handles; no per-potential-edge expansion.

**Step 13 - Security / trust**
- Input boundaries: React Flow event aliases, relation select value, backend text response.
- Validation / auth / encoding: aliases checked against current graph and backend; relation constrained to code constants; command tokens centralized; auth unchanged from WebSocket session.

**Step 14 - Observability / rollout**
- Debuggability: console command echo/result, ProtocolBus mutation event, node-action text result, modal/toast messages.
- Rollout / rollback: gated by `supportsAuthoring` and connection status; no data migration; rollback removes UI wiring only.

**Step 15 - Verification**
- Verification mapping summary: validation and command builder unit tests; parser/classifier unit tests; hook lifecycle tests; build checks; manual React Flow interaction QA.
- Design cross-reference complete: each verification row maps to shared relation contract, command builder, parser/event extension, authoring hook, modal, or GraphView wiring.

**Step 16 - Implementation / consumer readiness**
- Implementation slices: shared relation contract; parser/lifecycle; modal; React Flow wiring.
- Review questions / decision log: reviewers should approve relation option source, no custom relation input, no optimistic edge, and React Flow primitive fit.

**Step 17 - Pre-draft self-check**
- All self-check answers yes?: Yes.
- Enumeration completeness: draft creation, mutation, submit, accepted result, timeout, disconnect, cancel, invalid drop, and cleanup are covered.
- Cross-iteration regression: preserves node authoring command boundary, modal overlay pattern, graph mutation auto-refresh, and no-new-dependency constraint from prior UI specs.
