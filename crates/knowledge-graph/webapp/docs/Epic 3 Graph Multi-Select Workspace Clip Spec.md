# Epic 3 Graph Multi-Select Workspace Clip Spec

## Goal

Add graph multi-select support so users can select multiple nodes with `Shift + left click` or `Shift + drag`, then right-click the selected set and either clip those nodes to Workspace or delete those nodes in one action.

The implementation must also make the new interaction discoverable. After a graph is imported or otherwise first becomes available in the current page session, the graph canvas must show a lightweight tip explaining the multi-select controls.

## User-Visible Behavior

### Multi-Select

1. `Shift + left click` on a node toggles that node in the current selection.
2. `Shift + drag` from empty graph canvas draws a selection box and selects nodes inside the box.
3. A normal left click on a node keeps the existing single-node selection behavior.
4. A normal click on empty graph canvas clears selection and closes any open graph context menu.
5. Selection is graph-local UI state. It is not written to `localStorage`, `sessionStorage`, IndexedDB, or the backend.

Required interaction acceptance criteria:

1. `Shift + left click` on an unselected node must select that node immediately, even when no node is currently selected.
2. `Shift + left click` on a selected node must remove that node from the selection.
3. `Shift + drag` must start only from empty graph canvas. Starting a drag from a node must not create a selection box.
4. Box selection must use partial intersection: a node is selected if the selection rectangle touches any part of the node. The node does not need to be fully enclosed.
5. During `Shift + drag`, the active selection rectangle must visually match the user's pointer drag area.
6. After `Shift + drag` ends, the UI must not show a large group bounding rectangle that can be mistaken for the original selection box.
7. After `Shift + drag` ends, selected nodes must remain individually visible through node selected styling.
8. After box selection ends, right-clicking any selected node must open the multi-node context menu.
9. Right-clicking empty space inside the previous selection area must not be required for multi-node actions.

### Multi-Select Discoverability Tip

The graph must teach users the new shortcut without requiring them to open documentation.

Trigger:

1. Show the tip when `graphData` first changes from empty/null to a graph with at least one node during the current page session.
2. Show it only inside the Graph canvas, not globally.
3. Show it once per page session. A full page reload may show it again, matching the existing help shortcut hint behavior.
4. Do not show it for empty graphs.
5. Do not show it after ordinary graph refreshes if it already appeared in this page session.

Placement:

1. Place the tip in the graph canvas upper-left area.
2. It must not cover the graph toolbar controls.
3. It must not block right-click menus, node drag, pan, zoom, Workspace drag/drop, or node selection.
4. It should visually feel related to the existing help shortcut hint pattern, but must be scoped to the graph canvas.

Text:

```text
Multi-select: Shift + click nodes, or Shift + drag on the canvas.
```

Dismissal:

1. Clicking the tip dismisses it.
2. Any graph interaction dismisses it:
   - node click;
   - pane click;
   - node right-click;
   - pane right-click;
   - node drag;
   - pane drag/pan;
   - wheel zoom;
   - `Shift + drag` selection.
3. It also auto-dismisses after 5 seconds if the user does nothing.
4. Dismissal is session-only. Do not persist the dismissed state to local storage.
5. Dismissal must use the same kind of non-blocking fade behavior as the existing help hint if practical.

### Right-Click Menu Behavior

1. If the user right-clicks a node that is not part of the current multi-selection:
   - the context target is that clicked node only;
   - the menu shows the existing single-node actions.
2. If the user right-clicks a selected node and exactly one node is selected:
   - the menu behaves like the current single-node context menu.
3. If the user right-clicks a selected node and more than one node is selected:
   - the menu shows `Clip N selected nodes to Workspace`;
   - the menu shows `Delete N selected nodes` when authoring is supported and connected;
   - `Edit Node` must not be shown.
4. Multi-node menu items must include the selected count:

```text
Clip 3 selected nodes to Workspace
Delete 3 selected nodes
```

5. Selecting `Delete N selected nodes` must switch the menu into a confirmation state before any command is sent.
6. The confirmation copy must include the selected count:

```text
Delete 3 selected nodes?
```

7. The confirmation actions must be `Delete` and `Cancel`.
8. The context menu must continue opening at the pointer coordinates and must remain viewport-clamped.

### Batch Clip Feedback

Batch clip must show one summary toast instead of one toast per selected node.

Required summary behavior:

- all added: `3 nodes clipped to workspace`;
- some duplicates: `3 nodes clipped to workspace. 2 already existed.`;
- all duplicates: `All selected nodes already exist in workspace.`;
- some failures: `3 nodes clipped to workspace. 2 failed.`;
- no selected nodes after stale filtering: `No selected nodes are available to clip.`;
- selected count above limit: `Select 100 or fewer nodes to clip at once.`;
- all failed: `Failed to clip selected nodes to workspace.`;

The message must distinguish added, duplicate, failed, and no-op outcomes.

### Batch Delete Feedback

Batch delete must show one summary toast instead of one toast per selected node.

Required summary behavior:

- all accepted by the frontend send path: `3 delete-node commands sent. Waiting for backend response.`;
- all accepted by backend result events: `3 selected nodes deleted.`;
- partial backend failure: `2 selected nodes deleted. 1 failed.`;
- all backend failures: `Failed to delete selected nodes.`;
- no selected nodes after stale filtering: `No selected nodes are available to delete.`;
- selected count above limit: `Select 100 or fewer nodes to delete at once.`;
- disconnected before send: `Could not send delete-node commands because the WebSocket is not open.`;
- timeout before all results are observed: `Some delete-node commands were sent, but not all backend results were observed yet. Refresh the graph before trying again.`;

