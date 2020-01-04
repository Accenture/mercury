/*

    Copyright 2018-2020 Accenture Technology

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

 */

package org.platformlambda.rest.spring.system;

import org.glassfish.jersey.server.ResourceConfig;
import org.glassfish.jersey.servlet.ServletContainer;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Feature;
import org.platformlambda.core.util.SimpleClassScanner;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.web.servlet.FilterRegistrationBean;
import org.springframework.context.annotation.Bean;
import org.springframework.stereotype.Component;

import javax.servlet.DispatcherType;
import javax.ws.rs.Path;
import javax.ws.rs.ext.Provider;
import java.util.*;

@Component
public class RestLoader extends ResourceConfig {
    private static final Logger log = LoggerFactory.getLogger(RestLoader.class);

    private static final String BASE_URL = "spring.jersey.application-path";
    private static boolean loaded = false;

    public RestLoader() {
        if (!loaded) {
            log.info("Setting JAX-RS environment");
            // guarantee to do once
            loaded = true;
            SimpleClassScanner scanner = SimpleClassScanner.getInstance();
            /*
             * register JAX-RS "provider" modules (serializers and web filters)
             */
            int providerCount = 0;
            Set<String> packages = scanner.getPackages(true);
            for (String p : packages) {
                List<Class<?>> endpoints = scanner.getAnnotatedClasses(p, Provider.class);
                for (Class<?> cls : endpoints) {
                    if (!Feature.isRequired(cls)) {
                        continue;
                    }
                    register(cls);
                    providerCount++;
                    log.info("{} registered as provider", cls.getName());
                }
            }
            if (providerCount > 0) {
                log.info("Total {} provider{} registered", providerCount, providerCount == 1 ? "" : "s");
            }
            /*
             * register JAX-RS REST endpoints
             */
            int restCount = 0;
            for (String p : packages) {
                List<Class<?>> endpoints = scanner.getAnnotatedClasses(p, Path.class);
                for (Class<?> cls : endpoints) {
                    if (!Feature.isRequired(cls)) {
                        continue;
                    }
                    register(cls);
                    restCount++;
                    log.info("{} registered as REST", cls.getName());
                }
            }
            if (restCount > 0) {
                log.info("Total {} REST class{} registered", restCount, restCount == 1 ? "" : "es");
            }

        }
    }

    @Bean
    public FilterRegistrationBean<ServletContainer> jerseyRegistration() {
        AppConfigReader config = AppConfigReader.getInstance();
        String url = config.getProperty(BASE_URL, "/api");
        FilterRegistrationBean<ServletContainer> registration = new FilterRegistrationBean<>();
        registration.setFilter(new ServletContainer(this));
        registration.setUrlPatterns(Collections.singletonList(normalizeUrl(url)+"/*"));
        registration.setAsyncSupported(true);
        registration.setOrder(1);
        registration.setName("jaxRsFilter");
        registration.setDispatcherTypes(EnumSet.allOf(DispatcherType.class));
        return registration;
    }

    private String normalizeUrl(String url) {
        List<String> parts = Utility.getInstance().split(url, "/* ");
        if (parts.isEmpty()) {
            return "/api";
        }
        StringBuilder sb = new StringBuilder();
        for (String p: parts) {
            sb.append("/");
            sb.append(p);
        }
        return sb.toString();
    }

}
