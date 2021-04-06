package org.platformlambda.models;

public class TopicRequest {

    public String origin, txPath;
    public long created = System.currentTimeMillis();

    public TopicRequest(String origin, String txPath) {
        this.origin = origin;
        this.txPath = txPath;
    }

}
