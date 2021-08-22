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

package org.platformlambda.http;

import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.HttpServerRequest;
import io.vertx.core.http.HttpServerResponse;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.AsyncContextHolder;

import java.io.InputStream;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class SimpleHttpUtility {

    private static final SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
    private static final String DATE = "Date";
    private static final String ACCEPT = "Accept";
    private static final String CONTENT_TYPE = "Content-Type";
    private static final String CONTENT_LEN = "Content-Length";
    private static final String APPLICATION_JSON = "application/json";
    private static final String APPLICATION_XML = "application/xml";
    private static final String TEXT_HTML = "text/html";
    private static final String TEMPLATE = "/errorPage.html";
    private static final String HTTP_UNKNOWN_WARNING = "There may be a problem in processing your request";
    private static final String HTTP_400_WARNING = "The system is unable to process your request";
    private static final String HTTP_500_WARNING = "Something may be broken";
    private static final String SET_MESSAGE = "${message}";
    private static final String SET_PATH = "${path}";
    private static final String SET_STATUS = "${status}";
    private static final String SET_WARNING = "${warning}";
    // requestId -> context
    private static final ConcurrentMap<String, AsyncContextHolder> contexts = new ConcurrentHashMap<>();
    private final String template;

    private static final SimpleHttpUtility instance = new SimpleHttpUtility();

    private SimpleHttpUtility() {
        Utility util = Utility.getInstance();
        InputStream in = this.getClass().getResourceAsStream(TEMPLATE);
        template = util.stream2str(in);
    }

    public static SimpleHttpUtility getInstance() {
        return instance;
    }

    public String getHeaderCase(String header) {
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

    public void sendResponse(String requestId, HttpServerRequest request, int status, String message) {
        HttpRequestHandler.closeContext(requestId);
        String accept = request.getHeader(ACCEPT);
        if (accept == null) {
            accept = "?";
        }
        Utility util = Utility.getInstance();
        HttpServerResponse response = request.response().setStatusCode(status);
        if (accept.startsWith(TEXT_HTML)) {
            String errorPage = template.replace(SET_STATUS, String.valueOf(status))
                    .replace(SET_PATH, request.path())
                    .replace(SET_MESSAGE, message);
            if (status >= 500) {
                errorPage = errorPage.replace(SET_WARNING, HTTP_500_WARNING);
            } else if (status >= 400) {
                errorPage = errorPage.replace(SET_WARNING, HTTP_400_WARNING);
            } else {
                errorPage = errorPage.replace(SET_WARNING, HTTP_UNKNOWN_WARNING);
            }
            byte[] payload = util.getUTF(errorPage);
            response.putHeader(CONTENT_TYPE, TEXT_HTML);
            response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
            response.write(Buffer.buffer(payload));
        } else {
            Map<String, Object> result = new HashMap<>();
            result.put("status", status);
            result.put("message", message);
            result.put("type", status < 400? "ok" : "error");
            result.put("path", request.path());
            if (accept.startsWith(APPLICATION_XML)) {
                byte[] payload = util.getUTF(xmlWriter.write("error", result));
                response.putHeader(CONTENT_TYPE, APPLICATION_XML);
                response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                response.write(Buffer.buffer(payload));
            } else {
                byte[] payload = SimpleMapper.getInstance().getMapper().writeValueAsBytes(result);
                response.putHeader(CONTENT_TYPE, APPLICATION_JSON);
                response.putHeader(CONTENT_LEN, String.valueOf(payload.length));
                response.write(Buffer.buffer(payload));
            }
        }
        response.end();
    }

}

