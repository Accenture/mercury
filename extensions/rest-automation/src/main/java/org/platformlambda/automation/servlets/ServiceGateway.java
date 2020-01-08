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

package org.platformlambda.automation.servlets;

import org.platformlambda.automation.MainApp;
import org.platformlambda.automation.config.RoutingEntry;
import org.platformlambda.automation.models.AssignedRoute;
import org.platformlambda.automation.models.AsyncContextHolder;
import org.platformlambda.automation.models.CorsInfo;
import org.platformlambda.automation.util.AsyncHttpHandler;
import org.platformlambda.automation.util.SimpleHttpUtility;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlParser;
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamWriter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.AsyncContext;
import javax.servlet.ServletException;
import javax.servlet.annotation.MultipartConfig;
import javax.servlet.annotation.WebServlet;
import javax.servlet.http.*;
import javax.ws.rs.core.MediaType;
import java.io.BufferedInputStream;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

@WebServlet(urlPatterns="/api/*", asyncSupported=true)
@MultipartConfig
public class ServiceGateway extends HttpServlet {
    private static final Logger log = LoggerFactory.getLogger(ServiceGateway.class);

    private static final SimpleXmlParser xmlReader = new SimpleXmlParser();
    public static final String X_TRACE_ID = "X-Trace-Id";
    private static final String PROTOCOL = "x-forwarded-proto";
    private static final String HTTPS = "https";
    private static final String UTF_8 = "utf-8";
    private static final String BASE_PATH = "/api";
    private static final String COOKIE = "cookie";
    private static final String ASYNC_HTTP_RESPONSE = MainApp.ASYNC_HTTP_RESPONSE;
    private static final String OPTIONS = "OPTIONS";
    private static final String PUT = "PUT";
    private static final String POST = "POST";
    private static final String PATCH = "PATCH";
    private static final String ACCEPT = "accept";
    private static final int BUFFER_SIZE = 2048;
    // requestId -> context
    private static final ConcurrentMap<String, AsyncContextHolder> contexts = new ConcurrentHashMap<>();
    private static boolean ready = false;

    public static ConcurrentMap<String, AsyncContextHolder> getContexts() {
        return contexts;
    }