Batch delete must not silently ignore backend failures. If the backend result parser cannot identify a delete result for a selected alias, the action remains pending until timeout.

## Non-Goals

1. No multi-node edit.
2. No grouped Workspace item.
3. No group paste command.
4. No backend command or protocol change.
5. No checkbox-on-hover selection UI.
6. No persisted selection state.
7. No persisted multi-select tip dismissed state.
8. No change to the existing single-node duplicate replacement dialog.

## Current Architecture Context

Current graph context menu flow:

1. `GraphView.tsx` renders `<ReactFlow>`.
2. `onNodeContextMenu` opens `NodeContextMenu` for the right-clicked node.
3. `GraphView` resolves the node from `graphData` with `findNodeByAlias`.
4. Single-node clip extracts direct connections with `extractDirectConnections`.
5. `Playground.handleClipNode` calls `clipboardCtx.clipNode`.
6. `ClipboardContext.clipNode` writes one `ClipboardItemRecord` to IndexedDB.
7. `ClipboardSidebar` renders `clipboardCtx.items`.

Current single-node delete flow:

1. `NodeContextMenu` asks for confirmation inside the menu.
2. `GraphView` calls `onDeleteNode` with the selected `MinigraphNode`.
3. `Playground` passes the call to `graphAuthoring.deleteNode`.
4. `useGraphAuthoring.deleteNode` validates the alias against current `graphData`.
5. `buildDeleteNodeCommand` builds `delete node {alias}`.
6. `GraphAuthoringExecutor` sends raw text to the backend.
7. `useGraphAuthoring` tracks one pending delete action and matches backend text-result events.

Current help hint pattern:

1. `Playground.tsx` owns the help hint state.
2. The hint is non-blocking.
3. It can be dismissed by click or by related UI activity.
4. It is not a modal and does not own focus.

This feature extends the graph and Workspace flow. It must not replace the current single-node path.

## Chosen Architecture

Use React Flow's built-in selection primitives for the actual graph selection, then keep a derived selected-alias snapshot in `GraphView` for context menu decisions and batch clip payload creation.

React Flow props required:

```tsx
<ReactFlow
  selectionKeyCode="Shift"
  multiSelectionKeyCode="Shift"
  selectionOnDrag={false}
  selectionMode={SelectionMode.Partial}
  onSelectionChange={({ nodes }) => {
    // keep selected node aliases in GraphView
  }}
/>
```

Rationale:

- React Flow already owns node selected state and selected rendering.
- `GraphView` already wraps React Flow and owns node context menu behavior.
- Workspace already stores one node per `ClipboardItemRecord`.
- Batch clip can reuse the current `clipboardCtx.clipNode` validation, duplicate handling, IndexedDB write path, and BroadcastChannel sync.
- Batch delete must extend `useGraphAuthoring` with a dedicated multi-delete path. It must not call the existing single-node `deleteNode` repeatedly because that hook currently tracks one pending delete action at a time.

### React Flow Interaction Guardrails

This feature depends on React Flow selection behavior, so the implementation must explicitly handle React Flow's library-specific event and overlay behavior instead of assuming the high-level props are sufficient.

Required guardrails:

1. Node wrappers must opt out of pane-level `selectionKeyCode` capture when needed so `Shift + left click` reaches node selection on the first click.
2. Use React Flow's partial selection mode so box selection selects intersecting nodes instead of requiring full containment.
3. Keep the active drag marquee visible while the user is dragging.
4. Hide or neutralize React Flow's completed `nodesselection` group rectangle if it creates a misleading large bounding box after selection ends.
5. The completed group-selection surface must not intercept right-click events that should reach selected nodes.
6. Do not replace React Flow's selection engine with a custom hit-test engine unless React Flow cannot satisfy these contracts.
7. If React Flow changes these class names or semantics in a future upgrade, re-run the manual QA items for `Shift + click`, `Shift + drag`, and right-click after box selection before accepting the upgrade.

Known React Flow implementation notes:

- `selectionKeyCode="Shift"` can make the pane treat `Shift + pointer` over nodes as a selection-box start unless node wrappers are excluded from the pane key handler.
- `SelectionMode.Partial` is required for "swept/touched node is selected" behavior.
- React Flow renders a completed group-selection rectangle for selected nodes. In this app, individual selected node styling is the source of truth, so the group rectangle should not remain visible or pointer-active.

## Files And Responsibilities

### `src/components/GraphView/GraphView.tsx`

Responsibilities:

- enable React Flow `Shift` multi-select and box-select props;
- keep a derived `selectedNodeAliases` state from `onSelectionChange`;
- show and dismiss the multi-select discoverability tip;
- resolve right-click target as single-node or multi-node;
- close stale menus when graph data changes;
- pass single-node or multi-node mode to `NodeContextMenu`;
- build the selected node clip payload for `onClipNodes`;
- build the selected node delete payload for `onDeleteNodes`.

New props:

```ts
onClipNodes?: (items: GraphClipItem[]) => void;
onDeleteNodes?: (nodes: MinigraphNode[]) => void;
```

New local/shared shape:

```ts
interface GraphClipItem {
  node: MinigraphNode;
  connections: MinigraphConnection[];
}
```

The shape may live in a shared file if it is used by both `GraphView` and `Playground`.

### `src/components/GraphView/GraphMultiSelectTip.tsx`

