/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.automation.services;

import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.LambdaFunction;

import java.util.Date;
import java.util.HashMap;
import java.util.Map;

public class NotificationQuery implements LambdaFunction {
    private static final String TOPIC = "topic";
    private static final String TOPICS = "topics";
    private static final String TIME = "time";

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (body instanceof Map) {
            AsyncHttpRequest request = new AsyncHttpRequest(body);
            if (request.getUrl() != null) {
                String topic = request.getPathParameter(TOPIC);
                Map<String, Object> result = new HashMap<>();
                result.put(TIME, new Date());
                if (topic == null) {
                    result.put(TOPICS, SimpleNotification.getInstance().listTopics());
                } else {
                    result.put(topic, SimpleNotification.getInstance().getTopic(topic));
                }
                return result;
            }
        }
        throw new IllegalArgumentException("Invalid HTTP request");
    }
}
