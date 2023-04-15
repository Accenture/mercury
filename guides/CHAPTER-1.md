# Introduction

Mercury version 3 is a toolkit for writing composable applications.

At the platform level, composable architecture refers to loosely coupled platform services, utilities, and
business applications. With modular design, you can assemble platform components and applications to create
new use cases or to adjust for ever-changing business environment and requirements. Domain driven design (DDD),
Command Query Responsibility Segregation (CQRS) and Microservices patterns are the popular tools that architects
use to build composable architecture. You may deploy application in container, serverless or other means.

At the application level, a composable application means that an application is assembled from modular software
components or functions that are self-contained and pluggable. You can mix-n-match functions to form new applications.
You can retire outdated functions without adverse side effect to a production system. Multiple versions of a function
can exist, and you can decide how to route user requests to different versions of a function. Applications would be
easier to design, develop, maintain, deploy, and scale.

## Build the platform libraries

The first step is to build Mercury libraries from source.
To simplify the process, you may publish the libraries to your enterprise artifactory.

```shell
mkdir sandbox
cd sandox
git clone https://github.com/Accenture/mercury.git
cd mercury
mvn clean install
```

The above sample script clones the Mercury open sources project and builds the libraries from source.

The pre-requisite is maven 3.8.6 and openjdk 1.8 or higher. We have tested mercury with Java version 1.8 to 19.

This will build the mercury libraries and the sample applications.

The `platform-core` project is the foundation library for writing composable application.

## Run the lambda-example application

Assuming you follow the suggested project directory above, you can run a sample composable application
called "lambda-example" like this:

```shell
cd sandbox/mercury/examples/lambda-example
java -jar target/lambda-example-3.0.0.jar
```

You will find the following console output when the app starts

```text
Exact API paths [/api/event, /api/hello/download, /api/hello/upload, /api/hello/world]
Wildcard API paths [/api/hello/download/{filename}, /api/hello/generic/{id}]
```

Application parameters are defined in the resources/application.properties file (or application.yml if you prefer).
When `rest.automation=true` is defined, the system will parse the "rest.yaml" configuration for REST endpoints.

## REST automation

When REST automation is turned on, the system will start a lightweight non-blocking HTTP server.
By default, it will search for the "rest.yaml" file from "/tmp/config/rest.yaml" and then from "classpath:/rest.yaml".
Classpath refers to configuration files under the "resources" folder in your source code project.

To instruct the system to load from a specific path. You can add the `rest.automation.yaml` parameter.

To select another server port, change the `rest.server.port` parameter.

```properties
rest.server.port=8085
rest.automation=true
rest.automation.yaml=classpath:/rest.yaml
```

To create a REST endpoint, you can add an entry in the "rest" section of the "rest.yaml" config file like this:

```yaml
  - service: "hello.download"
    methods: [ 'GET' ]
    url: "/api/hello/download"
    timeout: 20s
    cors: cors_1
    headers: header_1
    tracing: true
```

The above example creates the "/api/hello/download" endpoint to route requests to the "hello.download" function.
We will elaborate more about REST automation in [Chapter-3](CHAPTER-3.md).

## Function is an event handler

A function is executed when an event arrives. You can define a "route name" for each function.
It is created by a class implementing one of the following interfaces:

1. `TypedLambdaFunction` allows you to use PoJo or HashMap as input and output
2. `LambdaFunction` is untyped, but it will transport PoJo from the caller to the input of your function
3. `KotlinLambdaFunction` is a typed lambda function using Kotlin suspend function

## Execute the "hello.world" function

With the application started in a command terminal, please use a browser to point to:
http://127.0.0.1:8085/api/hello/world

It will echo the HTTP headers from the browser like this:

```json
{
  "headers": {},
  "instance": 1,
  "origin": "20230324b709495174a649f1b36d401f43167ba9",
  "body": {
    "headers": {
      "sec-fetch-mode": "navigate",
      "sec-fetch-site": "none",
      "sec-ch-ua-mobile": "?0",
      "accept-language": "en-US,en;q=0.9",
      "sec-ch-ua-platform": "\"Windows\"",
      "upgrade-insecure-requests": "1",
      "sec-fetch-user": "?1",
      "accept": "text/html,application/xhtml+xml,application/xml,*/*",
      "sec-fetch-dest": "document",
      "user-agent": "Mozilla/5.0 Chrome/111.0.0.0"
    },
    "method": "GET",
    "ip": "127.0.0.1",
    "https": false,
    "url": "/api/hello/world",
    "timeout": 10
  }
}
```

