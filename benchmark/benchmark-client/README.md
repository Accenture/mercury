# Benchmark client

This benchmark client sends events to the benchmark server through the kafka event stream system.

It measures the round trip time in sending an event from the client to the server.

# Running this benchmark application

Please "cd" to this subproject and enter "mvn clean test".

# Sample benchmark

The following benchmark is measured with:
1. Windows 11 Pro
2. Intel i7 10th generation 2.7 GHz CPU and 16 GB memory
3. Java OpenJDK version 18

```
Small payload one-way of 72 bytes - min 513.875, max 790.514 events/second
Small payload 2-way of 69 bytes - min 674.309, max 749.064 events/second

Medium payload one-way of 10,069 bytes - min 704.225, max 907.441 events/second
Medium payload 2-way of 10,066 bytes - min 988.142, max 1,207.729 events/second

Large payload one-way of 100,071 bytes - min 413.907, max 479.386 events/second
Large payload 2-way of 100,068 bytes - min 339.674, max 370.92 events/second

Extra large payload one-way of 500,071 bytes - min 90.959, max 102.291 events/second
Extra large payload 2-way of 500,068 bytes - min 62.406, max 77.748 events/second
```

The above measurement uses parallel RPC (request-response). 
One-way means sending some payload and return a boolean value in the response.
Two-way means sending some payload and return the same payload in the response.

Therefore, the actual number of events processed is 2 times the above values.

For details, please refer to report in BENCHMARK.txt

# Behind the curtain

The underlying in-memory event system is Eclipse Vertx and Kafka network event stream system.

For flow-control, mercury implements a manager/workers pattern for each service.

The number of workers per service is configured to be 200.
Since the number of concurrent requests is more than 200, the system will queue the outstanding requests orderly.

Events are queued to an embedded Berkerly DB which is the same key-value store for many search engines.

Each application instance is allocated a dedicated kafka topic as the communication conduit. 
Events are serialized before sending through the network. Any payload that is larger than 64 KB will be automatically
broken into segments of 64 KB chunks.

The above performance benchmark report reflects this queueing mechanism and orderly execution.

# Most efficient payload size

As anticipated, the most efficient payload size over the network is 6 to 10 KB.

# Why we use AsyncRequest for the benchmark tests

Traditional RPC calls are blocking, the AsyncRequest is the non-blocking version of parallel RPC.

We are doing parallel RPC so we can demonstrate real-world performance when many requests and 
responses are processed in parallel.
