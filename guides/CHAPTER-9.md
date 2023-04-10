# API overview

## Main application

Each application has an entry point. You may implement an entry point in a main application like this:

```java
@MainApplication
public class MainApp implements EntryPoint {
   public static void main(String[] args) {
      AppStarter.main(args);
   }
   @Override
   public void start(String[] args) {
        // your startup logic here
      log.info("Started");
   }
}
```

In your main application, you will implement the `EntryPoint` interface that creates the "start" method.
Typically, a main application is used to initiate some application start up procedure.

In some case when your application does not need any start up logic, you can just print a message to indicate
that your application has started.

You may want to keep the static "main" method which can be used to run your application inside an IDE.

The pom.xml build script is designed to run the `AppStarter` start up function that will execute your main
application's start method.

In some case, your application may have more than one main application module. You can decide the sequence of
execution using the "sequence" parameter in the `MainApplication` annotation. The module with the smallest sequence
number will run first.

## Optional environment setup before MainApplication

Sometimes, it may be required to set up some environment configuration before your main application starts.
You can implement a `BeforeApplication` module. Its syntax is similar to the `MainApplication`.

```java
@BeforeApplication
public class EnvSetup implements EntryPoint {

   @Override
   public void start(String[] args) {
        // your environment setup logic here
      log.info("initialized");
   }
}
```

The `BeforeApplication` logic will run before your `MainApplication` module. This is useful when you want to do
special handling of environment variables. For example, decrypt an environment variable secret, construct a X.509
certificate, and save it in the "/tmp" folder before your main application starts.

## Event envelope

Mercury version 3 is an event-driven engine that encapsulates Eclipse Vertx and Kotlin coroutine and suspend function.

A composable application is a collection of functions that communicate with each other in events.
Each event is transported by an event envelope. Let's examine the envelope.

There are 3 elements in an event envelope:

| Element | Type     | Purpose                                                                                                           |
|:-------:|:---------|:------------------------------------------------------------------------------------------------------------------|
|    1    | metadata | Includes unique ID, target function name, reply address<br/> correlation ID, status, exception, trace ID and path |
|    2    | headers  | User defined key-value pairs                                                                                      |
|    3    | body     | Event payload (primitive, hash map or PoJo)                                                                       |

Headers and body are optional, but you must provide at least one of them. If the envelope do not have any headers
or body, the system will send your event as a "ping" command to the target function. The response acknowledgements
that the target function exists. This ping/pong protocol tests the event loop or service mesh. This test mechanism
is useful for DevSecOps admin dashboard.

## Custom exception using AppException

To reject an incoming request, you can throw an AppException like this:

```java
// example-1
throw new AppException(400, "My custom error message");
// example-2
throw new AppException(400, "My custom error message", ex);
```

Example-1 - a simple exception with status code (400) and an error message.

Example-2 - includes a nested exception (ex)

As a best practice, we recommend using error codes that are compatible with HTTP status codes.

## Defining a user function in Java

You can write a function in Java like this:

```java
@PreLoad(route = "hello.simple", instances = 10)
public class SimpleDemoEndpoint implements TypedLambdaFunction<AsyncHttpRequest, Object> {
    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) {
        // business logic here
    }
}
```

By default, a Java function will run using a kernel thread. To tell the system that you want to run the function as
a coroutine, you can add the `CoroutineRunner` annotation.

The `PreLoad` annotation tells the system to preload the function into memory and register it into the event loop.
You must provide a "route name" and configure the number of concurrent workers ("instances").

Route name is used by the event loop to find your function in memory. A route name must use lower letters and numbers,
and it must have at least one dot as a word separator. e.g. "hello.simple" is a proper route name but "HelloSimple" 
is not.

You can implement your function using the LambdaFunction or TypedLambdaFunction. The latter allows you to define
the input and output classes.

The system will map the event body into the `input` argument and the event headers into the `headers` argument.
The `instance` argument informs your function which worker is serving the current request.

Similarly, you can also write a "suspend function" in Kotlin like this:

```kotlin
@PreLoad(route = "hello.world", instances = 10, isPrivate = false, 
         envInstances = "instances.hello.world")
class HelloWorld : KotlinLambdaFunction<Any?, Map<String, Any>> {

    @Throws(Exception::class)
    override suspend fun handleEvent(headers: Map<String, String>, input: Any?, 
                                     instance: Int): Map<String, Any> {
        // business logic here
    }
}
```

