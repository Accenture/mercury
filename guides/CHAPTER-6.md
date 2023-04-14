# Spring Boot integration

While the platform-core foundation code includes a lightweight non-blocking HTTP server, you can also turn your
application into a regular Spring Boot application.

There are two ways to do that:

1. Add dependency for Spring Boot version 2.7.10 and implement your Spring Boot main application
2. Add the `rest-spring` add-on library for a pre-configured Spring Boot experience

## Add platform-core to an existing Spring Boot application

For option 1, the platform-core library can co-exist with Spring Boot. You can write code specific to Spring Boot
and the Spring framework ecosystem. Please make sure you add the following startup code to your Spring Boot
main application's startup sequence like this:

```java
@SpringBootApplication
public class MyMainApp extends SpringBootServletInitializer {

    public static void main(String[] args) {
        AppStarter.main(args);
        SpringApplication.run(MyMainApp.class, args);
    }

}
```
We suggest running `AppStarter.main` before the `SpringApplication.run` statement. This would allow the platform-core
foundation code to load the event-driven functions into memory before Spring Boot starts.

## Use the rest-spring library in your application

Adding the `rest-spring` library in your application would turn it into a pre-configured Spring Boot application.

The "rest-spring" library configures Spring Boot's serializers (XML and JSON) to behave consistently as the
built-in lightweight non-blocking HTTP server.

If you want to disable the lightweight HTTP server, you can set `rest.automation=false` in application.properties.
The REST automation engine and the lightweight HTTP server will be turned off.

> IMPORTANT: the platform-core library assumes the application configuration files to be either
  application.yml or application.properties. If you use custom Spring profile, please keep the
  application.yml or application.properties for the platform-core. If you use default Spring 
  profile, both platform-core and Spring Boot will use the same configuration files.

You can customize your error page using the default `errorPage.html` from the platform-core's or 
rest-spring's resources folder in the source project. The default page is shown below.

This is the HTML error page that the platform-core or rest-spring library will render. You can update it with
your corporate UI style. Please keep the parameters (status, message, path, warning) intact.

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <title>HTTP Error</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body>

<div>
    <h3>HTTP-${status}</h3>
    <div>${warning}</div><br/>
    <table>
        <tbody>
        <tr><td style="font-style: italic; width: 100px">Type</td><td>error</td></tr>
        <tr><td style="font-style: italic; width: 100px">Status</td><td>${status}</td></tr>
        <tr><td style="font-style: italic; width: 100px">Message</td><td>${message}</td></tr>
        <tr><td style="font-style: italic; width: 100px">Path</td><td>${path}</td></tr>
        </tbody>
    </table>

</div>
</body>
</html>
```

If you want to keep REST automation's lightweight HTTP server together with Spring Boot's Tomcat or other 
application server, please add the following to your application.properties file:

```properties
server.port=8083
rest.server.port=8085
rest.automation=true
```

The platform-core will use `rest.server.port` instead of `server.port` so that the lightweight HTTP server and
Spring Boot's Tomcat can co-exist.

## The rest-spring-example demo application

Let's review the `rest-spring-example` demo application in the "examples/rest-spring-example" project.

You can use the rest-spring-example as a template to quickly create a Spring Boot application.

In addition to the REST automation engine that let you create REST endpoints by configuration, you can also
programmatically create REST endpoints with the following choices:

1. JAX-RS
2. Spring Controller
3. Servlet 3.1

We will examine asynchronous REST endpoint with the `AsyncHelloWorld` class.

Since the platform-core is event-driven, we would like to use JAX-RS asynchronous HTTP context `AsyncResponse`
in the REST endpoints so that the endpoint does not block.

```java
@Path("/hello")
public class AsyncHelloWorld {

    private static final AtomicInteger seq = new AtomicInteger(0);

