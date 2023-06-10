# Migration guide

Let's discuss some migration tips from Mercury version 2 to 3.

# Breaking changes

Mercury version 3 is a fully non-blocking event system.

If you are using Mercury version 2 for production, please note that version 2 codebase has been archived
to the "release/v2-8-0" branch.

To enjoy rapid software development and higher application performance and throughput, we recommend porting
your application to Mercury version 3 as soon as possible.

The following are the breaking changes that require some code refactoring:

1. Retired blocking APIs - the "po.request" methods for RPC have been replaced by the new "FastRPC" APIs.
2. Distributed tracing - a new "PostOffice" class is available for compatibility with coroutine.
3. Support three function execution strategies - kernel thread pool, coroutine and suspend function.

We understand the inconvenience of a major release upgrade in production environment. We believe that the benefits
would out-weight the refactoring effort. Your new code will be composable, easier to read and faster.

Writing code with Mercury version 3 platform-core is straight forward. For example, you can make a Java function into
a "coroutine" with the annotation `CoroutineRunner`.

To write a suspend function, you can use IDE (JetBrains Intellij) automated code conversion to copy-n-paste 
Java statements into a KotlinLambdaFunction. This is the easiest way to port code. The conversion accuracy is high. 
With some minor touch up, you would get your new functions up and running quickly.

# Step-by-step upgrade

## Global replace of "PostOffice" to "EventEmitter"

The old PostOffice has been renamed as "EventEmitter". You can do a "global search and replace" to change
the class name.

## Fix broken code for RPC calls

Since blocking APIs have been removed, the original PostOffice's "request" methods are no longer available.

There are two ways to refactor the RPC calls.

### Convert code to asynchronous RPC calls

You can use the "asyncRequest" methods for RPC and fork-n-join. Since the asyncRequest's result is a "Future" object.
You must implement the "onSuccess" and optionally the "onFailure" logic blocks.

Since your new code is asynchronous, the function will immediately return before a future response arrives.

If your function may be called by another function, this would break your code. For this use case, you can annotate
your function as an "EventInterceptor" and return a dummy "null" value.

As an EventInterceptor, you can inspect metadata of the incoming event to retrieve the "replyTo" and "correlationId".

```java
@EventInterceptor
@PreLoad(route="my.function", instances=10)
public class MyFunction implements TypedLambdaFunction<EventEnvelope, Void> {
    @Override
    public Void handleEvent(Map<String, String> headers, EventEnvelope input, int instance) {
        PostOffice po = new PostOffice(headers, instance);
        // make asyncRequest RPC call
        EventEnvelope request = new EventEnvelope().setTo("some.target.service")
                                        .setBody(input.getBody());
        po.asyncRequest(request, 5000)
                .onSuccess(result -> {
                    String replyTo = input.getReplyTo();
                    String correlationId = input.getCorrelationId();
                    if (replyTo != null && correlationId != null) {
                        EventEnvelope response = new EventEnvelope();
                        response.setTo(replyTo).setBody(result.getBody())
                                .setCorrelationId(correlationId);
                        po.send(response);
                    }
                });
        return null;
    }
}
```

In the above example, "my.function" will immediately return a dummy "null" value which will be ignored by the
event system.

When it receives a response from a downstream service, it can return result to the upstream service by
asynchronously sending a response.

## Convert RPC code to a suspend function

You can convert your function containing RPC calls to a suspend function using the KotlinLambdaFunction interface.

It may look like this:

```kotlin
@PreLoad(route="my.function", instances=10)
class MyFunction: KotlinLambdaFunction<EventEnvelope, Any> {
    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, 
                                     instance: Int): Any {
        val fastRPC = FastRPC(headers)
        val request = EventEnvelope().setTo("some.target.service").setBody(input.body)
        return fastRPC.awaitRequest(request, 5000)
    }
}
```

The above example serves the same purpose as the asynchronous "my.function" earlier.
The code is much easier to read because it is expressed in a sequential manner.

Sequential non-blocking code communicates the intent clearly, and we highly recommend this coding style.

If you are new to Kotlin, you may want to leverage the Intellij IDE automated code conversion feature.

Just create a dummy Java class as a sketchpad. Write your code in Java and copy-n-paste the Java statements
into the new Kotlin class. The IDE will convert the code automatically. The code conversion is highly accurate.
With some minor touch up, your new code will be up and running quickly.

## The new PostOffice API

The new PostOffice class is backward compatible with the original asynchronous RPC and fork-n-join methods.

You can obtain an instance of the PostOffice API in the "handleEvent" method of your function.

