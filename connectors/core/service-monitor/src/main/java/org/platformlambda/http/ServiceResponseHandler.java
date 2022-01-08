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

import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.HttpServerResponse;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.AsyncContextHolder;

import java.util.Map;
import java.util.concurrent.ConcurrentMap;

@EventInterceptor
public class ServiceResponseHandler implements LambdaFunction {

    private static final SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
    private static final String APPLICATION_JSON = "application/json";
    private static final String APPLICATION_XML = "application/xml";
    private static final String TEXT_HTML = "text/html";
    private static final String HEAD = "HEAD";
    private static final String CONTENT_TYPE = "Content-Type";
    private static final String CONTENT_LEN = "Content-Length";
    private static final String HTML_START = "<!DOCTYPE html>\n<html>\n<body>\n<pre>\n";
    private static final String HTML_END = "\n</pre>\n<body>\n</html>";

    private final ConcurrentMap<String, AsyncContextHolder> contexts;

    public ServiceResponseHandler(ConcurrentMap<String, AsyncContextHolder> contexts) {
        this.contexts = contexts;
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
                String contentType = null;
                if (!event.getHeaders().isEmpty()) {
                    Map<String, String> evtHeaders = event.getHeaders();
                    for (String h : evtHeaders.keySet()) {
                        String key = h.toLowerCase();
                        String value = evtHeaders.get(h);
                        if (key.equalsIgnoreCase(CONTENT_TYPE)) {
                            contentType = value.toLowerCase();
                            response.putHeader(CONTENT_TYPE, contentType);
                        }
                    }
                }
                if (contentType == null && accept != null) {
                    if (accept.contains(TEXT_HTML)) {
                        contentType = TEXT_HTML;
                        response.putHeader(CONTENT_TYPE, TEXT_HTML);
                    } else if (accept.contains(APPLICATION_XML)) {
                        contentType = APPLICATION_XML;
                        response.putHeader(CONTENT_TYPE, APPLICATION_XML);
                    } else {
                        contentType = APPLICATION_JSON;
                        response.putHeader(CONTENT_TYPE, APPLICATION_JSON);
                    }
                }
                if (contentType == null) {
                    contentType = "?";
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
                    Object responseBody = event.getBody();
                    if (responseBody instanceof Map) {
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
                    }
                    if (responseBody instanceof String) {
                        byte[] payload = util.getUTF((String) responseBody);
                        response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                        response.write(Buffer.buffer(payload));
                    }
                }
                HttpRequestHandler.closeContext(requestId);
                response.end();
            }
        }
        return null;
    }

}