Create this component if keeping the tip inline in `GraphView.tsx` would make the graph component too large.

Responsibilities:

- render the non-blocking tip;
- support click-to-dismiss;
- support fade-out state if needed;
- expose no business logic.

Required props:

```ts
interface GraphMultiSelectTipProps {
  visible: boolean;
  fading: boolean;
  onDismiss: () => void;
}
```

The tip text must be:

```text
Multi-select: Shift + click nodes, or Shift + drag on the canvas.
```

### `src/components/GraphView/NodeContextMenu.tsx`

Responsibilities:

- preserve existing single-node menu rendering;
- add a multi-select mode;
- show `Clip N selected nodes to Workspace` in multi-select mode;
- show `Delete N selected nodes` in multi-select mode when deletion is available;
- keep `Edit Node` hidden in multi-select mode;
- require confirmation before multi-node delete;
- show selected count in accessible text;
- keep existing Escape, outside-click, scroll, and resize close behavior.

Expected props extension:

```ts
interface NodeContextMenuProps {
  mode: 'single-node' | 'multi-node';
  nodeAlias: string;
  selectedCount?: number;
  canClipSelectedNodes?: boolean;
  canDeleteSelectedNodes?: boolean;
  onClipSelectedNodes?: () => void;
  onDeleteSelectedNodes?: () => void;
}
```

Implementation may use a discriminated union instead of optional props if that keeps the component clearer.

### `src/components/RightPanel/RightPanel.tsx`

Responsibilities:

- pass `onClipNodes` from `Playground` to `GraphView`;
- pass `onDeleteNodes` from `Playground` to `GraphView`;
- keep `onClipNode`, `onEditNode`, and `onDeleteNode` unchanged.

### `src/components/Playground.tsx`

Responsibilities:

- add `handleClipNodes(items)` for batch clip;
- call `clipboardCtx.clipNode` once per selected node;
- skip duplicates in batch mode instead of opening the duplicate replacement dialog;
- show one summary toast after the batch finishes;
- keep existing `handleClipNode` behavior unchanged for single-node clip.

Batch clip must process selected nodes sequentially. This keeps result ordering deterministic and reuses the existing `clipNode` duplicate handling without adding parallel IndexedDB races.

`Playground` must also pass a multi-node delete callback from `graphAuthoring.deleteNodes` to `RightPanel` when authoring is supported.

### `src/components/GraphAuthoring/useGraphAuthoring.ts`

Responsibilities:

- add `deleteNodes(nodes: MinigraphNode[])`;
- validate all selected aliases against current `graphData` before sending;
- reject the whole batch if any alias is invalid or stale;
- reject the whole batch if another node action is already pending;
- build one `delete node {alias}` raw command per selected node;
- send commands sequentially through `GraphAuthoringExecutor`;
- track a batch pending state keyed by alias so backend text-result events can be counted;
- show one user message for accepted sends, completion, partial failure, full failure, timeout, or disconnect.

The hook may introduce a new pending shape:

```ts
interface PendingBatchDeleteSubmit {
  action: 'delete-nodes';
  aliases: string[];
  commands: string[];
  sentAt: string;
  results: Record<string, 'success' | 'error'>;
}
```

The exact type can differ, but it must support multiple aliases without overwriting the existing single-node pending action.

### `src/components/GraphView/selectionTargets.ts`

Create this small helper module if the right-click target logic becomes non-trivial inside `GraphView`.

Recommended exports:

```ts
export const MAX_BATCH_NODE_ACTIONS = 100;

export type NodeContextTarget =
  | { kind: 'single-node'; alias: string }
  | { kind: 'multi-node'; aliases: string[] };

export function resolveNodeContextTarget(
  clickedAlias: string,
  selectedAliases: string[],
): NodeContextTarget;

export function filterAliasesToGraphNodes(
  aliases: string[],
  graphData: MinigraphGraphData,
): MinigraphNode[];
```

Rules:

- if `clickedAlias` is included in `selectedAliases` and `selectedAliases.length > 1`, return `multi-node`;
- otherwise return `single-node` for `clickedAlias`;
- returned multi-node aliases must be stable and unique.

### `src/clipboard/batchClipSummary.ts`

Create this helper only if summary logic becomes too large for `Playground.tsx`.

Recommended exports:

```ts
export interface BatchClipSummary {
  added: number;
  duplicates: number;
  failed: number;
}

export function buildBatchClipToast(summary: BatchClipSummary): {
  message: string;
  level: 'success' | 'info' | 'error';
};
```

## Detailed Flow

### Flow 1: Graph Import Shows Multi-Select Tip

1. User imports a graph, runs a command that exposes graph data, or otherwise causes `graphData` to become non-empty.
2. `GraphView` detects the first transition in this page session from no graph nodes to at least one graph node.
3. `GraphView` shows the tip in the canvas upper-left area.
4. The tip does not take focus.
5. The tip does not stop graph pointer events except when the user clicks the tip itself to dismiss it.
6. The tip dismisses when:
   - user clicks the tip;
   - user interacts with the graph canvas;
   - 5 seconds pass.
7. The tip does not show again in the same page session.

Failure/no-op:

- if graph data is empty, do not show the tip;
- if the graph fails to render, do not show the tip;
- if the graph refreshes after the tip has appeared, do not show the tip again.

### Flow 2: Shift + Left Click Multi-Select

