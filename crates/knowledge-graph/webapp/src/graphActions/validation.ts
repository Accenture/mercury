import { MAX_BUFFER } from '../config/playgrounds';
import type { MinigraphGraphData } from '../utils/graphTypes';
import type { NodeFormState, NodeFormValidationErrors } from './nodeAuthoringTypes';

// Keep this token rule aligned with GraphProperties.validateName on the backend.
// The backend remains authoritative; this only blocks obviously invalid form
// input before it can become raw command text.
export const NODE_NAME_RE = /^[A-Za-z0-9_-]+$/;
const PROPERTY_PATH_SEGMENT_RE = /^[A-Za-z0-9_-]+(?:\[(?:0|[1-9]\d*)\])*$/;

// Mirrors MiniGraph's reserved alias list so the modal can show immediate field
// feedback instead of relying on a later generic backend ERROR response.
export const RESERVED_ALIASES = new Set([
  'input',
  'output',
  'model',
  'response',
  'result',
  'parameter',
  'none',
  'next',
  'api',
  'error',
]);

export interface NodeFormValidationOptions {
  graphData?: MinigraphGraphData | null;
  mode?: 'create' | 'edit';
  originalAlias?: string | null;
}

export interface NodeFormValidationResult {
  valid: boolean;
  errors: NodeFormValidationErrors;
}

export function getValidationErrorKeyForProperty(rowId: string, field: 'key' | 'value'): string {
  return `properties.${rowId}.${field}`;
}

export function isValidPropertyPath(path: string): boolean {
  return path.split('.').every((segment) => PROPERTY_PATH_SEGMENT_RE.test(segment));
}

function isValidPropertyKey(key: string, mode: 'create' | 'edit'): boolean {
  return mode === 'edit' ? isValidPropertyPath(key) : NODE_NAME_RE.test(key);
}

// Validate the supported authoring surface: alias, optional node type, and
// command-safe property rows. Duplicate aliases from graphData are advisory
// because graphData is a staleable frontend projection.
export function validateNodeFormState(
  formState: NodeFormState,
  options: NodeFormValidationOptions = {},
): NodeFormValidationResult {
  const errors: NodeFormValidationErrors = {};
  const mode = options.mode ?? 'create';
  const alias = formState.alias.trim();
  const originalAlias = options.originalAlias?.trim() ?? '';
  const nodeType = formState.nodeType.trim();

  if (mode === 'edit') {
    if (!originalAlias) {
      errors.alias = 'Original alias is required.';
    } else if (!NODE_NAME_RE.test(originalAlias)) {
      errors.alias = 'Use only letters, numbers, underscore, and hyphen.';
    }
  } else {
    if (!alias) {
      errors.alias = 'Alias is required.';
    } else if (!NODE_NAME_RE.test(alias)) {
      errors.alias = 'Use only letters, numbers, underscore, and hyphen.';
    } else if (RESERVED_ALIASES.has(alias.toLowerCase())) {
      errors.alias = `"${alias}" is reserved.`;
    } else if (options.graphData?.nodes.some((node) => node.alias.toLowerCase() === alias.toLowerCase())) {
      errors.alias = `Node "${alias}" already exists in the current graph.`;
    }
  }

  if (nodeType && !NODE_NAME_RE.test(nodeType)) {
    errors.nodeType = 'Use only letters, numbers, underscore, and hyphen.';
  }

  for (const row of formState.properties) {
    const key = row.key.trim();
    const value = row.value.trim();
    if (!key && !value) continue;

    if (!key && value) {
      errors[getValidationErrorKeyForProperty(row.id, 'key')] = 'Property key is required when value is present.';
    } else if (!isValidPropertyKey(key, mode)) {
      errors[getValidationErrorKeyForProperty(row.id, 'key')] = mode === 'edit'
        ? 'Use a property name or dot/bracket path, for example mapping[0] or config.value.'
        : 'Use only letters, numbers, underscore, and hyphen.';
    }

    if (mode === 'create' && (value.includes('\r') || value.includes('\n'))) {
      errors[getValidationErrorKeyForProperty(row.id, 'value')] = 'Property value must be a single line.';
    } else if (value.includes("'''")) {
      errors[getValidationErrorKeyForProperty(row.id, 'value')] = "Property value cannot contain '''.";
    }
  }

  return { valid: Object.keys(errors).length === 0, errors };
}

export interface DeleteNodeValidationOptions {
  graphData?: MinigraphGraphData | null;
}

export function validateDeleteNodeAlias(
  aliasInput: string,
  options: DeleteNodeValidationOptions = {},
): NodeFormValidationResult {
  const errors: NodeFormValidationErrors = {};
  const alias = aliasInput.trim();

  if (!alias) {
    errors.alias = 'Alias is required.';
  } else if (!NODE_NAME_RE.test(alias)) {
    errors.alias = 'Use only letters, numbers, underscore, and hyphen.';
  } else if (options.graphData && !options.graphData.nodes.some((node) => node.alias.toLowerCase() === alias.toLowerCase())) {
    errors.alias = `Node "${alias}" is no longer available in the current graph.`;
  }

  return { valid: Object.keys(errors).length === 0, errors };
}

// The command budget belongs to the WebSocket command path, so size is checked
// after serialization rather than estimating from individual fields.
export function validateCommandSize(commandText: string): NodeFormValidationResult {
  if (commandText.length <= MAX_BUFFER) return { valid: true, errors: {} };
  return {
    valid: false,
    errors: {
      command: 'The node command is too large. Shorten property values before submitting.',
    },
  };
}
