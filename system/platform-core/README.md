# Mercury - the Post Office for microservices

## Core engine library

With Mercury, any software module can be expressed as a Java anonymous functions or a Java class that implements the LambdaFunction interface.

Mercury provides the following benefits:
1. Very high level of decoupling - You may write business logic as anonymous functions that run concurrently and independently.
You may also encapsulate platform components and databases with anonymous functions.
2. Event driven programming - Each function is addressable with a unique route name and they communicate using events.
You make request from one function to another by making a service call through some simple Mercury Post Office API.
3. One or more functions can be packaged together as a microservices executable, usually deployed as a Docker image or similar container technology.
4. Cloud platform neutral - each microservice executable may import a cloud connector library so that functions can talk to each other over a network event stream system such as Kafka.
5. You can write your own cloud connectors to adapt to different cloud environments.
6. Support different programming languages - language connector will be available to support other programming languages such as Python, Node.js, etc.

## Publish locally

You may compile the library from source and publish it locally in your development laptop.
To do this, clone this project and perform "mvn clean install".

