/*

    Copyright 2018-2023 Accenture Technology

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
    private static final String APP_PROPS = "classpath:/application.properties";
    private static final String APP_YML = "classpath:/application.yml";
    private final ConfigReader propReader;
    private final ConfigReader yamlReader;
    private static final AppConfigReader INSTANCE = new AppConfigReader();

    public static AppConfigReader getInstance() {
        return INSTANCE;
    }

    private AppConfigReader() {
        ConfigReader.setBaseConfig(this);
        /*
         * Load application.properties
         * property substitution not required because this is the top level config file
         */
        propReader = new ConfigReader();
        try {
            propReader.load(APP_PROPS);
        } catch (IOException e) {
            // ok to ignore
        }
        /*
         * Load application.yml
         * property substitution not required because this is the top level config file
         */
        yamlReader = new ConfigReader();
        try {
            yamlReader.load(APP_YML);
        } catch (IOException e) {
            // ok to ignore
        }
        if (propReader.isEmpty() && yamlReader.isEmpty()) {
            log.error("Application config not loaded. Please check {} or {}", APP_PROPS, APP_YML);
        }
    }

    @Override
    public Object get(String key) {
        return propReader.exists(key) ? propReader.get(key) : yamlReader.get(key);
    }

    @Override
    public Object get(String key, Object defaultValue, String... loop) {
        return propReader.exists(key) ?
                propReader.get(key, defaultValue, loop) : yamlReader.get(key, defaultValue, loop);
    }

    @Override
    public String getProperty(String key) {
        Object value = get(key);
        if (value instanceof String) {
            return (String) value;
        } else {
            return value == null? null : String.valueOf(value);
        }
    }

    @Override
    public String getProperty(String key, String defaultValue) {
        String value = getProperty(key);
        return value == null? defaultValue : value;
    }

    public Map<String, Object> getPropertyMap() {
        return propReader.getMap();
    }

    public Map<String, Object> getYamlMap() {
        return yamlReader.getMap();
    }

    @Override
    public boolean exists(String key) {
        return propReader.exists(key) || yamlReader.exists(key);
    }

    @Override
    public boolean isEmpty() {
        return propReader.isEmpty() && yamlReader.isEmpty();
    }

}