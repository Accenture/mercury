# Event orchestration

In traditional programming, we can write modular software components and wire them together as a single application.
There are many ways to do that. You can rely on a "dependency injection" framework. In many cases, you would need
to write orchestration logic to coordinate how the various components talk to each other to process a transaction.

In a composable application, you write modular functions using the first principle of "input-process-output".

Functions communicate with each other using events and each function has a "handleEvent" method to process "input"
and return result as "output". Writing software component in the first principle makes Test Driven Development (TDD)
straight forward. You can write mock function and unit tests before you put in actual business logic.

Mocking an event-driven function in a composable application is as simple as overriding the function's route name
with a mock function.

## Register a function with the in-memory event system

There are two ways to register a function:

1. Programmatic registration
2. Declarative registration

In programmatic registration, you can register a function like this:

```shell
Platform platform = Platform.getInstance();
platform.registerPrivate("my.function", new MyFunction(), 10);
```

In the above example, You obtain a singleton instance of the Platform API class and use the "register" method
to register a private function `MyFunction` with a route name "my.function".

In declarative approach, you use the `PreLoad` annotation to declare a class to hold the event handler.

Your function should implement the LambdaFunction, TypedLambdaFunction or KotlinLambdaFunction. 
While LambdaFunction is untyped, the event system can transport PoJo and your function should
test the object type and cast it to the correct PoJo.

TypedLambdaFunction and KotlinLambdaFunction are typed, and you must declare the input and output classes
according to the input/output API contract of your function.

For example, the SimpleDemoEndpoint has the "PreLoad" annotation to declare the route name and number of worker
instances.

By default, LambdaFunction and TypedLambdaFunction are executed using "kernel thread pool" for the worker instances.
To change a function to a coroutine, you can add the `CoroutineRunner` annotation.

```java
@CoroutineRunner
@PreLoad(route = "hello.simple", instances = 10)
public class SimpleDemoEndpoint implements TypedLambdaFunction<AsyncHttpRequest, Object> {
    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance)
            throws Exception {
        // business logic here
    }
}
```

Once a function is created using the declarative method, you can override it with a mock function by using the
programmatic registration method in a unit test.

## Private vs public functions

When you use the programmatic registration approach, you can use the "register" or the "registerPrivate" method to
register the function as "public" or "private" respectively. For declarative approach, the `PreLoad` annotation
contains a parameter to define the visibility of the function.

```java
// or register it as "public"
platform.register("my.function", new MyFunction(), 10);

// register a function as "private"
platform.registerPrivate("my.function", new MyFunction(), 10);
```

A private function is visible by other functions in the same application memory space.

A public function is accessible by other function from another application instance using service mesh or HTTP method.
We will discuss inter-container communication in [Chapter-7](CHAPTER-7.md) and [Chapter-8](CHAPTER-8.md).

## Post Office API

To send an asynchronous event or an event RPC call from one function to another, you can use the `PostOffice` APIs.

In your function, you can obtain an instance of the PostOffice like this:

```java
@Override
public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance)
        throws Exception {
    PostOffice po = new PostOffice(headers, instance);
    // e.g. po.send and po.asyncRequest for sending asynchronous event and making RPC call
}
```

The PostOffice API detects if tracing is enabled in the incoming requeset. If yes, it will propagate tracing
information to the "downstream" functions.

## Event patterns

1. RPC `“Request-response”, best for interactivity`
2. Asynchronous `e.g. Drop-n-forget`
3. Callback `e.g. Progressive rendering`
4. Pipeline `e.g. Work-flow application`
5. Streaming `e.g. File transfer`

### Request-response (RPC)

In enterprise application, RPC is the most common pattern in making call from one function to another.

The "calling" function makes a request and waits for the response from the "called" function.

In Mercury version 3, there are 2 types of RPC calls - "asynchronous" and "sequential non-blocking".

#### Asynchronous RPC

You can use the `asyncRequest` method to make an asynchronous RPC call. Asynchronous means that the response
will be delivered to the `onSuccess` or `onFailure` callback method.

Note that normal response and exception are sent to the onSuccess method and timeout exception to the onFailure
method.

If you set "timeoutException" to false, the timeout exception will be delivered to the onSuccess callback and
the onFailure callback will be ignored.

```java
Future<EventEnvelope> asyncRequest(final EventEnvelope event, long timeout) 
                                   throws IOException;
Future<EventEnvelope> asyncRequest(final EventEnvelope event, long timeout, 
                                   boolean timeoutException) throws IOException;

// example
EventEnvelope request = new EventEnvelope().setTo(SERVICE).setBody(TEXT);
Future<EventEnvelope> response = po.asyncRequest(request, 2000);
response.onSuccess(result -> {
    // handle the response event
}).onFailure(ex -> {
    // handle timeout exception
});
```

