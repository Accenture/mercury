# API reference

Mercury has a small set of API for declaring microservices functions and sending events to functions.

## Basic platform classes and utilities

1. `Platform` - the platform class is a singleton object for managing life cycle of functions.
2. `PostOffice` - a singleton object for sending events to functions.
3. `ServerPersonality` - a singleton object for setting the personality or type of an application unit. e.g. REST, WEB, APP, RESOURCES and DEVOPS
4. `AppConfigReader` - a singleton object for reading application.properties that can be overriden by run-time parameters or environment variables.
5. `CryptoApi` - a convenient crypto library for common cryptographic tasks like hashing, AES and RSA encryption and digital signing.
6. `ManagedCache` - a useful in-memory cache store.
7. `MultiLevelMap` - a HashMap wrapper that allows you to retrieve item using the dot-bracket syntax. e.g. map.getElement("hello.world[3]")
8. `Utility` - a convenient singleton object for commonly methods such as UTC time string, UTF, stream, file, etc.
10. `SimpleMapper` - a preconfigured JSON serializer.
11. `SimpleXmlParser` and `SimpleXmlWriter` - efficient XML serializer.

## EventEnvelope

EventEnvelope is a vehicle for storing and transporting an event that contains headers and body. `headers` can be used to carry parameters and `body` is the message payload. Each event should have either or both of headers and body. You can set Java primitive, Map or PoJo into the body.

Mercury automatically performs serialization and deserialization using the EventEnvelope's `toBytes()` and `load(bytes)` methods. For performance and network efficiency, it is using [Gson](https://github.com/google/gson) and [MsgPack](https://msgpack.org/) for serialization into Map and byte array respectively.

EventEnvelope is used for both input and output. For simple use cases in asynchronous operation, you do not need to use the EventEnvelope. For RPC call, the response object is an EventEnvelope. The service response is usually stored in the "body" in the envelope. A service may also return key-values in the "headers" field.

Mercury is truly schemaless. It does not care if you are sending a Java primitive, Map or PoJo. The calling function and the called function must understand each other's API interface contract to communicate properly.

## Platform API

### Obtain an instance of the platform object

```
Platform platform = Platform.getInstance();
```

### Register a public function

To register a function, you can assign a route name to a function instance. You can also set the maximum number of concurrent workers in an application instance. This provides vertical scalability in addition to horizontal scaling by Docker/Kubernetes.

To create a singleton function, set instances to 1.

```
platform.register(String route, LambdaFunction lambda, int instances) throws IOException;

e.g.
platform.register("hello.world", echo, 20);
```

### Register a private function

Public functions are advertised to the whole system while private functions are encapsulated within an application instance.

You may define your function as `private` if it is used internally by other functions in the same application instance. 

```
platform.registerPrivate(String route, LambdaFunction lambda, int instances) throws IOException;
```

### Release a function

A function can be long term or transient. When a function is no longer required, you can cancel the function using the "release" method.

```
void release(String route) throws IOException;
```

### Connect to the cloud

You can write truly event-driven microservices as a standalone application. However, it would be more interesting to connect the services together through a network event stream system.

To do this, you can ask the platform to connect to the cloud.
```
platform.connectToCloud();
```

| Chapter-3                              | Home                                     |
| :-------------------------------------:|:----------------------------------------:|
| [Post Office API](CHAPTER-3.md)        | [Table of Contents](TABLE-OF-CONTENTS.md)|
