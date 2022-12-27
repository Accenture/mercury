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

package org.platformlambda.core.websocket.server;

import io.vertx.core.Handler;
import io.vertx.core.http.HttpServerRequest;
import io.vertx.core.http.HttpServerResponse;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import io.vertx.core.buffer.Buffer;

import java.util.*;

/**
 * HTTP admin endpoints for info, health, env, shutdown, suspend and resume
 * to be available with the same port when websocket server is deployed.
 * i.e. when user defined websocket server using WebSocketService is found.
 */
public class MinimalistHttpHandler implements Handler<HttpServerRequest> {

    private static final Logger log = LoggerFactory.getLogger(MinimalistHttpHandler.class);

    private static final String TYPE = "type";
    private static final String GET = "GET";
    private static final String POST = "POST";
    private static final String DATE = "Date";
    private static final String APP_INSTANCE = "X-App-Instance";
    private static final String USER = "user";
    private static final String WHEN = "when";
    private static final String NOW = "now";
    private static final String LATER = "later";
    private static final String CONTENT_TYPE = "Content-Type";
    private static final String CONTENT_LENGTH = "Content-Length";
    private static final String JSON = "application/json";
    private static final String TEXT_PLAIN = "text/plain";
    private static final String STATUS = "status";
    private static final String MESSAGE = "message";
    private static final String SHUTDOWN = "shutdown";
    private static final String PATH = "path";
    private static final String KEEP_ALIVE = "keep-alive";
    private static final String CONNECTION_HEADER = "Connection";
    private static final String REGISTRY = "system.service.registry";
    private static final String[] INFO_SERVICE = {"/info", "info"};
    private static final String[] INFO_LIB = {"/info/lib", "lib"};
    private static final String[] INFO_ROUTES = {"/info/routes", "routes"};
    private static final String[] HEALTH_SERVICE = {"/health", "health"};
    private static final String[] ENV_SERVICE = {"/env", "env"};
    private static final String[] LIVENESSPROBE = {"/livenessprobe", "livenessprobe"};
    public static final String[][] ADMIN_ENDPOINTS = {INFO_SERVICE, INFO_LIB, INFO_ROUTES,
            HEALTH_SERVICE, ENV_SERVICE, LIVENESSPROBE};
    private static final long GRACE_PERIOD = 5000;

    @Override
    public void handle(HttpServerRequest request) {
        PostOffice po = PostOffice.getInstance();
        Utility util = Utility.getInstance();
        HttpServerResponse response = request.response();
        response.putHeader(DATE, util.getHtmlDate(new Date()));
        String connectionType = request.getHeader(CONNECTION_HEADER);
        if (KEEP_ALIVE.equals(connectionType)) {
            response.putHeader(CONNECTION_HEADER, KEEP_ALIVE);
        }
        response.putHeader(CONTENT_TYPE, JSON);
        String path = util.getUrlDecodedPath(request.path());
        final String uri = path;
        String method = request.method().name();
        String origin = request.getHeader(APP_INSTANCE);
        if (origin != null && !po.exists(origin)) {
            sendError(response, uri, 404, origin+" is not reachable");
            return;
        }
        boolean processed = false;
        if (GET.equals(method)) {
            String type = getAdminEndpointType(uri);
            if (type != null) {
                EventEnvelope event = new EventEnvelope().setHeader(TYPE, type);
                event.setTo(origin != null? PostOffice.ACTUATOR_SERVICES+"@"+origin : PostOffice.ACTUATOR_SERVICES);
                try {
                    po.asyncRequest(event, 30000)
                        .onSuccess(result -> {
                            Map<String, String> headers = result.getHeaders();
                            if (result.hasError()) {
                                sendError(response, uri, result.getStatus(), result.getRawBody());
                            } else {
                                final byte[] b;
                                if (TEXT_PLAIN.equals(headers.get(CONTENT_TYPE)) &&
                                        result.getRawBody() instanceof String) {
                                    response.putHeader(CONTENT_TYPE, TEXT_PLAIN);
                                    b = util.getUTF((String) result.getRawBody());
                                } else {
                                    b = SimpleMapper.getInstance().getMapper().writeValueAsBytes(result.getRawBody());
                                }
                                response.putHeader(CONTENT_LENGTH, String.valueOf(b.length));
                                response.setStatusCode(result.getStatus());
                                response.write(Buffer.buffer(b));
                                response.end();
                            }
                        })
                        .onFailure(e -> sendError(response, uri, 408, e.getMessage()));
                    processed = true;
                } catch (IOException e) {
                    log.error("Unable to load {} - {}", uri, e.getMessage());
                }
            }
        }
        if (POST.equals(method)) {
            if ("/shutdown".equals(uri)) {
                sendShutdown(response, uri, origin);
                processed = true;
            }
            if (("/suspend").equals(uri) || ("/resume").equals(uri) ||
                ("/suspend/now").equals(uri) || ("/suspend/later").equals(uri) ||
                ("/resume/now").equals(uri) || ("/resume/later").equals(uri)) {
                suspendResume(response, uri, origin);
                processed = true;
            }
        }
        if (!processed) {
            if (uri.equals("/")) {
                Map<String, Object> instruction = new HashMap<>();
                List<String> endpoints = new ArrayList<>();
                instruction.put(MESSAGE, "Minimalist HTTP server supports these admin endpoints");
                instruction.put("endpoints", endpoints);
                for (String[] service: ADMIN_ENDPOINTS) {
                    endpoints.add(service[0]);
                }
                instruction.put("name", Platform.getInstance().getName());
                instruction.put("time", new Date());
                sendResponse("info", response, uri, 200, instruction);
            } else {
                sendError(response, uri, 404, "Resource not found");
            }
        }
    }

