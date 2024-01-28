# Service mesh

Service mesh is a dedicated infrastructure layer to facilitate inter-container communication using "sidecar" and
"control plane".

Service mesh systems require additional administrative containers (PODs) for "control plane" and "service discovery."

The additional infrastructure requirements vary among products.

# Using kafka as a service mesh

We will discuss using Kafka as a minimalist service mesh.

> Note: Service mesh is optional. You can use "event over HTTP" for inter-container
        communication if service mesh is not suitable.

Typically, a service mesh system uses a "side-car" to sit next to the application container in the same POD to provide
service discovery and network proxy services.

Kafka is a network event stream system. We have implemented libraries for a few "cloud connectors" to support
Kafka and Hazelcast as examples.

Instead of using a side-car proxy, the system maintains a distributed routing table in each application instance.
When a function requests the service of another function which is not in the same memory space, the "cloud.connector"
module will bridge the event to the peer application through a network event system like Kafka.

As shown in the following table, if "service.1" and "service.2" are in the same memory space of an application,
they will communicate using the in-memory event bus.

If they are in different applications and the applications are configured with Kafka, the two functions will 
communicate via the "cloud.connector" service. 

|    In-memory event bus     |              Network event stream               |
|:--------------------------:|:-----------------------------------------------:|
| "service.1" -> "service.2" | "service.1" -> "cloud.connector" -> "service.2" |

The system supports Kafka, Hazelcast out of the box. For example, to select kafka,
you can configure application.properties like this:

```properties
cloud.connector=kafka
```

The "cloud.connector" parameter can be set to "none", "kafka" or "hazelcast".
The default parameter of "cloud.connector" is "none". This means the application is not using
any network event system "connector", thus running independently.

Let's set up a minimalist service mesh with Kafka to see how it works.

## Set up a standalone Kafka server for development

You need a Kafka cluster as the network event stream system. For development and testing, you can build
and run a standalone Kafka server like this. Note that the `mvn clean package` command is optional because
the executable JAR should be available after the `mvn clean install` command in [Chapter-1](CHAPTER-1.md).

```shell
cd connectors/adapters/kafka/kafka-standalone
mvn clean package
java -jar target/kafka-standalone-3.0.8.jar
```

The standalone Kafka server will start at port 9092. You may adjust the "server.properties" in the standalone-kafka
project when necessary.

When the kafka server is started, it will create two temporary directories in the "/tmp" folder:

1. "/tmp/zookeeper"
2. "/tmp/kafka-logs"

> The kafka server is designed for development purpose only. The kafka and zookeeper data stores
  will be cleared when the server is restarted.

## Prepare the kafka-presence application

The "kafka-presence" is a "presence monitor" application. It is a minimalist "control plane" in service mesh
terminology.

What is a presence monitor? A presence monitor is the control plane that assigns unique "topic" for each
user application instance.

It monitors the "presence" of each application. If an application fails or stops, the presence monitor will 
advertise the event to the rest of the system so that each application container will update its corresponding
distributed routing table, thus bypassing the failed application and its services.

If an application has more than one container instance deployed, they will work together to share load evenly.

You will start the presence monitor like this:

```shell
cd connectors/adapters/kafka/kafka-presence
java -jar target/kafka-presence-3.0.8.jar
```

By default, the kafka-connector will run at port 8080. Partial start-up log is shown below:

```text
AppStarter:344 - Modules loaded in 2,370 ms
AppStarter:334 - Websocket server running on port-8080
ServiceLifeCycle:73 - service.monitor, partition 0 ready
HouseKeeper:72 - Registered monitor (me) 2023032896b12f9de149459f9c8b71ad8b6b49fa
```

The presence monitor will use the topic "service.monitor" to connect to the Kafka server and register itself
as a presence monitor.

Presence monitor is resilient. You can run more than one instance to back up each other.
If you are not using Docker or Kubernetes, you need to change the "server.port" parameter of the second instance
to 8081 so that the two application instances can run in the same laptop.

## Launch the rest-spring-2-example and lambda-example with kafka

Let's run the rest-spring-2-example (rest-spring-3-example) and lambda-example applications with
Kafka connector turned on.

For demo purpose, the rest-spring-2-example and lambda-example are pre-configured with "kafka-connector". 
If you do not need these libraries, please remove them from the pom.xml built script.

Since kafka-connector is pre-configured, we can start the two demo applications like this:

```text
cd examples/rest-spring-2-example
java -Dcloud.connector=kafka -Dmandatory.health.dependencies=cloud.connector.health 
     -jar target/rest-spring-2-example-3.0.8.jar
```

