# Epic 4 Session Collaboration UI Spec

**Audience:** frontend and full-stack engineers implementing session collaboration UI in the Minigraph playground  
**Status:** implementation-ready after backend contract confirmation  
**Scope:** Minigraph playground only

---

## 1. Goal

Add a top navigation **Session** dropdown, visually consistent with the existing **Tools** and **Quick Links** dropdowns, so users can:

1. See their current host session ID.
2. See which subscriber sessions are connected to their session.
3. See whether this session is currently subscribed to another host session.
4. Subscribe to another host session by session ID.
5. Unsubscribe from the current host session.
6. See live connection status with green indicators for active session relationships.
7. Automatically remove disconnected subscribers from the UI when the backend reports the disconnect.

The feature must use the existing WebSocket command path and backend `session` commands. It must not create a second WebSocket connection or duplicate the message-classification pipeline.

---

## 2. User-Facing Requirements

### 2.1 Session Menu Entry

Add a **Session** button in the top navigation area near **Tools** and **Quick Links**.

The button must:

- use the existing `NavMenu` dropdown styling and keyboard behavior;
- show an aggregate status dot;
- show green when the Minigraph WebSocket is connected and session status is known;
- show connecting/idle styling consistent with existing nav status dots when the WebSocket is connecting or disconnected;
- be visible only for playgrounds that support session collaboration.

The first implementation is Minigraph-only. JSON-Path must not show a non-working Session menu.

### 2.2 Connected State

When the user opens the Session menu while Minigraph is connected:

1. The UI sends or refreshes the backend `session` command.
2. The menu shows this session's host session ID.
3. The menu shows an optional **Subscribed to** row when this session is following another host.
4. The menu shows a **Subscribers** section listing active subscribers connected to this session.
5. Each active session relationship uses a green status dot.
6. If there are no subscribers, show an empty state:

```text
No subscribers connected.
```

The current session ID must be easy to copy. Provide a small **Copy** button beside the session ID. Copying uses `navigator.clipboard.writeText` and shows a success or failure toast.

### 2.3 Disconnected State

When the Minigraph WebSocket is disconnected:

- the Session dropdown must not show stale subscriber or target data;
- the subscribe input and action buttons are disabled;
- the menu shows:

```text
Connect Minigraph to view session details.
```

Local disconnect cleanup must clear:

- current session ID;
- subscribed target;
- subscriber list;
- pending session command status;
- stale error messages that came from an earlier connection.

### 2.4 Subscribe Flow

The Session menu includes a small form:

- input label: `Subscribe to session`;
- placeholder: `ws-123456-1`;
- action button: `Subscribe`;
- disabled when disconnected, input is blank, or a session command is already pending.

On submit:

1. Trim the input.
2. Validate the session ID format.
3. Send raw text through the existing WebSocket send path:

```text
session subscribe {sessionId}
```

4. Keep the menu open.
5. Show pending state on the subscribe button.
6. Wait for backend result text.
7. On success, clear the input and refresh session status by sending `session`.
8. On rejection, show the backend message inside the menu and as a toast.

Allowed session ID format:

```text
ws-{digits}-{digits}
```

The UI must reject invalid input before sending:

```text
Enter a valid session ID like ws-123456-1.
```

### 2.5 Unsubscribe Flow

When this session is subscribed to another host session, show an **Unsubscribe** button in the subscribed-target section.

On click:

1. Send:

```text
session unsubscribe
```

2. Keep the menu open.
3. Show pending state.
4. On success, clear the subscribed-target row and refresh session status.
5. On rejection, keep current state and show the backend message.

Do not show **Unsubscribe** when this session is not subscribed to another host.

### 2.6 Subscriber Disconnect Behavior

When a subscriber disconnects, unsubscribes, or is removed by backend session cleanup:

1. The backend sends a session notification to the host.
2. The frontend parses that notification into a typed session event.
3. The Session controller refreshes status by sending `session`.
4. The disconnected subscriber disappears from the subscriber list.

The UI must not keep a disconnected subscriber visible with a red indicator. The requirement is removal, not retained offline state.

### 2.7 Host Closed Behavior

When this session is subscribed to another host and the host closes or resets:

1. The backend sends:

```text
Session {hostSessionId} has closed
```