The PostOffice constructor takes function route name, optional trace ID and path from the headers of the incoming
event. These are READ only metadata inserted by the event system. It also needs the worker instance number to
track the current transaction.

```java
@Override
public Map<String, Object> handleEvent(Map<String, String> headers, 
                                      EventEnvelope event, int instance) {
    PostOffice po = new PostOffice(headers, instance);
    // your business logic here
}
```

When you use the PostOffice to send events or make RPC calls to other functions, the system can propagate
distributed tracing information along the transaction flow automatically.

## The new FastRPC API

The non-blocking "awaitRequest" methods for RPC and fork-n-join are available in a new FastRPC kotlin class.
The constructor is similar to the PostOffice.

```kotlin
val fastRPC = FastRPC(headers)
```

## Distributed tracing

The new PostOffice and FastRPC will propagate distributed tracing information along multiple functions in
a transaction path. It will automatically detect if "tracing" is enabled for a transaction.

## AsyncHttpClient service

In Mercury version 3, the "async.http.request" function can be used as a non-blocking HTTP client.

To make an HTTP request to an external REST endpoint, you can create an HTTP request object using the `AsyncHttpRequest`
class and make an async RPC call to the "async.http.request" function like this:

```java
PostOffice po = new PostOffice(headers, instance);
AsyncHttpRequest req = new AsyncHttpRequest();
req.setMethod("GET");
req.setHeader("accept", "application/json");
req.setUrl("/api/hello/world?hello world=abc");
req.setQueryParameter("x1", "y");
List<String> list = new ArrayList<>();
list.add("a");
list.add("b");
req.setQueryParameter("x2", list);
req.setTargetHost("http://127.0.0.1:8083");
EventEnvelope request = new EventEnvelope().setTo("async.http.request").setBody(req);
Future<EventEnvelope> res = po.asyncRequest(request, RPC_TIMEOUT);
res.onSuccess(response -> {
   // do something with the result 
});
```

In a suspend function using KotlinLambdaFunction, the same logic may look like this:

```kotlin
val req = AsyncHttpRequest()
req.setMethod("GET")
req.setHeader("accept", "application/json")
req.setUrl("/api/hello/world?hello world=abc")
req.setQueryParameter("x1", "y")
val list: MutableList<String> = ArrayList()
list.add("a")
list.add("b")
req.setQueryParameter("x2", list)
req.setTargetHost("http://127.0.0.1:8083")
val request = EventEnvelope().setTo("some.target.service").setBody(req)
val response = fastRPC.awaitRequest(request, 5000)
// do something with the result
```

There is virtually no performance difference between the asynchronous approach and sequential non-blocking style.
However, the latter demands less CPU resources and yields higher throughput.

# Kernel thread pool

A Java function implementing the LambdaFunction or TypedLambdaFunction will be executed using a kernel thread pool.

When using a kernel thread pool, please reduce the number of concurrent worker instances when you register 
your function.

You can register your function using the `PreLoad` annotation. For on-demand functions, you can programmatically 
register the function using the platform APIs.

Java provides preemptive multitasking using kernel threads. It offers the highest performance in terms of
operations per second. If your function is computational intensive and long-running, this function execution
strategy is ideal.

However, please be reminded that kernel thread pool is a finite resources and thus an application should not run too
many concurrent kernel threads. The context switching overheads would significantly reduce overall performance 
when the number of concurrent kernel threads exceed the available CPU power. A rule of thumb is to keep the number
of concurrent kernel threads to around 100.

## Coroutine

For some functions that are not computational intensive and do not make RPC calls, you can declare the function 
as a coroutine using the `CoroutineRunner` annotation.

This tells the system to execute the function in the event loop, thus reducing CPU load.

## Suspend function

For a function that make RPC calls, we would recommend writing it as a suspend function using the KotlinLambdaFunction
interface. This yields higher throughput to support more concurrent users and sessions.

## Things to avoid

You should avoid blocking methods in your functions. For example, the "Synchronous" keyword, Object wait and lock,
"Thread" sleep method, BlockingQueue, etc.

The non-blocking "delay" API is a direct replacement of the "Thread.sleep" method. You can also use the PostOffice's
"sendLater" API to schedule an event.

## Conclusion

While Mercury has been enhanced from the ground up, the core APIs are intact. The main breaking change is the
removal of blocking RPC APIs. You should leverage IDE automated code conversion to reduce migration risks.

The three function execution strategies would provide low-level control of how your application runs, making
performance tuning more scientific.
<br/>

|          Chapter-9           |                   Home                    |               Appendix-I                |
|:----------------------------:|:-----------------------------------------:|:---------------------------------------:|
| [API overview](CHAPTER-9.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [application.properties](APPENDIX-I.md) |