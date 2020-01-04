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

package org.platformlambda.services;

import org.platformlambda.MainApp;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.kafka.ConsumerLifeCycle;
import org.platformlambda.kafka.PresenceHandler;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;

public class KeepAlive extends Thread {
    private static final Logger log = LoggerFactory.getLogger(KeepAlive.class);

    public static final String MONITOR_ALIVE = "monitor_alive";
    private static final String TO = "to";
    private static final long INTERVAL = 20 * 1000;
    private static final String INIT = "init";
    private static final String TYPE = "type";
    private static final String ORIGIN = "origin";
    private static final String TIMESTAMP = "timestamp";

    private static boolean normal = true;

    @Override
    public void run() {
        // check Kafka readiness and initialize PresenceHandler
        initializeKafka();
        // begin keep-alive
        log.info("Started");
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

        Utility util = Utility.getInstance();
        String origin = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        // first cycle starts in 5 seconds
        long t0 = System.currentTimeMillis() - INTERVAL + 5000;
        while (normal) {
            long now = System.currentTimeMillis();
            if (now - t0 > INTERVAL) {
                t0 = now;
                /*
                 * broadcast to all presence monitors
                 */
                EventEnvelope event = new EventEnvelope();
                event.setTo(MainApp.PRESENCE_HOUSEKEEPER);
                event.setHeader(ORIGIN, origin);
                event.setHeader(TYPE, MONITOR_ALIVE);
                // token is used for leader election
                // use sortable timestamp yyyymmddhhmmss
                event.setHeader(TIMESTAMP, util.getTimestamp());
                // send my connection list
                event.setBody(new ArrayList<>(MonitorService.getConnections().keySet()));
                try {
                    po.send(PostOffice.CLOUD_CONNECTOR, event.toBytes(), new Kv(TO, "*"));
                } catch (IOException e) {
                    log.error("Unable to send keep-alive to other presence monitors - {}", e.getMessage());
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

    private void initializeKafka() {
        int n = 0;
        while (n < 20) {
            n++;
            if (ConsumerLifeCycle.isReady()) {
                EventEnvelope event = new EventEnvelope();
                event.setTo(MainApp.PRESENCE_HANDLER);
                event.setHeader(INIT, PresenceHandler.INIT_TOKEN);
                // "*" tell the event producer to send the INIT_TOKEN to the presence monitor
                try {
                    PostOffice.getInstance().send(PostOffice.CLOUD_CONNECTOR, event.toBytes(), new Kv(TO, "*"));
                    break;
                } catch (IOException e) {
                    log.error("Unable to initialize kafka connection - {}", e.getMessage());
                    System.exit(-1);
                }
            }
            log.info("Waiting for kafka to get ready... {}", n);
            try {
                Thread.sleep(2000);
            } catch (InterruptedException e) {
                // yield to the operating system
            }
        }
    }

    private void shutdown() {
        normal = false;
    }

}
