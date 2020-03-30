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
import java.util.HashMap;
import java.util.Map;

public class AppConfigReader implements ConfigBase {
    private static final Logger log = LoggerFactory.getLogger(AppConfigReader.class);
    private static final String APP_PROPS = "classpath:/application.properties";
    private static final String APP_YML = "classpath:/application.yml";
    private static ConfigReader propReader, yamlReader;
    private static MultiLevelMap multiMap;
    private static final AppConfigReader instance = new AppConfigReader();

    public static AppConfigReader getInstance() {
        return instance;
    }

    private AppConfigReader() {
        /*
         * Load application.properties
         * property substitution not required because this is the top level config file
         */
        propReader = new ConfigReader();
        try {
            propReader.doSubstitution = false;
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
            yamlReader.doSubstitution = false;
            yamlReader.load(APP_YML);
            multiMap = new MultiLevelMap(yamlReader.getMap());
        } catch (IOException e) {
            multiMap = new MultiLevelMap(new HashMap<>());
        }
        if (propReader.isEmpty() && multiMap.isEmpty()) {
            log.error("Application config not loaded. Please check {} or {}", APP_PROPS, APP_YML);
        }
    }

    @Override
    public Object get(String key) {
        // 1. get property value from system first
        Object value = propReader.isEmpty()? yamlReader.getSystemProperty(key) : propReader.getSystemProperty(key);
        if (value != null) {
            return value;
        }
        // 2. get it from application.properties or application.yml
        if (propReader.exists(key)) {
            return propReader.get(key);
        } else {
            return multiMap.getElement(key);
        }
    }

    @Override
    public Object get(String key, Object defaultValue) {
        Object value = get(key);
        return value == null? defaultValue : value;
    }

    @Override
    public String getProperty(String key) {
        Object value = get(key);
        return value instanceof String? (String) value : (value == null? null : value.toString());
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
        return multiMap.getMap();
    }

    @Override
    public boolean exists(String key) {
        return propReader.exists(key) || multiMap.exists(key);
    }

    @Override
    public boolean isEmpty() {
        return propReader.isEmpty() && multiMap.isEmpty();
    }

}