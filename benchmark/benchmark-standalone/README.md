# Benchmark test for standalone application

This application implements benchmark tests for a standalone application.

Performance metrics are measured for various scenarios for the in-memory event system.

# Running this benchmark application

Please "cd" to this subproject and enter "mvn clean test".

# Sample benchmark

The following benchmark is measured with:
1. Windows 11 Pro
2. Intel i7 10th generation 2.7 GHz CPU and 16 GB memory
3. Java OpenJDK version 18

```
Small payload one-way of 74 bytes - min 16,339.869, max 37,313.434 events/second
Small payload 2-way of 71 bytes - min 19,841.27, max 40,650.406 events/second

Medium payload one-way of 10,071 bytes - min 10,504.201, max 12,376.238 events/second
Medium payload 2-way of 10,068 bytes - min 9,900.99, max 11,933.174 events/second

Large payload one-way of 100,073 bytes - min 3,508.772, max 8,403.361 events/second
Large payload 2-way of 100,070 bytes - min 6,250, max 7,352.941 events/second

Extra large payload one-way of 500,073 bytes - min 1,976.285, max 2,212.389 events/second
Extra large payload 2-way of 500,070 bytes - min 950.57, max 1,315.789 events/second
```

The above measurement uses parallel RPC (request-response).
One-way means sending some payload and return a boolean value in the response.
Two-way means sending some payload and return the same payload in the response.

Therefore, the actual number of events processed is 2 times the above values.

For details, please refer to report in BENCHMARK.txt

# Behind the curtain

The underlying in-memory event system is Eclipse Vertx.

For flow-control, mercury implements a manager/workers pattern for each service.

The number of workers per service is configured to be 200.
Since the number of concurrent requests is more than 200, the system will queue the outstanding requests orderly.

Events are queued to an embedded Berkerly DB which is the same key-value store for many search engines. 

The above performance benchmark report reflects this queueing mechanism and orderly execution.

# Why we use AsyncRequest for the benchmark tests

Traditional RPC calls are blocking, the AsyncRequest is the non-blocking version of parallel RPC.

We are doing parallel RPC so we can demonstrate real-world performance when many requests and
responses are processed in parallel.
