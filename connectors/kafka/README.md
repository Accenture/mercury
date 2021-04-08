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
```

The default configuration for presence monitor is available in the kafka-connector's resources folder. The config file is called "presence.properties". To override this default, you can either create a new presence.properties in the resources folder of your project or put the config file under "/tmp/config" in the machine that runs the application.

```
url=ws://127.0.0.1:8080/ws/presence,ws://127.0.0.1:8081/ws/presence
```

# Presence monitor

The presence monitor application for kafka is available in the `kafka-presence` folder.

This application runs as a websocket server that all service container application instances will report to.

When a service application instance fails, the presence monitor will detect it and inform all other application instances to clear routing entries for the failed application instance.

# Kafka-standalone

This is a convenient application to run Kafka as a standalone server. It will start zookeeper and kafka orderly. It uses the "/tmp" directory to store working files. This Kafka standalone server is designed to simplify software development and testing and should not be used for production purpose.

Note that when you restart the Kafka standalone server, all topics will be deleted. This is intentional because the kafka standalone server is designed for dev and testing only.
