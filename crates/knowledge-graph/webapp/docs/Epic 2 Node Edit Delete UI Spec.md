# Epic 2 Node Edit/Delete UI Spec

**Audience:** frontend and full-stack engineers implementing node edit/delete in the Minigraph playground  
**Status:** implementation-ready  
**Scope:** Minigraph playground only

---

## 1. Goal

Add two node-level actions to the existing graph node context menu:

1. **Edit Node** - opens the same modal pattern used by Create Node, pre-populated from the selected node.
2. **Delete Node** - opens an inline confirmation state inside the context menu, then sends a delete command only after the user confirms.

The implementation must extend the current create-node authoring layer instead of building a separate transport path.

---

## 2. User-Facing Requirements

### 2.1 Node Context Menu

When a graph is imported or otherwise loaded and the user right-clicks a node:

- show the context menu at the mouse pointer position;
- keep the existing **Clip to Clipboard** menu item when clipboard support is available;
- add **Edit Node**;
- add **Delete Node**;
- disable or omit edit/delete when the WebSocket is disconnected;
- close the menu on Escape, outside click, pane click, scroll, or window resize.

The pane context menu remains separate and continues to show only **Create Node**.

### 2.2 Edit Node

When the user selects **Edit Node**:

- close the context menu;
- open the node modal in edit mode;
- pre-populate the modal from the selected node in the current `graphData`;
- focus the first editable field;
- allow the user to change:
  - node type;
  - property keys;
  - property values;
  - add/remove property rows;
- keep the alias field visible but read-only.

Alias is read-only because the backend node model treats alias as the node identifier. The editable user-facing node name is a normal property row such as `name=...`.

Edit mode submits an `update node {originalAlias}` command. It must not attempt alias rename.

### 2.3 Delete Node

When the user selects **Delete Node**:

- do not send a command immediately;
- replace the node menu contents with an inline confirmation state at the same menu position;
- show the selected alias in the confirmation copy;
- provide **Cancel** and **Delete** controls;
- focus the first confirmation control;
- Escape or Cancel returns to the normal node menu or closes the menu, as implemented consistently with the menu component;
- Delete sends `delete node {alias}` and closes the menu after the send attempt.

Suggested confirmation copy:

```text
Delete "fetchPerson"?
```

Do not use a modal for delete confirmation.

---

## 3. Explicit Non-Goals

- No alias rename in edit mode.
- No backend command changes.
- No optimistic graph mutation in the frontend.
- No localStorage or sessionStorage persistence for modal form state.
- No new runtime dependencies.
- No broad redesign of ReactFlow graph rendering.
- No support for editing unsupported node shapes by silently dropping, flattening, or stringifying data.

---

## 4. Backend Command Contract

The UI sends raw text commands through the existing WebSocket `sendRawText` path.

### 4.1 Edit Submit Command

Producer: frontend command builder  
Consumer: `GraphCommandService.handleMultiLineCommand` -> `handleUpdateNode`

```text
update node {originalAlias}
with type {nodeType}
with properties
{key}={value}
```

Rules:

- `{originalAlias}` is the alias from the selected graph node when the modal was opened.
- The alias field in the modal is display-only and must not affect the command target.
- If node type is blank, the command omits `with type`; the backend will apply its existing `untyped` fallback.
- Empty property rows are omitted.
- Property rows are serialized exactly like create-node property rows.

Expected backend responses:

```text
node {alias} updated
node {alias} not found
ERROR: {message}
```

### 4.2 Delete Command

Producer: frontend command builder  
Consumer: `GraphCommandService.handleDeleteCommand`

```text
delete node {alias}
```

Expected backend responses:

```text
node {alias} deleted
node {alias} not found
ERROR: {message}
```

### 4.3 Result Matching

The frontend must match backend text results against the currently pending action:

| Pending action | Accepted response | Rejected response | Error response |
|---|---|---|---|
| create-node | `node {alias} created` | `node {alias} already exists` | `ERROR: ...` |
| edit-node | `node {alias} updated` | `node {alias} not found` | `ERROR: ...` |
| delete-node | `node {alias} deleted` | `node {alias} not found` | `ERROR: ...` |

Echoed commands beginning with `> ` are not result messages.

---

## 5. Supported Edit Surface

Edit mode supports the same simple node-authoring surface as create mode:

- one alias, displayed read-only;
- one primary node type;
- flat property rows;
- property keys using letters, numbers, underscore, and hyphen;
- single-line property values.

