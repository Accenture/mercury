/*

    Copyright 2018-2022 Accenture Technology

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
import org.yaml.snakeyaml.Yaml;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;

public class ConfigReader implements ConfigBase {

    private static final String CLASSPATH = "classpath:";
    private static final String FILEPATH = "file:";
    private static final String JSON = ".json";
    private static final String YML = ".yml";
    private static final String YAML = ".yaml";
    private static final String DOT_PROPERTIES = ".properties";
    /*
     * A normalized map has composite keys expanded into simple key.
     * e.g. "hello.world: 1" becomes "hello: world: 1"
     */
    private boolean isNormalized = true;
    private Map<String, Object> properties = new HashMap<>();
    private MultiLevelMap config = new MultiLevelMap(new HashMap<>());

    public boolean isNormalizedMap() {
        return isNormalized;
    }

    public Object getRaw(String key) {
        if (key == null || key.length() == 0) {
            return null;
        }
        Object value = isNormalized? config.getElement(key) : properties.get(key);
        if (value instanceof String) {
            String s = (String) value;
            if (s.startsWith("${") && s.endsWith("}")) {
                return getEnvVariable(s);
            }
        }
        return value;
    }

    private String getEnvVariable(String s) {
        if (s.startsWith("${") && s.endsWith("}")) {
            String key = s.substring(2, s.length()-1).trim();
            String def = null;
            if (key.contains(":")) {
                int colon = key.indexOf(':');
                String k = key.substring(0, colon);
                def = key.substring(colon+1);
                key = k;
            }
            String property = System.getenv(key);
            if (property != null) {
                return property;
            }
            return System.getProperty(key, def);

        } else {
            return null;
        }
    }

    /**
     * Environment variable overrides application properties
     *
     * @param key parameter
     * @return value
     */
    @Override
    public Object get(String key) {
        if (key == null || key.length() == 0) {
            return null;
        }
        String systemProperty = getSystemProperty(key);
        if (systemProperty != null) {
            return systemProperty;
        }
        Object value = isNormalized? config.getElement(key) : properties.get(key);
        if (value instanceof String) {
            String s = (String) value;
            if (s.startsWith("${") && s.endsWith("}")) {
                String k = s.substring(2, s.length()-1).trim();
                if (!k.isEmpty()) {
                    if (key.equals(k)) {
                        return null;
                    }
                    // get replacement from parent or environment variable
                    Object replacement = AppConfigReader.getInstance().get(k);
                    return replacement != null? replacement : getEnvVariable(s);
                }
            }
        }
        return value;
    }

    private String getSystemProperty(String key) {
        if (key.isEmpty()) {
            return null;
        }
        return System.getProperty(key);
    }

    @Override
    public Object get(String key, Object defaultValue) {
        Object o = get(key);
        return o != null? o : defaultValue;
    }

    @Override
    public String getProperty(String key) {
        Object o = get(key);
        return o != null? o.toString() : null;
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
        InputStream in;
        if (path.startsWith(CLASSPATH)) {
            in = ConfigReader.class.getResourceAsStream(path.substring(CLASSPATH.length()));
        } else if (path.startsWith(FILEPATH)) {
            in = Files.newInputStream(Paths.get(path.substring(FILEPATH.length())));
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
                for (Object k : p.keySet()) {
                    properties.put(k.toString(), p.get(k));
                }
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
        for (String k : flat.keySet()) {
            multiMap.setElement(k, flat.get(k));
        }
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
                raw.put(k.toString(), v);
            }
            if (v instanceof Map) {
                enforceKeysAsText((Map) v);
            }
        }
    }

}
