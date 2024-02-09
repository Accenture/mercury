# Purpose

Real-world performance often differs from best case scenarios in a laboratory setting.

This benchmark system offers a scientific way to measure near real-world performance for both
the in-memory event system and your selected network event stream system such as Kafka.

We are also using this tool for our regression tests.

# Benchmark client

This benchmark client is generalized for the following use cases:

## In-memory event system

When the "cloud.connector" is set to "none", the benchmark system will run in standalone mode using
the in-memory event system.

> Figure 1 - Benchmark system in standalone mode

![benchmark-standalone.png](diagrams/benchmark-standalone.png)

## Network event system

When the "cloud.connector" is set to a cloud provider service such as "kafka", the benchmark system
will run in network mode connecting a benchmark client with a benchmark server.

> Figure 2 - Benchmark system in client-server mode

![benchmark-client-server.png](diagrams/benchmark-client-server.png)

# How it works?

## Event system abstraction

You may notice that Figures 1 and 2 have similar setup with the exception that the client-server mode
uses a Kafka event system.

Under the hood, the event system is encapsulated so that the user functions do not need to deal with
physical infrastructure. You can simply change the underlying infrastructure using the "cloud.connector"
parameter in application.properties. You therefore can change infrastructure with zero to minmal refactoring
of your user applications.

## Event flow

Since event flow is asynchronous, the user will connect to the benchmark client application using websocket.
A simple websocket client UI application is encoded in the "benchmark.html" page that can be selected from
the home page "index.html".

The user interface has a button to start/stop a websocket connection to the benchmark client application and
an input box to submit your command.

The command will be delivered to the "websocket query handler" in the benchmark client application.
Upon connection, it will register the websocket connection with a "Local Pub/Sub" topic called "benchmark.users"
that will push the benchmark result to the web browser automatically.

The Local Pub/Sub system allows more than one test user to see the real-time benchmark progress and result.

The websocket query handler will forward the user request to the "benchmark service" that will interpret
the command and construct a set of parallel events to the server.

In standalone mode, the client application will also serve as the "server" providing the two event consumers
"network.echo" and "network.one.way".

The "echo" service will return the original payload back to the benchmark service.

The "one-way" service will return a timestamp that indicates when an event is successfully received by
the "event consumer".

The timestamp in the one-way service is important because it allows the benchmark service to calculate
the "one-trip" latency in milliseconds. Since event system is heavily buffered by a commit log, this benchmark
method is more accurate and realistic than just measuring time to publish a set of events.

The echoed event is used by the benchmark service to compute the "round-trip" latency.

For the client-server mode, the principle of operation is exactly the same, except that the events
are delivered by a network event stream system such as Kafka in addition to using an in-memory event
system.

> Note: for client-server mode deployment, it is assumed that the client and server applications are running
  in the same subnet with real-time clock synchronization. We usually take this for granted. Please check
  with your DevSecOps admin if additional clock synchronization setup is required.

# Running this benchmark application

Please "cd" to this subproject and enter "mvn clean install" or "mvn clean verify".

Note that the "install" or "verify" phase of the maven build life cycle ensures that a clean up script
can be executed after the "package" phase. This generates an executable JAR for the subproject.

Before you run the benchmark application, build the application like this first:

```shell
# build Mercury libraries from source if you have not done so
cd sandbox/mercury
# build the client and server apps
mvn clean install
cd benchmark/benchmark-server
mvn clean install
cd ..
cd benchmark/benchmark-client
mvn clean install
```

The benchmark client is pre-configured to run in standalone mode. You can start it like this:

```shell
cd sandbox/mercury/benchmark-client
java -jar target/benchmark-client-3.0.9.jar
```

To run the benchmark in client-server mode, please start the server and client apps like this:

```shell
cd sandbox/mercury/benchmark-server
java -jar target/benchmark-server-3.0.9.jar
cd sandbox/mercury/connectors/adapter/kafka/kafka-presence
java -jar target/kafka-presence-3.0.9.jar
cd sandbox/mercury/benchmark-client
java -Dcloud.connector=kafka -jar target/benchmark-client-3.0.9.jar
```

Note that you can either update the application.properties in the client application or
use the Java run-time parameter "-Dcloud.connector=kafka" to tell the client to use Kafka
as the network event stream.

Then point your browser to http://127.0.0.1:8083 and select "benchmark" to see the benchmark
control screen.

Click "start service" and then enter "help". It will show the options.

The syntax for running a benchmark test is:

```shell
async | echo | http {count} payload {size}
```

"async" means it will send events from the producer to the consumer in a single direction.
The consumer will save a timestamp so that the latency can be calculated.

"echo" means it will send events from the producer to the consumer where the consumer will echo
back the input to complete a round trip measurement.