Before opening edit mode, convert the selected `MinigraphNode` to modal form state:

```ts
{
  alias: node.alias,
  nodeType: node.types[0] ?? '',
  properties: Object.entries(node.properties).map(([key, value]) => row(key, String(value)))
}
```

This conversion is allowed only when all of these are true:

- `node.alias` passes the existing node-name validation;
- `node.types.length <= 1`;
- every property key passes the existing property-key validation;
- every property value is a string, number, or boolean;
- every property value string is single-line;
- no property value contains `'''`.

If any condition fails, do not open the edit modal. Show a user-visible error:

```text
This node contains data that cannot be edited in the UI yet. Use the console edit command for this node.
```

This guard is required to prevent data loss from unsupported nested values, arrays, multi-line values, or multiple node types.

---

## 6. Frontend Architecture

### 6.1 Chosen Architecture

Extend the existing graph authoring path into a node action layer:

- keep `NodeDialog` as the presentational modal;
- extend `useGraphAuthoring` to own create/edit/delete action state;
- keep command serialization centralized in `graphActions/minigraphCommandBuilder.ts`;
- extend result parsing/classification in the protocol layer;
- replace the inline node menu JSX in `GraphView.tsx` with a dedicated node context menu component.

This keeps one source of truth for authoring lifecycle, validation, command send, timeout, disconnect, and backend result matching.

### 6.2 Files To Update

Expected touched files:

- `src/components/GraphView/GraphView.tsx`
- `src/components/GraphView/GraphContextMenu.tsx`
- `src/components/GraphView/GraphContextMenu.module.css`
- new `src/components/GraphView/NodeContextMenu.tsx`
- new `src/components/GraphView/NodeContextMenu.module.css`
- `src/components/NodeDialog/NodeDialog.tsx`
- `src/components/NodeDialog/NodeDialog.module.css`
- `src/components/GraphAuthoring/useGraphAuthoring.ts`
- `src/components/GraphAuthoring/GraphAuthoringModals.tsx`
- `src/graphActions/nodeAuthoringTypes.ts`
- `src/graphActions/propertyRows.ts`
- `src/graphActions/validation.ts`
- `src/graphActions/minigraphCommandBuilder.ts`
- `src/utils/messageParser.ts`
- `src/protocol/events.ts`
- `src/protocol/classifier.ts`
- related tests under `src/**/__tests__`

No new runtime dependency is expected.

### 6.3 Component Responsibilities

#### `GraphView.tsx`

Owns graph surface interactions:

- tracks pane menu state for create-node;
- tracks node menu state for clip/edit/delete;
- resolves the selected node from `graphData`;
- calls authoring callbacks passed from `Playground`;
- does not build commands;
- does not parse backend responses.

Integration rules:

- remove the old inline node context menu JSX from `GraphView.tsx`;
- render `NodeContextMenu` for node-level actions;
- keep `GraphContextMenu` pane-only for **Create Node**;
- open the node menu only from `onNodeContextMenu`, meaning the user has right-clicked an actual graph node;
- base node-menu visibility on whether at least one node action is available: `canClipNode || canEditNode || canDeleteNode`;
- do not make edit/delete depend on `onClipNode`; clipboard availability controls only the **Clip to Clipboard** item.

#### `NodeContextMenu.tsx`

Owns node menu rendering:

- pointer-positioned menu;
- normal action state;
- delete confirmation state;
- keyboard focus for menu and confirmation controls;
- Escape/outside lifecycle via parent callbacks;
- no WebSocket calls directly.

Required props:

```ts
interface NodeContextMenuProps {
  open: boolean;
  x: number;
  y: number;
  nodeAlias: string;
  canClipNode: boolean;
  canEditNode: boolean;
  canDeleteNode: boolean;
  onClipNode: () => void;
  onEditNode: () => void;
  onDeleteNode: () => void;
  onClose: () => void;
}
```

#### `NodeDialog.tsx`

Remains presentational:

- receives `mode: 'create' | 'edit'`;
- receives `aliasReadOnly: boolean`;
- renders title and submit label from mode;
- disables fields during sending or disconnected states;
- uses form submit so Enter submits from text inputs;
- does not build commands;
- does not inspect backend text.

Edit-mode labels:

- title: `Edit Node`
- submit button: `Save Changes`
- sending button: `Saving...`
- close aria label: `Close edit node dialog`

Create-mode labels stay:

- title: `Create Node`
- submit button: `Create Node`
- sending button: `Creating...`