The timeout value is measured in milliseconds.

#### Asynchronous fork-n-join

A special version of RPC is the fork-n-join API. This allows you to make concurrent requests to multiple functions.
The system will consolidate all responses and return them as a list of events.

Normal responses and user defined exceptions are sent to the onSuccess method and timeout exception to the onFailure
method. Your function will receive all responses or a timeout exception.

If you set "timeoutException" to false, partial results will be delivered to the onSuccess method when one or
more services fail to respond on-time. The onFailure method is not required.

```java
Future<List<EventEnvelope>> asyncRequest(final List<EventEnvelope> event, long timeout) 
                                         throws IOException;

Future<List<EventEnvelope>> asyncRequest(final List<EventEnvelope> event, long timeout, 
                                         boolean timeoutException) throws IOException;

// example
List<EventEnvelope> requests = new ArrayList<>();
requests.add(new EventEnvelope().setTo(SERVICE1).setBody(TEXT1));
requests.add(new EventEnvelope().setTo(SERVICE2).setBody(TEXT2));
Future<List<EventEnvelope>> responses = po.asyncRequest(requests, 2000);
responses.onSuccess(events -> {
    // handle the response events
}).onFailure(ex -> {
    // handle timeout exception
});
```

#### Asynchronous programming technique

When your function is a service by itself, asynchronous RPC and fork-n-join require different programming approaches. 

There are two ways to do that:
1. Your function returns an immediate result and waits for the response(s) to the onSuccess or onFailure callback
2. Your function is implemented as an "EventInterceptor"

For the first approach, your function can return an immediate result telling the caller that your function would need
time to process the request. This works when the caller can be reached by a callback.

For the second approach, your function is annotated with the keyword `EventInterceptor`. 
It can immediately return a "null" response that will be ignored by the event system. Your function can inspect
the "replyTo" address, correlation ID, trace ID and trace path in the incoming event and use it to return a future 
response to the caller.

#### Sequential non-blocking RPC and fork-n-join

To simplify coding, you can implement a "suspend function" using the KotlinLambdaFunction interface.

The following code segment illustrates the creation of the "hello.world" function that makes a non-blocking RPC
call to "another.service".

```kotlin
@PreLoad(route="hello.world", instances=10)
class FileUploadDemo: KotlinLambdaFunction<AsyncHttpRequest, Any> {
    override suspend fun handleEvent(headers: Map<String, String>, input: AsyncHttpRequest, 
                                     instance: Int): Any {
        val fastRPC = FastRPC(headers)
        // your business logic here...
        val req = EventEnvelope().setTo("another.service").setBody(myPoJo)
        return fastRPC.awaitRequest(req, 5000)
    }
}
```

The API method signature for non-blocking RPC and fork-n-join are as follows:

```kotlin
@Throws(IOException::class)
suspend fun awaitRequest(request: EventEnvelope, timeout: Long): EventEnvelope

@Throws(IOException::class)
suspend fun awaitRequest(requests: List<EventEnvelope>, timeout: Long): List<EventEnvelope>
```

### Asynchronous drop-n-forget

To make an asynchronous call from one function to another, use the `send` method.

```java
void send(String to, Kv... parameters) throws IOException;
void send(String to, Object body) throws IOException;
void send(String to, Object body, Kv... parameters) throws IOException;
void send(final EventEnvelope event) throws IOException;
```
Kv is a key-value pair for holding one parameter.

Asynchronous event calls are handled in the background without blocking so that your function can continue processing
additional business logic. For example, sending a notification message to a user.

### Callback

You can declare another function as a "callback". When you send a request to another function, you can set the 
"replyTo" address in the request event. When a response is received, your callback function will be invoked to 
handle the response event.

```java
EventEnvelope req = new EventEnvelope().setTo("some.service")
                        .setBody(myPoJo).setReplyTo("my.callback");
po.send(req);
```

In the above example, you have a callback function with route name "my.callback". You send the request event
with a MyPoJo object as payload to the "some.service" function. When a response is received, the "my.callback"
function will get the response as input.

### Pipeline

Pipeline is a linked list of event calls. There are many ways to do pipeline. One way is to keep the pipeline plan
in an event's header and pass the event across multiple functions where you can set the "replyTo" address from the next
task in a pipeline. You should handle exception cases when a pipeline breaks in the middle of a transaction.

An example of the pipeline header key-value may look like this:

```properties
pipeline=service.1, service.2, service.3, service.4, service.5
```

In the above example, when the pipeline event is received by a function, the function can check its position
in the pipeline by comparing its own route name with the pipeline plan.

```java
PostOffice po = new PostOffice(headers, instance);

// some business logic here...
String myRoute = po.getRoute();
```
Suppose myRoute is "service.2", the function can send the response event to "service.3".
When "service.3" receives the event, it can send its response event to the next one. i.e. "service.4".

