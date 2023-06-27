# Composable application example

The lambda-example demonstrates REST automation (lightweight non-blocking HTTP server) that allows you
to create REST endpoints by configuration instead of code.

It illustrates building individual event-driven functions using Java (LambdaFunction and TypedLambdaFunction) and
Kotlin (suspend function that implements the KotlinLambdaFunction interface).

Unit test examples are also provided.

## Spring Boot

This sample application does not have Spring framework or Spring Boot dependencies so that it can be used
with Spring Boot or other frameworks.

The "spring-boot-parent" dependency in the pom.xml is a convenient way to fetch latest open sources libraries
that have been vetted by the Spring community.

## Application configuration

You can define application configuration parameters in either application.properties or application.yml.

If you have both application.properties and application.yml, the system will evaluate both configuration files.

When the same parameter is defined in both application.properties and application.yml, the parameter in
application.properties will be used.

While application.properties can also store text based key-values, application.yml supports text, numbers, boolean,
list and map values.
