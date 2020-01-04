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

    protected Map<String, Object> config = new HashMap<>();
    protected boolean doSubstitution = true;
    private Map<String, String> environmentVars = new HashMap<>();
    private boolean flat = false;

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
        String value = getSystemProperty(key);
        return value != null? value : config.get(key);
    }

    private String getSystemProperty(String key) {
        if (key == null || key.length() == 0) {
            return null;
        }
        String value = System.getProperty(key);
        return value != null? value : (environmentVars.containsKey(key)? System.getenv(environmentVars.get(key)) : null);
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

    @Override
    public Map<String, Object> getMap() {
        return config;
    }

    @Override
    public boolean exists(String key) {
        if (key == null || key.length() == 0) {
            return false;
        }
        return config.containsKey(key);
    }

    @Override
    public boolean isEmpty() {
        return config.isEmpty();
    }

    @Override
    public int size() {
        return config.size();
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
                config = yaml.load(data.contains("\t")? data.replace("\t", "  ") : data);
                enforceKeysAsText(config);
            } else if (path.endsWith(JSON)) {
                config = SimpleMapper.getInstance().getMapper().readValue(in, Map.class);
                enforceKeysAsText(config);
            } else if (path.endsWith(PROPERTIES)) {
                Properties p = new Properties();
                p.load(in);
                for (Object k : p.keySet()) {
                    config.put(k.toString(), p.get(k));
                }
            }
            // load environment variables if any
            Object o = config.get(ENV_VARIABLES);
            if (o != null) {
                environmentVars.putAll(getEnvVars(o));
            }
            if (doSubstitution) {
                update(config);
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
        config = map;
        enforceKeysAsText(config);
        // load environment variables if any
        Object o = config.get(ENV_VARIABLES);
        if (o != null) {
            environmentVars.putAll(getEnvVars(o));
        }
        if (doSubstitution) {
            update(config);
        }
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

    public void flattenMap() {
        if (!flat) {
            flat = true;
            config = Utility.getInstance().getFlatMap(config);
        }
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

    @SuppressWarnings("unchecked")
    private void update(Map<String, Object> map) {
        for (String k: map.keySet()) {
            Object v = map.get(k);
            if (v instanceof String) {
                String s = ((String) v).trim();
                if (s.startsWith("${") && s.endsWith("}")) {
                    String key = s.substring(2, s.length()-1).trim();
                    if (key.length() > 0) {
                        Object replacement = AppConfigReader.getInstance().get(key);
                        if (replacement != null) {
                            map.put(k, replacement);
                        }
                    }
                }
            }
            if (v instanceof Map) {
                update((Map<String, Object>)v);
            }
            if (v instanceof List) {
                map.put(k, update((List<Object>)v));
            }
        }
    }

    @SuppressWarnings("unchecked")
    private List<Object> update(List<Object> list) {
        List<Object> result = new ArrayList<>();
        for (Object v: list) {
            boolean replaced = false;
            if (v instanceof String) {
                String s = ((String) v).trim();
                if (s.startsWith("${") && s.endsWith("}")) {
                    String key = s.substring(2, s.length()-1).trim();
                    if (key.length() > 0) {
                        Object replacement = AppConfigReader.getInstance().get(key);
                        if (replacement != null) {
                            result.add(replacement);
                            replaced = true;
                        }
                    }
                }
            }
            if (v instanceof Map) {
                update((Map<String, Object>)v);
            }
            if (!replaced) {
                result.add(v);
            }
        }
        return result;
    }

}
