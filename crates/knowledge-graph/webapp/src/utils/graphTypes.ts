/** Shape of a node as returned by the minigraph server. */
export interface MinigraphNodeProperties {
  skill?: string;
  description?: string;
  question?: string;
  mapping?: string[];
  [key: string]: unknown;
}

export interface MinigraphNode {
  /** Unique identifier / alias, e.g. "root", "fetchpersondata". */
  alias: string;
  /** Node type labels, e.g. ["entry_point"] */
  types: string[];
  properties: MinigraphNodeProperties;
}

/** A single directed relationship inside a connection object. */
export interface MinigraphRelation {
  type: string;
  properties: Record<string, unknown>;
}

/** One source → target connection block (may carry multiple relations). */
export interface MinigraphConnection {
  source: string;
  target: string;
  relations: MinigraphRelation[];
}

/** Top-level shape of the graph JSON returned by the server. */
export interface MinigraphGraphData {
  nodes: MinigraphNode[];
  connections: MinigraphConnection[];
}

/**
 * Returns true if the parsed object looks like a MinigraphGraphData payload.
 * We check for the presence of a "nodes" array only; connections may be absent
 * on an empty / partially-built graph.
 */
export function isMinigraphGraphData(value: unknown): value is MinigraphGraphData {
  if (typeof value !== 'object' || value === null) return false;
  const v = value as Record<string, unknown>;
  return Array.isArray(v.nodes);
}
