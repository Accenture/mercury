/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.rest.spring.servlet;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.VersionInfo;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDef;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.text.NumberFormat;
import java.util.*;
import java.util.concurrent.TimeoutException;

import static org.platformlambda.core.system.Platform.STREAM_MANAGER;

@WebServlet("/info/*")
public class InfoServlet extends HttpServlet {
	private static final long serialVersionUID = 376901501172978505L;
	private static final String ERROR = "error";
    private static final String SYSTEM_INFO = "additional.info";
    private static final String JAVA_VERSION = "java.version";
    private static final String JAVA_VM_VERSION = "java.vm.version";
    private static final String JAVA_RUNTIME_VERSION = "java.runtime.version";
    private static final String TYPE = "type";
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
    private static final String ROUTING = "routing";
    private static final String LIST_ROUTES = "/routes";
    private static final String LIB = "/lib";
    private static final String DOWNLOAD = "download";
    private static final String CLOUD_CONNECTOR = "cloud.connector";
    private static final String TIME = "time";
    private static final String LIBRARY = "library";

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        Platform platform = Platform.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        String description = config.getProperty(APP_DESCRIPTION, platform.getName());

        Map<String, Object> result = new HashMap<>();
        Map<String, Object> app = new HashMap<>();
        VersionInfo info = Utility.getInstance().getVersionInfo();
        result.put(APP, app);
        /*
         * When running inside IDE, there are no information about libraries
         * so it is better to take the application name from the application.properties
         */
        app.put(NAME, info.getArtifactId());
        app.put(VERSION, info.getVersion());
        app.put(DESCRIPTION, description);
        /*
         * add routing table information if any
         */
        if (LIST_ROUTES.equals(request.getPathInfo())) {
            try {
                result.put(ROUTING, getRoutingTable());
            } catch (TimeoutException e) {
                sendError(response, request.getRequestURI(), 408, e.getMessage());
                return;
            } catch (AppException e) {
                sendError(response, request.getRequestURI(), e.getStatus(), e.getMessage());
                return;
            }
        } else if (LIB.equals(request.getPathInfo())) {
            result.put(LIBRARY, Utility.getInstance().getLibraryList());

        } else if (request.getPathInfo() == null) {
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
            updateResult(STREAM_MANAGER, result);
            updateResult(SYSTEM_INFO, result);
            result.put(TIME, new Date());
            result.put(ORIGIN, platform.getOrigin());
        } else {
            sendError(response, request.getRequestURI(), 404, "Not found");
            return;
        }
        // send result
        response.setContentType("application/json");
        response.setCharacterEncoding("utf-8");
        response.getWriter().write(SimpleMapper.getInstance().getMapper().writeValueAsString(result));
    }

    private void sendError(HttpServletResponse response, String path, int status, String message) throws IOException {
        Map<String, Object> result = new HashMap<>();
        result.put("type", "error");
        result.put("status", status);
        result.put("message", message);
        result.put("path", path);
        response.setStatus(status);
        response.setContentType("application/json");
        response.setCharacterEncoding("utf-8");
        response.getWriter().write(SimpleMapper.getInstance().getMapper().writeValueAsString(result));
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
                response = PostOffice.getInstance().request(ServiceDiscovery.SERVICE_QUERY, 8000, new Kv(TYPE, DOWNLOAD));
            } catch (IOException e) {
                // event node is down - just return local routing table
                return getLocalRouting();
            }
            if (response.getBody() instanceof Map) {
                return (Map<String, Object>) response.getBody();
            }
        } else {
            return getLocalRouting();
        }
        return new HashMap<>();
    }

    private Map<String, Object> getLocalRouting() {
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

}