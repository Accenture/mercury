# Build, test and deploy

The first step in writing an application is to create an entry point for your application.

## Main application

A minimalist main application template is shown as follows:

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

Note that MainApplication is mandatory. You must have at least one "main application" module.

> Note: Please adjust the parameter "web.component.scan" in application.properties 
        to point to your user application package(s) in your source code project.

If your whole application uses the "declarative" approach in defining your functions, you may just print
a greeting message in the "start" method. If you have additional start up logic, you may add them in the
"start" method.

The `AppStarter.main()` statement in the "main" method is used when you want to start your application directly
from the IDE. You can "right-click" the main method and select "run".

You can also build and run the application from command line like this:

```shell
cd sandbox/mercury/examples/lambda-example
mvn clean package
java -jar target/lambda-example-3.0.0.jar
```

The lambda-example is a sample application that you can use as a template to write your own code. Please review
the pom.xml and the source directory structure. The pom.xml is pre-configured to support Java and Kotlin.

In the lambda-example project root, you will find the following directories:

```shell
src/main/java
src/main/kotlin
src/test/java
```

Note that kotlin unit test directory is not included because you can test all functions in Java unit tests.

Since all functions are connected using the in-memory event bus, you can test any function by sending events
from a unit test module in Java. If you are comfortable with the Kotlin language, you may also set up Kotlin 
unit tests accordingly. There is no harm having both types of unit tests in the same project.

## Source code documentation

Since the source project contains both Java and Kotlin, we have replaced javadoc maven plugin with Jetbrains "dokka"
documentation engine for both Java and Kotlin. Javadoc is useful if you want to write and publish your own libraries.

To generate Java and Kotlin source documentation, please run "mvn dokka:dokka". You may "cd" to the platform-core
project to try the maven dokka command to generate some source documentation. The doc home page will be available
in "target/dokka/index.html"

## Writing your functions

Please follow the step-by-step learning guide in [Chapter-1](CHAPTER-1.md) to write your own functions. You can then
configure new REST endpoints to use your new functions.

In [Chapter-1](CHAPTER-2.md), we have discussed the three function execution strategies to optimize your application to the full
potential of stability, performance and throughput.

## HTTP forwarding

In [Chapter-3](CHAPTER-3.md), we have presented the configuration syntax for the "rest.yaml" REST automation definition file.
Please review the sample rest.yaml file in the lambda-example project. You may notice that it has an entry 
for HTTP forwarding. The following entry in the sample rest.yaml file illustrates a HTTP forwarding endpoint.
In HTTP forwarding, you can replace the "service" route name with a direct HTTP target host. You can do "URL rewrite"
to change the URL path to the target endpoint path. In the below example, `/api/v1/*` will be mapped to `/api/*`
in the target endpoint.

```yaml
  - service: "http://127.0.0.1:${rest.server.port}"
    trust_all_cert: true
    methods: ['GET', 'PUT', 'POST']
    url: "/api/v1/*"
    url_rewrite: ['/api/v1', '/api']
    timeout: 20
    cors: cors_1
    headers: header_1
    tracing: true
```

## Sending HTTP request event to more than one service

One feature in REST automation "rest.yaml" configuration is that you can configure more than one function in the
"service" section. In the following example, there are two function route names ("hello.world" and "hello.copy"). 
The first one "hello.world" is the primary service provider. The second one "hello.copy" will receive a copy of 
the incoming event automatically.

This feature allows you to write new version of a function without disruption to current functionality. Once you are
happy with the new version of function, you can route the endpoint directly to the new version by updating the
"rest.yaml" configuration file.

```yaml
  - service: ["hello.world", "hello.copy"]
```

## Writing your first unit test

Please refer to "rpcTest" method in the "HelloWorldTest" class in the lambda-example to get started.

In unit test, we want to start the main application so that all the functions are ready for tests.

First, we write a "TestBase" class to use the BeforeClass setup method to start the main application like this:

```java
public class TestBase {

    private static final AtomicInteger seq = new AtomicInteger(0);

    @BeforeClass
    public static void setup() {
        if (seq.incrementAndGet() == 1) {
            AppStarter.main(new String[0]);
        }
    }
}
```

The atomic integer "seq" is used to ensure the main application entry point is executed only once.

Your first unit test may look like this:

