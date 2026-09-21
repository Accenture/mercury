// publish-inbound.js (program-2) - read lines from the console and publish each to demo.inbound.
// Run in its own terminal:  node publish-inbound.js   (then type a message and press Enter)
// An optional topic argument publishes elsewhere (the interop runbook):  node publish-inbound.js demo.interop.in

import readline from 'node:readline';
import { randomUUID, randomBytes } from 'node:crypto';
import kafkajs from 'kafkajs';
import cfg from './config.js';

const { Kafka, Partitioners } = kafkajs;

(async () => {
  const topic = process.argv[2] || cfg.inboundTopic;
  const kafka = new Kafka({ clientId: 'kafka-demo-publisher', brokers: cfg.brokers });
  // DefaultPartitioner avoids kafkajs' legacy-partitioner warning and spreads across the 10 partitions
  const producer = kafka.producer({ createPartitioner: Partitioners.DefaultPartitioner });
  await producer.connect();
  console.log(
    `[${cfg.ts()}] connected. Type a message + Enter to publish to '${topic}' (Ctrl-C to quit).`
  );

  async function publishOne(text) {
    const cid = randomUUID();
    // W3C traceparent: 00-<32-hex trace-id>-<16-hex span-id>-01. The Kafka flow adapter adopts this
    // trace-id, so the whole flow (and the message it publishes onward) shares it - making the
    // end-to-end trace continuity visible.
    const traceId = randomBytes(16).toString('hex');
    const traceparent = `00-${traceId}-${randomBytes(8).toString('hex')}-01`;
    try {
      await producer.send({
        topic,
        messages: [{ value: text, headers: { cid, traceparent } }],
      });
      console.log(`[${cfg.ts()}] -> ${topic} cid=${cid} traceId=${traceId} ${text}`);
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
  //   echo 'hello composable kafka' | node publish-inbound.js
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