In the suspend function example above, you may notice the optional `envInstances` parameter. This tells the system
to use a parameter from the application.properties (or application.yml) to configure the number of workers for the
function. When the parameter defined in "envInstances" is not found, the "instances" parameter is used as the
default value.

## Inspect event metadata

There are some reserved metadata for route name ("my_route"), trace ID ("my_trace_id") and trace path ("my_trace_path")
in the "headers" argument. They do not exist in the incoming event envelope. Instead, the system automatically
insert them as read-only metadata.

They are used when your code want to obtain an instance of PostOffice or FastRPC.

To inspect all metadata, you can declare the input as "EventEnvelope". The system will map the whole event envelope
into the "input" argument. You can retrieve the replyTo address and other useful metadata.

Note that the "replyTo" address is optional. It only exists when the caller is making an RPC call to your function.
If the caller sends an asynchronous request, the "replyTo" value is null.

## Platform API

You can obtain a singleton instance of the Platform object to do the following:

### Register a function

We recommend using the `PreLoad` annotation in a class to declare the function route name, number of worker instances
and whether the function is public or private.

In some use cases where you want to create and destroy functions on demand, you can register them programmatically.

In the following example, it registers "my.function" using the MyFunction class as a public function and 
"another.function" with the AnotherFunction class as a private function. It then registers two kotlin functions
in public and private scope respectively.

```java
Platform platform = Platform.getInstance();

// register a public function
platform.register("my.function", new MyFunction(), 10);

// register a private function
platform.registerPrivate("another.function", new AnotherFunction(), 20);

// register a public suspend function
platform.registerKoltin("my.suspend.function", new MySuspendFunction(), 10);

// register a private suspend function
platform.registerKoltinPrivate("another.suspend.function", new AnotherSuspendFunction(), 10);
```

### What is a public function?

A public function is visible by any application instances in the same network. When a function is declared as "public",
the function is reachable through the EventAPI REST endpoint or a service mesh.

A private function is invisible outside the memory space of the application instance that it resides.
This allows application to encapsulate business logic according to domain boundary. You can assemble closely
related functions as a composable application that can be deployed independently.

### Release a function

In some use cases, you want to release a function on-demand when it is no longer required.

```java
platform.release("another.function");
```

The above API will unload the function from memory and release it from the "event loop".

### Check if a function is available

You can check if a function with the named route has been deployed.

```java
if (platform.hasRoute("another.function")) {
    // do something
}
```

### Wait for a function to be ready

Functions are registered asynchronously. For functions registered using the `PreLoad` annotation, they are available
to your application when the MainApplication starts.

For functions that are registered on-demand, you can wait for the function to get ready like this:

```java
Future<Boolean> status = platform.waitForProvider("cloud.connector", 10);
status.onSuccess(ready -> {
   // business logic when "cloud.connector" is ready 
});
```
Note that the "onFailure" method is not required. The onSuccess will return true or false. In the above example,
your application waits for up to 10 seconds. If the function (i.e. the "provider") is available, the API will invoke
the "onSuccess" method immediately.

### Obtain the unique application instance ID

When an application instance starts, a unique ID is generated. We call this the "Origin ID".

```java
String originId = po.getOrigin();
```

When running the application in a minimalist service mesh using Kafka or similar network event stream system,
the origin ID is used to uniquely identify the application instance.

The origin ID is automatically appended to the "replyTo" address when making a RPC call over a network event stream
so that the system can send the response event back to the "originator" or "calling" application instance.

### Set application personality

An application may have one of the following personality:

1. REST - the deployed application is user facing
2. APP - the deployed application serves business logic
3. RESOURCES - this is a resource-tier service. e.g. database service, MQ gateway, legacy service proxy, utility, etc.

You can change the application personality like this:

```java
// the default value is "APP"
ServerPersonality.getInstance().setType(ServerPersonality.Type.REST);
```

The personality setting is for documentation purpose only. It does not affect the behavior of your application.
It will appear in the application "/info" endpoint.

## PostOffice API

You can obtain an instance of the PostOffice from the input "headers" and "instance" parameters in the input
arguments of your function.

```java
PostOffice po = new PostOffice(headers, instance);
```

The PostOffice is the event manager that you can use to send asynchronous events or to make RPC requests.
The constructor uses the READ only metadata in the "headers" argument in the "handleEvent" method of your function.

### Send an asynchronous event to a function

You can send an asynchronous event like this.