#### `useGraphAuthoring.ts`

Owns the action lifecycle:

- open create modal;
- open edit modal;
- send delete after confirmation;
- validate modal state;
- build raw text commands through command builder functions;
- execute commands through `GraphAuthoringExecutor`;
- track one pending node action at a time;
- handle timeout;
- handle WebSocket disconnect;
- consume protocol result events.

No component outside this hook should call `buildCreateNodeCommand`, `buildUpdateNodeCommand`, or `buildDeleteNodeCommand`.

---

## 7. State Model

Recommended state shape:

```ts
type NodeAction = 'create-node' | 'edit-node' | 'delete-node';

type AuthoringState =
  | { status: 'closed'; pendingSubmit: PendingNodeActionSubmit | null; serverMessage: string | null }
  | {
      status: 'open';
      action: 'create-node' | 'edit-node';
      phase: 'editing' | 'sending';
      formState: NodeFormState;
      originalAlias: string | null;
      pendingSubmit: PendingNodeActionSubmit | null;
      serverMessage: string | null;
      connectionLost: boolean;
    };

interface PendingNodeActionSubmit {
  action: NodeAction;
  alias: string;
  command: string;
  sentAt: string;
}
```

Rules:

- `originalAlias` is `null` for create mode.
- `originalAlias` is required for edit mode.
- delete-node can be pending while the modal is closed.
- only one pending node action is allowed at a time.
- no graph data is updated optimistically.
- modal form state is memory-only.

---

## 8. Validation

### 8.1 Create Mode

Keep current validation:

- alias required;
- alias must match the backend name rule;
- alias must not be reserved;
- alias must not already exist in current `graphData`;
- node type optional but must match the backend name rule when present;
- property rows follow the existing key/value validation;
- serialized command must not exceed `MAX_BUFFER`.

### 8.2 Edit Mode

Edit mode validation differs from create mode:

- alias is read-only and not duplicate-checked;
- original alias must match the backend name rule before command serialization;
- node type validation is the same as create mode;
- property row validation is the same as create mode;
- serialized command must not exceed `MAX_BUFFER`.

If the selected node is no longer found in current `graphData` when opening edit mode, do not open the modal. Show:

```text
This node is no longer available in the current graph.
```

### 8.3 Delete Mode

Before sending delete:

- selected alias must exist in current `graphData`;
- selected alias must match the backend name rule;
- WebSocket must be connected.

If validation fails, do not send the command and show a user-visible error.

---

## 9. Protocol Layer Changes

Add a generic node-action result event while keeping create-node behavior compatible.

```ts
export type NodeActionTextResultStatus = 'accepted' | 'rejected' | 'error';
export type NodeActionTextResultAction = 'create-node' | 'edit-node' | 'delete-node' | null;

export interface NodeActionTextResult {
  status: NodeActionTextResultStatus;
  action: NodeActionTextResultAction;
  alias: string | null;
  message: string;
}

export interface NodeActionTextResultEvent extends ProtocolEventBase {
  kind: 'minigraph.nodeAction.textResult';
  status: NodeActionTextResultStatus;
  action: NodeActionTextResultAction;
  alias: string | null;
  message: string;
}
```

Parser behavior:

| Raw text | Parsed action | Status |
|---|---|---|
| `node A created` | `create-node` | `accepted` |
| `node A already exists` | `create-node` | `rejected` |
| `node A updated` | `edit-node` | `accepted` |
| `node A deleted` | `delete-node` | `accepted` |
| `node A not found` | `null` | `rejected` |
| `ERROR: bad command` | `null` | `error` |
| `> update node A...` | no event | no event |

`graph.mutation` detection already covers `created`, `updated`, and `deleted`; keep that behavior and add tests if coverage is missing.

---

## 10. UX And Accessibility Details

### 10.1 Modal

- Modal remains centered.
- Overlay must block pointer interaction with panels underneath.
- Escape closes only when not sending.
- Enter submits through the form.
- Alias field in edit mode should be `readOnly`, not hidden.
- The read-only alias remains selectable and focusable unless the dialog is globally disabled.
- Use `aria-describedby` text for alias if needed:

```text
Alias is the node identifier and cannot be renamed here.
```

### 10.2 Node Context Menu

