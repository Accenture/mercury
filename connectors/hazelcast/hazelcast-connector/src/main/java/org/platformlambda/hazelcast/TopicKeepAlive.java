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

import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class TopicKeepAlive extends Thread {
    private static final Logger log = LoggerFactory.getLogger(TopicKeepAlive.class);

    private static final String MANAGER = HazelcastSetup.MANAGER;
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final long TOPIC_ALIVE_INTERVAL = 20 * 1000;

    private boolean normal = true;

    @Override
    public void run() {
        log.info("Started");
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

        PostOffice po = PostOffice.getInstance();

        long t0 = System.currentTimeMillis();
        while(normal) {
            long now = System.currentTimeMillis();
            if (now - t0 > TOPIC_ALIVE_INTERVAL) {
                t0 = now;
                try {
                    po.send(MANAGER, new Kv(TYPE, TopicManager.TOUCH));
                } catch (IOException e) {
                    // this should not happen
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
