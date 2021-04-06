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
package org.platformlambda.hazelcast.util;

import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.List;

public class ConfigUtil {
    private static final Logger log = LoggerFactory.getLogger(ConfigUtil.class);

    public static ConfigReader getConfig(String key, String defaultValue) throws IOException {
        AppConfigReader reader = AppConfigReader.getInstance();
        List<String> paths = Utility.getInstance().split(reader.getProperty(key, defaultValue), ", ");
        for (String p: paths) {
            ConfigReader config = new ConfigReader();
            try {
                config.load(p);
                log.info("Loading config from {}", p);
                return config;
            } catch (IOException e) {
                log.warn("Skipping {} - {}", p, e.getMessage());
            }
        }
        throw new IOException("Endpoint configuration not found in "+paths);
    }

}
