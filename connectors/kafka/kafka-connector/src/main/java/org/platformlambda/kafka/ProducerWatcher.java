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

package org.platformlambda.kafka;

import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class ProducerWatcher extends Thread {
    private static final Logger log = LoggerFactory.getLogger(ProducerWatcher.class);

    private static final String MANAGER = KafkaSetup.MANAGER;
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String STOP = "stop";
    private static final long ONE_MINUTE = 60 * 1000;
    private static final long IDLE_TIMER = 5 * ONE_MINUTE;   // 5 minutes
    private static final long MAX_INTERVAL = 15 * ONE_MINUTE;   // 15 minutes
    private boolean normal = true;

    @Override
    public void run() {

        log.info("Started");
        while (normal) {
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                // ok to ignore
            }
            long now = System.currentTimeMillis();
            /*
             * close producer after idle for 5 minutes
             */
            if (now - EventProducer.getLastActive() > IDLE_TIMER) {
                try {
                    PostOffice.getInstance().send(CLOUD_CONNECTOR, new Kv(TYPE, STOP));
                } catch (IOException e) {
                    log.error("Unable to close an idle Producer due to {}", e.getMessage());
                }
            }
            // restart producer after 15 minutes of use
            if (now - EventProducer.getLastStarted() > MAX_INTERVAL) {
                try {
                    PostOffice.getInstance().send(CLOUD_CONNECTOR, new Kv(TYPE, STOP));
                } catch (IOException e) {
                    log.error("Unable to reset an active Producer due to {}", e.getMessage());
                }
            }
            /*
             * close admin client after idle for 5 minutes
             */
            if (now - TopicManager.getLastActive() > IDLE_TIMER) {
                try {
                    PostOffice.getInstance().send(MANAGER, new Kv(TYPE, STOP));
                } catch (IOException e) {
                    log.error("Unable to close an idle AdminClient due to {}", e.getMessage());
                }
            }
            // restart producer after 15 minutes of use
            if (now - TopicManager.getLastStarted() > MAX_INTERVAL) {
                try {
                    PostOffice.getInstance().send(MANAGER, new Kv(TYPE, STOP));
                } catch (IOException e) {
                    log.error("Unable to reset an active AdminClient due to {}", e.getMessage());
                }
            }
        }
        log.info("Stopped");
    }

    public void shutdown() {
        normal = false;
    }

}
