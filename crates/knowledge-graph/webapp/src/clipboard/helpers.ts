import type { MinigraphNode, MinigraphConnection, MinigraphGraphData } from '../utils/graphTypes';

/** Extract the full MinigraphNode from graphData by alias. */
export function findNodeByAlias(
  graphData: MinigraphGraphData,
  alias: string,
): MinigraphNode | undefined {
  return graphData.nodes.find(n => n.alias === alias);
}

/**
 * Extract all connections where the given alias appears as source OR target,
 * excluding self-connections (where source === target).
 */
export function extractDirectConnections(
  graphData: MinigraphGraphData,
  alias: string,
): MinigraphConnection[] {
  return (graphData.connections ?? []).filter(
    c => c.source !== c.target && (c.source === alias || c.target === alias),
  );
}
