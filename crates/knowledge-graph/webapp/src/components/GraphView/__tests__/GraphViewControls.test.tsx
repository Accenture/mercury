// @vitest-environment happy-dom

import { type ReactNode } from 'react';
import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import GraphView from '../GraphView';

const flowStateMocks = vi.hoisted(() => ({
  setNodes: vi.fn(),
  onNodesChange: vi.fn(),
  setEdges: vi.fn(),
  onEdgesChange: vi.fn(),
}));

vi.mock('@xyflow/react', () => ({
  ReactFlow: ({ children }: { children: ReactNode }) => <div>{children}</div>,
  Background: () => null,
  Controls: () => <div data-testid="rf-controls" />,
  ControlButton: ({ children }: { children: ReactNode }) => <button>{children}</button>,
  useNodesState: (nodes: unknown[]) => [
    nodes,
    flowStateMocks.setNodes,
    flowStateMocks.onNodesChange,
  ],
  useEdgesState: (edges: unknown[]) => [
    edges,
    flowStateMocks.setEdges,
    flowStateMocks.onEdgesChange,
  ],
  BackgroundVariant: { Dots: 'dots' },
  SelectionMode: { Partial: 'partial' },
}));

vi.mock('../NodeTypes', () => ({
  AUTHORING_SOURCE_HANDLE_ID: 'authoring-source',
  AUTHORING_TARGET_HANDLE_ID: 'authoring-target',
  nodeTypes: {},
}));

vi.mock('../../../utils/graphTransformer', () => ({
  transformGraphData: () => ({
    nodes: [{ id: 'root', type: 'Root', position: { x: 0, y: 0 }, data: {} }],
    edges: [],
  }),
}));

vi.mock('../../GraphToolbar/GraphToolbar', () => ({
  default: ({ extraActions }: { extraActions?: ReactNode }) => <div>{extraActions}</div>,
}));

vi.mock('../GraphContextMenu', () => ({ default: () => null }));
vi.mock('../NodeContextMenu', () => ({ default: () => null }));
vi.mock('../GraphMultiSelectTip', () => ({ default: () => null }));
vi.mock('../GraphMinimap', () => ({
  default: ({ children }: { children?: ReactNode }) => (
    <div data-testid="rf-controls">{children}</div>
  ),
}));

describe('GraphView viewport controls composition', () => {
  afterEach(cleanup);

  it('delegates the only React Flow controls group to GraphMinimap', () => {
    render(
      <GraphView
        graphData={{
          nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
          connections: [],
        }}
        isActive
        isConnected
      />
    );

    expect(screen.getAllByTestId('rf-controls')).toHaveLength(1);
  });

  it('places the graph run controls in the graph toolbar action slot', () => {
    render(
      <GraphView
        graphData={{
          nodes: [{ alias: 'root', types: ['Root'], properties: {} }],
          connections: [],
        }}
        graphRunControls={{
          phase: 'idle',
          canInstantiate: true,
          canRun: false,
          disabledReason: '',
          onInstantiate: vi.fn(),
          onRun: vi.fn(),
        }}
        isActive
        isConnected
      />
    );

    expect(screen.getByRole('group', { name: 'Graph run controls' })).toBeTruthy();
    expect((screen.getByRole('button', { name: 'Instantiate graph' }) as HTMLButtonElement).disabled).toBe(false);
    expect((screen.getByRole('button', { name: 'Run graph' }) as HTMLButtonElement).disabled).toBe(true);
  });
});
