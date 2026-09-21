// publish-orders.js (program-3) - drive the SECOND-LEVEL ROUTING demo: publish mixed event types to
// demo.orders and watch the adapter's rule list pick a different target per message.
// Run in its own terminal:  node publish-orders.js   (then type a command and press Enter)
//
// Console commands (first word decides the record shape; each exercises one routing rule):
//
//   order [json]       'type: order' header (+ JSON body)   -> exact rule    -> flow://demo-order-flow
//   order-<x> [json]   'type: order-<x>' header (+ body)    -> wildcard rule -> flow://demo-order-flow
//   refund [json]      NO type header, {"event":{"kind":"refund"},...} body
//                                                           -> body rule     -> task://demo.refund.processor
//   <anything else>    raw text (not JSON, no type header)  -> default       -> flow://demo-catch-all-flow
//
// The optional [json] overrides the canned payload, e.g.:  order {"item":"laptop","qty":2}
// Rule evaluation is first-match-wins in declaration order; a non-match falls through to 'default'.

import readline from 'node:readline';
import { randomUUID, randomBytes } from 'node:crypto';
import kafkajs from 'kafkajs';
import cfg from './config.js';

const { Kafka, Partitioners } = kafkajs;

// Build {headers, value, rule} for one console line - the record shape decides which rule fires.
function toRecord(line) {
  const space = line.indexOf(' ');
  const keyword = space === -1 ? line : line.substring(0, space);
  const rest = space === -1 ? '' : line.substring(space + 1).trim();
  const customJson = rest.startsWith('{') || rest.startsWith('[') ? rest : null;
  if (keyword === 'order' || keyword.startsWith('order-')) {
    const matched = keyword === 'order'
      ? 'input.header.type(order) [exact]' : 'input.header.type(order-*) [wildcard]';
    // The rest of the line is the payload VERBATIM - json or not. A non-JSON payload keeps its raw
    // byte[] under serializer 'json', and the Map-typed order processor cannot digest raw bytes:
    // type 'order hello' to demo the failure path (3 retries, then dead-letter to demo.orders.dlq).
    const value = rest || JSON.stringify({ item: 'keyboard', qty: 1 });
    const failurePath = rest.length > 0 && !customJson;
    return {
      headers: { type: keyword },
      value,
      rule: failurePath
        ? `${matched} -> flow://demo-order-flow - payload is NOT JSON: expect 3 retries then dead-letter to demo.orders.dlq`
        : `${matched} -> flow://demo-order-flow`,
    };
  }
  if (keyword === 'refund') {
    // no 'type' header: the header rules cannot match, so the input.body rule decides.
    // The optional json holds the refund DETAILS only - the {"event":{"kind":"refund"}} envelope
    // the body rule matches on is added here, so 'refund {"order_id":"order-123"}' routes correctly.
    let details;
    try {
      details = customJson ? JSON.parse(customJson) : { orderId: randomUUID().substring(0, 8) };
    } catch (e) {
      return { error: `invalid json after 'refund': ${e.message}` };
    }
    return {
      headers: {},
      value: JSON.stringify({ event: { kind: 'refund' }, ...details }),
      rule: 'input.body.event.kind(refund) [body rule] -> task://demo.refund.processor (see the app log)',
    };
  }
  // raw text: not a JSON object/array, so serializer 'json' keeps the byte[] and no rule matches
  return {
    headers: {},
    value: line,
    rule: 'default -> flow://demo-catch-all-flow (raw bytes pass through)',
  };
}

(async () => {
  const kafka = new Kafka({ clientId: 'kafka-demo-orders-publisher', brokers: cfg.brokers });
  // DefaultPartitioner avoids kafkajs' legacy-partitioner warning and spreads across the 10 partitions
  const producer = kafka.producer({ createPartitioner: Partitioners.DefaultPartitioner });
  await producer.connect();
  console.log(`[${cfg.ts()}] connected. Publish to '${cfg.ordersTopic}' (Ctrl-C to quit).`);
  console.log('');
  console.log('1. Exact header rule (type=order) routes to flow://demo-order-flow');
  console.log('');
  console.log('   command: order [json]');
  console.log('   example: order {"item": "mobile-phone", "qty": 1}');
  console.log('');
  console.log('2. Wildcard header rule (type=order-*) also routes to flow://demo-order-flow');
  console.log('');
  console.log('   command: order-<id> [json]');
  console.log('   example: order-123 {"item": "laptop", "qty": 1}');
  console.log('');
  console.log('3. Body rule (event.kind=refund) routes to task://demo.refund.processor');
  console.log('');
  console.log('   command: refund [json]      (json = refund details; the event envelope is added)');
  console.log('   example: refund {"order_id": "order-123"}');
  console.log('');
  console.log('4. When no rule matches, default routes to flow://demo-catch-all-flow');
  console.log('');
  console.log('   command: <anything else>');
  console.log('   example: hello');
  console.log('');
  console.log('5. Failure path: a non-JSON payload on a matched order rule triggers the DLQ');
  console.log('');
  console.log('   command: order <plain text>');
  console.log('   example: order hello');
  console.log('   The order flow cannot digest raw bytes: 3 retries, then the original record is');
  console.log('   dead-lettered to demo.orders.dlq - watch it arrive with:  node listen-dlq.js');
  console.log('');

  async function publishOne(text) {
    const { headers, value, rule, error } = toRecord(text);
    if (error) {
      console.error(`[${cfg.ts()}] not published - ${error}`);
      return;
    }
    const cid = randomUUID();
    // W3C traceparent: the adapter adopts this trace-id, so the selected flow/task (and any message
    // it publishes to demo.outbound) shares it - end-to-end trace continuity per routed message.
    const traceId = randomBytes(16).toString('hex');
    const traceparent = `00-${traceId}-${randomBytes(8).toString('hex')}-01`;
    try {
      await producer.send({
        topic: cfg.ordersTopic,
        messages: [{ value, headers: { ...headers, cid, traceparent } }],
      });
      console.log(`[${cfg.ts()}] -> ${cfg.ordersTopic} cid=${cid} traceId=${traceId}`);
      console.log(`[${cfg.ts()}]    expected: ${rule}`);
    } catch (e) {
      console.error(`[${cfg.ts()}] publish failed:`, e.message);
    }
  }

  // Show the interactive prompt only on a TTY - piped input otherwise gets '>' characters
  // glued onto the log lines.
  const interactive = process.stdin.isTTY === true;
  const rl = readline.createInterface({
    input: process.stdin,
    output: interactive ? process.stdout : undefined,
    prompt: '> ',
  });
  if (interactive) rl.prompt();

  // chain the sends so they stay ordered and can be awaited on close - the script then works both
  // interactively AND with piped input for scripted regression runs, e.g.
  //   printf 'order\nrefund\nhello\n' | node publish-orders.js
  let inflight = Promise.resolve();

  rl.on('line', (line) => {
    const text = line.trim();
    if (text.length > 0) {
      inflight = inflight.then(() => publishOne(text));
    }
    if (interactive) rl.prompt();
  });

  rl.on('close', async () => {
    await inflight;
    await producer.disconnect();
    process.exit(0);
  });
})().catch((e) => {
  console.error(`[${cfg.ts()}] error:`, e.message);
  process.exit(1);
});
