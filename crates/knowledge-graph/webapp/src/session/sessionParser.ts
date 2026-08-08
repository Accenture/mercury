import { isValidSessionId } from './sessionTypes';

// Session collaboration currently comes back from the backend as plain text,
// not JSON. This module is the only place that knows those backend strings.
// Components and hooks consume typed parse results through the protocol layer
// instead of matching raw console text themselves.

export interface SessionStartedParseResult {
  sessionId: string;
  companionEndpoint: string | null;
}

export interface SessionStatusParseResult {
  sessionId: string;
  startedSince: string;
  subscribedTo: string | null;
  subscribers: string[];
}

export interface SessionCommandResultParseResult {
  command: 'subscribe' | 'unsubscribe' | 'show' | 'unknown';
  status: 'accepted' | 'rejected' | 'info';
  sessionId: string | null;
  message: string;
}

export interface SessionNotificationParseResult {
  type: 'subscriber-joined' | 'subscriber-left' | 'host-closed';
  sessionId: string;
  message: string;
}

const SESSION_ID_CAPTURE = '(ws-\\d+-\\d+)';

const STARTED_RE = new RegExp(`^session ${SESSION_ID_CAPTURE} started(?:\\nCompanion endpoint: (\\/api\\/companion\\/${SESSION_ID_CAPTURE}))?$`, 'i');
const STATUS_FIRST_LINE_RE = new RegExp(`^Session ${SESSION_ID_CAPTURE} started since (.+)$`);
const SUBSCRIBED_TO_RE = new RegExp(`^subscribed to ${SESSION_ID_CAPTURE}$`);
const SUBSCRIBED_BY_RE = /^subscribed by \[(.*)]$/;
const SUBSCRIBE_SUCCESS_RE = new RegExp(`^Subscribed to ${SESSION_ID_CAPTURE}$`);
const UNSUBSCRIBE_SUCCESS_RE = new RegExp(`^Session unsubscribed from ${SESSION_ID_CAPTURE}$`);
const SESSION_NOT_FOUND_RE = new RegExp(`^Session ${SESSION_ID_CAPTURE} not found$`);
const NOT_PRIMARY_RE = new RegExp(`^${SESSION_ID_CAPTURE} is not a primary session$`);
const ALREADY_SUBSCRIBED_RE = new RegExp(`^You have already subscribed to ${SESSION_ID_CAPTURE}(?:\\nPlease do 'session reset' before subscribing to another session)?$`);
const SUBSCRIBER_JOINED_RE = new RegExp(`^${SESSION_ID_CAPTURE} subscribed to your session$`);
const SUBSCRIBER_LEFT_RE = new RegExp(`^${SESSION_ID_CAPTURE} unsubscribed from your session$`);
const HOST_CLOSED_RE = new RegExp(`^Session ${SESSION_ID_CAPTURE} has closed$`);

// Ignore blank messages and echoed user commands such as "> session". Echoes
// are displayed in the console but must not drive Session dropdown state.
function normalizedBody(raw: string): string | null {
  const text = raw.trim();
  if (text.length === 0) return null;
  if (text.startsWith('> ')) return null;
  return text;
}

// Backend formats subscribers as a bracketed comma-separated list:
// "subscribed by [ws-1-1, ws-2-2]". Filter invalid ids defensively so a
// malformed item does not render as an active subscriber.
function parseSubscriberList(value: string): string[] {
  return value
    .split(',')
    .map(item => item.trim())
    .filter(item => item.length > 0 && isValidSessionId(item));
}

// Initial WebSocket lifecycle message, emitted when the engine starts a new
// session for this browser connection.
export function parseSessionStarted(raw: string): SessionStartedParseResult | null {
  const text = normalizedBody(raw);
  if (!text) return null;

  const match = text.match(STARTED_RE);
  if (!match) return null;

  return {
    sessionId: match[1],
    companionEndpoint: match[2] ?? null,
  };
}

// Full status response from the "session" command. This is the authoritative
// snapshot used to reconcile UI state after manual console commands, refreshes,
// or notification races.
export function parseSessionStatus(raw: string): SessionStatusParseResult | null {
  const text = normalizedBody(raw);
  if (!text) return null;

  const lines = text.split('\n').map(line => line.trim()).filter(Boolean);
  const firstLine = lines[0];
  if (!firstLine) return null;

  const first = firstLine.match(STATUS_FIRST_LINE_RE);
  if (!first) return null;

  let subscribedTo: string | null = null;
  let subscribers: string[] = [];

  for (const line of lines.slice(1)) {
    const target = line.match(SUBSCRIBED_TO_RE);
    if (target) {
      subscribedTo = target[1];
      continue;
    }

    const subscriberList = line.match(SUBSCRIBED_BY_RE);
    if (subscriberList) {
      subscribers = parseSubscriberList(subscriberList[1]);
    }
  }

  return {
    sessionId: first[1],
    startedSince: first[2],
    subscribedTo,
    subscribers,
  };
}

// Direct command responses for subscribe/unsubscribe. Accepted responses update
// local optimistic state; rejected responses surface the backend message under
// the dropdown form.
export function parseSessionCommandResult(raw: string): SessionCommandResultParseResult | null {
  const text = normalizedBody(raw);
  if (!text) return null;

  const subscribeSuccess = text.match(SUBSCRIBE_SUCCESS_RE);
  if (subscribeSuccess) {
    return {
      command: 'subscribe',
      status: 'accepted',
      sessionId: subscribeSuccess[1],
      message: text,
    };
  }

  const unsubscribeSuccess = text.match(UNSUBSCRIBE_SUCCESS_RE);
  if (unsubscribeSuccess) {
    return {
      command: 'unsubscribe',
      status: 'accepted',
      sessionId: unsubscribeSuccess[1],
      message: text,
    };
  }

  const notFound = text.match(SESSION_NOT_FOUND_RE);
  if (notFound) {
    return { command: 'subscribe', status: 'rejected', sessionId: notFound[1], message: text };
  }

  const notPrimary = text.match(NOT_PRIMARY_RE);
  if (notPrimary) {
    return { command: 'subscribe', status: 'rejected', sessionId: notPrimary[1], message: text };
  }

  const alreadySubscribed = text.match(ALREADY_SUBSCRIBED_RE);
  if (alreadySubscribed) {
    return { command: 'subscribe', status: 'rejected', sessionId: alreadySubscribed[1], message: text };
  }

  if (text === 'You cannot subscribe to yourself') {
    return { command: 'subscribe', status: 'rejected', sessionId: null, message: text };
  }

  if (text === 'Nothing to unsubscribe') {
    return { command: 'unsubscribe', status: 'rejected', sessionId: null, message: text };
  }

  if (text === 'Invalid session command') {
    return { command: 'unknown', status: 'rejected', sessionId: null, message: text };
  }

  return null;
}

// Asynchronous peer notifications sent to the other side of a relationship.
// Example: the host receives "ws-... subscribed to your session" when another
// browser subscribes to it.
export function parseSessionNotification(raw: string): SessionNotificationParseResult | null {
  const text = normalizedBody(raw);
  if (!text) return null;

  const joined = text.match(SUBSCRIBER_JOINED_RE);
  if (joined) {
    return { type: 'subscriber-joined', sessionId: joined[1], message: text };
  }

  const left = text.match(SUBSCRIBER_LEFT_RE);
  if (left) {
    return { type: 'subscriber-left', sessionId: left[1], message: text };
  }

  const hostClosed = text.match(HOST_CLOSED_RE);
  if (hostClosed) {
    return { type: 'host-closed', sessionId: hostClosed[1], message: text };
  }

  return null;
}
