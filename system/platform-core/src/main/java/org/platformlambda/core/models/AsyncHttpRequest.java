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

package org.platformlambda.core.models;

import java.net.MalformedURLException;
import java.net.URL;
import java.util.*;

public class AsyncHttpRequest {

    private static final String HEADERS = "headers";
    private static final String METHOD = "method";
    private static final String IP = "ip";
    private static final String TIMEOUT = "timeout";
    private static final String SESSION = "session";
    private static final String PARAMETERS = "parameters";
    private static final String HTTP_PROTOCOL = "http://";
    private static final String HTTPS_PROTOCOL = "https://";
    private static final String HTTPS = "https";
    private static final String QUERY = "query";
    private static final String PATH = "path";
    private static final String COOKIES = "cookies";
    private static final String URL_LABEL = "url";
    private static final String BODY = "body";
    private static final String UPLOAD = "upload";
    private static final String STREAM = "stream";
    private static final String FILE_NAME = "filename";
    private static final String CONTENT_LENGTH = "size";
    private static final String TRUST_ALL_CERT = "trust_all_cert";
    private static final String RELAY = "relay";

    private String method;
    private String queryString;
    private String url;
    private String ip;
    private String upload;
    private Map<String, String> headers = new HashMap<>();
    private Map<String, Object> queryParams = new HashMap<>();
    private Map<String, String> pathParams = new HashMap<>();
    private Map<String, String> cookies = new HashMap<>();
    private Map<String, String> session = new HashMap<>();
    private Object body;
    private String streamRoute;
    private String fileName;
    private String relay;
    private boolean trustAllCert = false;
    private boolean https = false;
    private int contentLength = -1;
    private int timeoutSeconds = -1;

    public AsyncHttpRequest() { }

    public AsyncHttpRequest(Object input) {
        fromMap(input);
    }

    public String getMethod() {
        return method;
    }

    public AsyncHttpRequest setMethod(String method) {
        this.method = method;
        return this;
    }

    public String getUrl() {
        return url;
    }

    public AsyncHttpRequest setUrl(String url) {
        this.url = url;
        return this;
    }

    public String getRemoteIp() {
        return ip;
    }

    public AsyncHttpRequest setRemoteIp(String ip) {
        this.ip = ip;
        return this;
    }

    public Map<String, String> getHeaders() {
        return headers;
    }

    public String getHeader(String key) {
        return headers.get(key.toLowerCase());
    }

    public AsyncHttpRequest setHeader(String key, String value) {
        this.headers.put(key.toLowerCase(), value);
        return this;
    }

    public Object getBody() {
        return body;
    }

    public AsyncHttpRequest setBody(Object body) {
        this.body = body;
        return this;
    }

    public String getStreamRoute() {
        return streamRoute;
    }

    public AsyncHttpRequest setStreamRoute(String streamRoute) {
        this.streamRoute = streamRoute;
        return this;
    }

    public boolean isStream() {
        return streamRoute != null;
    }

    public String getFileName() {
        return fileName;
    }

    public AsyncHttpRequest setFileName(String fileName) {
        this.fileName = fileName;
        return this;
    }

    public boolean isFile() {
        return fileName != null;
    }

    public int getTimeoutSeconds() {
        return timeoutSeconds;
    }

    public AsyncHttpRequest setTimeoutSeconds(int timeoutSeconds) {
        this.timeoutSeconds = timeoutSeconds;
        return this;
    }

    public int getContentLength() {
        return contentLength;
    }

    public AsyncHttpRequest setContentLength(int contentLength) {
        this.contentLength = contentLength;
        return this;
    }

    public Map<String, String> getSessionInfo() {
        return session;
    }

    public String getSessionInfo(String key) {
        return session.get(key.toLowerCase());
    }

    public AsyncHttpRequest setSessionInfo(String key, String value) {
        this.session.put(key.toLowerCase(), value);
        return this;
    }

    public AsyncHttpRequest removeSessionInfo(String key) {
        this.session.remove(key.toLowerCase());
        return this;
    }

