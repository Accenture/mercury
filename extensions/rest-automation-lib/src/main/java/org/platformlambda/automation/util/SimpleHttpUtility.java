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

package org.platformlambda.automation.util;

import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.HttpServerRequest;
import io.vertx.core.http.HttpServerResponse;
import org.platformlambda.automation.models.HeaderInfo;
import org.platformlambda.automation.services.ServiceGateway;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.util.Utility;

import java.io.InputStream;
import java.io.UnsupportedEncodingException;
import java.net.URLDecoder;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class SimpleHttpUtility {

    private static final SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
    private static final String DATE = "Date";
    private static final String SET_COOKIE = "set-cookie";
    private static final String COOKIE_SEPARATOR = "|";
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

    public void setCookies(HttpServerResponse response, String cookies) {
        String header = getHeaderCase(SET_COOKIE);
        if (cookies.contains(COOKIE_SEPARATOR)) {
            List<String> items = Utility.getInstance().split(cookies, COOKIE_SEPARATOR);
            for (String value: items) {
                response.putHeader(header, value);
            }
        } else {
            response.putHeader(header, cookies);
        }
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

    public Map<String, String> filterHeaders(HeaderInfo headerInfo, Map<String, String> headers) {
        Map<String, String> result = new HashMap<>(headers);
        if (headerInfo.keepHeaders != null && !headerInfo.keepHeaders.isEmpty()) {
            // drop all headers except those to be kept
            Map<String, String> toBeKept = new HashMap<>();
            for (String h: headers.keySet()) {
                if (headerInfo.keepHeaders.contains(h)) {
                    toBeKept.put(h, headers.get(h));
                }
            }
            result = toBeKept;
        } else if (headerInfo.dropHeaders != null && !headerInfo.dropHeaders.isEmpty()) {
            // drop the headers according to "drop" list
            Map<String, String> toBeKept = new HashMap<>();
            for (String h: headers.keySet()) {
                if (!headerInfo.dropHeaders.contains(h)) {
                    toBeKept.put(h, headers.get(h));
                }
            }
            result = toBeKept;
        }
        if (headerInfo.additionalHeaders != null && !headerInfo.additionalHeaders.isEmpty()) {
            for (String h: headerInfo.additionalHeaders.keySet()) {
                result.put(h, headerInfo.additionalHeaders.get(h));
            }
        }
        return result;
    }

    public String normalizeUrl(String url, List<String> urlRewrite) {
        if (urlRewrite != null && urlRewrite.size() == 2) {
            if (url.startsWith(urlRewrite.get(0))) {
                return urlRewrite.get(1) + url.substring(urlRewrite.get(0).length());
            }
        }
        return url;
    }

    public Map<String, String> decodeQueryString(String query) {
        Map<String, String> result = new HashMap<>();
        Utility util = Utility.getInstance();
        List<String> segments = util.split(query, "&");
        try {
            for (String para : segments) {
                int eq = para.indexOf('=');
                if (eq == -1) {
                    result.put(URLDecoder.decode(para, "UTF-8"), "");
                } else {
                    String key = para.substring(0, eq);
                    String value = para.substring(eq + 1);
                    result.put(URLDecoder.decode(key, "UTF-8"), URLDecoder.decode(value, "UTF-8"));
                }
            }
        } catch (UnsupportedEncodingException e) {
            // ok to ignore because UTF-8 is valid
        }
        return result;
    }

    public void sendResponse(String requestId, HttpServerRequest request, int status, String message) {
        ServiceGateway.closeContext(requestId);
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
            result.put("type", status < 400? "event" : "error");
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