1. User holds `Shift`.
2. User left-clicks a node.
3. React Flow toggles that node in the selected set through `multiSelectionKeyCode="Shift"`.
4. `onSelectionChange` fires.
5. `GraphView` stores selected aliases derived from the selected React Flow nodes.
6. The graph displays selected state using existing React Flow selected rendering.
7. If the multi-select tip is visible, the graph interaction dismisses it.

Failure/no-op:

- if graph data is empty, there are no selectable nodes;
- if a selected alias no longer exists after refresh, it is removed from the derived selection.

### Flow 3: Shift + Drag Box Select

1. User holds `Shift`.
2. User drags from empty graph canvas.
3. React Flow draws a selection box because `selectionKeyCode="Shift"`.
4. React Flow selects nodes inside the box.
5. `onSelectionChange` updates `GraphView` selected aliases.
6. If the multi-select tip is visible, the graph interaction dismisses it.

Guardrail:

- `selectionOnDrag` remains `false`, so ordinary drag does not become box selection.

### Flow 4: Right-Click One Node

1. User right-clicks a node.
2. `GraphView.onNodeContextMenu` prevents the browser menu.
3. If the multi-select tip is visible, the graph interaction dismisses it.
4. `GraphView` resolves the target:
   - if the clicked node is not part of a multi-selection, target is `single-node`;
   - if only one node is selected, target is `single-node`.
5. `NodeContextMenu` opens in `single-node` mode.
6. Menu shows:
   - `Clip to Workspace`;
   - `Edit Node` when authoring is supported and connected;
   - `Delete Node` when authoring is supported and connected.

### Flow 5: Right-Click Multi-Selected Nodes

1. User selects at least two nodes.
2. User right-clicks one selected node.
3. If the multi-select tip is visible, the graph interaction dismisses it.
4. `GraphView` resolves target as `multi-node`.
5. `NodeContextMenu` opens in `multi-node` mode.
6. Menu shows:
   - `Clip N selected nodes to Workspace`;
   - `Delete N selected nodes` when authoring is supported and connected.
7. Menu does not show `Edit Node`.

### Flow 6: Clip Multi-Selected Nodes

1. User opens the multi-node menu.
2. User clicks `Clip N selected nodes to Workspace`.
3. `GraphView` converts selected aliases to current `MinigraphNode` objects from `graphData`.
4. `GraphView` extracts direct connections for each selected node.
5. `GraphView` calls `onClipNodes(items)`.
6. `Playground.handleClipNodes` loops over the items and calls `clipboardCtx.clipNode` for each item.
7. Duplicate results are counted and skipped.
8. Errors are counted.
9. One summary toast is shown.
10. Workspace sidebar updates through existing `ClipboardContext` state and BroadcastChannel behavior.

### Flow 7: Delete Multi-Selected Nodes

1. User opens the multi-node menu.
2. User clicks `Delete N selected nodes`.
3. `NodeContextMenu` switches to confirmation state.
4. User clicks `Delete`.
5. `GraphView` converts selected aliases to current `MinigraphNode` objects from `graphData`.
6. `GraphView` calls `onDeleteNodes(nodes)`.
7. `useGraphAuthoring.deleteNodes` validates every alias against current `graphData`.
8. `useGraphAuthoring.deleteNodes` builds one `delete node {alias}` command per selected node.
9. Commands are sent sequentially through `GraphAuthoringExecutor`.
10. The hook records one pending batch delete state with all aliases.
11. Backend text-result events are matched by alias and delete action.
12. When all aliases have a terminal result, the hook shows one summary message.
13. Existing graph mutation refresh behavior updates the rendered graph.

Failure/no-op:

- if any alias is stale before send, do not send any delete command;
- if the WebSocket disconnects before send, do not send any delete command;
- if the WebSocket disconnects after partial send, show the pending-disconnected message and require refresh before retry;
- if backend results are incomplete by timeout, show the timeout message and require refresh before retry.

### Flow 8: Graph Refresh While Nodes Are Selected

1. Graph data refreshes after backend mutation or import.
2. `GraphView` receives new `graphData`.
3. `initialNodes` are recalculated.
4. `GraphView` resets React Flow nodes with the new data.
5. `GraphView` clears or filters `selectedNodeAliases`.
6. Any open node context menu closes.

Required behavior:

- do not allow a batch clip action to run against aliases that are no longer present in current `graphData`;
- do not allow a batch delete action to run against aliases that are no longer present in current `graphData`;
- if no selected nodes remain, show the no-op toast if the user somehow triggers the action.

## Source Of Truth

| Concept | Owner | Read Path | Write Path | Sync Rule |
|---|---|---|---|---|
| Graph data | `graphData` from backend fetch/import flow | `GraphView`, `GraphDataView` | backend/import/refresh | `GraphView` recalculates nodes when `graphData` changes |
| React Flow selected state | React Flow node state | `onSelectionChange` | user click/drag selection | `GraphView` mirrors selected aliases only for menu logic |
| Context menu target | `GraphView.contextMenu` | `NodeContextMenu` | right-click handlers | target is a snapshot at menu open time |
| Multi-select tip visibility | `GraphView` session state | `GraphView` / tip component | graph availability and dismiss handlers | show once per page session after first non-empty graph |
| Workspace items | `ClipboardContext` + IndexedDB | `ClipboardSidebar`, `Playground` | `clipboardCtx.clipNode` | BroadcastChannel syncs tabs |
| Batch clip result | `Playground.handleClipNodes` local counters | toast | per-node `clipNode` result | discarded after toast |
| Batch delete pending state | `useGraphAuthoring` | backend text-result event handler | `deleteNodes(nodes)` | terminal when every alias has success/error or timeout/disconnect fires |

