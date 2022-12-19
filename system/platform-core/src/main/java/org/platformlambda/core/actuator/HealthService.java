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

package org.platformlambda.core.actuator;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class HealthService implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(HealthService.class);

    private static final ManagedCache cache = ManagedCache.createCache("health.info", 5000);
    private static final String TYPE = "type";
    private static final String INFO = "info";
    private static final String HEALTH = "health";
    private static final String REQUIRED_SERVICES = "mandatory.health.dependencies";
    private static final String OPTIONAL_SERVICES = "optional.health.dependencies";
    private static final String ROUTE = "route";
    private static final String MESSAGE = "message";
    private static final String STATUS = "status";
    private static final String ORIGIN = "origin";
    private static final String NAME = "name";
    private static final String STATUS_CODE = "status_code";
    private static final String REQUIRED = "required";
    private static final String UPSTREAM = "upstream";
    private static final String NOT_FOUND = "not found";
    private static final String PLEASE_CHECK = "Please check - ";
    private final List<String> requiredServices;
    private final List<String> optionalServices;

    public HealthService() {
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

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        Platform platform = Platform.getInstance();
        boolean up = true;
        Map<String, Object> result = new HashMap<>();
        /*
         * Checking dependencies
         */
        List<Map<String, Object>> upstream = new ArrayList<>();
        checkServices(upstream, optionalServices, false);
        if (!checkServices(upstream, requiredServices, true)) {
            up = false;
        }
        // checkServices will update the "upstream" service list
        result.put(UPSTREAM, upstream);
        if (upstream.isEmpty()) {
            result.put(MESSAGE, "Did you forget to define "+REQUIRED_SERVICES+" or "+OPTIONAL_SERVICES);
        }
        result.put(STATUS, up? "UP" : "DOWN");
        result.put(ORIGIN, platform.getOrigin());
        result.put(NAME, platform.getName());
        return result;
    }

    @SuppressWarnings("unchecked")
    private boolean checkServices(List<Map<String, Object>> upstream, List<String> healthServices, boolean required) {
        PostOffice po = PostOffice.getInstance();
        boolean up = true;
        for (String route: healthServices) {
            Map<String, Object> m = new HashMap<>();
            m.put(ROUTE, route);
            m.put(REQUIRED, required);
            upstream.add(m);
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
                up = false;
                if (e.getMessage().contains(NOT_FOUND)) {
                    m.put(STATUS_CODE, 404);
                    m.put(MESSAGE, PLEASE_CHECK+e.getMessage());
                } else {
                    m.put(STATUS_CODE, 500);
                    m.put(MESSAGE, e.getMessage());
                }
            } catch (TimeoutException e) {
                up = false;
                m.put(STATUS_CODE, 408);
                m.put(MESSAGE, e.getMessage());
            } catch (AppException e) {
                up = false;
                m.put(STATUS_CODE, e.getStatus());
                m.put(MESSAGE, e.getMessage());
            }
        }
        return up;
    }

}
