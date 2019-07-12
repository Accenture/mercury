# Post Office API

Post Office is a platform abstraction layer that routes events among functions. It maintains a distributed routing table to ensure that service discovery is instantaneous,

## Obtain an instance of the post office object

```
PostOffice po = PostOffice.getInstance();
```

## Communication patterns

- RPC `“Request-response”, best for interactivity`
- Asynchronous `e.g. Drop-n-forget and long queries`
- Call-back `e.g. Progressive rendering`
- Pipeline `e.g. Work-flow application`
- Streaming `e.g. Data ingest`
- Broadcast `e.g. Concurrent processing of the same dataset with different outcomes`

### RPC (Request-response)

The Mercury framework is 100% event-driven and all communications are asynchronous. To emulate a synchronous RPC, it suspends the calling function and uses temporary Inbox as a callback function. The called function will send the reply to the callback function which in turns wakes up the calling function.

To make a RPC call, you can use the `request` method.

```java
EventEnvelope request(String to, long timeout, Object body) throws IOException, TimeoutException, AppException;
EventEnvelope request(String to, long timeout, Object body, Kv... parameters) throws IOException, TimeoutException, AppException;
EventEnvelope request(EventEnvelope event, long timeout) throws IOException, TimeoutException, AppException;

// example
EventEnvelope response = po.request("hello.world", 1000, somePayload);
System.out.println("I got response..."+response.getBody());

```

Note that Mercury supports Java primitive, Map and PoJo in the message body. If you put other object, it may throw serialization exception or the object may become empty.

### Asynchronous / Drop-n-forget

To make an asynchronous call, use the `send` method.

```java
void send(String to, Kv... parameters) throws IOException;
void send(String to, Object body) throws IOException;
void send(String to, Object body, Kv... parameters) throws IOException;
void send(final EventEnvelope event) throws IOException;

```
Kv is a key-value pair for holding one parameter.

### Deferred delivery

```java
String sendLater(final EventEnvelope event, Date future) throws IOException;
```
A scheduled ID will be returned. You may cancel the scheduled delivery with `cancelFutureEvent(id)`.

### Call-back

You can register a call back function and uses its route name as the "reply-to" address in the send method. To set a reply-to address, you need to use the EventEnvelope directly.

```java
void send(final EventEnvelope event) throws IOException;

// example
EventEnvelope event = new EventEnvelope();
event.setTo("hello.world").setBody(somePayload);
po.send(event);
```

### Pipeline

In a pipeline operation, there is stepwise event propagation. e.g. Function A sends to B and set the "reply-to" as C. Function B sends to C and set the "reply-to" as D, etc.

To pass a list of stepwise targets, you may send the list as a parameter. Each function of the pipeline should forward the pipeline list to the next function.

```java
EventEnvelope event = new EventEnvelope();
event.setTo("function.b").setBody(somePayload).setReplyTo("function.c")
     .setHeader("pipeline",  "function.a->function.b->function.c->function.d");
po.send(event);
```

### Streaming

You can use streams for functional programming. There are two ways to do streaming.

1. Singleton functions

To create a singleton, you can set `instances` of the calling and called functions to 1. When you send events from the calling function to the called function, the platform guarantees that the event sequencing of the data stream.

To guarantee that there is only one instance of the calling and called function, you should register them with a globally unique route name. e.g. using UUID like "producer-b351e7df-827f-449c-904f-a80f9f3ecafe" and "consumer-d15b639a-44d9-4bc2-bb54-79db4f866fe3".

Note that you can programmatically `register` and `release` a function at run-time.

If you create the functions at run-time, please remember to release the functions when processing is completed to avoid wasting system resources.

2. Object stream

To do object streaming, you can use the ObjectStreamIO to create a stream or open an existing stream.
Then, you can use the `ObjectStreamWriter` and the `ObjectStreamReader` classes to write to and read from the stream.

For the producer, if you close the output stream, the system will send a `EOF` to signal that that there are no more events to the stream. 

For the consumer, When you detect the end of stream, you can close the input stream to release the stream and all resources associated with it.

I/O stream consumes resources and thus you must close the input stream at the end of stream processing.
The system will automatically close the stream upon an expiry timer that you provide when a new stream is created.

The following unit test demonstrates this use case.

