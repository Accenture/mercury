/** Discriminated string union of all known message type values. */
export type MessageType = 'info' | 'error' | 'ping' | 'welcome' | 'raw';

/** Parsed structure of a single console message. */
export interface ParsedMessage {
  type:    MessageType;
  message: string;
  time:    string | null;
  raw:     string;
}

/** Result of attempting to parse a string as a JSON object/array. */
export interface JSONParseResult {
  isJSON: boolean;
  data:   object | null;
}

export const parseMessage = (msg: string): ParsedMessage => {
  try {
    const parsed = JSON.parse(msg);
    return {
      type: parsed.type || 'info',
      message: parsed.message || msg,
      time: parsed.time,
      raw: msg
    };
  } catch {
    return {
      type: 'raw',
      message: msg,
      time: null,
      raw: msg
    };
  }
};

export const getMessageIcon = (type: MessageType): string => {
  const icons: Record<MessageType, string> = {
    info:    'ℹ️',
    error:   '❌',
    ping:    '🔄',
    welcome: '👋',
    raw:     '',
  };
  return icons[type] ?? '•';
};

export const tryParseJSON = (str: string): JSONParseResult => {
  try {
    const parsed = JSON.parse(str);
    // Only consider it JSON if it's an object or array, not primitives
    if (typeof parsed === 'object' && parsed !== null) {
      return { isJSON: true, data: parsed };
    }
  } catch {
    // Not valid JSON
  }
  return { isJSON: false, data: null };
};

/**
 * Result type for a successfully matched graph export response.
 */
export interface GraphExportSuccess {
  /** The graph name segment from the API path, e.g. "my-graph" from "/api/graph/model/my-graph/1". */
  graphName: string;
  /** The full API path, e.g. "/api/graph/model/my-graph/1". */
  apiPath: string;
}

/**
 * Returns non-null only when `raw` contains the literal substring
 * `"Graph exported to "`.  Pure describe-graph responses (which also
 * contain `/api/graph/model/…`) do not contain that prefix and return
 * `null`, so Rule 6a never fires for background refresh traffic.
 */
export function extractGraphExportSuccess(raw: string): GraphExportSuccess | null {
  if (!raw.includes('Graph exported to ')) return null;
  const apiPath = extractGraphApiPath(raw);
  if (!apiPath) return null;
  const graphName = apiPath.split('/')[4]; // ["", "api", "graph", "model", "<name>", ...]
  if (!graphName) return null;
  return { graphName, apiPath };
}

/**
 * Detects known export-failure messages from the server.
 * Returns the failure reason or `null` if the message is not an export failure.
 */
export function detectGraphExportFailure(
  raw: string,
): { reason: 'invalid-name' | 'root-name-conflict' } | null {
  if (raw.includes('Invalid filename')) return { reason: 'invalid-name' };
  if (raw.includes('Expect root node name')) return { reason: 'root-name-conflict' };
  return null;
}

/**
 * Returns true when a raw WebSocket message string is a plain-text
 * (non-JSON) candidate for Markdown rendering.
 *
 * A message is NOT a Markdown candidate if:
 *  - it is a valid JSON object or array (handled by JsonView)
 *  - it is a JSON-encoded lifecycle event ({ type, message, time })
 *    i.e. tryParseJSON succeeds AND the parsed object has a "type" field
 *
 * Everything else — including multi-line text, Markdown syntax, XML snippets
 * that are not valid JSON — is considered a Markdown candidate.
 *
 * Edge case — bare JSON primitives (e.g. "hello", 42, true):
 *   tryParseJSON returns isJSON: false for primitives, so they fall through
 *   to `return true` and are treated as Markdown candidates. This is accepted
 *   as correct behaviour: the backend never sends bare primitives, and
 *   rendering the raw string as Markdown is a safe fallback if it ever does.
 *   Do NOT change tryParseJSON to accept primitives — that would break JsonView.
 */
