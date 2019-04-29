# Introduction

Mercury is all about microservices that are minimalist, event-driven and context bounded.

To this end, we use anonymous functions to encapsulate different domains of business logic and library dependencies.

Under the Mercury framework, business logic wrapped in anonymous functions are callable using a `route name`. Mercury resolves routing automatically so that you do not need to care whether the calling and called functions are in the same memory space or in different application instances. Mercury will route requests using a high performance memory event bus when the calling and called functions are int he same memory space and route requests through a network event stream system when the calling and called parties reside in different containers.

## Building the mercury framework libraries

Please follow the [README](../../README.md) file in the project root to build the Mercury framework libraries from source code.

## Writing your first microservices function

Your first function may look like this using Java 1.8 anonymous function syntax:
```
LambdaFunction f = (headers, body, instance) -> {
	// do some business logic
	return something
};
```

The easiest way to write your first microservices module is to use either the "lambda-example" or "rest-example" as a template.

Let's try with the "rest-example". You should update the application name in both the `application.properties` and the `pom.xml`. Then you can use your favorite IDE to import it as a "maven" project.

## Application unit

The rest-example is a deployable application unit. Behind the curtain, the mercury framework is using Spring Boot to provide REST and websocket capabilities. For microservices modules that do not need REST endpoints, you can use the "lambda-example" as a template.

## Main application

For each application unit, you will need a main application. This is the entry of your application unit.

The `MainApplication` annotation indicates that this is the main method for the application unit. Main application should also implements the `EntryPoint` interface which only has the "start" method. The "args" are optional command line arguments.

In the following example, when the application unit starts, it creates a microservices function and register "hello.world" as its route name. For concurrency, it also specifies 20 worker instances.

Application units are horizontally scalable. Within the application unit, you may specify concurrent "workers". This provides horizontal and verticial scalability respectively.


```
@MainApplication
public class MainApp implements EntryPoint {
  
    @Override
    public void start(String[] args) throws Exception {
        ServerPersonality.getInstance().setType(ServerPersonality.Type.WEB);
        Platform platform = Platform.getInstance();
        LambdaFunction echo = (headers, body, instance) -> {
            Map<String, Object> result = new HashMap<>();
            result.put("headers", headers);
            result.put("body", body);
            result.put("instance", instance);
            result.put("origin", platform.getOrigin());
            return result;
        };
        platform.register("hello.world", echo, 20);
    }
}

```

## Calling a function

Unlike traditional programming, you call a function by sending an event instead of calling its method. Mercury resolves routing automatically so events are delivered correctly no matter where the target function is, in the same memory space or another computer elsewhere in the network.

To make a service call to a function, you may do the following:
```
PostOffice po = PostOffice.getInstance();
EventEnvelope response = po.request("hello.world", 1000, "a test message");
System.out.println("I got response here..."+response.getBody());

// the above is an RPC call. For async call, it would be something like this:
po.send("hello.world", "another message");
```

You can call the function from another function or a REST endpoint. The latter connects REST API with a microservices function.

The following example forwards a request from the REST endpoint (`GET /api/hello/world`) to the "hello.world" service. Note that there are basic performance metrics from the response object.

```
@Path("/hello")
public class MyRestEndpoint {

    private static AtomicInteger seq = new AtomicInteger(0);

    @GET
    @Path("/world")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Map<String, Object> hello(@Context HttpServletRequest request) throws IOException, TimeoutException, AppException {

        PostOffice po = PostOffice.getInstance();

        Map<String, Object> forward = new HashMap<>();
        forward.put("time", new Date());

        Enumeration<String> headers = request.getHeaderNames();
        while (headers.hasMoreElements()) {
            String key = headers.nextElement();
            forward.put(key, request.getHeader(key));
        }
        // As a demo, just put the incoming HTTP headers as a payload and a parameter showing the sequence counter.
        // The eco service will return both.
        int n = seq.incrementAndGet();
        EventEnvelope response = po.request("hello.world", 3000, forward, new Kv("seq", n));

        Map<String, Object> result = new HashMap<>();
        result.put("status", response.getStatus());
        result.put("headers", response.getHeaders());
        result.put("body", response.getBody());
        result.put("execution_time", response.getExecutionTime());
        result.put("round_trip", response.getRoundTrip());
        return result;
    }

}
```

## Massive parallel processing

A function is invoked when an event happens. Before the event arrives, the function is just an entry in a routing table and it does not consume any additional resources like threads.

All functions are running in parallel without special coding. Behind the curtain, the system uses Java futures and asynchronous event loops for very efficient function execution.


| Chapter-2                           | Home                                     |
| :----------------------------------:|:----------------------------------------:|
| [Platform API](CHAPTER-2.md)        | [Table of Contents](TABLE-OF-CONTENTS.md)|