```java
@SuppressWarnings("unchecked")
@Test
public void rpcTest() throws IOException, InterruptedException {
    Utility util = Utility.getInstance();
    BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
    String name = "hello";
    String address = "world";
    String telephone = "123-456-7890";
    DemoPoJo pojo = new DemoPoJo(name, address, telephone);
    PostOffice po = new PostOffice("unit.test", "12345", "POST /api/hello/world");
    EventEnvelope request = new EventEnvelope().setTo("hello.world")
                                .setHeader("a", "b").setBody(pojo.toMap());
    po.asyncRequest(request, 800).onSuccess(bench::offer);
    EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
    assert response != null;
    Assert.assertEquals(HashMap.class, response.getBody().getClass());
    MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
    Assert.assertEquals("b", map.getElement("headers.a"));
    Assert.assertEquals(name, map.getElement("body.name"));
    Assert.assertEquals(address, map.getElement("body.address"));
    Assert.assertEquals(telephone, map.getElement("body.telephone"));
    Assert.assertEquals(util.date2str(pojo.time), map.getElement("body.time"));
}
```

Note that the PostOffice instance can be created with tracing information in a Unit Test. The above example
tells the system that the sender is "unit.test", the trace ID is 12345 and the trace path is "POST /api/hello/world".

For unit test, we need to convert the asynchronous code into "synchronous" so that unit test can run sequentially.
"BlockingQueue" is a good choice for this.

The "hello.world" is an echo function. The above unit test sends an event containing a key-value {"a":"b"} and
the payload of a HashMap from the DemoPoJo.

If the function is designed to handle PoJo, we can send PoJo directly instead of a Map.

> IMPORTANT: blocking code should only be used for unit tests. DO NOT use blocking code in your
             application code because it will block the event system and dramatically slow down
             your application.

### Convenient utility classes

The Utility and MultiLevelMap classes are convenient tools for unit tests. In the above example, we use the
Utility class to convert a date object into a UTC timestamp. It is because date object is serialized as a UTC
timestamp in an event.

The MultiLevelMap supports reading an element using the convenient "dot and bracket" format.

For example, given a map like this:
```json
{
  "body":
  {
    "time": "2023-03-27T18:10:34.234Z",
    "hello": [1, 2, 3]
  }
}
```

| Example | Command                         | Result                   |
|:-------:|:--------------------------------|:-------------------------|
|    1    | map.getElement("body.time")     | 2023-03-27T18:10:34.234Z |
|    2    | map.getElement("body.hello[2]") | 3                        |

## The second unit test

Let's do a unit test for PoJo. In this second unit test, it sends a RPC request to the "hello.pojo" function that
is designed to return a SamplePoJo object with some mock data.

Please refer to "pojoRpcTest" method in the "PoJoTest" class in the lambda-example for details.

The unit test verifies that the "hello.pojo" has correctly returned the SamplePoJo object with the pre-defined 
mock value.

```java
@Test
public void pojoTest() throws IOException, InterruptedException {
    Integer ID = 1;
    String NAME = "Simple PoJo class";
    String ADDRESS = "100 World Blvd, Planet Earth";
    BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
    PostOffice po = new PostOffice("unit.test", "20001", "GET /api/hello/pojo");
    EventEnvelope request = new EventEnvelope().setTo("hello.pojo").setHeader("id", "1");
    po.asyncRequest(request, 800).onSuccess(bench::offer);
    EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
    assert response != null;
    Assert.assertEquals(SamplePoJo.class, response.getBody().getClass());
    SamplePoJo pojo = response.getBody(SamplePoJo.class);
    Assert.assertEquals(ID, pojo.getId());
    Assert.assertEquals(NAME, pojo.getName());
    Assert.assertEquals(ADDRESS, pojo.getAddress());
}
```

Note that you can do class "casting" or use the built-in casting API as shown below:

> SamplePoJo pojo = (SamplePoJo) response.getBody()

> SamplePoJo pojo = response.getBody(SamplePoJo.class)

## The third unit test

Testing Kotlin suspend functions is challenging. However, testing suspend function using events is straight forward
because of loose coupling.

Let's do a unit test for the lambda-example's FileUploadDemo function. Its route name is "hello.upload".

Please refer to "uploadTest" method in the "SuspendFunctionTest" class in the lambda-example for details.