- The menu opens at `event.clientX` / `event.clientY`.
- If the menu would overflow the viewport, clamp it inside the viewport.
- Use `role="menu"` for the menu container and `role="menuitem"` for action buttons.
- Delete confirmation can use normal buttons inside the menu; it does not need `role="menuitem"` for the destructive confirm button if that makes the confirmation semantics clearer.
- The destructive button should be visually distinct but not oversized.
- Disabled actions must remain visually disabled and must not fire.

### 10.3 Disconnected State

If a modal is open and the WebSocket disconnects:

- disable all fields and submit controls;
- keep close available unless a send is actively unresolved;
- show an action-specific message.

Create message:

```text
Connection disconnected. Refresh the page and create the node again after the app reconnects.
```

Edit message:

```text
Connection disconnected. Refresh the page and edit the node again after the app reconnects.
```

Pending send message:

```text
Connection disconnected while the node action was pending. The outcome is unknown. Refresh the page and check the graph before trying again.
```

---

## 11. Failure Handling

| Failure | User-visible behavior |
|---|---|
| WebSocket disconnected before open | edit/delete actions disabled or unavailable |
| WebSocket disconnected while modal open | modal locks and shows disconnected message |
| WebSocket disconnected while command pending | pending timer cleared, unknown-outcome message shown |
| Send returns false | command not pending; show send-failure message |
| Validation fails | show inline field errors; do not send |
| Unsupported edit node shape | do not open modal; show unsupported-node message |
| Backend says not found | close delete pending state or return edit modal to editing; show backend message |
| Backend returns `ERROR: ...` | show backend error while current action is pending |
| Timeout | return pending modal to editing or show delete timeout message; outcome is unknown |
| User cancels delete confirmation | no command sent |
| User closes edit modal before submit | no command sent |

Timeout remains `DEFAULT_AUTHORING_TIMEOUT_MS = 10_000` unless existing code changes it.

---

## 12. Flow Details

### 12.1 Edit Flow

1. User right-clicks a node.
2. `GraphView` opens `NodeContextMenu` at pointer coordinates.
3. User selects **Edit Node**.
4. `GraphView` resolves the selected `MinigraphNode` from current `graphData`.
5. `useGraphAuthoring.openEditNode(node)` converts the node into modal form state.
6. If conversion is unsupported, show the unsupported-node message and stop.
7. `NodeDialog` opens in edit mode.
8. User changes type/properties.
9. User clicks **Save Changes** or presses Enter.
10. Hook validates edit state.
11. Hook builds `update node {originalAlias}` raw text.
12. Hook sends through `GraphAuthoringExecutor`.
13. Protocol parser emits `minigraph.nodeAction.textResult`.
14. Hook matches event to the pending edit action.
15. On `node {alias} updated`, modal closes.
16. Existing auto-refresh receives `graph.mutation` and refreshes the graph.

### 12.2 Delete Flow

1. User right-clicks a node.
2. `GraphView` opens `NodeContextMenu` at pointer coordinates.
3. User selects **Delete Node**.
4. Menu switches to confirmation state.
5. User confirms **Delete**.
6. Hook validates current connection and alias.
7. Hook builds `delete node {alias}` raw text.
8. Hook sends through `GraphAuthoringExecutor`.
9. Menu closes after the send attempt.
10. Protocol parser emits `minigraph.nodeAction.textResult`.
11. Hook matches event to the pending delete action.
12. On `node {alias} deleted`, existing auto-refresh receives `graph.mutation` and refreshes the graph.

---

## 13. Testing Requirements

### 13.1 Unit Tests

Add or update tests for:

- build update-node command;
- build delete-node command;
- edit validation skips duplicate alias checks;
- edit validation rejects unsupported original alias;
- selected-node-to-form-state conversion accepts scalar flat properties;
- selected-node-to-form-state conversion flattens arrays, nested objects, and multi-line leaf values;
- selected-node-to-form-state conversion rejects:
  - multiple types;
  - invalid keys;
  - empty arrays or objects;
  - values containing triple-quote delimiters;
- node-action result parser:
  - created;
  - already exists;
  - updated;
  - deleted;
  - not found;
  - `ERROR: ...`;
  - echoed commands ignored;
- classifier emits `minigraph.nodeAction.textResult`;
- existing create-node parser compatibility if retained;
- `detectMutation` still recognizes updated/deleted responses.

### 13.2 Manual QA

Verify in the browser:

