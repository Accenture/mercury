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

import java.util.Date;
import java.util.Map;

public class UserSession {

    private String userId, route, txPath, application, ip;
    private Map<String, String> query;
    private long lastUpdate;

    @SuppressWarnings("unchecked")
    public UserSession(String userId, String route, String txPath, Map<String, Object> metadata) {
        this.userId = userId;
        this.route = route;
        this.txPath = txPath;
        this.application = (String) metadata.get("application");
        this.ip = (String) metadata.get("ip");
        this.query = (Map<String, String>) metadata.get("query");
        this.lastUpdate = System.currentTimeMillis();
    }

    public UserSession touch() {
        lastUpdate = System.currentTimeMillis();
        return this;
    }

    public String getUserId() {
        return userId;
    }

    public String getRoute() {
        return route;
    }

    public String getTxPath() {
        return txPath;
    }

    public String getApplication() {
        return application;
    }

    public String getIp() {
        return ip;
    }

    public Map<String, String> getQuery() {
        return query;
    }

    public long getLastUpdate() {
        return lastUpdate;
    }

    @Override
    public String toString() {
        return "Session("+userId+", "+route+", "+txPath+", "+application+", "+ip+", "+query+", "+new Date(lastUpdate)+")";
    }

}
