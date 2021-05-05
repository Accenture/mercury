package org.platformlambda.hazelcast.services;

import com.hazelcast.core.LifecycleEvent;
import com.hazelcast.core.LifecycleListener;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class TopicLifecycleListener implements LifecycleListener {
    private static final Logger log = LoggerFactory.getLogger(TopicLifecycleListener.class);

    @Override
    public void stateChanged(LifecycleEvent event) {
        if (event.getState() == LifecycleEvent.LifecycleState.SHUTTING_DOWN) {
            log.error("Stopping application because Hazelcast is no longer available");
            System.exit(10);
        }
        if (event.getState() == LifecycleEvent.LifecycleState.CLIENT_DISCONNECTED) {
            log.error("Hazelcast is offline");
            System.exit(11);
        }
        if (event.getState() == LifecycleEvent.LifecycleState.CLIENT_CONNECTED) {
            log.info("Hazelcast is online");
        }
    }
}
