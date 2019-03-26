# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

`The current stable release is version 1.11.33, 3/25/2019`

---

## Version 1.11.33, 3/25/2019

### Added

N/A

### Removed

N/A

### Changed

Move safe.data.models validation rules from EventEnvelope to SimpleMapper
Apache fluent HTTP client downgraded to version 4.5.6 because the pom file in 4.5.7 is invalid

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
