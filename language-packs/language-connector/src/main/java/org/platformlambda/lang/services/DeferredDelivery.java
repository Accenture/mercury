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
        if (body instanceof EventEnvelope) {
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
                        Map<String, String> relayHeaders = (Map<String, String>) map.get(HEADERS);
                        for (String h: relayHeaders.keySet()) {
                            deferred.setHeader(h, relayHeaders.get(h));
                        }
                    }
                    if (map.containsKey(BODY)) {
                        deferred.setBody(map.get(BODY));
                    }
                    PostOffice.getInstance().sendLater(deferred, new Date(System.currentTimeMillis() + ms));
                }
            }
        }
        return null;
    }

}
