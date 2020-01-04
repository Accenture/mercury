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

import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class UserChannels {

    private String userId;
    private ConcurrentMap<String, Boolean> paths = new ConcurrentHashMap<>();
    private long lastUpdate = System.currentTimeMillis();

    public UserChannels(String userId, String txPath) {
        this.userId = userId;
        this.addPath(txPath);
    }

    public String getUserId() {
        return userId;
    }

    public UserChannels touch() {
        lastUpdate = System.currentTimeMillis();
        return this;
    }

    public void addPath(String txPath) {
        paths.put(txPath, true);
    }

    public void removePath(String txPath) {
        paths.remove(txPath);
    }

    public boolean isEmpty() {
        return paths.isEmpty();
    }

    public List<String> getPaths() {
        return new ArrayList<>(paths.keySet());
    }

    @Override
    public String toString() {
        return "Channels("+userId+", "+paths+", "+new Date(lastUpdate)+")";
    }

}
