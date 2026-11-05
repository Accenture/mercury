#!/usr/bin/env node
/*
    Copyright 2018-2026 Accenture Technology

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
 */

/**
 * AI companion broker — lets an AI agent HOST a MiniGraph Playground session
 * instead of borrowing a human's browser session.
 *
 * The broker opens (and keeps alive) one WebSocket session against a running
 * playground app, then exposes a tiny localhost HTTP API so an agent can read
 * the session id, restart the session, or stop it. Humans join as equal
 * collaborators from a browser with `session subscribe <id>`; because the
 * broker holds the primary session, a human who drops (e.g. browser tab
 * backgrounded past the server's idle timeout, 60 s by default) simply reloads,
 * re-subscribes, and the current work-in-progress graph syncs back to them.
 *
 * When the human's browser session came first, the broker can subscribe to it
 * instead (`POST /subscribe`): its session then holds a copy of the shared
 * draft, and if the human's browser closes, the engine frees the broker's
 * session as a primary that keeps the work-in-progress, so the agent hands its
 * own session id to the human, who subscribes and recovers.
 *
 * The agent still drives commands through the companion endpoint
 * (`POST /api/companion/{session-id}/sync`) — the broker only owns the
 * session's lifecycle, never its commands.
 *
 * Zero dependencies; requires Node.js >= 22 (built-in WebSocket client).
 * Works unchanged against the Java and Rust playground engines — both
 * announce `session ws-NNNNNN-N started` on open and answer the same
 * ping/pong keep-alive frames.
 *
 * Usage:
 *   node ai-companion-broker.mjs [--target http://127.0.0.1:8085] [--port 8765]
 *
 * Control API (binds 127.0.0.1 only — the Playground is dev-only and so is this):
 *   GET  /session            -> { connected, sessionId, previousSessionIds, subscribedTo, subscribers,
 *                                 target, since }
 *   GET  /console?lines=50   -> { lines: [...] } last console lines (ping/pong filtered)
 *   POST /subscribe          -> body { "sessionId": "ws-..." }: this session subscribes to that primary
 *                               session (`session subscribe <id>`); returns { sessionId, subscribedTo }
 *   POST /start              -> close any current session, open a fresh one; returns { sessionId }
 *   POST /stop               -> close the current session (auto-reconnect disabled until /start)
 *   POST /shutdown           -> stop the broker process
 */

import { createServer } from 'node:http';
import { parseArgs } from 'node:util';

const { values: args } = parseArgs({
    options: {
        target: { type: 'string', default: 'http://127.0.0.1:8085' },
        port: { type: 'string', default: '8765' }
    }
});

const TARGET = args.target.replace(/\/+$/, '');
const CONTROL_PORT = Number(args.port);
const WS_URL = TARGET.replace(/^http/, 'ws') + '/ws/graph/playground';
const PING_INTERVAL_MS = 20_000;    // same cadence as the Playground web UI
const CONSOLE_BUFFER_MAX = 200;
const SESSION_ID_PATTERN = /^session (ws-\d+-\d+) started$/;
const SESSION_ID_FORMAT = /^ws-\d+-\d+$/;
// the engine's session-topology lines (identical in the Java and Rust engines)
const SUBSCRIBED_PATTERN = /^Subscribed to (ws-\d+-\d+)$/;
const PRIMARY_CLOSED_PATTERN = /^Session (ws-\d+-\d+) has closed$/;
const UNSUBSCRIBED_PATTERN = /^Session unsubscribed from (ws-\d+-\d+)$/;
const SUBSCRIBER_JOINED_PATTERN = /^(ws-\d+-\d+) subscribed to your session$/;
const SUBSCRIBER_LEFT_PATTERN = /^(ws-\d+-\d+) unsubscribed from your session$/;
// the engine's refusals of `session subscribe <id>`, each the start of the line it sends
const subscribeRefusals = (target) => [`Session ${target} not found`, `${target} is not a primary session`,
    'You cannot subscribe to yourself', 'You have already subscribed to'];
const SUBSCRIBE_TIMEOUT_MS = 5000;
const RECONNECT_BACKOFF_MS = [1000, 2000, 5000, 10_000];

const state = {
    ws: null,
    connected: false,
    sessionId: null,
    previousSessionIds: [],
    since: null,
    subscribedTo: null,     // the primary session this one mirrors, or null when this session is a primary
    subscribers: [],        // the sessions that mirror this one, while it is a primary
    autoReconnect: true,
    reconnectAttempt: 0,
    reconnectTimer: null,
    pingTimer: null,
    consoleLines: []
};

// Bumped on every connect/stop; events from a superseded socket are ignored.
let generation = 0;

function log(message) {
    console.log(`${new Date().toISOString()} ${message}`);
}

