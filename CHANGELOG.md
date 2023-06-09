# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---
## Version 3.0.2, 6/9/2023

### Added

N/A

### Removed

N/A

### Changed

Consistent exception handling for Event API endpoint

---
## Version 3.0.1, 6/5/2023

In this release, we have replace Google HTTP Client with vertx non-blocking WebClient.
We also tested compatibility up to OpenJDK version 20 and maven 3.9.2.

### Added

When "x-raw-xml" HTTP request header is set to "true", the AsyncHttpClient will skip the built-in 
XML serialization so that your application can retrieve the original XML text.

### Removed

Retire Google HTTP client

### Changed

Upgrade maven plugin versions.


---
## Version 3.0.0, 4/18/2023

This is a major release with some breaking changes. Please refer to Chapter-10 (Migration guide) for details.
This version brings the best of preemptive and cooperating multitasking to Java (version 1.8 to 19) before
Java 19 virtual thread feature becomes officially available.

### Added

1. Function execution engine supporting kernel thread pool, Kotlin coroutine and suspend function
2. "Event over HTTP" service for inter-container communication
3. Support for Spring Boot version 3 and WebFlux
4. Sample code for a pre-configured Spring Boot 3 application

### Removed

1. Remove blocking APIs from platform-core
2. Retire PM2 process manager sample script due to compatibility issue

### Changed

1. Refactor "async.http.request" to use vertx web client for non-blocking operation
2. Update log4j2 version 2.20.0 and slf4j version 2.0.7 in platform-core
3. Update JBoss RestEasy JAX_RS to version 3.15.6.Final in rest-spring
4. Update vertx to 4.4.2
5. Update Spring Boot parent pom to 2.7.12 and 3.1.0 for spring boot 2 and 3 respectively
6. Remove com.fasterxml.classmate dependency from rest-spring

---
## Version 2.8.0, 3/20/2023


### Added

N/A

### Removed

N/A

### Changed

1. Improved load balancing in cloud-connector
2. Filter URI to avoid XSS attack
3. Upgrade to SnakeYaml 2.0 and patch Spring Boot 2.6.8 for compatibility with it
4. Upgrade to Vertx 4.4.0, classgraph 4.8.157, tomcat 9.0.73

---
## Version 2.7.1, 12/22/2022


### Added

1. standalone benchmark report app
2. client and server benchmark apps
3. add timeout tag to RPC events

### Removed

N/A

### Changed

1. Updated open sources dependencies
- Netty 4.1.86.Final
- Tomcat 9.0.69
- Vertx 4.3.6
- classgraph 4.8.152
- google-http-client 1.42.3

2. Improved unit tests to use assertThrows to evaluate exception
3. Enhanced AsyncHttpRequest serialization

---
## Version 2.7.0, 11/11/2022

In this version, REST automation code is moved to platform-core such that REST and Websocket
service can share the same port.

### Added

1. AsyncObjectStreamReader is added for non-blocking read operation from an object stream.
2. Support of LocalDateTime in SimpleMapper
3. Add "removeElement" method to MultiLevelMap
4. Automatically convert a map to a PoJo when the sender does not specify class in event body

### Removed

N/A

### Changed

1. REST automation becomes part of platform-core and it can co-exist with Spring Web in the rest-spring module
2. Enforce Spring Boot lifecycle management such that user apps will start after Spring Boot has loaded all components
3. Update netty to version 4.1.84.Final

---
## Version 2.6.0, 10/13/2022

In this version, websocket notification example code has been removed from the REST automation system.
If your application uses this feature, please recover the code from version 2.5.0 and refactor it as a
separate library.

### Added

N/A

### Removed

Simplify REST automation system by removing websocket notification example in REST automation.

### Changed

1. Replace Tomcat websocket server with Vertx non-blocking websocket server library
2. Update netty to version 4.1.79.Final
3. Update kafka client to version 2.8.2
4. Update snake yaml to version 1.33
5. Update gson to version 2.9.1

---
## Version 2.5.0, 9/10/2022

### Added

New Preload annotation class to automate pre-registration of LambdaFunction.

### Removed

