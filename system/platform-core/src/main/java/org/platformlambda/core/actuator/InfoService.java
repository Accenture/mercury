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

package org.platformlambda.core.actuator;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.VersionInfo;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.text.NumberFormat;
import java.util.*;
import java.util.concurrent.TimeoutException;

public class InfoService implements LambdaFunction {
    private static final String ERROR = "error";
    private static final String SYSTEM_INFO = "additional.info";
    private static final String STREAMS = "streams";
    private static final String JAVA_VERSION = "java.version";
    private static final String JAVA_VM_VERSION = "java.vm.version";
    private static final String JAVA_RUNTIME_VERSION = "java.runtime.version";
    private static final String TYPE = "type";
    private static final String INFO = "info";
    private static final String QUERY = "query";
    private static final String APP_DESCRIPTION = "info.app.description";
    private static final String APP = "app";
    private static final String NAME = "name";
    private static final String VERSION = "version";
    private static final String DESCRIPTION = "description";
    private static final String JVM = "vm";
    private static final String MEMORY = "memory";
    private static final String MAX = "max";
    private static final String ALLOCATED = "allocated";
    private static final String FREE = "free";
    private static final String ORIGIN = "origin";
    private static final String INSTANCE = "instance";
    private static final String PERSONALITY = "personality";
    private static final String ROUTING = "routing";
    private static final String ROUTES = "routes";
    private static final String LIB = "lib";
    private static final String ENV = "env";
    private static final String DOWNLOAD = "download";
    private static final String CLOUD_CONNECTOR = "cloud.connector";
    private static final String LIBRARY = "library";
    private static final String ROUTE_SUBSTITUTION = "route_substitution";
    private static final String TIME = "time";
    private static final String START = "start";
    private static final String CURRENT = "current";
    private static final String SHOW_ENV = "show.env.variables";
    private static final String SHOW_PROPERTIES = "show.application.properties";
    private static final String SYSTEM_ENV = "environment";
    private static final String APP_PROPS = "properties";
    private static final String MISSING = "missing";
    private static final Date START_TIME = new Date();
    private final String description;
    private final Boolean isServiceMonitor;

    public InfoService() {
        AppConfigReader config = AppConfigReader.getInstance();
        description = config.getProperty(APP_DESCRIPTION, Platform.getInstance().getName());
        isServiceMonitor = "true".equals(config.getProperty("service.monitor", "false"));
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance)
            throws AppException, TimeoutException {
        String type = headers.getOrDefault(TYPE, INFO);
        Platform platform = Platform.getInstance();
        Map<String, Object> result = new HashMap<>();
        Map<String, Object> app = new HashMap<>();
        Utility util = Utility.getInstance();
        VersionInfo info = util.getVersionInfo();
        result.put(APP, app);
        /*
         * When running inside IDE, there are no information about libraries
         * so it is better to take the application name from the application.properties
         */
        app.put(NAME, info.getArtifactId());
        app.put(VERSION, info.getVersion());
        app.put(DESCRIPTION, description);
        String appId = platform.getAppId();
        if (appId != null) {
            app.put(INSTANCE, appId);
        }
        if (ROUTES.equals(type)) {
            if (isServiceMonitor) {
                throw new IllegalArgumentException("Routing table is not visible from a presence monitor - " +
                        "please try it from a regular application instance");
            } else {
                result.put(ROUTING, getRoutingTable());
                // add route substitution list if any
                Map<String, String> substitutions = PostOffice.getInstance().getRouteSubstitutionList();
                if (!substitutions.isEmpty()) {
                    result.put(ROUTE_SUBSTITUTION, substitutions);
                }
            }

        } else if (LIB.equals(type)) {
            result.put(LIBRARY, util.getLibraryList());

        } else if (ENV.equals(type)) {
            result.put(ENV, getEnv());
            result.put(ROUTING, getRegisteredServices());

        } else {
            // java VM information
            Map<String, Object> jvm = new HashMap<>();
            result.put(JVM, jvm);
            jvm.put(JAVA_VERSION, System.getProperty(JAVA_VERSION));
            jvm.put(JAVA_VM_VERSION, System.getProperty(JAVA_VM_VERSION));
            jvm.put(JAVA_RUNTIME_VERSION, System.getProperty(JAVA_RUNTIME_VERSION));
            // normalize result - substitute dot with underline
            normalize(jvm);
            // memory usage
            Runtime runtime = Runtime.getRuntime();
            NumberFormat number = NumberFormat.getInstance();
            long maxMemory = runtime.maxMemory();
            long allocatedMemory = runtime.totalMemory();
            long freeMemory = runtime.freeMemory();
            Map<String, Object> memory = new HashMap<>();
            result.put(MEMORY, memory);
            memory.put(MAX, number.format(maxMemory));
            memory.put(ALLOCATED, number.format(allocatedMemory));
            memory.put(FREE, number.format(freeMemory));
            /*
             * check streams resources if any
             */
            result.put(STREAMS, ObjectStreamIO.getStreamInfo());
            updateResult(SYSTEM_INFO, result);
            result.put(ORIGIN, platform.getOrigin());
            result.put(PERSONALITY, ServerPersonality.getInstance().getType().name());
            Map<String, Object> time = new HashMap<>();
            time.put(START, START_TIME);
            time.put(CURRENT, new Date());
            result.put(TIME, time);
        }
        return result;
    }

