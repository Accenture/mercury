# Event stream connector for Kafka

Every event stream connector consists of a connector library and a presence monitor application

# Connector library

The connector library is available in the `kafka-connector` folder.

Please build the library from source like this:
```
cd kafka-connector
mvn clean install
```
This assumes you have already built the platform-core, rest-core and rest-spring libraries.

# Enabling Kafka connector for your microservices application

Include this dependency in the pom.xml of your application:
```
<dependency>
    <groupId>org.platformlambda</groupId>
    <artifactId>kafka-connector</artifactId>
    <version>{VERSION_NUMBER_HERE}</version>
</dependency>
```

In the application.properties config file in the application's resources folder:
```
cloud.connector=kafka
cloud.services=kafka.reporter
presence.monitor=ws://{HOST:PORT}:8080/ws/presence
```
The presence.monitor may contain more than one URL with comma separator. For production, you should deploy 2 to 3 presence monitor application instances for resilience. When more than one instance of presence monitor is deployed, they will detect each other and coordinate among themselves. There is no primary/secondary relationship. All presence monitors run as equal peers.

# Presence monitor

The presence monitor application for hazelcast is available in the `kafka-presence` folder.

This application runs as a websocket server that all service container application instances will connect to.

When a service application instance fails, the presence monitor will detect it and inform all other application instances to clear routing tables for the failed application instance.

# Kafka-standalone

This is a convenient application to run Kafka as a standalone server. It will start zookeeper and kafka orderly. It uses the "/tmp" directory to store working files. This Kafka standalone server is designed to simplify software development and testing and should not be used for production purpose.

Note that when you restart the Kafka standalone server, all topics will be deleted. This is intentional because the kafka standalone server is designed for dev and testing only.
