# The following patch has been applied

# SnakeYaml 2.0 compatibility with Spring Boot 2.7.10

### Patch

Added the OriginTrackedYamlLoader and YamlPropertySourceLoader under the package

```
org.platformlambda.spring.patch
```

The two Java source files are originated from Spring Boot 2.7.10

The createYaml(LoaderOptions) method has been patched to update as follows:

```
Representer representer = new Representer(dumperOptions);
```

### Additional configuration file

resources/META-INF/spring.factories
