# Customized Spring Boot application with platform-core

This example project folder contains an example application that use the rest-spring-2 library and platform-core
together to generate a Spring Boot application.

The rest-spring-2 library is a customized Spring Boot 2 application server. It is provided as a convenient and unified
way to build Spring Boot application that is composable. 

This is optional. You can add the platform-core library to a regular Spring Boot application if you prefer.

## Spring Boot

This sample application requires Spring Boot version 2 as its application server.

## Application configuration

You can define application configuration parameters in either application.properties or application.yml.

If you have both application.properties and application.yml, the system will evaluate both configuration files.

When the same parameter is defined in both application.properties and application.yml, the parameter in
application.properties will be used.

While application.properties can also store text based key-values, application.yml supports text, numbers, boolean,
list and map values.
