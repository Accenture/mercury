# REST automation

This is the recommended way to automate REST endpoint creation.

## Library and Application

The REST automation system consists of a library and an application. This sub-project is the library.

## REST endpoint creation

Instead of creating REST endpoints programmatically, this application allows REST endpoints to be created declaratively.
i.e. REST endpoints can be created with simple routing syntax in a YAML file.

## REST endpoint routing

A routing entry is a simple mapping of URL to a microservice.

## YAML syntax

```yaml
rest:
  # service should be a target service name or a list of service names
  # If more than one service name is provided, the first one is the primary target
  # and the rest are secondary target(s). The system will copy the HTTP request event to the secondary targets.
  #
  # This feature is used for seamless legacy system migration where we can send
  # the same request to the old and the new services at the same time for A/B comparison.
  - service: ["hello.world", "hello.world.2"]
    methods: ['GET', 'PUT', 'POST', 'HEAD', 'PATCH', 'DELETE']
    url: "/api/hello/world"
    timeout: 10s
    # optional authentication service which should return result as an EventEnvelope
    # with session info in headers and true or false in body
#    authentication: "v1.api.auth"
    cors: cors_1
    headers: header_1
    # for HTTP request body that is not JSON/XML, it will be turned into a stream if it is undefined
    # or larger than threshold. Otherwise, it will be delivered as a byte array in the message body.
    # Default is 50000 bytes, min is 5000, max is 500000
    threshold: 30000
    # optionally, you can turn on Distributed Tracing
    tracing: true

  - service: "hello.world"
    methods: ['GET', 'PUT', 'POST']
    url: "/api/test/ok*"
    # optional "upload" key if it is a multi-part file upload
    upload: "file"
    timeout: 15s
    headers: header_1
   cors: cors_1

  - service: "hello.world"
    methods: ['GET', 'PUT', 'POST']
    url: "/api/nice/{task}/*"
    timeout: 12
    headers: header_1
   cors: cors_1

  #
  # When service is a URL, it will relay HTTP or HTTPS requests.
  # "trust_all_cert" and "url_rewrite" are optional.
  #
  # For target host with self-signed certificate, you may set "trust_all_cert" to true.
  # trust_all_cert: true
  #
  # "url_rewrite", when present as a list of 2 strings, is used to rewrite the url.
  # e.g. url_rewrite: ['/api/v1', '/v1/api']
  # In this example, "/api/v1" will be replaced with "/v1/api"
  #
  - service: "http://127.0.0.1:8100"
    trust_all_cert: true
    methods: ['GET', 'PUT', 'POST']
    url: "/api/v1/*"
    url_rewrite: ['/api/v1', '/api']
    timeout: 20
    cors: cors_1
    headers: header_1
    tracing: true

#
# CORS HEADERS for pre-flight (HTTP OPTIONS) and normal cases
#
cors:

  - id: cors_1
    options:
      - "Access-Control-Allow-Origin: *"
      - "Access-Control-Allow-Methods: GET, DELETE, PUT, POST, OPTIONS"
      - "Access-Control-Allow-Headers: Origin, Authorization, X-Session-Id, Accept, Content-Type, X-Requested-With"
      - "Access-Control-Max-Age: 86400"
    headers:
      - "Access-Control-Allow-Origin: *"
      - "Access-Control-Allow-Methods: GET, DELETE, PUT, POST, OPTIONS"
      - "Access-Control-Allow-Headers: Origin, Authorization, X-Session-Id, Accept, Content-Type, X-Requested-With"
      - "Access-Control-Allow-Credentials: true"

#
# add/drop/keep header parameters
#
headers:

  - id: header_1
    # headers to be inserted
    add: ["hello-world: nice"]
#    keep: ['x-session-id', 'user-agent']
    drop: ['Upgrade-Insecure-Requests', 'cache-control', 'accept-encoding']
```

The `service` must be a service that is registered as public function.

The system will find the service at run-time. If the service is not available, the user will see HTTP-503 
"Service not reachable".

The `keep` and `drop` entries are mutually exclusive where `keep` has precedent over `drop`. When keep is empty 
and drop is not, it will drop only the headers in the drop list. The `add` entry allows the developer to insert 
additional header key-values before it reaches the target service that serves the REST endpoint.

For content types of XML and JSON, the system will try to convert the input stream into a map.

For binary content type, the system will check if the input stream is larger than a threshold.
`threshold` - the default threshold buffer is 50,000 bytes, min is 5,000 and max is 500,000

If the input stream is small, it will convert the input stream into a byte array and store it in the "body" parameter.

if the input stream is large, it will return a stream ID so that your service can read the data as a stream.

Cors header processing is optional. If present, it will return the configured key-values for pre-flight and regular 
responses.

