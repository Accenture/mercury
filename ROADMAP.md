# Technology roadmap for 2019

## Language Packs

The first language pack has been released with v1.11.38 for Python.

Upcoming language packs are Node.js and Go. 

To enable PolyGlot development, you can run the "language-support" service application as a "side-car" so that language packs can use a very thin pipe to connect to the platform-core and let the side-car to do all the heavy lifting.

Each language pack would provide the following functionality:
- In-memory event stream for private functions
- Registration of public and private functions
- Persistent socket connection to the "language-support" sidecar

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

The REST automation add-on would be handy when we want to test the API contract for a service. For REST endpoint, you can use OpenAPI to define the exact interface contract. For a microservice, you can use a POST request to test it. You can also restrict the IP address so that you only expose microservices for testing behind the firewall.

In this way, the OpenAPI YAML files can be used to present the services as a service catalog in addition to describe user facing REST endpoint.

Note that circular routing for pipeline is not allowed because it does not make sense to force your services into an infinite loop.

## Declarative service routing

Similar to REST automation, we can add declarative service routing in addition to programmatic routing. Currently, one service can request another service by specifying a route name like "v1.hello.world".

With declarative service routing, you can externalize the routing like this:

```
pipeline:
  v1.hello.world -> [v1.hello.backend, v1.hello.audit]
mapping:
  hello.world -> v1.hello.world
```

The declarative service routing is designed to address two use cases: (1) pipeline and (2) target route name mapping

In the above pipeline example, output from v1.hello.world will be piped as input to v1.hello.backend and v1.hello.audit concurrently.

Mapping is useful for service versioning. When both caller and called services are version aware, they can use the versioned route name. When the caller is not version aware, we can map the generic route to a versioned route name.


## Versioning and circuit breaker

This will be available soon.

## metrics gathering and consolidation

Currently all RPC call returns execution time and round trip latency values. We intend to add a metrics add-on so that performance metrics for all kind of usage patterns (RPC, async, pipeline, callback, streaming and broadcast) will be available for submission to an external metrics visualization system.

This is currently in design phase.



