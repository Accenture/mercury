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

import javax.servlet.ServletContext;
import javax.servlet.ServletContextEvent;
import javax.servlet.ServletContextListener;
import javax.servlet.annotation.WebListener;
import javax.websocket.DeploymentException;
import javax.websocket.server.ServerContainer;
import javax.websocket.server.ServerEndpoint;
import java.util.Arrays;
import java.util.List;
import java.util.Set;

@WebListener
public class WebsocketLoader implements ServletContextListener {
    private static final Logger log = LoggerFactory.getLogger(WebsocketLoader.class);

    @Override
    public void contextInitialized(ServletContextEvent event) {
        /*
         * Scan for annotated websocket endpoints (ServerEndpoint.class)
         */
        ServletContext context = event.getServletContext();
        SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        Set<String> packages = scanner.getPackages(true);

        // At this point, the underlying ServerContainer should be loaded
        Object sc = context.getAttribute("javax.websocket.server.ServerContainer");
        if (sc instanceof ServerContainer) {
            ServerContainer container = (ServerContainer) sc;
            int total = 0;
            for (String p : packages) {
                List<Class<?>> endpoints = scanner.getAnnotatedClasses(p, ServerEndpoint.class);
                for (Class<?> cls : endpoints) {
                    if (!Feature.isRequired(cls)) {
                        continue;
                    }
                    try {
                        container.addEndpoint(cls);
                        ServerEndpoint ep = cls.getAnnotation(ServerEndpoint.class);
                        total++;
                        log.info("{} registered as WEBSOCKET DISPATCHER {}", cls.getName(), Arrays.asList(ep.value()));
                    } catch (DeploymentException e) {
                        log.error("Unable to deploy websocket endpoint {} - {}", cls, e.getMessage());
                    }
                }
            }
            if (total > 0) {
                log.info("Total {} Websocket server endpoint{} registered (JSR-356)", total, total == 1 ? " is" : "s are");
            }
        } else {
            log.error("Unable to register any ServerEndpoints because javax.websocket.server.ServerContainer is not available");
        }
    }

    @Override
    public void contextDestroyed(ServletContextEvent event) {
        log.info("Stopped");
    }

}
