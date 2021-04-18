# Event stream connector for Tibco EMS

Every event stream connector consists of a connector library and a presence monitor application

# Connector library

The connector library is available in the `tibco-connector` folder.

Please build the library for the tibco-connector first.

```
cd tibco-connector
mvn clean install
```

This assumes you have built the platform-core, rest-core and rest-spring libraries from the project root using `mvn clean install`

# Enabling tibco connector for your microservices application

The cloud-connector for Tibco is provided as a reference. It is not included in pom.xml files of the rest-example and lambda-example. To use the rest-example and lambda-example as templates to write apps that use the Tibco cloud connector, please update your pom.xml with this dependency.

```
<dependency>
    <groupId>org.platformlambda</groupId>
    <artifactId>tibco-connector</artifactId>
    <version>{VERSION_NUMBER_HERE}</version>
</dependency>
```

In the application.properties config file in the application's resources folder:
```java
cloud.connector=tibco
```

# Default presence monitor configuration

The default configuration for presence monitor is available in the tibco-connector's resources folder. The config file is called "presence.properties". To override this default, you can either create a new presence.properties in the resources folder of your project or put the config file under "/tmp/config" in the machine that runs the application.

```
url=ws://127.0.0.1:8080/ws/presence,ws://127.0.0.1:8081/ws/presence
```

# Default Tibco configuration

The default configuration for tibco is available in the tibco-connector's resources folder. The config file is called "tibco.properties". To override this default, you can either create a new tibco.properties in the resources folder of your project or put the config file under "/tmp/config" in the machine that runs the application.

```
bootstrap.servers=tcp://127.0.0.1:7222
user.id=user
user.password=
admin.id=admin
admin.password=
```

# Presence monitor

The presence monitor application for tibco is available in the `tibco-presence` folder.

This application runs as a websocket server that all service container application instances will connect to.

When a service application instance fails, the presence monitor will detect it and inform all other application instances to clear routing tables for the failed application instance.

You can build the presence monitor like this:

```
cd tibco-presence
mvn clean package
```

# Tibco server

Tibco server is available from https://www.tibco.com/products/tibco-messaging/downloads

# Preparing Tibco library dependencies

The Tibco connector is not part of the regular build. It will not compile until you install the following tibco library files into your local ".m2" repository. Do not mix-n-match with other JMS 2.0 API library.

This setup procedure assumes TIBCO EMS 8.6.0 with JMS 2.0. Please modify the version numbers if you download a different TIBCO release.

```
jms-2.0.jar
tibjms.jar
tibjmsadmin.jar
```

mvn install:install-file -Dfile=jms-2.0.jar -DgroupId=com.tibco.ems -DartifactId=jms-api -Dversion=2.0 -Dpackaging=jar

mvn install:install-file -Dfile=tibjms.jar -DgroupId=com.tibco.ems -DartifactId=tibjms -Dversion=8.6.0 -Dpackaging=jar

mvn install:install-file -Dfile=tibjmsadmin.jar -DgroupId=com.tibco.ems -DartifactId=tibjms-admin -Dversion=8.6.0 -Dpackaging=jar

# Tibco server license

The following is a recap of the terms in https://www.tibco.com/products/tibco-messaging/downloads

"The Community Edition is a fully functional installation of the TIBCO Enteprise Message Service product, with the following limitations and exclusions:

Users may run up to 100 client instances in a production environment

No access to TIBCO Support but no-cost access to TIBCO Community is available as a support resource

Excludes fault tolerance of the server, unshared state failover, routing of messages between servers, central administration and JSON configuration files."

The Tibco EMS license is available from https://docs.tibco.com/pub/ems-ce/8.6.0/license/TIB_ems-ce_8.6.0_license.pdf
