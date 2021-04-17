# Event stream connector for ActiveMQ Artemis

Every event stream connector consists of a connector library and a presence monitor application.

Mercury has been tested with ActiveMQ Artemis 2.17.0

# Connector library

The connector library is available in the `activemq-connector` folder.

Please build the library for the activemq-connector first.

```
cd activemq-connector
mvn clean install
```

This assumes you have built the platform-core, rest-core and rest-spring libraries from the project root using `mvn clean install`

# Enabling activemq connector for your microservices application

The cloud-connector for activemq is provided as a reference. It is not included in pom.xml files of the rest-example and lambda-example. To use the rest-example and lambda-example as templates to write apps that use the activemq cloud connector, please update your pom.xml with this dependency.

```
<dependency>
    <groupId>org.platformlambda</groupId>
    <artifactId>activemq-connector</artifactId>
    <version>{VERSION_NUMBER_HERE}</version>
</dependency>
```

In the application.properties config file in the application's resources folder:
```
cloud.connector=activemq
```

The default configuration for presence monitor is available in the activemq-connector's resources folder. The config file is called "presence.properties". To override this default, you can either create a new presence.properties in the resources folder of your project or put the config file under "/tmp/config" in the machine that runs the application.

```
url=ws://127.0.0.1:8080/ws/presence,ws://127.0.0.1:8081/ws/presence
```

# Presence monitor

The presence monitor application for activemq is available in the `activemq-presence` folder.

This application runs as a websocket server that all service container application instances will connect to.

When a service application instance fails, the presence monitor will detect it and inform all other application instances to clear routing tables for the failed application instance.

You can build the presence monitor like this:

```
cd activemq-presence
mvn clean package
```

# ActiveMQ Artemis server

ActiveMQ Artemis server is available from https://activemq.apache.org/components/artemis/download/

# Create a new instance of Artemis server

You can create a new instance like this:

```
$ cd apache-artemis-2.17.0/bin
$ ./artemis create mercury
Creating ActiveMQ Artemis instance at: C:\sandbox\apache-artemis-2.17.0\bin\mercury
```

For development and testing, please select anonymous access.

If you do not select anonymous access, please update the application.properties of the activemq-presence, lambda-example and rest-example projects to update the user and admin ID and passwords.

# cloud connector specific parameters

Please update the following parameters in the application.properties file if your application uses the activemq cloud connector.

```java
cloud.connector=activemq
# please change these parameter values accordingly
activemq.cluster=tcp://127.0.0.1:61616
activemq.user.id=user
activemq.user.password=tbd
activemq.admin.id=admin
activemq.admin.password=tbd
```
