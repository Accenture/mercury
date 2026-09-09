// @vitest-environment happy-dom

import { type CSSProperties, type ReactNode } from 'react';
import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import RightPanel from '../RightPanel';
import type { RightTab } from '../RightPanel';

const graphViewRender = vi.hoisted(() => vi.fn());

vi.mock('../../GraphView/GraphView', () => ({
  default: (props: Record<string, unknown>) => {
    graphViewRender(props);
    return <div data-testid="graph-view" />;
  },
}));

vi.mock('../../PayloadEditor/PayloadEditor', () => ({
  default: () => <div />,
}));

vi.mock('../../GraphDataView/GraphDataView', () => ({
  default: () => <div />,
}));

vi.mock('react-resizable-panels', () => ({
  Group: ({ children }: { children: ReactNode }) => <div>{children}</div>,
  Panel: ({ children, style }: { children: ReactNode; style?: CSSProperties }) => (
    <div style={style}>{children}</div>
  ),
  Separator: () => <div />,
}));

function renderRightPanel({
  tabs = ['graph'] as RightTab[],
  activeTab = 'graph' as RightTab,
} = {}) {
  const graphRunControls = {
    phase: 'idle' as const,
    canInstantiate: true,
    canRun: false,
    disabledReason: '',
    onInstantiate: vi.fn(),
    onRun: vi.fn(),
  };
  return render(
    <RightPanel
      tabs={tabs}
      payload=""
      onChange={() => {}}
      validation={{ valid: true, error: null, type: null }}
      onFormat={() => {}}
      graphData={{ nodes: [], connections: [] }}
      activeTab={activeTab}
      onTabChange={() => {}}
      isConnected
      graphRunControls={graphRunControls}
    />
  );
}

describe('RightPanel tab strip', () => {
  beforeEach(() => {
    graphViewRender.mockClear();
    sessionStorage.clear();
  });

  afterEach(cleanup);

  it('hides the tab strip entirely for a single-tab playground', () => {
    renderRightPanel({ tabs: ['payload'], activeTab: 'payload' });

    expect(screen.queryByRole('tablist')).toBeNull();
    expect(screen.queryByRole('tab')).toBeNull();
  });

  it('renders the tab strip when the playground declares multiple tabs', () => {
    renderRightPanel({ tabs: ['graph', 'graph-data'], activeTab: 'graph' });

    expect(screen.getByRole('tablist', { name: 'Right panel tabs' })).toBeTruthy();
    expect(screen.getAllByRole('tab').map((tab) => tab.textContent))
      .toEqual(['Graph🕸️', 'Graph Data (Raw)']);
  });

  it('forwards graph-local run controls to GraphView', () => {
    renderRightPanel();

    expect(graphViewRender.mock.lastCall?.[0]).toMatchObject({
      isActive: true,
      graphRunControls: {
        phase: 'idle',
        canInstantiate: true,
        canRun: false,
      },
    });
  });
});
