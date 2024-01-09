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

    /**
     * This is the singleton object to hold the base configuration files
     * application.yml and application.properties.
     * <p>
     * Note that you can provide one or both files in the "resources" folder.
     */
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

    /**
     * Retrieve a parameter value by key
     *
     * @param key of a configuration parameter
     * @return parameter value
     */
    @Override
    public Object get(String key) {
        return propReader.exists(key) ? propReader.get(key) : yamlReader.get(key);
    }

    /**
     * Retrieve a parameter value by key, given a default value
     *
     * @param key of a configuration parameter
     * @param defaultValue if key does not exist
     * @param loop reserved for internal use to detect configuration loops
     * @return parameter value
     */
    @Override
    public Object get(String key, Object defaultValue, String... loop) {
        return propReader.exists(key) ?
                propReader.get(key, defaultValue, loop) : yamlReader.get(key, defaultValue, loop);
    }

    /**
     * Retrieve a parameter value by key with return value enforced as a string
     *
     * @param key of a configuration parameter
     * @return parameter value as a string
     */
    @Override
    public String getProperty(String key) {
        Object value = get(key);
        if (value instanceof String) {
            return (String) value;
        } else {
            return value == null? null : String.valueOf(value);
        }
    }

    /**
     * Retrieve a parameter value by key with return value enforced as a string, given a default value
     *
     * @param key of a configuration parameter
     * @param defaultValue if key does not exist
     * @return parameter value as a string
     */
    @Override
    public String getProperty(String key, String defaultValue) {
        String value = getProperty(key);
        return value == null? defaultValue : value;
    }

    /**
     * Retrieve the underlying property map
     * (Note that this returns a raw map without value substitution)
     *
     * @return map of key-values
     */
    public Map<String, Object> getPropertyMap() {
        return propReader.getMap();
    }

    /**
     * Retrieve the underlying YAML map
     * (Note that this returns a raw map without value substitution)
     *
     * @return map of key-values
     */
    public Map<String, Object> getYamlMap() {
        return yamlReader.getMap();
    }

    /**
     * Check if a key exists
     *
     * @param key of a configuration parameter
     * @return true if key exists
     */
    @Override
    public boolean exists(String key) {
        return propReader.exists(key) || yamlReader.exists(key);
    }

    /**
     * Check if the configuration file is empty
     *
     * @return true if empty
     */
    @Override
    public boolean isEmpty() {
        return propReader.isEmpty() && yamlReader.isEmpty();
    }

}