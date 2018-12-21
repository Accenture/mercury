/*

    Copyright 2018 Accenture Technology

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

package org.platformlambda.core.system;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.util.SimpleClassScanner;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Set;

public class AppStarter {
    private static final Logger log = LoggerFactory.getLogger(AppStarter.class);

    private static String[] parameters = new String[0];

    public static void main(String[] args) {
        parameters = args;
        AppStarter application = new AppStarter();
        application.begin();
    }

    public void begin() {
        int total = 0;
        SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        Set<String> packages = scanner.getPackages(true);
        for (String p : packages) {
            List<Class<?>> services = scanner.getAnnotatedClasses(p, MainApplication.class);
            for (Class<?> cls : services) {
                try {
                    Object o = cls.newInstance();
                    if (o instanceof EntryPoint) {
                        AppRunner app = new AppRunner((EntryPoint) o);
                        app.start();
                        total++;
                    } else {
                        log.error("Unable to start {} because it is not an instance of {}",
                                cls.getName(), EntryPoint.class.getName());
                    }

                } catch (InstantiationException | IllegalAccessException e) {
                    log.error("Unable to start {} - {}", cls.getName(), e.getMessage());
                }
            }
        }
        if (total == 0) {
            log.error("Main application not found. Remember to annotate it with {} that implements {}",
                    MainApplication.class.getName(), EntryPoint.class.getName());
            System.exit(-1);
        }
    }

    private class AppRunner extends Thread {

        private EntryPoint app;

        public AppRunner(EntryPoint app) {
            this.app = app;
        }

        @Override
        public void run() {
            try {
                log.info("Starting {}", app.getClass().getName());
                app.start(parameters);
            } catch (Exception e) {
                log.error("Unable to run "+app.getClass().getName(), e);
            }
        }
    }

}
