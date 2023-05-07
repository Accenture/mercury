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

import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.common.ConfigBase;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.yaml.snakeyaml.Yaml;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class ConfigReader implements ConfigBase {
    private static final Logger log = LoggerFactory.getLogger(ConfigReader.class);

    private static final ConcurrentMap<String, List<String>> loopDetection = new ConcurrentHashMap<>();
    private static final String CLASSPATH = "classpath:";
    private static final String FILEPATH = "file:";
    private static final String JSON = ".json";
    private static final String YML = ".yml";
    private static final String YAML = ".yaml";
    private static final String DOT_PROPERTIES = ".properties";
    private static final String CONFIG_LOOP = "* config loop *";
    /*
     * A normalized map has composite keys expanded into simple key.
     * e.g. "hello.world: 1" becomes "hello: world: 1"
     */
    private boolean isNormalized = true;

    private static AppConfigReader baseConfig;
    private Map<String, Object> properties = new HashMap<>();
    private MultiLevelMap config = new MultiLevelMap(new HashMap<>());

    public static void setBaseConfig(AppConfigReader config) {
        if (ConfigReader.baseConfig == null) {
            ConfigReader.baseConfig = config;
        }
    }

    public boolean isNormalizedMap() {
        return isNormalized;
    }

    /**
     * Environment variable overrides application properties
     *
     * @param key parameter
     * @return value
     */
    @Override
    public Object get(String key) {
        return get(key, null);
    }

    private String getSystemProperty(String key) {
        if (key.isEmpty()) {
            return null;
        }
        return System.getProperty(key);
    }

    @Override
    public Object get(String key, Object defaultValue, String... loop) {
        if (key == null || key.length() == 0) {
            return null;
        }
        String systemProperty = getSystemProperty(key);
        if (systemProperty != null) {
            return systemProperty;
        }
        Object value = isNormalized? config.getElement(key) : properties.get(key);
        if (value == null) {
            value = defaultValue;
        }
        if (value instanceof String) {
            String result = (String) value;
            int bracketStart = result.indexOf("${");
            int bracketEnd = result.lastIndexOf('}');
            if (bracketStart != -1 && bracketEnd != -1 && bracketEnd > bracketStart && baseConfig != null) {
                String middle = result.substring(bracketStart + 2, bracketEnd).trim();
                String middleDefault = null;
                if (!middle.isEmpty()) {
                    String loopId = loop.length == 1 && loop[0].length() > 0? loop[0] : Utility.getInstance().getUuid();
                    int colon = middle.lastIndexOf(':');
                    if (colon > 0) {
                        middleDefault = middle.substring(colon+1);
                        middle = middle.substring(0, colon);
                    }
                    String property = System.getenv(middle);
                    if (property != null) {
                        middle = property;
                    } else {
                        List<String> refs = loopDetection.getOrDefault(loopId, new ArrayList<>());
                        if (refs.contains(middle)) {
                            log.warn("Config loop for '{}' detected", key);
                            middle = CONFIG_LOOP;
                        } else {
                            refs.add(middle);
                            loopDetection.put(loopId, refs);
                            Object mid = baseConfig.get(middle, defaultValue, loopId);
                            middle = mid != null? String.valueOf(mid) : null;
                        }
                    }
                    loopDetection.remove(loopId);
                    String first = result.substring(0, bracketStart);
                    String last = result.substring(bracketEnd+1);
                    if (middleDefault == null) {
                        middleDefault = "";
                    }
                    return first + (middle != null? middle : middleDefault) + last;
                }
            }
        }
        return value;
    }

    @Override
    public String getProperty(String key) {
        Object o = get(key);
        return o != null? String.valueOf(o) : null;
    }

    @Override
    public String getProperty(String key, String defaultValue) {
        String s = getProperty(key);
        return s != null? s : defaultValue;
    }

    public Map<String, Object> getMap() {
        return isNormalized? config.getMap() : properties;
    }

    @Override
    public boolean exists(String key) {
        if (key == null || key.length() == 0) {
            return false;
        }
        return isNormalized? config.exists(key) : properties.containsKey(key);
    }

    @Override
    public boolean isEmpty() {
        return isNormalized? config.isEmpty() : properties.isEmpty();
    }

    @SuppressWarnings("unchecked")
    public void load(String path) throws IOException {
        InputStream in = null;
        if (path.startsWith(CLASSPATH)) {
            in = ConfigReader.class.getResourceAsStream(path.substring(CLASSPATH.length()));
        } else if (path.startsWith(FILEPATH)) {
            try {
                in = Files.newInputStream(Paths.get(path.substring(FILEPATH.length())));
            } catch (IOException e) {
                // ok to ignore
            }
        } else {
            in = ConfigReader.class.getResourceAsStream(path);
        }
        if (in == null) {
            throw new IOException(path+" not found");
        }
        try {
            if (path.endsWith(YML) || path.endsWith(YAML)) {
                Yaml yaml = new Yaml();
                String data = Utility.getInstance().stream2str(in);
                Map<String, Object> m = yaml.load(data.contains("\t")? data.replace("\t", "  ") : data);
                enforceKeysAsText(m);
                config = new MultiLevelMap(normalizeMap(m));
                isNormalized = true;
            } else if (path.endsWith(JSON)) {
                Map<String, Object> m = SimpleMapper.getInstance().getMapper().readValue(in, Map.class);
                enforceKeysAsText(m);
                config = new MultiLevelMap(normalizeMap(m));
                isNormalized = true;
            } else if (path.endsWith(DOT_PROPERTIES)) {
                properties = new HashMap<>();
                Properties p = new Properties();
                p.load(in);
                p.forEach((k, v) -> properties.put(String.valueOf(k), v));
                isNormalized = false;
            }
        } finally {
            try {
                in.close();
            } catch (IOException e) {
                //
            }
        }
    }

    public void load(Map<String, Object> map) {
        enforceKeysAsText(map);
        config = new MultiLevelMap(normalizeMap(map));
        isNormalized = true;
    }

    private Map<String, Object> normalizeMap(Map<String, Object> map) {
        Map<String, Object> flat = Utility.getInstance().getFlatMap(map);
        MultiLevelMap multiMap = new MultiLevelMap(new HashMap<>());
        flat.forEach(multiMap::setElement);
        return multiMap.getMap();
    }

    @SuppressWarnings({"rawtypes", "unchecked"})
    private void enforceKeysAsText(Map raw) {
        Set keys = new HashSet(raw.keySet());
        for (Object k: keys) {
            Object v = raw.get(k);
            // key is assumed to be string
            if (!(k instanceof String)) {
                raw.remove(k);
                raw.put(String.valueOf(k), v);
            }
            if (v instanceof Map) {
                enforceKeysAsText((Map) v);
            }
        }
    }

}
