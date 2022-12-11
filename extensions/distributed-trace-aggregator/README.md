# Distributed tracer sample application

This is a sample application for the distributed trace aggregator.

DO NOT use this for production. It is meant to be used as a demo app to illustrate how to aggregate trace metrics.
For this demo, the perf metrics are printed onto the standard output.

For production, please write your own custom aggregator.

This application provides service using the route "distributed.trace.processor" to receive trace information.

In addition to printing the trace information as log messages, it is a websocket server for distributed trace
UI applications to connect.

# IMPORTANT - distributed trace logging vs trace aggregator

There are 2 approaches in trace collection:

1. Decentralized

    You can implement a function with route "distributed.tracing" in each application to intercept perf metrics.

2. Centralized

    You can implement a separate application with service called "distributed.trace.processor" to collect
    traces from all application instances in the system. 

The advantage of decentralized approach is that it is lighter weight. It is like have an agent in each application
instance to report perf metrics.

The advantage of centralized approach is that you can aggregate traces from all application instances and decide
how to process them. e.g. You can keep them in a database or search engine for further analysis. However, the
disadvantage is that it would consume more network traffic in the network event stream system. Therefore, you
must ensure the network has sufficient capacity to handle the additional workload.

## Turning on tracing

Distributed traces are initiated at the edge by the REST automation system.

To enable distributed trace, please set "tracing=true" for the REST endpoints in the "rest.yaml" file that
you want to trace. For details, please refer to the REST automation application subproject in the "extensions" packages.

## Transaction journaling

Optionally, you may enable transaction journaling for selected services. To enable journaling, you can define
the service routes in journal config YAML file. Journaling is a superset of distributed trace. You would need
to write your own distributed trace aggregator.

## Sample trace metrics

The following is a sample output when the browser hits the "hello.world" service provided by a python service.
The trace shows that the event passes through 3 services: "hello.world" at the language-connector,
"hello.world" service in python script and "async.http.response" by the rest-automation system.

```
{
  "trace": {
    "path": "GET /api/hello/world",
    "service": "async.http.response",
    "success": true,
    "origin": "2020051088c413a3a33c4d6082be287b1d51a0d8",
    "start": "2020-05-10T23:44:19.290Z",
    "exec_time": 0.418,
    "id": "fee3d82fd3dd47fc883aefb61f2f2fe8"
  },
  "annotations": {},
  "type": "trace"
}
{
  "trace": {
    "path": "GET /api/hello/world",
    "service": "hello.world",
    "success": true,
    "origin": "py0356ba1413324686b2828439634a4d37",
    "start": "2020-05-10T23:44:19.283Z",
    "exec_time": 0.191,
    "id": "fee3d82fd3dd47fc883aefb61f2f2fe8"
  },
  "annotations": {},
  "type": "trace"
}
{
  "trace": {
    "path": "GET /api/hello/world",
    "service": "hello.world",
    "success": true,
    "origin": "202005109b77436f7d1141078fd1a6d65b2bd7bf",
    "start": "2020-05-10T23:44:19.278Z",
    "exec_time": 0.218,
    "id": "fee3d82fd3dd47fc883aefb61f2f2fe8"
  },
  "annotations": {
    "version": "language-connector 2.6.0",
    "target": "py0356ba1413324686b2828439634a4d37"
  },
  "type": "trace"
}
```

## UI application

If you save the perf metrics into a search engine. You can then render the metrics with a dashboard such as 
Kibana or Grafana.

If you want to do your own visualization, you may implement a single page application (React, Angular, etc.)
to render the metrics retrieved from the search engine.
