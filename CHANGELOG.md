# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

Application life-cycle management - User provided main application(s) will be started after Spring Boot declares web application ready. This ensures correct Spring autowiring or dependencies are available.

Bugfix for locale - String.format(float) returns comma as decimal point that breaks number parser. Replace with BigDecimal decimal point scaling.

Bugfix for Tomcat 9.0.35 - Change Async servlet default timeout from 30 seconds to -1 so the system can handle the whole life-cycle directly.

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
When there are more than one target application instances, the system ensures that the segments of the same event is delivered
to exactly the same target.

2. PersistentWsClient added - generalized persistent websocket client for Event Node, Kafka reporter and Hazelcast reporter.

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

For simplicity, retire route-substitution admin endpoint. Route substitution uses a simple static table in route-substitution.yaml.

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

Retired proprietary config manager since we can use the "BeforeApplication" approach to load config from Kubernetes configMap or other systems of config record.

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

Kafka-connector will shutdown application instance when the EventProducer cannot send event to Kafka. This would allow the infrastructure to restart application instance automatically.

---
## Version 1.12.31, 2/26/2020

### Added

N/A

### Removed

N/A

### Changed

1. Kafka-connector now supports external service provider for Kafka properties and credentials. If your application implements a function with route name "kafka.properties.provider" before connecting to cloud, the kafka-connector will retrieve kafka credentials on demand. This addresses case when kafka credentials change after application start-up.
2. Interceptors are designed to forward requests and thus they do not generate replies. However, if you implement a function as an EventInterceptor, your function can throw exception just like a regular function and the exception will be returned to the calling function. This makes it easier to write interceptors.

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

1. Rest-automation will transport X-Trace-Id from/to Http request/response, therefore extending distributed trace across systems that support the X-Trace-Id HTTP header.
2. Added endpoint and service to shutdown application instance.

### Removed

N/A

### Changed

1. Updated SimpleXmlParser with XML External Entity (XXE) injection prevention.
2. Bug fix for hazelcast recovery logic - when a hazelcast node is down, the app instance will restart the hazelcast client and reset routing table correctly.
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
2. Added `BeforeApplication` annotation - this allows user application to execute some setup logic before the main application starts. e.g. modifying parameters in application.properties
3. Added API playground as a convenient standalone application to render OpenAPI 2.0 and 3.0 yaml and json files
4. Added argument parser in rest-automation helper app to use a static HTML folder in the local file system if arguments `-html file_path` is given when starting the JAR file.

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

Minor refactoring of kafka-connector and hazelcast-connector to ensure that they can coexist if you want to include both of these dependencies in your project.
This is for convenience of dev and testing. In production, please select only one cloud connector library to reduce memory footprint.

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
To handle custom map serialization for correct representation of numbers, an unintended side effect was introduced in earlier releases.
List of inner PoJo would be incorrectly serialized as map, resulting in casting exception. This release resolves this issue.

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
2. "safe.data.models" is available as an option in the application.properties. This is an additional security measure to protect against Jackson deserialization vulnerability. See example below:

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

Hazelcast-connector is a cloud connector library. Hazelcast-presence is the "Presence Monitor" for monitoring the presence status of each application instance.

### Removed

`platform-core`

The "fixed resource manager" feature is removed because the same outcome can be achieved at the application level. e.g. The application can broadcast requests to multiple application instances with the same route name and use a callback function to receive response asynchronously. The services can provide resource metrics so that the caller can decide which is the most available instance to contact. 

For simplicity, resources management is better left to the cloud platform or the application itself.

### Changed

N/A

### Fixed

N/A
