# Function execution strategies

## Define a function

In a composable application, each function is self-contained with zero or minimal dependencies.

A function is a class that implements the LambdaFunction, TypedLambdaFunction or KotlinLambdaFunction interface.
Within each function boundary, it may have private methods that are fully contained within the class.

As discussed in Chapter-1, a function may look like this:

```java
@PreLoad(route = "my.first.function", instances = 10)
public class MyFirstFunction implements TypedLambdaFunction<AsyncHttpRequest, Object> {

    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) {
        // your business logic here
        return input;
    }
}
```

A function is an event listener with the "handleEvent" method. The data structures of input and output are defined
by API interface contract during application design phase.

In the above example, the input is AsyncHttpRequest because this function is designed to handle an HTTP request event
from a REST endpoint defined in the "rest.yaml" configuration file. We set the output as "Object" so that there is
flexibility in returning a HashMap or a PoJo. You can also enforce the use of a PoJo by updating the output type.

A single transaction may involve multiple functions. For example, the user submits a form from a browser that
sends an HTTP request to a function. In MVC pattern, the function receiving the user's input is the "controller".
It carries out input validation and forwards the event to a business logic function (the "view")
that performs some processing and then submits the event to a data persistent function (the "model") to save
a record into the database.

In cloud native application, the transaction flow may be more sophisticated than the typical "mvc" style. You can do
"event orchestration" in the function receiving the HTTP request and then make event requests to various functions.

This "event orchestration" can be done by code using the "PostOffice" and/or "FastRPC" API.

To further reduce coding effort, you can perform "event orchestration" by configuration using "Event Script".
Event Script is available as an optional enterprise add-on module from Accenture.

## Extensible authentication function

You can add authentication function using the optional `authentication` tag in a service. In "rest.yaml", a service
for a REST endpoint refers to a function in your application.

An authentication function can be written using a TypedLambdaFunction that takes the input as a "AsyncHttpRequest".
Your authentication function can return a boolean value to indicate if the request should be accepted or rejected.

A typical authentication function may validate an HTTP header or cookie. e.g. forward the "Bearer token" from the
"Authorization" header to your organization's OAuth 2.0 Identity Provider for validation.

To approve an incoming request, your custom authentication function can return `true`.

Optionally, you can add "session" key-values by returning an EventEnvelope like this:

```java
return new EventEnvelope().setHeader("user_id", "A12345").setBody(true);
```

The above example approves the incoming request and returns a "session" variable ("user_id": "A12345") to the next task.

If your authentication function returns `false`, the user will receive a "HTTP-401 Unauthorized" error response.

You can also control the status code and error message by throwing an `AppException` like this:

```shell
throw new AppException(401, "Invalid credentials");
```

A composable application is assembled from a collection of modular functions. For example, data persistence functions
and authentication functions are likely to be reusable in many applications.

## Number of workers for a function

```java
@PreLoad(route = "my.first.function", instances = 10)
```

In the above function, the parameter "instances" tells the system to reserve a number of workers for the function.
Workers are running on-demand to handle concurrent user requests.

Note that you can use smaller number of workers to handle many concurrent users if your function finishes
processing very quickly. If not, you should reserve more workers to handle the work load.

Concurrency requires careful planning for optimal performance and throughput. 
Let's review the strategies for function execution.

## Three strategies for function execution

A function is executed when an event arrives. There are three function execution strategies.

| Strategy         | Advantage                                                                                                     | Disadvantage                                                                   |
|:-----------------|:--------------------------------------------------------------------------------------------------------------|:-------------------------------------------------------------------------------|
| Kernel threads   | Highest performance in terms of<br/>operations per seconds                                                    | Lower number of concurrent threads<br/>due to high context switching overheads |
| Coroutine        | Highest throughput in terms of<br/>concurrent users served by virtual<br/>threads concurrently                | Not suitable for long running tasks                                            |
| Suspend function | Synchronous "non-blocking" for<br/>RPC (request-response) that<br/>makes code easier to read and<br/>maintain | Not suitable for long running tasks                                            |

### Kernel thread pool

When you write a function using LambdaFunction and TypedLambdaFunction, the function will be executed using
"kernel thread pool" and Java will run your function in native "preemptive multitasking" mode.

While preemptive multitasking fully utilizes the CPU, its context switching overheads may increase as the number of
kernel threads grow. As a rule of thumb, you should control the maximum number of kernel threads to less than 200.

The parameter `event.worker.pool` is defined with a default value of 100. You can change this value to adjust to
the actual CPU power in your environment. Keep the default value for best performance unless you have tested the
limit in your environment.

> When you have more concurrent requests, your application may slow down because some functions
  are blocked when the number of concurrent kernel threads is reached.

You should reduce the number of "instances" (i.e. worker pool) for a function to a small number so that your
application does not exceed the maximum limit of the `event.worker.pool` parameter.

Kernel threads are precious and finite resources. When your function is computational intensive or making 
external HTTP or database calls in a synchronous blocking manner, you may use it with a small number
of worker instances.

To rapidly release kernel thread resources, you can write "asynchronous" code. i.e. for event-driven programming,
you can use send event to another function asynchronously, and you can create a callback function to listen
to responses.

For RPC call, you can use the `asyncRequest` method to write asynchronous RPC calls. However, coding for asynchronous
RPC pattern is more challenging. For example, you may want to return a "pending" result immediately using HTTP-202. 
Your code will move on to execute using a "future" that will execute callback methods (`onSuccess` and `onFailure`).
Another approach is to annotate the function as an `EventInterceptor` so that your function can respond to the user
in a "future" callback.

