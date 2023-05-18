/*

    Copyright 2018-2023 Accenture Technology

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

import java.util.*;

public class RouteInfo {

    private List<String> authHeaders = new ArrayList<>();
    private Map<String, String> authServices = new HashMap<>();

    public String url, defaultAuthService, corsId, requestTransformId, responseTransformId, primary;
    public List<String> services;
    public int threshold = 50000;
    public boolean tracing = false;
    public List<String> methods;
    public int timeoutSeconds = 30;
    public boolean upload = false;
    // optional for HTTP relay
    public String host;
    public String flowId;
    public boolean trustAllCert = false;
    public List<String> urlRewrite;

    public String getAuthService(String headerKey) {
        return getAuthService(headerKey, "*");
    }

    public String getAuthService(String headerKey, String headerValue) {
        if (headerKey == null) {
            throw new IllegalArgumentException("HTTP header cannot be null");
        }
        return authServices.get(headerKey.toLowerCase()+":"+headerValue.toLowerCase());
    }

    public void setAuthService(String headerKey, String headerValue, String service) {
        if (headerKey != null && headerValue != null && !headerKey.isEmpty() && !headerValue.isEmpty()) {
            String lh = headerKey.toLowerCase();
            if (!authHeaders.contains(lh)) {
                authHeaders.add(lh);
            }
            authServices.put(lh + ":" + headerValue.toLowerCase(), service);
        }
    }

    public List<String> getAuthHeaders() {
        return authHeaders;
    }

}
