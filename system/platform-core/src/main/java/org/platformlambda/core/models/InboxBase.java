package org.platformlambda.core.models;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public abstract class InboxBase {

    protected String id;

    protected static final ConcurrentMap<String, InboxBase> inboxes = new ConcurrentHashMap<>();

    public static InboxBase getHolder(String inboxId) {
        return inboxes.get(inboxId);
    }

    public String getId() {
        return id;
    }

}
