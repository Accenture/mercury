package org.platformlambda.core.models;

import org.platformlambda.core.util.Utility;

public class NonBlockingInbox extends InboxBase implements AutoCloseable {

    public NonBlockingInbox() {
        this.id = "r."+ Utility.getInstance().getUuid();
        inboxes.put(id, this);
    }
    @Override
    public void close() throws Exception {
        InboxBase.inboxes.remove(id);
    }
}