2. The frontend clears `subscribedTo`.
3. The UI returns to primary/host-only state.
4. The menu shows an informational message:

```text
Host session {hostSessionId} disconnected.
```

The graph data lifecycle remains governed by the existing graph refresh/session reset logic. This feature only owns session UI state.

---

## 3. Explicit Non-Goals

- No new backend persistence.
- No new WebSocket endpoint.
- No new runtime dependency.
- No auth or permission redesign.
- No multi-user Workspace storage.
- No cross-browser shared workspace implementation.
- No polling interval unless backend notifications prove insufficient.
- No JSON-Path session UI in this epic.
- No modal UI for session management.
- No attempt to infer subscriber state from graph mutations.

---

## 4. Current Architecture Context

### 4.1 Existing Navigation Pattern

`Navigation.tsx` renders the existing top nav dropdowns through `NavMenu`:

- **Tools** dropdown at `Navigation.tsx`;
- **Quick Links** dropdown at `Navigation.tsx`;
- `NavMenu` owns open/close, outside click, Escape handling, ARIA dropdown state, and optional status dots.

The Session menu must reuse this dropdown primitive.

### 4.2 Existing WebSocket And Protocol Path

Current Minigraph command flow:

1. `Playground.tsx` creates one `ProtocolBus` per playground.
2. `useWebSocket` sends raw text through the shared `WebSocketContext`.
3. `WebSocketContext` owns socket connection state and message storage.
4. `useProtocolKernel` classifies new messages exactly once through `classifyMessage`.
5. Feature hooks subscribe to typed protocol events from `ProtocolBus`.

The Session UI must follow the same pattern:

- do not scan `messages` directly inside `SessionMenu`;
- do not open another WebSocket;
- do not parse session messages in React components;
- do not bypass `sendRawText`.

### 4.3 Existing Backend Session Commands

The backend already supports session collaboration commands:

```text
session
session subscribe {session-id}
session unsubscribe
session reset
```

Relevant backend behavior:

- `session` returns current session ID, optional subscribed target, and optional subscriber list.
- `session subscribe {id}` subscribes this session to a primary session.
- `session unsubscribe` detaches this session from its current target.
- host reset or close notifies subscribers.
- subscriber unsubscribe or close notifies the host.

The UI consumes this existing text protocol and centralizes parsing in the frontend protocol layer.

---

## 5. Backend Command Contract

The UI sends raw text commands through the existing WebSocket `sendRawText` path.

### 5.1 Show Session

Producer: frontend Session controller  
Consumer: `GraphCommandService.handleSessionCommand`

```text
session
```

Expected status response:

```text
Session ws-178443-2 started since 2026-06-02 10:20:32.054
subscribed to ws-111111-1
subscribed by [ws-485844-4, ws-222222-3]
```

Rules:

- first line is required for a status response;
- `subscribed to ...` is optional;
- `subscribed by [...]` is optional;
- subscriber list may contain zero, one, or many IDs;
- whitespace after commas must be accepted;
- echoed command lines beginning with `> ` are not status responses.

### 5.2 Subscribe

Producer: frontend Session controller  
Consumer: `GraphCommandService.subscribeSession`

```text
session subscribe {sessionId}
```

Expected success response to subscriber:

```text
Subscribed to ws-178443-2
```

Expected notification to host:

```text
ws-485844-4 subscribed to your session
```

Expected rejection responses:

```text
You cannot subscribe to yourself
Session ws-000000-0 not found
ws-485844-4 is not a primary session
You have already subscribed to ws-178443-2
Please do 'session reset' before subscribing to another session
```

### 5.3 Unsubscribe

Producer: frontend Session controller  
Consumer: `GraphCommandService.unsubscribeSession`

```text
session unsubscribe
```

Expected success response to subscriber:

```text
Session unsubscribed from ws-178443-2
```

Expected notification to previous host:

```text
ws-485844-4 unsubscribed from your session
```

Expected rejection response:

```text
Nothing to unsubscribe
```

### 5.4 Host Closed Or Reset

Expected notification to subscribers:

```text
Session ws-178443-2 has closed
```

Existing `session reset` behavior and `session.reset` graph invalidation remain unchanged. This spec extends session UI state handling for the same backend event family.

---

## 6. Frontend Protocol Events

Add typed protocol events so Session UI never depends on raw string parsing.