### Where is the "hello.world" function?

The function is defined in the MainApp class in the source project with the following segment of code:

```java
LambdaFunction echo = (headers, input, instance) -> {
    log.info("echo #{} got a request", instance);
    Map<String, Object> result = new HashMap<>();
    result.put("headers", headers);
    result.put("body", input);
    result.put("instance", instance);
    result.put("origin", platform.getOrigin());
    return result;
};
// Register the above inline lambda function
platform.register("hello.world", echo, 10);
```

The Hello World function is written as an "inline lambda function". It is registered programmatically using
the `platform.register` API.

The rest of the functions are written using regular classes implementing the LambdaFunction, TypedLambdaFunction
and KotlinLambdaFunction interfaces.

## TypedLambdaFunction

Let's examine the `SimpleDemoEndpoint` example under the "services" folder. It may look like this:

```java
@CoroutineRunner
@PreLoad(route = "hello.simple", instances = 10)
public class SimpleDemoEndpoint implements TypedLambdaFunction<AsyncHttpRequest, Object> {
    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) {
        // business logic here
    }
}
```

The `PreLoad` annotation assigns a route name to the Java class and registers it with an in-memory event system.
The `instances` parameter tells the system to create a number of workers to serve concurrent requests.

> Note that you don't need a lot of workers to handle a larger number of users
  and requests provided that your function can finish execution very quickly.

The `CoroutineRunner` annotation advises the system to run the function as a "coroutine".
There are three function execution strategies (Kernel thread pool, coroutine and suspend function).
We will explain the concept in [Chapter-2](CHAPTER-2.md)

In a composable application, a function is designed using the first principle of "input-process-output".

In the "hello.simple" function, the input is an HTTP request expressed as a class of `AsyncHttpRequest`.
You can ignore `headers` input argument for the moment. We will cover it later.

The output is declared as "Object" so that the function can return any data structure using a HashMap or PoJo.

You may want to review the REST endpoint `/api/simple/{task}/*` in the rest.yaml config file to see how it is
connected to the "hello.simple" function.

We take a minimalist approach for the rest.yaml syntax. The parser will detect any syntax errors. Please check
application log to ensure all REST endpoint entries in rest.yaml file are valid.

## Write your first function

Using the lambda-example as a template, let's create your first function by adding a function in the 
"services" package folder. You will give it the route name "my.first.function" in the "PreLoad" annotation.

> Note that route name must use lower case letters and numbers separated by the period character.

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

To connect this function with a REST endpoint, you can declare a new REST endpoint in the rest.yaml like this:

```yaml
  - service: "my.first.function"
    methods: [ 'GET' ]
    url: "/api/hello/my/function"
    timeout: 20s
    cors: cors_1
    headers: header_1
    tracing: true
```

If you do not put any business logic, the above function will echo the incoming HTTP request object back to the
browser.

Now you can examine the input HTTP request object and perform some data transformation before returning a result.

The AsyncHttpRequest class allows you to access data structure such as HTTP method, URL, path parameters,
query parameters, cookies, etc.

When you click the "rebuild" button in IDE and run the "MainApp", the new function will be available in the 
application. Alternatively, you can also do `mvn clean package` to generate a new executable JAR and run the 
JAR from command line.

To test your new function, visit http://127.0.0.1:8085/api/hello/my/function

## Event driven design

Your function automatically uses an in-memory event bus. The HTTP request from the browser is converted to
an event by the system for delivery to your function as the "input" argument.

The underlying HTTP server is asynchronous and non-blocking.
i.e. it does not consume CPU resources while waiting for a response.

This composable architecture allows you to design and implement applications so that you have precise control of
performance and throughput. Performance tuning is much easier.

## Deploy your new application

You can assemble related functions in a single composable application, and it can be compiled and built into
a single "executable" for deployment using `mvn clean package`.

The executable JAR is in the target folder. 

Composable application is by definition cloud native. It is designed to be deployable using Kubernetes or serverless.

A sample Dockerfile for your executable JAR may look like this:

```shell
FROM adoptopenjdk/openjdk11:jre-11.0.11_9-alpine
EXPOSE 8083
WORKDIR /app
COPY target/your-app-name.jar .
ENTRYPOINT ["java","-jar","your-app-name.jar"]
```
<br/>

|                   Home                    |                  Chapter-2                  |
|:-----------------------------------------:|:-------------------------------------------:|
| [Table of Contents](TABLE-OF-CONTENTS.md) | [Function Execution Strategy](CHAPTER-2.md) |
