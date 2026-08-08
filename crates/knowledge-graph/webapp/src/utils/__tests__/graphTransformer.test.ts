import { MarkerType, type Edge, type Node } from '@xyflow/react';
import { describe, expect, it } from 'vitest';
import tutorial3Fixture from '../../../../resources/graph/tutorial-3.json';
import tutorial4Fixture from '../../../../resources/graph/tutorial-4.json';
import tutorial5Fixture from '../../../../resources/graph/tutorial-5.json';
import tutorial6Fixture from '../../../../resources/graph/tutorial-6.json';
import tutorial9Fixture from '../../../../resources/graph/tutorial-9.json';
import tutorial12Fixture from '../../../../resources/graph/tutorial-12.json';
import helloFixture from '../../../../tests/resources/graph/hello.json';
import task4Fixture from '../../../../tests/resources/graph/unit-test-task-4.json';
import {
  transformGraphData,
  type GraphEdgeData,
  type GraphHandleData,
  type GraphNodeData,
} from '../graphTransformer';
import type { MinigraphGraphData } from '../graphTypes';

interface Point {
  x: number;
  y: number;
}

interface Endpoint extends Point {
  side: 'left' | 'right';
}

interface GeometryScore {
  edgeCrossings: number;
  nodeIntrusions: number;
}

const tutorial3 = tutorial3Fixture as MinigraphGraphData;
const tutorial4 = tutorial4Fixture as MinigraphGraphData;
const planarTutorials: Array<[string, MinigraphGraphData]> = [
  ['tutorial-5', tutorial5Fixture as MinigraphGraphData],
  ['tutorial-6', tutorial6Fixture as MinigraphGraphData],
  ['tutorial-9', tutorial9Fixture as MinigraphGraphData],
  ['tutorial-12', tutorial12Fixture as MinigraphGraphData],
];
const EPSILON = 1e-7;
const CURVE_FLATNESS_TOLERANCE = 0.25;
const MAX_CURVE_DEPTH = 14;

function nodeWidth(node: Node<GraphNodeData>): number {
  return node.width ?? 240;
}

function nodeHeight(node: Node<GraphNodeData>): number {
  return node.height ?? node.data.minHeight;
}

function findHandle(
  handles: GraphHandleData[],
  handleId: string,
): GraphHandleData | undefined {
  return handles.find(handle => handle.id === handleId);
}

function resolveEndpoint(
  node: Node<GraphNodeData>,
  handleId: string,
  endpointType: 'source' | 'target',
): Endpoint {
  const primaryHandles = endpointType === 'source'
    ? node.data.sourceHandles
    : node.data.targetHandles;
  const backHandles = endpointType === 'source'
    ? node.data.backSourceHandles
    : node.data.backTargetHandles;
  const primary = findHandle(primaryHandles, handleId);
  const back = findHandle(backHandles, handleId);
  const handle = primary ?? back;

  if (!handle) {
    throw new Error(`Missing ${endpointType} handle ${handleId} on ${node.id}`);
  }

  const side = endpointType === 'source'
    ? (primary ? 'right' : 'left')
    : (primary ? 'left' : 'right');

  return {
    x: node.position.x + (side === 'right' ? nodeWidth(node) : 0),
    y: node.position.y + (nodeHeight(node) / 2) + handle.offset,
    side,
  };
}

function controlOffset(distance: number): number {
  return distance >= 0
    ? 0.5 * distance
    : 0.25 * 25 * Math.sqrt(-distance);
}

function controlPoint(point: Endpoint, other: Endpoint): Point {
  if (point.side === 'left') {
    return {
      x: point.x - controlOffset(point.x - other.x),
      y: point.y,
    };
  }
  return {
    x: point.x + controlOffset(other.x - point.x),
    y: point.y,
  };
}

function midpoint(first: Point, second: Point): Point {
  return {
    x: (first.x + second.x) / 2,
    y: (first.y + second.y) / 2,
  };
}

function distanceToLine(point: Point, start: Point, end: Point): number {
  const dx = end.x - start.x;
  const dy = end.y - start.y;
  const length = Math.hypot(dx, dy);
  if (length <= EPSILON) return Math.hypot(point.x - start.x, point.y - start.y);
  return Math.abs(((point.x - start.x) * dy) - ((point.y - start.y) * dx)) / length;
}

