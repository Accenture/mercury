# rest-spring library

This is a pre-configured Spring Boot 3 container with platform-core library for writing composable application.

# Serializers and exception handlers

JSON and XML serializers and exception handlers are defined for Spring RestController.

# Spring WebFlux

The WebFlux library is included in the pom.xml

Corresponding unit tests are shown in the GreetingTest class.

# Minimal requirement of Java 17

Since Spring Boot 3 has a minimal requirement of Java 17, this library is not included in the root project
build script (pom.xml)

To build this library module, please update the root project pom.xml file if you have Java 17 or higher.

Alternatively, you can also build this library like this:

```text
mvn clean install
```
