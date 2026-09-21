// listen-dlq.js (program-4) - listen on the dead-letter topics and print every intercepted message.
// Run in its own terminal:  node listen-dlq.js
//
// A message lands here after exhausting its retries (kafka.flow.max.retries) - the adapter parks the
// ORIGINAL record (headers + body preserved verbatim) with two extra headers: 'dlq.error' (why it
// failed) and 'dlq.origin.topic' (the concrete topic it came from). Trigger one with the failure-path
// command in publish-orders.js:  order hello

import kafkajs from 'kafkajs';
import cfg from './config.js';

const { Kafka } = kafkajs;

(async () => {
  const kafka = new Kafka({ clientId: 'kafka-demo-dlq-listener', brokers: cfg.brokers });
  const consumer = kafka.consumer({ groupId: 'kafka-demo-node-dlq-listener' });
  await consumer.connect();
  for (const topic of cfg.dlqTopics) {
    await consumer.subscribe({ topic, fromBeginning: false });
  }
  console.log(`[${cfg.ts()}] listening on '${cfg.dlqTopics.join("', '")}' (Ctrl-C to quit) ...`);

  await consumer.run({
    eachMessage: async ({ topic, partition, message }) => {
      const h = message.headers || {};
      const cid = h.cid ? h.cid.toString() : '(none)';
      const origin = h['dlq.origin.topic'] ? h['dlq.origin.topic'].toString() : '(none)';
      const error = h['dlq.error'] ? h['dlq.error'].toString() : '(none)';
      console.log(`[${cfg.ts()}] <- ${topic}[p${partition}] cid=${cid}`);
      console.log(`[${cfg.ts()}]    origin: ${origin}`);
      console.log(`[${cfg.ts()}]    error:  ${error}`);
      console.log(`[${cfg.ts()}]    body:   ${message.value ? message.value.toString() : '(null)'}`);
    },
  });
})().catch((e) => {
  console.error(`[${cfg.ts()}] error:`, e.message);
  process.exit(1);
});