```java
// example-1
po.send("another.function", "test message");

// example-2
po.send("another.function", new Kv("some_key", "some_value"), new kv("another_key", "another_value"));

// example-3
po.send("another.function", somePoJo, new Kv("some_key", "some_value"));

// example-4
EventEnvelope event = new EventEnvelope().setTo("another.function")
                            .setHeader("some_key", "some_value").setBody(somePoJo);
po.send(event)

// example-5
po.sendLater(event, new Date(System.currentTimeMillis() + 5000));
```

1. Example-1 sends the text string "test message" to the target service named "another.function".
2. Example-2 sends two key-values as "headers" parameters to the same service.
3. Example-3 sends a PoJo and a key-value pair to the same service.
4. Example-4 is the same as example-3. It is using an EventEnvelope to construct the request.
5. Example-5 schedules an event to be sent 5 seconds later.

The first 3 APIs are convenient methods and the system will automatically create an EventEnvelope to hold the
target route name, key-values and/or event payload.

### Make an asynchronous RPC call to a function

You can make RPC call like this:

```java
// example-1
EventEnvelope request = new EventEnvelope().setTo("another.function")
                            .setHeader("some_key", "some_value").setBody(somePoJo);
Future<EventEnvelope> response = po.asyncRequest(request, 5000);
response.onSuccess(result -> {
    // result is the response event
});
response.onFailure(e -> {
    // handle timeout exception
});

// example-2
Future<EventEnvelope> response = po.asyncRequest(request, 5000, false);
response.onSuccess(result -> {
    // result is the response event
    // Timeout exception is returned as a response event with status=408
});

// example-3 with the "rpc" boolean parameter set to true
Future<EventEnvelope> response = po.asyncRequest(request, 5000, "http://mypeer/api/event", true);
response.onSuccess(result -> {
    // result is the response event
});
response.onFailure(e -> {
    // handle timeout exception
});
```

1. Example-1 makes a RPC call with a 5-second timer to "another.function".
2. Example-2 sets the "timeoutException" to false, telling system to return timeout exception as a regular event.
3. Example-3 makes an "event over HTTP" RPC call to "another.function" in another application instance called "mypeer".

"Event over HTTP" is an important topic. Please refer to [Chapter 7](CHAPTER-7.md)for more details.

### Perform a fork-n-join RPC call to multiple functions

In a similar fashion, you can make a fork-n-join call that sends request events in parallel to more than one function.

```java
// example-1
EventEnvelope request1 = new EventEnvelope().setTo("this.function")
                            .setHeader("hello", "world").setBody("test message");
EventEnvelope request2 = new EventEnvelope().setTo("that.function")
                            .setHeader("good", "day").setBody(somePoJo);
List<EventEnvelope> requests = new ArrayList<>();
requests.add(request1);
requests.add(request2);
Future<List<EventEnvelope>> responses = po.asyncRequest(requests, 5000);
response.onSuccess(results -> {
    // results contains the response events
});
response.onFailure(e -> {
    // handle timeout exception
});

// example-2
Future<List<EventEnvelope>> responses = po.asyncRequest(requests, 5000, false);
response.onSuccess(results -> {
    // results contains the response events.
    // Partial result list is returned if one or more functions did not respond.
});
```

### Make a sequential non-blocking RPC call to a function

You can make a sequential non-blocking RPC call from one function to another. The FastRPC is similar to the PostOffice.
It is the event manager for KotlinLambdaFunction. You can create an instance of the FastRPC using the "headers"
parameters in the input arguments of your function.

```kotlin
val fastRPC = new FastRPC(headers)
val request = EventEnvelope().setTo("another.function")
                            .setHeader("some_key", "some_value").setBody(somePoJo)
// example-1
val response = fastRPC.asyncRequest(request, 5000)
// handle the response event

// example-2 with the "rpc" boolean parameter set to true
val response = fastRPC.asyncRequest(request, 5000, "http://peer/api/event", true)
// handle the response event
```

1. Example-1 performs a non-blocking RPC call
2. Example-2 makes a non-blocking "Event Over HTTP" RPC call

Note that timeout exception is returned as a regular event with status 408.

Sequential non-blocking code is easier to read. Moreover, it handles more concurrent users and requests
without consuming a lot of CPU resources because it is "suspended" while waiting for a response from another function.

### Perform a sequential non-blocking fork-n-join call to multiple functions

You can make a sequential non-blocking fork-n-join call from one function to another using the FastRPC API like this:

