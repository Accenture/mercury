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

```
EventEnvelope request(String to, long timeout, Object body) throws IOException, TimeoutException, AppException;
EventEnvelope request(String to, long timeout, Object body, Kv... parameters) throws IOException, TimeoutException, AppException;
EventEnvelope request(EventEnvelope event, long timeout) throws IOException, TimeoutException, AppException;

e.g.
EventEnvelope response = po.request("hello.world", 1000, somePayload);
System.out.println("I got response..."+response.getBody());

```

Note that Mercury supports Java primitive, Map and PoJo in the message body. If you put other object, it may throw serialization exception or the object may become empty.

### Asynchronous / Drop-n-forget

To make an asynchronous call, use the `send` method.

```
void send(String to, Kv... parameters) throws IOException;
void send(String to, Object body) throws IOException;
void send(String to, Object body, Kv... parameters) throws IOException;
void send(final EventEnvelope event) throws IOException;

```
Kv is a key-value pair for holding one parameter.

### Call-back

You can register a call back function and uses its route name as the "reply-to" address in the send method. To set a reply-to address, you need to use the EventEnvelope directly.

```
void send(final EventEnvelope event) throws IOException;

e.g.
EventEnvelope event = new EventEnvelope();
event.setTo("hello.world").setBody(somePayload);
po.send(event);
```

### Pipeline

In a pipeline operation, there is stepwise event propagation. e.g. Function A sends to B and set the "reply-to" as C. Function B sends to C and set the "reply-to" as D, etc.

To pass a list of stepwise targets, you may send the list as a parameter. Each function of the pipeline should forward the pipeline list to the next function.

```
EventEnvelope event = new EventEnvelope();
event.setTo("function.b").setBody(somePayload).setReplyTo("function.c").setHeader("pipeline", "function.a->function.b->function.c->function.d");
po.send(event);
```

### Streaming

You can use streams for functional programming. There are two ways to do streaming.

1. Singleton functions

To create a singleton, you can set `instances` of the calling and called functions to 1. When you send events from the calling function to the called function, the platform guarantees that the event sequencing of the data stream.

To guarantee that there is only one instance of the calling and called function, you should register them with a globally unique route name. e.g. using UUID like "producer-b351e7df-827f-449c-904f-a80f9f3ecafe" and "consumer-d15b639a-44d9-4bc2-bb54-79db4f866fe3".

Note that you can programmatically `register` and `release` a function at run-time.

If you create the functions at run-time, please remember to release the functions when processing is completed to avoid wasting system resources.

2. Event stream

To do event streaming, you can ask the Event Manager (route=`system.streams.manager`) to create a stream and then use the `ObjectStreamWriter` and the `ObjectStreamReader` classes to write to and read from the stream.

Note that if you close the output stream, the manager will send a `EOF` to signal that that there are no more events to the stream. When you detect the end of stream, you can close the input stream. When you close the input stream, you will release the stream and all resources associated with it.

I/O stream consumes resources and thus you must close the input stream at the end of stream processing.

The following unit test demonstrates this use case.

```
String messageOne = "hello world";
String messageTwo = "it is great";

PostOffice po = PostOffice.getInstance();
EventEnvelope response = po.request(STREAM_MANAGER, 5000, new Kv("type", "create"));
assertFalse(response.hasError());
assertTrue(response.getBody() instanceof String);

String fqPath = (String) response.getBody();
assertTrue(fqPath.startsWith("stream."));
// fully qualified path = streamId @ origin
assertTrue(fqPath.contains("@"));

ObjectStreamWriter out = new ObjectStreamWriter(fqPath);
out.write(messageOne);
out.write(messageTwo);
// do not close output stream to demonstrate that the iterator will timeout during read
// out.close();

ObjectStreamReader in = new ObjectStreamReader(fqPath, 1000);
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
    } catch (IllegalArgumentException e) {
        // iterator will timeout since the stream was not closed
        assertTrue(e.getMessage().contains("timeout"));
        assertTrue(in.isPending());
        break;
    }
}
// must close input stream to release resources
in.close();

```

### Broadcast

Broadcast is the easiest way to do "pub/sub". To broadcast an event to multiple application instances, use the `broadcast` method.

```
void broadcast(String to, Kv... parameters) throws IOException;
void broadcast(String to, Object body) throws IOException;
void broadcast(String to, Object body, Kv... parameters) throws IOException;

e.g.
po.broadcast("hello.world", "hey, this is a broadcast message to all hello.world providers");

```

### Join-n-fork

You can perform join-n-fork RPC calls using a parallel version of the `request` method.

```
List<EventEnvelope> request(List<EventEnvelope> events, long timeout) throws IOException;

e.g.
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

### Check if a target service is available

To check if a target service is available, you can use the `exists` method.

```
boolean po.exists(String route);

e.g.
if (po.exists("hello.world")) {
    // do something
}

This service discovery process should be instantaneous.

```


| Chapter-4                                 | Home                                     |
| :----------------------------------------:|:----------------------------------------:|
| [REST and websocket](CHAPTER-4.md)        | [Table of Contents](TABLE-OF-CONTENTS.md)|
