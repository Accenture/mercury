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

package org.platformlambda.core.system;

import org.platformlambda.core.annotations.BeforeApplication;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Feature;
import org.platformlambda.core.util.SimpleClassScanner;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class AppStarter {
    private static final Logger log = LoggerFactory.getLogger(AppStarter.class);

    private static final int MAX_SEQ = 999;
    private static boolean preProcessing = false;

    public static void main(String[] args) {
        log.info("Starting main applications");
        AppStarter application = new AppStarter();
        application.begin(args);
    }

    public static void prepare(String[] args) {
        // find and execute optional preparation modules
        if (!preProcessing) {
            preProcessing = true;
            Utility util = Utility.getInstance();
            SimpleClassScanner scanner = SimpleClassScanner.getInstance();
            Set<String> packages = scanner.getPackages(true);
            for (String p : packages) {
                // sort loading sequence
                int n = 0;
                Map<String, Class<?>> steps = new HashMap<>();
                List<Class<?>> services = scanner.getAnnotatedClasses(p, BeforeApplication.class);
                for (Class<?> cls : services) {
                    if (Feature.isRequired(cls)) {
                        n++;
                        BeforeApplication before = cls.getAnnotation(BeforeApplication.class);
                        int seq = Math.min(MAX_SEQ, before.sequence());
                        String key = util.zeroFill(seq, MAX_SEQ) + "." + util.zeroFill(n, MAX_SEQ);
                        steps.put(key, cls);
                    } else {
                        log.debug("Skipping optional startup {}", cls);
                    }
                }
                List<String> list = new ArrayList<>(steps.keySet());
                if (list.size() > 1) {
                    Collections.sort(list);
                }
                for (String seq : list) {
                    Class<?> cls = steps.get(seq);
                    try {
                        Object o = cls.newInstance();
                        if (o instanceof EntryPoint) {
                            /*
                             * execute preparation logic as a blocking operation
                             * (e.g. setting up environment variable, override application.properties, etc.)
                             */
                            EntryPoint app = (EntryPoint) o;
                            try {
                                log.info("Starting {}", app.getClass().getName());
                                app.start(args);
                            } catch (Exception e) {
                                log.error("Unable to run " + app.getClass().getName(), e);
                            }
                        } else {
                            log.error("Unable to start {} because it is not an instance of {}",
                                    cls.getName(), EntryPoint.class.getName());
                        }

                    } catch (InstantiationException | IllegalAccessException e) {
                        log.error("Unable to start {} - {}", cls.getName(), e.getMessage());
                    }
                }
            }
            /*
             * In case values in application.properties are updated by the BeforeApplication modules,
             * the AppConfigReader must reload itself.
             */
            AppConfigReader.getInstance().reload();
        }
    }

    public void begin(String[] args) {
        // preparation step is executed only once
        AppStarter.prepare(args);
        // find and execute MainApplication modules
        int total = 0, skipped = 0;
        SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        Set<String> packages = scanner.getPackages(true);
        for (String p : packages) {
            List<Class<?>> services = scanner.getAnnotatedClasses(p, MainApplication.class);
            for (Class<?> cls : services) {
                if (Feature.isRequired(cls)) {
                    try {
                        Object o = cls.newInstance();
                        if (o instanceof EntryPoint) {
                            // execute MainApplication module in a separate thread for non-blocking operation
                            AppRunner app = new AppRunner((EntryPoint) o, args);
                            app.start();
                            total++;
                        } else {
                            log.error("Unable to start {} because it is not an instance of {}",
                                    cls.getName(), EntryPoint.class.getName());
                        }

                    } catch (InstantiationException | IllegalAccessException e) {
                        log.error("Unable to start {} - {}", cls.getName(), e.getMessage());
                    }
                } else {
                    skipped++;
                    log.info("Skipping optional main {}", cls);
                }
            }
        }
        if (total == 0) {
            if (skipped == 0) {
                log.error("MainApplication not found. Remember to annotate it with {} that implements {}",
                        MainApplication.class.getName(), EntryPoint.class.getName());
            } else {
                log.error("Please enable at least one MainApplication module and try again");
            }
            System.exit(-1);
        }
    }

    private class AppRunner extends Thread {

        private EntryPoint app;
        private String[] args;

        public AppRunner(EntryPoint app, String[] args) {
            this.app = app;
            this.args = args;
        }

        @Override
        public void run() {
            try {
                log.info("Starting {}", app.getClass().getName());
                app.start(args);
            } catch (Exception e) {
                log.error("Unable to run "+app.getClass().getName(), e);
            }
        }
    }

}