Removed Spring framework and Tomcat dependencies from platform-core so that the core library can be applied
to legacy J2EE application without library conflict.

### Changed

1. Bugfix for proper housekeeping of future events.
2. Make Gson and MsgPack handling of integer/long consistent

Updated open sources libraries.

1. Eclipse vertx-core version 4.3.4
2. MsgPack version 0.9.3
3. Google httpclient version 1.42.2
4. SnakeYaml version 1.31

---
## Version 2.3.6, 6/21/2022

### Added

Support more than one event stream cluster. User application can share the same event stream cluster
for pub/sub or connect to an alternative cluster for pub/sub use cases.

### Removed

N/A

### Changed

Cloud connector libraries update to Hazelcast 5.1.2

---
## Version 2.3.5, 5/30/2022

### Added

Add tagging feature to handle language connector's routing and exception handling

### Removed

Remove language pack's pub/sub broadcast feature

### Changed

1. Update Spring Boot parent to version 2.6.8 to fetch Netty 4.1.77 and Spring Framework 5.3.20
2. Streamlined language connector transport protocol for compatibility with both Python and Node.js

---
## Version 2.3.4, 5/14/2022

### Added

N/A

### Removed

1. Remove swagger-ui distribution from api-playground such that developer can clone the latest version

### Changed

1. Update application.properties (from spring.resources.static-locations to spring.web.resources.static-locations)
2. Update log4j, Tomcat and netty library version using Spring parent 2.6.6

---
## Version 2.3.3, 3/30/2022

### Added

Enhanced AsyncRequest to handle non-blocking fork-n-join

### Removed

N/A

### Changed

Upgrade Spring Boot from 2.6.3 to 2.6.6

---
## Version 2.3.2, 2/21/2022

### Added

Add support of queue API in native pub/sub module for improved ESB compatibility

### Removed

N/A

### Changed

N/A

---

## Version 2.3.1, 2/19/2022

### Added

N/A

### Removed

N/A

### Changed

1. Update Vertx to version 4.2.4
2. Update Tomcat to version 5.0.58
3. Use Tomcat websocket server for presence monitors
4. Bugfix - Simple Scheduler's leader election searches peers correctly

---
## Version 2.3.0, 1/28/2022

### Added

N/A

### Removed

N/A

### Changed

1. Update copyright notice
2. Update Vertx to version 4.2.3
3. Bugfix - RSA key generator supporting key length from 1024 to 4096 bits
4. CryptoAPI - support different AES algorithms and custom IV
5. Update Spring Boot to version 2.6.3

---
## Version 2.2.3, 12/29/2021

### Added

1. Transaction journaling
2. Add parameter `distributed.trace.aggregation` in application.properties such that trace aggregation 
   may be disabled.

### Removed

N/A

### Changed

1. Update JBoss RestEasy library to 3.15.3.Final
2. Improved po.search(route) to scan local and remote service registries. Added "remoteOnly" selection.
3. Fix bug in releasing presence monitor topic for specific closed user group
4. Update Apache log4j to version 2.17.1
5. Update Spring Boot parent to version 2.6.1
6. Update Netty to version 4.1.72.Final
7. Update Vertx to version 4.2.2
8. Convenient class "UserNotification" for backend service to publish events to the UI when REST automation is deployed

---
## Version 2.2.2, 11/12/2021

### Added

1. User defined API authentication functions can be selected using custom HTTP request header
2. "Exception chaining" feature in EventEnvelope
3. New "deferred.commit.log" parameter for backward compatibility with older PowerMock in unit tests

### Removed

N/A

### Changed

1. Improved and streamlined SimpleXmlParser to handle arrays
2. Bugfix for file upload in Service Gateway (REST automation library)
3. Update Tomcat library from 9.0.50 to 9.0.54
4. Update Spring Boot library to 2.5.6
5. Update GSON library to 2.8.9

---
## Version 2.2.1, 10/1/2021

### Added

Callback function can implement ServiceExceptionHandler to catch exception. It adds the onError() method.

### Removed

N/A

### Changed

Open sources library update - Vert.x 4.1.3, Netty 4.1.68-Final

---
## Version 2.1.1, 9/10/2021