function flattenCubic(
  start: Point,
  startControl: Point,
  endControl: Point,
  end: Point,
  output: Point[],
  depth = 0,
): void {
  const flatness = Math.max(
    distanceToLine(startControl, start, end),
    distanceToLine(endControl, start, end),
  );
  if (flatness <= CURVE_FLATNESS_TOLERANCE || depth >= MAX_CURVE_DEPTH) {
    output.push(end);
    return;
  }

  const startHalf = midpoint(start, startControl);
  const controlHalf = midpoint(startControl, endControl);
  const endHalf = midpoint(endControl, end);
  const firstControl = midpoint(startHalf, controlHalf);
  const secondControl = midpoint(controlHalf, endHalf);
  const curveHalf = midpoint(firstControl, secondControl);
  flattenCubic(start, startHalf, firstControl, curveHalf, output, depth + 1);
  flattenCubic(curveHalf, secondControl, endHalf, end, output, depth + 1);
}

function sampleEdge(
  edge: Edge<GraphEdgeData>,
  nodesById: Map<string, Node<GraphNodeData>>,
): Point[] {
  const sourceNode = nodesById.get(edge.source);
  const targetNode = nodesById.get(edge.target);
  if (!sourceNode || !targetNode) {
    throw new Error(`Missing endpoint node for ${edge.id}`);
  }
  if (!edge.sourceHandle || !edge.targetHandle) {
    throw new Error(`Missing endpoint handle for ${edge.id}`);
  }

  const source = resolveEndpoint(sourceNode, edge.sourceHandle, 'source');
  const target = resolveEndpoint(targetNode, edge.targetHandle, 'target');
  const sourceControl = controlPoint(source, target);
  const targetControl = controlPoint(target, source);
  const points: Point[] = [source];
  flattenCubic(source, sourceControl, targetControl, target, points);
  return points;
}

function crossProduct(a: Point, b: Point, c: Point): number {
  return ((b.x - a.x) * (c.y - a.y)) - ((b.y - a.y) * (c.x - a.x));
}

function pointOnSegment(point: Point, start: Point, end: Point): boolean {
  return point.x >= Math.min(start.x, end.x) - EPSILON &&
    point.x <= Math.max(start.x, end.x) + EPSILON &&
    point.y >= Math.min(start.y, end.y) - EPSILON &&
    point.y <= Math.max(start.y, end.y) + EPSILON;
}

function segmentsIntersect(a: Point, b: Point, c: Point, d: Point): boolean {
  const abc = crossProduct(a, b, c);
  const abd = crossProduct(a, b, d);
  const cda = crossProduct(c, d, a);
  const cdb = crossProduct(c, d, b);
  const firstStraddles = (abc > EPSILON && abd < -EPSILON) ||
    (abc < -EPSILON && abd > EPSILON);
  const secondStraddles = (cda > EPSILON && cdb < -EPSILON) ||
    (cda < -EPSILON && cdb > EPSILON);
  if (firstStraddles && secondStraddles) return true;

  return (Math.abs(abc) <= EPSILON && pointOnSegment(c, a, b)) ||
    (Math.abs(abd) <= EPSILON && pointOnSegment(d, a, b)) ||
    (Math.abs(cda) <= EPSILON && pointOnSegment(a, c, d)) ||
    (Math.abs(cdb) <= EPSILON && pointOnSegment(b, c, d));
}

function curvesCross(first: Point[], second: Point[]): boolean {
  for (let i = 1; i < first.length; i++) {
    for (let j = 1; j < second.length; j++) {
      if (segmentsIntersect(first[i - 1], first[i], second[j - 1], second[j])) {
        return true;
      }
    }
  }
  return false;
}

function pointInsideNode(point: Point, node: Node<GraphNodeData>): boolean {
  const left = node.position.x;
  const right = left + nodeWidth(node);
  const top = node.position.y;
  const bottom = top + nodeHeight(node);
  return point.x > left + EPSILON &&
    point.x < right - EPSILON &&
    point.y > top + EPSILON &&
    point.y < bottom - EPSILON;
}

