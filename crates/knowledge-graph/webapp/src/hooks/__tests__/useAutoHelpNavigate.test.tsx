// @vitest-environment happy-dom

import { act, renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi, type Mock } from 'vitest';
import { ProtocolBus } from '../../protocol/bus';
import { useAutoHelpNavigate } from '../useAutoHelpNavigate';

vi.mock('../../data/helpContent', () => ({
  getHelpContent: (topic: string, profile = 'minigraph') => {
    if (profile === 'json-path') return topic === '' ? '# JSON-Path Overview' : null;
    return topic === '' || topic === 'create' ? '# MiniGraph Help' : null;
  },
}));

function emitHelp(bus: ProtocolBus, commandText: string): void {
  bus.emit({
    kind: 'command.helpOrDescribe',
    msgId: 1,
    raw: `> ${commandText}`,
    commandText,
  });
}

describe('useAutoHelpNavigate', () => {
  let bus: ProtocolBus;
  let setHelpTopic: Mock<(topic: string) => void>;
  let openHelp: Mock<() => void>;

  beforeEach(() => {
    bus = new ProtocolBus();
    setHelpTopic = vi.fn<(topic: string) => void>();
    openHelp = vi.fn<() => void>();
  });

  it('opens the JSON-Path Overview for the bare help command', () => {
    renderHook(() => useAutoHelpNavigate({
      bus,
      setHelpTopic,
      onTabSwitch: openHelp,
      contentProfile: 'json-path',
    }));

    act(() => emitHelp(bus, 'help'));

    expect(setHelpTopic).toHaveBeenCalledWith('');
    expect(openHelp).toHaveBeenCalledOnce();
  });

  it('does not intercept MiniGraph-only topics in JSON-Path', () => {
    renderHook(() => useAutoHelpNavigate({
      bus,
      setHelpTopic,
      onTabSwitch: openHelp,
      contentProfile: 'json-path',
    }));

    act(() => emitHelp(bus, 'help create'));

    expect(setHelpTopic).not.toHaveBeenCalled();
    expect(openHelp).not.toHaveBeenCalled();
  });

  it('does not subscribe when help is disabled', () => {
    renderHook(() => useAutoHelpNavigate({
      bus,
      setHelpTopic,
      onTabSwitch: openHelp,
      enabled: false,
    }));

    act(() => emitHelp(bus, 'help'));

    expect(setHelpTopic).not.toHaveBeenCalled();
    expect(openHelp).not.toHaveBeenCalled();
  });
});
