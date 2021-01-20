/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.services;

import org.platformlambda.MainApp;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.TimeoutException;

public class PingScheduler extends Thread {
    private static final Logger log = LoggerFactory.getLogger(PingScheduler.class);

    private static final String MANAGER = MainApp.MANAGER;
    private static final long INTERVAL = 15 * 60 * 1000;
    private static final String TYPE = "type";
    private static final String LIST = "list";
    private static final String PING = "ping";

    private static long t0 = System.currentTimeMillis();
    private static boolean normal = true;

    @SuppressWarnings("unchecked")
    public void run() {
        log.info("Started");
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

        PostOffice po = PostOffice.getInstance();
        while(normal) {
            if (System.currentTimeMillis() - t0 > INTERVAL) {
                t0 = System.currentTimeMillis();
                // get topic list from kafka
                try {
                    EventEnvelope response = po.request(MANAGER, 30000, new Kv(TYPE, LIST));
                    List<String> topics = response.getBody() instanceof List? (List<String>) response.getBody() : new ArrayList<>();
                    List<String> origins = MonitorService.getOrigins();
                    List<String> targets = new ArrayList<>();
                    for (String p : origins) {
                        if (topics.contains(p)) {
                            try {
                                po.send(ServiceDiscovery.SERVICE_REGISTRY + "@" + p, new Kv(TYPE, PING));
                                targets.add(p);
                            } catch (IOException e) {
                                log.error("Unable to ping {} - {}", p, e.getMessage());
                            }
                        } else {
                            // this should not happen
                            log.error("{} is not a valid topic", p);
                        }
                    }
                    if (!targets.isEmpty()) {
                        log.info("Ping {}", targets);
                    }
                } catch (IOException | TimeoutException | AppException e) {
                    log.error("Ping aborted - {}", e.getMessage());
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

    public static void ping() {
        log.info("Ping requested by operator");
        PingScheduler.t0 = System.currentTimeMillis() - INTERVAL;
    }

    private void shutdown() {
        normal = false;
    }

}