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

package com.accenture.services;

import org.platformlambda.MainApp;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.security.GeneralSecurityException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class ConfigManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(ConfigManager.class);

    private static final CryptoApi crypto = new CryptoApi();

    private static final ConcurrentMap<String, ConcurrentMap<String, Object>> envConfig = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, ConcurrentMap<String, Object>> appConfig = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, ConcurrentMap<String, Object>> runtimeConfig = new ConcurrentHashMap<>();
    private static final Map<String, byte[]> masterKeys = new HashMap<>();

    private static final String TYPE = "type";
    private static final String CREATED = "created";
    private static final String FILE_UPLOAD = "file_upload";
    private static final String FILE_DOWNLOAD = "file_download";
    private static final String RELOAD = "reload";
    private static final String DECRYPT = "decrypt";
    private static final String ENCRYPTED = "encrypted";
    private static final String PAYLOAD = "payload";
    private static final String KEY_ID = "key";
    private static final String SALT = "salt";
    private static final String MESSAGE = "message";
    private static final String TIME = "time";
    private static final String CREATE_APP = "create_app";
    private static final String ADD_APP = "add_app";
    private static final String DELETE_APP = "delete_app";
    private static final String DROP_APP = "drop_app";
    private static final String GET = "get";
    private static final String KEYS = "keys";
    private static final String UPDATE = "update";
    private static final String MERGE = "merge";
    private static final String DELETE = "delete";
    private static final String CLEAR = "clear";
    private static final String ENTITY = "entity";
    private static final String APPLICATION = "application";
    private static final String DOWNLOAD = "download";
    private static final String RESTORE = "restore";
    private static final String ORIGIN = "origin";
    private static final String CONFIG = "config";
    private static final String ENV = "environment";
    private static final String APP_PROPERTIES = "application.properties";
    private static final String RUN_TIME = "runtime";
    private static final String CURRENT_APP_CONFIG = "current-config.json";
    private static boolean ready = false;
    private static File currentConfig;
    private static String currentMasterKeyId;

    public ConfigManager() {
        if (masterKeys.isEmpty()) {
            try {
                Utility util = Utility.getInstance();
                AppConfigReader reader = AppConfigReader.getInstance();
                if (reader.exists("app.config.key")) {
                    createMasterKeys(reader.getProperty("app.config.key", Utility.getInstance().getUuid()));
                    String storePath = reader.getProperty("app.config.store", "/tmp/config/store");
                    File dir = new File(storePath);
                    if (!dir.exists()) {
                        dir.mkdirs();
                    }
                    currentConfig = new File(dir, CURRENT_APP_CONFIG);
                    String current = "file:" + util.normalizeFolder(dir.getPath()) + "/" + CURRENT_APP_CONFIG;
                    List<String> paths = util.split(reader.getProperty("app.config.yaml",
                            "file:/tmp/config/app-config.yaml, classpath:/app-config.yaml"), ", ");
                    paths.add(0, current);
                    ConfigReader config = getConfig(paths);
                    if (config != null) {
                        // decrypt file if needed
                        config.load(decrypt(config.getMap()));
                        loadConfig(config);
                        saveConfig();
                        ready = true;
                    }
                } else {
                    throw new IllegalArgumentException("missing app.config.key in application.properties");
                }
            } catch (Exception e) {
                log.error("Config manager disabled - {}", e.getMessage());
            }
        }
    }

    private void createMasterKeys(String keySpecs) {
        Utility util = Utility.getInstance();
        List<String> keyList = util.split(keySpecs, ", ");
        int n = 0;
        for (String k: keyList) {
            int colon = k.indexOf(':');
            if (colon > 0) {
                String id = k.substring(0, colon);
                String value = k.substring(colon + 1);
                if (value.length() > 0) {
                    n++;
                    if (n == 1) {
                        currentMasterKeyId = id;
                    }
                    byte[] mk = crypto.getSHA256(util.getUTF(value));
                    masterKeys.put(id, mk);
                    log.info("Loaded API key {}", id);
                }
            }
        }
        if (n == 0) {
            log.error("Config manager disabled because there are no API keys");
        } else {
            log.info("Total {} key{} loaded, current key is {}", n, n == 1? "" : "s", currentMasterKeyId);
        }
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (!ready) {
            throw new AppException(503, "Config manager not ready");
        }
        if (headers.containsKey(TYPE)) {
            String me = Platform.getInstance().getOrigin();
            Utility util = Utility.getInstance();
            PostOffice po = PostOffice.getInstance();
            String type = headers.get(TYPE);
            if (type.equals(GET)) {
                if (headers.containsKey(APPLICATION) && headers.containsKey(ENTITY)) {
                    String entity = headers.get(ENTITY);
                    String app = headers.get(APPLICATION);
                    if (entity.equals(APP_PROPERTIES)) {
                        if (appConfig.containsKey(app)) {
                            return new HashMap<>(appConfig.get(app));
                        } else {
                            throw new AppException(404, app + " not found in configuration store");
                        }
                    } else if (entity.equals(ENV)) {
                        if (envConfig.containsKey(app)) {
                            return new HashMap<>(envConfig.get(app));
                        } else {
                            throw new AppException(404, app + " not found in configuration store");
                        }
                    } else if (entity.equals(RUN_TIME)) {
                        if (runtimeConfig.containsKey(app)) {
                            return new HashMap<>(runtimeConfig.get(app));
                        } else {
                            throw new AppException(404, app + " not found in configuration store");
                        }
                    } else {
                        throw new IllegalArgumentException("configuration type must be " +
                                ENV + ", " + APP_PROPERTIES + " or " + RUN_TIME);
                    }
                } else {
                    List<String> appList = new ArrayList<>(appConfig.keySet());
                    List<String> types = new ArrayList<>();
                    types.add(ENV);
                    types.add(APP_PROPERTIES);
                    types.add(RUN_TIME);
                    Map<String, Object> result = new HashMap<>();
                    result.put(APPLICATION, appList);
                    result.put(TYPE, types);
                    return result;
                }
            }
            if (type.equals(FILE_DOWNLOAD)) {
                boolean clearText = "true".equals(headers.get(DECRYPT));
                if (!currentConfig.exists()) {
                    throw new AppException(404, "Config not found - "+currentConfig.getPath());
                }
                byte[] b = Utility.getInstance().file2bytes(currentConfig);
                Map<String, Object> map = SimpleMapper.getInstance().getMapper().readValue(b, Map.class);
                if (clearText) {
                    map = decrypt(map);
                }
                return map;
            }
            /*
             * File upload is a 2-step process
             * 1. acknowledge user
             * 2. broadcast request to all config managers
             */
            if (type.equals(FILE_UPLOAD) && body instanceof byte[]) {
                byte[] b = (byte[]) body;
                Map<String, Object> encMap = SimpleMapper.getInstance().getMapper().readValue(b, Map.class);
                Map<String, Object> map = decrypt(encMap);
                validateConfig(map);
                // broadcast configuration parameters
                EventEnvelope event = new EventEnvelope();
                event.setTo(MainApp.CONFIG_MANAGER).setBody(map);
                event.setHeader(TYPE, RELOAD);
                po.send(MainApp.PRESENCE_MONITOR, event.toBytes());
                return true;
            }
            if (type.equals(RELOAD) && body instanceof Map) {
                log.info("Loading new configuration");
                Map<String, Object> map = (Map<String, Object>) body;
                ConfigReader config = new ConfigReader();
                config.load(map);
                loadConfig(config);
                saveConfig();
                return true;
            }
            /*
             * Update is a 2-step process
             * 1. acknowledge user
             * 2. broadcast request to all config managers
             */
            if (type.equals(UPDATE) && headers.containsKey(APPLICATION) && headers.containsKey(ENTITY) &&
                    body instanceof Map) {
                Map<String, Object> kv = (Map<String, Object>) body;
                if (kv.isEmpty()) {
                    throw new IllegalArgumentException("Input is empty");
                }
                String entity = headers.get(ENTITY);
                String app = headers.get(APPLICATION);
                if (entity.equals(APP_PROPERTIES)) {
                    if (!appConfig.containsKey(app)) {
                        throw new AppException(404, app+" not found in configuration store");
                    }
                } else if (entity.equals(ENV)) {
                    if (!envConfig.containsKey(app)) {
                        throw new AppException(404, app+" not found in configuration store");
                    }
                } else if (entity.equals(RUN_TIME)) {
                    if (!runtimeConfig.containsKey(app)) {
                        throw new AppException(404, app+" not found in configuration store");
                    }
                } else {
                    throw new IllegalArgumentException("configuration type must be "+
                            ENV+", "+APP_PROPERTIES+" or "+RUN_TIME);
                }
                Map<String, Object> result = new HashMap<>();
                result.put(TYPE, UPDATE);
                result.put(ENTITY, entity);
                result.put(APPLICATION, app);
                result.put(MESSAGE, "merging key-values");
                result.put(TIME, new Date());
                // broadcast configuration parameters
                EventEnvelope event = new EventEnvelope();
                event.setTo(MainApp.CONFIG_MANAGER).setBody(kv);
                event.setHeader(ENTITY, entity).setHeader(APPLICATION, app).setHeader(TYPE, MERGE);
                po.send(MainApp.PRESENCE_MONITOR, event.toBytes());
                return result;
            }
            if (type.equals(MERGE) && headers.containsKey(APPLICATION) && headers.containsKey(ENTITY) &&
                    body instanceof Map) {
                Map<String, Object> kv = (Map<String, Object>) body;
                if (kv.isEmpty()) {
                    return false;
                }
                String entity = headers.get(ENTITY);
                String app = headers.get(APPLICATION);
                if (entity.equals(APP_PROPERTIES) && appConfig.containsKey(app)) {
                    mergeMaps(app, APP_PROPERTIES, kv, appConfig.get(app));
                }
                if (entity.equals(ENV) && envConfig.containsKey(app)) {
                    mergeMaps(app, ENV, kv, envConfig.get(app));
                }
                if (entity.equals(RUN_TIME) && runtimeConfig.containsKey(app)) {
                    mergeMaps(app, RUN_TIME, kv, runtimeConfig.get(app));
                }
                saveConfig();
                return true;
            }
            /*
             * Delete is a 2-step process
             * 1. acknowledge user
             * 2. broadcast request to all config managers
             */
            if (type.equals(DELETE) && headers.containsKey(APPLICATION) && headers.containsKey(ENTITY) &&
                    body instanceof Map) {
                Map<String, Object> kv = (Map<String, Object>) body;
                if (kv.isEmpty()) {
                    throw new IllegalArgumentException("Input is empty");
                }
                if (!kv.containsKey(KEYS)) {
                    throw new IllegalArgumentException("Missing "+KEYS);
                }
                List<String> keys = util.split(kv.get(KEYS).toString(), ", ");
                if (keys.isEmpty()) {
                    throw new IllegalArgumentException("Input is empty");
                }
                String entity = headers.get(ENTITY);
                String app = headers.get(APPLICATION);
                if (entity.equals(APP_PROPERTIES)) {
                    if (!appConfig.containsKey(app)) {
                        throw new AppException(404, app+" not found in configuration store");
                    }
                } else if (entity.equals(ENV)) {
                    if (!envConfig.containsKey(app)) {
                        throw new AppException(404, app+" not found in configuration store");
                    }
                } else if (entity.equals(RUN_TIME)) {
                    if (!runtimeConfig.containsKey(app)) {
                        throw new AppException(404, app+" not found in configuration store");
                    }
                } else {
                    throw new IllegalArgumentException("configuration type must be "+
                            ENV+", "+APP_PROPERTIES+" or "+RUN_TIME);
                }
                Map<String, Object> result = new HashMap<>();
                result.put(TYPE, DELETE);
                result.put(ENTITY, entity);
                result.put(APPLICATION, app);
                result.put(MESSAGE, "deleting "+keys);
                result.put(TIME, new Date());
                // broadcast configuration parameters
                EventEnvelope event = new EventEnvelope();
                event.setTo(MainApp.CONFIG_MANAGER).setBody(keys);
                event.setHeader(ENTITY, entity).setHeader(APPLICATION, app).setHeader(TYPE, CLEAR);
                po.send(MainApp.PRESENCE_MONITOR, event.toBytes());
                return result;
            }
            if (type.equals(CLEAR) && headers.containsKey(APPLICATION) && headers.containsKey(ENTITY) &&
                    body instanceof List) {
                List<String> keys = (List<String>) body;
                if (keys.isEmpty()) {
                    return false;
                }
                String entity = headers.get(ENTITY);
                String app = headers.get(APPLICATION);
                if (entity.equals(APP_PROPERTIES) && appConfig.containsKey(app)) {
                    purgeMap(app, APP_PROPERTIES, keys, appConfig.get(app));
                }
                if (entity.equals(ENV) && envConfig.containsKey(app)) {
                    purgeMap(app, ENV, keys, envConfig.get(app));
                }
                if (entity.equals(RUN_TIME) && runtimeConfig.containsKey(app)) {
                    purgeMap(app, RUN_TIME, keys, runtimeConfig.get(app));
                }
                saveConfig();
                return true;
            }
            /*
             * Create application is a 2-step process
             * 1. acknowledge user
             * 2. broadcast request to all config managers
             */
            if (type.equals(CREATE_APP) && headers.containsKey(APPLICATION)) {
                String app = headers.get(APPLICATION);
                Map<String, Object> result = new HashMap<>();
                result.put(TYPE, CREATE_APP);
                result.put(APPLICATION, app);
                result.put(MESSAGE, "Create application "+app+" if not exists");
                result.put(TIME, new Date());
                // broadcast configuration parameters
                EventEnvelope event = new EventEnvelope();
                event.setTo(MainApp.CONFIG_MANAGER);
                event.setHeader(APPLICATION, app).setHeader(TYPE, ADD_APP);
                po.send(MainApp.PRESENCE_MONITOR, event.toBytes());
                return result;
            }
            if (type.equals(ADD_APP) && headers.containsKey(APPLICATION)) {
                String app = headers.get(APPLICATION);
                if (!appConfig.containsKey(app)) {
                    appConfig.put(app, new ConcurrentHashMap<>());
                    log.info("Application {} {} created", app, APP_PROPERTIES);
                }
                if (!envConfig.containsKey(app)) {
                    envConfig.put(app, new ConcurrentHashMap<>());
                    log.info("Application {} {} created", app, ENV);
                }
                if (!runtimeConfig.containsKey(app)) {
                    runtimeConfig.put(app, new ConcurrentHashMap<>());
                    log.info("Application {} {} created", app, RUN_TIME);
                }
                saveConfig();
                return true;
            }
            /*
             * Delete application is a 2-step process
             * 1. acknowledge user
             * 2. broadcast request to all config managers
             */
            if (type.equals(DELETE_APP) && headers.containsKey(APPLICATION)) {
                String app = headers.get(APPLICATION);
                Map<String, Object> result = new HashMap<>();
                result.put(TYPE, DELETE_APP);
                result.put(APPLICATION, app);
                result.put(MESSAGE, "Delete application "+app+" if exists");
                result.put(TIME, new Date());
                // broadcast configuration parameters
                EventEnvelope event = new EventEnvelope();
                event.setTo(MainApp.CONFIG_MANAGER);
                event.setHeader(APPLICATION, app).setHeader(TYPE, DROP_APP);
                po.send(MainApp.PRESENCE_MONITOR, event.toBytes());
                return result;
            }
            if (type.equals(DROP_APP) && headers.containsKey(APPLICATION)) {
                String app = headers.get(APPLICATION);
                if (appConfig.containsKey(app)) {
                    appConfig.remove(app);
                    log.info("Application {} {} dropped", app, APP_PROPERTIES);
                }
                if (envConfig.containsKey(app)) {
                    envConfig.remove(app);
                    log.info("Application {} {} dropped", app, ENV);
                }
                if (runtimeConfig.containsKey(app)) {
                    runtimeConfig.remove(app);
                    log.info("Application {} {} dropped", app, RUN_TIME);
                }
                saveConfig();
                return true;
            }
            /*
             * Download and restore are used to merge config parameters from a peer when this application starts
             */
            if (type.equals(DOWNLOAD) && headers.containsKey(ORIGIN)) {
                String origin = headers.get(ORIGIN);
                if (!origin.equals(me)) {
                    Map<String, Object> myConfig = new HashMap<>();
                    myConfig.put(ENV, envConfig);
                    myConfig.put(APP_PROPERTIES, appConfig);
                    myConfig.put(RUN_TIME, runtimeConfig);
                    // broadcast configuration parameters
                    EventEnvelope event = new EventEnvelope();
                    event.setTo(MainApp.CONFIG_MANAGER).setBody(myConfig);
                    event.setHeader(TYPE, RESTORE).setHeader(ORIGIN, me);
                    PostOffice.getInstance().send(MainApp.PRESENCE_MONITOR, event.toBytes());
                }
                return true;
            }
            if (type.equals(RESTORE) && headers.containsKey(ORIGIN)) {
                if (body instanceof Map) {
                    String origin = headers.get(ORIGIN);
                    if (!origin.equals(me)) {
                        Map<String, Object> peerConfig = (Map<String, Object>) body;
                        if (peerConfig.get(ENV) instanceof Map && peerConfig.get(APP_PROPERTIES) instanceof Map &&
                                peerConfig.get(RUN_TIME) instanceof Map) {
                            Map<String, Object> envMap = (Map<String, Object>) peerConfig.get(ENV);
                            for (String app : envMap.keySet()) {
                                Map<String, Object> peerMap = (Map<String, Object>) envMap.get(app);
                                ConcurrentMap<String, Object> myMap = envConfig.get(app);
                                mergeMaps(app, ENV, peerMap, myMap);
                            }
                            Map<String, Object> appMap = (Map<String, Object>) peerConfig.get(APP_PROPERTIES);
                            for (String app : appMap.keySet()) {
                                Map<String, Object> peerMap = (Map<String, Object>) appMap.get(app);
                                ConcurrentMap<String, Object> myMap = appConfig.get(app);
                                mergeMaps(app, APP_PROPERTIES, peerMap, myMap);
                            }
                            Map<String, Object> runtimeMap = (Map<String, Object>) peerConfig.get(RUN_TIME);
                            for (String app : runtimeMap.keySet()) {
                                Map<String, Object> peerMap = (Map<String, Object>) runtimeMap.get(app);
                                ConcurrentMap<String, Object> myMap = runtimeConfig.get(app);
                                mergeMaps(app, RUN_TIME, peerMap, myMap);
                            }
                            saveConfig();
                            // got peer config parameters - we can tell initial load to stop
                            if (po.exists(MainApp.INITIAL_LOAD)) {
                                po.send(MainApp.INITIAL_LOAD, "done");
                            }
                        }
                    }
                    return true;
                }
            }
        }
        throw new IllegalArgumentException("Invalid request");
    }

    private void mergeMaps(String app, String type, Map<String, Object> peerMap, ConcurrentMap<String, Object> myMap) {
        for (String k : peerMap.keySet()) {
            if (!peerMap.get(k).equals(myMap.get(k))) {
                myMap.put(k, peerMap.get(k));
                log.info("Updated {} in {} {}", k, type, app);
            }
        }
    }

    private void purgeMap(String app, String type, List<String> keys, ConcurrentMap<String, Object> myMap) {
        for (String k : keys) {
            if (myMap.containsKey(k)) {
                myMap.remove(k);
                log.info("Removed {} from {} {}", k, type, app);
            }
        }
    }

    @SuppressWarnings("unchecked")
    private void validateConfig(Map<String, Object> input) {
        if (input.isEmpty()) {
            throw new IllegalArgumentException("Config file is empty");
        }
        if (input.size() != 1) {
            throw new IllegalArgumentException("Invalid config file");
        }
        if (!input.containsKey(CONFIG)) {
            throw new IllegalArgumentException("Invalid config file - missing 'config' section");
        }
        Map<String, Object> map = (Map<String, Object>) input.get(CONFIG);
        for (String k: map.keySet()) {
            int n = 0;
            Object app = map.get(k);
            if (app instanceof Map) {
                Map<String, Object> m = (Map<String, Object>) app;
                if (m.size() > 3) {
                    throw new IllegalArgumentException("Invalid config file - application config must be a map of "+
                            ENV+", "+APP_PROPERTIES+", "+RUN_TIME);
                }
                for (String type: m.keySet()) {
                    if (!type.equals(ENV) && !type.equals(APP_PROPERTIES) && !type.equals(RUN_TIME)) {
                        throw new IllegalArgumentException("Invalid config file - Expect: "+
                                ENV+", "+APP_PROPERTIES+" and "+RUN_TIME + " Actual: "+m.keySet());
                    }
                }

            } else {
                throw new IllegalArgumentException("Invalid config file - application config may be a map of "+
                        ENV+", "+APP_PROPERTIES+", "+RUN_TIME);
            }

        }
    }

    @SuppressWarnings("unchecked")
    private void loadConfig(ConfigReader config) {
        // clear configuration
        envConfig.clear();
        appConfig.clear();
        runtimeConfig.clear();
        // load configuration
        Map<String, Object> map = config.getMap();
        Object data = map.get(CONFIG);
        if (data instanceof Map) {
            Map<String, Object> appList = (Map<String, Object>) data;
            for (String app: appList.keySet()) {
                Object appMap = appList.get(app);
                if (appMap instanceof Map) {
                    Map<String, Object> appData = (Map<String, Object>) appMap;
                    if (appData.containsKey(ENV) && appData.get(ENV) instanceof Map) {
                        Map<String, Object> m = (Map<String, Object>) appData.get(ENV);
                        envConfig.put(app, new ConcurrentHashMap<>(m));
                    } else {
                        envConfig.put(app, new ConcurrentHashMap<>());
                    }
                    if (appData.containsKey(APP_PROPERTIES) && appData.get(APP_PROPERTIES) instanceof Map) {
                        Map<String, Object> m = (Map<String, Object>) appData.get(APP_PROPERTIES);
                        appConfig.put(app, new ConcurrentHashMap<>(m));
                    } else {
                        appConfig.put(app, new ConcurrentHashMap<>());
                    }
                    if (appData.containsKey(RUN_TIME) && appData.get(RUN_TIME) instanceof Map) {
                        Map<String, Object> m = (Map<String, Object>) appData.get(RUN_TIME);
                        runtimeConfig.put(app, new ConcurrentHashMap<>(m));
                    } else {
                        runtimeConfig.put(app, new ConcurrentHashMap<>());
                    }
                }
            }
            log.info("{} loaded with {} application{}", currentConfig, envConfig.size(), envConfig.size() == 1? "" : "s");

        } else {
            log.warn("No application configuration parameters are found - missing 'config'");
        }
    }

    @SuppressWarnings("unchecked")
    private boolean sameConfig(Map<String, Object> map1, Map<String, Object> map2) {
        if (map1.size() != map2.size()) {
            return false;
        }
        List<String> key1 = new ArrayList<>(map1.keySet());
        List<String> key2 = new ArrayList<>(map2.keySet());
        if (key1.size() > 1) {
            Collections.sort(key1);
        }
        if (key2.size() > 1) {
            Collections.sort(key2);
        }
        if (!key1.equals(key2)) {
            return false;
        }
        for (String k: key1) {
            Object o1 = map1.get(k);
            Object o2 = map2.get(k);
            if (o1 instanceof Map && o2 instanceof Map) {
                if (!sameConfig((Map<String, Object>) o1, (Map<String, Object>) o2)) {
                    return false;
                }
            } else {
                if (!o1.equals(o2)) {
                    return false;
                }
            }
        }
        return true;
    }

    private ConfigReader getConfig(List<String> paths) throws GeneralSecurityException {
        for (String p: paths) {
            ConfigReader config = new ConfigReader();
            try {
                config.load(p);
                Map<String, Object> map = config.getMap();
                if (map.size() == 3 && ENCRYPTED.equals(map.get(TYPE)) && map.containsKey(SALT) && map.containsKey(PAYLOAD)) {
                    log.info("Decrypting config from {}", p);
                    config.load(decrypt(map));
                }
                log.info("Loaded config from {}", p);
                return config;
            } catch (IOException e) {
                log.warn("Skipping {} - {}", p, e.getMessage());
            }
        }
        return null;
    }

    @SuppressWarnings("unchecked")
    private void saveConfig() throws IOException, GeneralSecurityException {
        Utility util = Utility.getInstance();
        Map<String, Object> map = new HashMap<>();
        for (String app: envConfig.keySet()) {
            Map<String, Object> appMap = new HashMap<>();
            if (envConfig.containsKey(app)) {
                appMap.put(ENV, envConfig.get(app));
            }
            if (appConfig.containsKey(app)) {
                appMap.put(APP_PROPERTIES, appConfig.get(app));
            }
            if (runtimeConfig.containsKey(app)) {
                appMap.put(RUN_TIME, runtimeConfig.get(app));
            }
            map.put(app, appMap);
        }
        Map<String, Object> result = new HashMap<>();
        result.put(CONFIG, map);
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        if (currentConfig.exists()) {
            String prev = util.file2str(currentConfig);
            Map<String, Object> prevConfig = decrypt(mapper.readValue(prev, Map.class));
            if (!sameConfig(prevConfig, result)) {
                util.str2file(currentConfig, mapper.writeValueAsString(encrypt(result)));
                log.info("{} updated", currentConfig);
            }

        } else {
            util.str2file(currentConfig, mapper.writeValueAsString(encrypt(result)));
            log.info("{} created", currentConfig);
        }
    }

    private Map<String, Object> encrypt(Map<String, Object> map) throws IOException, GeneralSecurityException {
        Map<String, Object> result = new HashMap<>();
        result.put(TYPE, CONFIG);
        result.put(ENCRYPTED, true);
        // generate salt for session key
        Utility util = Utility.getInstance();
        String date = util.date2str(new Date());
        String salt = util.getDateUuid();
        byte[] sessionKey = getSessionKey(currentMasterKeyId, salt);
        // save salt and payload in an envelope
        Map<String, Object> envelope = new HashMap<>();
        envelope.put(PAYLOAD, map);
        envelope.put(SALT, salt);
        envelope.put(CREATED, date);
        // pack the envelope as bytes
        byte[] b = SimpleMapper.getInstance().getMapper().writeValueAsBytes(envelope);
        // encrypt the envelope using session key
        byte[] encText = crypto.aesEncrypt(b, sessionKey);
        String b64 = util.bytesToUrlBase64(encText);
        result.put(PAYLOAD, b64);
        result.put(SALT, salt);
        result.put(KEY_ID, currentMasterKeyId);
        result.put(CREATED, date);
        return result;
    }

    @SuppressWarnings("unchecked")
    private Map<String, Object> decrypt(Map<String, Object> map) throws IOException, GeneralSecurityException {
        if (Boolean.TRUE.equals(map.get(ENCRYPTED)) && map.containsKey(SALT) && map.containsKey(CREATED) &&
                map.containsKey(KEY_ID) && map.containsKey(PAYLOAD)) {
            Utility util = Utility.getInstance();
            String keyId = (String) map.get(KEY_ID);
            if (!masterKeys.containsKey(keyId)) {
                throw new IllegalArgumentException("Key "+keyId+" is invalid or expired");
            }
            String date = (String) map.get(CREATED);
            String salt = (String) map.get(SALT);
            String payload = (String) map.get(PAYLOAD);
            byte[] sessionKey = getSessionKey(keyId, salt);
            byte[] clearText = crypto.aesDecrypt(util.urlBase64ToBytes(payload), sessionKey);
            Map<String, Object> envelope = SimpleMapper.getInstance().getMapper().readValue(clearText, Map.class);
            if (envelope.containsKey(SALT) && envelope.containsKey(PAYLOAD)) {
                Object o = envelope.get(PAYLOAD);
                if (o instanceof Map && salt.equals(envelope.get(SALT)) && date.equals(envelope.get(CREATED))) {
                    return (Map<String, Object>) o;
                }
            }
            throw new IllegalArgumentException("Unable to decrypt - invalid dataset");

        } else {
            return map;
        }
    }

    private byte[] getSessionKey(String keyId, String salt) throws IOException {
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        out.write(Utility.getInstance().getUTF(salt));
        out.write(masterKeys.get(keyId));
        return crypto.getSHA256(out.toByteArray());
    }

}