For ease of programming, we recommend using suspend function to handle RPC calls.

### Coroutine

When you add the `CoroutineRunner` annotation in your function, the system will run your function as a coroutine.

Normally, coroutines are executed in an event loop using a single kernel thread. Note that the underlying Eclipse
vertx is a multithreaded event system that executes coroutines in a small number of event loops concurrently for
better performance. As a result, the system can handle tens of thousands of coroutines running concurrently.

Since coroutine is running in a single thread, you must avoid writing "blocking" code because it would slow down
the whole application significantly.

If your function can finish processing very quickly, coroutine is ideal.

### Suspend function

A suspend function is a coroutine that can be suspended and resumed. The best use case for a suspend function is
for handling of "sequential non-blocking" request-response. This is the same as "async/await" in node.js and other
programming language.

To implement a "suspend function", you must implement the KotlinLambdaFunction interface and write code in Kotlin.

If you are new to Kotlin, please download and run JetBrains Intellij IDE. The quickest way to get productive in Kotlin
is to write a few statements of Java code in a placeholder class and then copy-n-paste the Java statements into the
KotlinLambdaFunction's handleEvent method. Intellij will automatically convert Java code into Kotlin.

The automated code conversion is mostly accurate (roughly 90%). You may need some touch up to polish the converted
Kotlin code.

In a suspend function, you can use a set of "await" methods to make non-blocking request-response (RPC) calls.
For example, to make a RPC call to another function, you can use the `awaitRequest` method.

Please refer to the `FileUploadDemo` class in the "examples/lambda-example" project.

```kotlin
val po = PostOffice(headers, instance)
val fastRPC = FastRPC(headers)

val req = EventEnvelope().setTo(streamId).setHeader(TYPE, READ)
while (true) {
    val event = fastRPC.awaitRequest(req, 5000)
    // handle the response event
    if (EOF == event.headers[TYPE]) {
        log.info("{} saved", file)
        awaitBlocking {
            out.close()
        }
        po.send(streamId, Kv(TYPE, CLOSE))
        break;
    }
    if (DATA == event.headers[TYPE]) {
        val block = event.body
        if (block is ByteArray) {
            total += block.size
            log.info("Saving {} - {} bytes", filename, block.size)
            awaitBlocking {
                out.write(block)
            }
        }
    }
}
```
In the above code segment, it has a "while" loop to make RPC calls to continuously "fetch" blocks of data 
from a stream. The status of the stream is indicated in the event header "type". It will exit the "while" loop
when it detects the "End of Stream (EOF)" signal.

Suspend function will be "suspended" when it is waiting for a response. When it is suspended, it does not
consume CPU resources, thus your application can handle a large number of concurrent users and requests.

Coroutines run in a "cooperative multitasking" manner. Technically, each function is running sequentially.
However, when many functions are suspended during waiting, it appears that all functions are running concurrently.

You may notice that there is an `awaitBlocking` wrapper in the code segment.

Sometimes, you cannot avoid blocking code. In the above example, the Java's FileOutputStream is a blocking method.
To ensure that a small piece of blocking code in a coroutine does not slow down the "event loop", 
you can apply the `awaitBlocking` wrapper method. The system will run the blocking code in a separate worker thread
without blocking the event loop.

In addition to the "await" sets of API, the `delay(milliseconds)` method puts your function into sleep in a 
non-blocking manner. The `yield()` method is useful when your function requires more time to execute complex
business logic. You can add the `yield()` statement before you execute a block of code. The yield method releases
control to the event loop so that other coroutines and suspend functions will not be blocked by a heavy weighted
function.

Suspend function is a powerful way to write high throughput application. Your code is presented in a sequential
flow that is easier to write and maintain.

You may want to try the demo "file upload" REST endpoint to see how suspend function behaves. If you follow Chapter-1,
your lambda example application is already running. To test the file upload endpoint, here is a simple Python script:

```python
import requests
files = {'file': open('some_data_file.txt', 'rb')}
r = requests.post('http://127.0.0.1:8085/api/upload', files=files)
print(r.text)
```

This assumes you have the python "requests" package installed. If not, please do `pip install requests` to install
the dependency.

The uploaded file will be kept in the "/tmp/upload-download-demo" folder.

To download the file, point your browser to http://127.0.0.1:8085/api/download/some_data_file.txt
Your browser will usually save the file in the "Downloads" folder.

You may notice that the FileDownloadDemo class is written in Java using the interface
`TypedLambdaFunction<AsyncHttpRequest, EventEnvelope>`. The FileDownloadDemo class will run using a kernel thread.

Note that each function is independent and the functions with different execution strategies can communicate in events.

The output of your function is an "EventEnvelope" so that you can set the HTTP response header correctly.
e.g. content type and filename.

When downloading a file, the FileDownloadDemo function will block if it is sending a large file. 
Therefore, you want it to run as a kernel thread. 

For very large file download, you may want to write the FileDownloadDemo function using asynchronous programming
with the `EventInterceptor` annotation or implement a suspend function using KotlinLambdaFunction. Suspend function
is non-blocking.
<br/>

|          Chapter-1           |                   Home                    |            Chapter-3            |
|:----------------------------:|:-----------------------------------------:|:-------------------------------:|
| [Introduction](CHAPTER-1.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [REST automation](CHAPTER-3.md) |
