import type { MinigraphNode } from '../utils/graphTypes';
import { isValidPropertyPath, NODE_NAME_RE } from './validation';
import type { NodeFormState, NodeFormConversionResult, NodeFormSource, PropertyRow } from './nodeAuthoringTypes';

let rowCounter = 0;

// Row ids are only for React rendering and field-error keys. They are never
// sent to the backend.
export function createPropertyRow(key = '', value = ''): PropertyRow {
  rowCounter += 1;
  return { id: `property-row-${rowCounter}`, key, value };
}

// First-node authoring uses deterministic defaults. They are starting values
// only; normal validation still runs after the user edits or submits.
export function createDefaultNodeFormState(source: NodeFormSource): NodeFormState {
  return {
    alias: source === 'empty-graph' ? 'root' : '',
    nodeType: source === 'empty-graph' ? 'Root' : '',
    properties: [createPropertyRow()],
    source,
  };
}

// ── Engine-parity key presentation ───────────────────────────────────────────
// The backend's own `edit node` output (GraphCommandService.getRawProperties)
// sorts flattened keys with zero-filled single-bracket indices ([1] → [001] so
// [2] sorts before [10]) and prints them with the index removed — the []
// append signature.  MultiLevelMap.setElement("...[]...") appends to the list
// in line order and `update node` clears the property set before re-adding,
// so re-submitting rows in display order reconstructs arrays exactly.
// Keys with more than one bracket segment stay absolute, mirroring
// GraphCommandService.normalizeKey.

function findSingleBracketSegment(key: string): { open: number; close: number } | null {
  const open = key.indexOf('[');
  if (open === -1) return null;
  const close = key.indexOf(']', open + 1);
  if (close === -1) return null;
  if (key.indexOf('[', close + 1) !== -1) return null;
  return { open, close };
}

function toSortablePropertyKey(key: string): string {
  const segment = findSingleBracketSegment(key);
  if (!segment) return key;
  const index = key.slice(segment.open + 1, segment.close);
  if (!/^\d+$/.test(index)) return key;
  return key.slice(0, segment.open + 1) + index.padStart(3, '0') + key.slice(segment.close);
}

/** `input[0]` → `input[]` (single-bracket keys only; others stay absolute). */
export function toDisplayPropertyKey(key: string): string {
  const segment = findSingleBracketSegment(key);
  if (!segment) return key;
  const index = key.slice(segment.open + 1, segment.close);
  if (!/^\d+$/.test(index)) return key;
  return key.slice(0, segment.open + 1) + key.slice(segment.close);
}

/**
 * Stable ascending key sort matching the backend's `edit node` listing.
 * Rows sharing a key (the [] append signature) keep their relative order —
 * that order IS the array order on submit.  Rows without a key sink to the
 * end (fresh rows the user has not filled in yet).
 */
export function sortPropertyRowsByKey(rows: PropertyRow[]): PropertyRow[] {
  return rows.slice().sort((a, b) => {
    const aKey = toSortablePropertyKey(a.key.trim());
    const bKey = toSortablePropertyKey(b.key.trim());
    if (!aKey && !bKey) return 0;
    if (!aKey) return 1;
    if (!bKey) return -1;
    if (aKey < bKey) return -1;
    if (aKey > bKey) return 1;
    return 0;
  });
}

const UNSUPPORTED_EDIT_NODE_MESSAGE =
  'This node contains data that cannot be safely represented in the edit form. Use the console edit command for this node.';

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function toEditableValue(value: unknown): string {
  if (value === null) return 'null';
  return String(value);
}

function flattenPropertyValue(path: string, value: unknown, rows: PropertyRow[]): boolean {
  if (!isValidPropertyPath(path)) return false;

  if (Array.isArray(value)) {
    if (value.length === 0) return false;
    return value.every((item, index) => flattenPropertyValue(`${path}[${index}]`, item, rows));
  }

  if (isPlainObject(value)) {
    const entries = Object.entries(value);
    if (entries.length === 0) return false;
    return entries.every(([key, item]) => flattenPropertyValue(`${path}.${key}`, item, rows));
  }

  const stringValue = toEditableValue(value);
  if (stringValue.includes("'''")) return false;
  rows.push(createPropertyRow(path, stringValue));
  return true;
}

// Converts the rendered graph node into the flat path/value rows consumed by
// the backend update command. Arrays and nested objects use the same path
// grammar emitted by the console `edit node` command, e.g. mapping[0] or a.b.
export function createEditNodeFormState(node: MinigraphNode): NodeFormConversionResult {
  if (!NODE_NAME_RE.test(node.alias)) {
    return { valid: false, formState: null, message: UNSUPPORTED_EDIT_NODE_MESSAGE };
  }

  if (node.types.length > 1) {
    return { valid: false, formState: null, message: UNSUPPORTED_EDIT_NODE_MESSAGE };
  }

  const propertyEntries = Object.entries(node.properties);
  const properties: PropertyRow[] = [];
  for (const [key, value] of propertyEntries) {
    if (!flattenPropertyValue(key, value, properties)) {
      return { valid: false, formState: null, message: UNSUPPORTED_EDIT_NODE_MESSAGE };
    }
  }

  // Present rows the way the backend's own `edit node` does: sorted by key
  // (zero-fill compare keeps [2] before [10]) with array indices shown as the
  // [] append signature.
  const sortedDisplayRows = sortPropertyRowsByKey(properties)
    .map((row) => ({ ...row, key: toDisplayPropertyKey(row.key) }));

  return {
    valid: true,
    formState: {
      alias: node.alias,
      nodeType: node.types[0] ?? '',
      properties: sortedDisplayRows.length > 0 ? sortedDisplayRows : [createPropertyRow()],
      source: 'edit-node',
    },
    message: null,
  };
}