- right-click graph pane still opens create menu at pointer;
- right-click node opens node menu at pointer;
- Clip to Clipboard still works;
- Edit Node opens pre-populated modal;
- alias is read-only in edit mode;
- editing property key `name` is allowed;
- Enter submits edit modal;
- Save Changes sends raw update text and graph refreshes;
- Delete Node shows confirmation before sending;
- Cancel delete sends nothing;
- Confirm delete sends raw delete text and graph refreshes;
- menu closes on Escape and outside click;
- modal overlay still blocks panel resizing;
- disconnected modal locks fields and shows the disconnected message.

---

## 14. Rollout And Reversibility

This feature is limited to the Minigraph playground and should be gated by existing `supportsAuthoring` and WebSocket connection state.

Rollback path:

- remove edit/delete menu items;
- leave create-node implementation untouched;
- keep generic parser additions if already harmless, or revert the protocol event change with related tests.

No data migration is required.

---

## 15. Implementation Slices

### Slice 1 - Pure command and parser layer

Files:

- `graphActions/minigraphCommandBuilder.ts`
- `graphActions/validation.ts`
- `graphActions/propertyRows.ts`
- `graphActions/nodeAuthoringTypes.ts`
- `utils/messageParser.ts`
- `protocol/events.ts`
- `protocol/classifier.ts`
- related tests

Behavior:

- build update/delete commands;
- parse node-action results;
- validate edit/delete inputs;
- convert graph node data into edit form state.

Rollback:

- remove new pure functions and tests.

### Slice 2 - Authoring hook lifecycle

Files:

- `components/GraphAuthoring/useGraphAuthoring.ts`
- `components/GraphAuthoring/GraphAuthoringModals.tsx`

Behavior:

- add edit modal lifecycle;
- add delete pending lifecycle;
- match generic node-action result events;
- preserve existing create-node behavior.

Rollback:

- keep create-node state path and remove edit/delete branches.

### Slice 3 - UI components

Files:

- `components/NodeDialog/NodeDialog.tsx`
- `components/NodeDialog/NodeDialog.module.css`
- `components/GraphView/GraphView.tsx`
- new `components/GraphView/NodeContextMenu.tsx`
- new `components/GraphView/NodeContextMenu.module.css`

Behavior:

- mode-aware modal labels;
- read-only alias in edit mode;
- node context menu with edit/delete;
- inline delete confirmation.

Rollback:

- remove node menu edit/delete props and restore current clip-only node menu.

---

## 16. Decisions

| Decision | Selected | Alternatives considered | Reason |
|---|---|---|---|
| Alias editability | Read-only | Add frontend rename; add backend rename | Backend alias is immutable in the current node model. Rename is a separate feature. |
| Edit prefill source | Current `graphData` node | Send `edit node {alias}` first and parse returned text | Current graph data already backs the visible graph and avoids an extra request-response lifecycle. |
| Delete confirmation | Inline menu confirmation | Modal confirmation; browser confirm | User requested confirmation in the menu; inline keeps the flow local. |
| Unsupported node data | Block edit with message | Stringify/drop unsupported values | Blocking prevents silent data loss. |
| Transport | Existing raw WebSocket command path | REST endpoint; JSON command envelope | Backend already accepts these graph mutations as raw commands. |

---

## Appendix - Planning Artifact (from /spec-plan)

**Phase 0 - Domain calibration**
- Domain mix: Frontend UI/rendering 35%, state management/data flow 25%, network protocol/RPC 20%, product workflow/UX 15%, security/trust 5%.
- Dominant failure mode: UI and backend state can disagree if edit/delete uses a second command path or silently mutates frontend graph state.
- Pre-mortem watch-items: composition bugs between ReactFlow context menu and modal; stale graph data causing overwrite/not-found responses; unsupported node data being silently lost during edit.
- Calibration: Emphasis placed on source of truth, protocol contract, composition audit, and failure matrix.

**Step 1 - Requirement and journeys**
- Actor / trigger / success signal: a Minigraph user right-clicks a graph node; edit saves through backend `update node`; delete completes only after confirmation and backend `node {alias} deleted`.
- Primary journeys: edit simple node; delete simple node; cancel delete; disconnected edit/delete; unsupported node edit.
- Non-goals: alias rename, backend changes, optimistic graph mutation, storage-backed modal state, new runtime dependencies.

**Step 3 - Anti-anchoring and compatibility**
- Silent assumptions: create-node command path remains raw text; graphData is a frontend projection and can be stale; alias is an immutable backend identifier; current modal supports one node type and flat single-line properties; ReactFlow node context menus must not interfere with pane menus.
- Fresh-team delta: a fresh team might create a generic CRUD modal and REST endpoint; this spec keeps the existing raw command/WebSocket contract to avoid backend changes.
- Existing behavior contract: create-node unchanged and extended; pane context menu unchanged; clip-to-clipboard unchanged; auto-refresh unchanged; protocol classifier extended.

