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
package org.platformlambda.kafka.util;

import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class SetupUtil {
    private static final Logger log = LoggerFactory.getLogger(SetupUtil.class);

    private static final String FILEPATH = "file:";
    private static final ConcurrentMap<String, Boolean> messages = new ConcurrentHashMap<>();

    public static ConfigReader getConfig(String pathList) {
        ConfigReader config = new ConfigReader();
        List<String> paths = Utility.getInstance().split(pathList, ",");
        for (String p: paths) {
            // ignore trailing spaces
            String name = p.trim();
            try {
                // avoid redundant log
                if (!messages.containsKey(name)) {
                    messages.put(name, true);
                    log.info("Loading from {}", name);
                }
                if (name.startsWith(FILEPATH)) {
                    File f = new File(name.substring(FILEPATH.length()));
                    if (!f.exists()) {
                        if (!messages.containsKey(f.getPath())) {
                            messages.put(f.getPath(), true);
                            log.warn("{} not found", f);
                        }
                        continue;
                    }
                }
                config.load(name);
                return config;
            } catch (IOException e) {
                log.error("Unable to setup kafka from {} - {}", p, e.getMessage());
                System.exit(-1);
            }
        }
        return null;
    }

}