export function isMarkdownCandidate(raw: string): boolean {
  const result = tryParseJSON(raw);
  if (!result.isJSON) return true;                             // not JSON at all → candidate
  const obj = result.data as Record<string, unknown>;
  if (typeof obj['type'] === 'string') return false;           // lifecycle event ({ type, message, time }) → not candidate
  return false;                                                // any other JSON object/array → not candidate
}

/**
 * Extracts the first `/api/graph/model/<id>` path found in a raw message
 * string. Returns `null` if no such path is present.
 *
 * Matches both forms the server emits:
 *   "Graph exported to '/tmp/…'\nDescribed in /api/graph/model/my-graph"
 *   "Graph described in /api/graph/model/ws-122189-1"
 */
export function extractGraphApiPath(raw: string): string | null {
  const match = raw.match(/\/api\/graph\/model\/([^\s'"]+)/);
  return match ? match[0] : null;
}

/**
 * Returns true when a raw message contains a graph model API path that the
 * user can pin to load the graph visualiser.
 */
export function isGraphLinkMessage(raw: string): boolean {
  // Must be a plain-text message (not a JSON lifecycle event)
  if (!isMarkdownCandidate(raw)) return false;
  return extractGraphApiPath(raw) !== null;
}

/**
 * Extracts the `/api/json/content/{id}` path from the server's upload-ready
 * message: "Please upload XML/JSON text to /api/json/content/{id}"
 *
 * Returns the path string (e.g. "/api/json/content/{id}") or null
 * if the message does not contain such a path.
 */
export function extractUploadPath(raw: string): string | null {
  const match = raw.match(/\/api\/json\/content\/([\w-]+)/);
  return match ? match[0] : null;
}

/**
 * Represents a detected large-payload link with metadata extracted from the
 * server's "Large payload (N) -> GET /api/inspect/..." message.
 */
export interface LargePayloadLink {
  /** The relative API path to GET the payload, e.g. "/api/inspect/ws-734563-3/input.body" */
  apiPath:   string;
  /** The size in bytes reported by the server, e.g. 254922 */
  byteSize:  number;
  /**
   * A suggested filename for the download, derived from the last path segment,
   * e.g. "input.body.json"
   */
  filename:  string;
}

/**
 * Matches the server's large-payload message pattern:
 *   "Large payload (254922) -> GET /api/inspect/ws-734563-3/input.body"
 *
 * Returns a {@link LargePayloadLink} when the pattern is found, or null otherwise.
 *
 * The message is always a plain-text (non-JSON) line so there is no need to
 * guard with isMarkdownCandidate — the caller should already know the context.
 */
export function extractLargePayloadLink(raw: string): LargePayloadLink | null {
  // Pattern: Large payload (<bytes>) -> GET <path>
  const match = raw.match(/Large payload \((\d+)\)\s*->\s*GET\s+(\/api\/inspect\/[^\s]+)/i);
  if (!match) return null;

  const byteSize = parseInt(match[1], 10);
  const apiPath  = match[2];

  // Derive a safe filename from the last path segment plus ".json"
  const lastSegment = apiPath.split('/').filter(Boolean).pop() ?? 'payload';
  const filename    = `${lastSegment}.json`;

  return { apiPath, byteSize, filename };
}

/**
 * Returns true when a raw WebSocket message is a large-payload link.
 * Used by ConsoleMessage.tsx to apply a distinct visual treatment.
 */
export function isLargePayloadMessage(raw: string): boolean {
  return extractLargePayloadLink(raw) !== null;
}

/**
 * Extracts the POST path from a mock-data upload invitation:
 *   "You may upload JSON payload -> POST /api/mock/{id}"
 *
 * Returns the path string (e.g. "/api/mock/ws-417669-24") or null if
 * the message does not match the pattern.
 *
 * The regex is anchored to `/api/mock/` (not a generic POST capture) to
 * avoid false positives from future unrelated messages. The case-insensitive
 * flag handles any minor casing variation the server might introduce.
 */
export function extractMockUploadPath(raw: string): string | null {
  const match = raw.match(/You may upload .*?->\s*POST\s+(\/api\/mock\/[\w-]+)/i);
  return match ? match[1] : null;
}

/**
 * Returns true when a raw WebSocket message is a mock-data upload invitation.
 * Used by ConsoleMessage.tsx and useAutoMockUpload to detect the upload trigger.
 */
export function isMockUploadMessage(raw: string): boolean {
  return extractMockUploadPath(raw) !== null;
}

/**
 * Returns true when a raw WebSocket message is an echoed `help` or `describe`
 * command.  Used by the classifier (Rule 9) to emit `command.helpOrDescribe`
 * events on the ProtocolBus.
 *
 * Matches:
 *  - `> help`               — top-level help overview
 *  - `> help <topic>`       — topic-specific help (e.g. `help create`)
 *  - `> describe skill <r>` — skill documentation (returns plain-text markdown)
 *  - `> describe node <n>`  — node detail
 *  - `> describe connection <a> and <b>` — connection detail (plain-text)
 *
 * Explicitly EXCLUDED (returns a graph-link, not a text response):
 *  - `> describe graph`     — this produces a graph-link message that belongs
 *                             to the Graph tab.
 *
 * Note: the backend echoes every command with a `> ` prefix, so we match on
 * that prefix to distinguish user commands from server responses.
 */
export function isHelpOrDescribeCommand(raw: string): boolean {
  if (!raw.startsWith('> ')) return false;
  const lower = raw.slice(2).trim().toLowerCase();

  // Top-level help or `help <topic>` — matches any `help ...` variant
  if (lower === 'help' || lower.startsWith('help ')) return true;

  // `describe skill`, `describe node`, `describe connection` — but NOT `describe graph`
  if (lower.startsWith('describe ')) {
    const rest = lower.slice('describe '.length).trim();
    if (rest.startsWith('graph')) return false;   // `describe graph` → graph tab
    return true;                                  // everything else → text response
  }

  return false;
}

/**
 * Extracts the graph name from an echoed `import graph from {name}` command.
 *
 * The backend echoes every user command with a `> ` prefix, so the message
 * that arrives in the WebSocket stream looks like:
 *   "> import graph from my-graph-name"
 *
 * Returns the trimmed name string (e.g. `"my-graph-name"`) or `null` if the
 * message is not an import-graph echo.
 */
export function extractImportGraphName(raw: string): string | null {
  if (!raw.startsWith('> ')) return null;
  const lower = raw.slice(2).trimStart().toLowerCase();
  if (!lower.startsWith('import graph from ')) return null;
  const name = raw.slice(2).trimStart().slice('import graph from '.length).trim();
  return name.length > 0 ? name : null;
}

/**
 * Discriminated union of the two mutation kinds the auto-refresh hook handles.
 *  - 'node-mutation'  — any structural node or connection change
 *  - 'import-graph'   — a full graph model replace via `import graph from {name}`
 */
export type MutationKind = 'node-mutation' | 'import-graph';

export type CreateNodeTextResultStatus = 'accepted' | 'rejected' | 'error';
export type NodeActionTextResultStatus = 'accepted' | 'rejected' | 'error';
export type NodeActionTextResultAction = 'create-node' | 'edit-node' | 'delete-node' | null;

export interface CreateNodeTextResult {
  status: CreateNodeTextResultStatus;
  alias: string | null;
  message: string;
}

export interface NodeActionTextResult {
  status: NodeActionTextResultStatus;
  action: NodeActionTextResultAction;
  alias: string | null;
  message: string;
}

export const NODE_CREATED_RE = /^node ([A-Za-z0-9_-]+) created$/i;
export const NODE_ALREADY_EXISTS_RE = /^node ([A-Za-z0-9_-]+) already exists$/i;
export const NODE_UPDATED_RE = /^node ([A-Za-z0-9_-]+) updated$/i;
export const NODE_DELETED_RE = /^node ([A-Za-z0-9_-]+) deleted$/i;
export const NODE_NOT_FOUND_RE = /^node ([A-Za-z0-9_-]+) not found$/i;
export const ERROR_RE = /^ERROR: (.+)$/;

export function parseNodeActionTextResult(raw: string): NodeActionTextResult | null {
  const text = raw.trim();
  if (text.startsWith('> ')) return null;

  const created = text.match(NODE_CREATED_RE);
  if (created) {
    return { status: 'accepted', action: 'create-node', alias: created[1], message: text };
  }

  const alreadyExists = text.match(NODE_ALREADY_EXISTS_RE);
  if (alreadyExists) {
    return { status: 'rejected', action: 'create-node', alias: alreadyExists[1], message: text };
  }

  const updated = text.match(NODE_UPDATED_RE);
  if (updated) {
    return { status: 'accepted', action: 'edit-node', alias: updated[1], message: text };
  }

  const deleted = text.match(NODE_DELETED_RE);
  if (deleted) {
    return { status: 'accepted', action: 'delete-node', alias: deleted[1], message: text };
  }

  const notFound = text.match(NODE_NOT_FOUND_RE);
  if (notFound) {
    return { status: 'rejected', action: null, alias: notFound[1], message: text };
  }

  const error = text.match(ERROR_RE);
  if (error) {
    return { status: 'error', action: null, alias: null, message: text };
  }

  return null;
}

export function parseCreateNodeTextResult(raw: string): CreateNodeTextResult | null {
  const result = parseNodeActionTextResult(raw);
  if (!result) return null;
  if (result.action === 'create-node' || result.status === 'error') {
    return { status: result.status, alias: result.alias, message: result.message };
  }

  return null;
}

/**
 * Inspects a single raw WebSocket message string and returns whether it
 * represents a graph mutation that should trigger an auto-refresh.
 *
 * Returns:
 *  - `'node-mutation'`  when the message is a plain-text success response for
 *    a structural node or connection change (create / update / delete / connect /
 *    import-node / delete-connection).
 *  - `'import-graph'`   when the message is `"Graph model imported as draft"`.
 *  - `null`             for all other messages (read-only responses, JSON
 *    lifecycle events, echoed commands, graph-link messages, etc.).
 *
 * Matching rules:
 *  - Only non-JSON messages pass (`isMarkdownCandidate` guard).
 *  - Echoed commands (`raw.startsWith('> ')`) are excluded.
 *  - Graph-link messages (`isGraphLinkMessage`) are excluded.
 *  - Node-operation matches require `lower.startsWith('node ')` (mirrors the
 *    server's `NODE_NAME = "Node "` prefix) to avoid false positives from
 *    `"Graph instance created…"` and `"Root node created because…"`.
 *  - Connection-delete match requires both `" -> "` and `"removed"`.
 */
export function detectMutation(raw: string): MutationKind | null {
  if (!isMarkdownCandidate(raw)) return null;   // JSON messages → not a mutation
  if (raw.startsWith('> ')) return null;         // echoed user command → ignore
  if (isGraphLinkMessage(raw)) return null;      // graph-link messages handled by pin flow

  const lower = raw.toLowerCase();

  // ── import-graph ─────────────────────────────────────────────────────────
  if (lower.includes('graph model imported as draft')) return 'import-graph';

  // ── node-mutation ─────────────────────────────────────────────────────────
  // Connection delete: "{A} -> {B} removed" (one or two lines)
  if (lower.includes(' -> ') && lower.includes('removed')) return 'node-mutation';

  // Node structural ops — require "Node " prefix to avoid false positives
  const startsWithNode = lower.startsWith('node ');
  if (startsWithNode) {
    if (lower.includes(' created'))            return 'node-mutation';
    if (lower.includes(' updated'))            return 'node-mutation';
    if (lower.includes(' deleted'))            return 'node-mutation';
    if (lower.includes(' connected to '))      return 'node-mutation';
    if (lower.includes(' imported from '))     return 'node-mutation';
    if (lower.includes(' overwritten by node from ')) return 'node-mutation';
  }

  return null;
}
