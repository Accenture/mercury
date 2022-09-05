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

package org.platformlambda.automation.http;

import io.vertx.core.Handler;
import io.vertx.core.http.HttpServerRequest;
import io.vertx.core.http.HttpServerResponse;
import org.platformlambda.automation.MainModule;
import org.platformlambda.automation.config.RoutingEntry;
import org.platformlambda.automation.models.AssignedRoute;
import org.platformlambda.automation.models.AsyncContextHolder;
import org.platformlambda.automation.services.ServiceGateway;
import org.platformlambda.automation.util.SimpleHttpUtility;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Date;
import java.util.List;
import java.util.concurrent.ConcurrentMap;

public class HttpRequestHandler implements Handler<HttpServerRequest> {
    private static final Logger log = LoggerFactory.getLogger(HttpRequestHandler.class);

    private static final String ASYNC_HTTP_RESPONSE = MainModule.ASYNC_HTTP_RESPONSE;
    private static final String TYPE = "type";
    private static final String ACCEPT = "Accept";
    private static final String GET = "GET";
    private static final String POST = "POST";
    private static final String DATE = "Date";
    private static final String WS_PREFIX = "/ws/";
    private static final String APP_INSTANCE = "X-App-Instance";
    private static final String USER = "user";
    private static final String WHEN = "when";
    private static final String NOW = "now";
    private static final String LATER = "later";
    private static final String KEEP_ALIVE = "keep-alive";
    private static final String CONNECTION_HEADER = "Connection";
    private static final String REGISTRY = "system.service.registry";
    private static final String[] INFO_SERVICE = {"/info", "info"};
    private static final String[] INFO_LIB = {"/info/lib", "lib"};
    private static final String[] INFO_ROUTES = {"/info/routes", "routes"};
    private static final String[] HEALTH_SERVICE = {"/health", "health"};
    private static final String[] ENV_SERVICE = {"/env", "env"};
    private static final String[] LIVENESSPROBE = {"/livenessprobe", "livenessprobe"};
    private static final String[][] ADMIN_ENDPOINTS = {INFO_SERVICE, INFO_LIB, INFO_ROUTES,
                                                        HEALTH_SERVICE, ENV_SERVICE, LIVENESSPROBE};
    private static final long GRACE_PERIOD = 5000;

    private final ServiceGateway gateway;
    private final ConcurrentMap<String, AsyncContextHolder> contexts;
    private final boolean protectEndpoint;

    public HttpRequestHandler(ServiceGateway gateway) {
        AppConfigReader config = AppConfigReader.getInstance();
        this.gateway = gateway;
        this.contexts = gateway.getContexts();
        this.protectEndpoint = "true".equals(config.getProperty("protect.info.endpoints", "false"));
    }

    @Override
    public void handle(HttpServerRequest request) {
        Utility util = Utility.getInstance();
        HttpServerResponse response = request.response();
        response.putHeader(DATE, util.getHtmlDate(new Date()));
        String connectionType = request.getHeader(CONNECTION_HEADER);
        if (KEEP_ALIVE.equals(connectionType)) {
            response.putHeader(CONNECTION_HEADER, KEEP_ALIVE);
        }
        String url = request.path();
        String method = request.method().name();
        String requestId = util.getUuid();
        AsyncContextHolder holder = new AsyncContextHolder(request);
        String acceptContent = request.getHeader(ACCEPT);
        if (acceptContent != null) {
            holder.setAccept(acceptContent);
        }
        contexts.put(requestId, holder);
        if (GET.equals(method) && isAdminEndpoint(requestId, request, url)) {
            return;
        }
        if (POST.equals(method)) {
            if ("/shutdown".equals(url)) {
                shutdown(requestId, request);
                return;
            }
            if (url.equals("/suspend") || url.equals("/resume") ||
                    url.equals("/suspend/now") || url.equals("/suspend/later") ||
                    url.equals("/resume/now") || url.equals("/resume/later")) {
                suspendResume(requestId, request);
                return;
            }
        }
        RoutingEntry re = RoutingEntry.getInstance();
        AssignedRoute route = url.startsWith(WS_PREFIX)? null : re.getRouteInfo(method, url);
        int status = 200;
        String error = null;
        if (route == null) {
            status = 404;
            error = "Resource not found";
        } else if (route.info == null) {
            status = 405;
            error = "Method not allowed";
        } else {
            holder.setTimeout(route.info.timeoutSeconds * 1000L);
            if (POST.equals(method) && route.info.upload) {
                try {
                    request.setExpectMultipart(true);
                    request.pause();
                } catch (Exception e) {
                    status = 400;
                    error = e.getMessage();
                }
            }
        }
        gateway.handleEvent(route, requestId, status, error);
    }

