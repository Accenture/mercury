# application.properties

| Key                                         | Value                                     | Required |
| :-------------------------------------------|:------------------------------------------|:--------:|
| application.name or spring.application.name | Application name                          | Yes      |
| info.app.version                            | major.minor.build (e.g. 1.0.0)            | Yes      |
| info.app.description                        | Something about application               | Yes      |
| server.port                                 | e.g. 8083                                 | Yes*     |
| spring.mvc.static-path-pattern              | /**                                       | Yes*     |
| spring.resources.static-locations           | classpath:/public/                        | Yes*     |
| spring.jersey.application-path              | /api                                      | Yes*     |
| web.component.scan                          | your own package path(s)                  | Yes*     |
| show.env.variables                          | comma separated list of variable names    | Optional*|
| show.application.properties                 | comma separated list of property names    | Optional*|
| cloud.connector                             | event.node, hazelcast, kafka, etc.        | Optional |
| cloud.services                              | e.g. hazelcast.reporter                   | Optional |
| presence.monitor                            | e.g. ws://127.0.0.1:8080/ws/presence      | Optional |
| event.node.path                             | e.g. ws://127.0.0.1:8080/ws/events        | Optional |
| hazelcast.cluster                           | e.g. 127.0.0.1:5701,127.0.0.1:5702        | Optional |
| hazelcast.namespace                         | your system name for multi-tenancy        | Optional |
| snake.case.serialization                    | true (recommended)                        | Optional |
| app.config.enabled                          | true (default false)                      | Optional |
| app.config.key                              | comma separated list of key:secret        | Optional |
| app.config.manager                          | e.g. http://127.0.0.1:8080/api/config     | Optional |
| env.variables                               | e.g. MY_ENV:my.env                        | Optional |
| safe.data.models                            | packages pointing to your PoJo classes    | Optional |
| protected.info.endpoints                    | e.g. /route, /info, /env                  | Optional*|
| info.api.key.label                          | X-Info-Key                                | Optional*|
| info.api.key                                | some secret key (recommended to use UUID) | Optional*|
| application.feature.route.substitution      | default false                             | Optional |
| route.substitution                          | comma separated rules. e.g hi.here:v1.hi  | Optional |
| kafka.client.properties                     | classpath:/kafka.properties               | Kafka    |
| kafka.replication.factor                    | 3                                         | Kafka    |
| multi.tenancy.namespace                     | environment shortname                     | Optional |

`*` - when using the "rest-spring" library

# safe.data.models

PoJo may execute Java code. As a result, it is possible to inject malicious code that does harm when deserializing a PoJo.
This security risk applies to any JSON serialization engine.

For added security and peace of mind, you may want to white list your PoJo package paths.
When the "safe.data.models" property is configured, the underlying serializers for JAX-RS, Spring RestController, Servlets will respect this setting and enforce PoJo white listing.

Usually you do not need to use the serializer in your code because it is much better to deal with PoJo in your IDE.
However, if there is a genuine need to do low level coding, you may use the pre-configured serializer so that the serialization behavior is consistent.

You can get an instance of the serializer with `SimpleMapper.getInstance().getWhiteListMapper()`.

# info.api.key

This is used to authenticate HTTP requests to the "protected.info.endpoints".

# route substitution

This is usually used for blue/green environment tests. In some simple cases, you may use this for versioning. e.g. "hello.world" maps to "v1.hello.world".

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

# multi-tenancy for event stream systems

Since version 1.12.10, multi-tenancy is supported for hazelcast and kafka.
This is a convenient feature for non-prod environments to share a single event stream system.
For production, you should use a separate event stream cluster.

To enable multi-tenancy support, set the following parameters in application.properties like this:

```
# set a name for the environment. e.g. "dev"
# you can use any environment variable to map to a namespace. e.g. RUNTIME_ENV
multi.tenancy.namespace=dev
env.variables=RUNTIME_ENV:multi.tenancy.namespace
```

# Spring Boot

The foundation code uses Spring Boot in the "rest-spring" library. For loose coupling, we use the `@MainApplication` as a replacement for the `SpringApplication`. Please refer to the MainApp class in the "rest-example" project.
This allows us to use any REST application server when technology evolves.

If your code uses Spring Boot or Spring Framework directly, you can set the corresponding key-values in the application.properties file in the resources folder. e.g. changing the "auto-configuration" parameters.

[Table of Contents](TABLE-OF-CONTENTS.md)

