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

package org.platformlambda.automation.services;

import io.vertx.core.MultiMap;
import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.HttpServerRequest;
import io.vertx.core.http.HttpServerResponse;
import org.platformlambda.automation.MainModule;
import org.platformlambda.automation.config.RoutingEntry;
import org.platformlambda.automation.models.*;
import org.platformlambda.automation.util.SimpleHttpUtility;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlParser;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.*;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

public class ServiceGateway {
    private static final Logger log = LoggerFactory.getLogger(ServiceGateway.class);

    private static final CryptoApi crypto = new CryptoApi();
    private static final SimpleXmlParser xmlReader = new SimpleXmlParser();
    private static final String CONTENT_TYPE = "Content-Type";
    private static final String CONTENT_LEN = "Content-Length";
    private static final String APPLICATION_FORM_URLENCODED = "application/x-www-form-urlencoded";
    private static final String MULTIPART_FORM_DATA = "multipart/form-data";
    private static final String APPLICATION_JSON = "application/json";
    private static final String APPLICATION_XML = "application/xml";
    private static final String TEXT_HTML = "text/html";
    private static final String TEXT_PLAIN = "text/plain";
    private static final String PROTOCOL = "x-forwarded-proto";
    private static final String HTTPS = "https";
    private static final String COOKIE = "cookie";
    private static final String ASYNC_HTTP_RESPONSE = MainModule.ASYNC_HTTP_RESPONSE;
    private static final String OPTIONS = "OPTIONS";
    private static final String GET = "GET";
    private static final String PUT = "PUT";
    private static final String POST = "POST";
    private static final String PATCH = "PATCH";
    private static final String ACCEPT = "accept";
    private static final String INDEX_HTML = "index.html";
    private static final String CLASSPATH = "classpath:";
    private static final String FILEPATH = "file:";
    private static final String ETAG = "ETag";
    private static final String IF_NONE_MATCH = "If-None-Match";
    private static final int BUFFER_SIZE = 8 * 1024;
    // requestId -> context
    private static final ConcurrentMap<String, AsyncContextHolder> contexts = new ConcurrentHashMap<>();
    private static String defaultTraceIdLabel;
    private static List<String> traceIdLabels;
    private static String staticFolder, resourceFolder;

    public ServiceGateway() {
        Utility util = Utility.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        List<String> labels = util.split(config.getProperty("trace.http.header"), ", ");
        if (labels.isEmpty()) {
            labels.add("X-Trace-Id");
        }
        defaultTraceIdLabel = labels.get(0);
        traceIdLabels = labels;
        log.info("HTTP trace headers {}", traceIdLabels);
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
    }

    public static String getDefaultTraceIdLabel() {
        return defaultTraceIdLabel;
    }

    public ConcurrentMap<String, AsyncContextHolder> getContexts() {
        return contexts;
    }

    public static void closeContext(String requestId) {
        contexts.remove(requestId);
    }

