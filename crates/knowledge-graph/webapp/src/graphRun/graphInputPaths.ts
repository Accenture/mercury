import type { MinigraphGraphData } from '../utils/graphTypes';

const DESCRIPTIVE_PROPERTY_KEYS = new Set(['description', 'question', 'purpose']);
const BODY_PREFIX = 'input.body';

function isWordChar(char: string): boolean {
  return /[A-Za-z0-9_.]/.test(char);
}

function isTokenChar(char: string): boolean {
  return /[A-Za-z0-9_.*\[\]-]/.test(char);
}

function collectPathsFromText(text: string, paths: Set<string>): void {
  let searchFrom = 0;
  while (searchFrom < text.length) {
    const begin = text.indexOf(BODY_PREFIX, searchFrom);
    if (begin === -1) return;

    const before = begin > 0 ? text[begin - 1] : '';
    const beforeBefore = begin > 1 ? text[begin - 2] : '';
    const afterPrefix = text[begin + BODY_PREFIX.length] ?? '';
    if (
      (before && isWordChar(before) && !(before === '.' && beforeBefore === '$')) ||
      (afterPrefix && /[A-Za-z0-9_]/.test(afterPrefix))
    ) {
      searchFrom = begin + BODY_PREFIX.length;
      continue;
    }

    let end = begin + BODY_PREFIX.length;
    while (end < text.length && isTokenChar(text[end])) end += 1;

    let token = text.slice(begin, end);
    while (token.endsWith('.') || token.endsWith('-') || token.endsWith('[')) {
      token = token.slice(0, -1);
    }
    while (token.endsWith(']') && !token.includes('[')) {
      token = token.slice(0, -1);
    }
    paths.add(token);
    searchFrom = Math.max(end, begin + BODY_PREFIX.length);
  }
}

function visitPropertyValue(value: unknown, paths: Set<string>, propertyKey?: string): void {
  if (propertyKey && DESCRIPTIVE_PROPERTY_KEYS.has(propertyKey.toLowerCase())) return;
  if (typeof value === 'string') {
    collectPathsFromText(value, paths);
    return;
  }
  if (Array.isArray(value)) {
    for (const entry of value) visitPropertyValue(entry, paths);
    return;
  }
  if (typeof value === 'object' && value !== null) {
    for (const [key, entry] of Object.entries(value)) {
      visitPropertyValue(entry, paths, key);
    }
  }
}

/**
 * Derive referenced input.body paths from current node properties.
 * These are UI hints only: MiniGraph does not expose requiredness or input types.
 */
export function collectGraphInputBodyPaths(graphData: MinigraphGraphData | null): string[] {
  if (!graphData) return [];
  const paths = new Set<string>();
  for (const node of graphData.nodes) {
    visitPropertyValue(node.properties, paths);
  }
  return Array.from(paths).sort();
}
