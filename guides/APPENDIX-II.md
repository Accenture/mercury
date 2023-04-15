# Reserved route names

The Mercury foundation code is written using the same core API and each function has a route name.

The following route names are reserved. Please DO NOT use them in your application functions to avoid breaking
the system unintentionally.

| Route                        | Purpose                               | Modules          |
|:-----------------------------|:--------------------------------------|:-----------------|
| actuator.services            | Actuator endpoint services            | platform-core    |
| elastic.queue.cleanup        | Elastic event buffer clean up task    | platform-core    |
| distributed.tracing          | Distributed tracing logger            | platform-core    |
| system.ws.server.cleanup     | Websocket server cleanup service      | platform-core    |
| http.auth.handler            | REST automation authentication router | platform-core    |
| event.api.service            | Event API service                     | platform-core    |
| stream.to.bytes              | Event API helper function             | platform-core    |
| system.service.registry      | Distributed routing registry          | Connector        |
| system.service.query         | Distributed routing query             | Connector        |
| cloud.connector.health       | Cloud connector health service        | Connector        |
| cloud.manager                | Cloud manager service                 | Connector        |
| presence.service             | Presence signal service               | Connector        |
| presence.housekeeper         | Presence keep-alive service           | Connector        |
| cloud.connector              | Cloud event emitter                   | Connector        |
| init.multiplex.*             | reserved for event stream startup     | Connector        |
| completion.multiplex.*       | reserved for event stream clean up    | Connector        |
| async.http.request           | HTTP request event handler            | REST automation  |
| async.http.response          | HTTP response event handler           | REST automation  |
| cron.scheduler               | Cron job scheduler                    | Simple Scheduler |
| init.service.monitor.*       | reserved for event stream startup     | Service monitor  |
| completion.service.monitor.* | reserved for event stream clean up    | Service monitor  |

## Optional user defined functions

The following optional route names will be detected by the system for additional user defined features.

| Route                        | Purpose                                                                               |
|:-----------------------------|:--------------------------------------------------------------------------------------|
| additional.info              | User application function to return information<br/> about your application status    |
| distributed.trace.forwarder  | Custom function to forward performance metrics<br/> to a telemetry system             |
| transaction.journal.recorder | Custom function to record transaction request-response<br/> payloads into an audit DB |

The `additional.info` function, if implemented, will be invoked from the "/info" endpoint and its response
will be merged into the "/info" response.

For `distributed.trace.forwarder` and `transaction.journal.recorder`, please refer to [Chapter-5](CHAPTER-5.md)
for details.

## Reserved event header names

The following event headers are injected by the system as READ only metadata. They are available from the
input "headers". However, they are not part of the EventEnvelope.

| Header        | Purpose                                    | 
|:--------------|:-------------------------------------------|
| my_route      | route name of your function                |
| my_trace_id   | trace ID, if any, for the incoming event   |
| my_trace_path | trace path, if any, for the incoming event | 

You can create a trackable PostOffice using the "headers" and the "instance" parameters in the input arguments
of your function. The FastRPC instance requires only the "headers" parameters.

```java
// Java
PostOffice po = new PostOffice(headers, instance);

// Kotlin
val fastRPC = FastRPC(headers);
```
<br/>

|               Appendix-I                |                   Home                    |                Appendix-III                 |
|:---------------------------------------:|:-----------------------------------------:|:-------------------------------------------:|
| [application.properties](APPENDIX-I.md) | [Table of Contents](TABLE-OF-CONTENTS.md) | [Actuator and HTTP client](APPENDIX-III.md) |
