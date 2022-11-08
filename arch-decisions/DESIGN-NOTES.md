# Serialization engines

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

We use it as a schemaless binary transport for EventEnvelope, a vehicle for event metadata, headers and payload.

## Abstraction layer

The SimpleMapper and EventEnvelope classes are used to hide the complexity of serialization.

## Custom JSON and XML serializers for JAX-RS and Spring Boot

For the `rest-spring` project, we have customized JAX-RS, Spring Boot and Servlet serialization and exception 
handlers for consistency.

# Reactive design

Mercury uses the temporary local file system as an overflow area for events when the consumer is 
slower than the producer. Normally, user application does not have to handle back-pressure.

However, user application may also control back-pressure by implementing alternative flow-control mechanism.

# Vertx

Akka actor is used as the in-memory event bus in Mercury version 1.0.

Since Mercury 2.0, we have migrated from Akka to Eclipse Vertx as the in-memory event system.

# Non-blocking design

Under the hood, Mercury is non-blocking and event-driven. 
It uses a temporary inbox to emulate synchronous RPC and fork-n-join.
However, a running function would consume a thread in synchronous RPC.
Therefore, please avoid using "nested" synchronous RPC in your application.

To reduce memory footprint and thread usage, please use the non-blocking version of 
RPC and fork-n-join that return as Vertx Future.