```text
cd examples/lambda-example
java -Dcloud.connector=kafka -Dmandatory.health.dependencies=cloud.connector.health 
     -jar target/lambda-example-3.0.8.jar
```

The above command uses the "-D" parameters to configure the "cloud.connector" and "mandatory.health.dependencies".

The parameter `mandatory.health.dependencies=cloud.connector.health` tells the system to turn on the health check
endpoint for the application.
               
For the rest-spring-2-example, the start-up log may look like this:

```text
AppStarter:344 - Modules loaded in 2,825 ms
PresenceConnector:155 - Connected pc.abb4a4de.in, 127.0.0.1:8080, 
                        /ws/presence/202303282583899cf43a49b98f0522492b9ca178
EventConsumer:160 - Subscribed multiplex.0001.0
ServiceLifeCycle:73 - multiplex.0001, partition 0 ready
```

This means that the rest-spring-2-example has successfully connected to the presence monitor at port 8080.
It has subscribed to the topic "multiplex.0001" partition 0.

For the lambda-example, the log may look like this:

```text
AppStarter:344 - Modules loaded in 2,742 m
PresenceConnector:155 - Connected pc.991a2be0.in, 127.0.0.1:8080, 
                        /ws/presence/2023032808d82ebe2c0d4e5aa9ca96b3813bdd25
EventConsumer:160 - Subscribed multiplex.0001.1
ServiceLifeCycle:73 - multiplex.0001, partition 1 ready
ServiceRegistry:242 - Peer 202303282583899cf43a49b98f0522492b9ca178 joins (rest-spring-2-example 3.0.0)
ServiceRegistry:383 - hello.world (rest-spring-2-example, WEB.202303282583899cf43a49b98f0522492b9ca178) registered
```

You notice that the lambda-example has discovered the rest-spring-2-example through Kafka and added the 
"hello.world" to the distributed routing table.

At this point, the rest-spring-2-example will find the lambda-example application as well:

```text
ServiceRegistry:242 - Peer 2023032808d82ebe2c0d4e5aa9ca96b3813bdd25 joins (lambda-example 3.0.0)
ServiceRegistry:383 - hello.world (lambda-example, 
                                   APP.2023032808d82ebe2c0d4e5aa9ca96b3813bdd25) registered
ServiceRegistry:383 - hello.pojo (lambda-example, 
                                   APP.2023032808d82ebe2c0d4e5aa9ca96b3813bdd25) registered
```

This is real-time service discovery coordinated by the "kafka-presence" monitor application.

Now you have created a minimalist event-driven service mesh.

## Send an event request from rest-spring-2-example to lambda-example

In [Chapter-7](CHAPTER-7.md), you have sent a request from the rest-spring-2-example to the lambda-example using 
"Event over HTTP" without a service mesh.

In this section, you can make the same request using service mesh.

Please point your browser to http://127.0.0.1:8083/api/pojo/mesh/1
You will see the following response in your browser.

```json
{
  "id": 1,
  "name": "Simple PoJo class",
  "address": "100 World Blvd, Planet Earth",
  "date": "2023-03-28T17:53:41.696Z",
  "instance": 1,
  "seq": 1,
  "origin": "2023032808d82ebe2c0d4e5aa9ca96b3813bdd25"
}
```

## Presence monitor info endpoint

You can check the service mesh status from the presence monitor's "/info" endpoint. 

You can visit http://127.0.0.1:8080/info and it will show something like this:

```json
{
  "app": {
    "name": "kafka-presence",
    "description": "Presence Monitor",
    "version": "3.0.0"
  },
  "personality": "RESOURCES",
  "additional_info": {
    "total": {
      "topics": 2,
      "virtual_topics": 2,
      "connections": 2
    },
    "topics": [
      "multiplex.0001 (32)",
      "service.monitor (11)"
    ],
    "virtual_topics": [
      "multiplex.0001-000 -> 202303282583899cf43a49b98f0522492b9ca178, rest-spring-2-example v3.0.0",
      "multiplex.0001-001 -> 2023032808d82ebe2c0d4e5aa9ca96b3813bdd25, lambda-example v3.0.0"
    ],
    "connections": [
      {
        "elapsed": "25 minutes 12 seconds",
        "created": "2023-03-28T17:43:13Z",
        "origin": "2023032808d82ebe2c0d4e5aa9ca96b3813bdd25",
        "name": "lambda-example",
        "topic": "multiplex.0001-001",
        "monitor": "2023032896b12f9de149459f9c8b71ad8b6b49fa",
        "type": "APP",
        "updated": "2023-03-28T18:08:25Z",
        "version": "3.0.0",
        "seq": 65,
        "group": 1
      },
      {
        "elapsed": "29 minutes 42 seconds",
        "created": "2023-03-28T17:38:47Z",
        "origin": "202303282583899cf43a49b98f0522492b9ca178",
        "name": "rest-spring-2-example",
        "topic": "multiplex.0001-000",
        "monitor": "2023032896b12f9de149459f9c8b71ad8b6b49fa",
        "type": "WEB",
        "updated": "2023-03-28T18:08:29Z",
        "version": "3.0.0",
        "seq": 75,
        "group": 1
      }
    ],
    "monitors": [
      "2023032896b12f9de149459f9c8b71ad8b6b49fa - 2023-03-28T18:08:46Z"
    ]
  },
  "vm": {
    "java_vm_version": "18.0.2.1+1",
    "java_runtime_version": "18.0.2.1+1",
    "java_version": "18.0.2.1"
  },
  "origin": "2023032896b12f9de149459f9c8b71ad8b6b49fa",
  "time": {
    "current": "2023-03-28T18:08:47.613Z",
    "start": "2023-03-28T17:31:23.611Z"
  }
}
```

In this example, it shows that there are two user applications (rest-spring-2-example and lambda-example) connected.

## Presence monitor health endpoint

The presence monitor has a "/health" endpoint.

You can visit http://127.0.0.1:8080/health and it will show something like this:

```json
{
  "upstream": [
    {
      "route": "cloud.connector.health",
      "status_code": 200,
      "service": "kafka",
      "topics": "on-demand",
      "href": "127.0.0.1:9092",
      "message": "Loopback test took 3 ms; System contains 2 topics",
      "required": true
    }
  ],
  "origin": "2023032896b12f9de149459f9c8b71ad8b6b49fa",
  "name": "kafka-presence",
  "status": "UP"
}
```

## User application health endpoint

Similarly, you can check the health status of the rest-spring-2-example application with http://127.0.0.1:8083/health

```json
{
  "upstream": [
    {
      "route": "cloud.connector.health",
      "status_code": 200,
      "service": "kafka",
      "topics": "on-demand",
      "href": "127.0.0.1:9092",
      "message": "Loopback test took 4 ms",
      "required": true
    }
  ],
  "origin": "202303282583899cf43a49b98f0522492b9ca178",
  "name": "rest-spring-example",
  "status": "UP"
}
```

It looks similar to the health status of the presence monitor. However, only the presence monitor shows the total
number of topics because it handles topic issuance to each user application instance.

## Actuator endpoints

Additional actuator endpoints includes:

1. library endpoint ("/info/lib") - you can check the packaged libraries for each application
2. distributed routing table ("/info/routes") - this will display the distributed routing table for public functions
3. environment ("/env") - it shows all functions (public and private) with number of workers.
4. livenessproble ("/livenessprobe") - this should display "OK" to indicate the application is running

## Stop an application

You can press "control-C" to stop an application. Let's stop the lambda-example application.

Once you stopped lamdba-example from the command line, the rest-spring-2-example will detect it:

```text
ServiceRegistry:278 - Peer 2023032808d82ebe2c0d4e5aa9ca96b3813bdd25 left (lambda-example 3.0.0)
ServiceRegistry:401 - hello.world 2023032808d82ebe2c0d4e5aa9ca96b3813bdd25 unregistered
ServiceRegistry:401 - hello.pojo 2023032808d82ebe2c0d4e5aa9ca96b3813bdd25 unregistered
```

The rest-spring-2-example will update its distributed routing table automatically.

You will also find log messages in the kafka-presence application like this:

```text
MonitorService:120 - Member 2023032808d82ebe2c0d4e5aa9ca96b3813bdd25 left
TopicController:250 - multiplex.0001-001 released by 2023032808d82ebe2c0d4e5aa9ca96b3813bdd25,
                                                     lambda-example, 3.0.0
```

When an application instance stops, the presence monitor will detect the event, remove it from the registry and
release the topic associated with the disconnected application instance.

The presence monitor is using the "presence" feature in websocket, thus we call it "presence" monitor.

<br/>

|            Chapter-7            |                   Home                    |          CHAPTER-9           |
|:-------------------------------:|:-----------------------------------------:|:----------------------------:|
| [Event over HTTP](CHAPTER-6.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [API overview](CHAPTER-9.md) |
