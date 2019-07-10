package org.platformlambda.automation.models;

import javax.servlet.AsyncContext;

public class AsyncContextHolder {

    public AsyncContext context;
    public long timeout;
    public long lastAccess;
    public String url, corsId, accept;

    public AsyncContextHolder(AsyncContext context, long timeout) {
        this.context = context;
        this.timeout = timeout;
        this.touch();
    }

    public AsyncContextHolder setUrl(String url) {
        this.url = url;
        return this;
    }

    public AsyncContextHolder setCorsId(String corsId) {
        this.corsId = corsId;
        return this;
    }

    public AsyncContextHolder setAccept(String accept) {
        this.accept = accept;
        return this;
    }

    public void touch() {
        this.lastAccess = System.currentTimeMillis();
    }

}