### 6.1 Event Types

Add these event interfaces to `src/protocol/events.ts`:

```ts
export interface SessionStartedEvent extends ProtocolEventBase {
  kind: 'minigraph.session.started';
  sessionId: string;
  companionEndpoint: string | null;
}

export interface SessionStatusEvent extends ProtocolEventBase {
  kind: 'minigraph.session.status';
  sessionId: string;
  startedSince: string;
  subscribedTo: string | null;
  subscribers: string[];
}

export interface SessionCommandResultEvent extends ProtocolEventBase {
  kind: 'minigraph.session.commandResult';
  command: 'subscribe' | 'unsubscribe' | 'show' | 'unknown';
  status: 'accepted' | 'rejected' | 'info';
  sessionId: string | null;
  message: string;
}

export interface SessionNotificationEvent extends ProtocolEventBase {
  kind: 'minigraph.session.notification';
  type: 'subscriber-joined' | 'subscriber-left' | 'host-closed';
  sessionId: string;
  message: string;
}
```

### 6.2 Parser Functions

Add session parser helpers in `messageParser.ts` or a dedicated parser module imported by `messageParser.ts`.

Required exported helpers:

```ts
parseSessionStarted(raw): SessionStartedParseResult | null
parseSessionStatus(raw): SessionStatusParseResult | null
parseSessionCommandResult(raw): SessionCommandResultParseResult | null
parseSessionNotification(raw): SessionNotificationParseResult | null
```

All session regex constants must live with these helpers. Do not duplicate regex patterns in components or hooks.

### 6.3 Classifier Rules

Extend `classifyMessage` after JSON handling and before `docs.response` fallback:

1. Parse `minigraph.session.started`.
2. Parse `minigraph.session.status`.
3. Parse `minigraph.session.commandResult`.
4. Parse `minigraph.session.notification`.

Rules:

- echoed commands beginning with `> ` must not produce session status/result events;
- the existing `session.reset` event must remain for exact `Session restarted`;
- a message may produce both `session.reset` and a session command result only if explicitly useful; otherwise preserve current `session.reset` behavior and avoid duplicate UI handling.

---

## 7. Frontend State Model

Create a Minigraph-only session collaboration hook:

```ts
useSessionCollaboration({
  bus,
  connected,
  sendRawText,
  addToast,
})
```

### 7.1 Hook State

The hook owns:

```ts
interface SessionUiState {
  sessionId: string | null;
  startedSince: string | null;
  subscribedTo: string | null;
  subscribers: string[];
  loading: boolean;
  pendingCommand: 'refresh' | 'subscribe' | 'unsubscribe' | null;
  error: string | null;
  lastInfo: string | null;
}
```

Derived values:

```ts
isPrimary = subscribedTo === null
hasSubscribers = subscribers.length > 0
canSubscribe = connected && pendingCommand === null
canUnsubscribe = connected && subscribedTo !== null && pendingCommand === null
```

### 7.2 Source Of Truth

Backend `GraphSession` is the source of truth.

Frontend state is a derived cache only:

- created from `minigraph.session.started`;
- refreshed from `minigraph.session.status`;
- updated or invalidated after `minigraph.session.notification`;
- cleared on local WebSocket disconnect.

### 7.3 Refresh Strategy

The hook sends `session`:

1. when the menu opens and WebSocket is connected;
2. after `minigraph.session.started`;
3. after subscribe success;
4. after unsubscribe success;
5. after subscriber join/leave notification;
6. after host closed notification if still connected.

Do not add a repeating interval in the first implementation. Backend already pushes relationship-change notifications.

### 7.4 Pending Command Behavior

Pending commands are UI-only state. They must clear when:

- matching command result is observed;
- WebSocket disconnects;
- a 5-second timeout expires.

Timeout message:

```text
Session command was sent, but no confirmation was observed. Refresh session details before trying again.
```

---

## 8. UI Design

### 8.1 Menu Layout

Recommended layout:

```text
Session
────────────────────────
This session
ws-178443-2        Copy

Subscribed to
● ws-111111-1      Unsubscribe

Subscribers
● ws-485844-4
● ws-222222-3

Subscribe to session
[ ws-123456-1        ]
[ Subscribe          ]

Last updated just now
```

If no target:

