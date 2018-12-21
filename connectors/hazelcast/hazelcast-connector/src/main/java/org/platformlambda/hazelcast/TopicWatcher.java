/*

    Copyright 2018 Accenture Technology

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

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.Map;

public class TopicWatcher extends Thread {
    private static final Logger log = LoggerFactory.getLogger(TopicWatcher.class);

    private static final String MANAGER = HazelcastSetup.MANAGER;
    private static final String TOPIC_WATCHER = "topic.watcher";
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String ORIGIN = ServiceDiscovery.ORIGIN;
    private static final String LEAVE = "leave";
    private static final long APP_EXPIRY = 60 * 1000;
    private static final long INTERVAL = 15000;

    private boolean normal = true;

    @Override
    @SuppressWarnings("unchecked")
    public void run() {
        log.info("Started");
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

        PostOffice po = PostOffice.getInstance();
        LambdaFunction scanner = (headers, body, instance) -> {
            try {
                EventEnvelope response = po.request(MANAGER, 10000, new Kv(TYPE, TopicManager.LIST_TIMESTAMP));
                if (response.getBody() instanceof Map) {
                    Map<String, String> peers = (Map<String, String>) response.getBody();
                    List<String> running = new ArrayList<>();
                    List<String> expired = new ArrayList<>();
                    for (String p : peers.keySet()) {
                        if (isExpired(peers.get(p))) {
                            expired.add(p);
                        } else {
                            running.add(p);
                        }
                    }
                    for (String e : expired) {
                        for (String node : running) {
                            po.send(ServiceDiscovery.SERVICE_REGISTRY + "@" + node, new Kv(TYPE, LEAVE), new Kv(ORIGIN, e));
                        }
                        po.send(MANAGER, new Kv(TYPE, LEAVE), new Kv(ORIGIN, e));
                    }
                }
                return true;
            } catch (Exception e) {
                log.error("Unable to scan topics - {}", e.getMessage());
                return false;
            }
        };

        Platform platform = Platform.getInstance();
        try {
            platform.registerPrivate(TOPIC_WATCHER, scanner, 1);
        } catch (IOException e) {
            // this should not happen
            log.error("Unable to register {} - {}", TOPIC_WATCHER, e.getMessage());
        }

        long t1 = System.currentTimeMillis();
        while (normal) {
            long now = System.currentTimeMillis();
            if (now - t1 > INTERVAL) {
                t1 = now;
                try {
                    po.send(TOPIC_WATCHER, new Kv(TYPE, TOPIC_WATCHER));
                } catch (IOException e) {
                    // this should not happen
                    log.error("Unable to scan expired topics - {}", e.getMessage());
                }
            }
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                // yield to the operating system
            }
        }

        log.info("Stopped");
    }

    private boolean isExpired(String isoTimestamp) {
        Date time = Utility.getInstance().str2date(isoTimestamp);
        return System.currentTimeMillis() - time.getTime() > APP_EXPIRY;
    }

    private void shutdown() {
        normal = false;
    }

}
