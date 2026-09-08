// @vitest-environment happy-dom
/// <reference types="node" />

import { readFileSync } from 'node:fs';
import { render, waitFor } from '@testing-library/react';
import { ReactFlow, type Node } from '@xyflow/react';
import { describe, expect, it } from 'vitest';
import type { GraphNodeData } from '../../../utils/graphTransformer';
import { nodeTypes } from '../NodeTypes';

const graphViewCss = readFileSync('src/components/GraphView/GraphView.module.css', 'utf8');

const selectedNode: Node<GraphNodeData> = {
  id: 'root',
  type: 'Root',
  position: { x: 0, y: 0 },
  width: 240,
  height: 100,
  selected: true,
  data: {
    alias: 'root',
    nodeType: 'Root',
    properties: {},
    sourceHandles: [],
    targetHandles: [],
    backSourceHandles: [],
    backTargetHandles: [],
    supportsConnectionAuthoring: false,
    compact: false,
    minHeight: 100,
  },
};

describe('GraphView node resizing', () => {
  it('keeps resize controls visible for a selected node', async () => {
    const style = document.createElement('style');
    style.textContent = graphViewCss.replace(/:global\(([^)]+)\)/g, '$1');
    document.head.append(style);

    const view = render(
      <div style={{ width: 800, height: 600 }}>
        <ReactFlow
          nodes={[selectedNode]}
          edges={[]}
          nodeTypes={nodeTypes}
          fitView={false}
        />
      </div>,
    );

    try {
      await waitFor(() => {
        expect(view.container.querySelector('.react-flow__node.selected')).not.toBeNull();
      });

      const resizeControls = view.container.querySelectorAll<HTMLElement>('.react-flow__resize-control');
      expect(resizeControls.length).toBeGreaterThan(0);
      resizeControls.forEach((control) => {
        const computedStyle = window.getComputedStyle(control);
        expect(computedStyle.display).not.toBe('none');
        expect(computedStyle.visibility).not.toBe('hidden');
      });
    } finally {
      view.unmount();
      style.remove();
    }
  });
});
