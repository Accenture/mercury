# This folder contains optional extensions

## REST automation helper application

This is a convenient helper application to let you create REST endpoints without writing code.

It is as simple as a redirection from a URL to a service route of a function. Supports CORS, header transformation and user-defined authentication.

The REST automation helper application is language neutual so you can write your business logic in Java, Python or any Mercury supported language pack without any application server.

## WebSocket notificaton use case and sample application

WebSocket is usually employed as a notification channel to the browser so that
your service can detect "presence" of the user and asynchronously send
notification events to the browser.

The REST automation helper application supports this websocket notification use case. The sample rest.yaml configuration file contains a websocket routing entry
to the sample.ws.auth and ws.notification services.

The sample.ws.auth is the authentication service so that your backend application
can validate if the incoming websocket is associated with an authenticated user
session.

The ws.notification is the websocket service that receives incoming requests and save a mapping of the user's websocket outgoing paths that your backend services can send notification to.

This supports multi-device user sessions. When a user makes a request to change something, the backend services can send notification events to all connected devices of the same user.
