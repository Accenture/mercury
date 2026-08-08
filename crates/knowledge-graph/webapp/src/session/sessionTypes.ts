import type { ToastType } from '../hooks/useToast';
import type { ProtocolBus } from '../protocol/bus';
import type { ProtocolEvent } from '../protocol/events';

export const SESSION_ID_PATTERN = /^ws-\d+-\d+$/;

export type SessionPendingCommand = 'refresh' | 'subscribe' | 'unsubscribe' | 'reset' | null;

export interface SessionUiState {
  sessionId: string | null;
  startedSince: string | null;
  subscribedTo: string | null;
  subscribers: string[];
  loading: boolean;
  pendingCommand: SessionPendingCommand;
  error: string | null;
  lastInfo: string | null;
}

export interface SessionCollaborationController {
  state: SessionUiState;
  connected: boolean;
  isPrimary: boolean;
  hasSubscribers: boolean;
  canSubscribe: boolean;
  canUnsubscribe: boolean;
  canReset: boolean;
  subscribeToSession: (sessionId: string) => boolean;
  unsubscribe: () => boolean;
  resetSession: () => boolean;
  clearMessage: () => void;
}

export interface SessionCollaborationOptions {
  enabled: boolean;
  connected: boolean;
  bus: ProtocolBus;
  classificationMap?: Map<number, ProtocolEvent[]>;
  sendRawText: (text: string) => boolean;
  addToast: (message: string, type?: ToastType) => void;
}

export function isValidSessionId(value: string): boolean {
  return SESSION_ID_PATTERN.test(value.trim());
}
