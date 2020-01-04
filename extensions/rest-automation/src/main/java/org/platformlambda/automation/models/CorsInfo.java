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

package org.platformlambda.automation.models;

import java.util.HashMap;
import java.util.Map;

public class CorsInfo {

    private static final String ACCESS_CONTROL_ORIGIN = "access-control-allow-origin";
    private static final String HTTP = "http://";
    private static final String HTTPS = "https://";
    private String origin;
    public String id;
    public Map<String, String> options = new HashMap<>();
    public Map<String, String> headers = new HashMap<>();

    public CorsInfo(String id, Object origin) {
        this.id = id;
        this.origin = getOrigin(origin);
    }

    public String getOrigin(boolean isOption) {
        return isOption? options.get(ACCESS_CONTROL_ORIGIN) : headers.get(ACCESS_CONTROL_ORIGIN);
    }

    public void addOption(String element) {
        int colon = element.indexOf(':');
        String key = element.substring(0, colon).trim().toLowerCase();
        String value = element.substring(colon+1).trim();
        if (key.equals(ACCESS_CONTROL_ORIGIN) && origin != null) {
            options.put(key, origin);
        } else {
            options.put(key, value);
        }
    }

    public void addHeader(String element) {
        int colon = element.indexOf(':');
        String key = element.substring(0, colon).trim().toLowerCase();
        String value = element.substring(colon+1).trim();
        if (key.equals(ACCESS_CONTROL_ORIGIN) && origin != null) {
            headers.put(key, origin);
        } else {
            headers.put(key, value);
        }
    }

    private String getOrigin(Object origin) {
        if (origin instanceof String) {
            String o = (String) origin;
            if ("*".equals(o) || o.startsWith(HTTP) || o.startsWith(HTTPS)) {
                return o;
            }
        }
        return null;
    }

}