function pushConsoleLine(text) {
    state.consoleLines.push({ time: new Date().toISOString(), text });
    if (state.consoleLines.length > CONSOLE_BUFFER_MAX) {
        state.consoleLines.shift();
    }
}

function isKeepAliveFrame(data) {
    try {
        const parsed = JSON.parse(data);
        return parsed && typeof parsed === 'object' && (parsed.type === 'ping' || parsed.type === 'pong');
    } catch {
        return false;
    }
}

function connect() {
    const gen = ++generation;
    let ws;
    try {
        ws = new WebSocket(WS_URL);
    } catch {
        scheduleReconnect();
        return;
    }
    state.ws = ws;

    // A refused connection may fire only 'error' (no 'close'), so both paths
    // funnel into one drop handler; the generation guard drops stale events
    // from a socket that /start or /stop already replaced.
    const onDrop = () => {
        if (gen !== generation) {
            return;
        }
        cleanupConnection();
        scheduleReconnect();
    };
    ws.addEventListener('close', onDrop);
    ws.addEventListener('error', onDrop);

    ws.addEventListener('open', () => {
        if (gen !== generation) {
            return;
        }
        state.connected = true;
        state.since = new Date().toISOString();
        state.reconnectAttempt = 0;
        ws.send(JSON.stringify({ type: 'welcome' }));
        state.pingTimer = setInterval(() => {
            if (ws.readyState === WebSocket.OPEN) {
                ws.send(JSON.stringify({ type: 'ping', message: 'keep alive', time: new Date().toISOString() }));
            }
        }, PING_INTERVAL_MS);
        log(`connected to ${WS_URL}`);
    });

    ws.addEventListener('message', (event) => {
        if (gen !== generation) {
            return;
        }
        const data = typeof event.data === 'string' ? event.data : '';
        if (isKeepAliveFrame(data)) {
            return;
        }
        const match = data.match(SESSION_ID_PATTERN);
        if (match) {
            if (state.sessionId && state.sessionId !== match[1]) {
                state.previousSessionIds.push(state.sessionId);
            }
            state.sessionId = match[1];
            log(`session id: ${state.sessionId}`);
        }
        trackTopology(data);
        pushConsoleLine(data);
    });

}

/** Follow the session's place in a shared session from the engine's own topology lines. */
function trackTopology(line) {
    let m;
    if ((m = line.match(SUBSCRIBED_PATTERN))) {
        state.subscribedTo = m[1];
        log(`subscribed to ${m[1]}`);
    } else if ((m = line.match(PRIMARY_CLOSED_PATTERN)) && m[1] === state.subscribedTo) {
        state.subscribedTo = null;
        log(`primary session ${m[1]} has closed — ${state.sessionId} is now a primary session holding the ` +
            'work-in-progress graph; hand this id to the humans to subscribe');
    } else if ((m = line.match(UNSUBSCRIBED_PATTERN)) && m[1] === state.subscribedTo) {
        state.subscribedTo = null;
    } else if ((m = line.match(SUBSCRIBER_JOINED_PATTERN))) {
        if (!state.subscribers.includes(m[1])) {
            state.subscribers.push(m[1]);
        }
        log(`${m[1]} subscribed to this session`);
    } else if ((m = line.match(SUBSCRIBER_LEFT_PATTERN))) {
        state.subscribers = state.subscribers.filter((id) => id !== m[1]);
        log(`${m[1]} unsubscribed from this session`);
    }
}

/** Send `session subscribe <id>` over the broker's own WebSocket and wait for the engine's answer. */
function subscribe(target) {
    return new Promise((resolve, reject) => {
        const ws = state.ws;
        if (!state.connected || !ws || ws.readyState !== WebSocket.OPEN) {
            reject(Object.assign(new Error('no session - POST /start first'), { status: 409 }));
            return;
        }
        const from = state.consoleLines.length;
        ws.send(`session subscribe ${target}`);
        const startedAt = Date.now();
        const poll = setInterval(() => {
            const lines = state.consoleLines.slice(from).map((entry) => entry.text);
            if (state.subscribedTo === target) {
                clearInterval(poll);
                resolve(target);
                return;
            }
            const refusal = lines.find((text) => subscribeRefusals(target).some((words) => text.startsWith(words)));
            if (refusal) {
                clearInterval(poll);
                reject(Object.assign(new Error(refusal), { status: 409 }));
            } else if (Date.now() - startedAt > SUBSCRIBE_TIMEOUT_MS) {
                clearInterval(poll);
                reject(Object.assign(new Error(`no answer to 'session subscribe ${target}' within ` +
                    `${SUBSCRIBE_TIMEOUT_MS} ms`), { status: 504 }));
            }
        }, 100);
    });
}

