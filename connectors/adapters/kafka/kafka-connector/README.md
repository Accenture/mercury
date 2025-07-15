# Kafka Adapter feature

This connector is updated with a Kafka Adapter as a service

The Kafka Adapter contains two parts:
1. Kafka Listener - this module handles incoming message from a Kafka topic
2. Kafka Notification - this module allows you to send messages to any Kafka topic

## Kafka adapter configuration

To use the new Kafka Adapter, you must set the "cloud.connector" parameter to "kafka-adapter"
in application.properties.

```properties
cloud.connector=kafka-adapter
```

and create a `kafka-adapter.yaml` configuration file. An example of the configuration file is show below:

```yaml
consumer:
  - topic: 'hello.world'
    target: 'demo.listener'
    group: 'group-100'
    tracing: true
  - topic: 'hello.notice'
    target: 'notice.listener'
    group: 'group-100'
    tracing: true

producer.enabled: true

#
# automatically create the following topics if not present
#
create:
  - topic: 'hello.world'
    partition: 10
    replication: 1
  - topic: 'hello.notice'
    partition: 5
    replication: 1

```

The "create" section tells the system to create topics if they are not available in the system
Note that some organizations do not allow programmatic creation of topics. In this case, disable
this section by using the following:

```yaml
create: []
```

To enable Kafka notification service, set `producer.enabled` to true.

To configure services to listen to topics, use the "consumer" section.

In the above example, there are two topics to listen.

Incoming messages from the topic "hello.world" will be routed to the function "demo.listener"
and message from "hello.notice" to the function "notice.listener".

Note that a function is addressable by a route name.

The "group" is used to tell the system which "consumer group ID" to use.

When "tracing" is turned on, the system will track the events on an end-to-end basis.

## Sending messages to a kafka topic

The function that is responsible for sending messages to Kafka topics is callable using
the route "kafka.notification".

To send a message to a topic, do the following:

```java
Map<String, Object> map = new HashMap<>();
map.put("hello", "world");
Map<String, Object> content = new HashMap<>();
content.put("content", map);
po.send("kafka.notification", content, new Kv("topic", "hello.world"), new Kv("x-content-type", "json"));
```

In the above example, you are sending a map ("hello": "world") as message content to the topic "hello.world".

The interface contract for "kafka.notification" is:

1. header.topic - name of a Kafka topic
2. header.x-content-type - json or event
3. content - a hashmap containing a map inside the "content" key

If the x-content-type is json, the system will transport the map key-value in 'content' as a JSON string
that is encoded as byte array in a Kafka message.

If the x-content-type is event, the system will transport an event envelope as a byte array in the 'content' section.

The "json" content type method is convenient to transport a simple map of key-values.

The "event" content type method is more efficient and it can accommodate PoJo, primitive and hashmap content.

## Unit tests

The use case of handling inbound and outbound messages for Kafka topics are illustrated in the unit test
"KafkaAdapterTest". Please review the `pubSubTest` method for details.

To try the unit test, please run a standalone Kafka server in your development machine before running the
unit test. A convenient standalone Kafka server is available in the kafka-standalone project under the
"connnectors/adapter/kafka/kafka-standalone" folder.
