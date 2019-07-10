package org.platformlambda.automation.models;

import java.util.HashMap;
import java.util.Map;

public class AssignedRoute {

    public Map<String, String> arguments = new HashMap<>();

    public RouteInfo info;

    public AssignedRoute(RouteInfo info) {
        this.info = info;
    }

    public void setArgument(String key, String value) {
        arguments.put(key, value);
    }

}
