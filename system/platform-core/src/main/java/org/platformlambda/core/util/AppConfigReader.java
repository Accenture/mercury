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

package org.platformlambda.core.util;

import org.platformlambda.core.util.common.ConfigBase;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;

public class AppConfigReader implements ConfigBase {
    private static final Logger log = LoggerFactory.getLogger(AppConfigReader.class);
    private static final AppConfigReader instance = new AppConfigReader();

    private static ConfigReader reader;

    public static AppConfigReader getInstance() {
        return instance;
    }

    private static final String APP_PROPS = "classpath:/application.properties";
    private static final String APP_YML = "classpath:/application.yml";

    private AppConfigReader() {
        reload();
    }

    public void reload() {
        reader = new ConfigReader();
        /*
         * To avoid loop, skip parameter value substitution because AppConfigReader is the base configuration.
         *
         * Other ConfigReader may use the dollar bracket convention to substitution parameter values
         * from the base AppConfigReader.
         */
        reader.doSubstitution = false;
        try {
            // load application.properties
            reader.load(APP_PROPS);
            if (!reader.isEmpty()) {
                log.debug("Loaded {} item{} from {}", reader.size(),
                        reader.size() == 1? "" : "s", APP_PROPS);
                return;
            }
        } catch (IOException e) {
            log.error("Unable to load {}, {}", APP_PROPS, e.getMessage());
        }
        try {
            // try YAML if application.properties is not found
            reader.load(APP_YML);
            if (!reader.isEmpty()) {
                /*
                 * flatten the map so it can be read in dot and bracket format directly
                 * e.g. "application.name"
                 */
                reader.flattenMap();
                log.debug("Loaded {} item{} from {}", reader.size(), reader.size() == 1? "" : "s", APP_YML);
            }
        } catch (IOException e) {
            log.error("Unable to load {}, {}", APP_YML, e.getMessage());
        }
    }

    @Override
    public Object get(String key) {
        return reader.get(key);
    }

    @Override
    public Object get(String key, Object defaultValue) {
        return reader.get(key, defaultValue);
    }

    @Override
    public String getProperty(String key) {
        return reader.getProperty(key);
    }

    @Override
    public String getProperty(String key, String defaultValue) {
        return reader.getProperty(key, defaultValue);
    }

    @Override
    public Map<String, Object> getMap() {
        return reader.getMap();
    }

    @Override
    public boolean exists(String key) {
        return reader.exists(key);
    }

    @Override
    public boolean isEmpty() {
        return reader.isEmpty();
    }

    @Override
    public int size() {
        return reader.size();
    }

}
