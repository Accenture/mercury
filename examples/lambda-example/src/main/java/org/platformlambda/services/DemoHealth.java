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

package org.platformlambda.services;

import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.LambdaFunction;

import java.util.HashMap;
import java.util.Map;

@PreLoad(route="demo.health", instances=5)
public class DemoHealth implements LambdaFunction {

    private static final String TYPE = "type";
    private static final String INFO = "info";
    private static final String HEALTH = "health";

    @Override
    public Object handleEvent(Map<String, String> headers, Object input, int instance) {
        /*
         * The interface contract for a health check service includes both INFO and HEALTH responses
         */
        if (INFO.equals(headers.get(TYPE))) {
            Map<String, Object> result = new HashMap<>();
            result.put("service", "demo.service");
            result.put("href", "http://127.0.0.1");
            return result;
        }
        if (HEALTH.equals(headers.get(TYPE))) {
            /*
             * This is a place-holder for checking a downstream service.
             *
             * You may implement your own logic to test if a downstream service is running fine.
             * If running, just return a health status message.
             * Otherwise,
             *      throw new AppException(status, message)
             */
            return "demo.service is running fine";
        }
        throw new IllegalArgumentException("type must be info or health");
    }
}
