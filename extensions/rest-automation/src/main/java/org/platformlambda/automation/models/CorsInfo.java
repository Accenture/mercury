package org.platformlambda.automation.models;

import java.util.HashMap;
import java.util.Map;

public class CorsInfo {

    public String id;
    public Map<String, String> options = new HashMap<>();
    public Map<String, String> headers = new HashMap<>();

    public CorsInfo(String id) {
        this.id = id;
    }

    public void addOption(String element) {
        int colon = element.indexOf(':');
        String key = element.substring(0, colon).trim();
        String value = element.substring(colon+1).trim();
        options.put(key.toLowerCase(), value);
    }

    public void addHeader(String element) {
        int colon = element.indexOf(':');
        String key = element.substring(0, colon).trim();
        String value = element.substring(colon+1).trim();
        headers.put(key.toLowerCase(), value);
    }

}