```text
Subscribed to
Not subscribed to another session.
```

If no subscribers:

```text
Subscribers
No subscribers connected.
```

### 8.2 Styling

- Reuse `NavMenu` for dropdown shell.
- Create `SessionMenu.module.css` for content-specific layout.
- Use the same CSS variables as `Navigation.module.css`.
- Status dots:
  - connected relationship: `var(--success-color)`;
  - pending/loading: `var(--warning-color)`;
  - disconnected local WebSocket: existing idle/disconnected nav dot style.
- Keep the dropdown compact but readable. Minimum width: 300px.
- Long session IDs must not overflow; use monospace text and wrapping or truncation with title tooltip.

### 8.3 Accessibility

- The Session trigger uses `NavMenu` ARIA behavior.
- Subscribe input has a visible or screen-reader accessible label.
- Copy, Subscribe, and Unsubscribe buttons must be keyboard reachable.
- Pressing Enter in the subscribe input submits the subscribe command.
- Escape closes the dropdown through existing `NavMenu` behavior.
- Button disabled states must use `disabled`, not only CSS.

---

## 9. Dual-Role Semantics

Every connected browser session has its own session ID and can be shown as a host identity.

The UI supports both directions:

1. Other sessions can subscribe to this session.
2. This session can subscribe to another primary session.

If this session has active subscribers and the user subscribes to another host, the UI must not guess what backend sync topology means. The spec requires one of these backend-confirmed behaviors before implementation is considered complete:

1. Backend supports the dual-role topology and all current subscribers continue receiving synchronized commands/results correctly.
2. Backend rejects the request with a clear message.

If backend behavior is not already covered by tests, add or update backend tests before enabling the UI path for host-with-subscribers subscribe. Until then, the frontend may send the command but must show backend rejection or refresh status from the backend result. It must not locally invent a topology.

---

## 10. Error And Empty States

| Condition | UI behavior |
|---|---|
| WebSocket disconnected | Disable controls, clear stale state, show connect message |
| Session status not loaded yet | Show compact loading state after menu open |
| Invalid input ID | Inline validation message, do not send command |
| Subscribe to self | Show backend rejection |
| Unknown target session | Show backend rejection |
| Target not primary | Show backend rejection |
| Already subscribed | Show backend rejection and keep current target |
| Unsubscribe while primary | Show backend rejection |
| Host closed | Clear target, show host disconnected info |
| Subscriber left | Refresh and remove subscriber from list |
| Parser fails on unknown text | Leave message as console/docs response; do not mutate session UI |
| Clipboard copy fails | Show copy failure toast |

---

## 11. File-Level Implementation Plan

### 11.1 New Files

```text
src/components/SessionMenu/SessionMenu.tsx
src/components/SessionMenu/SessionMenu.module.css
src/session/useSessionCollaboration.ts
src/session/sessionTypes.ts
src/session/sessionParser.ts
src/session/__tests__/sessionParser.test.ts
src/session/__tests__/useSessionCollaboration.test.ts
```

If the implementation keeps parser helpers in `utils/messageParser.ts`, then `sessionParser.ts` may be omitted. The important rule is centralization: parser patterns must not live in UI components.

### 11.2 Modified Files

```text
src/components/Playground.tsx
src/components/Navigation.tsx
src/protocol/events.ts
src/protocol/classifier.ts
src/utils/messageParser.ts
src/protocol/__tests__/classifier.test.ts
```

Optional if current backend behavior is not covered:

```text
src/main/java/com/accenture/minigraph/services/GraphCommandService.java
src/test/java/com/accenture/minigraph/playground/SessionManagementTest.java
```

### 11.3 Wiring

`Playground.tsx` must create the session controller because it owns:

- `bus`;
- `ws.connected`;
- `ws.sendRawText`;
- `addToast`.

Example shape:

```tsx
const sessionCollaboration = supportsSessionCollaboration
  ? useSessionCollaboration({
      bus,
      connected: ws.connected,
      sendRawText: ws.sendRawText,
      addToast,
    })
  : null;

<Navigation
  addToast={addToast}
  sessionCollaboration={sessionCollaboration}
/>
```

`Navigation.tsx` then renders:

```tsx
{sessionCollaboration && (
  <SessionMenu controller={sessionCollaboration} />
)}
```