## URL matching

For performance and readability reason, the system is not using regular expression for URL matching. 
Instead it is using wildcard and path parameters.

### wildcard

The wildcard `*` character may be used as a URL path segment or as a suffix in a segment. 
Characters after the wildcard character in a segment will be ignored.

e.g. the following are valid wildcard URLs

```
/api/hello/world/*
/api/hello*/world*/*
```

### path parameters

The `{}` bracket can be used to indicate a path parameter. The value inside the bracket is the path parameter ID.

For example, the following URL will set path parameter "first_name" and "last_name".

```
/api/hello/{first_name}/{last_name}
```

### HTTP request dataset

The HTTP request (headers, query, body) will be encoded as a Map and the target service will receive it as the 
message `body`.

```yaml
headers: key-values of HTTP headers
cookies: optional key-values
method: HTTP request method
ip: caller's remote IP address
parameters: 
  path: optional path parameters
  query: optional query or URL encoded request body parameters
url: URL of the incoming request
timeout: timeout value (in seconds) of the REST endpoint

# optional (body or stream)
body: byte array of input data if it is smaller than a given threshold
stream: stream-ID of incoming data stream (e.g. file)
filename: filename if it is a file upload
content-length: number of bytes of the incoming stream

```

For Java, you can use AsyncHttpRequest as a convenient wrapper to read this dataset.

```javascript
AsyncHttpRequest request = new AsyncHttpRequest(body);
```

### Http response body

For simple use case, you can just send text, bytes or Map as the return value in your service.

For advanced use cases, you can send status code, set HTTP headers if your service returns an EventEnvelope object.

### HTTP status code

If your service set status code directly, you should use the standard 3-digit HTTP status code because it will be 
sent to the browser directly. i.e. 200 means OK and 404 is NOT_FOUND, etc.

### Setting cookies

You can ask a browser to set a cookie by setting the "Set-Cookie" key-value in an EventEnvelope's header which will be 
converted to a HTTP response header. 
For details, please see https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie

There is a convenient "getHtmlDate()" method in the Utility class if you want to set the "Expires" directive in a cookie.


### Exception handling

If your service throws exception, it will automatically translate into a regular HTTP exception with status code 
and message.

To control the HTTP error code, your service should use the AppException class. It allows you to set HTTP status code
and error message directly.

### Input stream

If you want to support file upload, you can specify the `upload` parameter in the rest.yaml configuration file.
The default value is "file". If your user uses different tag, you must change the `upload` parameter to match the
upload tag.

If incoming request is a file, your service will see the "stream", "filename" and "content-length" parameters.
If incoming request is a byte stream, your service will find the "stream" and "content-length" parameters.

### Output stream

If your service wants to send the output to the browser as a stream of text or bytes, you can create an ObjectStreamIO 
with a timeout value. You can then set the streamId in the HTTP header "stream". You should also set the HTTP header 
"timeout" to tell the REST endpoint to use it as IO stream read timeout value. The "stream" and "timeout" headers are
used by the REST automation framework. They will not be set as HTTP response headers.

You timeout value should be short and yet good enough for your service to send one block of data. The timer will be 
reset when there is I/O activity. One use case of output stream is file download.

### Location of the YAML configuration file

By default, the system will search for `file:/tmp/config/rest.yaml` and then `classpath:/rest.yaml`.

If needed, you can change this in the `rest.automation.yaml` property in the application.properties file.

The `/tmp/config/rest.yaml` allows you to externalize the REST automation configuration without rebuilding 
the REST automation helper application.

### Running the REST automation helper application

Before you build this application, you should follow the README in the top level project to build the Mercury 
libraries (platform-core, rest-core, rest-spring, kafka-connector, hazelcast-connector).

If you use the Event Node "platform-in-box" for dev and test, you can simply build the Event Node and run it in 
a command terminal.

Then you can build this helper app and run with:
```
java -Dcloud.connector=event.node target/rest-automation...jar
```

If you plan to use Hazelcast or Kafka as the network event stream system, you should also build the corresponding 
presence monitor (hazelcast-presence and kafka-presence).

Once the Mercury libraries and the presence monitors are built. You can start either Hazelcast or Kafka and build 
this helper app.

You can run the app with:
```
java -Dcloud.connector=event.node target/rest-automation...jar
or
java -Dcloud.connector=hazelcast -Dcloud.services=hazelcast.reporter target/rest-automation...jar
or
java -Dcloud.connector=kafka -Dcloud.services=kafka.reporter target/rest-automation...jar
```

## WebSocket automation

You can define a websocket endpoint for 2 purposes:
- a websocket server service that you write
- a websocket notification service for UI applications

### Websocket server service

