import { MarkerType, type Node, type Edge } from '@xyflow/react';
import type { MinigraphGraphData } from './graphTypes';
import { getMinigraphNodeShellStyle } from './minigraphNodeTheme';

/** Data bag attached to every ReactFlow node we create. */
export interface GraphNodeData extends Record<string, unknown> {
  alias: string;
  nodeType: string;   // primary type label, e.g. "Root"
  /** All properties from the MinigraphNode, passed through for rendering. */
  properties: Record<string, unknown>;
  sourceHandles: GraphHandleData[];
  targetHandles: GraphHandleData[];
  /** Back-edge source handles — rendered on the LEFT side (outgoing back-edges from this node). */
  backSourceHandles: GraphHandleData[];
  /** Back-edge target handles — rendered on the RIGHT side (incoming back-edges to this node). */
  backTargetHandles: GraphHandleData[];
  minHeight: number;
}

export interface GraphHandleData {
  id: string;
  offset: number;
}

/** Data bag attached to every ReactFlow edge we create. */
export interface GraphEdgeData extends Record<string, unknown> {
  relationTypes: string[];   // e.g. ["fetch"]
}

// ─── Layout constants ────────────────────────────────────────────────────────
// NODE_WIDTH / NODE_HEIGHT drive column/row spacing in the layout pass.
// They are also used as the initial `width`/`height` on each node so React
// Flow's wrapper has a reasonable size from the very first render.  After
// mount React Flow's own ResizeObserver measures the real DOM dimensions and
// updates accordingly, keeping NodeResizer in sync.
const NODE_WIDTH  = 240;
const NODE_HEIGHT = 100; // rough estimate; ResizeObserver will correct it post-mount
const ROW_GAP            = 60;   // vertical gap between nodes stacked in the same column
const COL_GAP            = 120;  // horizontal gap between columns (levels)
const COMPONENT_GAP      = 360;  // horizontal gap between independent flow trees
const SECTION_GAP        = 120;  // vertical gap between main flow and first segregated row
const SEGREGATED_ROW_GAP = 80;   // vertical gap between successive segregated rows

// ─── Edge styling constants ──────────────────────────────────────────────────
// EDGE_STROKE: --text-muted (rgb 148 163 184) at 42% opacity — slate-400 tinted stroke
// EDGE_LABEL_BG: references the --bg-secondary token directly so label badges
//   automatically track any future token-level surface change.
const EDGE_STROKE         = 'rgba(148, 163, 184, 0.42)';   // --text-muted at 42%
const EDGE_LABEL_BG       = 'var(--bg-secondary)';          // token: --bg-secondary = #f8fafc
const EDGE_HANDLE_GAP     = 24;   // px between adjacent handle anchors on the same side
const EDGE_HANDLE_PADDING = 32;   // min px from node top/bottom to first/last handle

// Semantic edge-type colors — intentional per-relation accent palette.
// Matches the same amber/green/purple axis used by NODE_ACCENT above.
const EDGE_FALLBACK_COLORS = [
  '#0369a1',   // sky-700
  '#15803d',   // green-700
  '#b45309',   // amber-700
  '#7e22ce',   // purple-700
  '#b91c1c',   // red-700
  '#0f766e',   // teal-700
  '#c2410c',   // orange-700
  '#a16207',   // yellow-700
];

// Named relation-type → accent color map.
const EDGE_COLOR_BY_RELATION: Record<string, string> = {
  fetch:        '#0369a1',   // sky-700
  details:      '#0369a1',
  'ext-call':   '#0369a1',
  mapping:      '#b45309',   // amber-700
  compute:      '#b45309',
  calculate:    '#b45309',
  evaluate:     '#b45309',
  fork:         '#7e22ce',   // purple-700
  join:         '#7e22ce',
  one:          '#7e22ce',
  two:          '#6d28d9',   // purple-800
  three:        '#5b21b6',   // violet-800
  more:         '#4c1d95',   // violet-900
  done:         '#15803d',   // green-700
  complete:     '#15803d',
  finish:       '#15803d',
  positive:     '#15803d',
  negative:     '#b91c1c',   // red-700
};

function hashString(value: string): number {
  let hash = 0;
  for (let i = 0; i < value.length; i++) {
    hash = ((hash << 5) - hash) + value.charCodeAt(i);
    hash |= 0;
  }
  return Math.abs(hash);
}

