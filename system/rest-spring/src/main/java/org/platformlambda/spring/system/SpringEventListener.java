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

package org.platformlambda.spring.system;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.util.Feature;
import org.platformlambda.core.util.SimpleClassScanner;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.context.event.ApplicationFailedEvent;
import org.springframework.boot.context.event.ApplicationReadyEvent;
import org.springframework.boot.web.servlet.context.ServletWebServerInitializedEvent;
import org.springframework.context.event.EventListener;
import org.springframework.stereotype.Component;

import java.lang.reflect.InvocationTargetException;
import java.util.*;

@Component
public class SpringEventListener {
    private static final Logger log = LoggerFactory.getLogger(SpringEventListener.class);

    private static final int MAX_SEQ = 999;
    private static boolean started = false;

    @EventListener
    public void handleEvent(Object event) {
        // do optional life-cycle management
        if (event instanceof ServletWebServerInitializedEvent) {
            // when deploy as WAR, this event will not happen
            log.debug("Loading Spring Boot");
        }
        if (event instanceof ApplicationReadyEvent) {
            /*
             * this event will happen in both WAR and JAR deployment mode
             * At this point, Spring Boot is ready
             */
            if (!started) {
                new MainApps().start();
            }
        }
        // in case Spring Boot fails, it does not make sense to keep the rest of the application running.
        if (event instanceof ApplicationFailedEvent) {
            log.error("{}", ((ApplicationFailedEvent) event).getException().getMessage());
            System.exit(-1);
        }
    }

    private class MainApps extends Thread {

        @Override
        public void run() {
            if (!started) {
                started = true;
                Utility util = Utility.getInstance();
                SimpleClassScanner scanner = SimpleClassScanner.getInstance();
                Set<String> packages = scanner.getPackages(true);
                int n = 0;
                Map<String, Class<?>> steps = new HashMap<>();
                for (String p : packages) {
                    List<Class<?>> services = scanner.getAnnotatedClasses(p, MainApplication.class);
                    for (Class<?> cls : services) {
                        if (Feature.isRequired(cls)) {
                            MainApplication before = cls.getAnnotation(MainApplication.class);
                            int seq = Math.min(MAX_SEQ, before.sequence());
                            String key = util.zeroFill(seq, MAX_SEQ) + "." + util.zeroFill(++n, MAX_SEQ);
                            steps.put(key, cls);
                        } else {
                            log.info("Skipping optional {}", cls);
                        }
                    }
                }
                executeOrderly(steps);
            }
        }

        private void executeOrderly(Map<String, Class<?>> steps) {
            List<String> list = new ArrayList<>(steps.keySet());
            if (list.size() > 1) {
                Collections.sort(list);
            }
            int n = 0, error = 0;
            for (String seq : list) {
                Class<?> cls = steps.get(seq);
                try {
                    Object o = cls.getDeclaredConstructor().newInstance();
                    if (o instanceof EntryPoint) {
                        EntryPoint app = (EntryPoint) o;
                        log.info("Starting {}", app.getClass().getName());
                        app.start(AppStarter.getArgs());
                        n++;
                    } else {
                        error++;
                        log.error("Unable to start {} because it is not an instance of {}",
                                cls.getName(), EntryPoint.class.getName());
                    }
                } catch (Exception e) {
                    error++;
                    log.error("Unable to start {} - {}", cls.getName(), e.getMessage());
                }
            }
            if (error == 0 && n == 0) {
                log.error("Missing MainApplication\n\n{}\n{}\n\n",
                    "Did you forget to annotate your main module with @MainApplication that extends EntryPoint?",
                    "and ensure the package parent is defined in 'web.component.scan' of application.properties.");
            }
        }
    }

}
