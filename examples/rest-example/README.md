# Mercury - the Post Office for microservices

## REST Endpoint example

This is an example for writing REST and websocket endpoints that can consume microservices.
It can be compiled and packaged as an executable Spring Boot application.

## REST endpoints

You may write REST endpoints in 3 ways.

1. JAX-RS Java REST reference implementation

Please refer to JaxRest and JaxRestConcurrent as examples. The endpoints are available at http://127.0.0.1:8083/api/hello/world and http://127.0.0.1:8083/api/hello/concurrent

2. Spring RestController

If you use Spring RestController annotation, your code is not portable. We therefore recommend the use of JAX-RS above.

3. WebServlet, WebFilter and WebListener

If you want low level control, you may write servlets directly. This is shown in DemoServlet. The endpoint is http://127.0.0.1:8083/demo

## Websocket server endpoints

Websocket server endpoints are useful for 2-way communication. Typical use is "service notification". For example, you can implement "lazy loading" using a websocket connection so that the service can send partial results to the browser.

The platform is 100% event driven. Each websocket connection is supported by two lambda functions. One for transmit and one for receive.

The websocket transmitter is provided by the platform automatically. You just need to write a lambda function to take care of incoming message.

An example for a websocket lambda function is shown in WsEchoDemo. The URL is ws://127.0.0.1:8083/ws/hello/{origin}

```
The URL path for your websocket server is always ws://host:port/ws/{path}/{origin}
```
The "path" can be specificed in the annotation "WebSocketService". The "origin" is given by the browser application. 
You websocket lambda function can get this value from the WsEnvelope as shown in the example.

## Main Application

Your main application should have the annotation "MainApplication" and implement the "EntryPoint" interface. Please see the MainApp example.

## Making requests to backend com.accenture.examples.services

You may use the PostOffice to make requests to backend microservices in the web-tier, app-tier and resources-tier. An example is shown in JaxRest.

## Static HTML contents

You may put static contents under the resources/public folder.

## Application configuration

The "root" configuration file is "application.properties".

## Custom HTML error page

You may modify the errorPage.html for your organization if needed.

## Logging

The logging configuration can be modified in logback.xml

## Unit tests

The source tree contains a "test" branch. You can use the "Test" annotation and "Assert" APIs to do unit tests.

## Advanced uses

The example demonstrates the use of RPC (request-response) pattern. More advanced examples include async, callback, pipeline and streaming.

## Executable JAR file packaging

A sample POM configuration is shown in pom-for-spring.xml. Copy it to pom.xml and do "mvn clean package" to generate an executable Spring Boot application.

## End to end test drive

1. Compile and install the platform-core, rest-core and rest-spring projects as libraries locally using "mvn clean install" in the 3 projects.
2. Compile and package the "Event Node" cloud emulator with "mvn clean package" for local test. Run it in a command terminal using "java -jar target\event-node-(version).jar"
3. Compile and package this lambda-example project with "mvn clean package". Run it in a command terminal.
4. Compile and package this rest-example project with "mvn clean package". Run it in a command terminal.
5. Visit http://127.0.0.1:8083/api/hello/pojo/1 to try out end-to-end test from the rest-example app to the lambda-example app via the Event Node.
6. Visit http://127.0.0.1:8083/api/hello/world to try out local demo service.

## Running in the cloud

You may connect your applications using the Event Node for local testing.

To deploy your application to the cloud, you must set cloud.connector correctly.