function segmentIntersectsNode(
  start: Point,
  end: Point,
  node: Node<GraphNodeData>,
): boolean {
  if (pointInsideNode(start, node) || pointInsideNode(end, node)) return true;
  const left = node.position.x + EPSILON;
  const right = node.position.x + nodeWidth(node) - EPSILON;
  const top = node.position.y + EPSILON;
  const bottom = node.position.y + nodeHeight(node) - EPSILON;
  const topLeft = { x: left, y: top };
  const topRight = { x: right, y: top };
  const bottomRight = { x: right, y: bottom };
  const bottomLeft = { x: left, y: bottom };
  return segmentsIntersect(start, end, topLeft, topRight) ||
    segmentsIntersect(start, end, topRight, bottomRight) ||
    segmentsIntersect(start, end, bottomRight, bottomLeft) ||
    segmentsIntersect(start, end, bottomLeft, topLeft);
}

function curveIntersectsNode(
  points: Point[],
  node: Node<GraphNodeData>,
): boolean {
  for (let index = 1; index < points.length; index++) {
    if (segmentIntersectsNode(points[index - 1], points[index], node)) return true;
  }
  return false;
}

function scoreGeometry(
  nodes: Node<GraphNodeData>[],
  edges: Edge<GraphEdgeData>[],
): GeometryScore {
  const nodesById = new Map(nodes.map(node => [node.id, node]));
  const samples = new Map(edges.map(edge => [edge.id, sampleEdge(edge, nodesById)]));
  let edgeCrossings = 0;
  let nodeIntrusions = 0;

  for (let i = 0; i < edges.length; i++) {
    const first = edges[i];
    for (let j = i + 1; j < edges.length; j++) {
      const second = edges[j];
      const sharesEndpoint = first.source === second.source ||
        first.source === second.target ||
        first.target === second.source ||
        first.target === second.target;
      if (sharesEndpoint) continue;
      if (curvesCross(samples.get(first.id)!, samples.get(second.id)!)) {
        edgeCrossings += 1;
      }
    }

    for (const node of nodes) {
      if (node.id === first.source || node.id === first.target) continue;
      if (curveIntersectsNode(samples.get(first.id)!, node)) {
        nodeIntrusions += 1;
      }
    }
  }

  return { edgeCrossings, nodeIntrusions };
}

function makeGraph(
  aliases: string[],
  endpointPairs: Array<readonly [string, string]>,
): MinigraphGraphData {
  return {
    nodes: aliases.map(alias => ({
      alias,
      types: [alias === 'root' ? 'Root' : 'Task'],
      properties: {},
    })),
    connections: endpointPairs.map(([source, target], index) => ({
      source,
      target,
      relations: [{
        type: `relation-${index}`,
        properties: {},
      }],
    })),
  };
}

function positionsByAlias(nodes: Node<GraphNodeData>[]): Record<string, Point> {
  return Object.fromEntries(
    nodes
      .map(node => [node.id, node.position] as const)
      .sort(([a], [b]) => a.localeCompare(b)),
  );
}

function expectGraphSemanticsPreserved(
  graph: MinigraphGraphData,
  nodes: Node<GraphNodeData>[],
  edges: Edge<GraphEdgeData>[],
): void {
  expect(nodes.map(node => node.id).sort()).toEqual(graph.nodes.map(node => node.alias).sort());
  expect(edges).toHaveLength(graph.connections.length);
  for (const [index, connection] of graph.connections.entries()) {
    const edge = edges[index];
    expect(edge.source).toBe(connection.source);
    expect(edge.target).toBe(connection.target);
    expect(edge.data?.relationTypes).toEqual(connection.relations.map(relation => relation.type));
    expect(edge.label).toBe(connection.relations.map(relation => relation.type).join(', '));
    expect(edge.type).toBe('bezier');
    expect(edge.markerEnd).toMatchObject({ type: MarkerType.ArrowClosed });
    expect(edge.sourceHandle).toBeTypeOf('string');
    expect(edge.targetHandle).toBeTypeOf('string');
  }
}