    private void suspendResume(HttpServerResponse response, String uri, String origin) {
        if (origin == null) {
            sendError(response, uri, 400, "Missing "+ APP_INSTANCE +" in request header");
            return;
        }
        PostOffice po = PostOffice.getInstance();
        Utility util = Utility.getInstance();
        List<String> parts = util.split(uri, "/");
        if (parts.size() == 1) {
            parts.add(NOW);
        }
        String type = parts.get(0);
        if (!po.exists(REGISTRY)) {
            sendError(response, uri, 400, type+" not available in standalone mode");
            return;
        }
        EventEnvelope event = new EventEnvelope().setHeader(TYPE, type);
        event.setTo(PostOffice.ACTUATOR_SERVICES+"@"+origin);
        event.setHeader(USER, System.getProperty("user.name"));
        String when = parts.get(1).equals(NOW) ? NOW : LATER;
        event.setHeader(WHEN, when);
        po.sendLater(event, new Date(System.currentTimeMillis() + GRACE_PERIOD));
        String message = type+" request sent to " + origin;
        if (LATER.equals(when)) {
            message += ". It will take effect in one minute.";
        }
        sendResponse(type, response, uri, 200, message);
    }

    private void sendShutdown(HttpServerResponse response, String uri, String origin) {
        if (origin == null) {
            sendError(response, uri, 400, "Missing "+ APP_INSTANCE +" in request header");
            return;
        }
        EventEnvelope event = new EventEnvelope().setHeader(TYPE, SHUTDOWN);
        event.setTo(PostOffice.ACTUATOR_SERVICES+"@"+origin);
        event.setHeader(USER, System.getProperty("user.name"));
        PostOffice.getInstance().sendLater(event, new Date(System.currentTimeMillis() + GRACE_PERIOD));
        sendResponse(SHUTDOWN, response, uri, 200, origin+" will be shutdown in "+GRACE_PERIOD+" ms");
    }

    private void sendError(HttpServerResponse response, String uri, int status, Object message) {
        sendResponse("error", response, uri, status, message);
    }

    @SuppressWarnings("unchecked")
    private void sendResponse(String type, HttpServerResponse response, String uri, int status, Object message) {
        final Map<String, Object> error;
        if (message instanceof Map) {
            error = (Map<String, Object>) message;
        } else {
            error = new HashMap<>();
            error.put(TYPE, type);
            error.put(STATUS, status);
            error.put(MESSAGE, message);
            error.put(PATH, uri);
        }
        byte[] b = SimpleMapper.getInstance().getMapper().writeValueAsBytes(error);
        response.putHeader(CONTENT_LENGTH, String.valueOf(b.length));
        response.setStatusCode(status);
        response.write(Buffer.buffer(b));
        response.end();
    }

    private String getAdminEndpointType(String path) {
        for (String[] service: ADMIN_ENDPOINTS) {
            if (path.equals(service[0])) {
                return service[1];
            }
        }
        return null;
    }

}
