# Reserved route names

The Mercury foundation functions are also written using the same event-driven platform-core. The following route names are reserved for the Mercury system functions.

Please do not use them in your application functions as it would disrupt the normal operation of the event-driven system and your application may not work as expected.

| Route name                    | Purpose                                | Modules                          |
| :-----------------------------|:---------------------------------------|:---------------------------------|
| object.streams.io             | Stream I/O handler                     | platform-core                    |
| distributed.tracing           | Distributed trace logger               | platform-core                    |
| shutdown.service              | Reserved for shutdown admin endpoint   | platform-core and rest-spring    |
| system.service.registry       | Distributed routing registry           | cloud connectors                 |
| system.service.query          | Distributed routing query              | cloud connectors                 |
| cloud.connector.health        | Cloud connector health service         | cloud connectors and rest-spring |
| additional.info               | Additional info service                | cloud connectors                 |
| cloud.manager                 | Cloud manager service                  | Cloud connectors                 |
| presence.service              | Presence reporter service              | cloud connectors                 |
| presence.housekeeper          | Presence housekeeper service           | cloud connectors                 |
| cloud.connector               | Cloud event emitter                    | Cloud connectors                 |
| async.http.request            | HTTP request event handler             | REST automation system           |
| async.http.response           | HTTP response event handler            | REST automation system           |
| notification.manager          | Simple notification service            | REST automation system           |
| ws.token.issuer               | Websocket access token issuer          | REST automation system           |
| ws.notification               | Websocket notification handler         | REST automation system           |
| ws.notification.internal      | Websocket notification handler         | REST automation system           |
| language.pack.inbox           | RPC inbox handler                      | Language Connector               |
| language.pack.registry        | Routing registry                       | Language Connector               |
| pub.sub.controller            | Pub/sub handler                        | Language Connector               |
| system.deferred.delivery      | Deferred delivery handler              | Language Connector               |
| cron.scheduler                | Cron job scheduler                     | Scheduler helper application     |

# Distributed trace processor

The route name "distributed.trace.processor" is reserved for user defined trace aggregator. If you implement a function with this route name, it will receive trace metrics in a real-time basis. Your custom application can then decide how to persist the metrics. e.g. Elastic Search or a database.

---

| Chapter-8                                | Home                                     |
| :---------------------------------------:|:----------------------------------------:|
| [Extras](CHAPTER-8.md)                   | [Table of Contents](TABLE-OF-CONTENTS.md)|