## Contracts

### `GraphClipItem`

Producer: `GraphView`.

Consumer: `Playground.handleClipNodes`.

```ts
interface GraphClipItem {
  node: MinigraphNode;
  connections: MinigraphConnection[];
}
```

Invalid input behavior:

- empty array: show no-op toast;
- item without a current graph node should not be produced by `GraphView`;
- if defensive validation detects missing `node.alias`, count it as failed.

### `ClipboardItemRecord`

No schema change.

Each selected node becomes one existing single-node record:

```ts
interface ClipboardItemRecord {
  id: string;
  clippedAt: string;
  sourceWsPath: string;
  sourceLabel: string;
  node: MinigraphNode;
  connections: MinigraphConnection[];
}
```

Duplicate behavior remains alias-based through the existing IndexedDB `by-alias` unique index.

### Context Menu Mode

Producer: `GraphView`.

Consumer: `NodeContextMenu`.

```ts
type NodeContextMenuMode = 'single-node' | 'multi-node';
```

Invalid input behavior:

- `multi-node` with `selectedCount <= 1` must fall back to single-node mode or render nothing;
- `multi-node` without `onClipSelectedNodes` and `onDeleteSelectedNodes` must render no action;
- `multi-node` must not render `Edit Node`.

### Batch Delete Submit

Producer and owner: `useGraphAuthoring`.

Commands sent:

```text
delete node {alias}
```

One command is produced per selected node alias.

Pending shape:

```ts
interface PendingBatchDeleteSubmit {
  action: 'delete-nodes';
  aliases: string[];
  commands: string[];
  sentAt: string;
  results: Record<string, 'success' | 'error'>;
}
```

Invalid input behavior:

- empty selected node array: no command is sent;
- any invalid/stale alias: no command is sent;
- existing pending authoring action: no command is sent;
- more than `MAX_BATCH_NODE_ACTIONS` selected nodes: no command is sent.

The implementation can use a renamed constant such as `MAX_BATCH_NODE_ACTIONS` if it is shared by batch clip and batch delete.

### Multi-Select Tip State

Producer and owner: `GraphView`.

Storage: in-memory React state only.

```ts
interface GraphMultiSelectTipState {
  hasShownThisSession: boolean;
  visible: boolean;
  fading: boolean;
}
```

Invalid input behavior:

- if graph data is empty, tip state must not transition to visible;
- if render error exists, tip must not show;
- if the component unmounts while a fade/auto-dismiss timer is pending, clear the timer.

## Limits

```ts
const MAX_BATCH_NODE_ACTIONS = 100;
```

If more than 100 nodes are selected, do not run the batch action. Show the operation-specific message:

```text
Select 100 or fewer nodes to clip at once.
Select 100 or fewer nodes to delete at once.
```

The limit exists because batch clip performs one IndexedDB write path per selected node and batch delete performs one backend command/result path per selected node. It can be revisited after a browser-level performance spike and backend command-flow review for larger selections.

## Accessibility And Interaction Requirements

### Context Menu

1. The multi-node menu must use `role="menu"` like the existing node context menu.
2. Each batch menu item must be focusable when enabled.
3. Escape closes the menu.
4. Outside pointer down closes the menu.
5. The menu label should include the selected count for screen readers:

```tsx
aria-label={`Actions for ${selectedCount} selected nodes`}
```

6. The multi-delete confirmation state must focus the destructive `Delete` confirmation button, matching the existing single-node confirmation pattern.

### Multi-Select Tip

1. The tip must use `role="status"`.
2. It must not auto-focus.
3. It must be dismissible by click.
4. It must be readable without opening help docs.
5. It must not be the only path to learning the shortcut if a future toolbar help affordance is added.
6. It must not introduce hover-only controls.

## Failure Handling

| Failure | Handling | User Feedback |
|---|---|---|
| No selected nodes after stale filtering for clip | Return without DB writes | `No selected nodes are available to clip.` |
| No selected nodes after stale filtering for delete | Return without backend sends | `No selected nodes are available to delete.` |
| More than 100 selected nodes for clip | Return without DB writes | `Select 100 or fewer nodes to clip at once.` |
| More than 100 selected nodes for delete | Return without backend sends | `Select 100 or fewer nodes to delete at once.` |
| Some selected aliases no longer exist | Drop stale aliases | Summary reflects only current nodes |
| Duplicate alias in Workspace | Count as duplicate, skip replacement dialog | Summary includes already-existed count |
| IndexedDB add failure | Count as failed | Summary includes failed count |
| All clip nodes fail | Error toast | `Failed to clip selected nodes to workspace.` |
| Delete validation failure | Reject whole batch before send | First validation error |
| WebSocket closed before delete send | Reject whole batch before send | `Could not send delete-node commands because the WebSocket is not open.` |
| WebSocket disconnects after partial delete send | Keep outcome unknown and require refresh | `Connection disconnected while the node action was pending. The outcome is unknown. Refresh the page and check the graph before trying again.` |
| Some backend delete results fail | Count success/error and finish batch | `2 selected nodes deleted. 1 failed.` |
| All backend delete results fail | Error toast | `Failed to delete selected nodes.` |
| Delete result timeout | Clear pending batch and require refresh | `Some delete-node commands were sent, but not all backend results were observed yet. Refresh the graph before trying again.` |
| Graph data missing | Return without DB writes or backend sends | Operation-specific no-op message |
| User closes menu | No mutation | No toast |
| Tip timer fires after unmount | Timer cleanup prevents setState | No user-visible error |

