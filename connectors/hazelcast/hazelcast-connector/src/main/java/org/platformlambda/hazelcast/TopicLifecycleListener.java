/*

    Copyright 2018-2019 Accenture Technology

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

import com.hazelcast.core.HazelcastInstance;
import com.hazelcast.core.ITopic;
import com.hazelcast.core.LifecycleEvent;
import com.hazelcast.core.LifecycleListener;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

public class TopicLifecycleListener implements LifecycleListener {
    private static final Logger log = LoggerFactory.getLogger(TopicLifecycleListener.class);

    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String SETUP_CONSUMER = "start.consumer";
    private static final String JOIN = "join";
    private static final String ORIGIN = "origin";
    private static final String RESTORE = "restore";
    private static boolean ready = false;
    private String realTopic;
    private boolean isServiceMonitor;

    public TopicLifecycleListener(String realTopic) {
        this.realTopic = realTopic;
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
        // create a function to setup consumer asynchronously
        LambdaFunction f = (headers, body, instance) -> {
            setupConsumer(RESTORE.equals(headers.get(TYPE)));
            return true;
        };
        try {
            Platform.getInstance().registerPrivate(SETUP_CONSUMER, f, 1);
            PostOffice.getInstance().send(SETUP_CONSUMER, new Kv(TYPE, JOIN));
        } catch (IOException e) {
            // this should not occur
        }
        log.info("Monitoring {}", realTopic);
    }

    public static boolean isReady() {
        return ready;
    }

    private void setupConsumer(boolean restore) {
        try {
            Platform.getInstance().waitForProvider(ServiceDiscovery.SERVICE_REGISTRY, 10);
        } catch (TimeoutException e) {
            log.error("Unable to setup event consumer - {}", e.getMessage());
        }
        HazelcastInstance client = HazelcastSetup.getHazelcastClient();
        ITopic<byte[]> topic = client.getReliableTopic(realTopic);
        topic.addMessageListener(new EventConsumer());
        if (restore) {
            log.info("Event consumer restored");
        } else {
            log.info("Event consumer started");
        }
        ready = true;
        if (!isServiceMonitor) {
            // tell peers that I have joined
            try {
                PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, JOIN), new Kv(ORIGIN, Platform.getInstance().getOrigin()));
            } catch (IOException e) {
                log.error("Unable to notify peers that I have joined - {}", e.getMessage());
            }
        }
    }

    @Override
    public void stateChanged(LifecycleEvent event) {
        if (event.getState() == LifecycleEvent.LifecycleState.CLIENT_DISCONNECTED) {
            ready = false;
            log.error("Standing by");
        }
        if (event.getState() == LifecycleEvent.LifecycleState.CLIENT_CONNECTED) {
            if (!ready) {
                try {
                    PostOffice.getInstance().send(SETUP_CONSUMER, new Kv(TYPE, RESTORE));
                } catch (IOException e) {
                    log.error("Unable to setup event consumer - {}", e.getMessage());
                }
            }
        }
    }

}
