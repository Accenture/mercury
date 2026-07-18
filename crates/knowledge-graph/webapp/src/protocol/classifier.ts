import type { ProtocolEvent } from './events';
import {
  tryParseJSON,
  extractLargePayloadLink,
  extractMockUploadPath,
  extractUploadPath,
  isGraphLinkMessage,
  extractGraphApiPath,
  detectMutation,
  isHelpOrDescribeCommand,
  extractImportGraphName,
  isMarkdownCandidate,
  extractGraphExportSuccess,
  detectGraphExportFailure,
  parseNodeActionTextResult,
} from '../utils/messageParser';

export const SESSION_RESTARTED_MSG = 'Session restarted';

const KNOWN_LIFECYCLE_TYPES = new Set(['info', 'error', 'ping', 'welcome']);

/**
 * Classifies a single raw WebSocket message string into one or more typed
 * protocol events.
 *
 * This function is the single source of truth for message classification in
 * the entire webapp.  All regex/JSON-parse heuristics are centralised here.
 *
 * Pure — no side effects, no state, no DOM access.  Safe to call from a
 * Web Worker in a future phase.
 */
export function classifyMessage(msgId: number, raw: string): ProtocolEvent[] {
  const events: ProtocolEvent[] = [];
  const base = { msgId, raw };

  // Track which rules matched for rule 11 exclusion
  let matchedEcho = false;
  let matchedGraphLink = false;
  let matchedMockUpload = false;
  let matchedUploadContentPath = false;
  let matchedLargePayload = false;

  // ── Rule 1: JSON lifecycle event ──────────────────────────────────────────
  const jsonCheck = tryParseJSON(raw);
  if (jsonCheck.isJSON) {
    const parsed = jsonCheck.data as Record<string, unknown>;
    if (typeof parsed['type'] === 'string') {
      const typeStr = parsed['type'] as string;
      events.push({
        ...base,
        kind: 'lifecycle',
        type: typeStr,
        knownType: KNOWN_LIFECYCLE_TYPES.has(typeStr),
        message: typeof parsed['message'] === 'string' ? parsed['message'] : raw,
        time: (parsed['time'] as string) ?? null,
      });
      // Lifecycle JSON does not produce json.response — return early
      return events.length > 0 ? events : [{ ...base, kind: 'unclassified' }];
    }

    // ── Rule 2: JSON object/array (non-lifecycle) ─────────────────────────
    events.push({
      ...base,
      kind: 'json.response',
      data: jsonCheck.data!,
    });
    // JSON messages don't match any text rules — return early
    return events.length > 0 ? events : [{ ...base, kind: 'unclassified' }];
  }

  // ── Text message rules (non-JSON) ─────────────────────────────────────────

  // ── Rule 3: Large payload link ────────────────────────────────────────────
  const largePayload = extractLargePayloadLink(raw);
  if (largePayload) {
    matchedLargePayload = true;
    events.push({
      ...base,
      kind: 'payload.large',
      apiPath: largePayload.apiPath,
      byteSize: largePayload.byteSize,
      filename: largePayload.filename,
    });
  }

  // ── Rule 4: Mock upload invitation ────────────────────────────────────────
  const mockUploadPath = extractMockUploadPath(raw);
  if (mockUploadPath) {
    matchedMockUpload = true;
    events.push({
      ...base,
      kind: 'upload.invitation',
      uploadPath: mockUploadPath,
    });
  }

  // ── Rule 5: Upload content path ───────────────────────────────────────────
  const uploadPath = extractUploadPath(raw);
  if (uploadPath) {
    matchedUploadContentPath = true;
    events.push({
      ...base,
      kind: 'upload.contentPath',
      uploadPath,
    });
  }

  // ── Rule 6: Graph link ────────────────────────────────────────────────────
  if (isGraphLinkMessage(raw)) {
    matchedGraphLink = true;
    const apiPath = extractGraphApiPath(raw);
    if (apiPath) {
      events.push({
        ...base,
        kind: 'graph.link',
        apiPath,
      });
    }
  }

  // ── Rule 6a: Graph export success ─────────────────────────────────────────────
  // Only fires when Rule 6 has already fired (matchedGraphLink === true).
  // Provides the save-correlation event; Rule 6's graph.link coexists.
  if (matchedGraphLink) {
    const exportResult = extractGraphExportSuccess(raw);
    if (exportResult) {
      events.push({
        ...base,
        kind: 'graph.exported',
        graphName: exportResult.graphName,
        apiPath: exportResult.apiPath,
      });
    }
  }

  // ── Rule 7: Graph mutation ────────────────────────────────────────────────
  const mutation = detectMutation(raw);
  if (mutation) {
    events.push({
      ...base,
      kind: 'graph.mutation',
      mutationType: mutation,
    });
  }

  // ── Rule 7a: Minigraph node-action text result ────────────────────────────
  const nodeActionResult = parseNodeActionTextResult(raw);
  if (nodeActionResult) {
    events.push({
      ...base,
      kind: 'minigraph.nodeAction.textResult',
      status: nodeActionResult.status,
      action: nodeActionResult.action,
      alias: nodeActionResult.alias,
      message: nodeActionResult.message,
    });
  }

  // ── Rule 7b: Backward-compatible create-node text result ─────────────────
  if (
    nodeActionResult &&
    (nodeActionResult.action === 'create-node' || nodeActionResult.status === 'error')
  ) {
    events.push({
      ...base,
      kind: 'minigraph.createNode.textResult',
      status: nodeActionResult.status,
      alias: nodeActionResult.alias,
      message: nodeActionResult.message,
    });
  }

  // ── Rule 7c: Session reset ────────────────────────────────────────────────
  if (raw === SESSION_RESTARTED_MSG) {
    events.push({ ...base, kind: 'session.reset' });
  }

  // ── Rule 8: Command echo (starts with "> ") ──────────────────────────────
  if (raw.startsWith('> ')) {
    matchedEcho = true;
    events.push({
      ...base,
      kind: 'command.echo',
      commandText: raw.slice(2),
    });
  }

  // ── Rule 9: Help or describe command ──────────────────────────────────────
  if (isHelpOrDescribeCommand(raw)) {
    events.push({
      ...base,
      kind: 'command.helpOrDescribe',
      commandText: raw.slice(2),
    });
  }

  // ── Rule 10: Import graph command ─────────────────────────────────────────
  const graphName = extractImportGraphName(raw);
  if (graphName) {
    events.push({
      ...base,
      kind: 'command.importGraph',
      graphName,
    });
  }

  // ── Rule 11a: Graph export failure ────────────────────────────────────────
  // Only for plain-text messages (not JSON, no graph-link) that match a
  // known export-failure pattern.  Rule 11 (docs.response) still fires for
  // the same message — both events coexist.
  const exportFailure = detectGraphExportFailure(raw);
  if (exportFailure) {
    events.push({ ...base, kind: 'graph.export.failed', reason: exportFailure.reason });
  }

  // ── Rule 11: Markdown / docs response ─────────────────────────────────────
  // Only for non-echo, non-graph-link, non-mock-upload, non-upload-content-path,
  // non-large-payload messages that are markdown candidates.
  // The isMarkdownCandidate check is technically always true here (we already
  // handled JSON above), but kept as a safety guard.
  if (
    !matchedEcho &&
    !matchedGraphLink &&
    !matchedMockUpload &&
    !matchedUploadContentPath &&
    !matchedLargePayload &&
    isMarkdownCandidate(raw)
  ) {
    events.push({
      ...base,
      kind: 'docs.response',
      isMarkdown: true,
    });
  }

  // ── Rule 12: Fallback ────────────────────────────────────────────────────
  if (events.length === 0) {
    events.push({ ...base, kind: 'unclassified' });
  }

  return events;
}