function edgeColor(relationTypes: string[]): string {
  if (relationTypes.length === 0) return EDGE_STROKE;
  const primary = relationTypes[0].trim().toLowerCase();
  const known = EDGE_COLOR_BY_RELATION[primary];
  if (known) return known;
  return EDGE_FALLBACK_COLORS[hashString(primary) % EDGE_FALLBACK_COLORS.length];
}

function edgeSourceHandleId(index: number): string {
  return `source-${index}`;
}

function edgeTargetHandleId(index: number): string {
  return `target-${index}`;
}

function backEdgeSourceHandleId(index: number): string {
  return `back-source-${index}`;
}

function backEdgeTargetHandleId(index: number): string {
  return `back-target-${index}`;
}

function edgeHandleOffset(index: number, total: number): number {
  if (total <= 1) return 0;
  if (total === 2) return index === 0 ? -EDGE_HANDLE_GAP : EDGE_HANDLE_GAP;
  return (index - (total - 1) / 2) * EDGE_HANDLE_GAP;
}


function nodeHeightForHandleCount(handleCount: number): number {
  if (handleCount <= 1) return NODE_HEIGHT;
  return Math.max(NODE_HEIGHT, ((handleCount - 1) * EDGE_HANDLE_GAP) + (EDGE_HANDLE_PADDING * 2));
}

// ─── Layout node classification ─────────────────────────────────────────────
// Nodes are classified into one of several layout categories that determine
// where they appear in the rendered graph.  The classification uses BOTH the
// node's primary type AND its runtime properties (skill, connections).
//
// MODULE_SKILLS — skill values that identify "module" nodes.  A node with one
// of these skills that participates in zero graph connections is a reusable
// computation block invoked via the EXECUTE keyword rather than graph traversal.
//
// SEGREGATED_ROW_ORDER — the ordered list of non-flow layout categories.  Each
// category gets its own horizontal row below the main flow.  Any node that does
// not match a named category falls into a trailing '__unknown__' catch-all row.
const MODULE_SKILLS = new Set(['graph.math', 'graph.js']);

const SEGREGATED_ROW_ORDER: readonly string[] = [
  'Dictionary',   // data extraction contracts from API responses
  'Provider',     // reusable API endpoint configurations
  'Module',       // reusable computation blocks (EXECUTE keyword)
  'Entity',       // skill-less data-holder nodes (business domain objects)
];

// ─── Node classification ────────────────────────────────────────────────────
// Layout categories returned by classifyNode.  'flow' nodes participate in the
// main BFS layout; all other categories are placed in segregated rows below.
type LayoutCategory = 'flow' | 'Dictionary' | 'Provider' | 'Entity' | 'Module' | '__unknown__';
type MinigraphNodeModel = MinigraphGraphData['nodes'][number];

interface FlowComponent {
  aliases: string[];
  nodes: MinigraphNodeModel[];
  hasRoot: boolean;
  hasEnd: boolean;
  sortKey: string;
}

const FLOW_COMPONENT_LAYOUT_ORDER = {
  ROOT_TREE: 0,
  DEFAULT_TREE: 1,
  END_TREE: 2,
} as const;

// Root-like nodes define the primary execution tree. When several disconnected
// flow trees exist, this component is anchored first so graph reading starts on
// the left from the graph entry point.
function isRootLikeNode(node: MinigraphNodeModel): boolean {
  return node.alias.toLowerCase() === 'root' ||
    node.types.includes('Root') ||
    node.types.includes('entry_point');
}

// End-like nodes are terminal-only branches in some imported models. Ranking
// their component last keeps those completion/error branches on the right.
function isEndLikeNode(node: MinigraphNodeModel): boolean {
  return node.alias.toLowerCase() === 'end' || node.types.includes('End');
}

// Keep component ordering policy centralized: root tree first, ordinary trees
// next, terminal/end tree last. The numeric values are only sort ranks.
function getFlowComponentRank(component: FlowComponent): number {
  if (component.hasRoot) return FLOW_COMPONENT_LAYOUT_ORDER.ROOT_TREE;
  if (component.hasEnd) return FLOW_COMPONENT_LAYOUT_ORDER.END_TREE;
  return FLOW_COMPONENT_LAYOUT_ORDER.DEFAULT_TREE;
}

