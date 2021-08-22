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

import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.HttpServerResponse;
import org.platformlambda.automation.config.RoutingEntry;
import org.platformlambda.automation.models.AsyncContextHolder;
import org.platformlambda.automation.models.HeaderInfo;
import org.platformlambda.automation.util.SimpleHttpUtility;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.system.ObjectStreamReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentMap;

@EventInterceptor
public class ServiceResponseHandler implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(ServiceResponseHandler.class);

    private static final SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
    private static final String DATE = "Date";
    private static final String APPLICATION_JSON = "application/json";
    private static final String APPLICATION_XML = "application/xml";
    private static final String TEXT_HTML = "text/html";
    private static final String TEXT_PLAIN = "text/plain";
    private static final String HEAD = "HEAD";
    private static final String STREAM = "stream";
    private static final String STREAM_PREFIX = "stream.";
    private static final String TIMEOUT = "timeout";
    private static final String SET_COOKIE = "Set-Cookie";
    private static final String CONTENT_TYPE = "Content-Type";
    private static final String CONTENT_LEN = "Content-Length";
    private static final String HTML_START = "<!DOCTYPE html>\n<html>\n<body>\n<pre>\n";
    private static final String HTML_END = "\n</pre>\n<body>\n</html>";
    private static final String RESULT = "result";
    private static final String ACCEPT_ANY = "*/*";

    private final ConcurrentMap<String, AsyncContextHolder> contexts;

    public ServiceResponseHandler(ConcurrentMap<String, AsyncContextHolder> contexts) {
        this.contexts = contexts;
    }

    private long getReadTimeout(String timeoutOverride, long contextTimeout) {
        if (timeoutOverride == null) {
            return contextTimeout;
        }
        // convert to milliseconds
        long timeout = Utility.getInstance().str2long(timeoutOverride) * 1000;
        if (timeout < 1) {
            return contextTimeout;
        }
        return Math.min(timeout, contextTimeout);
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        Utility util = Utility.getInstance();
        SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
        EventEnvelope event = (EventEnvelope) body;
        String requestId = event.getCorrelationId();
        if (requestId != null) {
            AsyncContextHolder holder = contexts.get(requestId);
            if (holder != null) {
                holder.touch();
                HttpServerResponse response = holder.request.response();
                if (event.getStatus() != 200) {
                    response.setStatusCode(event.getStatus());
                }
                String accept = holder.accept;
                String timeoutOverride = null;
                String streamId = null;
                String contentType = null;
                Map<String, String> resHeaders = new HashMap<>();
                if (!event.getHeaders().isEmpty()) {
                    Map<String, String> evtHeaders = event.getHeaders();
                    for (String h : evtHeaders.keySet()) {
                        String key = h.toLowerCase();
                        String value = evtHeaders.get(h);
                        // "stream" and "timeout" are reserved as stream ID and read timeout in seconds
                        if (key.equals(STREAM) && value.startsWith(STREAM_PREFIX) && value.contains("@")) {
                            streamId = evtHeaders.get(h);
                        } else if (key.equalsIgnoreCase(TIMEOUT)) {
                            timeoutOverride = evtHeaders.get(h);
                        } else if (key.equalsIgnoreCase(CONTENT_TYPE)) {
                            contentType = value.toLowerCase();
                            response.putHeader(CONTENT_TYPE, contentType);
                        } else if (key.equalsIgnoreCase(SET_COOKIE)) {
                            httpUtil.setCookies(response, value);
                        } else {
                            resHeaders.put(key, value);
                        }
                    }
                }
                if (holder.resHeaderId != null) {
                    HeaderInfo hi = RoutingEntry.getInstance().getResponseHeaderInfo(holder.resHeaderId);
                    resHeaders = httpUtil.filterHeaders(hi, resHeaders);
                }
                for (String h : resHeaders.keySet()) {
                    String prettyHeader = httpUtil.getHeaderCase(h);
                    if (prettyHeader != null) {
                        response.putHeader(prettyHeader, resHeaders.get(h));
                    }
                }
                if (contentType == null) {
                    if (accept == null) {
                        contentType = "?";
                        // content-type header will not be provided
                    } else if (accept.contains(TEXT_HTML)) {
                        contentType = TEXT_HTML;
                        response.putHeader(CONTENT_TYPE, TEXT_HTML);
                    } else if (accept.contains(APPLICATION_XML)) {
                        contentType = APPLICATION_XML;
                        response.putHeader(CONTENT_TYPE, APPLICATION_XML);
                    } else if (accept.contains(APPLICATION_JSON) || accept.contains(ACCEPT_ANY)) {
                        contentType = APPLICATION_JSON;
                        response.putHeader(CONTENT_TYPE, APPLICATION_JSON);
                    } else {
                        contentType = TEXT_PLAIN;
                        response.putHeader(CONTENT_TYPE, TEXT_PLAIN);
                    }
                }
                // is this an exception?
                int status = event.getStatus();
                /*
                 * status range 100: used for HTTP protocol handshake
                 * status range 200: normal responses
                 * status range 300: redirection or unchanged content
                 * status ranges 400 and 500: HTTP exceptions
                 */
                if (status >= 400 && event.getHeaders().isEmpty() && event.getBody() instanceof String) {
                    String message = ((String) event.getBody()).trim();
                    // make sure it does not look like JSON or XML
                    if (!message.startsWith("{") && !message.startsWith("[") && !message.startsWith("<")) {
                        httpUtil.sendResponse(requestId, holder.request, status, (String) event.getBody());
                        return null;
                    }
                }
                // With the exception of HEAD method, HTTP response may have a body
                if (!HEAD.equals(holder.method)) {
                    // output is a stream?
                    Object responseBody = event.getBody();
                    if (responseBody == null && streamId != null) {
                        response.setChunked(true);
                        try (ObjectStreamReader in = new ObjectStreamReader(streamId,
                                                        getReadTimeout(timeoutOverride, holder.timeout))) {
                            for (Object block : in) {
                                // update last access time
                                holder.touch();
                                /*
                                 * only bytes or text are supported when using output stream
                                 * e.g. for downloading a large file
                                 */
                                if (block instanceof byte[]) {
                                    byte[] b = (byte[]) block;
                                    if (b.length > 0) {
                                        response.write(Buffer.buffer(b));
                                    }
                                }
                                if (block instanceof String) {
                                    String text = (String) block;
                                    if (!text.isEmpty()) {
                                        response.write((String) block);
                                    }
                                }
                            }
                        } catch (IOException | RuntimeException e) {
                            log.warn("{} {} interrupted - {}", holder.url, streamId, e.getMessage());
                            httpUtil.sendResponse(requestId, holder.request,
                                    e.getMessage().contains("timeout") ? 408 : 500, e.getMessage());
                            return null;
                        }
                        // regular output
                    } else if (responseBody instanceof Map) {
                        if (contentType.startsWith(TEXT_HTML)) {
                            byte[] start = util.getUTF(HTML_START);
                            byte[] payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(responseBody);
                            byte[] end = util.getUTF(HTML_END);
                            response.putHeader(CONTENT_LEN, String.valueOf(start.length+payload.length+end.length));
                            response.write(HTML_START);
                            response.write(Buffer.buffer(payload));
                            response.write(HTML_END);
                        } else if (contentType.startsWith(APPLICATION_XML)) {
                            byte[] payload = util.getUTF(xmlWriter.write("result", responseBody));
                            response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                            response.write(Buffer.buffer(payload));
                        } else {
                            byte[] payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(responseBody);
                            response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                            response.write(Buffer.buffer(payload));
                        }
                    } else if (responseBody instanceof List) {
                        if (contentType.startsWith(TEXT_HTML)) {
                            byte[] start = util.getUTF(HTML_START);
                            byte[] payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(responseBody);
                            byte[] end = util.getUTF(HTML_END);
                            response.putHeader(CONTENT_LEN, String.valueOf(start.length+payload.length+end.length));
                            response.write(HTML_START);
                            response.write(Buffer.buffer(payload));
                            response.write(HTML_END);
                        } else if (contentType.startsWith(APPLICATION_XML)) {
                            // xml must be delivered as a map so we use a wrapper here
                            Map<String, Object> map = new HashMap<>();
                            map.put(RESULT, responseBody);
                            byte[] payload = util.getUTF(xmlWriter.write("result", map));
                            response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                            response.write(Buffer.buffer(payload));
                        } else {
                            byte[] payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(responseBody);
                            response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                            response.write(Buffer.buffer(payload));
                        }
                    } else if (responseBody instanceof String) {
                        byte[] payload = util.getUTF((String) responseBody);
                        response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                        response.write(Buffer.buffer(payload));
                    } else if (responseBody instanceof byte[]) {
                        byte[] payload = (byte[]) responseBody;
                        response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                        response.write(Buffer.buffer(payload));
                    } else if (responseBody != null) {
                        byte[] payload = util.getUTF(responseBody.toString());
                        response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                        response.write(Buffer.buffer(payload));
                    }
                }
                ServiceGateway.closeContext(requestId);
                response.end();
            }
        }
        return null;
    }

}
