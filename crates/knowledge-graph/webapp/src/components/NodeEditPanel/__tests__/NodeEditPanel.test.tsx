// @vitest-environment happy-dom

import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import NodeEditPanel from '../NodeEditPanel';
import type { NodeFormState } from '../../../graphActions/nodeAuthoringTypes';

function makeFormState(): NodeFormState {
  return {
    alias: 'fetcher',
    nodeType: 'Fetcher',
    properties: [
      { id: 'row-1', key: 'exception', value: 'error-handler' },
      { id: 'row-2', key: 'skill', value: 'graph.api.fetcher' },
    ],
    source: 'edit-node',
  };
}

function renderPanel(overrides: Partial<React.ComponentProps<typeof NodeEditPanel>> = {}) {
  const props = {
    mode: 'edit' as const,
    formState: makeFormState(),
    phase: 'editing' as const,
    lockReason: null,
    serverMessage: null,
    validationErrors: {},
    onFormStateChange: vi.fn(),
    onSubmit: vi.fn(),
    onClose: vi.fn(),
    ...overrides,
  };
  const view = render(<NodeEditPanel {...props} />);
  return { view, props };
}

describe('NodeEditPanel', () => {
  it('renders the magnified-node ribbon with alias and type badge', () => {
    renderPanel();

    const form = screen.getByRole('form', { name: 'Edit node fetcher' });
    expect(form.style.borderColor).toBe('#2563eb'); // Fetcher accent
    expect(screen.getByText('fetcher')).toBeTruthy();
    expect(screen.getByText('Fetcher')).toBeTruthy();
    // Property rows render as editable key/value pairs.
    expect(screen.getAllByLabelText('Property key')).toHaveLength(2);
    expect(screen.getAllByLabelText('Property value')).toHaveLength(2);
  });

  it('reports key edits through onFormStateChange', () => {
    const { props } = renderPanel();

    const keyInputs = screen.getAllByLabelText('Property key');
    fireEvent.change(keyInputs[0], { target: { value: 'timeout' } });

    expect(props.onFormStateChange).toHaveBeenCalledWith(expect.objectContaining({
      properties: [
        expect.objectContaining({ id: 'row-1', key: 'timeout' }),
        expect.objectContaining({ id: 'row-2', key: 'skill' }),
      ],
    }));
  });

  it('closes on Escape unless a save is in flight', () => {
    const { props, view } = renderPanel();
    fireEvent.keyDown(document, { key: 'Escape' });
    expect(props.onClose).toHaveBeenCalledTimes(1);
    view.unmount();

    const { props: sendingProps } = renderPanel({ phase: 'sending', lockReason: 'sending' });
    fireEvent.keyDown(document, { key: 'Escape' });
    expect(sendingProps.onClose).not.toHaveBeenCalled();
  });

  it('locks the form and shows progress while sending', () => {
    renderPanel({ phase: 'sending', lockReason: 'sending' });

    expect(screen.getByRole('button', { name: 'Saving...' })).toBeTruthy();
    for (const input of screen.getAllByLabelText('Property key')) {
      expect((input as HTMLInputElement).disabled).toBe(true);
    }
  });

  it('create mode edits the alias in the ribbon with create labels', () => {
    const { props } = renderPanel({
      mode: 'create',
      formState: {
        alias: '',
        nodeType: 'Fetcher',
        properties: [{ id: 'row-1', key: '', value: '' }],
        source: 'pane-context-menu',
      },
    });

    const aliasInput = screen.getByLabelText('Node alias') as HTMLInputElement;
    expect(document.activeElement).toBe(aliasInput);
    expect(screen.getByRole('form', { name: 'Create node' })).toBeTruthy();
    expect(screen.getByRole('button', { name: 'Create Node' })).toBeTruthy();

    fireEvent.change(aliasInput, { target: { value: 'my-node' } });
    expect(props.onFormStateChange).toHaveBeenCalledWith(expect.objectContaining({ alias: 'my-node' }));
  });

  it('re-sorts and regroups rows by key after a drag-and-drop reorder', () => {
    // input[] rows separated by an output[] row: dragging the stray input[]
    // onto the first input[] row must move it before that row, and the
    // post-drop stable sort regroups keys ascending.
    const { props } = renderPanel({
      formState: {
        alias: 'fetcher',
        nodeType: 'Fetcher',
        properties: [
          { id: 'row-a', key: 'input[]', value: 'first' },
          { id: 'row-b', key: 'output[]', value: 'mapped' },
          { id: 'row-c', key: 'input[]', value: 'second' },
        ],
        source: 'edit-node',
      },
    });

    const grips = screen.getAllByRole('button', { name: /Reorder property/ });
    fireEvent.dragStart(grips[2]); // row-c (input[] "second")
    const firstRow = grips[0].closest('[data-row-id]')!;
    fireEvent.dragOver(firstRow);
    fireEvent.drop(firstRow);

    expect(props.onFormStateChange).toHaveBeenCalledWith(expect.objectContaining({
      properties: [
        expect.objectContaining({ id: 'row-c', key: 'input[]', value: 'second' }),
        expect.objectContaining({ id: 'row-a', key: 'input[]', value: 'first' }),
        expect.objectContaining({ id: 'row-b', key: 'output[]', value: 'mapped' }),
      ],
    }));
  });

  it('moves a row to the end when dropped on the add-property zone', () => {
    const { props } = renderPanel({
      formState: {
        alias: 'fetcher',
        nodeType: 'Fetcher',
        properties: [
          { id: 'row-a', key: 'input[]', value: 'first' },
          { id: 'row-b', key: 'input[]', value: 'second' },
        ],
        source: 'edit-node',
      },
    });

    const grips = screen.getAllByRole('button', { name: /Reorder property/ });
    const addZone = screen.getByRole('button', { name: 'Add Property' }).parentElement!;
    fireEvent.dragStart(grips[0]); // row-a
    fireEvent.dragOver(addZone);
    fireEvent.drop(addZone);

    // row-a moves after row-b; the stable key sort keeps that order since
    // both rows share the input[] key.
    expect(props.onFormStateChange).toHaveBeenCalledWith(expect.objectContaining({
      properties: [
        expect.objectContaining({ id: 'row-b', value: 'second' }),
        expect.objectContaining({ id: 'row-a', value: 'first' }),
      ],
    }));
  });
});