// Sort disconnected flow trees before assigning global columns. Components with
// the same semantic rank use their first alias for deterministic visual order.
function compareFlowComponents(a: FlowComponent, b: FlowComponent): number {
  const rankDiff = getFlowComponentRank(a) - getFlowComponentRank(b);
  if (rankDiff !== 0) return rankDiff;
  return a.sortKey.localeCompare(b.sortKey);
}

/**
 * Classify a node into its layout category.
 *
 * Connected nodes always participate in the main left-to-right BFS flow,
 * matching the original layout behaviour.  Only orphaned (unconnected)
 * nodes are segregated into categorised rows below the flow.
 *
 * Priority order (first match wins):
 *  1. Connected — participates in at least one edge → flow.
 *  2. Dictionary / Provider — orphaned type-based segregation.
 *  3. Module — has a compute skill (graph.math / graph.js) with no
 *     connections.  Reusable computation blocks invoked via EXECUTE.
 *  4. Entity — no skill property; a passive data-holder node.
 *  5. __unknown__ — catch-all safety net for anything else.
 */
function classifyNode(
  node: MinigraphGraphData['nodes'][number],
  connectedAliases: Set<string>,
): LayoutCategory {
  const isConnected = connectedAliases.has(node.alias);
  if (isConnected) return 'flow';

  const pt    = node.types[0] ?? '';
  const skill = typeof node.properties.skill === 'string' ? node.properties.skill : undefined;

  if (pt === 'Dictionary') return 'Dictionary';
  if (pt === 'Provider')   return 'Provider';
  if (skill && MODULE_SKILLS.has(skill)) return 'Module';
  if (!skill) return 'Entity';

  return '__unknown__';
}

/**
 * Left-to-right topological layout with row segregation for non-flow nodes.
 *
 * Strategy:
 *  1. Classify every node via classifyNode (uses type, skill, and connection
 *     participation — see its JSDoc for the full priority chain).
 *     Partition into "flow" vs segregated categories.
 *  2. Split flow nodes into connected components so independent trees do not
 *     collapse into one shared column set.
 *  3. Sort components left-to-right: root component first, end-only component
 *     last, all other components alphabetically in the middle.
 *  4. Assign each component its own BFS "level" columns from root-like or
 *     in-degree-zero seeds. Within each local level, stack nodes vertically,
 *     centred at y = 0.
 *  5. Compute the bounding box of the main flow, then place each segregated
 *     category in its own horizontal row below it, left-aligned with the flow.
 *
 * Segregated row order: Dictionary → Provider → Module → Entity → __unknown__.
 *
 * If a component has no natural seed (no root-like, entry_point, or in-degree
 * zero node), its first alias is used so cyclic components remain renderable.
 */