    public Map<String, String> getCookies() {
        return cookies;
    }

    public String getCookie(String key) {
        return cookies.get(key.toLowerCase());
    }

    public AsyncHttpRequest setCookie(String key, String value) {
        this.cookies.put(key.toLowerCase(), value);
        return this;
    }

    public AsyncHttpRequest removeCookie(String key) {
        this.cookies.remove(key.toLowerCase());
        return this;
    }

    public Map<String, String> getPathParameters() {
        return pathParams;
    }

    public String getPathParameter(String key) {
        return pathParams.get(key.toLowerCase());
    }

    public AsyncHttpRequest setPathParameter(String key, String value) {
        this.pathParams.put(key.toLowerCase(), value);
        return this;
    }

    public AsyncHttpRequest removePathParameter(String key) {
        this.pathParams.remove(key.toLowerCase());
        return this;
    }

    public String getQueryString() {
        return queryString;
    }

    public AsyncHttpRequest setQueryString(String queryString) {
        this.queryString = queryString;
        return this;
    }

    public boolean isSecure() {
        return https;
    }

    public AsyncHttpRequest setSecure(boolean https) {
        this.https = https;
        return this;
    }

    public String getUploadTag() {
        return upload;
    }

    public AsyncHttpRequest setUploadTag(String tag) {
        this.upload = tag;
        return this;
    }

    public Map<String, Object> getQueryParameters() {
        return queryParams;
    }

    public String getRelay() {
        return relay;
    }

    public AsyncHttpRequest setRelay(String host) {
        if (host != null && (host.startsWith(HTTP_PROTOCOL) || host.startsWith(HTTPS_PROTOCOL))) {
            try {
                URL u = new URL(host);
                if (!u.getPath().isEmpty()) {
                    throw new IllegalArgumentException("Invalid host - Must not contain path");
                }
                if (u.getQuery() != null) {
                    throw new IllegalArgumentException("Invalid host - Must not contain query");
                }
            } catch (MalformedURLException e) {
                throw new IllegalArgumentException("Invalid host - "+e.getMessage());
            }
            this.relay = host;
            return this;
        } else {
            throw new IllegalArgumentException("Invalid host - must starts with "+HTTP_PROTOCOL+" or "+HTTPS_PROTOCOL);
        }
    }

    public boolean isTrustAllCert() {
        return trustAllCert;
    }