When the event reaches the last service ("service.5"), the processing will complete.

### Streaming

If you set a function as singleton (i.e. one worker instance), it will receive event in an orderly fashion.
This way you can "stream" events to the function, and it will process the events one by one.

Another means to do streaming is to create an "ObjectStreamIO" event stream like this:

```java
ObjectStreamIO stream = new ObjectStreamIO(60);
ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
out.write(messageOne);
out.write(messageTwo);
out.close();

String streamId = stream.getInputStreamId();
// pass the streamId to another function
```

In the code segment above, your function creates an object event stream and writes 2 messages into the stream
It then obtains the streamId of the event stream. When another function receives the stream ID, it can read
the data blocks orderly.

Note that when you can declare "end of stream" by closing the output stream. If you do not close the stream,
it remains open and idle. If a function is trying to read an input stream using the stream ID, it will time out.

A stream will be automatically closed when the idle inactivity timer is reached. In the above example, 
ObjectStreamIO(60) means an idle inactivity timer of 60 seconds.

> IMPORTANT: To improve the non-blocking design of your function, you can implement your function as a
             KotlinLambdaFunction. If you need to send many blocks of data continuously in a "while"
             loop, you should add the "yield()" statement before it writes a block of data to the 
             output stream. This way, a long-running function will be non-blocking.

There are two ways to read an input event stream - asynchronous or sequential non-blocking.

#### AsyncObjectStreamReader

To read events from a stream, you can create an instance of the AsyncObjectStreamReader like this:

```java
AsyncObjectStreamReader in = new AsyncObjectStreamReader(stream.getInputStreamId(), 8000);
Future<Object> block = in.get();
block.onSuccess(b -> {
    if (b != null) {
        // process the data block
    } else {
        // end of stream. Do additional processing.
        in.close();
    }
});
```

The above illustrates reading the first block of data. The function would need to iteratively read the stream
until end of stream (i.e. when the stream returns null). As a result, asynchronous application code for stream
processing is more challenging to write and maintain.

#### Sequential non-blocking method

The industry trend is to use sequential non-blocking method instead of "asynchronous programming" because your code
will be much easier to read.

You can use the `awaitRequest` method to read the next block of data from an event stream.

An example for reading a stream is shown in the `FileUploadDemo` kotlin class in the lambda-example project.
It is using a simple "while" loop to read the stream. When the function fetches the next block of data using
the `awaitRequest` method, the function is suspended until the next data block or "end of stream" signal is received.

It may look like this:

```kotlin
val po = PostOffice(headers, instance)
val fastRPC = FastRPC(headers)

val req = EventEnvelope().setTo(streamId).setHeader(TYPE, READ)
while (true) {
    val event = fastRPC.awaitRequest(req, 5000)
    if (event.status == 408) {
        // handle input stream timeout
        break
    }
    if (EOF == event.headers[TYPE]) {
        po.send(streamId, Kv(TYPE, CLOSE))
        break
    }
    if (DATA == event.headers[TYPE]) {
        val block = event.body
        if (block is ByteArray) {
            // handle the data block from the input stream
        }
    }
}
```

Since the code style is "sequential non-blocking", using a "while" loop does not block the "event loop" provided
that you are using "await" API inside the while-loop.

In this fashion, the intent of the code is clear. Sequential non-blocking method offers high throughput because
it does not consume CPU resources while the function is waiting for a response from another function.

We recommend sequential non-blocking style for more sophisticated event streaming logic.

> Note: "await" methods are only supported in KotlinLambdaFunction which is a suspend function.
        When Java 19 virtual thread feature becomes officially available, we will enhance
        the function execution strategies.

## Orchestration layer

Once you have implemented modular functions in a self-contained manner, the best practice is to write one or more
functions to do "event orchestration".

Think of the orchestration function as a music conductor who guides the whole team to perform.

For event orchestration, your function can be the "conductor" that sends events to the individual functions so that
they operate together as a single application. To simplify design, the best practice is to apply event orchestration
for each transaction or use case. The event orchestration function also serves as a living documentation about how
your application works. It makes your code more readable.

## Event Script

To simplify and automate event orchestration, there is an enterprise add-on module called "Event Script".
This is the idea of "config over code" or "declarative programming". The primary purpose of "Event Script"
is to reduce coding effort so that the team can focus their energy in improving application design and code quality. 
Please contact your Accenture representative if you would like to evaluate the additional tool.

In the next chapter, we will discuss the build, test and deploy process.
<br/>

|            Chapter-3            |                   Home                    |               Chapter-5                |
|:-------------------------------:|:-----------------------------------------:|:--------------------------------------:|
| [REST automation](CHAPTER-3.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [Build, test and deploy](CHAPTER-5.md) |