```java
String messageOne = "hello world";
String messageTwo = "it is great";
/*
 * Producer creates a new stream with 60 seconds inactivity expiry
 */
ObjectStreamIO producer = new ObjectStreamIO(60);
ObjectStreamWriter out = producer.getOutputStream();
out.write(messageOne);
out.write(messageTwo);
/*
 * If output stream is closed, it will send an EOF signal so that the input stream reader will detect it.
 * Otherwise, input stream reader will see a RuntimeException of timeout.
 *
 * For this test, we do not close the output stream to demonstrate the timeout.
 */
//  out.close();

/*
 * See all open streams in this application instance and verify that the new stream is there
 */
String streamId = producer.getRoute();
// remove the node-ID from the fully qualified route name
String path = streamId.substring(0, streamId.indexOf('@'));
Map<String, Object> localStreams = producer.getLocalStreams();
assertTrue(localStreams.containsKey(path));

/*
 * Producer should send the streamId to the consumer.
 * The consumer can then open the existing stream with the streamId.
 */
ObjectStreamIO consumer = new ObjectStreamIO(streamId);
/*
 * read object from the event stream
 * (minimum timeout value is one second)
 */
ObjectStreamReader in = consumer.getInputStream(1000);
int i = 0;
while (!in.isEof()) {
    try {
        for (Object data : in) {
            i++;
            if (i == 1) {
                assertEquals(messageOne, data);
            }
            if (i == 2) {
                assertEquals(messageTwo, data);
            }
        }
    } catch (RuntimeException e) {
        // iterator will timeout since the stream was not closed
        assertTrue(e.getMessage().contains("timeout"));
        assertTrue(in.isPending());
        break;
    }
}
// ensure that it has read the two messages
assertEquals(2, i);
// must close input stream to release resources
in.close();
```

### Broadcast

Broadcast is the easiest way to do "pub/sub". To broadcast an event to multiple application instances, use the `broadcast` method.

```java
void broadcast(String to, Kv... parameters) throws IOException;
void broadcast(String to, Object body) throws IOException;
void broadcast(String to, Object body, Kv... parameters) throws IOException;

// example
po.broadcast("hello.world", "hey, this is a broadcast message to all hello.world providers");

```

### Join-n-fork

You can perform join-n-fork RPC calls using a parallel version of the `request` method.

```java
List<EventEnvelope> request(List<EventEnvelope> events, long timeout) throws IOException;

// example
List<EventEnvelope> parallelEvents = new ArrayList<>();

EventEnvelope event1 = new EventEnvelope();
event1.setTo("hello.world.1");
event1.setBody(payload1);
event1.setHeader("request", "#1");
parallelEvents.add(event1);

EventEnvelope event2 = new EventEnvelope();
event2.setTo("hello.world.2");
event2.setBody(payload2);
event2.setHeader("request", "#2");
parallelEvents.add(event2);

List<EventEnvelope> responses = po.request(parallelEvents, 3000);
```

### Pub/Sub for store-n-forward event streaming

Native Pub/Sub will be automatically enabled if the underlying cloud connector supports it. e.g. Kafka.

Mercury provides real-time inter-service event streaming and you do not need to deal with low-level messaging.

However, if you want to do store-n-forward pub/sub for certain use cases, you may use the `PubSub` class.
Following are some useful pub/sub API:

```java
public boolean featureEnabled();
public boolean createTopic(String topic) throws IOException;
public void deleteTopic(String topic) throws IOException;
public void publish(String topic, Map<String, String> headers, Object body) throws IOException;
public void subscribe(String topic, LambdaFunction listener, String... parameters) throws IOException;
public void unsubscribe(String topic) throws IOException;
public boolean exists(String topic) throws IOException;
public List<String> list() throws IOException;

```
Some pub/sub engine would require additional parameters when subscribing a topic. For Kafka, you must provide the following parameters

1. clientId
2. groupId
3. optional read offset pointer

If the offset pointer is not given, Kafka will position the read pointer to the latest when the clientId and groupId are first seen.
Thereafter, Kafka will remember the read pointer for the groupId and resume read from the last read pointer.

As a result, for proper subscription, you must create the topic first and then create a lambda function to subscribe to the topic before publishing anything to the topic.

To read the event stream of a topic from the beginning, you can set offset to "0".

The system encapsulates the headers and body (aka payload) in an event envelope so that you do not need to do serialization yourself.
The payload can be PoJo, Map or Java primitives.

### Check if a target service is available

To check if a target service is available, you can use the `exists` method.

```java
boolean po.exists(String... route);

// input can be a single route or multiple routes
// it will return true only when all routes are available
// for examples

if (po.exists("hello.world")) {
    // do something
}

if (po.exists("hello.math", "v1.diff.equation")) {
    // do other things
}

```
This service discovery process is instantaneous using distributed routing table.


| Chapter-4                                 | Home                                     |
| :----------------------------------------:|:----------------------------------------:|
| [REST and websocket](CHAPTER-4.md)        | [Table of Contents](TABLE-OF-CONTENTS.md)|