To deploy a websocket endpoint for your own application, add this "websocket" section to the rest.yaml config file. 
You also need to update the "rest" section to expose a websocket access token issurance endpoint. 
You should implement your websocket authentication API. In this example, it is "v1.ws.api.auth". 
For testing, you can comment out the authentication portion.

```yaml
websocket:
  - application: "notification"
    recipient: "my.ws.handler"

rest:
  # The REST automation system includes a built-in websocket notification system.
  # If you want to enable this feature, you must add the ws.token.issuer service as
  # shown in this example. For added security, please enable authentication and point
  # it to a user defined authentication service that may authenticate the user credentials
  # and validate that the user is allowed to use a specific websocket application.
  #
  # the "ws.token.issuer" and "ws.notification" are reserved by the system for
  # websocket connection token issuance and notification topic query services
  #
  - service: "ws.token.issuer"
    methods: ['GET']
    url: "/api/ws/token/{application}"
    #    authentication: "v1.ws.api.auth"
    tracing: true
```

This asks the REST automation to route incoming websocket connection, message and close events to your function. 
In the above example, it is "my.ws.handler" that points to your custom function.

```javascript
LambdaFunction myWsHandler = (headers, body, instance) -> {
  // your custom websocket server logic here to handle open, close and message events 
  //
  // nothing to return because this is asynchronous
  return null;
};
platform.register("my.ws.handler", myWsHander, 1);
```

### Websocket open event

If you configure a "recipient" service in the websocket application, the websocket open event will be sent to the 
service. If you have deployed more than one application instance, all the instances will receive the same open events.
This allows your application to keep track of all websocket connections for the specific application.

An open event contains a header of "type" = "open". The event body contains the following:

```json
{
  "query": {"key": "value"},
  "application": "notification",
  "ip": "192.168.1.100",
  "origin": "ddd3bca7e7744a67a1b938dc67a76cd7",
  "tx_path": "ws.12345@ddd3bca7e7744a67a1b938dc67a76cd7"
}

```

To connect to your websocket server function, the UI may issue a websocket connection request to the following URL:

```
wss://hostname/ws/api/notification:{access_token}?optional_query_string
```

The application name "notification" is an example only. You can define any application name in the rest.yaml config file.

Note that your websocket server handler function will receive all websocket connection to the specific websocket 
application. Please implement logic to handle individual user which is identified by the "tx_path".

### UI keep-alive

Websocket connection is persistent. To release unused resources, REST automation will disconnect any idle websocket 
connection in 60 seconds. Please implement keep-alive by sending a "hello" message from the UI like this:

```json
{
  "type": "hello",
  "message": "keep-alive",
  "time": "ISO-8601 time-stamp"
}
```
Your UI application must implement a keep-alive protocol to send this event within a 60-second interval.
The REST automation will echo this "hello" message to the UI where it can be ignored.

### Websocket message event

For simplicity, the REST automation system supports TEXT message only.

If you configure a "recipient" service in the websocket application, the websocket message event will be sent to the 
service. The REST automation system will send the websocket message event in a load balance basis if you have deployed 
more than one application instance to handle the incoming websocket messages.

The event's body contains the incoming text message and the headers contains the following:

```json
{
  "type": "message",
  "application": "notification",
  "origin": "ddd3bca7e7744a67a1b938dc67a76cd7",
  "tx_path": "ws.12345@ddd3bca7e7744a67a1b938dc67a76cd7"
}
```
The "tx_path" is the outgoing route name for your application to send text messages. e.g.

```java
// payload can be Map or String where Map payload will be converted to a JSON string for delivery
po.send("ws.12345@ddd3bca7e7744a67a1b938dc67a76cd7", payload);
```

### Websocket close event

If you configure a "recipient" service in the websocket application, the websocket close event will be sent to the
service. If you have deployed more than one application instance, all the instances will receive the same close events.
This allows your application to keep track of all websocket connections for the specific application.

A close event contains a header of "type" = "close". The event body contains the following:

```json
{
  "application": "notification",
  "origin": "ddd3bca7e7744a67a1b938dc67a76cd7",
  "tx_path": "ws.12345@ddd3bca7e7744a67a1b938dc67a76cd7"
}
```

If your websocket server function creates temporary resource, you may release the resource using the "tx_path" as a 
reference.

WebSocket is usually employed as a notification channel to the browser so that your service can detect "presence" of 
the user and asynchronously send notification events to the browser.

The REST automation helper application supports this websocket notification use case. The sample rest.yaml 
configuration file contains a websocket routing entry to the sample.ws.auth and ws.notification services.

### Using websocket for simple notification to the browser

If your application requires only publish/subscribe feature, you can remove the "recipient" service from the websocket 
config. Just add the publish/subscibe features in the rest.yaml config file like this:

