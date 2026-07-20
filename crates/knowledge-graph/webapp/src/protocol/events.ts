// ── Protocol Event Type Definitions ─────────────────────────────────────────
// Discriminated union on `kind`.  No runtime code — types only.

/** Base fields present on every protocol event. */
export interface ProtocolEventBase {
  /** Monotonic message ID from the WebSocket context. */
  msgId: number;
  /** The original raw string as received from the WebSocket. */
  raw: string;
}

// ── Individual event interfaces ────────────────────────────────────────────

export interface GraphLinkEvent extends ProtocolEventBase {
  kind: 'graph.link';
  /** Extracted API path, e.g. "/api/graph/model/my-graph/1" */
  apiPath: string;
}

export interface GraphMutationEvent extends ProtocolEventBase {
  kind: 'graph.mutation';
  mutationType: 'node-mutation' | 'import-graph';
}

export interface CreateNodeTextResultEvent extends ProtocolEventBase {
  kind: 'minigraph.createNode.textResult';
  status: 'accepted' | 'rejected' | 'error';
  alias: string | null;
  message: string;
}

export interface NodeActionTextResultEvent extends ProtocolEventBase {
  kind: 'minigraph.nodeAction.textResult';
  status: 'accepted' | 'rejected' | 'error';
  action: 'create-node' | 'edit-node' | 'delete-node' | null;
  alias: string | null;
  message: string;
}

export interface LargePayloadEvent extends ProtocolEventBase {
  kind: 'payload.large';
  apiPath: string;
  byteSize: number;
  filename: string;
}

export interface UploadInvitationEvent extends ProtocolEventBase {
  kind: 'upload.invitation';
  /** The POST path, e.g. "/api/mock/ws-417669-24" */
  uploadPath: string;
}

export interface UploadContentPathEvent extends ProtocolEventBase {
  kind: 'upload.contentPath';
  /** The POST path, e.g. "/api/json/content/ws-123-4" */
  uploadPath: string;
}

export interface CommandEchoEvent extends ProtocolEventBase {
  kind: 'command.echo';
  /** The command text without the "> " prefix. */
  commandText: string;
}

export interface HelpOrDescribeCommandEvent extends ProtocolEventBase {
  kind: 'command.helpOrDescribe';
  /** The command text without the "> " prefix. */
  commandText: string;
}

export interface ImportGraphCommandEvent extends ProtocolEventBase {
  kind: 'command.importGraph';
  /** The graph name from "import graph from {name}". */
  graphName: string;
}

export interface DocsResponseEvent extends ProtocolEventBase {
  kind: 'docs.response';
  /** True when the raw message is a markdown candidate (non-JSON, non-echo). */
  isMarkdown: true;
}

export interface JsonResponseEvent extends ProtocolEventBase {
  kind: 'json.response';
  /**
   * The parsed JSON object/array.  Non-null because `isJSON: true`
   * guarantees `data !== null` from `tryParseJSON`.
   */
  data: object;
}

export interface LifecycleEvent extends ProtocolEventBase {
  kind: 'lifecycle';
  /** The type string from the JSON payload. Known values: 'info', 'error', 'ping', 'welcome'. */
  type: string;
  /** True when `type` is one of the four known lifecycle values. */
  knownType: boolean;
  message: string;
  /**
   * Timestamp from the JSON payload, or null if absent.
   * Normalized: `parsed.time ?? null` to avoid `undefined`.
   */
  time: string | null;
}

export interface GraphExportedEvent extends ProtocolEventBase {
  kind: 'graph.exported';
  /** The user-supplied name extracted from the export API path. */
  graphName: string;
  /** e.g. "/api/graph/model/my-graph/1" */
  apiPath: string;
}

export interface GraphExportFailedEvent extends ProtocolEventBase {
  kind: 'graph.export.failed';
  reason: 'invalid-name' | 'root-name-conflict';
}

/** Catch-all for messages that match no other classifier rule. */
export interface UnclassifiedEvent extends ProtocolEventBase {
  kind: 'unclassified';
}

export interface SessionResetEvent extends ProtocolEventBase {
  kind: 'session.reset';
}

// ── Discriminated union ────────────────────────────────────────────────────

export type ProtocolEvent =
  | GraphLinkEvent
  | GraphMutationEvent
  | CreateNodeTextResultEvent
  | NodeActionTextResultEvent
  | GraphExportedEvent
  | GraphExportFailedEvent
  | LargePayloadEvent
  | UploadInvitationEvent
  | UploadContentPathEvent
  | CommandEchoEvent
  | HelpOrDescribeCommandEvent
  | ImportGraphCommandEvent
  | DocsResponseEvent
  | JsonResponseEvent
  | LifecycleEvent
  | UnclassifiedEvent
  | SessionResetEvent;

/** The `kind` string literal union — all possible discriminator values. */
export type ProtocolEventKind = ProtocolEvent['kind'];