function computeLayout(
  nodes: MinigraphGraphData['nodes'],
  connections: MinigraphGraphData['connections'],
  nodeHeights: Map<string, number>,
): { positions: Map<string, { x: number; y: number }>; levelOf: Map<string, number> } {
  // ── Step 1: Classify & partition ──────────────────────────────────────────
  // Build the set of aliases that participate in at least one connection so
  // classifyNode can distinguish modules (disconnected) from flow nodes.
  const connectedAliases = new Set<string>();
  for (const conn of connections ?? []) {
    connectedAliases.add(conn.source);
    connectedAliases.add(conn.target);
  }

  const flowNodes:       MinigraphGraphData['nodes'] = [];
  const segregatedNodes: MinigraphGraphData['nodes'] = [];
  // Cache each node's category so we don't classify twice.
  const categoryOf = new Map<string, LayoutCategory>();

  for (const n of nodes) {
    const cat = classifyNode(n, connectedAliases);
    categoryOf.set(n.alias, cat);
    if (cat === 'flow') flowNodes.push(n);
    else segregatedNodes.push(n);
  }

  // ── Step 2: Build directed/undirected flow adjacency ──────────────────────
  const flowAliases = new Set(flowNodes.map(n => n.alias));
  const nodeByAlias = new Map(flowNodes.map(n => [n.alias, n]));
  const outEdges    = new Map<string, string[]>();
  const undirectedEdges = new Map<string, Set<string>>();
  const inDegree    = new Map<string, number>();

  for (const n of flowNodes) {
    outEdges.set(n.alias, []);
    undirectedEdges.set(n.alias, new Set());
    inDegree.set(n.alias, 0);
  }

  for (const conn of connections ?? []) {
    // Only count edges entirely within the flow set so that connections to/from
    // segregated nodes do not influence BFS level assignment.
    if (!flowAliases.has(conn.source) || !flowAliases.has(conn.target)) continue;
    outEdges.get(conn.source)?.push(conn.target);
    undirectedEdges.get(conn.source)?.add(conn.target);
    undirectedEdges.get(conn.target)?.add(conn.source);
    inDegree.set(conn.target, (inDegree.get(conn.target) ?? 0) + 1);
  }

  // Seeds: flow nodes with in-degree 0, or explicitly typed as entry_point.
  // These are used for cycle detection only. Each component chooses its own
  // seeds again during placement.
  const allSeeds = flowNodes
    .filter(n => inDegree.get(n.alias) === 0 || n.types.includes('entry_point') || isRootLikeNode(n))
    .map(n => n.alias);

  // ── Cycle detection: find back-edges via iterative DFS ────────────────────
  // Back-edges (edges pointing to an ancestor in the DFS tree) cause the
  // BFS level-assignment loop below to run forever — each node in a cycle
  // endlessly re-enqueues the other with ever-increasing levels.  We detect
  // them here and exclude them from BFS so cycles are broken for layout
  // purposes.  The edges are still rendered in the final ReactFlow output.
  const backEdges = new Set<string>();
  {
    const WHITE = 0, GRAY = 1, BLACK = 2;
    const color = new Map<string, number>();
    for (const n of flowNodes) color.set(n.alias, WHITE);

    function dfsFrom(root: string) {
      if (color.get(root) !== WHITE) return;
      color.set(root, GRAY);
      const stack: { node: string; childIdx: number }[] = [{ node: root, childIdx: 0 }];

      while (stack.length > 0) {
        const frame = stack[stack.length - 1];
        const neighbors = outEdges.get(frame.node) ?? [];

        if (frame.childIdx >= neighbors.length) {
          color.set(frame.node, BLACK);
          stack.pop();
          continue;
        }

        const neighbor = neighbors[frame.childIdx++];
        const nc = color.get(neighbor);
        if (nc === GRAY) {
          backEdges.add(`${frame.node}\t${neighbor}`);
        } else if (nc === WHITE) {
          color.set(neighbor, GRAY);
          stack.push({ node: neighbor, childIdx: 0 });
        }
        // BLACK → cross or forward edge, safe to ignore
      }
    }

    // Prefer starting from seeds so the DFS tree mirrors the BFS flow.
    for (const s of allSeeds) dfsFrom(s);
    for (const n of flowNodes) dfsFrom(n.alias);
  }

  // ── Step 3: Connected components for independent flow trees ───────────────
  const components: FlowComponent[] = [];
  const seen = new Set<string>();

  for (const start of Array.from(flowAliases).sort()) {
    if (seen.has(start)) continue;

    const aliases: string[] = [];
    const stack = [start];
    seen.add(start);

    while (stack.length > 0) {
      const alias = stack.pop()!;
      aliases.push(alias);
      for (const neighbor of undirectedEdges.get(alias) ?? []) {
        if (seen.has(neighbor)) continue;
        seen.add(neighbor);
        stack.push(neighbor);
      }
    }

    aliases.sort();
    const componentNodes = aliases
      .map(alias => nodeByAlias.get(alias))
      .filter((node): node is MinigraphNodeModel => Boolean(node));

    components.push({
      aliases,
      nodes: componentNodes,
      hasRoot: componentNodes.some(isRootLikeNode),
      hasEnd: componentNodes.some(isEndLikeNode),
      sortKey: aliases[0] ?? '',
    });
  }

  components.sort(compareFlowComponents);

  // ── Step 4: BFS layout for each component, placed left-to-right ───────────
  const levelOf = new Map<string, number>();
  const positions = new Map<string, { x: number; y: number }>();
  let componentLevelOffset = 0;
  let componentXOffset = 0;

  for (const component of components) {
    const componentAliases = new Set(component.aliases);
    const componentSeeds = component.nodes
      .filter(n => inDegree.get(n.alias) === 0 || n.types.includes('entry_point') || isRootLikeNode(n))
      .map(n => n.alias)
      .sort();

    // Cyclic components may have no natural in-degree-zero node. Seed from the
    // first alias so they remain renderable instead of collapsing into orphans.
    if (componentSeeds.length === 0 && component.aliases.length > 0) {
      componentSeeds.push(component.aliases[0]);
    }

    const localLevelOf = new Map<string, number>();
    const queue: string[] = [...componentSeeds];
    componentSeeds.forEach(seed => localLevelOf.set(seed, 0));

    // BFS to assign local levels within the component. This preserves the
    // original left-to-right topological flow, but prevents independent trees
    // from sharing the same column set.
    while (queue.length > 0) {
      const current = queue.shift()!;
      const currentLevel = localLevelOf.get(current) ?? 0;
      for (const neighbor of outEdges.get(current) ?? []) {
        // Skip cross-component edges defensively; components were built from
        // the same flow adjacency, so this should only guard stale input.
        if (!componentAliases.has(neighbor)) continue;
        // Skip back-edges — they create cycles and are excluded from layout
        // assignment but are still rendered as visual edges in the output.
        if (backEdges.has(`${current}\t${neighbor}`)) continue;
        // Only advance the level; never move a node to a shallower level.
        if (!localLevelOf.has(neighbor) || localLevelOf.get(neighbor)! <= currentLevel) {
          localLevelOf.set(neighbor, currentLevel + 1);
          queue.push(neighbor);
        }
      }
    }

    // Flow nodes that BFS never visited stay in this component and move to the
    // last local level + 1. This keeps cyclic or disconnected-within-component
    // data renderable instead of dropping nodes from the layout.
    const maxLocalLevel = localLevelOf.size > 0 ? Math.max(...localLevelOf.values()) : 0;
    for (const alias of component.aliases) {
      if (!localLevelOf.has(alias)) localLevelOf.set(alias, maxLocalLevel + 1);
    }

    const byLocalLevel = new Map<number, string[]>();
    for (const [alias, level] of localLevelOf) {
      if (!byLocalLevel.has(level)) byLocalLevel.set(level, []);
      byLocalLevel.get(level)!.push(alias);
    }

    // Assign pixel positions for this component's flow — centred at y = 0 per
    // local column. Per-node heights prevent overlap when nodes are taller than
    // NODE_HEIGHT. componentXOffset gives each independent tree a fixed pixel
    // gap from the previous tree instead of tying tree spacing to column count.
    let componentMaxX = componentXOffset;
    for (const [localLevel, aliases] of [...byLocalLevel].sort(([a], [b]) => a - b)) {
      const sortedAliases = aliases.slice().sort();
      const totalHeight = sortedAliases.reduce(
        (sum, alias) => sum + (nodeHeights.get(alias) ?? NODE_HEIGHT),
        0,
      ) + Math.max(0, sortedAliases.length - 1) * ROW_GAP;

      let cursorY = -totalHeight / 2;
      const globalLevel = componentLevelOffset + localLevel;
      const x = componentXOffset + localLevel * (NODE_WIDTH + COL_GAP);
      componentMaxX = Math.max(componentMaxX, x);
      sortedAliases.forEach((alias) => {
        const nodeHeight = nodeHeights.get(alias) ?? NODE_HEIGHT;
        levelOf.set(alias, globalLevel);
        positions.set(alias, {
          x,
          y: cursorY,
        });
        cursorY += nodeHeight + ROW_GAP;
      });
    }

    const componentMaxLevel = localLevelOf.size > 0 ? Math.max(...localLevelOf.values()) : 0;
    componentLevelOffset += componentMaxLevel + 1;
    componentXOffset = componentMaxX + NODE_WIDTH + COMPONENT_GAP;
  }

  // ── Step 5: Bounding box of the main flow ─────────────────────────────────
  // Used to anchor the vertical start of the segregated rows.
  let mainMaxY = 0;
  for (const [alias, pos] of positions) {
    mainMaxY = Math.max(mainMaxY, pos.y + (nodeHeights.get(alias) ?? NODE_HEIGHT));
  }
  // If there are no flow nodes at all, start at y = 0 with no section gap.
  let nextRowY = mainMaxY + (positions.size > 0 ? SECTION_GAP : 0);

  // ── Step 6: Segregated rows ───────────────────────────────────────────────
  // Group segregated nodes by their layout category (already computed in Step 1).
  const groupMap = new Map<string, string[]>();
  for (const key of SEGREGATED_ROW_ORDER) groupMap.set(key, []);
  groupMap.set('__unknown__', []);

  for (const n of segregatedNodes) {
    const cat = categoryOf.get(n.alias) as Exclude<LayoutCategory, 'flow'>;
    groupMap.get(cat)!.push(n.alias);
  }

  for (const key of [...SEGREGATED_ROW_ORDER, '__unknown__']) {
    const aliases = (groupMap.get(key) ?? []).slice().sort(); // alphabetical for visual stability
    if (aliases.length === 0) continue;

    const startX = 0; // left-align segregated rows with the main flow
    const rowHeight = aliases.reduce(
      (max, alias) => Math.max(max, nodeHeights.get(alias) ?? NODE_HEIGHT),
      0,
    );

    aliases.forEach((alias, i) => {
      positions.set(alias, {
        x: startX + i * (NODE_WIDTH + COL_GAP),
        y: nextRowY,
      });
    });

    nextRowY += rowHeight + SEGREGATED_ROW_GAP;
  }

  return { positions, levelOf };
}

