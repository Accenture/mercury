# Event stream connector for ActiveMQ Artemis

Every event stream connector consists of a connector library and a presence monitor application.

Mercury has been tested with ActiveMQ Artemis 2.17.0

# Connector library

The connector library is available in the `activemq-connector` folder.

It is automatically built when you run `mvn clean install` at project root.

# Enabling activemq connector for your microservices application

The cloud-connector for activemq is provided as a reference. It is not included in pom.xml files of the 
rest-spring-example and lambda-example. To use the rest-spring-example and lambda-example as templates to write apps that
use the activemq cloud connector, please update your pom.xml with this dependency.

```
<dependency>
    <groupId>org.platformlambda</groupId>
    <artifactId>activemq-connector</artifactId>
    <version>{VERSION_NUMBER_HERE}</version>
</dependency>
```

In the application.properties config file in the application's resources folder:
```java
cloud.connector=activemq
```

# Default presence monitor configuration

The default configuration for presence monitor is available in the activemq-connector's resources folder.
The config file is called "presence.properties". To override this default, you can either create a new 
presence.properties in the resources folder of your project or put the config file under "/tmp/config" in 
the machine that runs the application.

```
url=ws://127.0.0.1:8080/ws/presence,ws://127.0.0.1:8081/ws/presence
```

# Default ActiveMQ artemis configuration

The default configuration for activemq is available in the activemq-connector's resources folder.
The config file is called "activemq.properties". To override this default, you can either create a 
new activemq.properties in the resources folder of your project or put the config file under "/tmp/config" 
in the machine that runs the application.

```
bootstrap.servers=tcp://127.0.0.1:61616
user.id=user
user.password=
admin.id=admin
admin.password=
```

For cloud deployment, there are 3 ways to override the activemq.properties configuration.

1. Your DevOps script can create the activemq.properties when building the application image or save the 
   activemq.properties in "/tmp/config" before starting the application
2. You can write a "BeforeApplication" module to construct and write the activemq.properties and deposit it
   in "/tmp/config". BeforeApplication modules run before an application starts.
3. You can write a cloud connector wrapper using the "CloudConnector" class and point the "original" back
   to "activemq". When your application select cloud.connector as your cloud connector wrapper, the wrapper
   will run before the activemq connector is executed.

# Presence monitor

The presence monitor application for activemq is available in the `activemq-presence` folder.

This application runs as a websocket server that all service container application instances will connect to.

When a service application instance fails, the presence monitor will detect it and inform all other application
instances to clear routing tables for the failed application instance.

You can build the presence monitor like this:

```
cd activemq-presence
mvn clean package
```

# ActiveMQ artemis server

ActiveMQ artemis server is available from https://activemq.apache.org/components/artemis/download/

# Create a new instance of Artemis server

You can create a new instance like this:

```
$ cd apache-artemis-2.17.0/bin
$ ./artemis create mercury
Creating ActiveMQ Artemis instance at: C:\sandbox\apache-artemis-2.17.0\bin\mercury
```

For development and testing, please select anonymous access.

If you do not select anonymous access, please update the application.properties of the activemq-presence,
lambda-example and rest-spring-example projects to update the user and admin ID and passwords.
