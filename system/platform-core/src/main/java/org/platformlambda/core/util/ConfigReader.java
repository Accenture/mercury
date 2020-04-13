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

import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.common.ConfigBase;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.yaml.snakeyaml.Yaml;

import java.io.*;
import java.util.*;

public class ConfigReader implements ConfigBase {
    private static final Logger log = LoggerFactory.getLogger(ConfigReader.class);

    private static final String CLASSPATH = "classpath:";
    private static final String FILEPATH = "file:";
    private static final String JSON = ".json";
    private static final String YML = ".yml";
    private static final String YAML = ".yaml";
    private static final String PROPERTIES = ".properties";
    private static final String ENV_VARIABLES = "env.variables";
    /*
     * A normalized map has composite keys expanded into simple key.
     * e.g. "hello.world: 1" becomes "hello: world: 1"
     */
    private boolean isNormalized = true;
    private Map<String, Object> properties = new HashMap<>();
    private MultiLevelMap config = new MultiLevelMap(new HashMap<>());
    private Map<String, String> envVars = new HashMap<>();

    public boolean isNormalizedMap() {
        return isNormalized;
    }

    public Object getRaw(String key) {
        if (key == null || key.length() == 0) {
            return null;
        }
        String v = getSystemProperty(key);
        return v != null? v : (isNormalized? config.getElement(key) : properties.get(key));
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
        String v = getSystemProperty(key);
        Object value = v != null? v : (isNormalized? config.getElement(key) : properties.get(key));
        // get value substitution from application.yml and/or application.properties
        if (value instanceof String) {
            String s = (String) value;
            if (s.startsWith("${") && s.endsWith("}")) {
                String k = s.substring(2, s.length()-1).trim();
                if (k.length() > 0) {
                    Object replacement = AppConfigReader.getInstance().get(k);
                    if (replacement != null) {
                        return replacement;
                    }
                }
            }
        }
        return value;
    }

    public String getSystemProperty(String key) {
        if (key == null || key.length() == 0) {
            return null;
        }
        String value = System.getProperty(key);
        return value != null? value : (envVars.containsKey(key)? System.getenv(envVars.get(key)) : null);
    }

    @Override
    public Object get(String key, Object defaultValue) {
        Object o = get(key);
        return o != null? o : defaultValue;
    }

    @Override
    public String getProperty(String key) {
        Object o = get(key);
        return o != null? (o instanceof String? (String) o : o.toString()) : null;
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
            in = new FileInputStream(new File(path.substring(FILEPATH.length())));
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

            } else if (path.endsWith(PROPERTIES)) {
                properties = new HashMap<>();
                Properties p = new Properties();
                p.load(in);
                for (Object k : p.keySet()) {
                    properties.put(k.toString(), p.get(k));
                }
                isNormalized = false;
            }
            // load environment variables if any
            Object o = isNormalized? config.getElement(ENV_VARIABLES) : properties.get(ENV_VARIABLES);
            if (o != null) {
                envVars = getEnvVars(o);
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
        // load environment variables if any
        Object o = config.getElement(ENV_VARIABLES);
        if (o != null) {
            envVars = getEnvVars(o);
        }
        config = new MultiLevelMap(normalizeMap(map));
        isNormalized = true;
    }

    private Map<String, String> getEnvVars(Object o) {
        Map<String, String> result = new HashMap<>();
        String[] vars = o.toString().split(",");
        for (String v : vars) {
            String s = v.trim();
            if (s.length() > 0) {
                if (s.contains(":")) {
                    // custom mapping
                    int colon = s.indexOf(':');
                    String part1 = s.substring(0, colon).trim();
                    String part2 = s.substring(colon + 1).trim();
                    if (part1.length() > 0 && part2.length() > 0) {
                        result.put(part2, part1);
                    } else {
                        log.error("Invalid environment variable definition {}, format SOME_VAR:some.var", s);
                    }
                } else {
                    // default mapping: variable name = Environment Variable to lower case and replace UNDERSCORE with DOT characters
                    result.put(s.toLowerCase().replace('_', ','), s);
                }
            }
        }
        return result;
    }

    private Map<String, Object> normalizeMap(Map<String, Object> map) {
        Map<String, Object> flat = Utility.getInstance().getFlatMap(map);
        MultiLevelMap multiMap = new MultiLevelMap(new HashMap<>());
        for (String k : flat.keySet()) {
            multiMap.setElement(k, flat.get(k));
        }
        return multiMap.getMap();
    }

    @SuppressWarnings("unchecked")
    private void enforceKeysAsText(Map raw) {
        Set keys = new HashSet(raw.keySet());
        for (Object k: keys) {
            Object v = raw.get(k);
            if (!(k instanceof String)) {
                log.warn("key {} converted to String", k);
                raw.remove(k);
                raw.put(k.toString(), v);
            }
            if (v instanceof Map) {
                enforceKeysAsText((Map) v);
            }
        }
    }

}