**Step 4 - Three architectures**
- Option A - root primitive: lightest approach, add edit/delete inline in `GraphView` and send raw text directly.
- Option B - root primitive: codebase-conventional approach, extend `useGraphAuthoring` and command/parser helpers.
- Option C - root primitive: backend-driven approach, call `edit node {alias}` first and parse command text before opening modal.
- Picked: Option B.
- Axis spread verified: options vary by state owner, prefill source, transport lifecycle, component count, and parser responsibility.

**Step 5 - Source of truth and boundaries**
- Source-of-truth inventory: backend graph owns committed data; current `graphData` owns visible prefill snapshot; modal state owns unsaved edits; pending action owns command/result matching; protocol parser owns text-result classification.
- Boundary map: UI boundary via modal/menu props; WebSocket boundary via raw text commands; protocol boundary via typed events; async boundary via pending action timer; trust boundary via frontend validation plus backend authority.

**Step 6 - Contracts**
- Boundary-crossing contracts: `update node`, `delete node`, `minigraph.nodeAction.textResult`, `NodeFormState`, `PendingNodeActionSubmit`, selected-node-to-form-state conversion result.
- Shared constants / schema strategy: reuse backend-aligned `NODE_NAME_RE`; centralize command strings in command builder; centralize result regexes in message parser.

**Step 9 - Primitive fit and composition**
- Under-use / over-use findings: no new dialog/menu library required; existing React form and CSS modules are enough.
- Composed primitives: ReactFlow context menu events, fixed-position menu, modal overlay, WebSocket send, ProtocolBus event subscription, auto-refresh hook.
- Overlaps: node right-click vs pane right-click; modal overlay vs panel resize handles; delete confirmation vs menu dismissal; pending WebSocket result vs modal close.
- Guardrails: stop propagation on node context menu; separate pane and node menu state; fixed modal overlay absorbs pointer events; sending state blocks close; one pending node action at a time.

**Step 10 - From-scratch comparison**
- Materially simpler?: No.
- If yes, redesign adopted: Not applicable.

**Step 11 - Failure and lifecycle**
- Failure matrix summary: validation, unsupported data, disconnected socket, send failure, timeout, not found, backend error, cancel, duplicate pending action, and stale graph data are all handled with explicit user-visible outcomes.
- Lifecycle data-access findings: selected node is read at menu action time; form state is snapshot-based; WebSocket send is allowed only when connected; ProtocolBus events are matched only while an action is pending.

**Step 12 - Spike / performance**
- Spike required?: No.
- Result or reason not required: operations are user-triggered and per selected node; no loop scales beyond selected node property count.
- Per-N cost: converting a node is O(P) for P properties; menu rendering is O(1); parser matching is O(1) per WebSocket message.

**Step 13 - Security / trust**
- Input boundaries: modal field text and selected graph data cross into raw command text.
- Validation / auth / encoding: frontend validates syntax and size before serialization; backend remains authoritative; command builder owns raw text serialization; no sensitive data is persisted by this feature.

**Step 14 - Observability / rollout**
- Debuggability: console still shows backend command echoes/results; protocol events can be inspected from classifier tests and bus subscriptions; user messages include raw backend result text where useful.
- Rollout / rollback: scoped to Minigraph `supportsAuthoring`; rollback removes menu items and hook branches without data migration.

**Step 15 - Verification**
- Verification mapping summary: command builders, validation, selected-node conversion, parser, classifier, and mutation detection use unit tests; modal/menu interactions use manual QA.
- Design cross-reference complete: tests map to Sections 4, 5, 8, 9, 10, and 13.

**Step 16 - Implementation / consumer readiness**
- Implementation slices: pure command/parser layer; authoring hook lifecycle; UI components.
- Review questions / decision log: alias remains read-only; unsupported shapes block edit; delete confirmation stays inline; edit prefill uses current graphData.

**Step 17 - Pre-implementation self-check**
- All self-check answers yes?: Yes.
- Enumeration completeness: state creation, mutation, cleanup, timeout, disconnect, Escape/outside close, and auto-refresh trigger paths are specified.
- Cross-iteration regression: create-node raw command path, memory-only modal form state, centered modal overlay, pointer-positioned context menu, and backend-result validation are preserved.
