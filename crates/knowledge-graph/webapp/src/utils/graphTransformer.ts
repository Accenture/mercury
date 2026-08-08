import { MarkerType, type Node, type Edge } from '@xyflow/react';
import type { MinigraphGraphData } from './graphTypes';
import { getMinigraphNodeShellStyle } from './minigraphNodeTheme';
import { getConnectionRelationColor } from '../graphActions/connectionRelations';

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
  supportsConnectionAuthoring: boolean;
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
const CROSSING_REDUCTION_PASSES = 4;
const MAX_EXPANDED_LAYOUT_SEGMENTS = 10_000;
const MAX_GEOMETRY_EDGE_NODE_PAIRS = 2_000;
const MAX_TRANSPOSE_SLOTS = 64;
const MAX_TRANSPOSE_SEGMENTS = 256;
const MAX_TRANSPOSE_GEOMETRY_PAIRS = 256;
const MAX_TRANSPOSE_EVALUATIONS = 512;
const NODE_INTRUSION_EPSILON = 0.001;

// ─── Edge styling constants ──────────────────────────────────────────────────
// EDGE_STROKE: --text-muted (rgb 148 163 184) at 42% opacity — slate-400 tinted stroke
// EDGE_LABEL_BG: references the --bg-secondary token directly so label badges
//   automatically track any future token-level surface change.
const EDGE_STROKE         = 'rgba(148, 163, 184, 0.42)';   // --text-muted at 42%
const EDGE_LABEL_BG       = 'var(--bg-secondary)';          // token: --bg-secondary = #f8fafc
const EDGE_HANDLE_GAP     = 24;   // px between adjacent handle anchors on the same side
const EDGE_HANDLE_PADDING = 32;   // min px from node top/bottom to first/last handle

