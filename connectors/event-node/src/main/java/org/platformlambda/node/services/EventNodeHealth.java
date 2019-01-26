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

package org.platformlambda.node.services;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.AppConfigReader;

import java.util.HashMap;
import java.util.Map;

public class EventNodeHealth implements LambdaFunction {

    private static final String TYPE = "type";
    private static final String HEALTH = "health";
    private static final String INFO = "info";

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {

        if (INFO.equals(headers.get(TYPE))) {
            AppConfigReader reader = AppConfigReader.getInstance();
            String port = reader.getProperty("server.port", "8080");
            Map<String, Object> result = new HashMap<>();
            result.put("service", Platform.getInstance().getName());
            result.put("href", "ws://127.0.0.1:"+port+"/ws/events/{id}");
            return result;
        }

        if (HEALTH.equals(headers.get(TYPE))) {
            return "Event node is healthy";
        } else {
            throw new IllegalArgumentException("Usage: type=health");
        }

    }
}
