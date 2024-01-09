/*

    Copyright 2018-2024 Accenture Technology

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

import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;

import java.net.URI;
import java.net.URISyntaxException;
import java.util.*;

public class AsyncHttpRequest {

    private static final PayloadMapper converter = PayloadMapper.getInstance();
    private static final String HTTP_HEADERS = "headers";
    private static final String HTTP_METHOD = "method";
    private static final String IP_ADDRESS = "ip";
    private static final String CLASS = "clazz";
    private static final String TIMEOUT = "timeout";
    private static final String HTTP_SESSION = "session";
    private static final String PARAMETERS = "parameters";
    private static final String HTTP_PROTOCOL = "http://";
    private static final String HTTPS_PROTOCOL = "https://";
    private static final String HTTP_SECURE = "https";
    private static final String QUERY = "query";
    private static final String PATH = "path";
    private static final String HTTP_COOKIES = "cookies";
    private static final String URL_LABEL = "url";
    private static final String HTTP_BODY = "body";
    private static final String FILE_UPLOAD = "upload";
    private static final String STREAM = "stream";
    private static final String FILE_NAME = "filename";
    private static final String CONTENT_LENGTH = "size";
    private static final String TRUST_ALL_CERT = "trust_all_cert";
    private static final String TARGET_HOST = "host";
    private static final String FILE = "file";

    private String method;
    private String queryString;
    private String url;
    private String ip;
    private String upload;
    private String type;
    private Map<String, String> headers = new HashMap<>();
    private Map<String, Object> queryParams = new HashMap<>();
    private Map<String, String> pathParams = new HashMap<>();
    private Map<String, String> cookies = new HashMap<>();
    private Map<String, String> session = new HashMap<>();
    private Object body;
    private String streamRoute;
    private String fileName;
    private String targetHost;
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
        if (method != null) {
            this.method = method.toUpperCase();
        }
        return this;
    }

    public String getUrl() {
        return url == null? "/" : url;
    }

    public AsyncHttpRequest setUrl(String url) {
        if (url != null) {
            this.url = url;
        }
        return this;
    }

    public String getRemoteIp() {
        return ip;
    }

    public AsyncHttpRequest setRemoteIp(String ip) {
        if (ip != null) {
            this.ip = ip;
        }
        return this;
    }

    public Map<String, String> getHeaders() {
        return headers;
    }

    public String getHeader(String key) {
        return key != null? headers.get(key.toLowerCase()) : null;
    }

    public AsyncHttpRequest setHeader(String key, String value) {
        if (key != null) {
            this.headers.put(key.toLowerCase(), value != null? value : "");
        }
        return this;
    }

    /**
     * Restore body to PoJo if class type information is available
     *
     * @return original body
     */
    public Object getBody() {
        if (type == null) {
            return body;
        } else {
            Class<?> cls = PayloadMapper.getInstance().getClassByName(type);
            return cls == null? body : SimpleMapper.getInstance().getMapper().readValue(body, cls);
        }
    }

    /**
     * Get body in raw form without object mapping. i.e. Map or Java primitive
     *
     * @return body
     */
    public Object getRawBody() {
        return body;
    }

    public String getClassType() {
        return type;
    }

    /**
     * Convert body to a specific class
     * <p>
     * This would result in casting exception if using incompatible target class
     *
     * @param toValueType target class
     * @param <T> class type
     * @return target class
     */
    public <T> T getBody(Class<T> toValueType) {
        return SimpleMapper.getInstance().getMapper().readValue(body, toValueType);
    }

    /**
     * Convert body to a parameterizedClass
     *
     * @param toValueType target class
     * @param parameterClass parameter class(es)
     * @param <T> class type
     * @return target class with parameter class(es)
     */
    @SuppressWarnings("unchecked")
    public <T> T getBody(Class<T> toValueType, Class<?>... parameterClass) {
        if (parameterClass.length == 0) {
            throw new IllegalArgumentException("Missing parameter class");
        }
        StringBuilder sb = new StringBuilder();
        for (Class<?> cls: parameterClass) {
            sb.append(cls.getName());
            sb.append(',');
        }
        String parametricType = sb.substring(0, sb.length()-1);
        TypedPayload typed = new TypedPayload(toValueType.getName(), body).setParametricType(parametricType);
        try {
            return (T) converter.decode(typed);
        } catch (ClassNotFoundException e) {
            // this should not occur because the classes are given as arguments
            throw new IllegalArgumentException(e.getMessage());
        }
    }

    /**
     * Set request body
     *
     * @param body will be converted to map if it is a PoJo
     * @return this
     */
    public AsyncHttpRequest setBody(Object body) {
        if (body == null || body instanceof Map || body instanceof List ||
                PayloadMapper.getInstance().isPrimitive(body)) {
            this.body = body;
        } else if (body instanceof Date) {
            this.body = Utility.getInstance().date2str((Date) body);
        } else {
            this.body = SimpleMapper.getInstance().getMapper().readValue(body, Map.class);
            this.type = body.getClass().getName();
        }
        return this;
    }

    public String getStreamRoute() {
        return streamRoute;
    }

    public AsyncHttpRequest setStreamRoute(String streamRoute) {
        if (streamRoute != null) {
            this.streamRoute = streamRoute;
        }
        return this;
    }

    public boolean isStream() {
        return streamRoute != null;
    }

    public String getFileName() {
        return fileName;
    }

    public AsyncHttpRequest setFileName(String fileName) {
        if (fileName != null) {
            this.fileName = fileName;
        }
        return this;
    }

    public boolean isFile() {
        return fileName != null;
    }

    public int getTimeoutSeconds() {
        return Math.max(0, timeoutSeconds);
    }

    public AsyncHttpRequest setTimeoutSeconds(int timeoutSeconds) {
        this.timeoutSeconds = Math.max(0, timeoutSeconds);
        return this;
    }

    public int getContentLength() {
        return Math.max(0, contentLength);
    }

    public AsyncHttpRequest setContentLength(int contentLength) {
        this.contentLength = Math.max(0, contentLength);
        return this;
    }

    public Map<String, String> getSessionInfo() {
        return session;
    }

    public String getSessionInfo(String key) {
        return key != null? session.get(key.toLowerCase()) : null;
    }

    public AsyncHttpRequest setSessionInfo(String key, String value) {
        if (key != null) {
            this.session.put(key.toLowerCase(), value != null? value : "");
        }
        return this;
    }

    public AsyncHttpRequest removeSessionInfo(String key) {
        if (key != null) {
            this.session.remove(key.toLowerCase());
        }
        return this;
    }

    public Map<String, String> getCookies() {
        return cookies;
    }

    public String getCookie(String key) {
        return key != null? cookies.get(key.toLowerCase()) : null;
    }

    public AsyncHttpRequest setCookie(String key, String value) {
        if (key != null) {
            this.cookies.put(key.toLowerCase(), value != null? value : "");
        }
        return this;
    }

    public AsyncHttpRequest removeCookie(String key) {
        if (key != null) {
            this.cookies.remove(key.toLowerCase());
        }
        return this;
    }

    public Map<String, String> getPathParameters() {
        return pathParams;
    }

    public String getPathParameter(String key) {
        return key != null? pathParams.get(key.toLowerCase()) : null;
    }

    public AsyncHttpRequest setPathParameter(String key, String value) {
        if (key != null) {
            this.pathParams.put(key.toLowerCase(), value != null? value : "");
        }
        return this;
    }

    public AsyncHttpRequest removePathParameter(String key) {
        if (key != null) {
            this.pathParams.remove(key.toLowerCase());
        }
        return this;
    }

    public String getQueryString() {
        return queryString;
    }

    public AsyncHttpRequest setQueryString(String queryString) {
        if (queryString != null) {
            String value = queryString.trim();
            this.queryString = value.isEmpty()? null : value;
        } else {
            this.queryString = null;
        }
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
        return upload == null? FILE : upload;
    }

    public AsyncHttpRequest setUploadTag(String tag) {
        if (tag != null) {
            String value = tag.trim();
            this.upload = value.isEmpty()? null : value;
        } else {
            this.upload = null;
        }
        return this;
    }

    public Map<String, Object> getQueryParameters() {
        return queryParams;
    }

    public String getTargetHost() {
        return targetHost;
    }

    public AsyncHttpRequest setTargetHost(String host) {
        if (host != null && (host.startsWith(HTTP_PROTOCOL) || host.startsWith(HTTPS_PROTOCOL))) {
            try {
                URI u = new URI(host);
                if (!u.getPath().isEmpty()) {
                    throw new IllegalArgumentException("Invalid host - Must not contain path");
                }
                if (u.getQuery() != null) {
                    throw new IllegalArgumentException("Invalid host - Must not contain query");
                }
            } catch (URISyntaxException e) {
                throw new IllegalArgumentException("Invalid host - "+e.getMessage());
            }
            this.targetHost = host;
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
        if (key != null) {
            Object value = queryParams.get(key.toLowerCase());
            if (value instanceof String) {
                return (String) value;
            } else if (value instanceof List) {
                List<String> params = (List<String>) value;
                if (!params.isEmpty()) {
                    return params.get(0);
                }
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
        if (key != null) {
            Object values = queryParams.get(key.toLowerCase());
            if (values instanceof String) {
                return Collections.singletonList((String) values);
            } else if (values instanceof List) {
                return (List<String>) values;
            }
        }
        return Collections.emptyList();
    }

    @SuppressWarnings("unchecked")
    public AsyncHttpRequest setQueryParameter(String key, Object value) {
        if (key != null) {
            if (value instanceof String) {
                this.queryParams.put(key.toLowerCase(), value);
            } else if (value instanceof List) {
                List<String> params = new ArrayList<>();
                List<Object> list = (List<Object>) value;
                for (Object o : list) {
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
        }
        return this;
    }

    public AsyncHttpRequest removeQueryParameter(String key) {
        if (key != null) {
            this.queryParams.remove(key.toLowerCase());
        }
        return this;
    }

    /**
     * The set methods and toMap method are used for manually construct an HTTP request object
     * that are typically used for Unit Test or for a service to emulate a REST browser.
     *
     * In normal case, the AsyncHttpRequest map is generated by the rest-automation application.
     *
     * @return async http request object as a map
     */
    public Map<String, Object> toMap() {
        Map<String, Object> result = new HashMap<>();
        if (!headers.isEmpty()) {
            result.put(HTTP_HEADERS, setLowerCase(headers));
        }
        if (!cookies.isEmpty()) {
            result.put(HTTP_COOKIES, setLowerCase(cookies));
        }
        if (!session.isEmpty()) {
            result.put(HTTP_SESSION, setLowerCase(session));
        }
        if (method != null) {
            result.put(HTTP_METHOD, method);
        }
        if (ip != null) {
            result.put(IP_ADDRESS, ip);
        }
        if (type != null) {
            result.put(CLASS, type);
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
            result.put(HTTP_BODY, body);
        }
        if (queryString != null) {
            result.put(QUERY, queryString);
        }
        if (upload != null) {
            result.put(FILE_UPLOAD, upload);
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
        result.put(HTTP_SECURE, https);
        /*
         * Optional HTTP host name in the "relay" field
         *
         * This is used by the rest-automation "async.http.request" service
         * when forwarding HTTP request to a target HTTP endpoint.
         */
        if (targetHost != null) {
            result.put(TARGET_HOST, targetHost);
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
        if (input instanceof AsyncHttpRequest) {
            AsyncHttpRequest source = (AsyncHttpRequest) input;
            this.headers = source.headers;
            this.cookies = source.cookies;
            this.session = source.session;
            this.method = source.method;
            this.ip = source.ip;
            this.type = source.type;
            this.url = source.url;
            this.timeoutSeconds = source.timeoutSeconds;
            this.fileName = source.fileName;
            this.contentLength = source.contentLength;
            this.streamRoute = source.streamRoute;
            this.body = source.body;
            this.queryString = source.queryString;
            this.https = source.https;
            this.targetHost = source.targetHost;
            this.trustAllCert = source.trustAllCert;
            this.upload = source.upload;
            this.pathParams = source.pathParams;
            this.queryParams = source.queryParams;
        }
        if (input instanceof Map) {
            Map<String, Object> map = (Map<String, Object>) input;
            if (map.containsKey(HTTP_HEADERS)) {
                headers = setLowerCase((Map<String, String>) map.get(HTTP_HEADERS));
            }
            if (map.containsKey(HTTP_COOKIES)) {
                cookies = setLowerCase((Map<String, String>) map.get(HTTP_COOKIES));
            }
            if (map.containsKey(HTTP_SESSION)) {
                session = setLowerCase((Map<String, String>) map.get(HTTP_SESSION));
            }
            if (map.containsKey(HTTP_METHOD)) {
                method = (String) map.get(HTTP_METHOD);
            }
            if (map.containsKey(IP_ADDRESS)) {
                ip = (String) map.get(IP_ADDRESS);
            }
            if (map.containsKey(CLASS)) {
                type = (String) map.get(CLASS);
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
            if (map.containsKey(HTTP_BODY)) {
                body = map.get(HTTP_BODY);
            }
            if (map.containsKey(QUERY)) {
                queryString = (String) map.get(QUERY);
            }
            if (map.containsKey(HTTP_SECURE)) {
                https = (boolean) map.get(HTTP_SECURE);
            }
            if (map.containsKey(TARGET_HOST)) {
                targetHost = (String) map.get(TARGET_HOST);
            }
            if (map.containsKey(TRUST_ALL_CERT)) {
                trustAllCert = (boolean) map.get(TRUST_ALL_CERT);
            }
            if (map.containsKey(FILE_UPLOAD)) {
                upload = (String) map.get(FILE_UPLOAD);
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
        }
    }

}