Do not move `ProtocolBus` into `Navigation`. Do not create another bus there.

---

## 12. Verification Plan

### 12.1 Unit Tests

Parser tests:

- parse WebSocket start message:

```text
session ws-123456-1 started
Companion endpoint: /api/companion/ws-123456-1
```

- parse full status with target and subscribers;
- parse status with no target and no subscribers;
- parse one subscriber;
- parse multiple subscribers with spaces;
- parse subscribe success;
- parse host notification;
- parse unsubscribe success;
- parse subscriber-left notification;
- parse host-closed notification;
- parse known rejection messages;
- ignore echoed commands beginning with `> `;
- ignore unrelated docs/markdown messages.

Hook tests:

- menu-open refresh sends `session`;
- disconnect clears session UI state;
- subscribe validates input before send;
- subscribe success clears input and refreshes;
- subscribe rejection stores error;
- unsubscribe success clears target and refreshes;
- subscriber-left notification refreshes;
- host-closed notification clears target.

### 12.2 Component Tests

Session menu tests:

- connected state renders session ID and copy button;
- no subscribers empty state renders;
- subscribers render with green dots;
- disconnected state disables controls;
- Enter in input submits subscribe;
- Copy button calls clipboard API and toast callback;
- Escape closes through `NavMenu`.

### 12.3 Integration / Manual QA

Manual two-browser flow:

1. Start Minigraph in Browser A.
2. Open Session menu and copy A session ID.
3. Start Minigraph in Browser B.
4. In B, subscribe to A.
5. A shows B under Subscribers.
6. B shows subscribed target A.
7. Close B.
8. A removes B from Subscribers.
9. Reopen B and subscribe again.
10. Close/reset A.
11. B clears subscribed target and shows host disconnected info.

Host-with-subscriber dual-role flow:

1. A has subscriber B.
2. A attempts to subscribe to C.
3. Confirm backend either supports this topology correctly or rejects it clearly.
4. UI must reflect backend truth after refresh.

---

## 13. Rollout And Reversibility

Rollout:

- add `supportsSessionCollaboration?: boolean` to `PlaygroundConfig`;
- enable only for Minigraph;
- keep console commands available regardless of UI state.

Rollback:

- disable `supportsSessionCollaboration`;
- remove `SessionMenu` rendering;
- keep parser additions if harmless, or remove them with associated tests;
- backend session commands remain unchanged.

Compatibility:

- old backend text commands remain valid;
- users can still type `session`, `session subscribe ...`, and `session unsubscribe` manually;
- unknown session text remains visible in console even if not parsed.

---

## 14. Open Implementation Notes

1. Current backend emits text, not structured JSON. The frontend must centralize parsing and cover it with tests.
2. Current backend tests cover common subscribe/unsubscribe/reset flows. Confirm or add coverage for a host session that already has subscribers and then subscribes to another session.
3. The Session menu should refresh on open rather than continuously polling.
4. A subscriber is considered connected only if the backend includes it in the latest `session` status response.
5. This feature owns only session UI state. Graph rendering refresh and session reset graph invalidation remain in existing hooks.

---

## Appendix - Planning Artifact (from /spec-plan)

**Phase 0 - Domain calibration**
- Domain mix: Frontend UI/rendering 35%, state/data flow 25%, network protocol/RPC 25%, product UX 10%, backend platform contract 5%.
- Dominant failure mode: string-typed session protocol becoming scattered through UI code instead of centralized.
- Pre-mortem watch-items: magic-string parsing leaks into components; UI state disagrees with backend session truth; host/subscriber role changes are unclear after disconnect/reset.
- Calibration: emphasize source of truth, protocol contract, primitive composition, and observability.

**Step 1 - Requirement and journeys**
- Actor / trigger / success signal: Minigraph user opens Session dropdown, subscribes/unsubscribes, or observes subscriber changes; success is visible session ID, target, subscriber list, and accepted backend command result.
- Primary journeys: open menu and view status; subscribe to another host; receive subscriber join/leave; unsubscribe; host closed/reset while subscribed.
- Non-goals: no new persistence, no new WebSocket endpoint, no shared Workspace storage, no JSON-Path session UI.

