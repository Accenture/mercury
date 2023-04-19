# Design notes

## Non-blocking design

The foundation library (platform-core) has been integrated with Kotlin coroutine and
suspend function features. We have also removed all blocking APIs from the platform-core library.

Applications using Mercury version 2 blocking RPC calls should be refactored to "suspend function".
This would reduce memory footprint and increase throughput. i.e. support more concurrent users and requests.

Since many functions in an application may be waiting for responses from a database or from a REST endpoint,
"suspend function" approach releases CPU resources during the "wait" state, thus contributing to
higher application throughput.

## Low level control of function execution strategies

You can precisely control how your functions execute, using kernel thread pool, coroutine or suspend function
to address various use cases to yield the highest performance and throughput.

Kernel threads provide the highest performance in terms of operations per second when the number of threads is smaller.
As a rule of thumb, do not set "event.worker.pool" higher than 200.

Coroutine is ideal for functions that execute very quickly to yield control to other coroutines.

Suspend function should be used to support "sequential non-blocking" RPC or logic that requires artificial delay.
You can use the "awaitRequest" and "delay" APIs respectively.

## Serialization

### Gson

We are using Gson for its minimalist design.

We have customized the serialization behavior to be similar to Jackson and other serializers. 
i.e. Integer and long values are kept without decimal points.

For backward compatibility with Jackson, we have added the writeValueAsString, writeValueAsBytes and readValue methods. 
The convertValue method has been consolidated into the readValue method.

For simplicity, custom serialization annotations are discouraged.

### MsgPack

For efficient and serialization performance, we use MsgPack as schemaless binary transport for EventEnvelope that 
contains event metadata, headers and payload.

### Custom JSON and XML serializers

For consistency, we have customized JAX-RS, Spring Boot and Servlet serialization and exception handlers.

## Reactive design

Mercury uses the temporary local file system (`/tmp`) as an overflow area for events when the consumer is 
slower than the producer. This event buffering design means that user application does not have to handle
back-pressure logic directly.

However, it does not restrict you from implementing your flow-control logic.

## Vertx

In Mercury version 1, the Akka actor system is used as the in-memory event bus.
Since Mercury version 2, we have migrated from Akka to Eclipse Vertx.

In Mercury version 3, we extend the engine to be fully non-blocking with low-level control of application
performance and throughput.

## Java versions

The platform-core library is backward compatible to Java 1.8 so that it can be used for IT modernization of
legacy Java projects.

However, the codebase has been tested with Java 1.8 to 19 for projects, and you can apply the platform-core library
in your projects without JVM constraints.

## Spring Boot

The `platform-core` includes a non-blocking HTTP and websocket server for standalone operation without Spring Boot
or similar application server.

You may also use the platform-core with Spring Boot or other frameworks.

### Customized Spring Boot

The `rest-spring-2-example` project demonstrates the use of the `rest-spring-2` library to build a Spring Boot 2
executable. The `rest-spring-2` is a convenient library with customized Spring Boot serializers and exception handlers.

A corresponding library and example application for Spring Boot version 3 is `rest-spring-3` and
`rest-spring-3-example`.

### Regular Spring Boot

You can also use the platform-core library with a regular Spring Boot application if you prefer.
