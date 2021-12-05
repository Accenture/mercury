# Traditional REST endpoints

The recommended way to create REST endpoints is the REST automation system described in chapter 4.

However, if you want to write REST and websocket endpoints in traditional ways, Mercury supports the following 
for backward compatibility.

1. JAX-RS annotation
2. Spring REST controller
3. Java Servlet

### JAX-RS annotated endpoints

You class should include the `@Path(String pathPrefix)` annotation. Set pathPrefix accordingly. 

All JAX-RS REST endpoints are prefixed with "/api". If your pathPrefix is "/hello". The path is "/api/hello".

In your REST endpoint method, you should use another `@Path` to indicate any additional path information. 
Note that you can use `{pathParam}` in the URL path.
If you have pathParam, please add `@PathParam` parameter annotation to the argument in your method.

You can set the HTTP method in the method with `@GET`, `@POST`, `@PUT`, `@DELETE`, etc.
and specify content types in `@Consumes` and `@Produces` method annotation.

If you want your REST endpoint to be optional when a property in the application.properties exists, annotate your 
class with `OptionalService`.

Please refer to example code in `rest-example` and the [JAX-RS reference site](https://jersey.github.io/) for details.

### Java Servlet

For low level control, you can use the `@WebServlet(String urlPath)` to annotate your Servlet class that must 
extend the `HttpServlet` class.

If you want your Java Servlet to be optional when a property in the application.properties exists, annotate your
class with `OptionalService`.

### Spring REST controller

If you are more familar with the Spring framework, you may use Spring REST controller.
Note that this will create tight coupling and make your code less portable.

Please refer to [Spring documentation](https://spring.io/guides/gs/rest-service/) for details. 
Note that Spring REST controllers are installed in the URL root path.

## Websocket service

You can use `@WebSocketService(handlerName)` to annotate a websocket service that implements the
`LambdaFunction` interface. Your websocket service function becomes the incoming event handler for websocket. 
The system will automatically create an outgoing message handler that works with your websocket service.

Please refer to the sample code `WsEchoDemo` in the `rest-example`.

Mercury is 100% event-driven. This includes websocket service. The sample code includes example for 
OPEN, CLOSE, BYTES and TEXT events. In the OPEN event, you can detect the query parameter and token.

For standardization, websocket service uses a URL path as follows:

```
ws://host:port/ws/{handlerName}/{token}
```
The websocket client application or browser that connects to the websocket service must provide the "token" 
which is usually used for authentication.

IMPORTANT: websocket does not support custom HTTP headers for authentication. As a result, we usually use an 
authentication token in the URL or query parameter. For the Mercury framework, we recommend the use of a token 
as a URL suffix. Typically, a user logs on to your application with a REST endpoint like "/login" and then a 
session cookie is created. The browser may then use the sessionId as a token in the websocket connection.

`Websocket idle timeout` - there is a one-minute idle timeout for all websocket connection. 
To keep the websocket connection alive, you may send a message to the websocket service. 
For example, sending a BYTES or TEXT message to the service with some agreed protocol.

To disconnect a websocket connection, you may use the Utility class as follows:

```
void closeConnection(String txPath, CloseReason.CloseCodes status, String message) throws IOException;
```
The txPath is available to the websocket service mentioned earlier.

## Calling other microservices functions

You may call other microservices functions from your REST and websocket endpoints using the `send` or `request` 
methods of the Post Office.

## Static contents for HTML, CSS and Javascript

Under the hood, we are using Spring Boot. Therefore you may put static contents under the "/public" folder in the 
project's "resources". The static contents will be bundled with your executable application JAR when you 
do `mvn clean package`.

## Loose coupling with Spring Boot

The Mercury framework is loosely coupled with Spring Boot to support REST and websocket endpoints. 
The light-weight application server for Spring Boot can be Tomcat, Jetty or Undertow.

For consistency, we have optimized Spring Boot with custom serializers and exception handlers in the `rest-spring` 
module.

If you know what you are doing, you can use Spring Boot feature directly with the exception of the `@SpringApplication` 
annotation because we use the `@MainApplication` to enable additional automation. You can change the behavior of 
Spring Boot including auto-config classes using the `application.properties` file in the resources folder in the 
maven project.

We are currently using Tomcat. If your organization prefers Jetty or Undertow, you may adjust the pom.xml file in 
the `rest-spring` and `platform-core` projects.

## application.properties

In additon to the parameters defined by Spring Boot, the Mercury framework uses the following parameters.

1. web.component.scan - you should add your organizaton packages as a comma separated list to tell Mercury to scan 
   for your packages.
2. snake.case.serialization - we recommend the use of snake case for modern API
3. safe.data.models - Optional. For higher security, you may specify a list of safe data models to be accepted by 
   the serialization engine. This is to protect against hidden "evil" Java classes in certain open sources that 
   you have not vetted directly.
4. protected.info.endpoints - Optional. You may protect certain "known" REST endpoints such as "/info" or "/env" 
   from unauthorized access. It uses a simple API key set in the environment.
5. env.variables - parameters in the application.properties are automatically overriden by Java properties. 
   To allow some environment variables to override your run-time parameters, you may define them in this parameter.
6. spring.application.name/application.name, info.app.version and info.app.description - please update application 
   name and information before you start your project. spring.application.name and application.name can be used 
   interchangeably.

---

| Chapter-6                           | Home                                     |
| :----------------------------------:|:----------------------------------------:|
| [Cloud connectors](CHAPTER-6.md)    | [Table of Contents](TABLE-OF-CONTENTS.md)|