### Added

1. User defined PoJo and Generics mapping
2. Standardized serializers for default case, snake_case and camelCase
3. Support of EventEnvelope as input parameter in TypedLambdaFunction so application function can inspect event's 
   metadata
4. Application can subscribe to life cycle events of other application instances

### Removed

N/A

### Changed

1. Replace Tomcat websocket server engine with Vertx in presence monitor for higher performance
2. Bugfix for MsgPack transport of integer, long, BigInteger and BigDecimal

---
## Version 2.1.0, 7/25/2021

### Added

1. Multicast - application can define a multicast.yaml config to relay events to more than one target service.
2. StreamFunction - function that allows the application to control back-pressure

### Removed

"object.streams.io" route is removed from platform-core

### Changed

1. Elastic Queue - Refactored using Oracle Berkeley DB
2. Object stream I/O - simplified design using the new StreamFunction feature
3. Open sources library update - Spring Boot 2.5.2, Tomcat 9.0.50, Vert.x 4.1.1, Netty 4.1.66-Final

---
## Version 2.0.0, 5/5/2021

Vert.x is introduced as the in-memory event bus

### Added

1. ActiveMQ and Tibco connectors
2. Admin endpoints to stop, suspend and resume an application instance
3. Handle edge case to detect stalled application instances
4. Add "isStreamingPubSub" method to the PubSub interface

### Removed

1. Event Node event stream emulator has been retired. You may use standalone Kafka server as a replacement for 
   development and testing in your laptop.
2. Multi-tenancy namespace configuration has been retired. It is replaced by the "closed user group" feature.

### Changed

1. Refactored Kafka and Hazelcast connectors to support virtual topics and closed user groups.
2. Updated ConfigReader to be consistent with Spring value substitution logic for application properties
3. Replace Akka actor system with Vert.x event bus
4. Common code for various cloud connectors consolidated into cloud core libraries

---
## Version 1.13.0, 1/15/2021

Version 1.13.0 is the last version that uses Akka as the in-memory event system.

---
## Version 1.12.66, 1/15/2021

### Added

1. A simple websocket notification service is integrated into the REST automation system
2. Seamless migration feature is added to the REST automation system

### Removed

Legacy websocket notification example application

### Changed

N/A

---
## Version 1.12.65, 12/9/2020

### Added

1. "kafka.pubsub" is added as a cloud service
2. File download example in the lambda-example project
3. "trace.log.header" added to application.properties - when tracing is enabled, this inserts the trace-ID of the 
   transaction in the log context. For more details, please refer to the [Developer Guide](/guides/CHAPTER-5.md)
4. Add API to pub/sub engine to support creation of topic with partitions
5. TypedLambdaFunction is added so that developer can predefine input and output classes in a service without casting

### Removed

N/A

### Changed

1. Decouple Kafka pub/sub from kafka connector so that native pub/sub can be used when application is running in 
   standalone mode
2. Rename "relay" to "targetHost" in AsyncHttpRequest data model
3. Enhanced routing table distribution by sending a complete list of route tables, thus reducing network admin traffic.

---
## Version 1.12.64, 9/28/2020

### Added

If predictable topic is set, application instances will report their predictable topics as "instance ID"
to the presence monitor. This improves visibility when a developer tests their application in "hybrid" mode.
i.e. running the app locally and connect to the cloud remotely for event streams and cloud resources.

### Removed

N/A

### Changed

N/A

---
## Version 1.12.63, 8/27/2020

### Added

N/A

### Removed

N/A

### Changed

Improved Kafka producer and consumer pairing

---
## Version 1.12.62, 8/12/2020

### Added

New presence monitor's admin endpoint for the operator to force routing table synchronization ("/api/ping/now")

### Removed

N/A

### Changed

Improved routing table integrity check

---
## Version 1.12.61, 8/8/2020

### Added

Event stream systems like Kafka assume topic to be used long term. 
This version adds support to reuse the same topic when an application instance restarts.

You can create a predictable topic using unique application name and instance ID.
For example, with Kubernetes, you can use the POD name as the unique application instance topic.

### Removed

N/A