## Observability And Debuggability

Batch clip does not call the backend. Batch delete does call the backend through the existing raw text command path and correlates results by delete action plus node alias.

Developer debugging path:

1. `GraphView.tsx` for selected aliases, context target resolution, and multi-select tip state.
2. `GraphMultiSelectTip.tsx` if extracted.
3. `NodeContextMenu.tsx` for single vs multi menu rendering.
4. `Playground.tsx` for batch clip result aggregation.
5. `ClipboardContext.tsx` for per-node add/duplicate/error behavior.
6. Browser IndexedDB `minigraph-clipboard` store for persisted Workspace items.
7. `useGraphAuthoring.ts` for batch delete pending state, timeout, disconnect handling, and backend text-result matching.

Console logging is not required for successful batch clips or batch deletes. Existing `ClipboardContext` DB open warning behavior remains unchanged.

## Verification Plan

### Unit Tests

Add tests for selection target resolution:

- clicked selected node with two selected aliases returns `multi-node`;
- clicked unselected node with two selected aliases returns `single-node`;
- one selected alias returns `single-node`;
- stale aliases are filtered out against current graph data;
- selected aliases are deduplicated.
- transformed React Flow graph nodes include the class/attribute needed to prevent pane-level `Shift` selection capture from swallowing node-level `Shift + click`.

Add tests for batch summary formatting:

- all added;
- added plus duplicate;
- all duplicate;
- added plus failure;
- all failed;
- empty input.

Add tests for batch delete state/result handling:

- rejects empty node array;
- rejects stale alias before sending;
- rejects when another node action is pending;
- builds one `delete node {alias}` command per selected node;
- matches backend delete result events by alias;
- completes when every alias has a terminal success/error result;
- times out when at least one alias has no terminal result.

Add tests for multi-select tip state if extracted into a helper:

- tip shows when graph first becomes non-empty;
- tip does not show for empty graph;
- tip does not show again in the same page session after dismissal;
- graph interaction dismisses the tip;
- timer cleanup is safe on unmount.

### Component/Integration Tests If Existing Test Setup Allows

- `NodeContextMenu` in single-node mode shows `Clip to Workspace`, `Edit Node`, `Delete Node`.
- `NodeContextMenu` in multi-node mode shows `Clip N selected nodes to Workspace` and `Delete N selected nodes`.
- `NodeContextMenu` in multi-node mode does not show `Edit Node`.
- `NodeContextMenu` requires confirmation before calling `onDeleteSelectedNodes`.
- disabled or over-limit multi-node action does not call handler.
- `GraphMultiSelectTip` renders the expected copy and calls dismiss on click.
- if component-level graph tests are available, `Shift + click` on an unselected node with no previous selection selects it immediately.
- if component-level graph tests are available, a partial-intersection selection rectangle selects touched nodes.
- if component-level graph tests are available, the completed selection group rectangle does not block right-click on selected nodes.

### Manual QA

1. Import a graph with at least three nodes.
2. Verify the multi-select tip appears in the graph canvas upper-left area.
3. Verify clicking the tip dismisses it.
4. Reload, import again, and verify the tip can appear again in a new page session.
5. Reload, import again, interact with the graph, and verify the tip dismisses.
6. Hold `Shift` and click an unselected node when nothing is selected. Verify it becomes selected immediately.
7. Continue holding `Shift` and click a second node. Verify both nodes are selected.
8. Continue holding `Shift` and click one selected node again. Verify it is removed from selection.
9. `Shift + drag` from empty canvas and verify the active selection rectangle visually follows the pointer drag area.
10. Drag a selection rectangle that only touches part of a node. Verify the touched node is selected without being fully enclosed.
11. Start `Shift + drag` from a node. Verify it does not create a selection box.
12. After box selection ends, verify there is no large group bounding rectangle that looks like the original marquee.
13. After box selection ends, verify selected nodes still show individual selected styling.
14. After box selection ends, right-click one selected node.
15. Verify `Clip N selected nodes to Workspace` appears.
16. Verify `Delete N selected nodes` appears when connected and authoring is supported.
17. Verify `Edit Node` does not appear in the multi-node menu.
18. Click clip and verify selected nodes appear as separate Workspace items.
19. Repeat clip with one selected node already in Workspace and verify one summary toast.
20. Re-select two nodes, click delete, verify confirmation appears, cancel, and verify no command is sent.
21. Re-select two nodes, confirm delete, and verify one summary message after backend results.
22. Right-click an unselected node while multiple nodes are selected and verify the single-node menu appears.
23. Refresh/import a different graph and verify old selection does not survive.
24. Verify single-node edit/delete still work.

## Implementation Slices

### Slice 1: Selection Target Plumbing

Files:

- `src/components/GraphView/GraphView.tsx`
- optional `src/components/GraphView/selectionTargets.ts`
- optional `src/components/GraphView/__tests__/selectionTargets.test.ts`

Behavior:

- enable `Shift + click` and `Shift + drag`;
- track selected aliases;
- resolve right-click target.

Rollback:

- remove React Flow selection props and selected alias state.

### Slice 2: Multi-Mode Context Menu

Files:

- `src/components/GraphView/NodeContextMenu.tsx`
- `src/components/GraphView/NodeContextMenu.module.css` only if styling is needed.

Behavior:

- single-node menu stays unchanged;
- multi-node menu exposes batch clip and batch delete;
- multi-node menu never exposes edit;
- menu items include selected count;
- delete requires a confirmation state before sending.