    public AsyncHttpRequest setTrustAllCert(boolean trustAllCert) {
        this.trustAllCert = trustAllCert;
        return this;
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
    public AsyncHttpRequest setQueryParameter(String key, Object value) {
        if (value instanceof String) {
            this.queryParams.put(key.toLowerCase(), value);
        } else if (value instanceof List) {
            List<String> params = new ArrayList<>();
            List<Object> list = (List<Object>) value;
            for (Object o: list) {
                if (o != null) {
                    params.add(o instanceof String ? (String) o : o.toString());
                }
            }
            this.queryParams.put(key.toLowerCase(), params);
        } else if (value == null) {
            this.queryParams.put(key.toLowerCase(), "");
        } else {
            this.queryParams.put(key.toLowerCase(), value.toString());
        }
        return this;
    }

    public AsyncHttpRequest removeQueryParameter(String key) {
        this.queryParams.remove(key.toLowerCase());
        return this;
    }

    /**
     * The set methods and toMap method are used for manually construct a HTTP request object
     * that are typically used for Unit Test or for a service to emulate a REST browser.
     *
     * In normal case, the AsyncHttpRequest map is generated by the rest-automation application.
     *
     * @return async http request object as a map
     */
    public Map<String, Object> toMap() {
        Map<String, Object> result = new HashMap<>();
        if (!headers.isEmpty()) {
            result.put(HEADERS, setLowerCase(headers));
        }
        if (!cookies.isEmpty()) {
            result.put(COOKIES, setLowerCase(cookies));
        }
        if (!session.isEmpty()) {
            result.put(SESSION, setLowerCase(session));
        }
        if (method != null) {
            result.put(METHOD, method);
        }
        if (ip != null) {
            result.put(IP, ip);
        }
        if (url != null) {
            result.put(URL_LABEL, url);
        }
        if (timeoutSeconds != -1) {
            result.put(TIMEOUT, timeoutSeconds);
        }
        if (fileName != null) {
            result.put(FILE_NAME, fileName);
        }
        if (contentLength != -1) {
            result.put(CONTENT_LENGTH, contentLength);
        }
        if (streamRoute != null) {
            result.put(STREAM, streamRoute);
        }
        if (body != null) {
            result.put(BODY, body);
        }
        if (queryString != null) {
            result.put(QUERY, queryString);
        }
        if (upload != null) {
            result.put(UPLOAD, upload);
        }
        if (!pathParams.isEmpty() || !queryParams.isEmpty()) {
            Map<String, Object> parameters = new HashMap<>();
            result.put(PARAMETERS, parameters);
            if (!pathParams.isEmpty()) {
                parameters.put(PATH, setLowerCase(pathParams));
            }
            if (!queryParams.isEmpty()) {
                parameters.put(QUERY, setLowerCaseQuery(queryParams));
            }
        }
        result.put(HTTPS, https);
        /*
         * Optional HTTP host name in the "relay" field
         *
         * This is used by the rest-automation "async.http.request" service
         * when forwarding HTTP request to a target HTTP endpoint.
         */
        if (relay != null) {
            result.put(RELAY, relay);
            result.put(TRUST_ALL_CERT, trustAllCert);
        }
        return result;
    }

    private Map<String, String> setLowerCase(Map<String, String> source) {
        Map<String, String> result = new HashMap<>();
        for (String key: source.keySet()) {
            result.put(key.toLowerCase(), source.get(key));
        }
        return result;
    }

    private Map<String, Object> setLowerCaseQuery(Map<String, Object> source) {
        Map<String, Object> result = new HashMap<>();
        for (String key: source.keySet()) {
            result.put(key.toLowerCase(), source.get(key));
        }
        return result;
    }

    @SuppressWarnings("unchecked")
    private void fromMap(Object input) {
        if (input instanceof Map) {
            Map<String, Object> map = (Map<String, Object>) input;
            if (map.containsKey(HEADERS)) {
                headers = setLowerCase((Map<String, String>) map.get(HEADERS));
            }
            if (map.containsKey(COOKIES)) {
                cookies = setLowerCase((Map<String, String>) map.get(COOKIES));
            }
            if (map.containsKey(SESSION)) {
                session = setLowerCase((Map<String, String>) map.get(SESSION));
            }
            if (map.containsKey(METHOD)) {
                method = (String) map.get(METHOD);
            }
            if (map.containsKey(IP)) {
                ip = (String) map.get(IP);
            }
            if (map.containsKey(URL_LABEL)) {
                url = (String) map.get(URL_LABEL);
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
            if (map.containsKey(QUERY)) {
                queryString = (String) map.get(QUERY);
            }
            if (map.containsKey(HTTPS)) {
                https = (boolean) map.get(HTTPS);
            }
            if (map.containsKey(RELAY)) {
                relay = (String) map.get(RELAY);
            }
            if (map.containsKey(TRUST_ALL_CERT)) {
                trustAllCert = (boolean) map.get(TRUST_ALL_CERT);
            }
            if (map.containsKey(UPLOAD)) {
                upload = (String) map.get(UPLOAD);
            }
            if (map.containsKey(PARAMETERS)) {
                Map<String, Object> parameters = (Map<String, Object>) map.get(PARAMETERS);
                if (parameters.containsKey(PATH)) {
                    pathParams = setLowerCase((Map<String, String>) parameters.get(PATH));
                }
                if (parameters.containsKey(QUERY)) {
                    queryParams = setLowerCaseQuery((Map<String, Object>) parameters.get(QUERY));
                }
            }
        } else {
            throw new IllegalArgumentException("Expect: Map, actual: "+input.getClass().getSimpleName());
        }
    }
}
