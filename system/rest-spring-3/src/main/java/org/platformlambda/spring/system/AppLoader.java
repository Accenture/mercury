/*

    Copyright 2018-2024 Accenture Technology

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

package org.platformlambda.spring.system;

import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.context.event.ApplicationFailedEvent;
import org.springframework.boot.context.event.ApplicationReadyEvent;
import org.springframework.boot.web.servlet.context.ServletWebServerInitializedEvent;
import org.springframework.context.event.ContextClosedEvent;
import org.springframework.context.event.EventListener;
import org.springframework.stereotype.Component;

import java.util.Set;

@Component
public class AppLoader {
    private static final Logger log = LoggerFactory.getLogger(AppLoader.class);

    private static final long BEGIN = System.currentTimeMillis();

    @EventListener
    public void handleEvent(Object event) {
        if (event instanceof ApplicationReadyEvent ready) {
            /*
             * this event will happen in both WAR and JAR deployment mode
             * At this point, Spring Boot is ready
             */
            Set<String> profiles = ready.getSpringApplication().getAdditionalProfiles();
            if (profiles.isEmpty()) {
                log.info("Loading user application");
            } else {
                log.info("Loading user application with {}", profiles);
            }
            AppStarter.runMainApp();
        }
        if (event instanceof ServletWebServerInitializedEvent init) {
            log.info("Spring Boot is using port-{}", init.getWebServer().getPort());
        }
        if (event instanceof ContextClosedEvent close) {
            String elapsed = Utility.getInstance().elapsedTime(close.getTimestamp() - BEGIN);
            log.info("Spring Boot stopped. It has run for {}.", elapsed);
        }
        // in case Spring Boot fails, it does not make sense to keep the rest of the application running.
        if (event instanceof ApplicationFailedEvent fail) {
            log.error("{}", fail.getException().getMessage());
            System.exit(-1);
        }
    }

}
