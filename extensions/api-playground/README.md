# API playground

This is a standalone application that uses swagger UI for the rendering of OpenAPI 2.0 and 3.0 specification files.

This application is designed as a convenient tool for API design, tests and discussion among developers and product owners.

## OpenAPI specs file folder

The default directory is `/tmp/api-playground`

You can change the directory location by overriding the "api.playground.apps" parameter when starting up this application.

```
java -Dapi.playground.apps=another_location -jar api-playground-1.12.16.jar

where another location is a folder in the local file system.
```

When you click the "API playground" in the top navigation bar, it will reload the application dropdown box with the available files in the API specs folder.


## Running this application

Please visit http://127.0.0.1:8083 after starting this application.

Using default API specs location,
```
java -jar api-playground-1.12.16.jar
```

With user defined API specs location,
```
java -Dapi.playground.apps=another_location -jar api-playground-1.12.16.jar
```

## How this application is created?

1. This application is created using the rest-example as a template.
2. Clone https://github.com/swagger-api/swagger-ui and copy content in the "dist" folder to the "public" folder under resources.
3. Download bootstrap 4.4.1 and Jquery 3.4.1 and save them under bootstrap and jquery folders in the "public" folder under resources.
4. Write a simple "HomePage" WebServlet to override the "index.html" page.
5. Create a sample home.yaml OpenAPI file and deposit it under the "playground" folder in the "public" folder under resources.
6. Write a PlaygroundApi JAX-RS endpoint to serve a sample home.yaml OpenAPI file.

## Acknowledgements

Parts of this application use the following open sources:
1. Accenture mercury under Apache 2.0 license - https://github.com/Accenture/mercury/blob/master/LICENSE
2. Swagger UI under Apache 2.0 license - https://swagger.io/license/
3. Jquery under MIT license - https://jquery.org/license/
4. Bootstrap under MIT license - https://github.com/twbs/bootstrap/blob/master/LICENSE
