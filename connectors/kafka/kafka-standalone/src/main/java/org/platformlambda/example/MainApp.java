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

package org.platformlambda.example;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.util.Utility;
import org.platformlambda.system.EmbeddedKafka;
import org.platformlambda.system.EmbeddedZk;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static void main(String[] args) {
        AppStarter.main(args);
    }

    @Override
    public void start(String[] args) throws InterruptedException {
        if (zkRunning()) {
            log.error("Application quits because another standalone instance is running");
            System.exit(-1);
        } else {
            // start zookeeper
            EmbeddedZk zk = new EmbeddedZk();
            zk.start();
            int timeout = 10;
            if (!zkReady(10)) {
                log.error("Application quits because standalone zookeeper does not start in {} seconds", timeout);
                zk.shutdown();
                System.exit(-1);
            }
            // start Kafka single node
            EmbeddedKafka kafka = new EmbeddedKafka(zk);
            kafka.start();
        }
    }

    private boolean zkReady(int seconds) throws InterruptedException {
        int seq = seconds;
        while (seq > 0 && !zkRunning()) {
            seq--;
            Thread.sleep(1000);
        }
        return zkRunning();
    }

    private boolean zkRunning() {
        return Utility.getInstance().portReady("127.0.0.1", 2181, 5000);
    }

}