Rollback:

- revert prop changes and render only the existing single-node actions.

### Slice 3: Batch Clip Handler

Files:

- `src/components/Playground.tsx`
- `src/components/RightPanel/RightPanel.tsx`
- optional `src/clipboard/batchClipSummary.ts`
- optional `src/clipboard/__tests__/batchClipSummary.test.ts`

Behavior:

- pass `onClipNodes`;
- call `clipboardCtx.clipNode` once per selected node;
- summarize added/duplicate/failed results.

Rollback:

- remove `onClipNodes` prop path; single-node clip remains unchanged.

### Slice 4: Multi-Select Tip Discoverability

Files:

- `src/components/GraphView/GraphView.tsx`
- optional `src/components/GraphView/GraphMultiSelectTip.tsx`
- `src/components/GraphView/GraphView.module.css`
- optional component/helper tests.

Behavior:

- show a lightweight tip after first non-empty graph appears in the current page session;
- dismiss on click, graph interaction, or 5-second timeout;
- do not persist dismissed state.

Rollback:

- remove tip component and state; multi-select behavior still works.

### Slice 5: Batch Delete Authoring

Files:

- `src/components/GraphAuthoring/useGraphAuthoring.ts`
- `src/graphActions/minigraphCommandBuilder.ts` only if shared batch helpers are useful
- `src/graphActions/validation.ts` only if batch validation helper is useful
- focused authoring tests.

Behavior:

- add `deleteNodes(nodes)`;
- validate every selected alias before sending;
- send one `delete node {alias}` command per selected node;
- track a multi-alias pending delete state;
- aggregate backend text-result events into one user message.

Rollback:

- remove `deleteNodes` and `onDeleteNodes`; single-node delete remains unchanged.

### Slice 6: Verification And Polish

Files:

- focused unit tests;
- manual QA notes if needed.

Behavior:

- cover target resolution, summary logic, and tip behavior;
- cover batch delete validation/result aggregation;
- verify existing create/edit/delete node behavior still works.

## Decision Log

| Decision | Alternatives | Why Selected | Tradeoff |
|---|---|---|---|
| Use React Flow selection | Hover checkboxes, custom pointer state | Uses existing graph primitive and selected rendering | Need to understand React Flow selection props |
| Store selected aliases in `GraphView` only | Store in global context, persist selection | Selection is graph-local UI state | Selection disappears on refresh |
| Batch as separate Workspace items | Grouped Workspace item | No DB migration, existing paste works | Does not preserve a group paste operation |
| Skip duplicates in batch | Open duplicate dialog for each duplicate | Avoids repeated dialogs | User cannot replace duplicates from batch action |
| Support multi-delete but not multi-edit | Hide both actions, show both actions | Delete can use one confirmation and command per alias; edit would require multiple forms/modals | Batch delete needs multi-result pending tracking |
| Limit batch to 100 nodes | Unlimited batch | Keeps UI work bounded | Large selections require smaller batches |
| Show tip after first graph appears | Only docs/help panel, only toolbar tooltip, first node click | Makes the shortcut visible at the point the feature becomes relevant | Users who only inspect the graph still see the tip |
| Do not persist tip dismissal | Persist in localStorage | Matches existing help hint being session/app-load oriented | Tip can appear again after page reload |

## Appendix - Planning Artifact (from /spec-plan)

**Phase 0 - Domain calibration**
- Domain mix: Frontend UI/rendering 30%, state management/data flow 30%, data persistence 15%, network/protocol result matching 10%, product workflow/UX 10%, security/privacy 5%.
- Dominant failure mode: composition bugs and multiple sources of truth between React Flow selection, context menu target state, Workspace storage, batch delete pending state, and discoverability tip state.
- Pre-mortem watch-items: wrong right-click target after multi-select; duplicate dialogs opening once per node; stale selected aliases after graph refresh; tip blocking canvas interaction; multi-delete backend results overwriting the existing single-delete pending state.
- Calibration: emphasized Step 5 source of truth, Step 9 composition audit, Step 11 lifecycle cleanup, and Step 12 per-N batch write cost.

**Step 1 - Requirement and journeys**
- Actor / trigger / success signal: graph user imports a graph, sees a multi-select tip, selects nodes with `Shift + left click` or `Shift + drag`, right-clicks selected nodes, and either clips selected nodes to Workspace or deletes selected nodes after confirmation with a summary message.
- Primary journeys: graph import shows tip; tip dismisses on interaction; Shift-click multi-select; Shift-drag box-select; single selected node right-click; multi-selected node right-click; batch clip; batch delete; graph refresh after selection.
- Non-goals: multi-edit, grouped Workspace item, backend protocol change, checkbox hover UI, persisted selection, persisted tip dismissal.

**Step 3 - Anti-anchoring and compatibility**
- Silent assumptions: node alias is unique; `graphData` is the source of truth for node snapshots; Workspace items remain single-node records; duplicate detection remains alias-based; selection and tip state are UI-only.
- Fresh-team delta: a fresh team might build checkbox overlays, a grouped clipboard item, or a global onboarding modal. This spec keeps React Flow selection, single-node Workspace records, and a graph-scoped hint.
- Existing behavior contract: single-node clip extended; single-node edit unchanged; single-node delete unchanged; Workspace DB schema unchanged; backend command protocol unchanged; delete result matching extended to support a batch pending state; help hint pattern referenced but not modified.

