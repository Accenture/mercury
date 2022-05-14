# API playground

This is a standalone application that uses swagger UI for the rendering of OpenAPI 2.0 and 3.0 specification files.

This application is designed as a convenient tool for API design, tests and discussion among developers and 
product owners.

You can drop your OpenAPI 3.0 JSON or YAML files in the api-playground folder so that you can select anyone
of them to render the documentation.

## Before you start

1. Clone https://github.com/swagger-api/swagger-ui and copy content in the "dist" folder to the "swagger-ui" folder
under resources
2. Remove all *.js.map files from the "swagger-ui" folder
3. Perform `mvn clean package` to generate the executable JAR

## OpenAPI specs file folder

The default directory is `/tmp/api-playground`

To change this file location, you can do:
1. update application.properties and rebuild the application, or
2. change the directory location by overriding the "api.playground.apps" parameter when starting up this 
application.

```
java -Dapi.playground.apps=another_location -jar api-playground-2.3.3.jar

where another location is a folder in the local file system.
```

When you click the "API playground" in the top navigation bar, it will reload the application dropdown box with the 
available files in the API specs folder.


## Running this application

Please visit http://127.0.0.1:8200 after starting this application.

Using default API specs location,
```
java -jar api-playground-2.3.3.jar
```

With user defined API specs location,
```
java -Dapi.playground.apps=another_location -jar api-playground-2.3.3.jar
```

## How this application is created?

1. This application is created using "mercury/examples/rest-example" as a template
2. Clone https://github.com/swagger-api/swagger-ui and copy content in the "dist" folder to the "swagger-ui" folder 
   under resources
3. Remove all *.js.map files from the "swagger-ui" folder
4. Download bootstrap 4.4.1 and Jquery 3.4.1 and save them under bootstrap and jquery folders in the "public" 
   folder under resources
5. A custom "HomePage" WebServlet for user to select a OpenAPI 3.0 JSON/YAML file to override the "index.html" page
6. A sample home.yaml OpenAPI file in the "public/playground" folder
7. A playgroundApi JAX-RS endpoint to render the home page

## Acknowledgements

This application use the following open sources:
1. Accenture mercury under Apache 2.0 license - https://github.com/Accenture/mercury/blob/master/LICENSE
2. Swagger UI under Apache 2.0 license - https://swagger.io/license/
3. Jquery under MIT license - https://jquery.org/license/
4. Bootstrap under MIT license - https://github.com/twbs/bootstrap/blob/master/LICENSE
