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

package org.platformlambda.http;

import io.vertx.core.Handler;
import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.HttpServerRequest;
import io.vertx.core.http.HttpServerResponse;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.AsyncContextHolder;
import org.platformlambda.models.EtagFile;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.util.Date;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class HttpRequestHandler implements Handler<HttpServerRequest> {
    private static final Logger log = LoggerFactory.getLogger(HttpRequestHandler.class);

    private static final CryptoApi crypto = new CryptoApi();
    private static final String ASYNC_HTTP_RESPONSE = "async.http.response";
    private static final String DATE = "Date";
    private static final String KEEP_ALIVE = "keep-alive";
    private static final String CONNECTION_HEADER = "Connection";
    private static final String ALLOW = "Allow";
    private static final String TYPE = "type";
    private static final String OPTIONS = "OPTIONS";
    private static final String GET = "GET";
    private static final String POST = "POST";
    private static final String APP_INSTANCE = "X-App-Instance";
    private static final String USER = "user";
    private static final String WHEN = "when";
    private static final String NOW = "now";
    private static final String LATER = "later";
    private static final String REGISTRY = "system.service.registry";
    private static final String[] INFO_SERVICE = {"/info", "info"};
    private static final String[] INFO_LIB = {"/info/lib", "lib"};
    private static final String[] INFO_ROUTES = {"/info/routes", "routes"};
    private static final String[] HEALTH_SERVICE = {"/health", "health"};
    private static final String[] ENV_SERVICE = {"/env", "env"};
    private static final String[] LIVENESSPROBE = {"/livenessprobe", "livenessprobe"};
    private static final String[][] ADMIN_ENDPOINTS = {INFO_SERVICE, INFO_LIB, INFO_ROUTES,
                                                        HEALTH_SERVICE, ENV_SERVICE, LIVENESSPROBE};
    private static final String INDEX_HTML = "index.html";
    private static final String CLASSPATH = "classpath:";
    private static final String FILEPATH = "file:";
    private static final String ETAG = "ETag";
    private static final String IF_NONE_MATCH = "If-None-Match";
    private static final String CONTENT_TYPE = "Content-Type";
    private static final String CONTENT_LEN = "Content-Length";
    private static final String TEXT_HTML = "text/html";
    private static final long GRACE_PERIOD = 5000;
    private static String staticFolder, resourceFolder;
    // requestId -> context
    private static final ConcurrentMap<String, AsyncContextHolder> contexts = new ConcurrentHashMap<>();
    private final boolean protectEndpoint;

    public HttpRequestHandler() {
        Platform platform = Platform.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        String folder = config.getProperty("spring.resources.static-locations", "classpath:/public");
        if (folder.startsWith(CLASSPATH)) {
            String resource = folder.substring(CLASSPATH.length());
            resourceFolder = resource.endsWith("/")? resource.substring(0, resource.length()-1) : resource;
        } else if (folder.startsWith(FILEPATH)) {
            staticFolder = folder.substring(FILEPATH.length());
        } else if (folder.startsWith("/")) {
            staticFolder = folder;
        } else {
            log.warn("Static content folder must start with {} or {}", CLASSPATH, FILEPATH);
        }
        AsyncTimeoutHandler timeoutHandler = new AsyncTimeoutHandler(contexts);
        timeoutHandler.start();
        this.protectEndpoint = "true".equals(config.getProperty("protect.info.endpoints", "false"));
        try {
            platform.registerPrivate(ASYNC_HTTP_RESPONSE, new ServiceResponseHandler(contexts), 300);
        } catch (IOException e) {
            log.error("Unable to setup HTTP handler - {}", e.getMessage());
            System.exit(10);
        }
    }

    public static void closeContext(String requestId) {
        contexts.remove(requestId);
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
        // simulate OPTIONS response
        if (OPTIONS.equals(method)) {
            response.putHeader(ALLOW, "GET, POST, OPTIONS");
            response.putHeader(CONTENT_LEN, "0");
            response.end();
            return;
        }
        SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
        String requestId = util.getUuid();
        AsyncContextHolder holder = new AsyncContextHolder(request);
        contexts.put(requestId, holder);
        if (GET.equals(method) && handleAdminEndpoint(requestId, request, url)) {
            return;
        }
        if (POST.equals(method)) {
            if ("/shutdown".equals(url) || "/shutdown/simulated".equals(url)) {
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
        if (GET.equals(request.method().name())) {
            EtagFile file = getStaticFile(request.path());
            if (file != null) {
                response.putHeader(CONTENT_TYPE, getFileContentType(request.path()));
                String ifNoneMatch = request.getHeader(IF_NONE_MATCH);
                if (file.equals(ifNoneMatch)) {
                    response.setStatusCode(304);
                    response.putHeader(CONTENT_LEN, String.valueOf(0));
                } else {
                    response.putHeader(ETAG, file.eTag);
                    response.putHeader(CONTENT_LEN, String.valueOf(file.content.length));
                    response.write(Buffer.buffer(file.content));
                }
                closeContext(requestId);
                response.end();
                return;
            }
        }
        httpUtil.sendResponse(requestId, request, 404, "Resource not found");
    }

    private boolean handleAdminEndpoint(String requestId, HttpServerRequest request, String path) {
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
        Utility util = Utility.getInstance();
        String url = request.path();
        List<String> parts = util.split(url, "/");
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
        try {
            if (parts.size() == 1) {
                po.sendLater(event, new Date(System.currentTimeMillis() + GRACE_PERIOD));
            } else {
                log.info("Unit test only - {}", parts);
            }
        } catch (IOException e) {
            log.error("Unable to send shutdown request - {}", e.getMessage());
        }
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

    private String getFileContentType(String path) {
        if (path.endsWith("/") || path.endsWith(".html") || path.endsWith(".htm")) {
            return TEXT_HTML;
        } else if (path.endsWith(".css")) {
            return "text/css";
        } else if (path.endsWith(".js")) {
            return "text/javascript";
        } else {
            return "application/octet-stream";
        }
    }

    private EtagFile getStaticFile(String path) {
        if (path.endsWith("/")) {
            path += INDEX_HTML;
        }
        if (resourceFolder != null) {
            return getResourceFile(path);
        }
        if (staticFolder != null) {
            return getLocalFile(path);
        }
        return null;
    }

    private EtagFile getResourceFile(String path) {
        Utility util = Utility.getInstance();
        InputStream in = this.getClass().getResourceAsStream(resourceFolder+path);
        if (in != null) {
            byte[] b = Utility.getInstance().stream2bytes(in);
            return new EtagFile(util.bytes2hex(crypto.getSHA1(b)), b);
        }
        return null;
    }

    private EtagFile getLocalFile(String path) {
        Utility util = Utility.getInstance();
        File f = new File(staticFolder, path);
        if (f.exists()) {
            byte[] b = Utility.getInstance().file2bytes(f);
            return new EtagFile(util.bytes2hex(crypto.getSHA1(b)), b);
        }
        return null;
    }
}