Size of the event payload is measured in bytes. The payload does not count the event metadata. 
However, the metadata overheads are minimal and they do not impact the accuracy of the benchmark.

> Note that the send and receive paths are served by two separate topic/partition
  when running in network mode.

"http" is a special case for "Event Over HTTP". You would need to provide the target server URL
using the "set target {eventOverHttpTarget}" command first.

e.g. http://127.0.0.1:8083 would tell the system to make "event over HTTP" calls to itself through
the network. http://127.0.0.1:8085 would tell the system to make "event over HTTP" calls to the
benchmark server in localhost. If you deploy the benchmark client and server in your environment,
please replace the localhost address with your target server address.

This would evaluate the inter-container communication efficiency between application containers
using HTTP.

# Behind the curtain

The underlying in-memory event system is Eclipse Vertx and Kafka network event stream system.

For reactive flow-control, mercury implements a manager/workers pattern for each service.

The number of workers per service is configured to be 200. The benchmark functions run as "coroutines" to
reduce stress on the Java VM. When the number of concurrent requests is more than 200, the system will 
queue the outstanding requests orderly.

Each application instance is allocated a dedicated kafka topic as the communication conduit. 
Events are serialized before sending through the network. Any payload that is larger than 64 KB will be
automatically broken into segments of 64 KB chunks. For events passing through the in-memory event
system, payload segmentation is not needed.

The performance benchmark report will reflect this queueing mechanism and orderly execution, thus
offering a more realistic performance projection in real-world use cases.

# Payload size and number of events per test

As anticipated, the most efficient payload size over the network is 6 to 10 KB.

However, you can adjust the size of the payload for the event from 10 to 2,000,000 bytes to mimic your
real-world use cases.

You can select the number of parallel events from 1 to 5,000 to simulate your real-world use cases.

# Deploying to performance test environment

The performance test environment may be on-prem or in the cloud.

If you are using Kubernetes, you should dockerize the benchmark applications using your organization's
approved docker image. You can use Java (JRE or JDK) version 1.8 to 21 as the base image. The kernel
is likely to be a bare minimal linux but your environment may use different operating system that you
prefer.

You may place a pre-configured "kafka.properties" and "presence.properties" files in the
"/tmp/config" folder in the dockerized image using a deployment pipeline.

For higher security, you may want to generate the above configuration files and programmatically
place them in the "/tmp/config" folder at run-time. Usually you may get the configuration and Kafka
credentials from the environment variables or a secret manager in the cloud.

Your configuration approach may vary. To accommodate different way of configuration in the pipeline
or the cloud, please review the placeholder class "SetupEnv.java" in the "setup" folder. You should
write a few lines of code to retrieve the configuration and secrets and generate the two config files
above.

## Sample kafka.properties

```shell
#
# Example for SASL authentication
# Please consult your DevOps colleagues for exact configuration values
#
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

## Sample presence.properties

```shell
#
# URL - a list of websocket URLs for connecting to the presence monitor(s)
#
url=ws://127.0.0.1:8080/ws/presence,ws://127.0.0.1:8081/ws/presence,ws://127.0.0.1:8082/ws/presence
```

The multiple URLs are used for testing more than one presence monitor in a local dev machine.

When using Kubernetes, the host and port are defined in the pipeline.

For example:
```shell
url=ws://presence-monitor-live:8080/ws/presence
```

In the above example, the host "presence-monitor-live" may have more than one application instance deployed.

# Warming up the benchmark system

Since Java takes a bit more time to load new "objects" into memory, please ignore the first round of test
when the application is first deployed because it is likely to be slower than normal. 

Similarly, you should also ignore the first round of test after a long period of idle.

The in-memory event system and Kafka are designed to handle stress very well. You may see performance
gain with more events. We recommend to start the test with 1,000 events per test. It is normal to see
better performance results when the system is under heavy load and this may not be intuitive.

You can also test smaller number of events to simulate the use case with very light traffic.

# Advanced configuration

The benchmark client and server application uses a separate topic and partition pair for send and receive.

Modern event stream system like Kafka uses commit log for each partition and thus achieving almost linear
scaling of performance with more partitions for a topic.

If you want to validate this claim, you can run more than one set of client and server pair by using
a concept called "closed user group".

```shell
# when using a cloud connector, you can run each pair of benchmark client and server
# in different "closed user group" so that you can evaluate performance when
# scaling up and down.
closed.user.group=1
max.closed.user.groups=10
```

The default "closed user group" is "1" for regular application.

To run another set of "closed user group", you may change the number accordingly, up to the maximum
closed user group limit.

Note that "closed user group" parameter is only required for the benchmark client and server applications
only. Do not set it in the "presence-monitor" application. The presence monitor will perform the
"closed user group" allocation accordingly.
