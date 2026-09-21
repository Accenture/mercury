// Shared configuration for the kafka-demo Node helpers (identical to the Java example's, plus the
// two interop topics). Override the broker list with KAFKA_BOOTSTRAP_SERVERS (matches the app's default).

export default {
  brokers: (process.env.KAFKA_BOOTSTRAP_SERVERS || '127.0.0.1:9092').split(','),
  inboundTopic: 'demo.inbound',   // node publisher -> here -> kafka-demo app (direct routing)
  ordersTopic: 'demo.orders',     // node publisher -> here -> kafka-demo app (second-level routing)
  outboundTopic: 'demo.outbound', // kafka-demo app -> here -> node listener
  // dead-letter topics for the two consumer bindings - DLQ topics must be pre-provisioned
  // (auto-creation is off in production), so the create-topics helper makes them too
  dlqTopics: ['demo.inbound.dlq', 'demo.orders.dlq'],
  // the Java-interop relay legs (the 'interop' profile): in -> Rust -> demo.inbound -> Java;
  // Java -> demo.outbound -> Rust -> out
  interopTopics: ['demo.interop.in', 'demo.interop.out'],
  partitions: 10,
  // ISO-8601 timestamp for simple, sortable console logging
  ts: () => new Date().toISOString(),
};
