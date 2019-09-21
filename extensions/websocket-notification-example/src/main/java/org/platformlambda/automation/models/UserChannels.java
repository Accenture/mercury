package org.platformlambda.automation.models;

import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class UserChannels {

    private String userId;
    private ConcurrentMap<String, Boolean> paths = new ConcurrentHashMap<>();
    private long lastUpdate = System.currentTimeMillis();

    public UserChannels(String userId, String txPath) {
        this.userId = userId;
        this.addPath(txPath);
    }

    public String getUserId() {
        return userId;
    }

    public UserChannels touch() {
        lastUpdate = System.currentTimeMillis();
        return this;
    }

    public void addPath(String txPath) {
        paths.put(txPath, true);
    }

    public void removePath(String txPath) {
        paths.remove(txPath);
    }

    public boolean isEmpty() {
        return paths.isEmpty();
    }

    public List<String> getPaths() {
        return new ArrayList<>(paths.keySet());
    }

    @Override
    public String toString() {
        return "Channels("+userId+", "+paths+", "+new Date(lastUpdate)+")";
    }

}
