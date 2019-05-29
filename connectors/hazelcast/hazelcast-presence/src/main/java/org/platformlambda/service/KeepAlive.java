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

package org.platformlambda.service;

import org.platformlambda.MainApp;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.hazelcast.PresenceHandler;
import org.platformlambda.hazelcast.TopicLifecycleListener;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.security.SecureRandom;
import java.util.ArrayList;
import java.util.Map;

public class KeepAlive extends Thread {
    private static final Logger log = LoggerFactory.getLogger(KeepAlive.class);

    private static final SecureRandom random = new SecureRandom();
    private static final long INTERVAL = 20 * 1000;
    private static final String INIT = "init";
    private static final String TYPE = "type";
    private static final String ORIGIN = "origin";
    private static final String ALIVE = "alive";
    private static final String TOKEN = "token";
    private static final String TIMESTAMP = "timestamp";
    private static final long TOKEN_LIFE = 5 * 60 * 1000;
    private static final int TOKEN_BOUND = 10000;

    private int token = random.nextInt(TOKEN_BOUND);
    private long updated = System.currentTimeMillis();
    private static boolean normal = true;

    @Override
    public void run() {
        // check hazelcast readiness and initialize PresenceHandler
        initializeHazelcast();
        // begin keep-alive
        log.info("Started");
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

        Utility util = Utility.getInstance();
        String origin = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        // first cycle starts in 10 seconds
        long t0 = System.currentTimeMillis() - INTERVAL + 10000;
        while(normal) {
            long now = System.currentTimeMillis();
            if (now - t0 > INTERVAL) {
                t0 = now;
                /*
                 * broadcast to all presence monitors
                 */
                if (now - updated > TOKEN_LIFE) {
                    generateToken();
                }
                // send keep-alive
                EventEnvelope event = new EventEnvelope();
                event.setTo(MainApp.PRESENCE_HOUSEKEEPER);
                event.setHeader(ORIGIN, origin);
                event.setHeader(TYPE, ALIVE);
                // token is used for leader election
                event.setHeader(TOKEN, token);
                // use sortable timestamp yyyymmddhhmmss
                event.setHeader(TIMESTAMP, util.getTimestamp());
                // send my connection list
                event.setBody(new ArrayList<>(MonitorService.getConnections().keySet()));
                try {
                    po.send(MainApp.PRESENCE_MONITOR, event.toBytes());
                } catch (IOException e) {
                    log.error("Unable to send keep-alive to other presence monitors");
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

    private void generateToken() {
        token = random.nextInt(TOKEN_BOUND);
        updated = System.currentTimeMillis();
    }

    private void initializeHazelcast() {
        int n = 0;
        while (n < 20) {
            n++;
            if (TopicLifecycleListener.isReady()) {
                try {
                    EventEnvelope event = new EventEnvelope();
                    event.setTo(MainApp.PRESENCE_HANDLER);
                    event.setHeader(INIT, PresenceHandler.INIT_TOKEN);
                    PostOffice.getInstance().send(MainApp.PRESENCE_MONITOR, event.toBytes());
                    break;
                } catch (IOException e) {
                    log.error("Unable to initialize kafka connection - {}", e.getMessage());
                    System.exit(-1);
                }
            }
            log.info("Waiting for hazelcast connection to get ready... {}", n);
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                // yield to the operating system
            }
        }
    }

    private void shutdown() {
        normal = false;
    }

}