**Step 4 - Three architectures**
- Option A - root primitive: custom hover checkbox selection with component-owned state.
- Option B - root primitive: React Flow selection props with context menu target resolution and graph-scoped tip.
- Option C - root primitive: new grouped Workspace record and group paste model.
- Picked: Option B.
- Axis spread verified: options vary by UI primitive, storage strategy, abstraction level, and migration scope.

**Step 5 - Source of truth and boundaries**
- Source-of-truth inventory: graph data from backend/import flow; selected node visual state from React Flow; selected alias mirror in `GraphView`; context target snapshot in `GraphView`; multi-select tip state in `GraphView`; Workspace items in `ClipboardContext` and IndexedDB; batch delete pending state in `useGraphAuthoring`.
- Boundary map: UI boundary between React Flow and app components; persistence boundary into IndexedDB; async boundary through per-node `clipNode`; backend raw-command/text-result boundary through `GraphAuthoringExecutor` and protocol parser; cross-tab boundary through BroadcastChannel.

**Step 6 - Contracts**
- Boundary-crossing contracts: `GraphClipItem`, context menu mode, existing `ClipboardItemRecord`, batch clip summary counts, batch delete pending/result counts, in-memory `GraphMultiSelectTipState`.
- Shared constants / schema strategy: `MAX_BATCH_NODE_ACTIONS = 100`; no DB version bump; no backend constants; no localStorage key for the tip.

**Step 9 - Primitive fit and composition**
- Under-use / over-use findings: React Flow already provides selection primitives; custom checkbox selection would under-use the graph framework.
- Composed primitives: React Flow selection, app context menu, graph tip overlay, ClipboardContext, IndexedDB, GraphAuthoringExecutor, protocol text-result events, BroadcastChannel.
- Overlaps: node right-click vs selected set; selection box vs pan; tip overlay vs canvas pointer interactions; batch clip vs duplicate dialog; batch delete vs single pending authoring action; graph refresh vs open menu.
- Guardrails: `selectionKeyCode="Shift"`, `multiSelectionKeyCode="Shift"`, `selectionOnDrag={false}`, context target snapshot, duplicate summary instead of repeated dialogs, dedicated batch delete pending shape, selection cleanup on graph refresh, tip dismisses on graph interaction.

**Step 10 - From-scratch comparison**
- Materially simpler?: no.
- If yes, redesign adopted: not applicable.

**Step 11 - Failure and lifecycle**
- Failure matrix summary: covers stale selection, empty selection, over-limit selection, duplicates, IndexedDB failures, delete validation/send failures, partial backend delete results, timeout/disconnect, user cancellation, graph data missing, and tip timer cleanup.
- Lifecycle data-access findings: selected aliases read from `onSelectionChange`; current nodes read from `graphData` at menu action time; tip visibility follows graph availability; IndexedDB writes are async and summarized after completion; delete result events are matched only while a batch pending state exists.

**Step 12 - Spike / performance**
- Spike required?: yes, because batch clip performs per-node IndexedDB writes and batch delete performs one backend command/result path per selected node.
- Result or reason not required: local fake-indexedDB spike measured 1 item at 2.3 ms and 100 items at 9.717 ms, about 0.097 ms per item in that environment.
- Per-N cost: one `clipNode` call and one IndexedDB add path per selected node for clip; one raw delete command and one expected text-result event per selected node for delete; both capped at 100 nodes.

**Step 13 - Security / trust**
- Input boundaries: graph data from existing backend/import flow; selected aliases from UI; Workspace writes to local IndexedDB; delete commands cross the existing backend raw-command boundary.
- Validation / auth / encoding: selected aliases are filtered against current `graphData`; delete aliases use existing alias validation and command builder; no backend auth change; React text rendering remains escaped by React; Workspace stores existing graph node objects.

**Step 14 - Observability / rollout**
- Debuggability: inspect `GraphView`, optional `GraphMultiSelectTip`, `NodeContextMenu`, `Playground.handleClipNodes`, `useGraphAuthoring.deleteNodes`, `ClipboardContext`, browser IndexedDB, and protocol text-result events.
- Rollout / rollback: no feature flag required; rollback removes multi-select props, `onClipNodes`, `onDeleteNodes`, the graph tip, and `deleteNodes`; single-node clip/edit/delete remain.

**Step 15 - Verification**
- Verification mapping summary: selection target helper by unit test; batch clip summary helper by unit test; batch delete pending/result handling by hook/unit tests; tip behavior by helper/component test; menu rendering by component test if available; React Flow behavior by manual QA; DB schema unchanged by existing clipboard tests.
- Design cross-reference complete: yes.

**Step 16 - Implementation / consumer readiness**
- Implementation slices: selection target plumbing; multi-mode context menu; batch clip handler; multi-select tip discoverability; batch delete authoring; verification and polish.
- Review questions / decision log: approve separate Workspace items instead of grouped item; approve duplicate skip behavior; approve multi-delete without multi-edit; approve 100-node batch limit; approve session-only graph tip behavior.

**Step 17 - Pre-draft self-check**
- All self-check answers yes?: yes.
- Enumeration completeness: selection creation through React Flow; mutation through user selection; cleanup on pane click/menu close/graph refresh; tip creation on first non-empty graph; tip cleanup on click/graph interaction/timer/unmount; batch clip trigger through context menu; batch delete trigger through context menu confirmation.
- Cross-iteration regression: preserves existing single-node menu, existing Workspace schema, existing duplicate replacement dialog for single-node clip, existing no-backend clip behavior, existing single-node delete behavior, and existing help hint as a separate feature.
