// @vitest-environment happy-dom

import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import ConnectionPopover from '../ConnectionPopover';
import type { ConnectionFormState } from '../../../graphActions/connectionAuthoringTypes';

function makeFormState(overrides: Partial<ConnectionFormState> = {}): ConnectionFormState {
  return { sourceAlias: 'root', targetAlias: 'fetcher', relation: '', ...overrides };
}

function renderPopover(overrides: Partial<React.ComponentProps<typeof ConnectionPopover>> = {}) {
  const props = {
    formState: makeFormState(),
    phase: 'editing' as const,
    lockReason: null,
    serverMessage: null,
    validationErrors: {},
    anchor: { x: 200, y: 200 },
    onFormStateChange: vi.fn(),
    onSubmit: vi.fn(),
    onClose: vi.fn(),
    ...overrides,
  };
  const view = render(<ConnectionPopover {...props} />);
  return { view, props };
}

describe('ConnectionPopover', () => {
  it('shows the endpoints and the relation vocabulary as chips', () => {
    renderPopover();

    expect(screen.getByRole('dialog', { name: 'Create connection from root to fetcher' })).toBeTruthy();
    expect(screen.getByRole('button', { name: 'fetch' })).toBeTruthy();
    expect(screen.getByRole('button', { name: 'complete' })).toBeTruthy();
    expect(screen.getByLabelText('Relation name')).toBeTruthy();
  });

  it('a chip sets the relation and submits once the new state round-trips', () => {
    const { view, props } = renderPopover();

    fireEvent.click(screen.getByRole('button', { name: 'fetch' }));
    expect(props.onFormStateChange).toHaveBeenCalledWith(expect.objectContaining({ relation: 'fetch' }));
    expect(props.onSubmit).not.toHaveBeenCalled();

    // The host applies the change and re-renders with the updated form state —
    // only then does the armed chip submit fire.
    view.rerender(
      <ConnectionPopover
        formState={makeFormState({ relation: 'fetch' })}
        phase="editing"
        lockReason={null}
        serverMessage={null}
        validationErrors={{}}
        anchor={{ x: 200, y: 200 }}
        onFormStateChange={props.onFormStateChange}
        onSubmit={props.onSubmit}
        onClose={props.onClose}
      />,
    );
    expect(props.onSubmit).toHaveBeenCalledTimes(1);
  });

  it('submits typed custom relations and disables the empty submit', () => {
    const { props } = renderPopover({ formState: makeFormState({ relation: 'my-relation' }) });

    const submit = screen.getByRole('button', { name: 'Connect' }) as HTMLButtonElement;
    expect(submit.disabled).toBe(false);
    fireEvent.submit(submit.closest('form')!);
    expect(props.onSubmit).toHaveBeenCalledTimes(1);
  });

  it('closes on Escape unless a create is in flight', () => {
    const { view, props } = renderPopover();
    fireEvent.keyDown(document, { key: 'Escape' });
    expect(props.onClose).toHaveBeenCalledTimes(1);
    view.unmount();

    const { props: sendingProps } = renderPopover({ phase: 'sending', lockReason: 'sending' });
    fireEvent.keyDown(document, { key: 'Escape' });
    expect(sendingProps.onClose).not.toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Creating…' })).toBeTruthy();
  });
});
