package org.platformlambda.rest.spring.system;

import org.jboss.resteasy.plugins.server.servlet.HttpServlet30Dispatcher;
import org.jboss.resteasy.plugins.server.servlet.ResteasyContextParameters;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Feature;
import org.platformlambda.core.util.SimpleClassScanner;
import org.platformlambda.core.util.Utility;
import org.springframework.boot.web.servlet.ServletContextInitializer;
import org.springframework.stereotype.Component;

import javax.servlet.ServletContext;
import javax.servlet.ServletException;
import javax.servlet.ServletRegistration;
import javax.ws.rs.Path;
import javax.ws.rs.ext.Provider;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashSet;
import java.util.List;
import java.util.Set;


@Component
public class RestLoader implements ServletContextInitializer {
    private static final Logger log = LoggerFactory.getLogger(RestLoader.class);

    private static final String BASE_URL = "jax.rs.application.path";
    private static final String DEFAULT_API_PATH = "/api";
    private static final String RESTEASY_MAPPING_PREFIX = "resteasy.servlet.mapping.prefix";
    private static final String RESTEASY_DISABLE_HTML_ESCAPE = "resteasy.disable.html.sanitizer";
    private static boolean loaded = false;

    @Override
    public void onStartup(ServletContext servletContext) throws ServletException {
        if (!loaded) {
            // guarantee to do once
            loaded = true;
            AppConfigReader config = AppConfigReader.getInstance();
            String apiPrefix = normalizeUrlPrefix(config.getProperty(BASE_URL, DEFAULT_API_PATH));
            log.info("{} = {}", BASE_URL, apiPrefix);
            Set<Class<?>> providers = new HashSet<>();
            Set<Class<?>> resources = new HashSet<>();
            SimpleClassScanner scanner = SimpleClassScanner.getInstance();
            Set<String> packages = scanner.getPackages(true);
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
                    resources.add(cls);
                    restCount++;
                    log.info("{} registered as REST", cls.getName());
                }
            }
            if (restCount == 0) {
                log.info("No REST endpoints are found");
                return;
            }
            log.info("Total {} REST class{} registered", restCount, restCount == 1 ? "" : "es");
            /*
             * register JAX-RS "provider" modules (serializers and web filters)
             */
            int providerCount = 0;
            for (String p : packages) {
                List<Class<?>> endpoints = scanner.getAnnotatedClasses(p, Provider.class);
                for (Class<?> cls : endpoints) {
                    if (!Feature.isRequired(cls)) {
                        continue;
                    }
                    providers.add(cls);
                    providerCount++;
                    log.info("{} registered as provider", cls.getName());
                }
            }
            if (providerCount > 0) {
                log.info("Total {} provider{} registered", providerCount, providerCount == 1 ? "" : "s");
            }
            String clsName = this.getClass().getName();
            ServletRegistration.Dynamic reg = servletContext.addServlet(clsName, HttpServlet30Dispatcher.class);
            reg.setLoadOnStartup(1);
            reg.setAsyncSupported(true);
            // 1. DO NOT set "javax.ws.rs.Application"
            // 2. Must disable HTML escape to support HTML and XML output
            reg.setInitParameter(RESTEASY_DISABLE_HTML_ESCAPE, String.valueOf(true));
            // 3. Set context path mapping
            reg.setInitParameter(RESTEASY_MAPPING_PREFIX, apiPrefix);
            reg.addMapping(apiPrefix+"/*");
            // 4. Tell RestEasy to scan REST endpoints
            reg.setInitParameter(ResteasyContextParameters.RESTEASY_SCANNED_RESOURCES, getClassList(resources));
            // 5. Tell RestEasy to scan providers
            if (!providers.isEmpty()) {
                reg.setInitParameter(ResteasyContextParameters.RESTEASY_SCANNED_PROVIDERS, getClassList(providers));
            }
        }
    }

    private String getClassList(Set<Class<?>> classes) {
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        for (Class<?> cls : classes) {
            if (first) {
                first = false;
            } else {
                sb.append(",");
            }
            sb.append(cls.getName());
        }
        return sb.toString();
    }

    private String normalizeUrlPrefix(String path) {
        List<String> parts = Utility.getInstance().split(path, "/");
        if (parts.isEmpty()) {
            return DEFAULT_API_PATH;
        }
        StringBuilder sb = new StringBuilder();
        for (String p: parts) {
            sb.append('/');
            sb.append(p.toLowerCase());
        }
        return sb.toString();
    }

}
