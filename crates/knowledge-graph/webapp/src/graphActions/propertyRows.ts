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

  return {
    valid: true,
    formState: {
      alias: node.alias,
      nodeType: node.types[0] ?? '',
      properties: properties.length > 0 ? properties : [createPropertyRow()],
      source: 'edit-node',
    },
    message: null,
  };
}
