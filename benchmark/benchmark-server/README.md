# Benchmark server

This server application must be run before executing the benchmark client tests

# Pre-requisites

The benchmark client and server communicate over the Kafka event stream system.

You may start a standalone kafka server and a presence monitor like this:

From one command terminal

```text
cd sandbox/connector/adapters/kafka/kafka-standalone
java -jar target/kafka-standalone-3.0.5.jar
```

From another command terminal

```text
cd sandbox/connector/adapters/kafka/kafka-presence
java -jar target/kafka-presence-3.0.5.jar
```

# Running the benchmark server

do this

```text
mvn clean package
java -jar target/benchmark-server-3.0.5.jar
```

Once the server is up and running, you will see it connects to Kafka and reports to the presence monitor.

You can verify if the server is running by visiting the INFO page of the presence monitor.

http://127.0.0.1:8080/info

It will show something like this:

```json
{
  "connections": [
    {
      "elapsed": "33 seconds",
      "created": "2022-12-04T01:55:21Z",
      "origin": "202212045e033c057c5e4d57a3d8982510914a10",
      "name": "benchmark-server",
      "topic": "multiplex.0001-000",
      "monitor": "202212049f398a700479433d902c35ecf10eedb9",
      "type": "APP",
      "updated": "2022-12-04T01:55:54Z",
      "version": "2.7.0",
      "seq": 9,
      "group": 1
    }
  ]
}
```

Now you can run the benchmark client tests. Please refer to the benchmark-client subproject's README file.
