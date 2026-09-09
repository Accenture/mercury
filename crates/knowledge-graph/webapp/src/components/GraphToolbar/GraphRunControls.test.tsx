// @vitest-environment happy-dom
/// <reference types="node" />

import { readFileSync } from 'node:fs';
import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import GraphRunControls from './GraphRunControls';
import GraphToolbar from './GraphToolbar';

const graphToolbarCss = readFileSync(
  'src/components/GraphToolbar/GraphToolbar.module.css',
  'utf8',
);

afterEach(cleanup);

describe('GraphRunControls', () => {
  it('requires Instantiate before enabling the separate Run action', () => {
    const onInstantiate = vi.fn();
    const onRun = vi.fn();
    const { rerender } = render(
      <GraphRunControls
        phase="idle"
        canInstantiate
        canRun={false}
        disabledReason=""
        onInstantiate={onInstantiate}
        onRun={onRun}
      />,
    );

    const instantiate = screen.getByRole('button', { name: 'Instantiate graph' }) as HTMLButtonElement;
    const run = screen.getByRole('button', { name: 'Run graph' }) as HTMLButtonElement;
    expect(instantiate.disabled).toBe(false);
    expect(run.disabled).toBe(true);
    fireEvent.click(instantiate);
    expect(onInstantiate).toHaveBeenCalledTimes(1);
    expect(onRun).not.toHaveBeenCalled();

    rerender(
      <GraphRunControls
        phase="ready"
        canInstantiate={false}
        canRun
        disabledReason=""
        onInstantiate={onInstantiate}
        onRun={onRun}
      />,
    );

    expect((screen.getByRole('button', { name: 'Instantiate graph' }) as HTMLButtonElement).disabled).toBe(true);
    const readyRun = screen.getByRole('button', { name: 'Run instantiated graph' }) as HTMLButtonElement;
    expect(readyRun.disabled).toBe(false);
    fireEvent.click(readyRun);
    expect(onRun).toHaveBeenCalledTimes(1);
  });

  it('disables both toolbar actions while setup is busy', () => {
    render(
      <GraphRunControls
        phase="instantiating"
        canInstantiate={false}
        canRun={false}
        disabledReason=""
        onInstantiate={vi.fn()}
        onRun={vi.fn()}
      />,
    );

    const instantiate = screen.getByRole('button', { name: 'Graph is being instantiated' }) as HTMLButtonElement;
    expect(instantiate.disabled).toBe(true);
    expect(instantiate.getAttribute('aria-busy')).toBe('true');
    expect((screen.getByRole('button', { name: 'Run graph' }) as HTMLButtonElement).disabled).toBe(true);
  });

  it('renders Instantiate and Run immediately before the existing Copy action', () => {
    render(
      <GraphToolbar
        graphData={{ nodes: [{ alias: 'root', types: ['Root'], properties: {} }], connections: [] }}
        extraActions={(
          <GraphRunControls
            phase="idle"
            canInstantiate
            canRun={false}
            disabledReason=""
            onInstantiate={vi.fn()}
            onRun={vi.fn()}
          />
        )}
      />,
    );

    const instantiate = screen.getByRole('button', { name: 'Instantiate graph' });
    const run = screen.getByRole('button', { name: 'Run graph' });
    const copy = screen.getByRole('button', { name: 'Copy raw graph JSON to clipboard' });
    expect(instantiate.compareDocumentPosition(run) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(run.compareDocumentPosition(copy) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  });

  it('uses one decorative 16px SVG family instead of platform text glyphs', () => {
    render(
      <GraphToolbar
        graphData={{ nodes: [{ alias: 'root', types: ['Root'], properties: {} }], connections: [] }}
        extraActions={(
          <GraphRunControls
            phase="idle"
            canInstantiate
            canRun={false}
            disabledReason=""
            onInstantiate={vi.fn()}
            onRun={vi.fn()}
          />
        )}
      />,
    );

    const buttons = [
      screen.getByRole('button', { name: 'Instantiate graph' }),
      screen.getByRole('button', { name: 'Run graph' }),
      screen.getByRole('button', { name: 'Copy raw graph JSON to clipboard' }),
    ];

    for (const button of buttons) {
      const icon = button.querySelector('svg');
      expect(icon, button.outerHTML).not.toBeNull();
      expect(icon?.getAttribute('viewBox')).toBe('0 0 16 16');
      expect(icon?.getAttribute('width')).toBe('16');
      expect(icon?.getAttribute('height')).toBe('16');
      expect(icon?.getAttribute('fill')).toBe('none');
      expect(icon?.getAttribute('stroke')).toBe('currentColor');
      expect(icon?.getAttribute('stroke-width')).toBe('1.5');
      expect(icon?.getAttribute('stroke-linecap')).toBe('round');
      expect(icon?.getAttribute('stroke-linejoin')).toBe('round');
      expect(icon?.getAttribute('aria-hidden')).toBe('true');
    }
    expect(buttons[1].textContent).not.toContain('▶');
    expect(buttons[2].textContent).not.toContain('📑');
  });

  it('shows each custom tooltip for its own hover or keyboard focus', () => {
    const onInstantiate = vi.fn();
    const onRun = vi.fn();
    const { rerender } = render(
      <GraphRunControls
        phase="idle"
        canInstantiate
        canRun={false}
        disabledReason=""
        onInstantiate={onInstantiate}
        onRun={onRun}
      />,
    );

    const instantiate = screen.getByRole('button', { name: 'Instantiate graph' }) as HTMLButtonElement;
    const run = screen.getByRole('button', { name: 'Run graph' }) as HTMLButtonElement;
    const [instantiateTooltip, runTooltip] = screen.getAllByRole('tooltip');

    expect(instantiateTooltip.textContent).toBe(
      'Prepare the current graph to run and add input if required.',
    );
    expect(runTooltip.textContent).toBe(
      'Run the graph after it has been instantiated. Instantiate the graph first.',
    );
    expect(run.disabled).toBe(true);
    expect(instantiate.getAttribute('aria-describedby')).toBe(instantiateTooltip.id);
    expect(run.getAttribute('aria-describedby')).toBe(runTooltip.id);
    expect(instantiateTooltip.id).not.toBe(runTooltip.id);
    expect(instantiateTooltip.parentElement).toBe(instantiate.parentElement);
    expect(runTooltip.parentElement).toBe(run.parentElement);
    expect(instantiate.getAttribute('title')).toBeNull();
    expect(run.getAttribute('title')).toBeNull();
    expect(instantiateTooltip.getAttribute('data-state')).toBe('closed');
    expect(runTooltip.getAttribute('data-state')).toBe('closed');

    fireEvent.focus(instantiate);
    expect(instantiateTooltip.getAttribute('data-state')).toBe('open');
    expect(runTooltip.getAttribute('data-state')).toBe('closed');
    fireEvent.blur(instantiate, { relatedTarget: document.body });
    expect(instantiateTooltip.getAttribute('data-state')).toBe('closed');

    const disabledRunAnchor = run.parentElement!;
    expect(disabledRunAnchor.getAttribute('tabindex')).toBe('0');
    expect(disabledRunAnchor.getAttribute('aria-describedby')).toBe(runTooltip.id);
    fireEvent.focus(disabledRunAnchor);
    expect(runTooltip.getAttribute('data-state')).toBe('open');
    fireEvent.blur(disabledRunAnchor, { relatedTarget: document.body });
    expect(runTooltip.getAttribute('data-state')).toBe('closed');

    fireEvent.mouseEnter(instantiate.parentElement!);
    expect(instantiateTooltip.getAttribute('data-state')).toBe('open');
    expect(runTooltip.getAttribute('data-state')).toBe('closed');
    fireEvent.click(instantiate);
    expect(onInstantiate).toHaveBeenCalledTimes(1);
    fireEvent.mouseLeave(instantiate.parentElement!);
    expect(instantiateTooltip.getAttribute('data-state')).toBe('closed');

    fireEvent.mouseEnter(run.parentElement!);
    expect(instantiateTooltip.getAttribute('data-state')).toBe('closed');
    expect(runTooltip.getAttribute('data-state')).toBe('open');
    fireEvent.click(run);
    expect(onRun).not.toHaveBeenCalled();
    fireEvent.mouseLeave(run.parentElement!);
    expect(runTooltip.getAttribute('data-state')).toBe('closed');
    expect(graphToolbarCss).toContain('visibility: hidden');
    expect(graphToolbarCss).toContain('transition: none');

    rerender(
      <GraphRunControls
        phase="ready"
        canInstantiate={false}
        canRun
        disabledReason=""
        onInstantiate={vi.fn()}
        onRun={vi.fn()}
      />,
    );

    expect(screen.getAllByRole('tooltip')[0].textContent).toBe(
      'Prepare the current graph to run and add input if required. The graph is already instantiated.',
    );
    expect(screen.getAllByRole('tooltip')[1].textContent).toBe(
      'Run the graph after it has been instantiated.',
    );
  });
});
