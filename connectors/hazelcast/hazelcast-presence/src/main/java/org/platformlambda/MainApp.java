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

package org.platformlambda;

import com.hazelcast.core.HazelcastInstance;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.hazelcast.*;
import org.platformlambda.rest.RestServer;
import org.platformlambda.service.InfoService;
import org.platformlambda.service.PingScheduler;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static final String PRESENCE_MONITOR = "presence.monitor";
    public static final String PRESENCE_HANDLER = "presence.service";

    private static final String INFO_SERVICE = "additional.info";
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String MANAGER = HazelcastSetup.MANAGER;

    public static void main(String[] args) {
        RestServer.main(args);
    }

    @Override
    public void start(String[] args) throws TimeoutException, IOException {

        ServerPersonality.getInstance().setType(ServerPersonality.Type.RESOURCES);

        Platform platform = Platform.getInstance();

        platform.connectToCloud();
        platform.waitForProvider(CLOUD_CONNECTOR, 20);
        platform.waitForProvider(MANAGER, 20);
        /*
         * start additional info service
         */
        platform.registerPrivate(INFO_SERVICE, new InfoService(), 1);
        /*
         * ping connected applications periodically
         */
        new PingScheduler().start();
        /*
         * setup presence.monitor topic producer and consumer
         */
        setupHazelcast();
        log.info("Started");
    }

    private void setupHazelcast() throws IOException {
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        HazelcastInstance client = HazelcastSetup.getHazelcastClient();
        String topic = HazelcastSetup.getNamespace()+"monitor";
        client.getLifecycleService().addLifecycleListener(new TopicLifecycleListener(topic));

        // setup producer
        platform.registerPrivate(PRESENCE_MONITOR, new PresenceProducer(client), 1);
        // setup presence handler
        platform.registerPrivate(PRESENCE_HANDLER, new PresenceHandler(), 1);

        // initialize presence handler
        PresenceHandler.initialize();
    }


}
