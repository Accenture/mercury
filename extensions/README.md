# This folder contains optional extensions

## REST automation helper application

This is a convenient helper application to let you create REST endpoints without writing code.

It is as simple as a redirection from a URL to a service route of a function. Supports CORS, header transformation and user-defined authentication.

The REST automation helper application is language neutual so you can write your business logic in Java, Python or any Mercury supported language pack without any application server.

## User defined trace processor (upcoming)

The trace processor is provided as an example so you can custom it for your own use cases.

It saves trace information and performance metrics into an in-memory H2 database for testing purpose only.

In production, we would recommend to change the database to PostGreSQL.

You may also write your own trace processor to save the traces to a search engine (e.g. Elastic Search) or 3rd visualization tool.
