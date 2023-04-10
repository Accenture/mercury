# Kafka standalone server

This is provided as a convenient application to run Kafka as a standalone server. It will start zookeeper and kafka orderly. 
It uses the "/tmp" directory to store working files. This Kafka standalone server is designed to simplify 
software development and testing and should not be used for production purpose.

Note that when you restart the Kafka standalone server, all topics will be deleted. This is intentional 
because the kafka standalone server is designed for dev and testing only.

Please note that this tool is not designed for production use.

## Using docker

If you are using Windows machine, the best way to run this kafka standalone server is to dockerize it. Please make sure your base image uses Java version 11 or 16.

```
docker build -t kafka-standalone .
docker run -p 9092:9092 -p 2181:2181 kafka-standalone
```

After this step, you can start/stop it from the Docker Desktop app.

## IMPORTANT - Known problem as of September 2021

Please use Java version 11 or 16 to run this standalone Kafka server application.

The FileChannelImplementation class would throw "Map Failed" error when running in Java version 1.8.0_292 or version 15.
This would result in system failure. The kafka client will go into an endless loop when it retries updating
metadata.

Please perform a test run with the following setup to validate that it is compatible with your development environment.
1. this standalone kafka server
2. presence monitor
3. rest-spring-example
4. lambda-example

If all 4 apps start up normally, your JVM is compatible with this standalone kafka server.

# kafka connector compatibility

The kafka-connector client library for Mercury powered applications has been tested for compatibility with Java 1.8 to Java 16.
