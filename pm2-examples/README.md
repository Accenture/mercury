# Process manager example

Mercury application modules are usually deployed using Docker/Kubernetes.

However, some developer's laptops may require admin privileges to install Docker and Kubernetes.

In this case, you may consider using a Process Manager such as "pm2" of the Node ecosystem.
pm2 can deploy Java and Python applications in addition to Node.js ones.


## PM2 command

To start an application, use `pm2 start {app-name}.json`.

To log an application, use `pm2 start {app-name}`. Note that you have to remove the ".json" suffix from the command.

To stop and application, use `pm2 start {app-name}.json`.

To remove an application from pm2, stop it and enter `pm2 delete {app-name}.json`.
