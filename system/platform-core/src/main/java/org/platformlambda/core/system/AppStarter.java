/*

    Copyright 2018-2021 Accenture Technology

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
import org.platformlambda.core.util.Feature;
import org.platformlambda.core.util.SimpleClassScanner;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class AppStarter {
    private static final Logger log = LoggerFactory.getLogger(AppStarter.class);

    private static final int MAX_SEQ = 999;
    private static boolean loaded = false, webapp = false;
    private static String[] args = new String[0];

    public static void main(String[] args) {
        if (!loaded) {
            loaded = true;
            AppStarter.args = args;
            AppStarter begin = new AppStarter();
            begin.doApps(args, false);
            /*
             * Do not start MainApplication(s) if this is a webapp
             */
            if (!webapp) {
                begin.doApps(args, true);
            }
        }
    }

    public static void setWebApp(boolean enable) {
        AppStarter.webapp = enable;
    }

    public static String[] getArgs() {
        return args;
    }

    private void doApps(String[] args, boolean main) {
        // find and execute optional preparation modules
        Utility util = Utility.getInstance();
        SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        Set<String> packages = scanner.getPackages(true);
        int n = 0;
        Map<String, Class<?>> steps = new HashMap<>();
        for (String p : packages) {
            List<Class<?>> services = scanner.getAnnotatedClasses(p, main?
                                        MainApplication.class : BeforeApplication.class);
            for (Class<?> cls : services) {
                if (Feature.isRequired(cls)) {
                    int seq = getSequence(cls, main);
                    String key = util.zeroFill(seq, MAX_SEQ) + "." + util.zeroFill(++n, MAX_SEQ);
                    steps.put(key, cls);
                } else {
                    log.info("Skipping optional {}", cls);
                }
            }
        }
        executeOrderly(steps, args, main);
    }

    private int getSequence(Class<?> cls, boolean main) {
        if (main) {
            MainApplication before = cls.getAnnotation(MainApplication.class);
            return Math.min(MAX_SEQ, before.sequence());
        } else {
            BeforeApplication before = cls.getAnnotation(BeforeApplication.class);
            return Math.min(MAX_SEQ, before.sequence());
        }
    }

    private void executeOrderly(Map<String, Class<?>> steps, String[] args, boolean main) {
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
                    app.start(args);
                    n++;
                } else {
                    error++;
                    log.error("Unable to start {} because it is not an instance of {}",
                            cls.getName(), EntryPoint.class.getName());
                }
            } catch (Exception e) {
                error++;
                log.error("Unable to start - " + cls.getName(), e);
            }
        }
        if (main && error == 0 && n == 0) {
            log.error("Missing MainApplication\n\n{}\n{}\n\n",
                    "Did you forget to annotate your main module with @MainApplication that extends EntryPoint?",
                    "and ensure the package parent is defined in 'web.component.scan' of application.properties.");
        }
    }

}
