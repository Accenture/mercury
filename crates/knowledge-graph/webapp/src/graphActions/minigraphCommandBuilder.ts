import type { NodeFormState } from './nodeAuthoringTypes';
import { validateCommandSize, validateDeleteNodeAlias, validateNodeFormState, type DeleteNodeValidationOptions } from './validation';

function getSerializablePropertyRows(formState: NodeFormState, preserveValue = false) {
  return formState.properties
    .map((row) => ({
      key: row.key.trim(),
      value: preserveValue ? row.value.replace(/\r\n/g, '\n').replace(/\r/g, '\n') : row.value.trim(),
    }))
    .filter((row) => row.key || row.value.trim());
}

function assertValidCommandSize(command: string): void {
  const sizeValidation = validateCommandSize(command);
  if (!sizeValidation.valid) {
    throw new Error(sizeValidation.errors.command);
  }
}

function appendSerializedProperty(lines: string[], key: string, value: string): void {
  if (value.includes('\n')) {
    lines.push(`${key}='''`);
    lines.push(value);
    lines.push("'''");
    return;
  }

  lines.push(`${key}=${value}`);
}

// Single serialization boundary for node UI authoring. Callers pass typed form
// state; only this builder is allowed to produce backend raw command text so
// validation and injection guards stay centralized.
export function buildCreateNodeCommand(formState: NodeFormState): string {
  const validation = validateNodeFormState(formState);
  if (!validation.valid) {
    throw new Error(Object.values(validation.errors)[0] ?? 'Invalid node form state.');
  }

  const alias = formState.alias.trim();
  const nodeType = formState.nodeType.trim();
  const propertyRows = getSerializablePropertyRows(formState);

  // Match the existing multiline command grammar consumed by
  // GraphCommandService.handleMultiLineCommand.
  const lines = [`create node ${alias}`];
  if (nodeType) {
    lines.push(`with type ${nodeType}`);
  }
  if (propertyRows.length > 0) {
    lines.push('with properties');
    for (const row of propertyRows) {
      appendSerializedProperty(lines, row.key, row.value);
    }
  }

  const command = lines.join('\n');
  assertValidCommandSize(command);

  return command;
}

export function buildUpdateNodeCommand(formState: NodeFormState, originalAliasInput: string): string {
  const originalAlias = originalAliasInput.trim();
  const validation = validateNodeFormState(formState, { mode: 'edit', originalAlias });
  if (!validation.valid) {
    throw new Error(Object.values(validation.errors)[0] ?? 'Invalid node form state.');
  }

  const nodeType = formState.nodeType.trim();
  const propertyRows = getSerializablePropertyRows(formState, true);

  const lines = [`update node ${originalAlias}`];
  if (nodeType) {
    lines.push(`with type ${nodeType}`);
  }
  if (propertyRows.length > 0) {
    lines.push('with properties');
    for (const row of propertyRows) {
      appendSerializedProperty(lines, row.key, row.value);
    }
  }

  const command = lines.join('\n');
  assertValidCommandSize(command);
  return command;
}

export function buildDeleteNodeCommand(aliasInput: string, options: DeleteNodeValidationOptions = {}): string {
  const alias = aliasInput.trim();
  const validation = validateDeleteNodeAlias(alias, options);
  if (!validation.valid) {
    throw new Error(Object.values(validation.errors)[0] ?? 'Invalid node alias.');
  }

  const command = `delete node ${alias}`;
  assertValidCommandSize(command);
  return command;
}
