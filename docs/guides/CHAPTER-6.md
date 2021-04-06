# application.properties

| Key                                         | Value (example)                           | Required  |
| :-------------------------------------------|:------------------------------------------|:----------|
| application.name or spring.application.name | Application name                          | Yes       |
| info.app.version                            | major.minor.build (e.g. 1.0.0)            | Yes       |
| info.app.description                        | Something about application               | Yes       |
| web.component.scan                          | your own package path or parent path      | Yes       |
| server.port                                 | e.g. 8083                                 | Yes*      |
| spring.mvc.static-path-pattern              | /**                                       | Yes*      |
| spring.resources.static-locations           | classpath:/public/                        | Yes*      |
| jax.rs.application.path                     | /api                                      | Optional*1|
| show.env.variables                          | comma separated list of variable names    | Optional*1|
| show.application.properties                 | comma separated list of property names    | Optional*1|
| cloud.connector                             | kafka, hazelcast, none, etc.              | Optional  |
| cloud.services                              | e.g. some.interesting.service             | Optional  |
| hazelcast.cluster                           | e.g. 127.0.0.1:5701,127.0.0.1:5702        | Optional  |
| snake.case.serialization                    | true (recommended)                        | Optional  |
| env.variables                               | e.g. MY_ENV:my.env                        | Optional  |
| safe.data.models                            | packages pointing to your PoJo classes    | Optional  |
| protected.info.endpoints                    | e.g. /route, /info, /env                  | Optional*1|
| info.api.key                                | some secret key (recommended to use UUID) | Optional*1|
| trace.http.header                           | comma separated list traceId labels       | *2        |
| trace.log.header                            | default value is X-Trace-Id               | Optional  |
| index.redirection                           | comma separated list of URI paths         | Optional*1|
| index.page                                  | default value is index.html               | Optional*1|
| application.feature.route.substitution      | default value is false                    | Optional  |
| route.substitution.file                     | comma separated file(s) or classpath(s)   | Optional  |
| kafka.client.properties                     | classpath:/kafka.properties               | Kafka     |
| kafka.replication.factor                    | 3                                         | Kafka     |
| app.shutdown.key                            | secret key to shutdown app instance       | Optional  |
| default.app.group.id                        | kafka groupId for the app instance        | Optional  |
| default.monitor.group.id                    | kafka groupId for the presence-monitor    | Optional  |
| monitor.topic                               | kafka topic for the presence-monitor      | Optional  |
| app.topic.prefix                            | multiplex (default value, DO NOT change)  | Optional  |
| app.partitions.per.topic                    | Max Kafka partitions per topic            | Optional  |
| max.virtual.topics                          | Max virtual topics = partitions * topics  | Optional  |
| max.closed.user.groups                      | Number of closed user groups              | Optional  |
| closed.user.group                           | Closed user group (default 1) for the app | Optional  |

`*1` - when using the "rest-spring" library
`*2` - applies to the REST automation helper application only

# safe.data.models

PoJo may execute Java code. As a result, it is possible to inject malicious code that does harm when deserializing a PoJo.
This security risk applies to any JSON serialization engine.

For added security and peace of mind, you may want to white list your PoJo package paths.
When the "safe.data.models" property is configured, the underlying serializers for JAX-RS, Spring RestController, Servlets will respect this setting and enforce PoJo white listing.

Usually you do not need to use the serializer in your code because it is much better to deal with PoJo in your IDE.
However, if there is a genuine need to do low level coding, you may use the pre-configured serializer so that the serialization behavior is consistent.

You can get an instance of the serializer with `SimpleMapper.getInstance().getWhiteListMapper()`.

# info.api.key

When "protected.info.endpoints" are configured, you must provide this secret in the "X-Info-Key" header when accessing the protected endpoints. 

# trace.http.header

Identify the HTTP header for traceId. When configured with more than one label, the system will retrieve traceID from the
corresponding HTTP header and propagate it through the transaction that may be served by multiple services.

If traceId is presented in a HTTP request, the system will use the same label to set HTTP response traceId header.

e.g. 
X-Trace-Id: a9a4e1ec-1663-4c52-b4c3-7b34b3e33697
or
X-Correlation-Id: a9a4e1ec-1663-4c52-b4c3-7b34b3e33697

# trace.log.header

If tracing is enabled for a transaction, this will insert the trace-ID into the logger's ThreadContext using the trace.log.header.

Note that trace.log.header is case sensitive and you must put the corresponding value in log4j2.xml.
The default value is "X-Trace-Id" if this parameter is not provided in application.properties.
e.g.

```xml
<PatternLayout pattern="%d{yyyy-MM-dd HH:mm:ss.SSS} %-5level %logger:%line [%X{X-Trace-Id}] - %msg%n" />
```

# app.shutdown.key

If this parameter is given, the shutdown endpoint will be activated.
```
POST /shutdown

content-type: "application/x-www-form-urlencoded"
body: key=the_shutdown_key&origin=originId
```

# route substitution

This is usually used for blue/green environment tests. In some simple cases, you may use this for versioning. 

Example parameters in application.properties
```yaml
application.feature.route.substitution=true
# the system will load the first available file if more than one location is given
route.substitution.file=file:/tmp/config/route-substitution.yaml , classpath:/route-substitution.yaml
```

Example route-substitution.yaml file
```yaml
route:
  substitution:
    - "hello.test -> hello.world"
```

# Kafka specific configuration

If you use the kafka-connector (cloud connector) and kafka-presence (presence monitor), you may want to externalize kafka.properties.
It is recommended to set `kafka.client.properties=file:/tmp/config/kafka.properties`

Note that "classpath" refers to embedded config file in the "resources" folder in your source code and "file" refers to an external config file.

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

The event node is a platform-in-a-box to emulate a network event stream in the same developer's laptop.

For production, you would be using Kafka or Hazelcast as the event stream. In this case, you will deploy a "presence monitor" in the system.

You can then configure a "presence reporter" in your service module to report to the presence monitor. It uses websocket "presence" technology to inform the monitor when your module fails so that a new instance can be started.

# Spring Boot

The foundation code uses Spring Boot in the "rest-spring" library. For loose coupling, we use the `@MainApplication` as a replacement for the `SpringApplication`. Please refer to the MainApp class in the "rest-example" project.
This allows us to use any REST application server when technology evolves.

If your code uses Spring Boot or Spring Framework directly, you can set the corresponding key-values in the application.properties file in the resources folder. e.g. changing the "auto-configuration" parameters.

---

| Chapter-7                                | Home                                     |
| :---------------------------------------:|:----------------------------------------:|
| [Reserved route names](CHAPTER-7.md)     | [Table of Contents](TABLE-OF-CONTENTS.md)|