    private void updateResult(String service, Map<String, Object> result) {
        if (Platform.getInstance().hasRoute(service)) {
            try {
                EventEnvelope res = PostOffice.getInstance().request(service, 5000, new Kv(TYPE, QUERY));
                result.put(service, res.getBody());
            } catch (TimeoutException | IOException | AppException e) {
                result.put(ERROR, e.getMessage());
            }
        }
    }

    @SuppressWarnings("unchecked")
    private Map<String, Object> getRoutingTable() throws AppException, TimeoutException {
        Platform platform = Platform.getInstance();
        if (platform.hasRoute(ServiceDiscovery.SERVICE_QUERY) || platform.hasRoute(CLOUD_CONNECTOR)) {
            EventEnvelope response;
            try {
                response = PostOffice.getInstance().request(ServiceDiscovery.SERVICE_QUERY, 8000,
                        new Kv(ORIGIN, platform.getOrigin()), new Kv(TYPE, DOWNLOAD));
            } catch (IOException e) {
                // just return local routing table
                return getLocalPublicRouting();
            }
            if (response.getBody() instanceof Map) {
                return (Map<String, Object>) response.getBody();
            }
        } else {
            return getLocalPublicRouting();
        }
        return new HashMap<>();
    }

    private Map<String, Object> getLocalPublicRouting() {
        Map<String, Object> result = new HashMap<>();
        Map<String, ServiceDef> map = Platform.getInstance().getLocalRoutingTable();
        for (String route: map.keySet()) {
            ServiceDef service = map.get(route);
            if (!service.isPrivate()) {
                result.put(route, service.getCreated());
            }
        }
        return result;
    }

    private Map<String, List<String>> getRegisteredServices() {
        Map<String, List<String>> result = new HashMap<>();
        result.put("public", getLocalRoutingDetails(false));
        result.put("private", getLocalRoutingDetails(true));
        return result;
    }

    private List<String> getLocalRoutingDetails(boolean isPrivate) {
        List<String> result = new ArrayList<>();
        Map<String, ServiceDef> map = Platform.getInstance().getLocalRoutingTable();
        for (String route: map.keySet()) {
            ServiceDef service = map.get(route);
            if (service.isPrivate() == isPrivate) {
                ServiceQueue queue = service.getManager();
                long read = queue.getReadCounter();
                long write = queue.getWriteCounter();
                result.add(route + " (" + queue.getFreeWorkers() + "/" + service.getConcurrency() + ") " +
                            " r/w=" + read + "/" + write);
            }
        }
        if (result.size() > 1) {
            Collections.sort(result);
        }
        return result;
    }

    @SuppressWarnings("unchecked")
    private void normalize(Map<String, Object> data) {
        List<String> keys = new ArrayList<>(data.keySet());
        for (String k: keys) {
            Object o = data.get(k);
            if (o instanceof Map) {
                normalize((Map<String, Object>) o);
            }
            if (k.contains(".")) {
                data.put(k.replace('.', '_'), o);
                data.remove(k);
            }
        }
    }

    private Map<String, Object> getEnv() {
        Map<String, Object> result = new HashMap<>();
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        List<String> envVars = util.split(reader.getProperty(SHOW_ENV, ""), ", ");
        List<String> properties = util.split(reader.getProperty(SHOW_PROPERTIES, ""), ", ");
        List<String> missingVars = new ArrayList<>();
        Map<String, Object> eMap = new HashMap<>();
        if (!envVars.isEmpty()) {
            for (String key: envVars) {
                String v = System.getenv(key);
                if (v == null) {
                    missingVars.add(key);
                } else {
                    eMap.put(key, v);
                }
            }
        }
        result.put(SYSTEM_ENV, eMap);
        List<String> missingProp = new ArrayList<>();
        Map<String, Object> pMap = new HashMap<>();
        if (!properties.isEmpty()) {
            for (String key: properties) {
                String v = reader.getProperty(key);
                if (v == null) {
                    missingProp.add(key);
                } else {
                    pMap.put(key, v);
                }
            }
        }
        result.put(APP_PROPS, pMap);
        // any missing keys?
        Map<String, Object> missingKeys = new HashMap<>();
        if (!missingVars.isEmpty()) {
            missingKeys.put(SYSTEM_ENV, missingVars);
        }
        if (!missingProp.isEmpty()) {
            missingKeys.put(APP_PROPS, missingProp);
        }
        if (!missingKeys.isEmpty()) {
            result.put(MISSING, missingKeys);
        }
        return result;
    }

}