    public void handleEvent(AssignedRoute route, String requestId, int status, String error) {
        AsyncContextHolder holder = contexts.get(requestId);
        if (holder != null) {
            HttpServerRequest request = holder.request;
            SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
            if (error != null) {
                if (GET.equals(request.method().name()) && status == 404) {
                    EtagFile file = getStaticFile(request.path());
                    if (file != null) {
                        HttpServerResponse response = request.response();
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
                httpUtil.sendError(requestId, request, status, error);
            } else {
                try {
                    routeRequest(requestId, route, holder);
                } catch (AppException e) {
                    httpUtil.sendError(requestId, request, e.getStatus(), e.getMessage());
                } catch (IOException e) {
                    httpUtil.sendError(requestId, request, 500, e.getMessage());
                }
            }
        }
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
        InputStream in = ServiceGateway.class.getResourceAsStream(resourceFolder+path);
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

    private void routeRequest(String requestId, AssignedRoute route, AsyncContextHolder holder)
            throws AppException, IOException {
        HttpServerRequest request = holder.request;
        String url = request.path();
        String method = request.method().name();
        holder.setUrl(url).setMethod(method).setResHeaderId(route.info.responseTransformId);
        String acceptContent = request.getHeader(ACCEPT);
        if (acceptContent != null) {
            holder.setAccept(acceptContent);
        }
        SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
        Utility util = Utility.getInstance();
        if (OPTIONS.equals(method)) {
            // insert CORS headers for OPTIONS
            if (route.info.corsId == null) {
                throw new AppException(405, "Method not allowed");
            } else {
                CorsInfo corsInfo = RoutingEntry.getInstance().getCorsInfo(route.info.corsId);
                if (corsInfo != null && !corsInfo.options.isEmpty()) {
                    HttpServerResponse response = request.response();
                    for (String ch : corsInfo.options.keySet()) {
                        String prettyHeader = getHeaderCase(ch);
                        if (prettyHeader != null) {
                            response.putHeader(prettyHeader, corsInfo.options.get(ch));
                        }
                    }
                    closeContext(requestId);
                    response.setStatusCode(204).end();
                } else {
                    throw new AppException(405, "Method not allowed");
                }
            }
            return;
        }
        HttpServerResponse response = request.response();
        // insert CORS headers for the HTTP response
        if (route.info.corsId != null) {
            CorsInfo corsInfo = RoutingEntry.getInstance().getCorsInfo(route.info.corsId);
            if (corsInfo != null && !corsInfo.headers.isEmpty()) {
                for (String ch : corsInfo.headers.keySet()) {
                    String prettyHeader = getHeaderCase(ch);
                    if (prettyHeader != null) {
                        response.putHeader(prettyHeader, corsInfo.headers.get(ch));
                    }
                }
            }
        }
        // check if target service is available
        PostOffice po = PostOffice.getInstance();
        if (route.info.authService == null) {
            if (!po.exists(route.info.primary)) {
                throw new AppException(503, "Service " + route.info.primary + " not reachable");
            }
        } else {
            if (!po.exists(route.info.primary, route.info.authService)) {
                throw new AppException(503, "Service " + route.info.primary + " or " +
                                        route.info.authService + " not reachable");
            }
        }
        AsyncHttpRequest req = new AsyncHttpRequest();
        String queryString = request.query();
        if (queryString != null) {
            req.setQueryString(queryString);
        }
        req.setUrl(httpUtil.normalizeUrl(url, route.info.urlRewrite));
        if (route.info.host != null) {
            req.setTargetHost(route.info.host);
            req.setTrustAllCert(route.info.trustAllCert);
        }
        req.setMethod(method);
        req.setSecure(HTTPS.equals(request.getHeader(PROTOCOL)));
        req.setTimeoutSeconds(route.info.timeoutSeconds);
        if (!route.arguments.isEmpty()) {
            for (String p: route.arguments.keySet()) {
                req.setPathParameter(p, route.arguments.get(p));
            }
        }
        MultiMap params = request.params();
        for (String key: params.names()) {
            List<String> values = params.getAll(key);
            if (values.size() == 1) {
                req.setQueryParameter(key, values.get(0));
            }
            if (values.size() > 1) {
                req.setQueryParameter(key, values);
            }
        }
        boolean hasCookies = false;
        Map<String, String> headers = new HashMap<>();
        MultiMap headerMap = request.headers();
        for (String key: headerMap.names()) {
            /*
             * Single-value HTTP header is assumed.
             */
            String value = headerMap.get(key);
            if (key.equalsIgnoreCase(COOKIE)) {
                // cookie is not kept in the headers
                hasCookies = true;
            } else {
                headers.put(key.toLowerCase(), value);
            }
        }
        // load cookies
        if (hasCookies) {
            Map<String, io.vertx.core.http.Cookie> cookies = request.cookieMap();
            for (String item : cookies.keySet()) {
                io.vertx.core.http.Cookie c = cookies.get(item);
                req.setCookie(c.getName(), c.getValue());
            }
        }
        RoutingEntry re = RoutingEntry.getInstance();
        if (route.info.requestTransformId != null) {
            headers = httpUtil.filterHeaders(re.getRequestHeaderInfo(route.info.requestTransformId), headers);
        }
        for (String h: headers.keySet()) {
            req.setHeader(h, headers.get(h));
        }
        req.setRemoteIp(request.remoteAddress().hostAddress());
        // Distributed tracing required?
        String traceId = null;
        String tracePath = null;
        // Set trace header if needed
        if (route.info.tracing) {
            List<String> traceHeader = getTraceId(request);
            traceId = traceHeader.get(1);
            tracePath = method + " " + url;
            if (queryString != null) {
                tracePath += "?" + queryString;
            }
            response.putHeader(traceHeader.get(0), traceHeader.get(1));
        }
        // authentication required?
        if (route.info.authService != null) {
            try {
                long authTimeout = route.info.timeoutSeconds * 1000L;
                EventEnvelope authRequest = new EventEnvelope();
                // the AsyncHttpRequest is sent as a map
                authRequest.setTo(route.info.authService).setBody(req.toMap());
                // distributed tracing required?
                if (route.info.tracing) {
                    authRequest.setFrom("http.request");
                    authRequest.setTrace(traceId, tracePath);
                }
                EventEnvelope authResponse = po.request(authRequest, authTimeout);
                if (!authResponse.hasError() && authResponse.getBody() instanceof Boolean) {
                    Boolean authOK = (Boolean) authResponse.getBody();
                    if (authOK) {
                        /*
                         * Upon successful authentication,
                         * the authentication service may save session information as headers
                         * (auth headers are converted to lower case for case insensitivity)
                         */
                        Map<String, String> authResHeaders = authResponse.getHeaders();
                        for (String k : authResHeaders.keySet()) {
                            req.setSessionInfo(k, authResHeaders.get(k));
                        }
                    } else {
                        throw new AppException(401, "Unauthorized");
                    }
                }
            } catch (IOException e) {
                log.error("REST authentication - {}", e.getMessage());
                throw new AppException(400, e.getMessage());
            } catch (TimeoutException e) {
                log.error("REST authentication - {}", e.getMessage());
                throw new AppException(408, e.getMessage());
            } catch (AppException e) {
                // allow the authentication service to throw exception back to the browser
                throw new AppException(e.getStatus(), e.getMessage());
            }
        }
        // load HTTP body
        if (POST.equals(method) || PUT.equals(method) || PATCH.equals(method)) {
            final AtomicBoolean inputComplete = new AtomicBoolean(false);
            final String traceIdFinal = traceId;
            final String tracePathFinal = tracePath;
            final ByteArrayOutputStream requestBody = new ByteArrayOutputStream();
            String contentType = request.getHeader(CONTENT_TYPE);
            if (contentType == null) {
                contentType = "?";
            }
            if (contentType.startsWith(MULTIPART_FORM_DATA) && POST.equals(method)) {
                final ObjectStreamWrapper stream = new ObjectStreamWrapper(route.info.timeoutSeconds);
                request.uploadHandler(upload -> {
                    req.setFileName(upload.filename());
                    final AtomicInteger total = new AtomicInteger();
                    upload.handler(block -> {
                        int len = block.length();
                        if (len > 0) {
                            total.addAndGet(len);
                            readInputStream(stream, block, len);
                        }
                    }).endHandler(end -> {
                        try {
                            int size = total.get();
                            req.setContentLength(size);
                            req.setStreamRoute(stream.getId());
                            stream.close();
                        } catch (IOException e) {
                            log.error("Unexpected error while closing HTTP input stream - {}", e.getMessage());
                        }
                        sendRequestToService(request, req, route, requestId, traceIdFinal, tracePathFinal);
                    });
                });
                request.resume();

            } else if (contentType.startsWith(APPLICATION_JSON)) {
                request.bodyHandler(block -> {
                    byte[] b = block.getBytes(0, block.length());
                    requestBody.write(b, 0, b.length);
                    if (inputComplete.get()) {
                        String text = util.getUTF(requestBody.toByteArray()).trim();
                        if (text.length() == 0) {
                            req.setBody(new HashMap<>());
                        } else {
                            if (text.startsWith("{") && text.endsWith("}")) {
                                req.setBody(SimpleMapper.getInstance().getMapper().readValue(text, Map.class));
                            } else if (text.startsWith("[") && text.endsWith("]")) {
                                req.setBody(SimpleMapper.getInstance().getMapper().readValue(text, List.class));
                            } else {
                                req.setBody(text);
                            }
                        }
                        sendRequestToService(request, req, route, requestId, traceIdFinal, tracePathFinal);
                    }
                }).endHandler(done -> inputComplete.set(true));
            } else if (contentType.startsWith(APPLICATION_XML)) {
                request.bodyHandler(block -> {
                    byte[] b = block.getBytes(0, block.length());
                    requestBody.write(b, 0, b.length);
                    if (inputComplete.get()) {
                        String text = util.getUTF(requestBody.toByteArray()).trim();
                        try {
                            req.setBody(text.isEmpty()? new HashMap<>() : xmlReader.parse(text));
                        } catch (Exception e) {
                            req.setBody(text);
                        }
                        sendRequestToService(request, req, route, requestId, traceIdFinal, tracePathFinal);
                    }
                }).endHandler(done -> inputComplete.set(true));
            } else if (contentType.equals(APPLICATION_FORM_URLENCODED) ||
                    contentType.startsWith(TEXT_HTML) || contentType.startsWith(TEXT_PLAIN)) {
                final boolean urlEncodeParameters = contentType.equals(APPLICATION_FORM_URLENCODED);
                request.bodyHandler(block -> {
                    byte[] b = block.getBytes(0, block.length());
                    requestBody.write(b, 0, b.length);
                    if (inputComplete.get()) {
                        String text = util.getUTF(requestBody.toByteArray());
                        if (urlEncodeParameters) {
                            Map<String, String> kv = httpUtil.decodeQueryString(text);
                            for (String k: kv.keySet()) {
                                req.setQueryParameter(k, kv.get(k));
                            }
                        } else {
                            req.setBody(text);
                        }
                        sendRequestToService(request, req, route, requestId, traceIdFinal, tracePathFinal);
                    }
                }).endHandler(done -> inputComplete.set(true));
            } else {
                /*
                 * The input is not JSON, XML or TEXT
                 * Check if the content-length is larger than threshold.
                 * For large payload, it is better to deliver as a stream.
                 */
                int contentLen = util.str2int(request.getHeader(CONTENT_LEN));
                if (contentLen > 0 && contentLen <= route.info.threshold) {
                    request.bodyHandler(block -> {
                        byte[] b = block.getBytes(0, block.length());
                        requestBody.write(b, 0, b.length);
                        if (inputComplete.get()) {
                            req.setBody(requestBody.toByteArray());
                            sendRequestToService(request, req, route, requestId, traceIdFinal, tracePathFinal);
                        }
                    }).endHandler(done -> inputComplete.set(true));
                } else {
                    final AtomicInteger total = new AtomicInteger();
                    final ObjectStreamWrapper stream = new ObjectStreamWrapper(route.info.timeoutSeconds);
                    request.bodyHandler(block -> {
                        int len = block.length();
                        if (len > 0) {
                            total.addAndGet(len);
                            readInputStream(stream, block, len);
                        }
                        if (inputComplete.get()) {
                            try {
                                int size = total.get();
                                req.setContentLength(size);
                                req.setStreamRoute(stream.getId());
                                stream.close();
                            } catch (IOException e) {
                                log.error("Unexpected error while closing HTTP input stream - {}", e.getMessage());
                            }
                            sendRequestToService(request, req, route, requestId, traceIdFinal, tracePathFinal);
                        }
                    }).endHandler(end -> inputComplete.set(true));
                }
            }
        } else {
            sendRequestToService(request, req, route, requestId, traceId, tracePath);
        }
    }

    public void sendRequestToService(HttpServerRequest request, AsyncHttpRequest req,
                                     AssignedRoute route, String requestId, String traceId, String tracePath) {
        PostOffice po = PostOffice.getInstance();
        Map<String, Object> requestBody = req.toMap();
        EventEnvelope event = new EventEnvelope();
        event.setTo(route.info.primary).setBody(requestBody)
                .setCorrelationId(requestId).setReplyTo(ASYNC_HTTP_RESPONSE +"@"+Platform.getInstance().getOrigin());
        // enable distributed tracing if needed
        if (route.info.tracing) {
            event.setFrom("http.request");
            event.setTrace(traceId, tracePath);
        }
        try {
            po.send(event);
            // copying to secondary services if any
            if (route.info.services.size() > 1) {
                for (String secondary: route.info.services) {
                    if (!secondary.equals(route.info.primary)) {
                        EventEnvelope copy = new EventEnvelope().setTo(secondary).setBody(requestBody);
                        if (route.info.tracing) {
                            copy.setFrom("http.request");
                            copy.setTrace(traceId, tracePath);
                        }
                        try {
                            po.send(copy);
                        } catch (Exception e) {
                            log.warn("Unable to copy event to secondary - {}", e.getMessage());
                        }
                    }
                }
            }
        } catch (IOException e) {
            SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
            httpUtil.sendError(requestId, request,400, e.getMessage());
        }
    }

    private void readInputStream(ObjectStreamWrapper stream, Buffer block, int len) {
        try {
            byte[] data = block.getBytes(0, len);
            if (data.length > BUFFER_SIZE) {
                int bytesRead = 0;
                byte[] buf = new byte[BUFFER_SIZE];
                ByteArrayInputStream in = new ByteArrayInputStream(data);
                while (bytesRead < data.length) {
                    int n = in.read(buf);
                    bytesRead += n;
                    stream.write(data, 0, n);
                }
            } else {
                stream.write(data);
            }
        } catch (IOException e) {
            log.error("Unexpected error while reading HTTP input stream", e);
        }
    }

    /**
     * Get X-Trace-Id from HTTP request headers if any.
     * Otherwise generate a unique ID.
     *
     * @param request HTTP
     * @return traceLabel and traceId
     */
    private List<String> getTraceId(HttpServerRequest request) {
        List<String> result = new ArrayList<>();
        for (String label: traceIdLabels) {
            String id = request.getHeader(label);
            if (id != null) {
                result.add(label);
                result.add(id);
            }
        }
        result.add(getDefaultTraceIdLabel());
        result.add(Utility.getInstance().getUuid());
        return result;
    }

    private String getHeaderCase(String header) {
        StringBuilder sb = new StringBuilder();
        List<String> parts = Utility.getInstance().split(header, "-");
        for (String p: parts) {
            sb.append(p.substring(0, 1).toUpperCase());
            if (p.length() > 1) {
                sb.append(p.substring(1));
            }
            sb.append('-');
        }
        return sb.length() == 0? null : sb.substring(0, sb.length()-1);
    }

}
