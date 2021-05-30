# Version control

Sometimes, it would be useful to deploy more than one version of the same service in an environment.

This allows us to test new versions without impacting existing functionality.

Version control is supported in the REST automation system and the Post Office event core engine.

To do that, we can add a prefix or suffix to a service route name.

For example, the service route name is originally "hello.world". The versioned services would be:

```
v1.hello.world - the current version
v2.hello.world - the next version of the service with functional improvement or bug fix
```

There are two ways to do versioning:

1. REST automation
2. Route substitution

 Let's examine how to do versioning from the REST automation system.

# REST automation

Versioning would start with adding version in the URI path of a REST endpoint.

In the rest.yaml configuration file, we can define multiple versions of the REST endpoint to route to different versions of backend services.

e.g.

```
  - service: "v1.hello.world"
    methods: ['GET']
    url: "/api/v1/hello/world"
  - service: "v2.hello.world"
    methods: ['GET']
    url: "/api/v2/hello/world"
```

In this case, the UI application must be aware of the different versions of URI path.

# Route substitution

Route substitution works differently. The URI path of a specific REST endpoint is version neutral. 

e.g.
```
  - service: "hello.world"
    methods: ['GET']
    url: "/api/hello/world"
```

We would create a route substitution to automatically change the routing.

you can turn on route substitution with the following parameters in application.properties

```
application.feature.route.substitution=true
route.substitution.file=file:/tmp/config/route-substitution.yaml,classpath:/route-substitution.yaml
```

You can point the `route.substitution.file` to a file. e.g. file:/tmp/config/route-substitution.yaml

The content of route substitution file may look like this:

```
route:
  substitution:
    - "hello.world -> v2.hello.world"
```

In the above example, the REST endpoint "/api/hello/world" will route the HTTP request to "hello.world" and
the system will route "hello.world" to "v2.hello.world".

In this case, we usually deploy more than one REST automation gatways for user facing. We can use DNS routing to route a percentage of current user requests to the REST automation gateway that sends requests to version 2 of the backend service.


---

| Appendix-I                                | Home                                     |
| :----------------------------------------:|:----------------------------------------:|
| [application.properties](APPENDIX-I.md)   | [Table of Contents](TABLE-OF-CONTENTS.md)|
