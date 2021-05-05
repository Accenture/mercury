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
```java
cloud.connector=kafka
```

# Default presence monitor configuration

The default configuration for presence monitor is available in the kafka-connector's resources folder. The config file is called "presence.properties". To override this default, you can either create a new presence.properties in the resources folder of your project or put the config file under "/tmp/config" in the machine that runs the application.

```
url=ws://127.0.0.1:8080/ws/presence,ws://127.0.0.1:8081/ws/presence
```

# Default kafka configuration

The default configuration for kafka is available in the kafka-connector's resources folder. The config file is called "kafka.properties". To override this default, you can either create a new kafka.properties in the resources folder of your project or put the config file under "/tmp/config" in the machine that runs the application.

```
#security.protocol=SASL_SSL
#sasl.mechanism=PLAIN
#sasl.jaas.config=org.apache.kafka.common.security.plain.PlainLoginModule required username={CHANGE_THIS} password={CHANGE_THIS};
#request.timeout.ms=15000
#retry.backoff.ms=1000
#reconnect.backoff.max.ms=5000
#reconnect.backoff.ms=1000
#bootstrap.servers={host1:port1}:9092,{host2:port2}:9092
#
# Sample config for dev and testing using a standalone kafka server
#
bootstrap.servers=127.0.0.1:9092
```

For cloud deployment, there are 3 ways to override the kafka.properties configuration.

1. Your DevOps script can create the kafka.properties when building the application image or save the kafka.properties in "/tmp/config" before starting the application
2. You can write a "BeforeApplication" module to construct and write the kafka.properties and deposit it in "/tmp/config". BeforeApplication modules run before an application starts.
3. You can write a cloud connector wrapper using the "CloudConnector" class and point the "original" back to "kafka". When your application select cloud.connector as your cloud connector wrapper, the wrapper will run before the kafka connector is executed.

# Presence monitor

The presence monitor application for kafka is available in the `kafka-presence` folder.

This application runs as a websocket server that all service container application instances will report to.

When a service application instance fails, the presence monitor will detect it and inform all other application instances to clear routing entries for the failed application instance.

# Kafka-standalone

This is a convenient application to run Kafka as a standalone server. It will start zookeeper and kafka orderly. It uses the "/tmp" directory to store working files. This Kafka standalone server is designed to simplify software development and testing and should not be used for production purpose.

Note that when you restart the Kafka standalone server, all topics will be deleted. This is intentional because the kafka standalone server is designed for dev and testing only.
