// create-topics.js - administratively create the demo topics (10 partitions each) on the local broker.
// Run once after starting kafka-standalone:  node create-topics.js

import kafkajs from 'kafkajs';
import cfg from './config.js';

const { Kafka } = kafkajs;

(async () => {
  const kafka = new Kafka({ clientId: 'kafka-demo-admin', brokers: cfg.brokers });
  const admin = kafka.admin();
  await admin.connect();
  try {
    const wanted = [cfg.inboundTopic, cfg.ordersTopic, cfg.outboundTopic, ...cfg.dlqTopics, ...cfg.interopTopics];
    const existing = await admin.listTopics();
    const toCreate = wanted
      .filter((t) => !existing.includes(t))
      .map((topic) => ({ topic, numPartitions: cfg.partitions, replicationFactor: 1 }));

    if (toCreate.length === 0) {
      console.log(`[${cfg.ts()}] topics already exist: ${wanted.join(', ')}`);
    } else {
      await admin.createTopics({ topics: toCreate, waitForLeaders: true });
      console.log(
        `[${cfg.ts()}] created (${cfg.partitions} partitions each): ${toCreate.map((t) => t.topic).join(', ')}`
      );
    }
  } finally {
    await admin.disconnect();
  }
})().catch((e) => {
  console.error(`[${cfg.ts()}] error:`, e.message);
  process.exit(1);
});
