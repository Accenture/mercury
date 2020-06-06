# Simple Scheduler

Simple scheduler that uses YAML configuration for storing "cron" job schedules

## Cloud native deployment

More than one instance of this application can be deployed.
When this happens, the multiple instances will run in parallel.

To avoid duplicated job scheduling, the multiple instances will detect the presence of each other
and elect an leader to execute the scheduled jobs.

It does not need a database to do clustering. The multiple instances will sync up its job status.

## cron schedules

Cron jobs can be defined in a "cron.yaml" file.

You can change the file path in application.properties:
```
#
# path to cron.yaml
#
cron.yaml=file:/tmp/config/cron.yaml, classpath:/cron.yaml
```
In this example, it assumes the configuration file is available in "/tmp/config/cron.yaml".
If it is not there, it will use the default one in the classpath.

Note the the one in the classpath is just an example. Please update the YAML configuration accordingly.

## Admin endpoints

There are a few admin endpoints
```
GET /api/jobs
This retrieves a list of scheduled jobs

DELETE /api/jobs/{name}
This stop a job if it has been started

PUT /api/jobs/{name}
This start a job if it has been stopped

POST /api/jobs/{name}
This execute a job immediately

```

## Sample cron.yaml

```
jobs:
  # you can cancel a job from an admin endpoint and restart it anytime
  - name: "demo"
    description: "execute demo service every minute"
    cron: "0 0/1 * 1/1 * ? *"
    service: "hello.world"
    # optional parameter to tell the service what to do
    parameters:
      hello: "world"
```
In this example, there is one scheduled job called "demo" and it will execute the service "hello.world" every minute.

## Sample job listing

```
GET http://127.0.0.1:8083/api/jobs

{
  "total": 1,
  "jobs": [
    {
      "start_time": "2020-06-05T22:14:11.021Z",
      "last_execution": "2020-06-05T22:15:00.012Z",
      "cron_schedule": "0 0/1 * 1/1 * ? *",
      "service": "hello.world",
      "created": "2020-06-05T22:14:10.928Z",
      "name": "demo",
      "description": "execute demo service every minute",
      "parameters": {
        "hello": "world"
      },
      "iterations": 1
    }
  ],
  "time": "2020-06-05T22:15:05.970Z"
}
```