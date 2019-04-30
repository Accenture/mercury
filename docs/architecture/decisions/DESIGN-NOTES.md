# Serialization engines

## Gson

We are using Gson for its minimalist design.

We have customized its serialization behavior to be in line with Jackson and other serializers. i.e. Integer and long values are kept without decimal points.

For backward compatibility with Jackson, we have added the writeValueAsString, writeValueAsBytes and readValue methods. The convertValue method has been consolidated into the readValue method.

Custom serialization annotations are discouraged.

## MsgPack

MsgPack is used as the serialization engine from Map to byte array and vice versa because we use Map as the intermediate object for events.

We use it as a schemaless binary transport for EventEnvelope, a vehicle for event metadata, headers and payload.

## Abstraction layer

The SimpleMapper and EventEnvelope classes are used to hide the complexity of serialization.

## Custom JSON and XML serializers for JAX-RS and Spring Boot

For the `rest-spring` project, we have customized JAX-RS, Spring Boot and Servlet serialization and exception handlers for consistency.

# Reactive design

The simplest reactive design is to use the temporary file system as an overflow area for events in case the consumer are slower than the producer.

# Akka

Akka actor is used as the in-memory event bus. We may migrate to Java 1.9 Flow API or reactive stream at a later time.

# Java Futures

We use Java Future for managing thread pool that is used to execute concurrent service functions.
