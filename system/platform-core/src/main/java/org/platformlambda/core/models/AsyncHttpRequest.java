/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.core.models;

import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class AsyncHttpRequest {

    private static final String HEADERS = "headers";
    private static final String METHOD = "method";
    private static final String IP = "ip";
    private static final String TIMEOUT = "timeout";
    private static final String SESSION = "session";
    private static final String PARAMETERS = "parameters";
    private static final String QUERY = "query";
    private static final String PATH = "path";
    private static final String COOKIES = "cookies";
    private static final String URL = "url";
    private static final String BODY = "body";
    private static final String STREAM = "stream";
    private static final String FILE_NAME = "filename";
    private static final String CONTENT_LENGTH = "content-length";

    private Map<String, String> headers = new HashMap<>();
    private String method;
    private String url;
    private String ip;
    private Map<String, Object> queryParams = new HashMap<>();
    private Map<String, String> pathParams = new HashMap<>();
    private Map<String, String> cookies = new HashMap<>();
    private Map<String, String> session = new HashMap<>();
    private Object body;
    private String streamRoute;
    private String fileName;
    private int contentLength = -1;
    private int timeoutSeconds = -1;

    public AsyncHttpRequest(Object input) {
        fromMap(input);
    }

    public String getMethod() {
        return method;
    }

    public String getUrl() {
        return url;
    }

    public String getRemoteIp() {
        return ip;
    }

    public Map<String, String> getHeaders() {
        return headers;
    }

    public String getHeader(String key) {
        return headers.get(key.toLowerCase());
    }

    public Object getBody() {
        return body;
    }

    public String getStreamRoute() {
        return streamRoute;
    }

    public boolean isStream() {
        return streamRoute != null;
    }

    public String getFileName() {
        return fileName;
    }

    public boolean isFile() {
        return fileName != null;
    }

    public int getTimeoutSeconds() {
        return timeoutSeconds;
    }

    public int getContentLength() {
        return contentLength;
    }

    public Map<String, String> getSessionInfo() {
        return session;
    }

    public String getSessionInfo(String key) {
        return session.get(key.toLowerCase());
    }

    public Map<String, String> getCookies() {
        return cookies;
    }

    public String getCookie(String key) {
        return cookies.get(key.toLowerCase());
    }

    public Map<String, String> getPathParameters() {
        return pathParams;
    }

    public String getPathParameter(String key) {
        return pathParams.get(key.toLowerCase());
    }

    public Map<String, Object> getQueryParameters() {
        return queryParams;
    }

    /**
     * Use this when you know it is a single value item.
     * @param key of the parameter
     * @return value of the parameter
     */
    @SuppressWarnings("unchecked")
    public String getQueryParameter(String key) {
        Object para = queryParams.get(key.toLowerCase());
        if (para instanceof String) {
            return (String) para;
        } else if (para instanceof List) {
            List<String> params = (List<String>) para;
            if (!params.isEmpty()) {
                return params.get(0);
            }
        }
        return null;
    }

    /**
     * Use this when you know it is a multi-value item.
     * @param key of the parameter
     * @return values of the parameter
     */
    @SuppressWarnings("unchecked")
    public List<String> getQueryParameters(String key) {
        Object para = queryParams.get(key.toLowerCase());
        if (para instanceof String) {
            return Collections.singletonList((String) para);
        } else if (para instanceof List) {
            return (List<String>) para;
        }
        return null;
    }

    @SuppressWarnings("unchecked")
    private void fromMap(Object input) {
        if (input instanceof Map) {
            Map<String, Object> map = (Map<String, Object>) input;
            if (map.containsKey(HEADERS)) {
                headers.putAll((Map<String, String>) map.get(HEADERS));
            }
            if (map.containsKey(COOKIES)) {
                cookies.putAll((Map<String, String>) map.get(COOKIES));
            }
            if (map.containsKey(SESSION)) {
                session.putAll((Map<String, String>) map.get(SESSION));
            }
            if (map.containsKey(METHOD)) {
                method = (String) map.get(METHOD);
            }
            if (map.containsKey(IP)) {
                ip = (String) map.get(IP);
            }
            if (map.containsKey(URL)) {
                url = (String) map.get(URL);
            }
            if (map.containsKey(TIMEOUT)) {
                timeoutSeconds = (int) map.get(TIMEOUT);
            }
            if (map.containsKey(FILE_NAME)) {
                fileName = (String) map.get(FILE_NAME);
            }
            if (map.containsKey(CONTENT_LENGTH)) {
                contentLength = (int) map.get(CONTENT_LENGTH);
            }
            if (map.containsKey(STREAM)) {
                streamRoute = (String) map.get(STREAM);
            }
            if (map.containsKey(BODY)) {
                body = map.get(BODY);
            }
            if (map.containsKey(PARAMETERS)) {
                Map<String, Object> params = (Map<String, Object>) map.get(PARAMETERS);
                if (params.containsKey(PATH)) {
                    pathParams.putAll((Map<String, String>) params.get(PATH));
                }
                if (params.containsKey(QUERY)) {
                    queryParams.putAll((Map<String, String>) params.get(QUERY));
                }
            }
        } else {
            throw new IllegalArgumentException("Input - expect: Map, actual: "+input.getClass().getSimpleName());
        }
    }
}
