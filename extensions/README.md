# This folder contains optional extensions

## REST automation

This is a recommended way to create and manage your REST and websocket endpoints without writing code.

It is as simple as a redirection from a URL to a service route of a function. Supports CORS, header transformation 
and user-defined authentication.

The REST automation application is language neutual so you can write your business logic in Java, Python or any Mercury 
supported language pack without any application server.

## Library and Application

The REST automation system consists of a library and an application.

The REST automation application is a standalone application that you can use it as a simple REST to event automation
gateway.

In some cases that you want to have the same REST automation feature embedded in another standalone application or 
service module, you may add the REST automation library to your application's pom.xml file.

## IMPORTANT

The REST automation application sub-project contains a demo websocket HTML page (`ws.html`) in the resources/public 
folder and the rest.yaml config file in the resources folder has rest entries to support the websocket demo.

They are provided as a demo and a template. Please remove or adjust them before you deploy the REST automation system 
for production.
