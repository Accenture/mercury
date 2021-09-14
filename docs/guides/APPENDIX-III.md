# Additional features

## Admin endpoints to stop, suspend or resume

You can stop, suspend or resume an application instance from a `presence monitor`.

1. `Shutdown` - stop an application so that the container management system will restart it
2. `Suspend` - tell the application instance not to accept incoming requests
3. `Resume` - tell the application instance to accept incoming requests

Suspend and resume commands are usually used to simulate error cases for development and regression test purposes.

For example, to simulate a stalled application instance, you can use the "POST /suspend/later" command.

If you do not want your application instance to receive any service request, you can isolate it with the
"POST /suspend/now" command.

```
POST /shutdown
POST /suspend/{now | later}
POST /resume/{now | later}

HTTP request header
X-App-Instance=origin_id_here
```

## Actuator endpoints

The following admin endpoints are available.

```
GET /info
GET /info/routes
GET /info/lib
GET /env
GET /health
GET /livenessprobe

Optional HTTP request header
X-App-Instance=origin_id_here
```

If you provide the optional X-App-Instance HTTP header, you can execute the admin endpoint from `any application instance` using the event stream system.

## Custom health services

You can extend the "/health" endpoint by implementing and registering lambda functions to be added to the health.dependencies.

```
mandatory.health.dependencies=cloud.cache.health,cloud.connector.health
#optional.health.dependencies=other.service.health
```

Your custom health service must respond to the following requests:

1. Info request (type=info) - it should return a map that includes (1) service name and (2) href (protocol, hostname and port)
2. Health check (type=health) - it should return a text string of the health check. e.g. read/write test result. It can throw AppException with status code and error message if health check fails.

The health service can retrieve the "type" of the request from the "headers".

## Application instance life-cycle events

Any application can subscribe to life-cycle events of other application instances.
Sample code is available in https://github.com/Accenture/mercury/blob/master/extensions/rest-automation-lib/src/main/java/org/platformlambda/automation/services/NotificationManager.java

To listen to life cycle events, you can do something like this:
```
String AUTOMATION_NOTIFICATION = "member.life.cycle.listener";
String appName = platform.getName();
LambdaFunction f = (headers, body, instance) -> {
    if (CONNECTED.equals(type)) {
        // handle connection event - this event is fired when your app is connected to the event stream system
        log.info("connected");
    }
    if (DISCONNECTED.equals(type)) {
        // handle disconnection event - this event is fired when your app is disconnected from the event stream system
        log.info("disconnected");
    }
    if (JOIN.equals(type) && headers.containsKey(ORIGIN) && appName.equals(headers.get(NAME))) {
        // handle member join event
    }
    if (LEAVE.equals(type) && headers.containsKey(ORIGIN)) {
        // handle member leave event
    }
}
platform.registerPrivate(AUTOMATION_NOTIFICATION, f, 1);

...
EventEnvelope event = new EventEnvelope()
                        .setTo(AUTOMATION_NOTIFICATION).setHeader(TYPE, SUBSCRIBE_LIFE_CYCLE);
po.sendLater(event, new Date(System.currentTimeMillis() + 2000));
```

## HttpClient as a service

Starting from version 1.12.30, the rest-automation system, when deployed, will provide the "async.http.request" service.

This means you can make a HTTP request without using a HttpClient.

For example, a simple HTTP GET request may look like this:

```java
// the target URL is constructed from the relay 
PostOffice po = PostOffice.getInstance();
AsyncHttpRequest req = new AsyncHttpRequest();
req.setMethod("GET");
req.setHeader("accept", "application/json");
req.setUrl("/api/search?keywords="+body);
req.setTargetHost("https://service_provider_host");
try {
    EventEnvelope res = po.request("async.http.request", 5000, req.toMap());
    log.info("GOT {} {}", res.getHeaders(), res.getBody());
    /*
     * res.getHeaders() contains HTTP response headers
     * res.getBody() is the HTTP response body
     *
     * Note that the HTTP body will be provided as be set a HashMap
     * if the input content-type is application/json or application/xml.
     */ 
    // process HTTP response here (HTTP-200)
    
} catch (AppException e) {
    log.error("Rejected by service provider HTTP-{} {}", 
               e.getStatus(), e.getMessage().replace("\n", ""));
    // handle exception here
}
```
In the above example, we are using RPC method. You may also use callback method for handling the HTTP response.

## Sending HTTP request body for HTTP PUT, POST and PATCH methods

For most cases, you can just set a HashMap into the request body and specify content-type as JSON or XML.
The system will perform serialization properly.

Example code may look like this:

```java
AsyncHttpRequest req = new AsyncHttpRequest();
req.setMethod("POST");
req.setHeader("accept", "application/json");
req.setHeader("content-type", "application/json");
req.setUrl("/api/book/new_book/12345");
req.setTargetHost("https://service_provider_host");
req.setBody(keyValues);
// where keyValues is a HashMap
```

## Sending HTTP request body as a stream

For larger payload, you may use streaming method. See sample code below:

```java
int len;
byte[] buffer = new byte[BUFFER_SIZE];
BufferedInputStream in = new BufferedInputStream(someFileInputStream);
ObjectStreamIO stream = new ObjectStreamIO(timeoutInSeconds);
ObjectStreamWriter out = stream.getOutputStream();
while ((len = in.read(buffer, 0, buffer.length)) != -1) {
    out.write(buffer, 0, len);
}
// closing the output stream would save an EOF mark in the stream
out.close();
// update the AsyncHttpRequest object
req.setStreamRoute(stream.getRoute());
```

## Handle HTTP response body stream

If content length is not given, the response body will be received as a stream.

Your application should check if the HTTP response headers contains a "stream" header.
Sample code to read the stream may look like this:

```java
PostOffice po = PostOffice.getInstance();
AsyncHttpRequest req = new AsyncHttpRequest();
req.setMethod("GET");
req.setHeader("accept", "application/json");
req.setUrl("/api/search?keywords="+body);
req.setTargetHost("https://service_provider_host");
EventEnvelope res = po.request("async.http.request", 5000, req.toMap());
Map<String, String> resHeaders = res.getHeaders();
if (resHeaders.containsKey("stream")) {
    ObjectStreamIO consumer = new ObjectStreamIO(resHeaders.get("stream"));
    /*
     * For demonstration, we are using ByteArrayOutputStream.
     * For production code, you should stream the input to persistent storage
     * or handle the input stream directly.
     */
    ByteArrayOutputStream out = new ByteArrayOutputStream();
    ObjectStreamReader in = consumer.getInputStream(1000);
    for (Object d: in) {
        if (d instanceof byte[]) {
            out.write((byte[]) d);
        }
    }
    // remember to close the input stream
    in.close();
    // handle the result
    byte[] result = out.toByteArray();
}
```

## Content length for HTTP request

`Important` - Do not set the "content-length" HTTP header because the system will automatically compute the
correct content-length for small payload. For large payload, it will use the chunking method.

The system may use data compression. Manually setting content length for HTTP request body would result
in unintended side effects.

---

| Chapter-1                                | Home                                     |
| :---------------------------------------:|:----------------------------------------:|
| [Introduction](CHAPTER-1.md)             | [Table of Contents](TABLE-OF-CONTENTS.md)|
