/*

    Copyright 2018-2020 Accenture Technology

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

 */

package org.platformlambda.hazelcast;

import com.hazelcast.core.*;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.hazelcast.reporter.PresenceConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

public class TopicLifecycleListener implements LifecycleListener {
    private static final Logger log = LoggerFactory.getLogger(TopicLifecycleListener.class);

    public static final String SETUP_CONSUMER = "hazelcast.connection.monitor";
    public static final String CLUSTER_CHANGED = "cluster_changed";
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String SERVICE_REGISTRY = ServiceDiscovery.SERVICE_REGISTRY;
    private static final String PRESENCE_HANDLER = "presence.service";
    private static final String START = "start";
    private static final String RESET = "reset";
    private static final String JOIN = "join";
    private static final String LEAVE = "leave";
    private static final String ORIGIN = "origin";
    private static final String RESTORE = "restore";

    private static boolean ready = false;
    private ITopic<byte[]> topic = null;
    private String registrationId = null;
    private String realTopic;
    private String setupType = null;
    private boolean isServiceMonitor;

    public TopicLifecycleListener(String realTopic) {
        this.realTopic = realTopic;
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
        // create a function to setup consumer asynchronously
        LambdaFunction f = (headers, body, instance) -> {
            setupConsumer(headers.get(TYPE));
            return true;
        };
        try {
            Platform.getInstance().registerPrivate(SETUP_CONSUMER, f, 1);
            PostOffice.getInstance().send(SETUP_CONSUMER, new Kv(TYPE, START));
        } catch (IOException e) {
            // this should not occur
        }
    }

    public static boolean isReady() {
        return ready;
    }

    private void setupConsumer(String type) {
        ready = false;
        log.info("Hazelcast {}", type.toUpperCase());
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        if (!po.exists(SERVICE_REGISTRY)) {
            try {
                platform.waitForProvider(SERVICE_REGISTRY, 10);
            } catch (TimeoutException e) {
                log.error("Unable to setup event consumer - {}", e.getMessage());
            }
        }
        HazelcastInstance client = HazelcastSetup.getHazelcastClient();
        ClusterListener.setMembers(client.getCluster().getMembers());
        if (topic != null && registrationId != null) {
            topic.removeMessageListener(registrationId);
            log.info("Event consumer {} for {} stopped", registrationId, realTopic);
        }
        if (!START.equals(type) && isServiceMonitor) {
            try {
                po.send(PRESENCE_HANDLER, new Kv(TYPE, RESET), new Kv(ORIGIN, platform.getOrigin()));
            } catch (IOException e) {
                log.error("Unable to reset application connections - {}", e.getMessage());
            }
        }
        topic = client.getReliableTopic(realTopic);
        registrationId = topic.addMessageListener(new EventConsumer());
        log.info("Event consumer {} for {} started", registrationId, realTopic);
        ready = true;
        // reset connection with presence monitor to force syncing routing table
        if (!isServiceMonitor) {
            PresenceConnector connector = PresenceConnector.getInstance();
            if (connector.isConnected() && connector.isReady()) {
                connector.resetMonitor();
            }
        }
    }

    @Override
    public void stateChanged(LifecycleEvent event) {
        PostOffice po = PostOffice.getInstance();
        if (event.getState() == LifecycleEvent.LifecycleState.CLIENT_DISCONNECTED) {
            String origin = Platform.getInstance().getOrigin();
            ready = false;
            log.error("Hazelcast is offline");
            if (isServiceMonitor) {
                try {
                    po.send(PRESENCE_HANDLER, new Kv(TYPE, RESET), new Kv(ORIGIN, origin));
                } catch (IOException e) {
                    log.error("Unable to reset application connections - {}", e.getMessage());
                }
            } else {
                try {
                    po.send(SERVICE_REGISTRY, new Kv(TYPE, LEAVE), new Kv(ORIGIN, origin));
                } catch (IOException e) {
                    log.error("Unable to reset routing table - {}", e.getMessage());
                }
            }
        }
        if (event.getState() == LifecycleEvent.LifecycleState.CLIENT_CONNECTED) {
            if (!ready) {
                try {
                    po.send(SETUP_CONSUMER, new Kv(TYPE, RESTORE));
                } catch (IOException e) {
                    log.error("Unable to setup event consumer - {}", e.getMessage());
                }
            }
        }
    }

}
