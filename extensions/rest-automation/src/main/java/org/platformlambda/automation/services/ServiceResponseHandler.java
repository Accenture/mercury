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

package org.platformlambda.automation.services;

import org.platformlambda.automation.config.RoutingEntry;
import org.platformlambda.automation.models.AsyncContextHolder;
import org.platformlambda.automation.models.HeaderInfo;
import org.platformlambda.automation.util.SimpleHttpUtility;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.ServletResponse;
import javax.servlet.http.HttpServletResponse;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.io.OutputStream;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentMap;

@EventInterceptor
public class ServiceResponseHandler implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(ServiceResponseHandler.class);

    private static final SimpleXmlWriter xmlWriter = new SimpleXmlWriter();

    private static final String HEAD = "HEAD";
    private static final String STREAM = "stream";
    private static final String STREAM_PREFIX = "stream.";
    private static final String TIMEOUT = "timeout";
    private static final String SET_COOKIE = "set-cookie";
    private static final String CONTENT_TYPE = "content-type";
    private static final String HTML_START = "<!DOCTYPE html>\n<html>\n<body>\n<pre>\n";
    private static final String HTML_END = "\n</pre>\n<body>\n</html>";
    private static final String RESULT = "result";
    private static final String ACCEPT_ANY = "*/*";

    private ConcurrentMap<String, AsyncContextHolder> contexts;

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
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (body instanceof EventEnvelope) {
            Utility util = Utility.getInstance();
            SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
            EventEnvelope event = (EventEnvelope) body;
            String requestId = event.getCorrelationId();
            if (requestId != null) {
                AsyncContextHolder holder = contexts.get(requestId);
                if (holder != null) {
                    holder.touch();
                    ServletResponse res = holder.context.getResponse();
                    if (res instanceof HttpServletResponse) {
                        HttpServletResponse response = (HttpServletResponse) res;
                        if (event.getStatus() != 200) {
                            response.setStatus(event.getStatus());
                        }
                        String accept = holder.accept;
                        String timeoutOverride = null;
                        String streamId = null;
                        String contentType = null;
                        Map<String, String> resHeaders = new HashMap<>();
                        if (!event.getHeaders().isEmpty()) {
                            Map<String, String> evtHeaders = event.getHeaders();
                            for (String h: evtHeaders.keySet()) {
                                String key = h.toLowerCase();
                                String value = evtHeaders.get(h);
                                // "stream" and "timeout" are reserved as stream ID and read timeout in seconds
                                if (key.equals(STREAM) && value.startsWith(STREAM_PREFIX) && value.contains("@")) {
                                    streamId = evtHeaders.get(h);
                                } else if (key.equals(TIMEOUT)) {
                                    timeoutOverride = evtHeaders.get(h);
                                } else if (key.equals(CONTENT_TYPE)) {
                                    contentType = value.toLowerCase();
                                    response.setContentType(contentType);
                                } else if (key.equals(SET_COOKIE)) {
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
                        for (String h: resHeaders.keySet()) {
                            String prettyHeader = httpUtil.getHeaderCase(h);
                            if (prettyHeader != null) {
                                response.setHeader(prettyHeader, resHeaders.get(h));
                            }
                        }
                        // default content type is JSON
                        if (contentType == null) {
                            if (accept == null) {
                                contentType = MediaType.APPLICATION_JSON;
                                response.setContentType(MediaType.APPLICATION_JSON);
                            } else if (accept.contains(MediaType.TEXT_HTML)) {
                                contentType = MediaType.TEXT_HTML;
                                response.setContentType(MediaType.TEXT_HTML);
                            } else if (accept.contains(MediaType.APPLICATION_XML)) {
                                contentType = MediaType.APPLICATION_XML;
                                response.setContentType(MediaType.APPLICATION_XML);
                            } else if (accept.contains(MediaType.APPLICATION_JSON) || accept.contains(ACCEPT_ANY)) {
                                contentType = MediaType.APPLICATION_JSON;
                                response.setContentType(MediaType.APPLICATION_JSON);
                            } else {
                                contentType = MediaType.TEXT_PLAIN;
                                response.setContentType(MediaType.TEXT_PLAIN);
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
                                response.sendError(status, (String) event.getBody());
                                holder.context.complete();
                                return null;
                            }
                        }
                        // With the exception of HEAD method, HTTP response may have a body
                        if (!HEAD.equals(holder.method)) {
                            // output is a stream?
                            Object resBody = event.getBody();
                            if (resBody == null && streamId != null) {
                                ObjectStreamIO io = new ObjectStreamIO(streamId);
                                ObjectStreamReader in = io.getInputStream(getReadTimeout(timeoutOverride, holder.timeout));
                                try {
                                    OutputStream out = response.getOutputStream();
                                    for (Object block : in) {
                                        // update last access time
                                        holder.touch();
                                        /*
                                         * only bytes or text are supported when using output stream
                                         * e.g. for downloading a large file
                                         */
                                        if (block instanceof byte[]) {
                                            out.write((byte[]) block);
                                        }
                                        if (block instanceof String) {
                                            out.write(util.getUTF((String) block));
                                        }
                                    }
                                } catch (IOException | RuntimeException e) {
                                    log.warn("{} output stream {} interrupted - {}", holder.url, streamId, e.getMessage());
                                    if (e.getMessage().contains("timeout")) {
                                        response.sendError(408, e.getMessage());
                                    } else {
                                        response.sendError(500, e.getMessage());
                                    }
                                } finally {
                                    in.close();
                                }
                                // regular output
                            } else if (resBody instanceof Map) {
                                if (contentType.startsWith(MediaType.TEXT_HTML)) {
                                    byte[] payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(resBody);
                                    OutputStream out = response.getOutputStream();
                                    out.write(util.getUTF(HTML_START));
                                    out.write(payload);
                                    out.write(util.getUTF(HTML_END));
                                } else if (contentType.startsWith(MediaType.APPLICATION_XML)) {
                                    response.getOutputStream().write(util.getUTF(xmlWriter.write(resBody)));
                                } else {
                                    byte[] payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(resBody);
                                    response.getOutputStream().write(payload);
                                }
                            } else if (resBody instanceof List) {
                                if (contentType.startsWith(MediaType.TEXT_HTML)) {
                                    byte[] payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(resBody);
                                    OutputStream out = response.getOutputStream();
                                    out.write(util.getUTF(HTML_START));
                                    out.write(payload);
                                    out.write(util.getUTF(HTML_END));
                                } else if (contentType.startsWith(MediaType.APPLICATION_XML)) {
                                    // xml must be delivered as a map so we use a wrapper here
                                    Map<String, Object> map = new HashMap<>();
                                    map.put(RESULT, resBody);
                                    response.getOutputStream().write(util.getUTF(xmlWriter.write(map)));
                                } else {
                                    byte[] payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(resBody);
                                    response.getOutputStream().write(payload);
                                }
                            } else if (resBody instanceof String) {
                                String text = (String) resBody;
                                response.getOutputStream().write(util.getUTF(text));
                            } else if (resBody instanceof byte[]) {
                                byte[] binary = (byte[]) resBody;
                                response.getOutputStream().write(binary);
                            } else if (resBody != null) {
                                response.getOutputStream().write(util.getUTF(resBody.toString()));
                            }
                        }
                    }
                    holder.context.complete();
                }
            }
        }
        return null;
    }

}
