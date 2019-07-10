package org.platformlambda.automation.models;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class HeaderInfo {

    public String id;
    public Map<String, String> additionalHeaders = new HashMap<>();
    public List<String> keepHeaders = new ArrayList<>();
    public List<String> dropHeaders = new ArrayList<>();

    public HeaderInfo(String id) {
        this.id = id;
    }

    public void addHeader(String key, String value) {
        additionalHeaders.put(key, value);
    }

    public void drop(String header) {
        if (!dropHeaders.contains(header)) {
            dropHeaders.add(header);
        }
    }

    public void keep(String header) {
        if (!keepHeaders.contains(header)) {
            keepHeaders.add(header);
        }
    }

}
