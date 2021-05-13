# Kafka standalone server

This is a convenient application to run Kafka as a standalone server. It will start zookeeper and kafka orderly. 
It uses the "/tmp" directory to store working files. This Kafka standalone server is designed to simplify 
software development and testing and should not be used for production purpose.

Note that when you restart the Kafka standalone server, all topics will be deleted. This is intentional 
because the kafka standalone server is designed for dev and testing only.

## Using docker

If you are using Windows machine and you have "Docker for Windows" installed, the best way to run this 
kafka standalone server is to dockerize it. Please make sure your base image uses Java version 11 or higher.

```
docker build -t kafka-standalone .
docker run -p 9092:9092 -p 2181:2181 kafka-standalone
```

After this step, you can start/stop it from the Docker Desktop app.

## Known problem as of May 2021

The FileChannelImplementation class would throw "Map Failed" error when running in Java version 1.8.0_292.
This would result in system failure. The kafka client will go into an endless loop when it retries updating
metadata.

# IMPORTANT

Please use Java version 11 or higher to run this standalone Kafka server application.
