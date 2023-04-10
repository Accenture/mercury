# pm2 example start-up files

Since relative path is used in these example pm2 "process" files, you must "cd" to this directory before
running "pm2 start file_name".

# start an application

To start an application, do "pm2 start application_name.json"

# stop an application

do "pm2 stop application_name"

# list all running application

do "pm2 list"

# Starting multiple applications

Once the applications are built, you can start them in this sequence. Kafka and presence-monitor form
the service mesh for the user applications.

```bash
pm2 start kafka.json
pm2 start presence-monitor.json
pm2 start rest-spring-example.json
pm2 start lambda-example.json
```

# application log

You may tail the log like this:

```
pm2 log application_name
```