### Changed

N/A

---
## Version 1.12.56, 8/4/2020

### Added

Automate trace for fork-n-join use case

### Removed

N/A

### Changed

N/A

---
## Version 1.12.55, 7/19/2020

### Added

N/A

### Removed

N/A

### Changed

Improved distributed trace - set the "from" address in EventEnvelope automatically.

---
## Version 1.12.54, 7/10/2020

### Added

N/A

### Removed

N/A

### Changed

Application life-cycle management - User provided main application(s) will be started after Spring Boot declares web
application ready. This ensures correct Spring autowiring or dependencies are available.

Bugfix for locale - String.format(float) returns comma as decimal point that breaks number parser. 
Replace with BigDecimal decimal point scaling.

Bugfix for Tomcat 9.0.35 - Change Async servlet default timeout from 30 seconds to -1 so the system can handle the 
whole life-cycle directly.

---
## Version 1.12.52, 6/11/2020

### Added

1. new "search" method in Post Office to return a list of application instances for a service
2. simple "cron" job scheduler as an extension project
3. add "sequence" to MainApplication annotation for orderly execution when more than one MainApplication is available
4. support "Optional" object in EventEnvelope so a LambdaFunction can read and return Optional

### Removed

N/A

### Changed

1. The rest-spring library has been updated to support both JAR and WAR deployment
2. All pom.xml files updated accordingly
3. PersistentWsClient will back off for 10 seconds when disconnected by remote host

---
## Version 1.12.50, 5/20/2020

### Added

1. Payload segmentation

   For large payload in an event, the payload is automatically segmented into 64 KB segments.
   When there are more than one target application instances, the system ensures that the segments of the same event 
   is delivered to exactly the same target.

2. PersistentWsClient added - generalized persistent websocket client for Event Node, Kafka reporter and Hazelcast
   reporter.

### Removed

N/A

### Changed

1. Code cleaning to improve consistency
2. Upgraded to hibernate-validator to v6.1.5.Final and Hazelcast version 4.0.1
3. REST automation is provided as a library and an application to handle different use cases

---
## Version 1.12.40, 5/4/2020

### Added

N/A

### Removed

N/A

### Changed

For security reason, upgrade log4j to version 2.13.2

---
## Version 1.12.39, 5/3/2020

### Added

Use RestEasy JAX-RS library

### Removed

For security reason, removed Jersey JAX-RS library

### Changed

1. Updated RestLoader to initialize RestEasy servlet dispatcher
2. Support nested arrays in MultiLevelMap

---
## Version 1.12.36, 4/16/2020

### Added

N/A

### Removed

For simplicity, retire route-substitution admin endpoint. Route substitution uses a simple static table in 
route-substitution.yaml.

### Changed

N/A

---
## Version 1.12.35, 4/12/2020

### Added

N/A

### Removed

SimpleRBAC class is retired

### Changed

1. Improved ConfigReader and AppConfigReader with automatic key-value normalization for YAML and JSON files
2. Improved pub/sub module in kafka-connector

---
## Version 1.12.34, 3/28/2020

### Added

N/A

### Removed

Retired proprietary config manager since we can use the "BeforeApplication" approach to load config from Kubernetes 
configMap or other systems of config record.

### Changed

1. Added "isZero" method to the SimpleMapper class
2. Convert BigDecimal to string without scientific notation (i.e. toPlainString instead of toString)
3. Corresponding unit tests added to verify behavior

---
## Version 1.12.32, 3/14/2020

### Added

N/A

### Removed

N/A

### Changed

Kafka-connector will shutdown application instance when the EventProducer cannot send event to Kafka. 
This would allow the infrastructure to restart application instance automatically.

---
## Version 1.12.31, 2/26/2020

### Added

N/A

### Removed

N/A

### Changed

1. Kafka-connector now supports external service provider for Kafka properties and credentials. 
   If your application implements a function with route name "kafka.properties.provider" before connecting to cloud, 
   the kafka-connector will retrieve kafka credentials on demand. This addresses case when kafka credentials change 
   after application start-up.
