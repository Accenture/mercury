import type { MinigraphNode } from '../utils/graphTypes';

/**
 * Serialise a property value for the "key=value" line in a create/update command.
 * Multiline values are wrapped in triple-quotes (''').
 */
function serializePropertyValue(value: unknown): string {
  if (value === null || value === undefined) return '';

  const str = typeof value === 'string' ? value : JSON.stringify(value);

  if (str.includes("'''")) {
    console.warn(
      `[commandBuilder] Property value contains "'''" which cannot be escaped ` +
      `in the backend grammar. The value may be truncated on paste.`
    );
  }

  if (str.includes('\n')) {
    return `'''\n${str}\n'''`;
  }

  return str;
}

/**
 * Build a create or update node multi-line command string from a MinigraphNode.
 */
export function buildNodeCommand(
  verb: 'create' | 'update',
  node: MinigraphNode,
): string {
  const lines: string[] = [`${verb} node ${node.alias}`];

  if (node.types.length > 0) {
    lines.push(`with type ${node.types[0]}`);
  }

  const propEntries = Object.entries(node.properties).filter(
    ([, v]) => v !== undefined && v !== null,
  );

  if (propEntries.length > 0) {
    lines.push('with properties');
    for (const [key, value] of propEntries) {
      if (Array.isArray(value)) {
        for (const element of value) {
          lines.push(`${key}[]=${serializePropertyValue(element)}`);
        }
      } else {
        lines.push(`${key}[]=${serializePropertyValue(value)}`);
      }
    }
  }

  return lines.join('\n');
}

/**
 * Build a `connect` command for a single relation.
 */
export function buildConnectCommand(
  source: string,
  target: string,
  relationType: string,
): string {
  return `connect ${source} to ${target} with ${relationType}`;
}