    @GET
    @Path("/world")
    @Produces({MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML})
    public void hello(@Context HttpServletRequest request,
                      @Suspended AsyncResponse response) {

        String traceId = Utility.getInstance().getUuid();
        PostOffice po = new PostOffice("hello.world.endpoint", traceId, "GET /api/hello/world");
        Map<String, Object> forward = new HashMap<>();

        Enumeration<String> headers = request.getHeaderNames();
        while (headers.hasMoreElements()) {
            String key = headers.nextElement();
            forward.put(key, request.getHeader(key));
        }
        // As a demo, just put the incoming HTTP headers as a payload and add a counter
        // The echo service will return both.
        int n = seq.incrementAndGet();
        EventEnvelope req = new EventEnvelope();
        req.setTo("hello.world").setBody(forward).setHeader("seq", n);
        Future<EventEnvelope> res = po.asyncRequest(req, 3000);
        res.onSuccess(event -> {
            Map<String, Object> result = new HashMap<>();
            result.put("status", event.getStatus());
            result.put("headers", event.getHeaders());
            result.put("body", event.getBody());
            result.put("execution_time", event.getExecutionTime());
            result.put("round_trip", event.getRoundTrip());
            response.resume(result);
        });
        res.onFailure(ex -> response.resume(new AppException(408, ex.getMessage())));
    }
}
```

In this hello world REST endpoint, JAX-RS run the "hello" method asynchronously without waiting for a response.

The example code copies the HTTP requests and sends it as the request payload to the "hello.world" function.
The function is defined in the MainApp like this:

```java
Platform platform = Platform.getInstance();
LambdaFunction echo = (headers, input, instance) -> {
    Map<String, Object> result = new HashMap<>();
    result.put("headers", headers);
    result.put("body", input);
    result.put("instance", instance);
    result.put("origin", platform.getOrigin());
    return result;
};
platform.register("hello.world", echo, 20);
```

When "hello.world" responds, its result set will be returned to the `onSuccess` method as the "future response".

The "onSuccess" method then sends the response to the browser using JAX-RS resume mechanism.

The `AsyncHelloConcurrent` is the same as the `AsyncHelloWorld` except that it performs a "fork-n-join" operation
to multiple instances of the "hello.world" function.

Unlike "rest.yaml" that defines tracing by configuration, you can turn on tracing programmatically in a JAX-RS
endpoint. To enable tracing, the function sets the trace ID and path in the PostOffice constructor. 

When you try the endpoint at http://127.0.0.1:8083/api/hello/world, it will echo your HTTP request headers. 
In the command terminal, you will see tracing information in the console log like this:

```text
DistributedTrace:67 - trace={path=GET /api/hello/world, service=hello.world, success=true, 
  origin=20230403364f70ebeb54477f91986289dfcd7b75, exec_time=0.249, start=2023-04-03T04:42:43.445Z, 
  from=hello.world.endpoint, id=e12e871096ba4938b871ee72ef09aa0a, round_trip=20.018, status=200}
```

## Lightweight non-blocking websocket server

If you want to turn on a non-blocking websocket server, you can add the following configuration to 
application.properties.

```properties
server.port=8083
websocket.server.port=8085
```

The above assumes Spring Boot runs on port 8083 and the websocket server runs on port 8085.

You can create a websocket service with a Java class like this:

```java
@WebSocketService("hello")
public class WsEchoDemo implements LambdaFunction {

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        // handle the incoming websocket events (type = open, close, bytes or string)
    }
}
```

The above creates a websocket service at the URL "/ws/hello" server endpoint.

Please review the example code in the WsEchoDemo class in the rest-spring-example project for details.

If you want to use Spring Boot's Tomcat websocket server, you can disable the non-blocking websocket server feature
by removing the `websocket.server.port` configuration and any classes with the `WebSocketService` annotation.

To try out the demo websocket server, visit http://127.0.0.1:8083 and select "Websocket demo".
<br/>

|               Chapter-5                |                   Home                    |            Chapter-7            |
|:--------------------------------------:|:-----------------------------------------:|:-------------------------------:|
| [Build, test and deploy](CHAPTER-4.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [Event over HTTP](CHAPTER-7.md) |
