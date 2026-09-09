// @vitest-environment happy-dom

import {
  useState,
  type ButtonHTMLAttributes,
  type CSSProperties,
  type ReactNode,
} from 'react';
import { cleanup, fireEvent, render, screen, within } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import GraphMinimap from '../GraphMinimap';

const miniMapRender = vi.hoisted(() => vi.fn());

vi.mock('@xyflow/react', () => ({
  MiniMap: (props: Record<string, unknown>) => {
    miniMapRender(props);
    return <div data-testid="graph-minimap" />;
  },
  Panel: ({ children, position, style }: {
    children: ReactNode;
    position?: string;
    style?: CSSProperties;
  }) => (
    <div
      data-testid="minimap-toggle-panel"
      data-position={position}
      data-bottom={style?.bottom}
    >
      {children}
    </div>
  ),
  Controls: ({
    children,
    className,
    position = 'bottom-left',
    showInteractive = true,
  }: {
    children?: ReactNode;
    className?: string;
    position?: string;
    showInteractive?: boolean;
  }) => (
    <div
      className={`react-flow__controls ${className ?? ''}`}
      data-testid="rf-controls"
      data-position={position}
      data-show-interactive={showInteractive}
    >
      <button type="button" className="react-flow__controls-button" aria-label="Zoom in" />
      <button type="button" className="react-flow__controls-button" aria-label="Zoom out" />
      <button type="button" className="react-flow__controls-button" aria-label="Fit view" />
      {showInteractive && (
        <button
          type="button"
          className="react-flow__controls-button"
          aria-label="Toggle interactivity"
        />
      )}
      {children}
    </div>
  ),
  ControlButton: ({ children, className, ...props }: ButtonHTMLAttributes<HTMLButtonElement>) => (
    <button
      type="button"
      className={`react-flow__controls-button ${className ?? ''}`}
      {...props}
    >
      {children}
    </button>
  ),
}));

function GraphMinimapHarness({
  initialOpen = false,
  hotkeyEnabled = true,
}: {
  initialOpen?: boolean;
  hotkeyEnabled?: boolean;
}) {
  const [open, setOpen] = useState(initialOpen);
  return (
    <GraphMinimap
      open={open}
      onOpenChange={setOpen}
      hotkeyEnabled={hotkeyEnabled}
    />
  );
}

function EditableTargetHarness() {
  return (
    <>
      <input aria-label="Graph name" />
      <GraphMinimapHarness />
    </>
  );
}

function KeyedBoundaryHarness({ boundaryKey }: { boundaryKey: string }) {
  const [open, setOpen] = useState(true);
  return (
    <div key={boundaryKey}>
      <GraphMinimap
        open={open}
        onOpenChange={setOpen}
        hotkeyEnabled
      />
    </div>
  );
}

