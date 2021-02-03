# pm2 example start-up files

Since relative path is used in these example pm2 "process" files, you must "cd" to this directory before running "pm2 start file_name".

# start an application

To start an application, do "pm2 start application_name.json"

# stop an application

do "pm2 stop application_name"

# list all running application

do "pm2 list"

# Event node, Hazelcast and Kafka

They are mutually exclusive. The hazelcast and kafka process files are here as an example only.

# Starting multiple applications

Once the applications are built, you can start them in this sequence. Kafka and presence-monitor are pre-requisites for the user applications. The language-connector is a helper app or sidecar for non-Java applications. e.g. Python.

```bash
pm2 start kafka.json
pm2 start presence-monitor.json
pm2 start rest-automation.json
pm2 start rest-example.json
pm2 start lambda-example.json
pm2 start language-connector.json
```

# application log

You may tail the log with 

```
pm2 log application_name
```

