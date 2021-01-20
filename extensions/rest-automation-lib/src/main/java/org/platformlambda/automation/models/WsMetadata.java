package org.platformlambda.automation.models;

public class WsMetadata {

    public String application, recipient, txPath;
    public Boolean publish, subscribe;

    public WsMetadata(String application, String recipient, String txPath, boolean publish, boolean subscribe) {
        this.application = application;
        this.recipient = recipient;
        this.txPath = txPath;
        this.publish = publish;
        this.subscribe = subscribe;
    }
}
