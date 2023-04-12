# Design notes

## Gson

We are using Gson for its minimalist design.

We have customized its serialization behavior to be in line with Jackson and other serializers. 
i.e. Integer and long values are kept without decimal points.

For backward compatibility with Jackson, we have added the writeValueAsString, writeValueAsBytes and readValue methods. 
The convertValue method has been consolidated into the readValue method.

Custom serialization annotations are discouraged.

## MsgPack

MsgPack is used as the serialization engine from Map to byte array and vice versa because we use Map as the 
intermediate object for events.

For efficient and serialization performance, we use it as schemaless binary transport for EventEnvelope, 
a vehicle for event metadata, headers and payload.

## Abstraction layer

The SimpleMapper and EventEnvelope classes are used to hide the complexity of serialization.

## Custom JSON and XML serializers for JAX-RS and Spring Boot

For the `rest-spring` project, we have customized JAX-RS, Spring Boot and Servlet serialization and exception 
handlers for consistency.

## Reactive design

Mercury uses the temporary local file system as an overflow area for events when the consumer is 
slower than the producer. Normally, user application does not have to handle back-pressure.

However, user application may also control back-pressure by implementing alternative flow-control mechanism.

## Vertx

Akka actor is used as the in-memory event bus in Mercury version 1.

Since Mercury version 2, we have migrated from Akka to Eclipse Vertx as the in-memory event system.

In Mercury version 3, we extend it to be fully non-blocking and provide low-level control of application
performance and throughput as follows.

## Non-blocking design

The foundation library (platform-core) has been integrated with Kotlin coroutine and
suspend function features. We have also removed all blocking APIs from the platform-core library.

Applications using Mercury version 2 blocking RPC calls should be refactored to "suspend function".
This would reduce memory footprint and increase throughput. i.e. support more concurrent users and requests.

Since many functions in an application may be waiting for responses from a database or from a REST endpoint,
"suspend function" pattern releases CPU resources during the "wait" state, thus significantly improve 
overall system throughput.

## Low level control of function execution strategies

You can precisely control how your functions execute, using kernel thread pool, coroutine or suspend function
to address various use cases to yield the highest performance and throughput.

Kernel threads provide the highest performance in terms of operations per second if the number of threads is smaller.
As a rule of thumb, do not set "event.worker.pool" higher than 200.

Coroutine is ideal for functions that execute very quickly because they can yield control to other coroutines.

Suspend function should be used to support "sequential non-blocking" RPC or logic that need artificial delay.
You can use the "awaitRequest" and "delay" methods respectively.

## Optional Spring Boot

The `platform-core` includes a non-blocking HTTP and websocket server.

The platform-core is designed to run in standalone mode or as a bundle with Spring Boot or other frameworks.
You can add "Spring Boot" support using the `rest-spring` library or a regular Spring Boot application server directly.
