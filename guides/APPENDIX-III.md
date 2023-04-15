# Actuators and HTTP client

## Actuator endpoints

The following admin endpoints are available.

```
GET /info
GET /info/routes
GET /info/lib
GET /env
GET /health
GET /livenessprobe
POST /shutdown
```

| Endpoint       | Purpose                                                                             | 
|:---------------|:------------------------------------------------------------------------------------|
| /info          | Describe the application                                                            |
| /info/routes   | Show public routing table                                                           |
| /info/lib      | List libraries packed with this executable                                          |
| /env           | List all private and public function route names and selected environment variables |
| /health        | Application health check endpoint                                                   |
| /livenessprobe | Check if application is running normally                                            |
| /shutdown      | Operator may use this endpoint to do a POST command to stop the application         |

For the shutdown endpoint, you must provide an `X-App-Instance` HTTP header where the value is the "origin ID"
of the application. You can get the value from the "/info" endpoint.

## Custom health services

You can extend the "/health" endpoint by implementing and registering lambda functions to be added to the 
"health check" dependencies.

```properties
mandatory.health.dependencies=cloud.connector.health, demo.health
optional.health.dependencies=other.service.health
```

Your custom health service must respond to the following requests:

1. Info request (type=info) - it should return a map that includes service name and href (protocol, hostname and port)
2. Health check (type=health) - it should return a text string of the health check. e.g. read/write test result. 
   It can throw AppException with status code and error message if health check fails.

A sample health service is available in the `DemoHealth` class of the `lambda-example` project as follows:

```java
@PreLoad(route="demo.health", instances=5)
public class DemoHealth implements LambdaFunction {

    private static final String TYPE = "type";
    private static final String INFO = "info";
    private static final String HEALTH = "health";

    @Override
    public Object handleEvent(Map<String, String> headers, Object input, int instance) {
        /*
         * The interface contract for a health check service includes both INFO and HEALTH responses
         */
        if (INFO.equals(headers.get(TYPE))) {
            Map<String, Object> result = new HashMap<>();
            result.put("service", "demo.service");
            result.put("href", "http://127.0.0.1");
            return result;
        }
        if (HEALTH.equals(headers.get(TYPE))) {
            /*
             * This is a place-holder for checking a downstream service.
             *
             * You may implement your own logic to test if a downstream service is running fine.
             * If running, just return a health status message.
             * Otherwise,
             *      throw new AppException(status, message)
             */
            return "demo.service is running fine";
        }
        throw new IllegalArgumentException("type must be info or health");
    }
}

```

## Non-blocking HTTP client

The "async.http.request" function can be used as a non-blocking HTTP client.

To make an HTTP request to an external REST endpoint, you can create an HTTP request object using the
`AsyncHttpRequest` class and make an async RPC call to the "async.http.request" function like this:

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
Future<EventEnvelope> res = po.asyncRequest(request, 5000);
res.onSuccess(response -> {
   // do something with the result 
});
```

In a suspend function using KotlinLambdaFunction, the same logic may look like this:

```kotlin
val fastRPC = FastRPC(headers)
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
val request = EventEnvelope().setTo("async.http.request").setBody(req)
val response = fastRPC.awaitRequest(request, 5000)
// do something with the result
```

### Send HTTP request body for HTTP PUT, POST and PATCH methods

For most cases, you can just set a HashMap into the request body and specify content-type as JSON or XML.
The system will perform serialization properly.

Example code may look like this:

```java
AsyncHttpRequest req = new AsyncHttpRequest();
req.setMethod("POST");
req.setHeader("accept", "application/json");
req.setHeader("content-type", "application/json");
req.setUrl("/api/book");
req.setTargetHost("https://service_provider_host");
req.setBody(mapOfKeyValues);
// where keyValues is a HashMap
```

## Send HTTP request body as a stream

For larger payload, you may use the streaming method. See sample code below:

```java
int len;
byte[] buffer = new byte[4096];
FileInputStream in = new FileInputStream(myFile);
ObjectStreamIO stream = new ObjectStreamIO(timeoutInSeconds);
ObjectStreamWriter out = stream.getOutputStream();
while ((len = in.read(buffer, 0, buffer.length)) != -1) {
    out.write(buffer, 0, len);
}
// closing the output stream would send a EOF signal to the stream
out.close();
// tell the HTTP client to read the input stream
req.setStreamRoute(stream.getInputStreamId());
```

## Read HTTP response body stream

If content length is not given, the response body will be received as a stream.

Your application should check if the HTTP response header "stream" exists. Its value is an input "stream ID".

For simplicity and readability, we recommend using "suspend function" to read the input byte-array stream.

It may look like this:

```kotlin
val po = PostOffice(headers, instance)
val fastRPC = FastRPC(headers)

val req = EventEnvelope().setTo(streamId).setHeader("type", "read")
while (true) {
    val event = fastRPC.awaitRequest(req, 5000)
    if (event.status == 408) {
        // handle input stream timeout
        break
    }
    if ("eof" == event.headers["type"]) {
        po.send(streamId, Kv("type", "close"))
        break
    }
    if ("data" == event.headers["type"]) {
        val block = event.body
        if (block is ByteArray) {
            // handle the data block from the input stream
        }
    }
}
```

## Content length for HTTP request

IMPORTANT: Do not set the "content-length" HTTP header because the system will automatically compute the
correct content-length for small payload. For large payload, it will use the chunking method.
<br/>

|              Appendix-II               |                   Home                    | 
|:--------------------------------------:|:-----------------------------------------:|
| [Reserved route names](APPENDIX-II.md) | [Table of Contents](TABLE-OF-CONTENTS.md) |
