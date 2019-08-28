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

import org.platformlambda.core.util.Utility;

import java.util.Date;
import java.util.HashMap;
import java.util.Map;

public class TraceInfo {

    public String id, path;
    public String startTime;
    public Map<String, String> annotations = new HashMap<>();

    public TraceInfo(String id, String path) {
        if (id == null) {
            throw new IllegalArgumentException("trace id must not be null");
        }
        this.id = id;
        this.path = path == null? "?" : path;
        this.startTime = Utility.getInstance().date2str(new Date());
    }

    public void annotate(String key, String value) {
        annotations.put(key, value);
    }

    public String toString() {
        return id +" "+annotations;
    }

}