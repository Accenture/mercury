# Event over HTTP

The in-memory event system allows functions to communicate with each other in the same application memory space.

In composable architecture, applications are modular components in a network. Some transactions may require
the services of more than one application. "Event over HTTP" extends the event system beyond a single application.

The Event API service (`event.api.service`) is a built-in function in the system.

## The Event API endpoint

To enable "Event over HTTP", you must first turn on the REST automation engine with the following parameters
in the application.properties file:

```properties
rest.server.port=8085
rest.automation=true
```

and then check if the following entry is configured in the "rest.yaml" endpoint definition file. 
If not, update "rest.yaml" accordingly. The "timeout" value is set to 60 seconds to fit common use cases.

```yaml
  - service: [ "event.api.service" ]
    methods: [ 'POST' ]
    url: "/api/event"
    timeout: 60s
    tracing: true
```

This will expose the Event API endpoint at port 8085 and URL "/api/event". 

In kubernetes, The Event API endpoint of each application is reachable through internal DNS and there is no need
to create "ingress" for this purpose.

## Test drive Event API

You may now test drive the Event API service.

First, build and run the lambda-example application in port 8085.

```shell
cd examples/lambda-example
java -jar target/lambda-example-3.0.9.jar
```

Second, build and run the rest-spring-example application.

```shell
cd examples/rest-spring-example-2
java -jar target/rest-spring-2-example-3.0.9.jar
```

The rest-spring-2-example application will run as a Spring Boot application in port 8083 and 8086.

These two applications will start independently.

You may point your browser to http://127.0.0.1:8083/api/pojo/http/1 to invoke the `HelloPojoEventOverHttp` 
endpoint service that will in turn makes an Event API call to the lambda-example's "hello.pojo" service.

You will see the following response in the browser. This means the rest-spring-example application has successfully
made an event API call to the lambda-example application using the Event API endpoint.

```json
{
  "id": 1,
  "name": "Simple PoJo class",
  "address": "100 World Blvd, Planet Earth",
  "date": "2023-03-27T23:17:19.257Z",
  "instance": 6,
  "seq": 66,
  "origin": "2023032791b6938a47614cf48779b1cf02fc89c4"
}
```

To examine how the application makes the Event API call, please refer to the `HelloPojoEventOverHttp` class
in the rest-spring-example. The class is extracted below:

```java
@Path("/pojo")
public class HelloPoJoEventOverHttp {

    @GET
    @Path("/http/{id}")
    @Produces({MediaType.APPLICATION_XML, MediaType.APPLICATION_JSON})
    public void getPoJo(@PathParam("id") Integer id, @Suspended AsyncResponse response) {
        AppConfigReader config = AppConfigReader.getInstance();
        String remotePort = config.getProperty("lambda.example.port", "8085");
        String remoteEndpoint = "http://127.0.0.1:"+remotePort+"/api/event";
        String traceId = Utility.getInstance().getUuid();
        PostOffice po = new PostOffice("hello.pojo.endpoint", traceId, "GET /api/pojo/http");
        EventEnvelope req = new EventEnvelope().setTo("hello.pojo").setHeader("id", id);
        Future<EventEnvelope> res = po.asyncRequest(req, 5000, Collections.emptyMap(), remoteEndpoint, true);
        res.onSuccess(event -> {
            // confirm that the PoJo object is transported correctly over the event stream system
            if (event.getBody() instanceof SamplePoJo) {
                response.resume(event.getBody());
            } else {
                response.resume(new AppException(event.getStatus(), event.getError()));
            }
        });
        res.onFailure(response::resume);
    }
}
```

The method signatures of the Event API is shown as follows:

### Asynchronous API (Java)

```java
public Future<EventEnvelope> asyncRequest(final EventEnvelope event, long timeout,
                                          Map<String, String> headers,
                                          String eventEndpoint, boolean rpc) throws IOException;
```

### Sequential non-blocking API (Kotlin suspend function)

```kotlin
suspend fun awaitRequest(request: EventEnvelope?, timeout: Long, 
                          headers: Map<String, String>,
                          eventEndpoint: String, rpc: Boolean): EventEnvelope
```

Optionally, you may add security headers in the "headers" argument. e.g. the "Authorization" header.

The eventEndpoint is a fully qualified URL. e.g. `http://peer/api/event`

The "rpc" boolean value is set to true so that the response from the service of the peer application instance 
will be delivered. For drop-n-forget use case, you can set the "rpc" value to false. It will immediately return
an HTTP-202 response.

## Advantages

The Event API exposes all public functions of an application instance to the network using a single REST endpoint.

The advantages of Event API includes:

1. Convenient - you do not need to write or configure individual endpoint for each public service
2. Efficient - events are transported in binary format from one application to another
3. Secure - you can protect the Event API endpoint with an authentication service

The following configuration adds authentication service to the Event API endpoint:
```yaml
  - service: [ "event.api.service" ]
    methods: [ 'POST' ]
    url: "/api/event"
    timeout: 60s
    authentication: "v1.api.auth"
    tracing: true
```

This enforces every incoming request to the Event API endpoint to be authenticated by the "v1.api.auth" service
before passing to the Event API service. You can plug in your own authentication service such as OAuth 2.0 
"bearer token" validation.

Please refer to [Chapter-3 - REST automation](CHAPTER-3.md) for details.
<br/>

|          Chapter-6          |                   Home                    |          Chapter-8           |
|:---------------------------:|:-----------------------------------------:|:----------------------------:|
| [Spring Boot](CHAPTER-6.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [Service mesh](CHAPTER-8.md) |