2. Interceptors are designed to forward requests and thus they do not generate replies. However, if you implement a 
   function as an EventInterceptor, your function can throw exception just like a regular function and the exception 
   will be returned to the calling function. This makes it easier to write interceptors.

---
## Version 1.12.30, 2/6/2020

### Added

1. Expose "async.http.request" as a PUBLIC function ("HttpClient as a service")

### Removed

N/A

### Changed

1. Improved Hazelcast client connection stability
2. Improved Kafka native pub/sub

---
## Version 1.12.29, 1/10/2020

### Added

1. Rest-automation will transport X-Trace-Id from/to Http request/response, therefore extending distributed trace 
   across systems that support the X-Trace-Id HTTP header.
2. Added endpoint and service to shutdown application instance.

### Removed

N/A

### Changed

1. Updated SimpleXmlParser with XML External Entity (XXE) injection prevention.
2. Bug fix for hazelcast recovery logic - when a hazelcast node is down, the app instance will restart the hazelcast 
   client and reset routing table correctly.
3. HSTS header insertion is optional so that we can disable it to avoid duplicated header when API gateway is doing it.

---
## Version 1.12.26, 1/4/2020

### Added

Feature to disable PoJo deserialization so that caller can decide if the result set should be in PoJo or a Map.

### Removed

N/A

### Changed

1. Simplified key management for Event Node
2. AsyncHttpRequest case insensitivity for headers, cookies, path parameters and session key-values
3. Make built-in configuration management optional

---
## Version 1.12.19, 12/28/2019

### Added

Added HTTP relay feature in rest-automation project

### Removed

N/A

### Changed

1. Improved hazelcast retry and peer discovery logic
2. Refactored rest-automation's service gateway module to use AsyncHttpRequest
3. Info endpoint to show routing table of a peer

---

## Version 1.12.17, 12/16/2019

### Added

1. Simple configuration management is added to event-node, hazelcast-presence and kafka-presence monitors
2. Added `BeforeApplication` annotation - this allows user application to execute some setup logic before the main 
   application starts. e.g. modifying parameters in application.properties
3. Added API playground as a convenient standalone application to render OpenAPI 2.0 and 3.0 yaml and json files
4. Added argument parser in rest-automation helper app to use a static HTML folder in the local file system if 
   arguments `-html file_path` is given when starting the JAR file.

### Removed

N/A

### Changed

1. Kafka publisher timeout value changed from 10 to 20 seconds
2. Log a warning when Kafka takes more than 5 seconds to send an event

---
## Version 1.12.14, 11/20/2019

### Added

1. getRoute() method is added to PostOffice to facilitate RBAC
2. The route name of the current service is added to an outgoing event when the "from" field is not present
3. Simple RBAC using YAML configuration instead of code

### Removed

N/A

### Changed

Updated Spring Boot to v2.2.1

---
## Version 1.12.12, 10/26/2019

### Added

Multi-tenancy support for event streams (Hazelcast and Kafka).
This allows the use of a single event stream cluster for multiple non-prod environments.
For production, it must use a separate event stream cluster for security reason.

### Removed

N/A

### Changed

1. logging framework changed from logback to log4j2 (version 2.12.1)
2. Use JSR-356 websocket annotated ClientEndpoint
3. Improved websocket reconnection logic

---
## Version 1.12.9, 9/14/2019

### Added

1. Distributed tracing implemented in platform-core and rest-automation
2. Improved HTTP header transformation for rest-automation

### Removed

N/A

### Changed

language pack API key obtained from environment variable 

---
## Version 1.12.8, 8/15/2019

### Added

N/A

### Removed

rest-core subproject has been merged with rest-spring

### Changed

N/A

---
## Version 1.12.7, 7/15/2019

### Added

1. Periodic routing table integrity check (15 minutes)
2. Set kafka read pointer to the beginning for new application instances except presence monitor
3. REST automation helper application in the "extensions" project
4. Support service discovery of multiple routes in the updated PostOffice's exists() method
5. logback to set log level based on environment variable LOG_LEVEL (default is INFO)

### Removed

N/A

### Changed

Minor refactoring of kafka-connector and hazelcast-connector to ensure that they can coexist if you want to include 
both of these dependencies in your project.

