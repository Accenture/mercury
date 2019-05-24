# Kafka standalone server

This is a convenient application to run Kafka as a standalone server. It will start zookeeper and kafka orderly. It uses the "/tmp" directory to store working files. This Kafka standalone server is designed to simplify software development and testing and should not be used for production purpose.

Note that when you restart the Kafka standalone server, all topics will be deleted. This is intentional because the kafka standalone server is designed for dev and testing only.

## Limitation of the Kafka standalone server in Windows environment

The Kafka standalone server runs in Mac and Linux environments. When running in Windows environment, it will crash when the presence monitor tries to delete an expired topic.

If you really need to run the Kafka standalone server in a Windows laptop, please run the kafka presence monitor with `delete.topic.enable` set to `false`.  e.g.

```
java -Ddelete.topic.enable=false -jar target\kafka-presence-1.11.40.jar
```
If delete.topic.enable is set to false, the presence monitor's housekeeper will not delete expired topics.