/**
 * Converts a MinigraphGraphData object into the ReactFlow `nodes` + `edges`
 * arrays ready to be passed to `<ReactFlow>`.
 */
export function transformGraphData(
  data: MinigraphGraphData,
): { nodes: Node<GraphNodeData>[]; edges: Edge<GraphEdgeData>[] } {
  const connections = data.connections ?? [];

  // ── Approximate node heights for layout ────────────────────────────────────
  // Count total outgoing/incoming to get rough handle counts.  The layout only
  // needs heights for vertical stacking; accurate per-side counts come later
  // once we know which edges are back-edges.
  const totalOutgoing = new Map<string, number>();
  const totalIncoming = new Map<string, number>();
  for (const conn of connections) {
    totalOutgoing.set(conn.source, (totalOutgoing.get(conn.source) ?? 0) + 1);
    totalIncoming.set(conn.target, (totalIncoming.get(conn.target) ?? 0) + 1);
  }

  const approxNodeHeights = new Map(
    data.nodes.map(n => [
      n.alias,
      nodeHeightForHandleCount(Math.max(
        totalOutgoing.get(n.alias) ?? 0,
        totalIncoming.get(n.alias) ?? 0,
      )),
    ]),
  );
  const { positions, levelOf } = computeLayout(data.nodes, connections, approxNodeHeights);

  // ── Classify connections as forward or backward ───────────────────────────
  // A back-edge goes from a deeper (or equal) level to a shallower level.
  // These edges exit from the LEFT side of the source and enter the RIGHT
  // side of the target — the reverse of forward edges — so the bezier curve
  // arcs naturally backward.
  const backEdgeIndices = new Set<number>();
  for (const [i, conn] of connections.entries()) {
    const srcLevel = levelOf.get(conn.source);
    const tgtLevel = levelOf.get(conn.target);
    if (srcLevel !== undefined && tgtLevel !== undefined && srcLevel >= tgtLevel) {
      backEdgeIndices.add(i);
    }
  }

  // ── Collect per-node, per-side connections sorted by peer y-position ─────
  // Sorting handles by the y-position of the connected peer node prevents
  // crossing: connections to a higher peer get a higher handle slot, and
  // connections to a lower peer get a lower slot.  Forward and back-edge
  // handles are interleaved within the sorted order rather than grouped
  // separately, so a node that has both a forward edge and a retry to the
  // same peer gets adjacent handles for both.
  //
  // Each side entry records the connection index, the peer alias, and
  // whether the connection is a back-edge.  After sorting we walk the
  // entries to build handle arrays and a connectionIndex → handleId map
  // used when constructing ReactFlow edges.

  interface SideEntry { connIndex: number; peerAlias: string; isBack: boolean }

  const rightSide = new Map<string, SideEntry[]>(); // source (fwd out) + back-target (back in)
  const leftSide  = new Map<string, SideEntry[]>(); // target (fwd in)  + back-source (back out)

  for (const n of data.nodes) {
    rightSide.set(n.alias, []);
    leftSide.set(n.alias, []);
  }

  for (const [i, conn] of connections.entries()) {
    if (backEdgeIndices.has(i)) {
      // Back-edge: source exits LEFT, target enters RIGHT
      leftSide.get(conn.source)!.push({ connIndex: i, peerAlias: conn.target, isBack: true });
      rightSide.get(conn.target)!.push({ connIndex: i, peerAlias: conn.source, isBack: true });
    } else {
      // Forward: source exits RIGHT, target enters LEFT
      rightSide.get(conn.source)!.push({ connIndex: i, peerAlias: conn.target, isBack: false });
      leftSide.get(conn.target)!.push({ connIndex: i, peerAlias: conn.source, isBack: false });
    }
  }

  // Sort each side by peer y-position so handle order matches spatial layout.
  const peerY = (alias: string) => positions.get(alias)?.y ?? 0;
  for (const entries of rightSide.values()) entries.sort((a, b) => peerY(a.peerAlias) - peerY(b.peerAlias));
  for (const entries of leftSide.values())  entries.sort((a, b) => peerY(a.peerAlias) - peerY(b.peerAlias));

  // Maps from connection index → handle ID, populated during node building.
  const connSourceHandle = new Map<number, string>();
  const connTargetHandle = new Map<number, string>();

  const rfNodes: Node<GraphNodeData>[] = data.nodes.map(n => {
    const right = rightSide.get(n.alias) ?? [];
    const left  = leftSide.get(n.alias) ?? [];
    const nodeHeight = nodeHeightForHandleCount(Math.max(right.length, left.length));

    // ── Right side: interleaved source + back-target handles ──
    const sourceHandles:     GraphHandleData[] = [];
    const backTargetHandles: GraphHandleData[] = [];
    let srcIdx = 0, btIdx = 0;
    for (let i = 0; i < right.length; i++) {
      const entry = right[i];
      const offset = edgeHandleOffset(i, right.length);
      if (entry.isBack) {
        const id = backEdgeTargetHandleId(btIdx++);
        backTargetHandles.push({ id, offset });
        connTargetHandle.set(entry.connIndex, id);
      } else {
        const id = edgeSourceHandleId(srcIdx++);
        sourceHandles.push({ id, offset });
        connSourceHandle.set(entry.connIndex, id);
      }
    }

    // ── Left side: interleaved target + back-source handles ──
    const targetHandles:     GraphHandleData[] = [];
    const backSourceHandles: GraphHandleData[] = [];
    let tgtIdx = 0, bsIdx = 0;
    for (let i = 0; i < left.length; i++) {
      const entry = left[i];
      const offset = edgeHandleOffset(i, left.length);
      if (entry.isBack) {
        const id = backEdgeSourceHandleId(bsIdx++);
        backSourceHandles.push({ id, offset });
        connSourceHandle.set(entry.connIndex, id);
      } else {
        const id = edgeTargetHandleId(tgtIdx++);
        targetHandles.push({ id, offset });
        connTargetHandle.set(entry.connIndex, id);
      }
    }

    return {
      id:       n.alias,
      type:     n.types[0] ?? 'default',
      position: positions.get(n.alias) ?? { x: 0, y: 0 },
      width:  NODE_WIDTH,
      height: nodeHeight,
      style: getMinigraphNodeShellStyle(n.types[0] ?? 'unknown'),
      data: {
        alias:         n.alias,
        nodeType:      n.types[0] ?? 'unknown',
        properties:    n.properties,
        sourceHandles,
        targetHandles,
        backSourceHandles,
        backTargetHandles,
        minHeight:     nodeHeight,
      },
    };
  });

  // ── Build edges using the pre-computed handle mappings ─────────────────────
  const rfEdges: Edge<GraphEdgeData>[] = [];
  for (const [index, conn] of connections.entries()) {
    const relationTypes = conn.relations.map(r => r.type);
    const edgeId = `${conn.source}__${conn.target}__${index}`;
    const labelColor = edgeColor(relationTypes);

    rfEdges.push({
      id:           edgeId,
      source:       conn.source,
      target:       conn.target,
      sourceHandle: connSourceHandle.get(index)!,
      targetHandle: connTargetHandle.get(index)!,
      label:        relationTypes.join(', '),
      type:         'bezier',
      markerEnd: {
        type:   MarkerType.ArrowClosed,
        width:  16,
        height: 16,
        color:  EDGE_STROKE,
      },
      style: {
        stroke:      EDGE_STROKE,
        strokeWidth: 2,
      },
      labelStyle: {
        fill:       labelColor,
        fontSize:   10,
        fontWeight: 700,
      },
      labelBgStyle: {
        fill:        EDGE_LABEL_BG,
        fillOpacity: 0.94,
        stroke:      'rgba(15, 23, 42, 0.16)',
        strokeWidth: 1,
      },
      labelBgPadding:      [5, 2],
      labelBgBorderRadius: 6,
      data: { relationTypes },
    });
  }

  return { nodes: rfNodes, edges: rfEdges };
}
