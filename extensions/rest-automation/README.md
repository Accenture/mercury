# REST Automation

This is a convenient helper application to automate REST endpoint creation and routing for microservices that use the Mercury framework.

## REST endpoint creation

Instead of creating REST endpoints programmatically, this application allows REST endpoints to be created declaratively.
i.e. REST endpoints can be created with simple routing syntax in a YAML file.

## REST endpoint routing

A routing entry is a simple mapping of URL to a microservice.

## YAML syntax

```yaml
rest:
  - service: "hello.world"
    methods: ['GET', 'POST']
    url: "/api/hello/world"
    timeout: 10s
    # optional authentication service
#    authentication: "v1.api.auth"
    threshold: 30000
    cors: cors_1
    headers: header_1

  - service: "hello.world"
    methods: ['GET', 'PUT', 'POST']
    url: "/api/test/ok*"
    # optional "upload" key if it is a multi-part file upload
    upload: "file"
    timeout: 15s
    headers: header_1
#    cors: cors_1

  - service: "hello.world"
    methods: ['GET', 'PUT', 'POST']
    url: "/api/nice/{task}/*"
    timeout: 12
    headers: header_1
#    cors: cors_1

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

The `service` must be a service that is registered as public function. The required service does not need to be available when the REST automation helper application starts.

The system will find the service at run-time. If the service is not available, the user will see an exception "Route not found".

The `keep` and `drop` entries are mutually exclusive where `keep` has precedent over `drop`. When keep is empty and drop is not, it will drop only the headers in the drop list.
The `add` entry allow the developer to insert additional header key-values before it reaches the target service that serves the REST endpoint.

For content types of XML and JSON, the system will try to convert the input stream into a map.

For binary content type, the system will check if the input stream is larger than a threshold.
`threshold` - the default threshold buffer is 50,000 bytes, min is 5,000 and max is 500,000

If the input stream is small, it will convert the input stream into a byte array and store it in the "body" parameter.

if the input stream is large, it will return a stream ID so that your service can read the data as a stream.

Cors header processing is optional. If present, it will return the configured key-values for pre-flight and regular responses.

## URL matching

For performance and readability reason, the system is not using regular expression for URL matching. Instead it is using wildcard and path parameters.

### wildcard

The wildcard `*` character may be used as a URL path segment or as a suffix in a segment. Characters after the wildcard character in a segment will be ignored.

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

The HTTP request (headers, query, body) will be encoded as a Map and the target service will receive it as the message `body`.

```yaml
headers: key-values of HTTP headers
method: HTTP request method
ip: caller's remote IP address
parameters: 
  path: path parameters if any
  query: query or URL encoded request body parameters if any
url: URL of the incoming request
timeout: timeout value (in seconds) of the REST endpoint

# optional
stream: stream-ID of incoming data stream (e.g. file)
filename: filename if it is a file upload
content_length: number of bytes of the incoming stream

```

### HttpResponseBody

For simple use case, you can just send text, bytes or Map as the return value in your service.

For advanced use cases, you can send status code, set HTTP headers if your service returns an EventEnvelope object.

### HTTP status code

If your service set status code directly, you should use the standard 3-digit HTTP status code because it will be sent to the browser directly.
i.e. 200 means OK and 404 for NOT_FOUND, etc.

### Exception handling

If you service throws exception, it will automatically translate into a regular HTTP exception with status code and message.

To control the HTTP error code, your service should use the AppException class. It allows you to set HTTP status code and error message directly.

### Input stream

If you want to support file upload, you can specify the `upload` parameter in the rest.yaml configuration file.
The default value is "file". If your user uses different tag, you must change the `upload` parameter to match the upload tag.

If incoming request is a file, your service will see the "stream", "filename" and "content_length" parameters.
If incoming request is a byte stream, your service will find the "stream" and "content_length" parameters.

### Output stream

If your service wants to send the output to the browser as a stream of text or bytes, you can create an ObjectStreamIO with a timeout value.
You can then set the streamId in the HTTP header "stream". You should also set the HTTP header "timeout" to tell the REST endpoint to use it as IO stream read timeout value.
The "stream" and "timeout" headers are used by the REST automation framework. They will not be set as HTTP response headers.

You timeout value should be short and yet good enough for your service to send one block of data. The timer will be reset when there is I/O activity. 
One use case of output stream is file download.

### Location of the YAML configuration file

By default, the system will search for `file:/tmp/config/rest.yaml` and then `classpath:/rest.yaml`.

If needed, you can change this in the `rest.automation.yaml` property in the application.properties file.

The `/tmp/config/rest.yaml` allows you to externalize the REST automation configuration without rebuilding the REST automation helper application.

### Running this helper application

Before you build this application, you should follow the README in the top level project to build the Mercury libraries (platform-core, rest-core, rest-spring, kafka-connector, hazelcast-connector).

If you use the Event Node "platform-in-box" for dev and test, you can simply build the Event Node and run it in a command terminal.

Then you can build this helper app and run with:
```
java -Dcloud.connector=event.node target/rest-automation...jar
```

If you plan to use Hazelcast or Kafka as the network event stream system, you should also build the corresponding presence monitor (hazelcast-presence and kafka-presence).

Once the Mercury libraries and the presence monitors are built. You can start either Hazelcast or Kafka and build this helper app.

You can run the app with:
```
java -Dcloud.connector=hazelcast -Dcloud.services=hazelcast.reporter target/rest-automation...jar
or
java -Dcloud.connector=kafka -Dcloud.services=kafka.reporter target/rest-automation...jar
```
