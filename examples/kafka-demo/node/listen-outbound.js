// listen-outbound.js (program-1) - listen on demo.outbound and log every message the app publishes.
// Run in its own terminal:  node listen-outbound.js
// An optional topic argument listens elsewhere (the interop runbook):  node listen-outbound.js demo.interop.out

import kafkajs from 'kafkajs';
import cfg from './config.js';

const { Kafka } = kafkajs;

(async () => {
  const topic = process.argv[2] || cfg.outboundTopic;
  const kafka = new Kafka({ clientId: 'kafka-demo-listener', brokers: cfg.brokers });
  const consumer = kafka.consumer({ groupId: `kafka-demo-node-listener-${topic}` });
  await consumer.connect();
  await consumer.subscribe({ topic, fromBeginning: false });
  console.log(`[${cfg.ts()}] listening on '${topic}' (Ctrl-C to quit) ...`);

  await consumer.run({
    eachMessage: async ({ partition, message }) => {
      const h = message.headers || {};
      const cid = h.cid ? h.cid.toString() : '(none)';
      // simple.kafka.notification stamps a fresh traceparent whose trace-id is the SAME one the publisher
      // sent (continuous across both Kafka hops) but with a NEW span-id (a new hop). Show both: the
      // trace-id should match the publisher / the app's telemetry; the span-id should differ from what was sent.
      const tp = h.traceparent ? h.traceparent.toString().split('-') : null;
      const traceId = tp ? tp[1] : '(none)';
      const spanId = tp ? tp[2] : '(none)';
      console.log(`[${cfg.ts()}] <- ${topic}[p${partition}] cid=${cid} traceId=${traceId} span=${spanId} ${message.value.toString()}`);
    },
  });
})().catch((e) => {
  console.error(`[${cfg.ts()}] error:`, e.message);
  process.exit(1);
});