**Step 3 - Anti-anchoring and compatibility**
- Silent assumptions: use existing `NavMenu`; backend owns session truth; console output remains visible; Minigraph-only; disconnect clears stale UI state.
- Fresh-team delta: a fresh team might build a new REST/JSON session endpoint. This spec keeps the existing WebSocket command contract to reduce backend scope.
- Existing behavior contract: Tools/Quick Links unchanged; console session commands unchanged; protocol classifier extended; backend text contract consumed, not replaced.

**Step 4 - Three architectures**
- Option A - root primitive: direct component-level parsing of WebSocket messages.
- Option B - root primitive: `NavMenu` + session hook + centralized parser/classifier events.
- Option C - root primitive: backend structured JSON session event API.
- Picked: Option B.
- Axis spread verified: options differ by parsing location, protocol shape, backend scope, and abstraction level.

**Step 5 - Source of truth and boundaries**
- Source-of-truth inventory: backend `GraphSession` owns session ID, target, subscribers; frontend hook owns derived UI cache; WebSocketContext owns connection phase.
- Boundary map: UI dropdown, WebSocket command, backend text output, protocol classifier, ProtocolBus event, async connection lifecycle.

**Step 6 - Contracts**
- Boundary-crossing contracts: `session`, `session subscribe`, `session unsubscribe`; status response; subscribe/unsubscribe notifications; host-closed notification; typed frontend session events.
- Shared constants / schema strategy: session regex patterns and parse result types live in one parser module; components consume typed events/state only.

**Step 9 - Primitive fit and composition**
- Under-use / over-use findings: existing `NavMenu`, WebSocketContext, and ProtocolBus fit the feature. A new socket or raw component parser would under-use current architecture.
- Composed primitives: `NavMenu` shell, SessionMenu content, useSessionCollaboration hook, WebSocket sendRawText, ProtocolBus events.
- Overlaps: dropdown close behavior with input/form controls, async command pending state with menu open/close, connection disconnect with pending commands.
- Guardrails: form events stop accidental menu close; pending state clears on disconnect/timeout; parser is centralized.

**Step 10 - From-scratch comparison**
- Materially simpler?: No. A structured JSON backend API is cleaner long-term but larger for this feature.
- If yes, redesign adopted: Not applicable.

**Step 11 - Failure and lifecycle**
- Failure matrix summary: disconnected socket, invalid ID, self-subscribe, unknown target, non-primary target, already subscribed, unsubscribe while primary, host closed, subscriber left, parser miss, timeout.
- Lifecycle data-access findings: session ID is available only after WebSocket start/status messages; clipboard copy may fail; sendRawText returns false when not connected.

**Step 12 - Spike / performance**
- Spike required?: No.
- Result or reason not required: subscriber count is small and rendering is O(N) per latest status.
- Per-N cost: one row per subscriber, one parser pass per WebSocket message through existing protocol kernel.

**Step 13 - Security / trust**
- Input boundaries: user-entered session ID and backend text messages.
- Validation / auth / encoding: validate `ws-{digits}-{digits}` before send; React escapes displayed IDs; backend remains responsible for session authorization/existence.

**Step 14 - Observability / rollout**
- Debuggability: raw backend messages remain in console; parsed events covered by classifier tests; user-visible toasts for accepted/rejected actions.
- Rollout / rollback: gate behind Minigraph config flag; disabling the flag removes UI while manual console commands still work.

**Step 15 - Verification**
- Verification mapping summary: parser/unit tests for contract; hook tests for state transitions; component tests for UI states; backend integration tests for dual-role semantics if needed.
- Design cross-reference complete: tests map to parser, classifier, hook, SessionMenu, and backend session semantics.

**Step 16 - Implementation / consumer readiness**
- Implementation slices: parser/types; hook; SessionMenu UI; Navigation/Playground wiring; tests; optional backend test coverage.
- Review questions / decision log: choose Option B to reuse current architecture; revisit Option C if backend session protocol becomes broader or more than UI needs consume it.

**Step 17 - Pre-draft self-check**
- All self-check answers yes?: Yes.
- Enumeration completeness: creation from start/status events, mutation from command/notification events, cleanup on disconnect/host closed/timeout, trigger on menu open and backend notifications.
- Cross-iteration regression: preserves prior centralized command/result parsing pattern from graph authoring; avoids prior issue of UI behavior depending on scattered raw strings.
