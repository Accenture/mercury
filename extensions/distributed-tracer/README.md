# Distributed tracer sample application

This is a sample distributed trace aggregator.

This application subscribes to the route "distributed.trace.processor" to receive distributed trace information.

In additon to printing the trace information as log messages, it is a websocket server for distributed trace
UI applications to connect.

## Turning on tracing

Distributed traces are initiated at the edge by the REST automation system.

To enable distributed trace, please set "tracing=true" for the REST endpoints in the "rest.yaml" file that
you want to trace. For details, please refer to the REST automation application subproject in the "extensions" packages.

## Demo tracer HTML page

To demonstrate how to integrate with this distributed trace aggregator, you can deploy this application.

In a localhost environment, you can visit http://127.0.0.1:8300/trace.html.
In a cloud deployment, the URL will be defined by the cloud administrator.

It will return a sample HTML page that connects to the aggregator's websocket service port and
display the tracing information in real-time.

## UI application

You may implement a UI tracer to connect to the aggregator's websocket service port to collect the tracing information.

Visualization is usually done by ingesting the raw tracing metrics information to an external DevOps tool
such as Graphite or Grafana.

If you want to do your own visualization, you may implement a single page application (React, Angular, etc.)
to render and filter the tracing metrics data.
