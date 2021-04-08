# Event stream connector for Hazelcast

Every event stream connector consists of a connector library and a presence monitor application

# Connector library

The connector library is available in the `hazelcast-connector` folder.

Please build the library from source like this:
```
cd hazelcast-connector
mvn clean install
```
This assumes you have already built the platform-core, rest-core and rest-spring libraries.

# Enabling Hazelcast connector for your microservices application

Include this dependency in the pom.xml of your application:
```
<dependency>
    <groupId>org.platformlambda</groupId>
    <artifactId>hazelcast-connector</artifactId>
    <version>{VERSION_NUMBER_HERE}</version>
</dependency>
```

In the application.properties config file in the application's resources folder:
```
cloud.connector=hazelcast
```

The default configuration for presence monitor is available in the hazelcast-connector's resources folder. The config file is called "presence.properties". To override this default, you can either create a new presence.properties in the resources folder of your project or put the config file under "/tmp/config" in the machine that runs the application.

```
url=ws://127.0.0.1:8080/ws/presence,ws://127.0.0.1:8081/ws/presence
```

# Presence monitor

The presence monitor application for hazelcast is available in the `hazelcast-presence` folder.

This application runs as a websocket server that all service container application instances will connect to.

When a service application instance fails, the presence monitor will detect it and inform all other application instances to clear routing tables for the failed application instance.

# hazelcast server

Hazelcast server is available from https://hazelcast.org/imdg/download/