    public static void setReady() {
        ready = true;
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

    @Override
    protected void service(HttpServletRequest request, HttpServletResponse response) throws IOException {
        if (!ready) {
            response.sendError(503, "Unable to serve requests because REST endpoints are not ready");
        } else {
            String path = request.getPathInfo();
            if (path == null || path.equals("/")) {
                response.sendError(404, "Resource not found");
            } else {
                String url = BASE_PATH + path;
                RoutingEntry re = RoutingEntry.getInstance();
                AssignedRoute route = re.getRouteInfo(request.getMethod(), url);
                if (route == null) {
                    response.sendError(404, "Resource not found");
                } else {
                    if (route.info == null) {
                        response.sendError(405, "Method not allowed");
                    } else {
                        routeRequest(url, route, request, response);
                    }
                }
            }
        }
    }

    @SuppressWarnings("unchecked")
    private void routeRequest(String url, AssignedRoute route, HttpServletRequest request, HttpServletResponse response) throws IOException {
        request.setCharacterEncoding(UTF_8);
        response.setCharacterEncoding(UTF_8);
        String method = request.getMethod();
        SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
        Utility util = Utility.getInstance();
        if (OPTIONS.equals(method)) {
            // insert CORS headers for OPTIONS
            if (route.info.corsId == null) {
                response.sendError(405, "Method not allowed");
            } else {
                CorsInfo corsInfo = RoutingEntry.getInstance().getCorsInfo(route.info.corsId);
                if (corsInfo != null && !corsInfo.options.isEmpty()) {
                    for (String ch : corsInfo.options.keySet()) {
                        String prettyHeader = getHeaderCase(ch);
                        if (prettyHeader != null) {
                            response.setHeader(prettyHeader, corsInfo.options.get(ch));
                        }
                    }
                    response.setStatus(204);
                } else {
                    response.sendError(405, "Method not allowed");
                }
            }
            return;
        } else {
            // insert CORS headers for the HTTP response
            if (route.info.corsId != null) {
                CorsInfo corsInfo = RoutingEntry.getInstance().getCorsInfo(route.info.corsId);
                if (corsInfo != null && !corsInfo.headers.isEmpty()) {
                    for (String ch : corsInfo.headers.keySet()) {
                        String prettyHeader = getHeaderCase(ch);
                        if (prettyHeader != null) {
                            response.setHeader(prettyHeader, corsInfo.headers.get(ch));
                        }
                    }
                }
            }
        }
        // check if target service is available
        PostOffice po = PostOffice.getInstance();
        if (route.info.authService == null) {
            if (!po.exists(route.info.service)) {
                response.sendError(503, "Service " + route.info.service + " not reachable");
                return;
            }
        } else {
            if (!po.exists(route.info.service, route.info.authService)) {
                response.sendError(503, "Service " + route.info.service + " or " + route.info.authService + " not reachable");
                return;
            }
        }
        AsyncHttpRequest req = new AsyncHttpRequest();
        String queryString = request.getQueryString();
        if (queryString != null) {
            req.setQueryString(queryString);
        }
        req.setUrl(httpUtil.normalizeUrl(url, route.info.urlRewrite));
        if (route.info.host != null) {
            req.setRelay(route.info.host);
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
        Enumeration<String> pNames = request.getParameterNames();
        while (pNames.hasMoreElements()) {
            String key = pNames.nextElement();
            String[] values = request.getParameterValues(key);
            if (values.length == 1) {
                req.setQueryParameter(key, values[0]);
            }
            if (values.length > 1) {
                req.setQueryParameter(key, Arrays.asList(values));
            }
        }
        boolean hasCookies = false;
        Map<String, String> headers = new HashMap<>();
        Enumeration<String> hNames = request.getHeaderNames();
        while (hNames.hasMoreElements()) {
            String key = hNames.nextElement();
            /*
             * Single-value HTTP header is assumed.
             */
            String value = request.getHeader(key);
            if (key.equalsIgnoreCase(COOKIE)) {
                // cookie is not kept in the headers
                hasCookies = true;
            } else {
                headers.put(key.toLowerCase(), value);
            }
        }
        // load cookies
        if (hasCookies) {
            Cookie[] cookies = request.getCookies();
            for (Cookie c : cookies) {
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
        req.setRemoteIp(request.getRemoteAddr());
        // Distributed tracing required?
        String traceId = null;
        String tracePath = null;
        // Set trace header if needed
        if (route.info.tracing) {
            /*
             * Use X-Trace-Id from HTTP request headers if any.
             * Otherwise, generate a unique ID.
             */
            String httpTrace = request.getHeader(X_TRACE_ID);
            traceId = httpTrace == null? util.getUuid() : httpTrace;
            tracePath = method + " " + url;
            if (queryString != null) {
                tracePath += "?" + queryString;
            }
            response.setHeader(X_TRACE_ID, traceId);
        }
        // authentication required?
        if (route.info.authService != null) {
            try {
                long authTimeout = route.info.timeoutSeconds * 1000;
                EventEnvelope authRequest = new EventEnvelope();
                // the AsyncHttpRequest is sent as a map
                authRequest.setTo(route.info.authService).setBody(req.toMap());
                // distributed tracing required?
                if (route.info.tracing) {
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
                        response.sendError(401, "Unauthorized");
                        return;
                    }
                }
            } catch (IOException e) {
                log.error("REST authentication - {}", e.getMessage());
                response.sendError(400, e.getMessage());
                return;
            } catch (TimeoutException e) {
                log.error("REST authentication - {}", e.getMessage());
                response.sendError(408, e.getMessage());
                return;
            } catch (AppException e) {
                // allow the authentication service to throw exception back to the browser
                response.sendError(e.getStatus(), e.getMessage());
                return;
            }
        }
        // load HTTP body
        if (POST.equals(method) || PUT.equals(method) || PATCH.equals(method)) {
            String contentType = request.getContentType();
            // ignore URL encoded content type because it has been already fetched as request/query parameters
            if (contentType != null && !contentType.startsWith(MediaType.APPLICATION_FORM_URLENCODED)) {
                // handle request body
                if (contentType.startsWith(MediaType.MULTIPART_FORM_DATA) && POST.equals(method)) {
                    // file upload
                    try {
                        Part filePart = request.getPart(route.info.upload);
                        if (filePart != null) {
                            String fileName = httpUtil.getFileName(filePart);
                            if (fileName != null) {
                                int len;
                                int total = 0;
                                byte[] buffer = new byte[BUFFER_SIZE];
                                BufferedInputStream in = new BufferedInputStream(filePart.getInputStream());
                                ObjectStreamIO stream = null;
                                ObjectStreamWriter out = null;
                                while ((len = in.read(buffer, 0, buffer.length)) != -1) {
                                    if (out == null) {
                                        // Depending on input file size, the servlet will save the input stream
                                        // into memory or local file system, the system therefore defer creation
                                        // of the object stream until the buffered input stream is ready.
                                        stream = new ObjectStreamIO(route.info.timeoutSeconds);
                                        out = stream.getOutputStream();
                                    }
                                    total += len;
                                    out.write(buffer, 0, len);
                                }
                                if (out != null) {
                                    out.close();
                                    req.setStreamRoute(stream.getRoute());
                                    req.setFileName(fileName);
                                    req.setContentLength(total);
                                    req.setUploadTag(route.info.upload);
                                }
                            }
                        }
                    } catch (ServletException e) {
                        response.sendError(400, e.getMessage());
                        return;
                    }

                } else if (contentType.startsWith(MediaType.APPLICATION_JSON)) {
                    // request body is assumed to be JSON
                    String text = util.getUTF(util.stream2bytes(request.getInputStream(), false)).trim();
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

                } else if (contentType.startsWith(MediaType.APPLICATION_XML)) {
                    // request body is assumed to be XML
                    String text = util.getUTF(util.stream2bytes(request.getInputStream(), false));
                    try {
                        req.setBody(text.isEmpty()? new HashMap<>() : xmlReader.parse(text));
                    } catch (Exception e) {
                        req.setBody(text);
                    }
                } else if (contentType.startsWith(MediaType.TEXT_HTML) || contentType.startsWith(MediaType.TEXT_PLAIN)) {
                    String text = util.getUTF(util.stream2bytes(request.getInputStream(), false));
                    req.setBody(text);
                } else {
                    /*
                     * The input is not JSON, XML or TEXT
                     * Check if the content-length is larger than threshold.
                     * For large payload, it is better to deliver as a stream.
                     */
                    int contentLen = request.getContentLength();
                    if (contentLen > 0 && contentLen <= route.info.threshold) {
                        byte[] bytes = util.stream2bytes(request.getInputStream(), false);
                        req.setBody(bytes);
                    } else {
                        // If content-length is undefined or larger than threshold, it will be sent as a stream.
                        int len;
                        int total = 0;
                        byte[] buffer = new byte[BUFFER_SIZE];
                        BufferedInputStream in = new BufferedInputStream(request.getInputStream());
                        ObjectStreamIO stream = null;
                        ObjectStreamWriter out = null;
                        while ((len = in.read(buffer, 0, buffer.length)) != -1) {
                            if (out == null) {
                                // defer creation of object stream because input stream may be empty
                                stream = new ObjectStreamIO(route.info.timeoutSeconds);
                                out = stream.getOutputStream();
                            }
                            total += len;
                            out.write(buffer, 0, len);
                        }
                        if (out != null) {
                            out.close();
                            req.setStreamRoute(stream.getRoute());
                            req.setContentLength(total);
                        }
                    }
                }
            }
        }
        // generate unique requestId
        String requestId = Utility.getInstance().getUuid();
        // create HTTP async context
        AsyncContext context = request.startAsync(request, response);
        context.addListener(new AsyncHttpHandler(contexts, requestId));
        // save to context map
        AsyncContextHolder holder = new AsyncContextHolder(context, route.info.timeoutSeconds * 1000);
        holder.setUrl(url).setMethod(method).setResHeaderId(route.info.responseTransformId);
        String acceptContent = request.getHeader(ACCEPT);
        if (acceptContent != null) {
            holder.setAccept(acceptContent);
        }
        contexts.put(requestId, holder);
        /*
         * Forward the AsyncHttpRequest to the target service.
         * It is delivered as a map so it is more generic
         */
        EventEnvelope event = new EventEnvelope();
        event.setTo(route.info.service).setBody(req.toMap())
                .setCorrelationId(requestId).setReplyTo(ASYNC_HTTP_RESPONSE +"@"+Platform.getInstance().getOrigin());
        // enable distributed tracing if needed
        if (route.info.tracing) {
            event.setTrace(traceId, tracePath);
        }
        try {
            po.send(event);
        } catch (IOException e) {
            response.sendError(400, e.getMessage());
            context.complete();
        }
    }

}
