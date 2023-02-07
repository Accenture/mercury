# Benchmark server

This server application must be run before executing the benchmark client tests

# Pre-requisites

The benchmark client and server communicate over the Kafka event stream system.

You may start a standalone kafka server and a presence monitor using the "pm2-examples/kafka" folder.

Start them as follows:

```agsl
pm2 start kafka.json
pm2 start presence-monitor.json
```

Note that "pm2" requires node.js. Please refer to https://www.npmjs.com/package/pm2 for instruction to install pm2.

If you prefer to start kafka and presence monitor manually. You may go to their respective sub-project folders
and start the application using a command line terminal.

# Running the benchmark server

do this

```agsl
mvn clean package
java -jar target/benchmark-server-2.8.0.jar
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

Now you can run the benchmark client tests. Please refer to the benchmark-client sub-project's README file.