function edgeColor(relationTypes: string[]): string {
  return getConnectionRelationColor(relationTypes, EDGE_STROKE);
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

interface LayoutSlot {
  id: string;
  /** Real node alias. Virtual long-edge slots have no alias and are never rendered. */
  alias?: string;
  height: number;
  stableKey: string;
}

interface LayoutSegment {
  sourceId: string;
  targetId: string;
  level: number;
}

interface LayoutScore {
  nodeIntrusions: number;
  edgeCrossings: number;
}

interface CandidateNodeGeometry {
  alias: string;
  level: number;
  x: number;
  y: number;
  height: number;
}

interface CandidateSideEntry {
  connectionIndex: number;
  peerAlias: string;
  isBack: boolean;
  stableKey: string;
}

interface GeometryPair {
  connectionIndex: number;
  nodeAlias: string;
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

function realSlotId(alias: string): string {
  return `real:${alias}`;
}

function cloneLayers(layers: Map<number, LayoutSlot[]>): Map<number, LayoutSlot[]> {
  return new Map([...layers].map(([level, slots]) => [level, slots.slice()]));
}

function slotPositions(slots: LayoutSlot[]): Map<string, number> {
  return new Map(slots.map((slot, index) => [slot.id, index]));
}

function averageNeighborPosition(
  slotId: string,
  neighborsBySlot: Map<string, string[]>,
  neighborPositions: Map<string, number>,
): number | undefined {
  const positions = (neighborsBySlot.get(slotId) ?? [])
    .map(neighbor => neighborPositions.get(neighbor))
    .filter((position): position is number => position !== undefined);
  if (positions.length === 0) return undefined;
  return positions.reduce((sum, position) => sum + position, 0) / positions.length;
}

/**
 * Reorder one rank using the barycenter of its adjacent-rank neighbors.
 * Previous position and stable key make equal barycenters deterministic.
 */
function reorderLayer(
  slots: LayoutSlot[],
  neighborsBySlot: Map<string, string[]>,
  neighborPositions: Map<string, number>,
): LayoutSlot[] {
  const previousPositions = slotPositions(slots);
  return slots.slice().sort((a, b) => {
    const aBarycenter = averageNeighborPosition(a.id, neighborsBySlot, neighborPositions);
    const bBarycenter = averageNeighborPosition(b.id, neighborsBySlot, neighborPositions);
    if (aBarycenter !== undefined && bBarycenter !== undefined) {
      const barycenterDifference = aBarycenter - bBarycenter;
      if (Math.abs(barycenterDifference) > Number.EPSILON) return barycenterDifference;
    }
    const positionDifference = previousPositions.get(a.id)! - previousPositions.get(b.id)!;
    if (positionDifference !== 0) return positionDifference;
    return a.stableKey.localeCompare(b.stableKey);
  });
}

function countStrictInversions(values: number[], valueCount: number): number {
  const tree = new Array<number>(valueCount + 1).fill(0);
  const query = (index: number) => {
    let total = 0;
    for (let cursor = index; cursor > 0; cursor -= cursor & -cursor) {
      total += tree[cursor];
    }
    return total;
  };
  const update = (index: number) => {
    for (let cursor = index; cursor < tree.length; cursor += cursor & -cursor) {
      tree[cursor] += 1;
    }
  };

  let inversions = 0;
  for (let index = values.length - 1; index >= 0; index--) {
    const oneBasedValue = values[index] + 1;
    inversions += query(oneBasedValue - 1);
    update(oneBasedValue);
  }
  return inversions;
}

/**
 * Count crossings in the layered representation. Segments are ordered by
 * source position and target position, so shared sources and shared targets
 * do not count as crossings.
 */
function countLayerCrossings(
  layers: Map<number, LayoutSlot[]>,
  segments: LayoutSegment[],
): number {
  const positionsByLevel = new Map(
    [...layers].map(([level, slots]) => [level, slotPositions(slots)]),
  );
  const segmentsByLevel = new Map<number, LayoutSegment[]>();
  for (const segment of segments) {
    if (!segmentsByLevel.has(segment.level)) segmentsByLevel.set(segment.level, []);
    segmentsByLevel.get(segment.level)!.push(segment);
  }

  let crossings = 0;
  for (const [level, levelSegments] of segmentsByLevel) {
    const sourcePositions = positionsByLevel.get(level);
    const targetPositions = positionsByLevel.get(level + 1);
    if (!sourcePositions || !targetPositions) continue;
    const orderedTargetPositions = levelSegments
      .map(segment => ({
        sourceId: segment.sourceId,
        targetId: segment.targetId,
        sourcePosition: sourcePositions.get(segment.sourceId)!,
        targetPosition: targetPositions.get(segment.targetId)!,
      }))
      .sort((a, b) =>
        a.sourcePosition - b.sourcePosition ||
        a.targetPosition - b.targetPosition ||
        a.sourceId.localeCompare(b.sourceId) ||
        a.targetId.localeCompare(b.targetId),
      )
      .map(segment => segment.targetPosition);
    crossings += countStrictInversions(orderedTargetPositions, targetPositions.size);
  }
  return crossings;
}

function addNeighbor(
  neighborsBySlot: Map<string, string[]>,
  slotId: string,
  neighborId: string,
): void {
  if (!neighborsBySlot.has(slotId)) neighborsBySlot.set(slotId, []);
  neighborsBySlot.get(slotId)!.push(neighborId);
}

function connectionStableKey(
  connection: MinigraphGraphData['connections'][number],
): string {
  return [
    connection.source,
    connection.target,
    ...connection.relations.map(relation => relation.type),
  ].join('\t');
}

function isScoreBetter(candidate: LayoutScore, current: LayoutScore): boolean {
  return candidate.nodeIntrusions < current.nodeIntrusions ||
    (candidate.nodeIntrusions === current.nodeIntrusions &&
      candidate.edgeCrossings < current.edgeCrossings);
}

function candidateNodeGeometry(
  layers: Map<number, LayoutSlot[]>,
): Map<string, CandidateNodeGeometry> {
  const geometry = new Map<string, CandidateNodeGeometry>();
  for (const [level, slots] of layers) {
    const totalHeight = slots.reduce(
      (sum, slot) => sum + slot.height,
      0,
    ) + Math.max(0, slots.length - 1) * ROW_GAP;
    let cursorY = -totalHeight / 2;
    for (const slot of slots) {
      if (slot.alias) {
        geometry.set(slot.alias, {
          alias: slot.alias,
          level,
          x: level * (NODE_WIDTH + COL_GAP),
          y: cursorY,
          height: slot.height,
        });
      }
      cursorY += slot.height + ROW_GAP;
    }
  }
  return geometry;
}

function cubicCoordinate(
  start: number,
  startControl: number,
  endControl: number,
  end: number,
  parameter: number,
): number {
  const inverse = 1 - parameter;
  return (inverse * inverse * inverse * start) +
    (3 * inverse * inverse * parameter * startControl) +
    (3 * inverse * parameter * parameter * endControl) +
    (parameter * parameter * parameter * end);
}

function parameterAtX(
  startX: number,
  controlX: number,
  endX: number,
  targetX: number,
): number {
  let low = 0;
  let high = 1;
  for (let iteration = 0; iteration < 32; iteration++) {
    const middle = (low + high) / 2;
    const x = cubicCoordinate(startX, controlX, controlX, endX, middle);
    if (x < targetX) low = middle;
    else high = middle;
  }
  return (low + high) / 2;
}

function forwardBezierIntersectsNode(
  source: CandidateNodeGeometry,
  target: CandidateNodeGeometry,
  sourceOffset: number,
  targetOffset: number,
  node: CandidateNodeGeometry,
): boolean {
  const sourceX = source.x + NODE_WIDTH;
  const targetX = target.x;
  const nodeLeft = node.x + NODE_INTRUSION_EPSILON;
  const nodeRight = node.x + NODE_WIDTH - NODE_INTRUSION_EPSILON;
  if (nodeLeft >= targetX || nodeRight <= sourceX) return false;

  const controlX = sourceX + ((targetX - sourceX) / 2);
  const startParameter = parameterAtX(sourceX, controlX, targetX, nodeLeft);
  const endParameter = parameterAtX(sourceX, controlX, targetX, nodeRight);
  const sourceY = source.y + (source.height / 2) + sourceOffset;
  const targetY = target.y + (target.height / 2) + targetOffset;
  const startY = cubicCoordinate(
    sourceY,
    sourceY,
    targetY,
    targetY,
    startParameter,
  );
  const endY = cubicCoordinate(
    sourceY,
    sourceY,
    targetY,
    targetY,
    endParameter,
  );
  const nodeTop = node.y + NODE_INTRUSION_EPSILON;
  const nodeBottom = node.y + node.height - NODE_INTRUSION_EPSILON;
  return Math.max(startY, endY) > nodeTop &&
    Math.min(startY, endY) < nodeBottom;
}

function collectGeometryPairs(
  connections: MinigraphGraphData['connections'],
  localLevelOf: Map<string, number>,
): GeometryPair[] | null {
  const aliasesByLevel = new Map<number, string[]>();
  for (const [alias, level] of localLevelOf) {
    if (!aliasesByLevel.has(level)) aliasesByLevel.set(level, []);
    aliasesByLevel.get(level)!.push(alias);
  }
  for (const aliases of aliasesByLevel.values()) aliases.sort();

  const pairs: GeometryPair[] = [];
  for (const [connectionIndex, connection] of connections.entries()) {
    const sourceLevel = localLevelOf.get(connection.source);
    const targetLevel = localLevelOf.get(connection.target);
    if (sourceLevel === undefined || targetLevel === undefined || targetLevel - sourceLevel <= 1) {
      continue;
    }
    for (let level = sourceLevel + 1; level < targetLevel; level++) {
      for (const nodeAlias of aliasesByLevel.get(level) ?? []) {
        pairs.push({ connectionIndex, nodeAlias });
        if (pairs.length > MAX_GEOMETRY_EDGE_NODE_PAIRS) return null;
      }
    }
  }
  return pairs;
}

/**
 * Score the geometry that React Flow will actually draw for forward long
 * edges. Positive-distance left/right Beziers have monotonic x and y, so
 * testing their y-range while they traverse a node's x-range is exact.
 */
function countCandidateNodeIntrusions(
  layers: Map<number, LayoutSlot[]>,
  localLevelOf: Map<string, number>,
  connections: MinigraphGraphData['connections'],
  geometryPairs: GeometryPair[],
): number {
  const geometry = candidateNodeGeometry(layers);
  const rightSide = new Map<string, CandidateSideEntry[]>();
  const leftSide = new Map<string, CandidateSideEntry[]>();
  for (const alias of localLevelOf.keys()) {
    rightSide.set(alias, []);
    leftSide.set(alias, []);
  }

  for (const [connectionIndex, connection] of connections.entries()) {
    const sourceLevel = localLevelOf.get(connection.source);
    const targetLevel = localLevelOf.get(connection.target);
    if (sourceLevel === undefined || targetLevel === undefined) continue;
    const isBack = sourceLevel >= targetLevel;
    const stableKey = connectionStableKey(connection);
    if (isBack) {
      leftSide.get(connection.source)!.push({
        connectionIndex,
        peerAlias: connection.target,
        isBack,
        stableKey,
      });
      rightSide.get(connection.target)!.push({
        connectionIndex,
        peerAlias: connection.source,
        isBack,
        stableKey,
      });
    } else {
      rightSide.get(connection.source)!.push({
        connectionIndex,
        peerAlias: connection.target,
        isBack,
        stableKey,
      });
      leftSide.get(connection.target)!.push({
        connectionIndex,
        peerAlias: connection.source,
        isBack,
        stableKey,
      });
    }
  }

  const peerY = (alias: string) => geometry.get(alias)?.y ?? 0;
  const compareEntries = (a: CandidateSideEntry, b: CandidateSideEntry) =>
    peerY(a.peerAlias) - peerY(b.peerAlias) ||
    a.peerAlias.localeCompare(b.peerAlias) ||
    a.stableKey.localeCompare(b.stableKey) ||
    a.connectionIndex - b.connectionIndex;
  for (const entries of rightSide.values()) entries.sort(compareEntries);
  for (const entries of leftSide.values()) entries.sort(compareEntries);

  const sourceOffsets = new Map<number, number>();
  const targetOffsets = new Map<number, number>();
  for (const entries of rightSide.values()) {
    for (const [index, entry] of entries.entries()) {
      const offset = edgeHandleOffset(index, entries.length);
      if (entry.isBack) targetOffsets.set(entry.connectionIndex, offset);
      else sourceOffsets.set(entry.connectionIndex, offset);
    }
  }
  for (const entries of leftSide.values()) {
    for (const [index, entry] of entries.entries()) {
      const offset = edgeHandleOffset(index, entries.length);
      if (entry.isBack) sourceOffsets.set(entry.connectionIndex, offset);
      else targetOffsets.set(entry.connectionIndex, offset);
    }
  }

  let intrusions = 0;
  for (const { connectionIndex, nodeAlias } of geometryPairs) {
    const connection = connections[connectionIndex];
    const source = geometry.get(connection.source);
    const target = geometry.get(connection.target);
    const node = geometry.get(nodeAlias);
    if (!source || !target || !node) continue;
    if (forwardBezierIntersectsNode(
      source,
      target,
      sourceOffsets.get(connectionIndex) ?? 0,
      targetOffsets.get(connectionIndex) ?? 0,
      node,
    )) {
      intrusions += 1;
    }
  }
  return intrusions;
}

function transposeForBetterScore(
  layers: Map<number, LayoutSlot[]>,
  levels: number[],
  evaluate: (candidate: Map<number, LayoutSlot[]>) => LayoutScore,
  evaluationBudget: number,
): number {
  const signature = (candidate: Map<number, LayoutSlot[]>) => levels
    .map(level => candidate.get(level)!.map(slot => slot.id).join('\t'))
    .join('\n');
  const start = cloneLayers(layers);
  let evaluations = 1;
  let best = start;
  let bestScore = evaluate(start);
  const queue: Array<{ candidate: Map<number, LayoutSlot[]>; depth: number }> = [
    { candidate: start, depth: 0 },
  ];
  const seen = new Set([signature(start)]);
  let queueIndex = 0;

  while (queueIndex < queue.length && evaluations < evaluationBudget) {
    const { candidate: current, depth } = queue[queueIndex++];
    if (depth >= 2) continue;
    for (const level of levels) {
      const slots = current.get(level)!;
      for (let from = 0; from < slots.length; from++) {
        for (let to = 0; to < slots.length; to++) {
          if (from === to) continue;
          const next = cloneLayers(current);
          const nextSlots = next.get(level)!;
          const [moved] = nextSlots.splice(from, 1);
          nextSlots.splice(to, 0, moved);
          const nextSignature = signature(next);
          if (seen.has(nextSignature)) continue;
          seen.add(nextSignature);
          const score = evaluate(next);
          evaluations += 1;
          if (isScoreBetter(score, bestScore)) {
            best = next;
            bestScore = score;
          }
          if (depth + 1 < 2) {
            queue.push({ candidate: next, depth: depth + 1 });
          }
          if (evaluations >= evaluationBudget) break;
        }
        if (evaluations >= evaluationBudget) break;
      }
      if (evaluations >= evaluationBudget) break;
    }
  }

  for (const level of levels) {
    layers.set(level, best.get(level)!.slice());
  }
  return evaluations;
}

/**
 * Build a layered representation with virtual slots for long forward edges,
 * then keep the best ordering found by a fixed number of down/up sweeps.
 */
function minimizeCrossings(
  localLevelOf: Map<string, number>,
  connections: MinigraphGraphData['connections'],
  backEdges: Set<string>,
  nodeHeights: Map<string, number>,
): Map<number, LayoutSlot[]> {
  const realLayers = new Map<number, LayoutSlot[]>();
  for (const [alias, level] of localLevelOf) {
    if (!realLayers.has(level)) realLayers.set(level, []);
    realLayers.get(level)!.push({
      id: realSlotId(alias),
      alias,
      height: nodeHeights.get(alias) ?? NODE_HEIGHT,
      stableKey: `real:${alias}`,
    });
  }
  for (const slots of realLayers.values()) {
    slots.sort((a, b) => a.stableKey.localeCompare(b.stableKey));
  }
  const baselineLayers = cloneLayers(realLayers);
  const layers = cloneLayers(realLayers);

  const forwardEdges = connections
    .map((connection, connectionIndex) => ({
      connectionIndex,
      source: connection.source,
      target: connection.target,
      sourceLevel: localLevelOf.get(connection.source),
      targetLevel: localLevelOf.get(connection.target),
      stableKey: connectionStableKey(connection),
    }))
    .filter(edge =>
      edge.sourceLevel !== undefined &&
      edge.targetLevel !== undefined &&
      !backEdges.has(`${edge.source}\t${edge.target}`) &&
      edge.sourceLevel < edge.targetLevel,
    )
    .sort((a, b) =>
      a.source.localeCompare(b.source) ||
      a.target.localeCompare(b.target) ||
      a.stableKey.localeCompare(b.stableKey) ||
      a.connectionIndex - b.connectionIndex,
    );
  const expandedSegmentCount = forwardEdges.reduce(
    (sum, edge) => sum + (edge.targetLevel! - edge.sourceLevel!),
    0,
  );
  const expandLongEdges = expandedSegmentCount <= MAX_EXPANDED_LAYOUT_SEGMENTS;

  const predecessors = new Map<string, string[]>();
  const successors = new Map<string, string[]>();
  const segments: LayoutSegment[] = [];

  for (const [edgeIndex, edge] of forwardEdges.entries()) {
    const sourceLevel = edge.sourceLevel!;
    const targetLevel = edge.targetLevel!;
    if (!expandLongEdges && targetLevel - sourceLevel > 1) continue;
    let previousId = realSlotId(edge.source);
    let previousLevel = sourceLevel;

    for (let level = sourceLevel + 1; level < targetLevel; level++) {
      const dummyId = `dummy:${edge.source}\t${edge.target}\t${edgeIndex}\t${level}`;
      if (!layers.has(level)) layers.set(level, []);
      layers.get(level)!.push({
        id: dummyId,
        height: NODE_HEIGHT,
        stableKey: dummyId,
      });
      segments.push({ sourceId: previousId, targetId: dummyId, level: previousLevel });
      addNeighbor(successors, previousId, dummyId);
      addNeighbor(predecessors, dummyId, previousId);
      previousId = dummyId;
      previousLevel = level;
    }

    const targetId = realSlotId(edge.target);
    segments.push({ sourceId: previousId, targetId, level: previousLevel });
    addNeighbor(successors, previousId, targetId);
    addNeighbor(predecessors, targetId, previousId);
  }

  for (const slots of layers.values()) {
    slots.sort((a, b) => a.stableKey.localeCompare(b.stableKey));
  }

  const levels = [...layers.keys()].sort((a, b) => a - b);
  if (levels.length <= 1 || segments.length === 0) return baselineLayers;

  let working = cloneLayers(layers);
  let best = cloneLayers(layers);
  const geometryPairs = collectGeometryPairs(connections, localLevelOf);
  const geometryPairCount = geometryPairs?.length ?? (MAX_GEOMETRY_EDGE_NODE_PAIRS + 1);
  const scoreGeometry = geometryPairs !== null;
  const evaluate = (candidate: Map<number, LayoutSlot[]>): LayoutScore => ({
    nodeIntrusions: scoreGeometry
      ? countCandidateNodeIntrusions(
          candidate,
          localLevelOf,
          connections,
          geometryPairs ?? [],
        )
      : 0,
    edgeCrossings: countLayerCrossings(candidate, segments),
  });
  let bestScore = evaluate(best);

  const keepIfBetter = () => {
    const score = evaluate(working);
    if (isScoreBetter(score, bestScore)) {
      bestScore = score;
      best = cloneLayers(working);
    }
  };

  const totalSlots = [...layers.values()].reduce((sum, slots) => sum + slots.length, 0);
  const canTranspose = totalSlots <= MAX_TRANSPOSE_SLOTS &&
    segments.length <= MAX_TRANSPOSE_SEGMENTS &&
    geometryPairCount <= MAX_TRANSPOSE_GEOMETRY_PAIRS;
  let transposeEvaluations = 0;
  const transpose = () => {
    const remaining = MAX_TRANSPOSE_EVALUATIONS - transposeEvaluations;
    if (!canTranspose || remaining <= 1) return;
    transposeEvaluations += transposeForBetterScore(
      working,
      levels,
      evaluate,
      remaining,
    );
    keepIfBetter();
  };

  for (let pass = 0; pass < CROSSING_REDUCTION_PASSES; pass++) {
    for (let index = 1; index < levels.length; index++) {
      const level = levels[index];
      const previousLevel = levels[index - 1];
      working.set(
        level,
        reorderLayer(
          working.get(level)!,
          predecessors,
          slotPositions(working.get(previousLevel)!),
        ),
      );
    }
    keepIfBetter();
    transpose();

    for (let index = levels.length - 2; index >= 0; index--) {
      const level = levels[index];
      const nextLevel = levels[index + 1];
      working.set(
        level,
        reorderLayer(
          working.get(level)!,
          successors,
          slotPositions(working.get(nextLevel)!),
        ),
      );
    }
    keepIfBetter();
    transpose();
  }

  if (scoreGeometry) {
    const baselineIntrusions = countCandidateNodeIntrusions(
      baselineLayers,
      localLevelOf,
      connections,
      geometryPairs ?? [],
    );
    if (baselineIntrusions < bestScore.nodeIntrusions) return baselineLayers;
  }
  return best;
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
 *     in-degree-zero seeds. Use fixed barycentric sweeps and presentation-only
 *     virtual slots to reduce crossings and reserve long-edge corridors.
 *  5. Stack the ordered real/virtual slots vertically, centred at y = 0.
 *  6. Compute the bounding box of the main flow, then place each segregated
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
  for (const neighbors of outEdges.values()) neighbors.sort();

  // Seeds: flow nodes with in-degree 0, or explicitly typed as entry_point.
  // These are used for cycle detection only. Each component chooses its own
  // seeds again during placement.
  const allSeeds = flowNodes
    .filter(n => inDegree.get(n.alias) === 0 || n.types.includes('entry_point') || isRootLikeNode(n))
    .map(n => n.alias)
    .sort();

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
    for (const alias of [...flowAliases].sort()) dfsFrom(alias);
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
  const componentIndexByAlias = new Map<string, number>();
  components.forEach((component, componentIndex) => {
    component.aliases.forEach(alias => componentIndexByAlias.set(alias, componentIndex));
  });
  const connectionsByComponent = components.map(
    () => [] as MinigraphGraphData['connections'],
  );
  for (const connection of connections) {
    const sourceComponent = componentIndexByAlias.get(connection.source);
    const targetComponent = componentIndexByAlias.get(connection.target);
    if (
      sourceComponent !== undefined &&
      sourceComponent === targetComponent
    ) {
      connectionsByComponent[sourceComponent].push(connection);
    }
  }

  // ── Step 4: BFS layout for each component, placed left-to-right ───────────
  const levelOf = new Map<string, number>();
  const positions = new Map<string, { x: number; y: number }>();
  let componentLevelOffset = 0;
  let componentXOffset = 0;

  for (const [componentIndex, component] of components.entries()) {
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
    let queueIndex = 0;
    while (queueIndex < queue.length) {
      const current = queue[queueIndex++];
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

    const orderedLayers = minimizeCrossings(
      localLevelOf,
      connectionsByComponent[componentIndex],
      backEdges,
      nodeHeights,
    );

    // Assign pixel positions for this component's flow — centred at y = 0 per
    // local column. Virtual slots reserve candidate clearance for edges that
    // span intermediate levels; actual Bezier/body scoring rejects collisions.
    // componentXOffset gives each independent tree a fixed pixel gap from the
    // previous tree instead of tying tree spacing to column count.
    let componentMaxX = componentXOffset;
    for (const [localLevel, slots] of [...orderedLayers].sort(([a], [b]) => a - b)) {
      const totalHeight = slots.reduce(
        (sum, slot) => sum + slot.height,
        0,
      ) + Math.max(0, slots.length - 1) * ROW_GAP;

      let cursorY = -totalHeight / 2;
      const globalLevel = componentLevelOffset + localLevel;
      const x = componentXOffset + localLevel * (NODE_WIDTH + COL_GAP);
      componentMaxX = Math.max(componentMaxX, x);
      for (const slot of slots) {
        if (slot.alias) {
          levelOf.set(slot.alias, globalLevel);
          positions.set(slot.alias, {
            x,
            y: cursorY,
          });
        }
        cursorY += slot.height + ROW_GAP;
      }
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
  options: { supportsConnectionAuthoring?: boolean } = {},
): { nodes: Node<GraphNodeData>[]; edges: Edge<GraphEdgeData>[] } {
  const connections = data.connections ?? [];
  const supportsConnectionAuthoring = options.supportsConnectionAuthoring === true;

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

  interface SideEntry {
    connIndex: number;
    peerAlias: string;
    isBack: boolean;
    stableKey: string;
  }

  const rightSide = new Map<string, SideEntry[]>(); // source (fwd out) + back-target (back in)
  const leftSide  = new Map<string, SideEntry[]>(); // target (fwd in)  + back-source (back out)

  for (const n of data.nodes) {
    rightSide.set(n.alias, []);
    leftSide.set(n.alias, []);
  }

  for (const [i, conn] of connections.entries()) {
    const stableKey = connectionStableKey(conn);
    if (backEdgeIndices.has(i)) {
      // Back-edge: source exits LEFT, target enters RIGHT
      leftSide.get(conn.source)!.push({
        connIndex: i,
        peerAlias: conn.target,
        isBack: true,
        stableKey,
      });
      rightSide.get(conn.target)!.push({
        connIndex: i,
        peerAlias: conn.source,
        isBack: true,
        stableKey,
      });
    } else {
      // Forward: source exits RIGHT, target enters LEFT
      rightSide.get(conn.source)!.push({
        connIndex: i,
        peerAlias: conn.target,
        isBack: false,
        stableKey,
      });
      leftSide.get(conn.target)!.push({
        connIndex: i,
        peerAlias: conn.source,
        isBack: false,
        stableKey,
      });
    }
  }

  // Sort each side by peer y-position so handle order matches spatial layout.
  const peerY = (alias: string) => positions.get(alias)?.y ?? 0;
  const compareSideEntries = (a: SideEntry, b: SideEntry) =>
    peerY(a.peerAlias) - peerY(b.peerAlias) ||
    a.peerAlias.localeCompare(b.peerAlias) ||
    a.stableKey.localeCompare(b.stableKey) ||
    a.connIndex - b.connIndex;
  for (const entries of rightSide.values()) entries.sort(compareSideEntries);
  for (const entries of leftSide.values())  entries.sort(compareSideEntries);

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
      // React Flow otherwise consumes selection-key pointer events on nodes as
      // the start of a pane selection box. This class keeps modifier-click
      // node toggling available while Shift+drag from empty canvas box-selects.
      className: 'nokey',
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
        supportsConnectionAuthoring,
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
