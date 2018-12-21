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
