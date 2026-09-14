#!/usr/bin/env node
/*
    SSE client for the streaming-return-route demo, printing each event with a
    wall-clock timestamp (HH:MM:SS.mmm) - the evidence format of the cross-pod
    test report.

    Start the stream-ui pod first, then:
        node scripts/sse-client.mjs
        node scripts/sse-client.mjs "http://127.0.0.1:8600/api/notifications" 6

    The first SSE event is "event: cid" - quote its data value in POSTs to the
    producer pod's /api/produce so backend services know where to post. The
    optional second argument sets the channel's idle allowance in seconds
    (the x-stream-idle-seconds request header). Requires Node.js 18+.
*/

const url = process.argv[2] ?? 'http://127.0.0.1:8600/api/notifications';
const idleSeconds = process.argv[3];

const stamp = () => {
    const now = new Date();
    const pad = (n, w = 2) => String(n).padStart(w, '0');
    return `${pad(now.getHours())}:${pad(now.getMinutes())}:${pad(now.getSeconds())}.${pad(now.getMilliseconds(), 3)}`;
};

const headers = { accept: 'text/event-stream' };
if (idleSeconds) {
    headers['x-stream-idle-seconds'] = String(idleSeconds);
}
const response = await fetch(url, { headers });
console.log(`${stamp()}  HTTP ${response.status} (${response.headers.get('content-type')})`);
if (!response.ok || !response.body) {
    console.error(await response.text());
    process.exit(1);
}

// Minimal SSE reader: frames are separated by a blank line; each frame carries
// an optional "event:" name and one or more "data:" lines. A line starting
// with ":" is a keep-alive comment.
let buffer = '';
const decoder = new TextDecoder();
for await (const chunk of response.body) {
    buffer += decoder.decode(chunk, { stream: true });
    let boundary;
    while ((boundary = buffer.indexOf('\n\n')) >= 0) {
        renderFrame(buffer.slice(0, boundary));
        buffer = buffer.slice(boundary + 2);
    }
}
console.log(`${stamp()}  (stream closed by the server)`);

function renderFrame(frame) {
    for (const line of frame.split('\n')) {
        if (line.startsWith(':')) {
            continue; // keep-alive comment
        }
        if (line.trim().length > 0) {
            console.log(`${stamp()}  ${line}`);
        }
    }
}
