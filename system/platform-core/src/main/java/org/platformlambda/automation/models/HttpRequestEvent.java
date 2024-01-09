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

package org.platformlambda.automation.models;

import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.util.Utility;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class HttpRequestEvent {

    private static final String REQUEST_ID = "1";
    private static final String HTTP_REQUEST = "2";
    private static final String PRIMARY = "3";
    private static final String AUTH_SERVICE = "4";
    private static final String TRACE_ID = "5";
    private static final String TRACE_PATH = "6";
    private static final String SERVICES = "7";
    private static final String TIMEOUT = "8";
    private static final String TRACING = "9";

    public String requestId;
    public String primary;
    public Map<String, Object> httpRequest;
    public String authService;
    public String traceId;
    public String tracePath;
    public List<String> services;
    public long timeout;
    public boolean tracing;

    @SuppressWarnings("unchecked")
    public HttpRequestEvent(Object data) {
        if (data instanceof Map) {
            Map<String, Object> map = (Map<String, Object>) data;
            this.requestId = (String) map.get(REQUEST_ID);
            this.primary = (String) map.get(PRIMARY);
            this.httpRequest = (Map<String, Object>) map.get(HTTP_REQUEST);
            this.authService = (String) map.get(AUTH_SERVICE);
            this.traceId = (String) map.get(TRACE_ID);
            this.tracePath = (String) map.get(TRACE_PATH);
            this.services = (List<String>) map.get(SERVICES);
            this.timeout = Utility.getInstance().str2long(map.get(TIMEOUT).toString());
            this.tracing = (boolean) map.get(TRACING);
        }
    }

    public HttpRequestEvent(String requestId,
                            String primary, String authService, String traceId, String tracePath,
                            List<String> services, long timeout, boolean tracing) {
        this.requestId = requestId;
        this.primary = primary;
        this.authService = authService;
        this.traceId = traceId;
        this.tracePath = tracePath;
        this.services = services;
        this.timeout = timeout;
        this.tracing = tracing;
    }

    public HttpRequestEvent setHttpRequest(AsyncHttpRequest request) {
        this.httpRequest = request.toMap();
        return this;
    }

    public Map<String, Object> toMap() {
        Map<String, Object> result = new HashMap<>();
        result.put(REQUEST_ID, requestId);
        result.put(PRIMARY, primary);
        result.put(HTTP_REQUEST, httpRequest);
        result.put(AUTH_SERVICE, authService);
        result.put(TRACE_ID, traceId);
        result.put(TRACE_PATH, tracePath);
        result.put(SERVICES, services);
        result.put(TIMEOUT, timeout);
        result.put(TRACING, tracing);
        return result;
    }

}
