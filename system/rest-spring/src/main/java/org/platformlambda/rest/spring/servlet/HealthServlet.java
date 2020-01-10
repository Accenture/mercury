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

package org.platformlambda.rest.spring.servlet;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.TimeoutException;

@WebServlet("/health")
public class HealthServlet extends HttpServlet {
    private static final long serialVersionUID = 231981954669130491L;

    private static final Logger log = LoggerFactory.getLogger(HealthServlet.class);
    private static final ManagedCache cache = ManagedCache.createCache("health.info", 5000);
    private static final String TYPE = "type";
    private static final String QUERY = "query";
    private static final String INFO = "info";
    private static final String HEALTH = "health";
    private static final String NODE_INFO = "node.info";
    private static final String REQUIRED_SERVICES = "mandatory.health.dependencies";
    private static final String OPTIONAL_SERVICES = "optional.health.dependencies";
    private static final String ROUTE = "route";
    private static final String MESSAGE = "message";
    private static final String STATUS = "status";
    private static final String STATUS_CODE = "statusCode";
    private static final String REQUIRED = "required";
    private static final String UPSTREAM = "upstream";
    private static final String NOT_FOUND = "not found";
    private static final String PLEASE_CHECK = "Please check: ";
    private static List<String> requiredServices = new ArrayList<>();
    private static List<String> optionalServices = new ArrayList<>();
    private static boolean loaded = false;

    public HealthServlet() {
        if (!loaded) {
            loaded = true;
            AppConfigReader reader = AppConfigReader.getInstance();
            requiredServices = Utility.getInstance().split(reader.getProperty(REQUIRED_SERVICES, ""), ", ");
            if (requiredServices.isEmpty()) {
                log.info("Mandatory service dependencies - {}", requiredServices);
            }
            optionalServices = Utility.getInstance().split(reader.getProperty(OPTIONAL_SERVICES, ""), ", ");
            if (!optionalServices.isEmpty()) {
                log.info("Optional services dependencies - {}", optionalServices);
            }
        }
    }

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        boolean up = true;
        Map<String, Object> result = new HashMap<>(getBasicInfo());
        List<Map<String, Object>> upstream = new ArrayList<>();
        result.put(UPSTREAM, upstream);
        /*
         * Checking dependencies
         */
        checkServices(upstream, optionalServices, false);
        if (!checkServices(upstream, requiredServices, true)) {
            up = false;
        }
        result.put(STATUS, up? "UP" : "DOWN");
        response.setContentType("application/json");
        response.setCharacterEncoding("utf-8");
        response.getWriter().write(SimpleMapper.getInstance().getMapper().writeValueAsString(result));
    }

    @SuppressWarnings("unchecked")
    public static Map<String, Object> getBasicInfo() {
        Object o = cache.get(NODE_INFO);
        if (o instanceof Map) {
            return (Map<String, Object>) o;
        }
        // get platform specific node information
        AppConfigReader reader = AppConfigReader.getInstance();
        String nodeInfo = reader.getProperty(NODE_INFO, NODE_INFO);
        if (Platform.getInstance().hasRoute(nodeInfo)) {
            try {
                EventEnvelope response = PostOffice.getInstance().request(nodeInfo, 5000, new Kv(TYPE, QUERY));
                if (response.getBody() instanceof Map) {
                    cache.put(NODE_INFO, response.getBody());
                    return (Map<String, Object>) response.getBody();
                }
            } catch (IOException | TimeoutException | AppException e) {
                log.warn("Unable to obtain node info - {}", e.getMessage());
            }
        }
        return Collections.EMPTY_MAP;
    }

    @SuppressWarnings("unchecked")
    private boolean checkServices(List<Map<String, Object>> upstream, List<String> healthServices, boolean required) {
        PostOffice po = PostOffice.getInstance();
        boolean up = true;
        for (String route: healthServices) {
            Map<String, Object> m = new HashMap<>();
            m.put(ROUTE, route);
            m.put(REQUIRED, required);
            try {
                String key = INFO+"/"+route;
                if (!cache.exists(key)) {
                    EventEnvelope infoRes = po.request(route, 3000, new Kv(TYPE, INFO));
                    if (infoRes.getBody() instanceof Map) {
                        cache.put(key, infoRes.getBody());
                    }
                }
                Object info = cache.get(key);
                if (info instanceof Map) {
                    Map<String, Object> map = (Map<String, Object>) info;
                    for (String k : map.keySet()) {
                        m.put(k, map.get(k));
                    }
                }
                EventEnvelope res = po.request(route, 10000, new Kv(TYPE, HEALTH));
                if (res.getBody() instanceof String) {
                    m.put(STATUS_CODE, res.getStatus());
                    m.put(MESSAGE, res.getBody());
                    if (res.getStatus() != 200) {
                        up = false;
                    }
                }
            } catch (IOException e) {
                if (e.getMessage().contains(NOT_FOUND)) {
                    /*
                     * This means the configured health check service is not started.
                     * Just show a warning message to avoid blocking the health check.
                     */
                    m.put(STATUS_CODE, 200);
                    m.put(MESSAGE, PLEASE_CHECK+e.getMessage());
                } else {
                    m.put(STATUS_CODE, 500);
                    m.put(MESSAGE, e.getMessage());
                    up = false;
                }
            } catch (TimeoutException e) {
                m.put(STATUS_CODE, 408);
                m.put(MESSAGE, e.getMessage());
                up = false;
            } catch (AppException e) {
                m.put(STATUS_CODE, e.getStatus());
                m.put(MESSAGE, e.getMessage());
                up = false;
            }
            upstream.add(m);
        }
        return up;
    }

}
