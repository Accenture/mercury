# API playground

This is a standalone application that uses swagger UI for the rendering of OpenAPI 3.0 YAML files.

This application is designed as a convenient tool for API design and tests.

## Before you start

1. Clone the latest version from https://github.com/swagger-api/swagger-ui
2. Copy content from the swagger-ui `dist` folder to the "resources/public" folder
3. Perform `mvn clean package` to generate the executable JAR

## OpenAPI specs file folder

The default directory is `resources/sample/yaml`

In this folder, you will find the demo.yaml swagger config file.
This file is a sample for illustration purpose only.

## Sample REST endpoints

To support the demo.yaml config file, two REST endpoints are created in the resources/rest.yaml file.

```text
GET http://127.0.0.1:8200/specs

# this endpoint returns a list of swagger files
# under the resources/sample/yaml folder.

GET http://127.0.0.1:8200/specs/demo.yaml

# this endpoint returns content of the demo.yaml file.
```

The two endpoints are served by their corresponding functions in the package under com.accenture.examples.services.

## Running this application

To run this application:
```
java -jar api-playground-3.0.17.jar
```

You will see it starting a Reactive HTTP server at port 8222.
Then it will run as a Spring Boot app using port-8200.

The port 8222 illustrates that you can use swagger-ui to connect to another host.

Please visit http://127.0.0.1:8200 to see the swagger-ui home page.

Then enter "http://127.0.0.1:8200/yaml/demo.yaml" to load the sample swagger demo.yaml file.

## Loading your own swagger files

You can copy them into the resources/sample/yaml folder and rebuild this app.

Alternatively, you can externalize the swagger folder in the local file system.

You would need to update application.properties and replace "classpath:/sample" with
"file:/your/local/folder":

```properties
spring.web.resources.static-locations=classpath:/public/,classpath:/sample/
```

Note that you must keep the "classpath:/public/" in the static-locations parameter above.

This allows the app to serve the swagger-ui from classpath:/public/ and
your own swagger files from file:/your/local/folder.

## Acknowledgements

This application use the following open sources:
1. Accenture mercury under Apache 2.0 license - https://github.com/Accenture/mercury/blob/master/LICENSE
2. Swagger UI under Apache 2.0 license - https://swagger.io/license/
3. Jquery under MIT license - https://jquery.org/license/
4. Bootstrap under MIT license - https://github.com/twbs/bootstrap/blob/master/LICENSE
