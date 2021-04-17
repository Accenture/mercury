package org.platformlambda.models;

public class PendingConnection {

    public String route, txPath;
    public long created = System.currentTimeMillis();

    public PendingConnection(String route, String txPath) {
        this.route = route;
        this.txPath = txPath;
    }
}
