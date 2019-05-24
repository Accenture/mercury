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
cloud.services=hazelcast.reporter
presence.monitor=ws://{HOST:PORT}:8080/ws/presence
```
The presence.monitor may contain more than one URL with comma separator. For production, you should deploy 2 to 3 presence monitor application instances for resilience. When more than one instance of presence monitor is deployed, they will detect each other and coordinate among themselves. There is no master/slave relationship. All presence monitors run as equal peers.

# Presence monitor

The presence monitor application for hazelcast is available in the `hazelcast-presence` folder.

This application runs as a websocket server that all service container application instances will connect to.

When a service application instance fails, the presence monitor will detect it and inform all other application instances to clear routing tables for the failed application instance.