describe('transformGraphData crossing minimization', () => {
  it('removes the avoidable fork/join crossings in tutorial-3', () => {
    const result = transformGraphData(tutorial3);

    expect(scoreGeometry(result.nodes, result.edges)).toEqual({
      edgeCrossings: 0,
      nodeIntrusions: 0,
    });
    expectGraphSemanticsPreserved(tutorial3, result.nodes, result.edges);
  });

  it('reserves a long-edge corridor around the decision node in tutorial-4', () => {
    const result = transformGraphData(tutorial4);

    expect(scoreGeometry(result.nodes, result.edges)).toEqual({
      edgeCrossings: 0,
      nodeIntrusions: 0,
    });
    expectGraphSemanticsPreserved(tutorial4, result.nodes, result.edges);
  });

  it.each(planarTutorials)('removes avoidable crossings from %s', (_name, graph) => {
    const result = transformGraphData(graph);

    expect(scoreGeometry(result.nodes, result.edges)).toEqual({
      edgeCrossings: 0,
      nodeIntrusions: 0,
    });
    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
  });

  it.each([
    ['hello', helloFixture as MinigraphGraphData],
    ['unit-test-task-4', task4Fixture as MinigraphGraphData],
  ])('keeps long skip-edges out of non-incident nodes in %s', (_name, graph) => {
    const result = transformGraphData(graph);

    expect(scoreGeometry(result.nodes, result.edges).nodeIntrusions).toBe(0);
    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
  });

  it('reorders a reversed two-layer matching without changing its semantics', () => {
    const graph = makeGraph(
      ['root', 'alpha', 'beta', 'charlie', 'delta'],
      [
        ['root', 'alpha'],
        ['root', 'beta'],
        ['alpha', 'delta'],
        ['beta', 'charlie'],
      ],
    );
    const result = transformGraphData(graph);

    expect(scoreGeometry(result.nodes, result.edges)).toEqual({
      edgeCrossings: 0,
      nodeIntrusions: 0,
    });
    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
  });

  it('uses local transposition to escape a barycentric local minimum', () => {
    const graph = makeGraph(
      ['root', 'b', 'c', 'd', 'e', 'f'],
      [
        ['root', 'd'],
        ['b', 'd'],
        ['b', 'e'],
        ['c', 'e'],
        ['c', 'f'],
      ],
    );
    const result = transformGraphData(graph);

    expect(scoreGeometry(result.nodes, result.edges)).toEqual({
      edgeCrossings: 0,
      nodeIntrusions: 0,
    });
    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
  });

  it.each([
    [
      'short skip edge',
      makeGraph(
        ['root', 'a', 'b', 'c', 'd'],
        [
          ['root', 'a'],
          ['root', 'b'],
          ['root', 'd'],
          ['a', 'b'],
          ['b', 'c'],
        ],
      ),
    ],
    [
      'multi-rank skip edge',
      makeGraph(
        ['root', 'b', 'c', 'd', 'e', 'f'],
        [
          ['root', 'b'],
          ['root', 'c'],
          ['root', 'd'],
          ['root', 'e'],
          ['b', 'c'],
          ['c', 'd'],
          ['e', 'f'],
        ],
      ),
    ],
  ])('keeps actual Bezier geometry out of nodes for a %s', (_name, graph) => {
    const result = transformGraphData(graph);

    expect(scoreGeometry(result.nodes, result.edges).nodeIntrusions).toBe(0);
    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
  });

  it('does not claim an impossible zero-crossing layout for a K3,3 graph', () => {
    const left = ['left-a', 'left-b', 'left-c'];
    const right = ['right-a', 'right-b', 'right-c'];
    const graph = makeGraph(
      [...left, ...right],
      left.flatMap(source => right.map(target => [source, target] as const)),
    );
    const result = transformGraphData(graph);

    const score = scoreGeometry(result.nodes, result.edges);
    expect(score.edgeCrossings).toBeGreaterThan(0);
    expect(score.nodeIntrusions).toBe(0);
    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
  });

  it('preserves cycles and assigns their back-edge handles', () => {
    const graph = makeGraph(
      ['alpha', 'beta', 'gamma'],
      [['alpha', 'beta'], ['beta', 'gamma'], ['gamma', 'alpha']],
    );
    const result = transformGraphData(graph);
    const reordered = transformGraphData({
      nodes: graph.nodes.slice().reverse(),
      connections: graph.connections.slice().reverse(),
    });
    const backEdge = result.edges.find(edge => edge.source === 'gamma' && edge.target === 'alpha');
    const backEdgePairs = (edges: Edge<GraphEdgeData>[]) => edges
      .filter(edge => edge.sourceHandle?.startsWith('back-source-'))
      .map(edge => `${edge.source}->${edge.target}`)
      .sort();

    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
    expect(backEdge?.sourceHandle).toMatch(/^back-source-/);
    expect(backEdge?.targetHandle).toMatch(/^back-target-/);
    expect(positionsByAlias(reordered.nodes)).toEqual(positionsByAlias(result.nodes));
    expect(backEdgePairs(reordered.edges)).toEqual(backEdgePairs(result.edges));
    for (const node of result.nodes) {
      expect(Number.isFinite(node.position.x)).toBe(true);
      expect(Number.isFinite(node.position.y)).toBe(true);
    }
  });

  it('keeps positions stable for equivalent reordered payloads', () => {
    const graph = makeGraph(
      ['root', 'upper', 'lower', 'upper-end', 'lower-end', 'end'],
      [
        ['root', 'upper'],
        ['root', 'lower'],
        ['upper', 'upper-end'],
        ['lower', 'lower-end'],
        ['upper-end', 'end'],
        ['lower-end', 'end'],
      ],
    );
    const reordered: MinigraphGraphData = {
      nodes: graph.nodes.slice().reverse(),
      connections: graph.connections.slice().reverse(),
    };

    const first = transformGraphData(graph);
    const repeated = transformGraphData(graph);
    const shuffled = transformGraphData(reordered);

    expect(positionsByAlias(repeated.nodes)).toEqual(positionsByAlias(first.nodes));
    expect(positionsByAlias(shuffled.nodes)).toEqual(positionsByAlias(first.nodes));
    expect(scoreGeometry(shuffled.nodes, shuffled.edges)).toEqual(
      scoreGeometry(first.nodes, first.edges),
    );
  });

  it('bounds virtual-slot work for an adversarial chain with many skip edges', () => {
    const aliases = [
      'root',
      ...Array.from({ length: 299 }, (_, index) => `node-${String(index + 1).padStart(3, '0')}`),
    ];
    const chain = aliases.slice(0, -1).map(
      (alias, index) => [alias, aliases[index + 1]] as const,
    );
    const skips = aliases.slice(2).map(alias => ['root', alias] as const);
    const graph = makeGraph(aliases, [...chain, ...skips]);
    const result = transformGraphData(graph);

    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
    expect(
      Math.max(
        ...result.nodes
          .filter(node => node.id !== 'root')
          .map(node => Math.abs(node.position.y)),
      ),
    ).toBeLessThanOrEqual(50);
  });

  it('preserves parallel edges with distinct rendered handles', () => {
    const graph = makeGraph(
      ['root', 'end'],
      [['root', 'end'], ['root', 'end']],
    );
    const result = transformGraphData(graph, { supportsConnectionAuthoring: true });

    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
    expect(new Set(result.edges.map(edge => edge.sourceHandle)).size).toBe(2);
    expect(new Set(result.edges.map(edge => edge.targetHandle)).size).toBe(2);
    expect(result.nodes.every(node => node.data.supportsConnectionAuthoring)).toBe(true);
  });

  it('retains component ordering and keeps segregated orphans below the flow', () => {
    const graph: MinigraphGraphData = {
      nodes: [
        { alias: 'root', types: ['Root'], properties: {} },
        { alias: 'root-task', types: ['Task'], properties: {} },
        { alias: 'ordinary-a', types: ['Task'], properties: {} },
        { alias: 'ordinary-b', types: ['Task'], properties: {} },
        { alias: 'end-source', types: ['Task'], properties: {} },
        { alias: 'end', types: ['End'], properties: {} },
        { alias: 'dictionary-orphan', types: ['Dictionary'], properties: {} },
      ],
      connections: [
        { source: 'root', target: 'root-task', relations: [] },
        { source: 'ordinary-a', target: 'ordinary-b', relations: [] },
        { source: 'end-source', target: 'end', relations: [] },
      ],
    };
    const result = transformGraphData(graph, { supportsConnectionAuthoring: true });
    const nodesById = new Map(result.nodes.map(node => [node.id, node]));
    const mainBottom = Math.max(
      ...result.nodes
        .filter(node => node.id !== 'dictionary-orphan')
        .map(node => node.position.y + nodeHeight(node)),
    );

    expect(nodesById.get('root')!.position.x).toBeLessThan(nodesById.get('ordinary-a')!.position.x);
    expect(nodesById.get('ordinary-a')!.position.x).toBeLessThan(nodesById.get('end-source')!.position.x);
    expect(nodesById.get('dictionary-orphan')!.position.y).toBeGreaterThan(mainBottom);
    expect(result.nodes.every(node => node.data.supportsConnectionAuthoring)).toBe(true);
    expectGraphSemanticsPreserved(graph, result.nodes, result.edges);
  });
});
