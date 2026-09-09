// @vitest-environment happy-dom

import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import MockUploadPanel from './MockUploadPanel';

afterEach(cleanup);

describe('MockUploadPanel graph-run context', () => {
  it('preserves the existing manual upload action label by default', () => {
    render(
      <MockUploadPanel
        uploadPath="/api/mock/ws-123-1"
        onSuccess={vi.fn()}
        onClose={vi.fn()}
        onError={vi.fn()}
      />,
    );

    expect(screen.getByRole('button', { name: 'Upload ▶' })).toBeTruthy();
  });

  it('can explain the derived graph inputs and next action without changing upload mechanics', () => {
    render(
      <MockUploadPanel
        uploadPath="/api/mock/ws-123-1"
        title="Add graph input"
        description="These paths are referenced by the graph."
        inputPathHints={['input.body.user.id', 'input.body.enabled']}
        submitLabel="Upload & Run"
        onSuccess={vi.fn()}
        onClose={vi.fn()}
        onError={vi.fn()}
      />,
    );

    expect(screen.getByText('Add graph input')).toBeTruthy();
    expect(screen.getByText('These paths are referenced by the graph.')).toBeTruthy();
    expect(screen.getByText('input.body.user.id')).toBeTruthy();
    expect(screen.getByText('input.body.enabled')).toBeTruthy();
    expect((screen.getByRole('button', { name: 'Upload & Run' }) as HTMLButtonElement).disabled).toBe(true);
  });

  it('renders in place — a left-slot panel region, not a dialog', () => {
    render(
      <MockUploadPanel
        uploadPath="/api/mock/ws-123-1"
        onSuccess={vi.fn()}
        onClose={vi.fn()}
        onError={vi.fn()}
      />,
    );

    expect(screen.queryByRole('dialog')).toBeNull();
    expect(screen.getByRole('region', { name: '⬆️ Upload Mock Data' })).toBeTruthy();
    expect(document.activeElement).toBe(screen.getByLabelText('JSON Payload'));
  });

  it('closes on Escape with the owning upload path, like the node editor', () => {
    const onClose = vi.fn();
    render(
      <MockUploadPanel
        uploadPath="/api/mock/ws-123-1"
        onSuccess={vi.fn()}
        onClose={onClose}
        onError={vi.fn()}
      />,
    );

    fireEvent.keyDown(document, { key: 'Escape' });

    expect(onClose).toHaveBeenCalledWith('/api/mock/ws-123-1');
  });

  it('does not close when the nested file picker dispatches a cancel event', () => {
    const onClose = vi.fn();
    const { container } = render(
      <MockUploadPanel
        uploadPath="/api/mock/ws-123-1"
        onSuccess={vi.fn()}
        onClose={onClose}
        onError={vi.fn()}
      />,
    );
    const fileInput = container.querySelector<HTMLInputElement>('input[type="file"]');
    expect(fileInput).not.toBeNull();

    fireEvent(fileInput!, new Event('cancel', { bubbles: true, cancelable: true }));

    expect(onClose).not.toHaveBeenCalled();
    expect(screen.getByRole('region', { name: '⬆️ Upload Mock Data' })).toBeTruthy();
  });
});
