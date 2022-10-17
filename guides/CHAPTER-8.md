# Multicast feature

Multicast is a convenient feature that relays an event from a route to multiple targets.

It is configured using a multicast.yaml file like this:

```
multicast:
  - source: "v1.hello.world"
    targets:
    - "v1.hello.service.1"
    - "v1.hello.service.2"
    - "v1.hello.service.3"
```

In the above example, the route "v1.hello.world" will be automatically multicasted to the target services with 
the configured routes.

# application.properties

To enable multicast, please add the following to application.properties

```
multicast.yaml=classpath:/multicast.yaml
```

If you want to externalize the multicast.yaml configuration file, you can change the parameter like this:

```
multicast.yaml=file:/tmp/config/multicast.yaml
```

# Routing behavior

Multicast is designed for best effort delivery. If a target route is not reachable, it will display a warning in
the application log like this:

```
Unable to relay v1.hello.world -> v1.hello.service.3 - target not reachable
```

# Real-time vs store-n-forward

The PostOffice is a real-time event system. For store-n-forward use case, please refer to streaming
Pub/Sub APIs in [Post Office API](CHAPTER-3.md)

---

| Appendix-I                                | Home                                     |
| :----------------------------------------:|:----------------------------------------:|
| [application.properties](APPENDIX-I.md)   | [Table of Contents](TABLE-OF-CONTENTS.md)|
