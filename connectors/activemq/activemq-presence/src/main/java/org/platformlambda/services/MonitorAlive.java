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
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class MonitorAlive extends Thread {
    private static final Logger log = LoggerFactory.getLogger(MonitorAlive.class);

    private static final String MONITOR_PARTITION = MainApp.MONITOR_PARTITION;
    private static final String MONITOR_ALIVE = MainApp.MONITOR_ALIVE;
    private static final String TYPE = "type";
    private static final String ORIGIN = "origin";
    private static final String INSTANCE = "instance";
    private static final long INTERVAL = 20 * 1000;
    private static boolean ready = false;
    private static long t0 = 0;
    private boolean normal = true;

    public static void setReady() {
        MonitorAlive.ready = true;
        MonitorAlive.t0 = 0;
    }

    @Override
    public void run() {
        log.info("Started");
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        String appId = platform.getAppId();
        String origin = platform.getOrigin();
        while (normal) {
            long now = System.currentTimeMillis();
            if (now - t0 > INTERVAL) {
                t0 = now;
                if (MonitorAlive.ready) {
                    try {
                        // broadcast to all presence monitors
                        List<String> payload = new ArrayList<>(MonitorService.getConnections().keySet());
                        EventEnvelope event = new EventEnvelope().setBody(payload)
                                .setTo(MainApp.PRESENCE_HOUSEKEEPER + MONITOR_PARTITION)
                                .setHeader(TYPE, MONITOR_ALIVE).setHeader(ORIGIN, origin);
                        /*
                         * Optional app instance ID (e.g. Kubernetes' pod-ID)
                         * can be set using Platform.setAppId("some unique instance ID")
                         * before the app starts.
                         */
                        if (appId != null) {
                            event.setHeader(INSTANCE, appId);
                        }
                        po.send(event);
                    } catch (IOException e) {
                        log.error("Unable to send keep-alive - {}", e.getMessage());
                    }
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

    private void shutdown() {
        normal = false;
    }

}
