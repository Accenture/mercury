// Shared configuration for the sync-over-async-demo topic admin helper.
// Override the broker list with KAFKA_BOOTSTRAP_SERVERS (matches the application's template default).

export default {
  brokers: (process.env.KAFKA_BOOTSTRAP_SERVERS || '127.0.0.1:9092').split(','),
  requestTopic: 'soa.request',    // facade -> here -> backend (system-of-record)   [byte[] transport]
  responseTopic: 'soa.response',  // backend -> here -> facade (soa-reply)          [byte[] transport]
  jsonRequestTopic: 'json-topic-1',   // facade -> backend (system-of-record-json)  [JSON Schema]
  jsonResponseTopic: 'json-topic-2',  // backend -> facade (soa-reply-json)         [JSON Schema]
  avroRequestTopic: 'avro-topic-1',   // facade -> backend (system-of-record-avro)  [Avro]
  avroResponseTopic: 'avro-topic-2',  // backend -> facade (soa-reply-avro)         [Avro]
  // Protobuf topics are deliberately not created here - see README.md's "Protobuf (not currently
  // supported)" section for why.
  // Multiple partitions so the facade consumer group (soa-reply-group) can spread across more than one
  // facade instance - that is what lets you run two facades and see the Redis return route deliver each
  // reply to the pod that originated the request. (Auto-created topics get only 1 partition.)
  partitions: 10,
  // ISO-8601 timestamp for simple, sortable console logging
  ts: () => new Date().toISOString(),
};