```yaml
websocket:
  - application: "notification"
    publish: true
    subscribe: true

rest:
  - service: "ws.token.issuer"
    methods: ['GET']
    url: "/api/ws/token/{application}"
    #    authentication: "v1.ws.api.auth"
    tracing: true
  - service: "ws.notification"
    methods: ['GET']
    url: "/api/notification"
    #    authentication: "v1.api.auth"
    tracing: true
  - service: "ws.notification"
    methods: ['GET']
    url: "/api/notification/{topic}"
    #    authentication: "v1.api.auth"
    tracing: true
```

The "subscribe" feature must be set to true for the browser to subscribe to one or more notification topics. 
The "publish" feature, if turn on, allows peer-to-peer messaging. For security, we recommend to set it to false. 
You can expose a REST endpoint for a user to send events through a backend service.

The `/api/notification` endpoints are for admin purpose if you want to expose them to DevOps. 
The two admin endpoints show a list of all topics or a list of websocket connections under a specific topic respectively.

### Subscribe to a notification topic

The browser can send a subscription request like this:

```json
{
  "type": "subscribe",
  "topic": "my.simple.topic"
}
```

### Unsubscribe from a notification topic

The browser can unsubscribe from a topic like this:

```json
{
  "type": "unsubscribe",
  "topic": "my.simple.topic"
}
```

A browser will also automatically unsubscribe from all subscribed topics when the browser closes. 
When the connected websocket backend service application instance fails, the websocket connection to the browser 
will be closed and current subscriptions will be dropped. The browser application should acquire a websocket access 
token and reconnect to an available backend service instance. Then subscribe to the topic(s) again.

Note that a browser can subscribe to more than one notification topics. e.g. system.alerts, user.1200, workflow.100, etc.

### Notification topic vs service route name

While both notification topics and service route names use the same convention of lower case and "dots", 
they are maintained in different registries and thus there is no conflict between the two types of names.

### Publish from a browser

If "publish" feature is turned on, the browser can send text events to a topic like this:

```json
{
  "type": "publish",
  "topic": "my.simple.topic",
  "message": "text message here"
}
```

### Publish from a backend service

For security, we recommend to disable the publish feature in rest.yaml and perform publishing from a backend service.

The following backend service APIs are available

```java
public void publish(String topic, String message);
public List<String> listTopics() throws TimeoutException, AppException, IOException;
public Map<String, List<String>> getTopic(String topic) throws TimeoutException, AppException, IOException;

```

## Sample browser application for connecting to a notification channel

The ws.html sample app is available under the resources folder and the browser app can be tested by visiting
http://127.0.0.1:8100/ws.html

## Custom HTML error page

You may customize the standardized `errorPage.html` in the resources folder for your organization.

## Static HTML folder

You can tell the rest-automation application to use a static HTML folder in the local file system with one of these 
methods:

application.properties
```
spring.resources.static-locations=file:/tmp/html
```

or startup parameters
```
java -jar rest-automation.jar -html file:/tmp/html
```

# API definition file (rest.yaml)

Please design and deploy your own rest.yaml file in /tmp/config/rest.yaml

The rest.yaml in "/tmp/config" will override the sample rest.yaml in the resources folder.

If your application does not support websocket notification channel, you can remove the websocket section in rest.yaml

# Test drive the REST automation system

You can test drive the REST automation by deploying the event-node application, the rest-automation-app and the 
lamdba-example. The quickest way to try that is to use the pm2-example/event-node or pm2-example/kafka start up scripts.

For event node, you can start a demo system with:
```bash
pm2 start event-node.json
pm2 start lambda-example.json
pm2 start rest-automation.json
```
Then you can visit the demo page at http://127.0.0.1:8100/api/hello/world

This will send a HTTP request to the REST automation application that will turn the HTTP request into an event for 
forwarding to the "hello.world" service in the lambda example. When the "hello.world" service responds, the result 
is send as an event to the "async.http.response" service for delivery to the browser.

# Test drive the Websocket notification system

With the above setup, visit http://127.0.0.1:8100/ws.html

You will see a demo websocket page that requires an access token to connect. Visit http://127.0.0.1:8100/api/ws/token 
to obtain an access token.

Once you are connected, you can subscribe to a notification topic. e.g. "system.alerts".

To publish to the browser, open another browser, obtain an access token above to connect to the websocket notification 
system. Then enter "system.alerts:hello world" and press "publish". This will send the "hello world" message to the 
first browser.

## IMPORTANT

The REST automation application sub-project contains a demo websocket HTML page (`ws.html`) in the resources/public 
folder and the rest.yaml config file in the resources folder has rest entries to support the websocket demo.

They are provided as a demo and a template. Please remove or adjust them before you deploy the REST automation system 
for production.
