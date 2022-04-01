# application.properties

| Key                                         | Value (example)                          | Required   |
|:--------------------------------------------|:-----------------------------------------|:-----------|
| application.name or spring.application.name | Application name                         | Yes        |
| info.app.version                            | major.minor.build (e.g. 1.0.0)           | Yes        |
| info.app.description                        | Something about application              | Yes        |
| web.component.scan                          | your own package path or parent path     | Yes        |
| server.port                                 | e.g. 8083                                | Yes*       |
| rest.server.port                            | e.g. 8083                                | Required*2 |
| spring.mvc.static-path-pattern              | /**                                      | Yes*       |
| spring.resources.static-locations           | classpath:/public/                       | Yes*       |
| jax.rs.application.path                     | /api                                     | Optional*1 |
| show.env.variables                          | comma separated list of variable names   | Optional*1 |
| show.application.properties                 | comma separated list of property names   | Optional*1 |
| cloud.connector                             | kafka, hazelcast, none, etc.             | Optional   |
| cloud.services                              | e.g. some.interesting.service            | Optional   |
| snake.case.serialization                    | true (recommended)                       | Optional   |
| env.variables                               | e.g. MY_ENV:my.env                       | Optional   |
| safe.data.models                            | packages pointing to your PoJo classes   | Optional   |
| protect.info.endpoints                      | true or false (default is false)         | Optional*1 |
| trace.http.header                           | comma separated list traceId labels      | *2         |
| trace.log.header                            | default value is X-Trace-Id              | Optional   |
| index.redirection                           | comma separated list of URI paths        | Optional*1 |
| index.page                                  | default value is index.html              | Optional*1 |
| application.feature.route.substitution      | default value is false                   | Optional   |
| route.substitution.file                     | comma separated file(s) or classpath(s)  | Optional   |
| application.feature.topic.substitution      | default value is false                   | Optional   |
| topic.substitution.file                     | comma separated file(s) or classpath(s)  | Optional   |
| cloud.client.properties                     | e.g. classpath:/kafka.properties         | connectors |
| kafka.replication.factor                    | 3                                        | Kafka      |
| default.app.group.id                        | kafka groupId for the app instance       | Optional   |
| default.monitor.group.id                    | kafka groupId for the presence-monitor   | Optional   |
| monitor.topic                               | kafka topic for the presence-monitor     | Optional   |
| app.topic.prefix                            | multiplex (default value, DO NOT change) | Optional   |
| app.partitions.per.topic                    | Max Kafka partitions per topic           | Optional   |
| max.virtual.topics                          | Max virtual topics = partitions * topics | Optional   |
| max.closed.user.groups                      | Number of closed user groups             | Optional   |
| closed.user.group                           | Closed user group (default 1)            | Optional   |
| transient.data.store                        | Default value: /tmp/reactive             | Optional   |
| running.in.cloud                            | Default value: false                     | Optional   |
| multicast.yaml                              | This is used to define config file path  | Optional   |
| journal.yaml                                | This is used to define config file path  | Optional   |
| distributed.trace.aggregation               | Default value: true                      | Optional   |
| deferred.commit.log                         | Default value: false                     | Optional*3 |

`*1` - when using the "rest-spring" library
`*2` - applies to the REST automation application only
`*3` - optional for unit test purpose only. DO NOT set this parameter in "main" branch for production code.

# transient data store

The system handles back-pressure automatically by overflowing memory to a transcient data store. 
As a cloud native best practice, the folder must be under "/tmp". The default is "/tmp/reactive". 
The "running.in.cloud" must be set to false when your apps are running in IDE or in your laptop. 
When running in kubernetes, it can be set to true.

# safe.data.models

PoJo may execute Java code. As a result, it is possible to inject malicious code that does harm when 
deserializing a PoJo. This security risk applies to any JSON serialization engine.

For added security and peace of mind, you may want to white list your PoJo package paths.
When the "safe.data.models" property is configured, the underlying serializers for JAX-RS, Spring RestController, 
Servlets will respect this setting and enforce PoJo white listing.

Usually you do not need to use the serializer in your code because it is much better to deal with PoJo in your IDE.
However, if there is a genuine need to do low level coding, you may use the pre-configured serializer so that the 
serialization behavior is consistent.

You can get an instance of the serializer with `SimpleMapper.getInstance().getWhiteListMapper()`.

# trace.http.header

Identify the HTTP header for traceId. When configured with more than one label, the system will retrieve traceID 
from the corresponding HTTP header and propagate it through the transaction that may be served by multiple services.

If traceId is presented in a HTTP request, the system will use the same label to set HTTP response traceId header.

e.g. 
X-Trace-Id: a9a4e1ec-1663-4c52-b4c3-7b34b3e33697
or
X-Correlation-Id: a9a4e1ec-1663-4c52-b4c3-7b34b3e33697

# trace.log.header

If tracing is enabled for a transaction, this will insert the trace-ID into the logger's ThreadContext using the 
trace.log.header.

Note that trace.log.header is case sensitive and you must put the corresponding value in log4j2.xml.
The default value is "X-Trace-Id" if this parameter is not provided in application.properties.
e.g.

```xml
<PatternLayout pattern="%d{yyyy-MM-dd HH:mm:ss.SSS} %-5level %logger:%line [%X{X-Trace-Id}] - %msg%n" />
```

# Kafka specific configuration

If you use the kafka-connector (cloud connector) and kafka-presence (presence monitor), you may want to 
externalize kafka.properties. It is recommended to set `kafka.client.properties=file:/tmp/config/kafka.properties`

Note that "classpath" refers to embedded config file in the "resources" folder in your source code and "file" 
refers to an external config file.

You want also use the embedded config file as a backup like this:

```
# this is the configuration used by the kafka.presence monitor
kafka.client.properties=file:/tmp/config/kafka.properties,classpath:/kafka.properties
```

`kafka.replication.factor` is usually determined by DevOps. Contact your administrator for the correct value. 
If the available number of kafka brokers are more than the replication factor, it will use the given replication factor.
Otherwise, the system will fall back to a smaller value and this optimization may not work in production.
Therefore, please discuss with your DevOps administrator for the optimal replication factor value.

```
Number of replicated copies = replication factor - 1
```

# presence monitor

You will deploy a "presence monitor" to assign topics to any connected user application instances. 

Your user application instances will connect to the presence monitor using websocket. 
When an application instance fails, the presence monitor can detect it immediately so that its peer application 
instances can update the routing table.

# distributed trace logging, aggregation and transaction journaling

To enable distributed trace logging, please set this in log4j2.xml. For cloud native apps, you should redirect 
logging from standard out to a centralized logging system such as Elastic Search or Splunk.

```
<logger name="org.platformlambda.core.services.DistributedTrace" level="INFO" />
```

Distributed trace aggregation will be available when you deploy your custom aggregation service with the route name
`distributed.trace.processor`. 

If you want to disable aggregation for an application, set "distributed.trace.aggregation" to false
in application.properties.

Journaling service will be available with you deploy your custom aggregation service with the route name 
`distributed.trace.processor` and configure the "journal.yaml" parameter in application.properties pointing 
to a YAML file containing the following:

```
journal:
 - "my.service.1"
 - ...
# my.service.1 is an example. You should enter the correct route names for your services to be journaled.
```

The system will send transaction input/output payloads to the journaling service when trace is turned on and 
journal.yaml contains the service route name.

For security and privacy, transaction journal should be saved to a database with access control because the 
transaction payload may contain sensitive information.

# Spring Boot

The foundation code uses Spring Boot in the "rest-spring" library. For loose coupling, we use the `@MainApplication` 
as a replacement for the `SpringApplication`. Please refer to the MainApp class in the "rest-example" project.
This allows us to use any REST application server when technology evolves.

If your code uses Spring Boot or Spring Framework directly, you can set the corresponding key-values in the 
application.properties file in the resources folder. e.g. changing the "auto-configuration" parameters.

---

| Appendix-II                              | Home                                     |
| :---------------------------------------:|:----------------------------------------:|
| [Reserved route names](APPENDIX-II.md)   | [Table of Contents](TABLE-OF-CONTENTS.md)|
