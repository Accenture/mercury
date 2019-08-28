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

package org.platformlambda.automation.config;

import org.platformlambda.automation.models.CorsInfo;
import org.platformlambda.automation.models.HeaderInfo;
import org.platformlambda.automation.models.RouteInfo;
import org.platformlambda.automation.models.AssignedRoute;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class RoutingEntry {
    private static final Logger log = LoggerFactory.getLogger(RoutingEntry.class);

    private static final String REST = "rest";
    private static final String CORS = "cors";
    private static final String AUTH = "authentication";
    private static final String UPLOAD = "upload";
    private static final String THRESHOLD = "threshold";
    private static final String TRACING = "tracing";
    private static final String SERVICE = "service";
    private static final String METHODS = "methods";
    private static final String URL = "url";
    private static final String ID = "id";
    private static final String OPTIONS_METHOD = "OPTIONS";
    private static final String OPTIONS = "options";
    private static final String HEADERS = "headers";
    private static final String TIMEOUT = "timeout";
    private static final String ADD = "add";
    private static final String DROP = "drop";
    private static final String KEEP = "keep";
    private static final String ACCESS_CONTROL_PREFIX = "Access-Control-";
    private static final String[] VALID_METHODS = {"GET", "PUT", "POST", "DELETE", "HEAD", "OPTIONS"};
    private static final List<String> METHOD_LIST = Arrays.asList(VALID_METHODS);
    private static final int FIVE_MINUTES = 5 * 60;
    private static final int MIN_THRESHOLD = 5000;
    private static final int REG_THRESHOLD = 50000;
    private static final int MAX_THRESHOLD = 500000;

    private Map<String, RouteInfo> routes = new HashMap<>();
    private Map<String, Boolean> exactRoutes = new HashMap<>();
    // id -> {maps for options and headers}
    private Map<String, CorsInfo> cors = new HashMap<>();
    // id -> add, drop, keep
    private Map<String, HeaderInfo> headerTransform = new HashMap<>();
    private List<String> urlPaths = new ArrayList<>();

    private static RoutingEntry instance = new RoutingEntry();

    private RoutingEntry() {
        // singleton
    }

    public static RoutingEntry getInstance() {
        return instance;
    }

    public AssignedRoute getRouteInfo(String method, String url) {
        Utility util = Utility.getInstance();
        StringBuilder sb = new StringBuilder();
        List<String> input = util.split(url.toLowerCase(), "/");
        for (String p: input) {
            sb.append('/');
            sb.append(p.trim());
        }
        String nUrl = sb.toString();
        String key = method+":"+nUrl;
        if (exactRoutes.containsKey(nUrl)) {
            return new AssignedRoute(routes.get(key));
        } else {
            for (String u: urlPaths) {
                AssignedRoute info = getMatchedRoute(input, method, u);
                if (info != null) {
                    return info;
                }
            }
            return null;
        }
    }

    public HeaderInfo getHeaderInfo(String id) {
        return headerTransform.get(id);
    }

    public CorsInfo getCorsInfo(String id) {
        return cors.get(id);
    }

    private AssignedRoute getMatchedRoute(List<String> input, String method, String configured) {
        String key = method+":"+configured;
        AssignedRoute result = new AssignedRoute(routes.get(key));
        Utility util = Utility.getInstance();
        List<String> segments = util.split(configured, "/");
        if (matchRoute(input, segments, configured.endsWith("*"))) {
            addArguments(result, input, segments);
            return result;
        }
        return null;
    }

    private void addArguments(AssignedRoute info, List<String> input, List<String> configured) {
        for (int i=0; i < configured.size(); i++) {
            String configuredItem = configured.get(i);
            if (configuredItem.startsWith("{") && configuredItem.endsWith("}")) {
                info.setArgument(configuredItem.substring(1, configuredItem.length()-1), input.get(i));
            }
        }
    }

    private boolean matchRoute(List<String> input, List<String> configured, boolean wildcard) {
        if (wildcard) {
            if (configured.size() > input.size()) {
                return false;
            }
        } else {
            if (configured.size() != input.size()) {
                return false;
            }
        }
        for (int i=0; i < configured.size(); i++) {
            String configuredItem = configured.get(i);
            if (configuredItem.startsWith("{") && configuredItem.endsWith("}")) {
                continue;
            }
            if (configuredItem.equals("*")) {
                continue;
            }
            String inputItem = input.get(i);
            if (configuredItem.endsWith("*")) {
                String prefix = configuredItem.substring(0, configuredItem.length()-1);
                if (inputItem.startsWith(prefix)) {
                    continue;
                }
            }
            if (inputItem.equals(configuredItem)) {
                continue;
            }
            return false;
        }
        return true;
    }

    @SuppressWarnings("unchecked")
    public void load(Map<String, Object> config) {
        if (config.containsKey(HEADERS)) {
            Object headerList = config.get(HEADERS);
            if (headerList instanceof List) {
                if (isMap((List<Object>) headerList)) {
                    loadHeaderTransform((List<Map<String, Object>>) headerList);
                } else {
                    log.error("'headers' section must be a list of configuration where each entry is a map of id, add, drop and keep");
                }

            } else {
                log.error("'headers' section must contain a list of configuration. Actual: {}", headerList.getClass().getSimpleName());
            }
        }
        if (config.containsKey(CORS)) {
            Object corsList = config.get(CORS);
            if (corsList instanceof List) {
                if (isMap((List<Object>) corsList)) {
                    loadCors((List<Map<String, Object>>) corsList);
                } else {
                    log.error("'cors' section must be a list of Access-Control configuration where each entry is a map of id, options and headers");
                }

            } else {
                log.error("'cors' section must contain a list of Access-Control configuration. Actual: {}", corsList.getClass().getSimpleName());
            }
        }
        if (config.containsKey(REST)) {
            Object rest = config.get(REST);
            if (rest instanceof List) {
                if (isMap((List<Object>) rest)) {
                    loadRest((List<Map<String, Object>>) rest);
                } else {
                    log.error("'rest' section must be a list of configuration where each endpoint is a map of url, service, methods, timeout, etc.");
                }

            } else {
                log.error("'rest' section must contain a list of configuration. Actual: {}", rest.getClass().getSimpleName());
            }
            List<String> exact = new ArrayList<>();
            for (String r: exactRoutes.keySet()) {
                if (!exact.contains(r)) {
                    exact.add(r);
                }
            }
            if (exact.size() > 1) {
                Collections.sort(exact);
            }
            if (!exact.isEmpty()) {
                log.info("Number of exact URLs = {}", exact.size());
            }
            // sort URLs for easy parsing
            if (!routes.isEmpty()) {
                for (String r: routes.keySet()) {
                    int colon = r.indexOf(':');
                    if (colon > 0) {
                        String urlOnly = r.substring(colon+1);
                        if (!exactRoutes.containsKey(urlOnly) && !urlPaths.contains(urlOnly)) {
                            urlPaths.add(urlOnly);
                        }
                    }
                }
            }
            if (urlPaths.size() > 1) {
                Collections.sort(urlPaths);
            }
            if (!urlPaths.isEmpty()) {
                log.info("Number of wildcard URLs = {}", urlPaths.size());
            }
        }
    }

    private boolean isMap(List<Object> list) {
        for (Object o: list) {
            if (!(o instanceof Map)) {
                return false;
            }
        }
        return true;
    }

    private void loadRest(List<Map<String, Object>> config) {
        for (Map<String, Object> entry: config) {
            if (entry.containsKey(SERVICE) && entry.containsKey(METHODS) && entry.containsKey(URL)
                    && entry.get(URL) instanceof String) {
                String url = (String) entry.get(URL);
                if (url.contains("{") || url.contains("}") || url.contains("*")) {
                    loadRestEntry(entry, false);
                } else {
                    loadRestEntry(entry, true);
                }
            } else {
                log.error("Skipping invalid REST entry {}", entry);
            }
        }
    }


    @SuppressWarnings("unchecked")
    private void loadRestEntry(Map<String, Object> entry, boolean exact) {
        Utility util = Utility.getInstance();
        RouteInfo info = new RouteInfo();
        if (entry.get(SERVICE) instanceof String && entry.get(METHODS) instanceof List) {
            // default multipart label for upload is "file"
            if (entry.containsKey(UPLOAD) && entry.get(UPLOAD) instanceof String) {
                String upload = ((String) entry.get(UPLOAD)).trim();
                if (upload.length() > 1) {
                    info.upload = upload;
                }
            }
            if (entry.containsKey(AUTH)) {
                String auth = entry.get(AUTH).toString().trim();
                if (!util.validServiceName(auth)) {
                    log.error("Skipping entry with invalid authentication service name {}", entry);
                    return;
                } else {
                    info.authService = auth;
                }
            }
            if (entry.containsKey(THRESHOLD)) {
                int value = util.str2int(entry.get(THRESHOLD).toString());
                if (value < MIN_THRESHOLD) {
                    value = MIN_THRESHOLD;
                    log.warn("{} - threshold set to {} because {} is too small", entry.get(URL), REG_THRESHOLD, value);
                }
                if (value > MAX_THRESHOLD) {
                    value = MAX_THRESHOLD;
                    log.warn("{} - threshold set to {} because {} is too big", entry.get(URL), MAX_THRESHOLD, value);
                }
                info.threshold = value;
            }
            if (entry.containsKey(TRACING)) {
                if ("true".equalsIgnoreCase(entry.get(TRACING).toString())) {
                    info.tracing = true;
                }
            }
            String service = (String) entry.get(SERVICE);
            List<String> methods = (List<String>) entry.get(METHODS);
            String url = (String) entry.get(URL);
            int timeout = -1;
            if (entry.containsKey(TIMEOUT)) {
                Object t = entry.get(TIMEOUT);
                if (t instanceof String) {
                    String v = (String) t;
                    if (v.endsWith("s")) {
                        String vt = v.substring(0, v.length()-1);
                        if (util.isDigits(vt)) {
                            timeout = util.str2int(vt);
                        }
                    }
                }
                if (t instanceof Integer) {
                    timeout = (Integer) t;
                }
            }
            if (timeout == -1) {
                log.warn("Default timeout of 30s is used");
            } else if (timeout > FIVE_MINUTES ) {
                log.warn("Default timeout of 30s is used because {}s is more than 5 minutes", timeout);
            } else {
                info.timeoutSeconds = timeout;
            }
            if (entry.containsKey(CORS)) {
                String id = entry.get(CORS).toString();
                if (cors.containsKey(id)) {
                    info.corsId = id;
                } else {
                    log.error("Skipping invalid entry because cors ID {} is not found, {}", id, entry);
                    return;
                }
            }
            if (entry.containsKey(HEADERS)) {
                String id = entry.get(HEADERS).toString();
                if (headerTransform.containsKey(id)) {
                    info.transformId = id;
                } else {
                    log.error("Skipping invalid entry because headers ID {} is not found, {}", id, entry);
                    return;
                }
            }
            if (!util.validServiceName(service)) {
                log.error("Skipping entry with invalid service name {}", entry);
            } else {
                info.service = service;
                if (validMethods(methods)) {
                    info.methods = methods;
                    if (exact) {
                        exactRoutes.put(url, true);
                    }
                    String nUrl = getUrl(url, exact);
                    if (nUrl == null) {
                        log.error("Skipping invalid entry {}", entry);
                    } else {
                        info.url = nUrl;
                        List<String> allMethods = new ArrayList<>(methods);
                        if (!allMethods.contains(OPTIONS_METHOD)) {
                            allMethods.add(OPTIONS_METHOD);
                        }
                        for (String m: allMethods) {
                            String key = m+":"+nUrl;
                            if (routes.containsKey(key)) {
                                if (!m.equals(OPTIONS_METHOD)) {
                                    log.error("Skipping duplicated method and URL {}", key);
                                }
                            } else {
                                routes.put(key, info);
                                // OPTIONS method is not traced
                                if (m.equals(OPTIONS_METHOD)) {
                                    log.info("{} {} -> {}, timeout={}s", m, nUrl, service, info.timeoutSeconds);
                                } else {
                                    log.info("{} {} -> {}, timeout={}s, tracing={}", m, nUrl, service,
                                            info.timeoutSeconds, info.tracing);
                                }
                            }
                        }
                    }
                } else {
                    log.error("Skipping entry with invalid method {}", entry);
                }
            }

        } else {
            log.error("Skipping invalid REST entry {}", entry);
        }
    }

    private boolean validMethods(List<String> methods) {
        if (methods.isEmpty()) {
            return false;
        }
        for (String m: methods) {
            if (!METHOD_LIST.contains(m)) {
                return false;
            }
        }
        return true;
    }

    private String getUrl(String url, boolean exact) {
        StringBuilder sb = new StringBuilder();
        List<String> parts = Utility.getInstance().split(url.toLowerCase(), "/");
        for (String p: parts) {
            String s = p.trim();
            sb.append('/');
            if (exact) {
                sb.append(s);
            } else {
                if (s.contains("{") || s.contains("}")) {
                    if (s.contains("*")) {
                        log.error("wildcard url segment must not mix arguments with *, actual: {}", s);
                        return null;
                    }
                    if (!validArgument(s)) {
                        log.error("Argument url segment must be enclosed curly brackets, actual: {}", s);
                        return null;
                    }
                }
                if (s.contains("*")) {
                    if (!validWildcard(s)) {
                        log.error("wildcard url segment must ends with *, actual: {}", url);
                        return null;
                    }
                }
                sb.append(s);
            }
        }
        return sb.toString();
    }

    private boolean validArgument(String arg) {
        if (arg.startsWith("{") && arg.endsWith("}")) {
            String v = arg.substring(1, arg.length()-1);
            if (v.length() == 0) {
                return false;
            } else if (v.contains("{") || v.contains("}")) {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    private boolean validWildcard(String wildcard) {
        if (wildcard.equals("*")) {
            return true;
        }
        if (!wildcard.endsWith("*")) {
            return false;
        }
        List<String> stars = Utility.getInstance().split(wildcard, "*");
        return stars.size() == 1;
    }

    private void loadCors(List<Map<String, Object>> config) {
        for (Map<String, Object> entry: config) {
            if (entry.containsKey(ID) && entry.containsKey(OPTIONS) && entry.containsKey(HEADERS)
                    && entry.get(ID) instanceof String) {
                loadCorsEntry(entry);
            } else {
                log.error("Skipping invalid CORS definition {}", entry);
            }
        }
    }

    @SuppressWarnings("unchecked")
    private void loadCorsEntry(Map<String, Object> entry) {
        if (entry.get(OPTIONS) instanceof List && entry.get(HEADERS) instanceof List) {
            String id = (String) entry.get(ID);
            List<Object> options = (List<Object>) entry.get(OPTIONS);
            List<Object> headers = (List<Object>) entry.get(HEADERS);
            if (validCorsList(options) && validCorsList(headers)) {
                CorsInfo info = new CorsInfo(id);
                for (Object o: options) {
                    info.addOption(o.toString());
                }
                for (Object h: headers) {
                    info.addHeader(h.toString());
                }
                cors.put(id, info);
                log.info("Loaded CORS definition with id {}", id);

            } else {
                log.error("Skipping invalid CORS entry {}", entry);
            }
        }
    }

    private boolean validCorsList(List<Object> list) {
        for (Object o: list) {
            if (o instanceof String) {
                if (!validCorsElement((String) o)) {
                    return false;
                }
            } else {
                log.error("Cors header must be a list of strings, actual: {}", list);
                return false;
            }
        }
        return true;
    }

    private boolean validCorsElement(String element) {
        if (!element.startsWith(ACCESS_CONTROL_PREFIX)) {
            log.error("Cors header must start with {}, actual: {}", ACCESS_CONTROL_PREFIX, element);
            return false;
        }
        int colon = element.indexOf(':');
        if (colon == -1) {
            log.error("Cors header must contain key-value separated by a colon, actual: {}", element);
            return false;
        }
        String value = element.substring(colon+1).trim();
        if (value.length() == 0) {
            log.error("Missing value in Cors header {}, actual: {}", element.substring(0, colon), element);
            return false;
        }
        return true;
    }

    private void loadHeaderTransform(List<Map<String, Object>> config) {
        for (Map<String, Object> entry: config) {
            if (entry.containsKey(ID) &&
                    (entry.containsKey(ADD) || entry.containsKey(DROP) || entry.containsKey(KEEP))) {
                loadHeaderEntry(entry);
            } else {
                log.error("Skipping invalid HEADERS definition - it must contain id and one or more of add/drop/keep {}", entry);
            }
        }
    }

    @SuppressWarnings("unchecked")
    private void loadHeaderEntry(Map<String, Object> entry) {
        String id = (String) entry.get(ID);
        HeaderInfo info = new HeaderInfo(id);
        if (entry.get(ADD) instanceof List) {
            List<Object> items = (List<Object>) entry.get(ADD);
            for (Object o: items) {
                String keyValue = o.toString();
                int colon = keyValue.indexOf(':');
                if (colon > 0) {
                    info.addHeader(keyValue.substring(0, colon).trim().toLowerCase(), keyValue.substring(colon+1).trim());
                } else {
                    log.warn("Skipping invalid ADD entry {} for HEADERS definition {}", o, id);
                }
            }
        }
        if (entry.get(DROP) instanceof List) {
            List<Object> items = (List<Object>) entry.get(DROP);
            for (Object o: items) {
                info.drop(o.toString().toLowerCase());
            }
        }
        if (entry.get(KEEP) instanceof List) {
            List<Object> items = (List<Object>) entry.get(KEEP);
            for (Object o: items) {
                info.keep(o.toString().toLowerCase());
            }
        }
        headerTransform.put(id, info);
        log.info("Loaded HEADERS definition with id {}", id);
    }

}
