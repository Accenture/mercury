package org.platformlambda.mock;

import org.platformlambda.core.models.LambdaFunction;

import java.util.HashMap;
import java.util.Map;

public class MockCloudHealth implements LambdaFunction {

    private static final String TYPE = "type";
    private static final String HEALTH = "health";
    private static final String INFO = "info";

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        if (INFO.equals(headers.get(TYPE))) {
            Map<String, Object> result = new HashMap<>();
            result.put("service", "mock-cloud");
            result.put("href", "http://127.0.0.1/health");
            result.put("topics", "on-demand");
            return result;
        }
        if (HEALTH.equals(headers.get(TYPE))) {
            return "mock cloud is running";
        }
        return null;
    }
}
