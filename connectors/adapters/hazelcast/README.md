# Event stream connector for Hazelcast

Every event stream connector consists of a connector library and a presence monitor application

# Connector library

The connector library is available in the `hazelcast-connector` folder.

It is automatically built when you run `mvn clean install` at project root.

# Enabling Hazelcast connector for your microservices application

Include this dependency in the pom.xml of your application:
```xml
<dependency>
    <groupId>org.platformlambda</groupId>
    <artifactId>hazelcast-connector</artifactId>
    <version>{VERSION_NUMBER_HERE}</version>
</dependency>
```

In the application.properties config file in the application's resources folder:
```text
cloud.connector=hazelcast
```

# Default presence monitor configuration

The default configuration for presence monitor is available in the hazelcast-connector's resources folder.
The config file is called "presence.properties". To override this default, you can either create a new
presence.properties in the resources folder of your project or put the config file under "/tmp/config"
in the machine that runs the application.

```properties
url=ws://127.0.0.1:8080/ws/presence,ws://127.0.0.1:8081/ws/presence
```

# Default hazelcast configuration

The default configuration for hazelcast is available in the hazelcast-connector's resources folder.
The config file is called "hazelcast.properties". To override this default, you can either create a
new hazelcast.properties in the resources folder of your project or put the config file under "/tmp/config"
in the machine that runs the application.

```properties
bootstrap.servers=127.0.0.1:5701
```

For cloud deployment, there are 3 ways to override the hazelcast.properties configuration.

1. Your DevOps script can create the hazelcast.properties when building the application image or save the
   hazelcast.properties in "/tmp/config" before starting the application
2. You can write a "BeforeApplication" module to construct and write the hazelcast.properties and deposit it
   in "/tmp/config". BeforeApplication modules run before an application starts.
3. You can write a cloud connector wrapper using the "CloudConnector" class and point the "original" back to
   "hazelcast". When your application select cloud.connector as your cloud connector wrapper, the wrapper will
   run before the hazelcast connector is executed.

# Presence monitor

The presence monitor application for hazelcast is available in the `hazelcast-presence` folder.

This application runs as a websocket server that all service container application instances will connect to.

When a service application instance fails, the presence monitor will detect it and inform all other application
instances to clear routing tables for the failed application instance.

# hazelcast server

Hazelcast server is available from https://hazelcast.org/imdg/download/