```java
@SuppressWarnings("unchecked")
@Test
public void uploadTest() throws IOException, InterruptedException {
    String FILENAME = "unit-test-data.txt";
    BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
    Utility util = Utility.getInstance();
    PostOffice po = PostOffice.getInstance();
    int len = 0;
    ByteArrayOutputStream bytes = new ByteArrayOutputStream();
    ObjectStreamIO stream = new ObjectStreamIO();
    ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
    for (int i=0; i < 10; i++) {
        String line = "hello world "+i+"\n";
        byte[] d = util.getUTF(line);
        out.write(d);
        bytes.write(d);
        len += d.length;
    }
    out.close();
    // emulate a multi-part file upload
    AsyncHttpRequest req = new AsyncHttpRequest();
    req.setMethod("POST");
    req.setUrl("/api/upload/demo");
    req.setTargetHost("http://127.0.0.1:8080");
    req.setHeader("accept", "application/json");
    req.setHeader("content-type", "multipart/form-data");
    req.setContentLength(len);
    req.setFileName(FILENAME);
    req.setStreamRoute(stream.getInputStreamId());
    // send the HTTP request event to the "hello.upload" function
    EventEnvelope request = new EventEnvelope().setTo("hello.upload").setBody(req);
    po.asyncRequest(request, 8000).onSuccess(bench::offer);
    EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
    assert response != null;
    Assert.assertEquals(HashMap.class, response.getBody().getClass());
    Map<String, Object> map = (Map<String, Object>) response.getBody();
    System.out.println(response.getBody());
    Assert.assertEquals(len, map.get("expected_size"));
    Assert.assertEquals(len, map.get("actual_size"));
    Assert.assertEquals(FILENAME, map.get("filename"));
    Assert.assertEquals("Upload completed", map.get("message"));
    // finally check that "hello.upload" has saved the test file
    File dir = new File("/tmp/upload-download-demo");
    File file = new File(dir, FILENAME);
    Assert.assertTrue(file.exists());
    Assert.assertEquals(len, file.length());
    // compare file content
    byte[] b = Utility.getInstance().file2bytes(file);
    Assert.assertArrayEquals(bytes.toByteArray(), b);
}
```

In the above unit test, we use the ObjectStreamIO to emulate a file stream and write 10 blocks of data into it.
The unit test then makes an RPC call to the "hello.upload" with the emulated HTTP request event.

The "hello.upload" is a Kotlin suspend function. It will be executed when the event arrives.
After saving the test file, it will return a HTTP response object that the unit test can validate.

In this fashion, you can create unit tests to test suspend functions in an event-driven manner.

## Deployment

The pom.xml is pre-configured to generate an executable JAR. The following is extracted from the pom.xml.

The main class is `AppStarter` that will load the "main application" and use it as the entry point to
run the application.

```xml
<plugin>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-maven-plugin</artifactId>
    <configuration>
        <mainClass>org.platformlambda.core.system.AppStarter</mainClass>
    </configuration>
    <executions>
        <execution>
            <id>build-info</id>
            <goals>
                <goal>build-info</goal>
            </goals>
        </execution>
    </executions>
</plugin>
```

Composable application is designed to be deployable using Kubernetes or serverless.

A sample Dockerfile for an executable JAR may look like this:

```shell
FROM adoptopenjdk/openjdk11:jre-11.0.11_9-alpine
EXPOSE 8083
WORKDIR /app
COPY target/your-app-name.jar .
ENTRYPOINT ["java","-jar","your-app-name.jar"]
```

## Distributed tracing

The system has a built-in distributed tracing feature. You can enable tracing for any REST endpoint by adding
"tracing=true" in the endpoint definition in the "rest.yaml" configuration file.

You may also upload performance metrics from the distributed tracing data to your favorite telemetry system dashboard.

To do that, please implement a custom metrics function with the route name `distributed.trace.forwarder`.

The input to the function will be a HashMap like this:

```shell
trace={path=/api/upload/demo, service=hello.upload, success=true, 
       origin=2023032731e2a5eeae8f4da09f3d9ac6b55fb0a4, 
       exec_time=77.462, start=2023-03-27T19:38:30.061Z, 
       from=http.request, id=12345, round_trip=132.296, status=200}
```

The system will detect if `distributed.trace.forwarder` is available. If yes, it will forward performance metrics
from distributed trace to your custom function.

## Request-response journaling

Optionally, you may also implement a custom audit function named `transaction.journal.recorder` to monitor 
request-response payloads.

To enable journaling, please add this to the application.properties file.

```properties
journal.yaml=classpath:/journal.yaml
```
and add the "journal.yaml" configuration file to the project's resources folder with content like this:

```yaml
journal:
  - "my.test.function"
  - "another.function"
```

In the above example, the "my.test.function" and "another.function" will be monitored and their request-response
payloads will be forwarded to your custom audit function. The input to your audit function will be a HashMap
containing the performance metrics data and a "journal" section with the request and response payloads in clear form.

> IMPORTANT: journaling may contain sensitive personally identifiable data and secrets. Please check
             security compliance before storing them into an access restricted audit data store.

<br/>

|              Chapter-4              |                   Home                    |          Chapter-6          |
|:-----------------------------------:|:-----------------------------------------:|:---------------------------:|
| [Event orchestration](CHAPTER-4.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [Spring Boot](CHAPTER-6.md) |
