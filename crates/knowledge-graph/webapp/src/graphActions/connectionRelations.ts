export const CONNECTION_RELATION_OPTIONS = [
  'fetch',
  'details',
  'ext-call',
  'mapping',
  'compute',
  'calculate',
  'evaluate',
  'fork',
  'join',
  'one',
  'two',
  'three',
  'more',
  'done',
  'complete',
  'finish',
  'positive',
  'negative',
] as const;

export type ConnectionRelation = (typeof CONNECTION_RELATION_OPTIONS)[number];

export const CONNECTION_RELATION_COLORS: Record<ConnectionRelation, string> = {
  fetch:      '#0369a1',
  details:    '#0369a1',
  'ext-call': '#0369a1',
  mapping:    '#b45309',
  compute:    '#b45309',
  calculate:  '#b45309',
  evaluate:   '#b45309',
  fork:       '#7e22ce',
  join:       '#7e22ce',
  one:        '#7e22ce',
  two:        '#6d28d9',
  three:      '#5b21b6',
  more:       '#4c1d95',
  done:       '#15803d',
  complete:   '#15803d',
  finish:     '#15803d',
  positive:   '#15803d',
  negative:   '#b91c1c',
};

const EDGE_FALLBACK_COLORS = [
  '#0369a1',
  '#15803d',
  '#b45309',
  '#7e22ce',
  '#b91c1c',
  '#0f766e',
  '#c2410c',
  '#a16207',
] as const;

function hashString(value: string): number {
  let hash = 0;
  for (let i = 0; i < value.length; i++) {
    hash = ((hash << 5) - hash) + value.charCodeAt(i);
    hash |= 0;
  }
  return Math.abs(hash);
}

export function getConnectionRelationColor(
  relationTypes: readonly string[],
  emptyColor: string,
): string {
  if (relationTypes.length === 0) return emptyColor;

  const primary = relationTypes[0].trim().toLowerCase();
  const known = CONNECTION_RELATION_COLORS[primary as ConnectionRelation];
  if (known) return known;

  return EDGE_FALLBACK_COLORS[hashString(primary) % EDGE_FALLBACK_COLORS.length];
}
