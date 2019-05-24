# Mercury - the Post Office for microservices

## microservices example

This is an example project to demonstrate how to create a microservices container as an executable JAR without REST application server.

## Import the platform core library

This is done by simply adding the following to the library dependencies.

```
<dependency>
    <groupId>org.platformlambda</groupId>
    <artifactId>platform-core</artifactId>
    <version>1.11.27</version>
</dependency>
```

The platform core library should normally be fetched from a repository like Nexus or JFrog.

## Packaging your services as an executable

To generate an executable JAR, perform the following:

```
mvn clean package
```

## application name and properties

You should setup an application name in the pom.xml and the application.properties files.

## application entry point

An example for the application entry point is shown in the MainApp class.
To make a class the entry point, annotate it with "MainApplication" and implements the EntryPoint interface.

To run the MainApp from the IDE, please add the following static method:

```
public static void main(String[] args) {
    AppStarter.main(args);
}
```


## Using the platform core library

To start using the platform core library, obtain a handle of the platform module first.

```
Platform platform = Platform.getInstance();
```

## Lambda function as a microservice


You can then create a lambda function using Java 1.8 syntax as follows:

```
LambdaFunction f = (headers, body, instance) -> {
    // business logic here
    return something;
}
```

For more complex function, you may create a class that implements the LambdaFunction interface.

If you create a Java class as a lambda, please use variables in functional scope as a best practice.

However, if your function is a "singleton", you can use private variables because they are used exclusively by the function.
Otherwise, multiple instances of the function will compete for the same variable and data integrity is not guaranteed.

Sometimes, you want to allow sharing memory among multiple instances of the same service in the same VM.
In this case, you may use a concurrent hash map.


## Register your function with concurrency

```
LambdaFunction echo = (headers, body, instance) -> {
    // simple echo function
    return body;
}
platform.register("hello.world", echo, 10);
```

In the above example, you have created an echo function that return the original payload to the caller.
You told the platform to register 10 instances of the echo function.

The number of concurrent instances allows you to scale your application vertically.

When you application is deployed in a network, DevOps can scale up your application in a horizontal manner.

## Singleton

To create a singleton, just set the number of instances to 1.

Singleton is an important design pattern. In event-driven design, this guarantees orderly processing of incoming events.

## application configuration parameters

The "root" configuration may be stored in the "application.properties" file and the parameters may be overridden using java environment variables.

## logging configuration

By default, logs are sent to the console. You may modify the logback.xml file if you have more sophisticated logging needs.

## Unit tests

The source tree contains a "test" branch. You can use the "Test" annotation and "Assert" APIs to do unit tests.

## End to end test drive

1. Compile and install the platform-core, rest-core and rest-spring projects as libraries locally using "mvn clean install" in the 3 projects.
2. Compile and package the "Event Node" cloud emulator with "mvn clean package" for local test. Run it in a command terminal using "java -jar target\event-node-(version).jar"
3. Compile and package this lambda-example project with "mvn clean package". Run it in a command terminal.
4. Compile and package this rest-example project with "mvn clean package". Run it in a command terminal.
5. Visit http://127.0.0.1:8083/api/hello/pojo/1 to try out end-to-end test from the rest-example app to the lambda-example app via the Event Node.
6. Visit http://127.0.0.1:8083/api/hello/world to try out local demo service.