describe('GraphMinimap', () => {
  beforeEach(() => {
    miniMapRender.mockClear();
    localStorage.clear();
  });
  afterEach(cleanup);

  it('starts collapsed with the minimap as the bottom button in one native left control group', () => {
    render(<GraphMinimapHarness />);

    const showButton = screen.getByRole('button', { name: 'Show minimap' });
    const controls = screen.getByTestId('rf-controls');
    expect(controls.getAttribute('data-position')).toBe('bottom-left');
    expect(controls.getAttribute('data-show-interactive')).toBe('false');
    expect(within(controls).getAllByRole('button').map((button) => button.getAttribute('aria-label')))
      .toEqual(['Zoom in', 'Zoom out', 'Fit view', 'Show minimap']);
    expect(Array.from(controls.children).filter((child) => (
      child.matches('button.react-flow__controls-button')
    ))).toHaveLength(4);
    expect(controls.lastElementChild).toBe(showButton);
    expect(showButton.className).toContain('react-flow__controls-button');
    expect(showButton.className).toContain('nodrag');
    expect(showButton.className).toContain('nopan');
    expect(showButton.getAttribute('aria-pressed')).toBe('false');
    expect(showButton.getAttribute('aria-keyshortcuts')).toBe('Control+M');
    expect(showButton.getAttribute('title')).toBe('Show minimap (Ctrl + M)');
    expect(showButton.querySelector('svg')?.getAttribute('viewBox')).toBe('1.5 2 17 16');
    expect(screen.queryByTestId('graph-minimap')).toBeNull();

    fireEvent.click(showButton);

    const hideButton = screen.getByRole('button', { name: 'Hide minimap' });
    expect(hideButton.getAttribute('aria-pressed')).toBe('true');
    expect(screen.getByTestId('graph-minimap')).toBeTruthy();

    fireEvent.click(hideButton);

    expect(screen.getByRole('button', { name: 'Show minimap' })).toBeTruthy();
    expect(screen.queryByTestId('graph-minimap')).toBeNull();
  });

  it('toggles the minimap with the Ctrl + M hotkey', () => {
    render(<GraphMinimapHarness />);

    fireEvent.keyDown(window, { key: 'm', code: 'KeyM', ctrlKey: true });

    expect(screen.getByRole('button', { name: 'Hide minimap' })).toBeTruthy();
    expect(screen.getByTestId('graph-minimap')).toBeTruthy();

    fireEvent.keyDown(window, { key: 'M', code: 'KeyM', ctrlKey: true });

    expect(screen.getByRole('button', { name: 'Show minimap' })).toBeTruthy();
    expect(screen.queryByTestId('graph-minimap')).toBeNull();
  });

  it('does not intercept typing, other shortcuts, repeats, or prevented events', () => {
    render(<EditableTargetHarness />);

    fireEvent.keyDown(screen.getByRole('textbox', { name: 'Graph name' }), {
      key: 'm',
      code: 'KeyM',
      ctrlKey: true,
    });
    fireEvent.keyDown(window, { key: 'm', code: 'KeyM' });
    fireEvent.keyDown(window, { key: 'm', code: 'KeyM', altKey: true });
    fireEvent.keyDown(window, { key: 'm', code: 'KeyM', ctrlKey: true, metaKey: true });
    fireEvent.keyDown(window, { key: 'm', code: 'KeyM', ctrlKey: true, altKey: true });
    fireEvent.keyDown(window, { key: 'm', code: 'KeyM', ctrlKey: true, shiftKey: true });
    fireEvent.keyDown(window, { key: 'm', code: 'KeyM', ctrlKey: true, repeat: true });

    const preventedEvent = new KeyboardEvent('keydown', {
      key: 'm',
      code: 'KeyM',
      ctrlKey: true,
      bubbles: true,
      cancelable: true,
    });
    preventedEvent.preventDefault();
    fireEvent(window, preventedEvent);

    expect(screen.getByRole('button', { name: 'Show minimap' })).toBeTruthy();
    expect(screen.queryByTestId('graph-minimap')).toBeNull();
  });

  it('does not toggle while the graph tab is inactive', () => {
    render(<GraphMinimapHarness hotkeyEnabled={false} />);

    fireEvent.keyDown(window, { key: 'm', code: 'KeyM', ctrlKey: true });

    expect(screen.getByRole('button', { name: 'Show minimap' })).toBeTruthy();
    expect(screen.queryByTestId('graph-minimap')).toBeNull();
  });

  it('renders the open minimap as a draggable island with viewport panning enabled', () => {
    render(<GraphMinimapHarness />);
    fireEvent.click(screen.getByRole('button', { name: 'Show minimap' }));

    // The island floats at its default anchor beside the control stack.
    const island = screen.getByRole('group', { name: 'Graph minimap' });
    expect(island.style.left).toBe('50px');
    expect(island.style.bottom).toBe('15px');
    expect(within(island).getByRole('button', { name: 'Move minimap' })).toBeTruthy();
    expect(within(island).getByTestId('graph-minimap')).toBeTruthy();

    // The map itself keeps React Flow's pannable behavior and sheds the
    // panel positioning (the island wrapper owns placement).
    const lastRender = miniMapRender.mock.calls[miniMapRender.mock.calls.length - 1];
    const props = lastRender[0] as {
      maskColor?: string;
      nodeColor?: (node: { type?: string }) => string;
      pannable?: boolean;
      style?: Record<string, unknown>;
      position?: string;
      zoomable?: boolean;
    };

    expect(props.maskColor).toBe('rgba(0,0,0,0.3)');
    expect(props.style).toEqual({ position: 'relative', margin: 0, background: '#fff' });
    expect(props.position).toBeUndefined();
    expect(props.pannable).toBe(true);
    expect(props.zoomable).toBeUndefined();
    expect({
      Root: props.nodeColor?.({ type: 'Root' }),
      End: props.nodeColor?.({ type: 'End' }),
      Fetcher: props.nodeColor?.({ type: 'Fetcher' }),
      mapper: props.nodeColor?.({ type: 'mapper' }),
      Math: props.nodeColor?.({ type: 'Math' }),
      JavaScript: props.nodeColor?.({ type: 'JavaScript' }),
      Provider: props.nodeColor?.({ type: 'Provider' }),
      Dictionary: props.nodeColor?.({ type: 'Dictionary' }),
      Join: props.nodeColor?.({ type: 'Join' }),
      Extension: props.nodeColor?.({ type: 'Extension' }),
      Island: props.nodeColor?.({ type: 'Island' }),
      Decision: props.nodeColor?.({ type: 'Decision' }),
      fallback: props.nodeColor?.({ type: 'Unknown' }),
    }).toEqual({
      Root: '#15803d',
      End: '#dc2626',
      Fetcher: '#2563eb',
      mapper: '#ea580c',
      Math: '#a16207',
      JavaScript: '#7e22ce',
      Provider: '#be185d',
      Dictionary: '#0e7490',
      Join: '#65a30d',
      Extension: '#4338ca',
      Island: '#475569',
      Decision: '#b45309',
      fallback: '#6c7086',
    });
  });

  it('moves the island by dragging its grip and persists the position', () => {
    render(<GraphMinimapHarness initialOpen />);

    const grip = screen.getByRole('button', { name: 'Move minimap' });
    fireEvent.pointerDown(grip, { pointerId: 1, clientX: 100, clientY: 200, button: 0 });
    fireEvent.pointerMove(window, { pointerId: 1, clientX: 130, clientY: 180 });

    const island = screen.getByRole('group', { name: 'Graph minimap' });
    // +30px right, 20px up (bottom-anchored, so bottom grows).
    expect(island.style.left).toBe('80px');
    expect(island.style.bottom).toBe('35px');

    fireEvent.pointerUp(window, { pointerId: 1, clientX: 130, clientY: 180 });

    expect(JSON.parse(localStorage.getItem('graph-minimap-position') ?? 'null'))
      .toEqual({ left: 80, bottom: 35 });

    // Further movement after release must not drag the island.
    fireEvent.pointerMove(window, { pointerId: 1, clientX: 300, clientY: 50 });
    expect(island.style.left).toBe('80px');
    expect(island.style.bottom).toBe('35px');
  });

  it('reopens the island at its persisted position', () => {
    localStorage.setItem('graph-minimap-position', JSON.stringify({ left: 220, bottom: 90 }));
    render(<GraphMinimapHarness initialOpen />);

    const island = screen.getByRole('group', { name: 'Graph minimap' });
    expect(island.style.left).toBe('220px');
    expect(island.style.bottom).toBe('90px');
  });

  it('keeps an open minimap visible when its keyed canvas boundary remounts', () => {
    const { rerender } = render(<KeyedBoundaryHarness boundaryKey="root" />);

    expect(screen.getByRole('button', { name: 'Hide minimap' })).toBeTruthy();
    expect(screen.getByTestId('graph-minimap')).toBeTruthy();

    rerender(<KeyedBoundaryHarness boundaryKey="root,end" />);

    expect(screen.getByRole('button', { name: 'Hide minimap' })).toBeTruthy();
    expect(screen.getByTestId('graph-minimap')).toBeTruthy();
  });
});