This is for convenience of dev and testing. In production, please select only one cloud connector library to reduce
memory footprint.

---

## Version 1.12.4, 6/24/2019

### Added

Add inactivity expiry timer to ObjectStreamIO so that house-keeper can clean up resources that are idle

### Removed

N/A

### Changed

1. Disable HTML encape sequence for GSON serializer
2. Bug fix for GSON serialization optimization
3. Bug fix for Object Stream housekeeper

By default, GSON serializer converts all numbers to double, resulting in unwanted decimal point for integer and long.
To handle custom map serialization for correct representation of numbers, an unintended side effect was introduced in 
earlier releases.

List of inner PoJo would be incorrectly serialized as map, resulting in casting exception. 
This release resolves this issue.

---

## Version 1.12.1, 6/10/2019

### Added

1. Store-n-forward pub/sub API will be automatically enabled if the underlying cloud connector supports it. e.g. kafka
2. ObjectStreamIO, a convenient wrapper class, to provide event stream I/O API.
3. Object stream feature is now a standard feature instead of optional.
4. Deferred delivery added to language connector.

### Removed

N/A

### Changed

N/A

---

## Version 1.11.40, 5/25/2019

### Added

1. Route substitution for simple versioning use case
2. Add "Strict Transport Security" header if HTTPS (https://tools.ietf.org/html/rfc6797)
3. Event stream connector for Kafka
4. Distributed housekeeper feature for Hazelcast connector

### Removed

System log service

### Changed

Refactoring of Hazelcast event stream connector library to sync up with the new Kafka connector.

---

## Version 1.11.39, 4/30/2019

### Added

Language-support service application for Python, Node.js and Go, etc.
Python language pack project is available at https://github.com/Accenture/mercury-python

### Removed

N/A

### Changed

1. replace Jackson serialization engine with Gson (`platform-core` project)
2. replace Apache HttpClient with Google Http Client (`rest-spring`)
3. remove Jackson dependencies from Spring Boot (`rest-spring`)
4. interceptor improvement

---

## Version 1.11.33, 3/25/2019

### Added

N/A

### Removed

N/A

### Changed

1. Move safe.data.models validation rules from EventEnvelope to SimpleMapper
2. Apache fluent HTTP client downgraded to version 4.5.6 because the pom file in 4.5.7 is invalid

---

## Version 1.11.30, 3/7/2019

### Added

Added retry logic in persistent queue when OS cannot update local file metadata in real-time for Windows based machine.

### Removed

N/A

### Changed

pom.xml changes - update with latest 3rd party open sources dependencies. 

---

## Version 1.11.29, 1/25/2019

### Added

`platform-core`

1. Support for long running functions so that any long queries will not block the rest of the system.
2. "safe.data.models" is available as an option in the application.properties. 
   This is an additional security measure to protect against Jackson deserialization vulnerability. 
   See example below:

```
#
# additional security to protect against model injection
# comma separated list of model packages that are considered safe to be used for object deserialization
#
#safe.data.models=com.accenture.models
```

`rest-spring`

"/env" endpoint is added. See sample application.properties below:

```
#
# environment and system properties to be exposed to the "/env" admin endpoint
#
show.env.variables=USER, TEST
show.application.properties=server.port, cloud.connector
```

### Removed

N/A

### Changed

`platform-core`

Use Java Future and an elastic cached thread pool for executing user functions.

### Fixed

N/A

---


## Version 1.11.28, 12/20/2018

### Added

Hazelcast support is added. This includes two projects (hazelcast-connector and hazelcast-presence).

Hazelcast-connector is a cloud connector library. Hazelcast-presence is the "Presence Monitor" for monitoring the 
presence status of each application instance.

### Removed

`platform-core`

The "fixed resource manager" feature is removed because the same outcome can be achieved at the application level. 
e.g. The application can broadcast requests to multiple application instances with the same route name and use a 
callback function to receive response asynchronously. The services can provide resource metrics so that the caller
can decide which is the most available instance to contact. 

For simplicity, resources management is better left to the cloud platform or the application itself.

### Changed

N/A

### Fixed

N/A
