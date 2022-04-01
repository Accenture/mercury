/*

    Copyright 2018-2022 Accenture Technology

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
package org.platformlambda.lang.services;

import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;

import java.util.Date;
import java.util.Map;

@EventInterceptor
public class DeferredDelivery implements LambdaFunction {

    private static final String ROUTE = "route";
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String SECONDS = "seconds";
    private static final int ONE_MINUTE = 60 * 1000;
    private static final int ONE_HOUR = 60 * ONE_MINUTE;
    private static final int ONE_DAY = 24 * ONE_HOUR;

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        EventEnvelope event = (EventEnvelope) body;
        Object relay = event.getBody();
        if (relay instanceof Map) {
            Map<String, Object> map = (Map<String, Object>) relay;
            if (map.containsKey(ROUTE) && map.containsKey(SECONDS)) {
                int ms = Math.round(Utility.getInstance().str2float(map.get(SECONDS).toString()) * 1000);
                // reset to minimum value of 100 ms
                if (ms < 100) {
                    ms = 100;
                }
                // reset to maximum value of one day
                if (ms > ONE_DAY) {
                    ms = ONE_DAY;
                }
                String route = (String) map.get(ROUTE);
                EventEnvelope deferred = new EventEnvelope();
                deferred.setTo(route);
                if (map.containsKey(HEADERS)) {
                    // relay headers
                    deferred.setHeaders((Map<String, String>) map.get(HEADERS));
                }
                if (map.containsKey(BODY)) {
                    deferred.setBody(map.get(BODY));
                }
                PostOffice.getInstance().sendLater(deferred, new Date(System.currentTimeMillis() + ms));
            }
        }
        return null;
    }

}