/** The JSON body of a control request. */
function readJson(req) {
    return new Promise((resolve, reject) => {
        let text = '';
        req.setEncoding('utf8');
        req.on('data', (chunk) => { text += chunk; });
        req.on('end', () => {
            try {
                resolve(text.trim() ? JSON.parse(text) : {});
            } catch {
                reject(Object.assign(new Error('the body must be JSON, e.g. {"sessionId":"ws-123456-1"}'),
                    { status: 400 }));
            }
        });
        req.on('error', reject);
    });
}

function scheduleReconnect() {
    if (!state.autoReconnect || state.reconnectTimer) {
        return;
    }
    const delay = RECONNECT_BACKOFF_MS[Math.min(state.reconnectAttempt, RECONNECT_BACKOFF_MS.length - 1)];
    state.reconnectAttempt++;
    log(`connection down — retrying in ${delay} ms (the app may be restarting; ` +
        'a NEW session id will be issued and subscribers must re-subscribe)');
    state.reconnectTimer = setTimeout(() => {
        state.reconnectTimer = null;
        if (state.autoReconnect && !state.connected) {
            connect();
        }
    }, delay);
}

function cleanupConnection() {
    state.connected = false;
    // a reconnect is a new session: it mirrors nobody and nobody mirrors it
    state.subscribedTo = null;
    state.subscribers = [];
    if (state.pingTimer) {
        clearInterval(state.pingTimer);
        state.pingTimer = null;
    }
    if (state.sessionId) {
        state.previousSessionIds.push(state.sessionId);
        state.sessionId = null;
    }
    state.ws = null;
}

function closeCurrent() {
    state.autoReconnect = false;
    if (state.reconnectTimer) {
        clearTimeout(state.reconnectTimer);
        state.reconnectTimer = null;
    }
    const ws = state.ws;
    generation++; // orphan any in-flight events from the socket being closed
    cleanupConnection();
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.close();
    }
}

/** Resolve once a session id is captured, or reject after timeoutMs. */
function waitForSessionId(timeoutMs = 10_000) {
    return new Promise((resolve, reject) => {
        const startedAt = Date.now();
        const poll = setInterval(() => {
            if (state.sessionId) {
                clearInterval(poll);
                resolve(state.sessionId);
            } else if (Date.now() - startedAt > timeoutMs) {
                clearInterval(poll);
                reject(new Error(`no session id within ${timeoutMs} ms — is the playground app running at ${TARGET}?`));
            }
        }, 100);
    });
}

function sendJson(res, status, body) {
    res.writeHead(status, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify(body, null, 2));
}

const server = createServer(async (req, res) => {
    const url = new URL(req.url, `http://127.0.0.1:${CONTROL_PORT}`);
    const route = `${req.method} ${url.pathname}`;
    try {
        if (route === 'GET /session') {
            sendJson(res, 200, {
                connected: state.connected,
                sessionId: state.sessionId,
                previousSessionIds: state.previousSessionIds,
                subscribedTo: state.subscribedTo,
                subscribers: state.subscribers,
                target: TARGET,
                since: state.since
            });
        } else if (route === 'GET /console') {
            const n = Math.min(Number(url.searchParams.get('lines')) || 50, CONSOLE_BUFFER_MAX);
            sendJson(res, 200, { lines: state.consoleLines.slice(-n) });
        } else if (route === 'POST /subscribe') {
            const body = await readJson(req);
            const target = typeof body.sessionId === 'string' ? body.sessionId.trim() : '';
            if (!SESSION_ID_FORMAT.test(target)) {
                sendJson(res, 400, { error: 'the body must name a session, e.g. {"sessionId":"ws-123456-1"}' });
                return;
            }
            const subscribedTo = await subscribe(target);
            sendJson(res, 200, { sessionId: state.sessionId, subscribedTo });
        } else if (route === 'POST /start') {
            closeCurrent();
            state.autoReconnect = true;
            connect();
            const sessionId = await waitForSessionId();
            sendJson(res, 200, { sessionId });
        } else if (route === 'POST /stop') {
            closeCurrent();
            sendJson(res, 200, { stopped: true });
        } else if (route === 'POST /shutdown') {
            sendJson(res, 200, { shutdown: true });
            closeCurrent();
            server.close(() => process.exit(0));
        } else {
            sendJson(res, 404, { error: 'unknown route', routes: ['GET /session', 'GET /console', 'POST /subscribe',
                'POST /start', 'POST /stop', 'POST /shutdown'] });
        }
    } catch (e) {
        sendJson(res, e.status ?? 500, { error: e.message });
    }
});

server.listen(CONTROL_PORT, '127.0.0.1', () => {
    log(`AI companion broker - control API on http://127.0.0.1:${CONTROL_PORT}, target playground ${TARGET}`);
    connect();
});
