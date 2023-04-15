# application.properties

| Key                                        | Value (example)                                               | Required  |
|:-------------------------------------------|:--------------------------------------------------------------|:----------|
| application.name                           | Application name                                              | Yes       |
| spring.application.name                    | Alias for application name                                    | Yes*1     |
| info.app.version                           | major.minor.build (e.g. 1.0.0)                                | Yes       |
| info.app.description                       | Something about your application                              | Yes       |
| web.component.scan                         | your own package path or parent path                          | Yes       |
| server.port                                | e.g. 8083                                                     | Yes*1     |
| rest.automation                            | true if you want to enable automation                         | Optional  |
| rest.server.port                           | e.g. 8085                                                     | Optional  |
| websocket.server.port                      | Alias for rest.server.port                                    | Optional  |
| static.html.folder                         | classpath:/public/                                            | Yes*      |
| spring.web.resources.static-locations      | (alias for static.html.folder)                                | Yes*1     |
| spring.mvc.static-path-pattern             | /**                                                           | Yes*1     |
| jax.rs.application.path                    | /api                                                          | Optional* |
| show.env.variables                         | comma separated list of variable names                        | Optional  |
| show.application.properties                | comma separated list of property names                        | Optional  |
| cloud.connector                            | kafka, hazelcast, none, etc.                                  | Optional  |
| cloud.services                             | e.g. some.interesting.service                                 | Optional  |
| snake.case.serialization                   | true (recommended)                                            | Optional  |
| safe.data.models                           | packages pointing to your PoJo classes                        | Optional  |
| protect.info.endpoints                     | true to disable actuators. Default: true                      | Optional  |
| trace.http.header                          | comma separated list. Default "X-Trace-Id"                    | Optional  |
| index.redirection                          | comma separated list of URI paths                             | Optional* |
| index.page                                 | default is index.html                                         | Optional* |
| hsts.feature                               | default is true                                               | Optional* |
| application.feature.route.substitution     | default is false                                              | Optional  |
| route.substitution.file                    | points to a config file                                       | Optional  |
| application.feature.topic.substitution     | default is false                                              | Optional  |
| topic.substitution.file                    | points to a config file                                       | Optional  |
| kafka.replication.factor                   | 3                                                             | Kafka     |
| cloud.client.properties                    | e.g. classpath:/kafka.properties                              | Connector |
| user.cloud.client.properties               | e.g. classpath:/second-kafka.properties                       | Connector |
| default.app.group.id                       | groupId for the app instance.<br/>Default: appGroup           | Connector |
| default.monitor.group.id                   | groupId for the presence-monitor.<br/>Default: monitorGroup   | Connector |
| monitor.topic                              | topic for the presence-monitor.<br/>Default: service.monitor  | Connector |
| app.topic.prefix                           | Default: multiplex (DO NOT change)                            | Connector |
| app.partitions.per.topic                   | Max Kafka partitions per topic.<br/>Default: 32               | Connector |
| max.virtual.topics                         | Max virtual topics = partitions * topics.<br/> Default: 288   | Connector |
| max.closed.user.groups                     | Number of closed user groups. <br/>Default: 10, range: 3 - 30 | Connector |
| closed.user.group                          | Closed user group. Default: 1                                 | Connector |
| transient.data.store                       | Default is "/tmp/reactive"                                    | Optional  |
| running.in.cloud                           | Default is false (set to true if containerized)               | Optional  |
| multicast.yaml                             | points to the multicast.yaml config file                      | Optional  |
| journal.yaml                               | points to the journal.yaml config file                        | Optional  |
| deferred.commit.log                        | Default is false (may be set to true in unit tests)           | Optional  |

`*` - when using the "rest-spring" library

# HTTP and websocket port assignment

If `rest.automation=true` and `rest.server.port or server.port` are configured, the system will start
a lightweight non-blocking HTTP server. If `rest.server.port` is not available, it will fall back to `server.port`.

If `rest.automation=false` and you have a websocket server endpoint annotated as `WebsocketService`, the system
will start a non-blocking Websocket server with a minimalist HTTP server that provides actuator services.
If `websocket.server.port` is not available, it will fall back to `rest.server.port` or `server.port`.

If you add Spring Boot dependency, Spring Boot will use `server.port` to start Tomcat or similar HTTP server.

The built-in lightweight non-blocking HTTP server and Spring Boot can co-exist when you configure
`rest.server.port` and `server.port` to use different ports.

Note that the `websocket.server.port` parameter is an alias of `rest.server.port`.

# Transient data store

The system handles back-pressure automatically by overflowing events from memory to a transient data store. 
As a cloud native best practice, the folder must be under "/tmp". The default is "/tmp/reactive". 
The "running.in.cloud" parameter must be set to false when your apps are running in IDE or in your laptop. 
When running in kubernetes, it can be set to true.

# The safe.data.models parameter

PoJo may contain Java code. As a result, it is possible to inject malicious code that does harm when 
deserializing a PoJo. This security risk applies to any JSON serialization engine.

For added security and peace of mind, you may want to protect your PoJo package paths.
When the `safe.data.models` parameter is configured, the underlying serializers for JAX-RS, Spring RestController
and Servlets will respect this setting and enforce PoJo filtering.

If there is a genuine need to programmatically perform serialization, you may use the pre-configured serializer 
so that the serialization behavior is consistent.

You can get an instance of the serializer with `SimpleMapper.getInstance().getMapper()`.

The serializer may perform snake case or camel serialization depending on the parameter `snake.case.serialization`.

If you want to ensure snake case or camel, you can select the serializer like this:

```java
SimpleObjectMapper snakeCaseMapper = SimpleMapper.getInstance().getSnakeCaseMapper();
SimpleObjectMapper camelCaseMapper = SimpleMapper.getInstance().getCamelCaseMapper();
```

# The trace.http.header parameter

The `trace.http.header` parameter sets the HTTP header for trace ID. When configured with more than one label,
the system will retrieve trace ID from the corresponding HTTP header and propagate it through the transaction
that may be served by multiple services.

If trace ID is presented in an HTTP request, the system will use the same label to set HTTP response traceId header.

```text
X-Trace-Id: a9a4e1ec-1663-4c52-b4c3-7b34b3e33697
or
X-Correlation-Id: a9a4e1ec-1663-4c52-b4c3-7b34b3e33697
```

# Kafka specific configuration

If you use the kafka-connector (cloud connector) and kafka-presence (presence monitor), you may want to 
externalize kafka.properties like this:

```properties
cloud.client.properties=file:/tmp/config/kafka.properties
```

Note that "classpath" refers to embedded config file in the "resources" folder in your source code and "file" 
refers to an external config file.

You want also use the embedded config file as a backup like this:

```properties
cloud.client.properties=file:/tmp/config/kafka.properties,classpath:/kafka.properties
```

# Distributed trace

To enable distributed trace logging, please set this in log4j2.xml:

```
<logger name="org.platformlambda.core.services.DistributedTrace" level="INFO" />
```
<br/>

|          Chapter-8           |                   Home                    |              Appendix-II               |
|:----------------------------:|:-----------------------------------------:|:--------------------------------------:|
| [Service mesh](CHAPTER-8.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [Reserved route names](APPENDIX-II.md) |
