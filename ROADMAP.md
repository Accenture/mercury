# Technology roadmap for 2019

## Language Packs

The first language pack has been released with v1.11.38 for Python.

Upcoming language packs are Node.js and Go. 

To enable PolyGlot development, you can run the "language-connector" service application as a "side-car" so that language packs can use a very thin pipe to connect to the platform-core and let the side-car to do the heavy lifting.

Each language pack would provide the following functionality:
- In-memory event stream for private functions
- Registration of public and private functions
- Persistent socket connection to the "language-connector" sidecar

All the communication patterns (RPC, async, callback, pipeline, streaming and broadcast) are supported in language packs.

## REST automation and OpenAPI integration

The REST automation add-on would make REST definition "declarative" in addition to programmatic. Currently you can write REST endpoints in JAX-RS, Servlet or Spring.

Declarative approach would allow you to focus in writing your services. REST endpoints will be declaratively created from a YAML file. At a further iteration, we intend to create a UI to generate the YAML file.

Conceptually you can define something like this:

```
[GET, POST, PUT] /api/hello/world/* -> v1.my.web.tier.service
```

For performance reason, we will support direct URL or wildcard suffix `/*` instead of regular expression.

The YAML configuration file can be defined as a resource file inside your package or a file external to your application.

There will be a parameter in application.properties like this:

```
rest.automation=classpath:/rest.yaml
or
rest.automation=file:/config/rest.yaml
```

The REST automation add-on would be handy when we want to document and test a service. For REST endpoint, you can use OpenAPI to define the API contract.

In this way, the OpenAPI YAML files can be used to present the services as a service catalog.

## Versioning, circuit breaker and service metrics

These 3 features are closely related and thus they will be released together.

Currently RPC call returns execution time and round trip latency values. We intend to add a metrics add-on so that performance metrics for all kind of usage patterns (RPC, async, pipeline, callback, streaming and broadcast) will be available for submission to an external metrics visualization system.
