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
package org.platformlambda.kafka;

import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class AppAlive extends Thread {
    private static final Logger log = LoggerFactory.getLogger(AppAlive.class);

    private static final String MONITOR_PARTITION = KafkaSetup.MONITOR_PARTITION;
    private static final String TYPE = "type";
    private static final String ORIGIN = "origin";
    private static final String NAME = "name";
    private static final String TIMESTAMP = "timestamp";
    private static final String APP_ALIVE = "app_alive";
    private static final String PRESENCE_HOUSEKEEPER = "presence.housekeeper";
    private static final long INTERVAL = 30 * 1000;
    private static long t0 = 0;
    private static boolean ready = false;
    private static boolean normal = true;

    public static void setReady(boolean ready) {
        t0 = System.currentTimeMillis() - 10000;
        AppAlive.ready = ready;
    }

    @Override
    public void run() {
        log.info("Started");
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

        Platform platform = Platform.getInstance();
        Utility util = Utility.getInstance();
        String origin = platform.getOrigin();
        PostOffice po = PostOffice.getInstance();
        t0 = System.currentTimeMillis() - 10000;
        while (normal) {
            long now = System.currentTimeMillis();
            if (now - t0 > INTERVAL) {
                t0 = now;
                if (ready) {
                    try {
                        po.send(PRESENCE_HOUSEKEEPER + MONITOR_PARTITION, new Kv(TYPE, APP_ALIVE),
                                new Kv(ORIGIN, origin), new Kv(NAME, Platform.getInstance().getName()),
                                new Kv(TIMESTAMP, util.getTimestamp()));
                    } catch (IOException e) {
                        log.error("Unable to send keep-alive to presence monitor - {}", e.getMessage());
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
