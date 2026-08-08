import type { MinigraphConnection, MinigraphGraphData, MinigraphNode } from '../../utils/graphTypes';

export interface GraphClipItem {
  node: MinigraphNode;
  connections: MinigraphConnection[];
}

export type NodeContextTarget =
  | { kind: 'single-node'; alias: string }
  | { kind: 'multi-node'; aliases: string[] };

function normalizeAlias(alias: string): string {
  return alias.trim().toLowerCase();
}

function uniqueAliases(aliases: string[]): string[] {
  const seen = new Set<string>();
  return aliases.filter((alias) => {
    const normalized = normalizeAlias(alias);
    if (seen.has(normalized)) return false;
    seen.add(normalized);
    return true;
  });
}

// A right-click only represents the current selection when the clicked node is
// already part of a selection of two or more nodes. Otherwise its actions apply
// to that concrete clicked node.
export function resolveNodeContextTarget(
  clickedAlias: string,
  selectedAliases: string[],
): NodeContextTarget {
  const aliases = uniqueAliases(selectedAliases);
  const clickedNormalized = normalizeAlias(clickedAlias);
  if (aliases.length > 1 && aliases.some((alias) => normalizeAlias(alias) === clickedNormalized)) {
    return { kind: 'multi-node', aliases };
  }
  return { kind: 'single-node', alias: clickedAlias };
}

// Resolve the selection snapshot against current graph data at action time so
// a refresh cannot cause Workspace writes for removed nodes.
export function filterAliasesToGraphNodes(
  aliases: string[],
  graphData: MinigraphGraphData,
): MinigraphNode[] {
  const nodesByAlias = new Map(
    graphData.nodes.map((node) => [normalizeAlias(node.alias), node]),
  );
  return uniqueAliases(aliases)
    .map((alias) => nodesByAlias.get(normalizeAlias(alias)))
    .filter((node): node is MinigraphNode => node !== undefined);
}