    private boolean isAdminEndpoint(String requestId, HttpServerRequest request, String path) {
        for (String[] service: ADMIN_ENDPOINTS) {
            if (path.equals(service[0])) {
                return infoService(requestId, request, service[1]);
            }
        }
        return false;
    }

    private boolean infoService(String requestId, HttpServerRequest request, String type) {
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
        String origin = request.getHeader(APP_INSTANCE);
        if (origin == null) {
            if (protectEndpoint && !type.equals("livenessprobe") && !isLocalHost(request)) {
                return false;
            }
            origin = platform.getOrigin();
        }
        EventEnvelope event = new EventEnvelope().setHeader(TYPE, type);
        if (origin.equals(Platform.getInstance().getOrigin())) {
            event.setTo(PostOffice.ACTUATOR_SERVICES);
        } else {
            if (!po.exists(origin)) {
                httpUtil.sendResponse(requestId, request, 400, origin+" is not reachable");
                return true;
            }
            event.setTo(PostOffice.ACTUATOR_SERVICES+"@"+origin);
        }
        event.setCorrelationId(requestId).setReplyTo(ASYNC_HTTP_RESPONSE +"@"+ platform.getOrigin());
        try {
            po.send(event);
        } catch (IOException e) {
            log.warn("Unable to send request to {} - {}", PostOffice.ACTUATOR_SERVICES, e.getMessage());
        }
        return true;
    }

    private void shutdown(String requestId, HttpServerRequest request) {
        PostOffice po = PostOffice.getInstance();
        SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
        String origin = request.getHeader(APP_INSTANCE);
        if (origin == null) {
            httpUtil.sendResponse(requestId, request, 400, "Missing "+ APP_INSTANCE +" in request header");
            return;
        }
        EventEnvelope event = new EventEnvelope().setHeader(TYPE, "shutdown");
        if (origin.equals(Platform.getInstance().getOrigin())) {
            event.setTo(PostOffice.ACTUATOR_SERVICES);
        } else {
            if (!po.exists(origin)) {
                httpUtil.sendResponse(requestId, request, 400, origin+" is not reachable");
                return;
            }
            event.setTo(PostOffice.ACTUATOR_SERVICES+"@"+origin);
        }
        event.setHeader(USER, System.getProperty("user.name"));
        po.sendLater(event, new Date(System.currentTimeMillis() + GRACE_PERIOD));
        httpUtil.sendResponse(requestId, request, 200, origin+" will be shutdown in "+GRACE_PERIOD+" ms");
    }

    private void suspendResume(String requestId, HttpServerRequest request) {
        PostOffice po = PostOffice.getInstance();
        SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
        String origin = request.getHeader(APP_INSTANCE);
        if (origin == null) {
            httpUtil.sendResponse(requestId, request, 400, "Missing "+ APP_INSTANCE +" in request header");
            return;
        }
        Utility util = Utility.getInstance();
        String url = request.path();
        List<String> parts = util.split(url, "/");
        if (parts.size() == 1) {
            parts.add(NOW);
        }
        String type = parts.get(0);
        if (!po.exists(REGISTRY)) {
            httpUtil.sendResponse(requestId, request, 400, type+" not available in standalone mode");
            return;
        }
        EventEnvelope event = new EventEnvelope().setHeader(TYPE, type)
                .setHeader(USER, System.getProperty("user.name"));
        if (origin.equals(Platform.getInstance().getOrigin())) {
            event.setTo(PostOffice.ACTUATOR_SERVICES);
        } else {
            if (!po.exists(origin)) {
                httpUtil.sendResponse(requestId, request, 400, origin+" is not reachable");
                return;
            }
            event.setTo(PostOffice.ACTUATOR_SERVICES+"@"+origin);
        }
        String when = parts.get(1).equals(NOW) ? NOW : LATER;
        event.setHeader(WHEN, when);
        try {
            po.send(event);
        } catch (IOException e) {
            log.error("Unable to send {} request - {}", type, e.getMessage());
        }
        String message = type+" request sent to " + origin;
        if (LATER.equals(when)) {
            message += ". It will take effect in one minute.";
        }
        httpUtil.sendResponse(requestId, request, 200, message);
    }

    private boolean isLocalHost(HttpServerRequest request) {
        String host = request.getHeader("host");
        return host != null && host.contains("127.0.0.1");
    }

}
