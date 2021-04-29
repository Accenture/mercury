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

package org.platformlambda.servlets;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.VersionInfo;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;

import javax.servlet.ServletContext;
import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.text.NumberFormat;
import java.util.*;
import java.util.concurrent.TimeoutException;

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
    private static final String APP_ID = "app_id";
    private static final String PERSONALITY = "personality";
    private static final String ROUTING = "routing";
    private static final String LIST_ROUTES = "routes";
    private static final String LIB = "lib";
    private static final String DOWNLOAD = "download";
    private static final String CLOUD_CONNECTOR = "cloud.connector";
    private static final String LIBRARY = "library";
    private static final String DEPLOYMENT = "deployment";
    private static final String ROUTE_SUBSTITUTION = "route_substitution";
    private static final int TOPIC_LEN = Utility.getInstance().getDateUuid().length();
    private static final String META_INF = "/META-INF/";
    private static final String MANIFEST = "/META-INF/MANIFEST.MF";
    private static final String IMPL_VERSION = "Implementation-Version";
    private static final String WEB_INF = "/WEB-INF/";
    private static final String[] JAR_PATHS = { "/WEB-INF/lib/", "/WEB-INF/lib-provided/" };
    private static final String JAR = ".jar";
    private static final String TIME = "time";
    private static final String START = "start";
    private static final String CURRENT = "current";
    private static final Date START_TIME = new Date();
    private static final Object ORDERLY_SCAN = new Object[0];
    private static boolean loaded = false;
    private static String deploymentMode = "unknown";
    private static List<String> libraryList = new ArrayList<>();
    private static Boolean isServiceMonitor;

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        Platform platform = Platform.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        String description = config.getProperty(APP_DESCRIPTION, platform.getName());
        if (isServiceMonitor == null) {
            isServiceMonitor = "true".equals(config.getProperty("service.monitor", "false"));
        }
        if (!loaded) {
            scanLibInfo(request.getServletContext());
            loaded = true;
        }
        Map<String, Object> result = new HashMap<>();
        Map<String, Object> app = new HashMap<>();
        Utility util = Utility.getInstance();
        VersionInfo info = util.getVersionInfo();
        result.put(APP, app);
        result.put(DEPLOYMENT, deploymentMode);
        /*
         * When running inside IDE, there are no information about libraries
         * so it is better to take the application name from the application.properties
         */
        app.put(NAME, info.getArtifactId());
        app.put(VERSION, info.getVersion());
        app.put(DESCRIPTION, description);
        String appId = platform.getAppId();
        if (appId != null) {
            app.put(APP_ID, appId);
        }
        List<String> pathElements = util.split(request.getPathInfo(), "/");
        /*
         * add routing table information if any
         */
        if (pathElements.size() == 1 && LIST_ROUTES.equals(pathElements.get(0))) {
            if (isServiceMonitor) {
                response.sendError(400, "Routing table is not shown from a presence monitor");
                return;
            }
            String node = request.getParameter(ORIGIN);
            if (node != null) {
                if (isServiceMonitor) {
                    response.sendError(400, "Remote routing table is not shown when using Presence Monitor");
                    return;
                }
                if (!node.equals(platform.getOrigin())) {
                    showRemoteRouting(node, response);
                    return;
                }
            }
            try {
                result.put(ROUTING, getRoutingTable());
                // add route substitution list if any
                Map<String, String> substitutions = PostOffice.getInstance().getRouteSubstitutionList();
                if (!substitutions.isEmpty()) {
                    result.put(ROUTE_SUBSTITUTION, substitutions);
                }

            } catch (TimeoutException e) {
                sendError(response, request.getRequestURI(), 408, e.getMessage());
                return;
            } catch (AppException e) {
                sendError(response, request.getRequestURI(), e.getStatus(), e.getMessage());
                return;
            }
        } else if (pathElements.size() == 1 && LIB.equals(pathElements.get(0))) {
            result.put(LIBRARY, libraryList);

        } else if (pathElements.isEmpty()) {
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
            updateResult(PostOffice.STREAM_MANAGER, result);
            updateResult(SYSTEM_INFO, result);
            result.put(ORIGIN, platform.getOrigin());
            result.put(PERSONALITY, ServerPersonality.getInstance().getType().name());
            Map<String, Object> time = new HashMap<>();
            time.put(START, START_TIME);
            time.put(CURRENT, new Date());
            result.put(TIME, time);
        } else {
            sendError(response, request.getRequestURI(), 404, "Not found");
            return;
        }
        // send result
        response.setContentType("application/json");
        response.setCharacterEncoding("utf-8");
        response.getWriter().write(SimpleMapper.getInstance().getMapper().writeValueAsString(result));
    }

    private List<String> getJarList(ServletContext context, String path) {
        List<String> list = new ArrayList<>();
        Set<String> filenames = context.getResourcePaths(path);
        if (filenames != null) {
            for (String name : filenames) {
                String jar = name.startsWith(path)? name.substring(path.length()) : name;
                list.add(jar.endsWith(JAR) ? jar.substring(0, jar.length() - JAR.length()) : jar);
            }
        }
        return list;
    }

    private void scanLibInfo(ServletContext context) {
        synchronized (ORDERLY_SCAN) {
            Utility util = Utility.getInstance();
            List<String> list = util.getLibraryList();
            Set<String> resPaths = context.getResourcePaths("/");
            if (list.isEmpty()) {
                if (resPaths == null) {
                    deploymentMode = "IDE";
                } else {
                    deploymentMode = "webapp";
                    // get library list from webapp container
                    if (resPaths.contains(WEB_INF)) {
                        for (String p : JAR_PATHS) {
                            list.addAll(getJarList(context, p));
                        }
                    }
                    // get version number from MANIFEST.MF
                    if (resPaths.contains(META_INF)) {
                        VersionInfo info = util.getVersionInfo();
                        String version = getVersion(context);
                        if (version != null) {
                            info.setVersion(version);
                        }
                    }
                }
            } else {
                // get library list from JAR or WAR file
                deploymentMode = resPaths == null ? "jar" : "war";
            }
            /*
             * Sort the library names in ascending order
             */
            if (list.size() > 1) {
                Collections.sort(list);
            }
            List<String> result = new ArrayList<>();
            if (!list.isEmpty()) {
                int size = list.size();
                int n = 0;
                for (String f : list) {
                    result.add(util.zeroFill(++n, size) + ". " + f);
                }
                result.add("total: " + list.size());
            }
            libraryList = result;
        }
    }

    private String getVersion(ServletContext context) {
        InputStream in = context.getResourceAsStream(MANIFEST);
        if (in != null) {
            Properties p = new Properties();
            try {
                p.load(new ByteArrayInputStream(Utility.getInstance().stream2bytes(in)));
                return p.getProperty(IMPL_VERSION);
            } catch (IOException e) {
                // nothing we can do
            }
        }
        return null;
    }

    private void showRemoteRouting(String node, HttpServletResponse response) throws IOException {
        if (regularTopicFormat(node)) {
            try {
                Map<String, Object> result = new HashMap<>();
                result.put(TYPE, "remote");
                result.put(ROUTING, getRemoteRouting(node));
                result.put(TIME, new Date());
                result.put(ORIGIN, Platform.getInstance().getOrigin());
                // send result
                response.setContentType("application/json");
                response.setCharacterEncoding("utf-8");
                response.getWriter().write(SimpleMapper.getInstance().getMapper().writeValueAsString(result));

            } catch (AppException e) {
                response.sendError(e.getStatus(), e.getMessage());
            } catch (TimeoutException e) {
                response.sendError(408, e.getMessage());
            }
        } else {
            response.sendError(400, "Invalid application instance format (origin)");
        }
    }

    @SuppressWarnings("unchecked")
    private Map<String, Object> getRemoteRouting(String node) throws AppException, TimeoutException, IOException {
        Platform platform = Platform.getInstance();
        if (platform.hasRoute(ServiceDiscovery.SERVICE_QUERY) || platform.hasRoute(CLOUD_CONNECTOR)) {
            EventEnvelope response = PostOffice.getInstance().request(ServiceDiscovery.SERVICE_QUERY+"@"+node,
                    8000, new Kv(ORIGIN, platform.getOrigin()), new Kv(TYPE, DOWNLOAD));
            if (response.getBody() instanceof Map) {
                return (Map<String, Object>) response.getBody();
            }
        }
        return new HashMap<>();
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
                response = PostOffice.getInstance().request(ServiceDiscovery.SERVICE_QUERY, 8000,
                                new Kv(ORIGIN, platform.getOrigin()), new Kv(TYPE, DOWNLOAD));
            } catch (IOException e) {
                // just return local routing table
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

    /**
     * Validate a topic ID for an application instance
     *
     * @param topic in format of yyyymmdd uuid
     * @return true if valid
     */
    private boolean regularTopicFormat(String topic) {
        if (topic.length() != TOPIC_LEN) {
            return false;
        }
        // first 8 digits is a date stamp
        String uuid = topic.substring(8);
        if (!Utility.getInstance().isDigits(topic.substring(0, 8))) {
            return false;
        }
        for (int i=0; i < uuid.length(); i++) {
            if (uuid.charAt(i) >= '0' && uuid.charAt(i) <= '9') continue;
            if (uuid.charAt(i) >= 'a' && uuid.charAt(i) <= 'f') continue;
            return false;
        }
        return true;
    }

}