```kotlin
val fastRPC = FastRPC(headers)
val template = EventEnvelope().setTo("hello.world").setHeader("someKey", "someValue")
val requests  = ArrayList<EventEnvelope>()
// create a list of 4 request events
for (i in 0..3) {
    requests.add(EventEnvelope(template.toBytes()).setBody(i).setCorrelationId("cid-$i"))
}
val responses: List<EventEnvelope> = fastRPC.awaitRequest(requests, 5000)
// handle the response events
```

In the above example, the function creates a list of request events from a template event with target service
"hello.world". It sets the number 0 to 3 to the individual events with unique correlation IDs.

The response events contain the same set of correlation IDs so that your business logic can decide how to
handle individual response event.

The result may be a partial list of response events if one or more functions failed to respond on time.

### Check if a function with a named route exists

The PostOffice provides the "exists()" method that is similar to the "platform.hasRoute()" command.

The difference is that the "exists()" method can discover functions of another application instance when running
in the "service mesh" mode.

If your application is not deployed in a service mesh, the PostOffice's "exists" and Platform's "hasRoute" APIs
will provide the same result.

```java
boolean found = po.exists("another.function");
if (found) {
    // do something
}
```

### Retrieve trace ID and path

If you want to know the route name and optional trace ID and path, you can use the following APIs.

For example, if tracing is enabled, the trace ID will be available. You can put the trace ID in application log
messages. This would group log messages of the same transaction together when you search the trace ID from 
a centralized logging dashboard such as Splunk.

```java
String myRoute = po.getRoute();
String traceId = po.getTraceId();
String tracePath = po.getTracePath();
```

## Trace annotation

You can use the PostOffice instance to annotate a trace in your function like this:

```java
// annotate a trace with the key-value "hello:world"
po.annotateTrace("hello", "world");
```

This is useful when you want to attach transaction specific information in the performance metrics.
For example, the traces may be used in production transaction analytics.

> IMPORTANT: do not annotate sensitive or secret information such as PII, PHI, PCI data because 
             the distributed trace log containing performance metrics and annotations would be 
             logged by the system. The trace would also be forwarded to a centralized telemetry
             dashboard. 

## Minimalist API design for event orchestration

As a best practice, we advocate a minimalist approach in API integration.
To build powerful composable applications, this small set of APIs shown above is sufficient to perform
"event orchestration" where you write code to coordinate how the various functions work together as a
single "executable". Please refer to [Chapter-4](CHAPTER-4.md) for more details about event orchestration. 

Since Mercury is used in production installations, we will exercise the best effort to keep the above API stable.

Other APIs in the toolkits are used internally to build the engine itself, and they may change from time to time.
They are mostly convenient methods and utilities. The engine is fully encapsulated and any internal API changes
should not impact your applications.

## Optional Event Scripting

To further reduce coding effort, you can perform "event orchestration" by configuration using "Event Script".
It is available as an enterprise add-on module from Accenture.

## Co-existence with other development frameworks

Mercury libraries are designed to co-exist with your favorite frameworks and tools. Inside a class implementing
the `LambdaFunction`, `TypedLambdaFunction` or `KotlinLambdaFunction`, you can use any coding style and frameworks
as you like, including sequential, object-oriented and reactive programming styles.

Mercury has a built-in light weight non-blocking HTTP server, but you can also use Spring Boot and other
application server framework with it.

A sample Spring Boot integration is provided in the "rest-spring" project. It is an optional feature, and you can
decide to use a regular Spring Boot application with Mercury or to pick the customized Spring Boot in the
"rest-spring" library.

## Template application for quick start

You can use the `lambda-example` project as a template to start writing your own applications. It is preconfigured
to support kernel threads, coroutine and suspend function.

## Source code update frequency

This project is licensed under the Apache 2.0 open sources license. We will update the public codebase after
it passes regression tests and meets stability and performance benchmarks in our production systems.

Mercury is developed as an engine for you to build the latest cloud native and composable applications.
While we are updating the technology frequently, the essential internals and the core APIs are stable.

We are monitoring the progress of the upcoming Java 19 Virtual Thread feature. We are committed to embrace it.
We will include the feature in our minimalist API set when Java 19 Virtual Thread becomes officially available.

## Technical support

For enterprise clients, optional technical support is available. Please contact your Accenture representative
for details.
<br/>

|          Chapter-8           |                   Home                    |            Chapter-10            |
|:----------------------------:|:-----------------------------------------:|:--------------------------------:|
| [Service mesh](CHAPTER-8.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [Migration guide](CHAPTER-10.md) |
