# Cloud connectors

Mercury has been integrated and tested with both event stream systems and enterprise service bus messaging system.

The following connectors are provided as sample.

1. Apache Kafka
2. Hazelcast

## Minimalist service mesh

To create a minimalist service mesh, you would need to deploy a "presence monitor" as the "control plane" and
add a cloud connector library to your application.

Each application instance will report to the "presence monitor" automatically.

The minimalist service mesh is optional.

To support inter-container communication without a service mesh, you can use the "Event over HTTP" approach.
Please refer to the Developer Guide for details.
