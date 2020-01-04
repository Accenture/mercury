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

import org.platformlambda.core.util.Feature;
import org.platformlambda.core.util.SimpleClassScanner;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.web.servlet.ServletContextInitializer;
import org.springframework.stereotype.Component;

import javax.servlet.DispatcherType;
import javax.servlet.FilterRegistration;
import javax.servlet.ServletContext;
import javax.servlet.ServletRegistration;
import javax.servlet.annotation.WebFilter;
import javax.servlet.annotation.WebInitParam;
import javax.servlet.annotation.WebListener;
import javax.servlet.annotation.WebServlet;
import java.util.Arrays;
import java.util.EnumSet;
import java.util.List;
import java.util.Set;

@Component
public class WebAppLoader implements ServletContextInitializer {
    private static final Logger log = LoggerFactory.getLogger(WebAppLoader.class);

    @Override
    public void onStartup(ServletContext context) {

        SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        Set<String> packages = scanner.getPackages(false);
        int totalServlets = 0, totalFilters = 0, totalListeners = 0;
        for (String p : packages) {
            List<Class<?>> endpoints = scanner.getAnnotatedClasses(p, WebServlet.class);
            for (Class<?> cls : endpoints) {
                log.info("Scanning {}", cls);
                WebServlet servlet = cls.getAnnotation(WebServlet.class);
                if (!Feature.isRequired(cls)) {
                    continue;
                }
                ServletRegistration.Dynamic dynamic = context.addServlet(cls.getSimpleName(), cls.getName());
                String[] urls = servlet.value().length == 0? servlet.urlPatterns() : servlet.value();
                if (urls.length > 0) {
                    dynamic.addMapping(urls);
                    dynamic.setAsyncSupported(servlet.asyncSupported());
                    WebInitParam[] params = servlet.initParams();
                    for (WebInitParam kv: params) {
                        dynamic.setInitParameter(kv.name(), kv.value());
                    }
                    if (servlet.loadOnStartup() > 0) {
                        dynamic.setLoadOnStartup(servlet.loadOnStartup());
                    }
                    totalServlets++;
                    log.info("{} registered as WEB SERVLET {}{}{}", cls.getName(), Arrays.asList(urls),
                            servlet.asyncSupported()? " with async support" : "",
                            servlet.loadOnStartup() > 0? ", start up sequence "+servlet.loadOnStartup() : "");
                } else {
                    log.error("WebServlet {} is missing value or urlPatterns", cls.getName());
                }
            }
            endpoints = scanner.getAnnotatedClasses(p, WebFilter.class);
            for (Class<?> cls : endpoints) {
                WebFilter filter = cls.getAnnotation(WebFilter.class);
                if (!Feature.isRequired(cls)) {
                    continue;
                }
                FilterRegistration.Dynamic dynamic = context.addFilter(cls.getSimpleName(), cls.getName());
                String[] urls = filter.value().length == 0? filter.urlPatterns() : filter.value();
                if (urls.length > 0) {
                    List<DispatcherType> dispatcherTypes = Arrays.asList(filter.dispatcherTypes());
                    dynamic.addMappingForUrlPatterns(EnumSet.copyOf(dispatcherTypes), true, urls);
                    totalFilters++;
                    log.info("{} registered as WEB FILTER {}", cls.getName(), Arrays.asList(urls));
                } else {
                    log.error("WebFilter {} is missing value or urlPatterns", cls.getName());
                }
            }
            endpoints = scanner.getAnnotatedClasses(p, WebListener.class);
            for (Class<?> cls : endpoints) {
                if (!Feature.isRequired(cls)) {
                    continue;
                }
                context.addListener(cls.getName());
                totalListeners++;
                log.info("{} registered as WEB LISTENER", cls.getName());
            }
        }
        if (totalServlets > 0) {
            log.info("Total {} WebServlet{} registered", totalServlets, totalServlets == 1 ? "" : "s");
        }
        if (totalFilters > 0) {
            log.info("Total {} WebFilter{} registered", totalFilters, totalFilters == 1 ? "" : "s");
        }
        if (totalListeners > 0) {
            log.info("Total {} WebListener{} registered", totalListeners, totalListeners == 1 ? "" : "s");
        }

    }

}
