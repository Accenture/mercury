#!/usr/bin/env node
/*
    Progressive rendering demo client for the hello-world SSE endpoint.

    Start the application first:
        cargo run -p hello-world

    Then run this script:
        node scripts/sse-client.mjs
        node scripts/sse-client.mjs "http://127.0.0.1:8085/api/hello/sse?delay=500&count=5"

    Each Server-Sent Event is printed the moment it arrives, prefixed with the
    elapsed time, so you can see the progressive delivery - the messages render
    one by one instead of appearing all at once. Requires Node.js 18 or higher.
*/

const url = process.argv[2] ?? 'http://127.0.0.1:8085/api/hello/sse';
const started = Date.now();
const elapsed = () => String(Date.now() - started).padStart(6, ' ');

const response = await fetch(url, { headers: { accept: 'text/event-stream' } });
console.log(`[${elapsed()} ms] HTTP ${response.status} (${response.headers.get('content-type')})`);
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
console.log(`[${elapsed()} ms] socket closed`);

function renderFrame(frame) {
    const lines = frame.split('\n');
    const event = lines.find(line => line.startsWith('event:'))?.slice(6).trim();
    const data = lines.filter(line => line.startsWith('data:'))
        .map(line => line.slice(5).trim()).join('\n');
    if (event) {
        console.log(`[${elapsed()} ms] event: ${event} - ${data}`);
    } else if (data) {
        console.log(`[${elapsed()} ms] ${data}`);
    } else if (lines.some(line => line.startsWith(':'))) {
        console.log(`[${elapsed()} ms] (keep-alive)`);
    }
}
