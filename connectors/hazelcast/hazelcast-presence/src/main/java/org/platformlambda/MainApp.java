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
import org.platformlambda.hazelcast.HazelcastSetup;
import org.platformlambda.hazelcast.PresenceHandler;
import org.platformlambda.hazelcast.PresenceProducer;
import org.platformlambda.hazelcast.TopicLifecycleListener;
import org.platformlambda.rest.RestServer;
import org.platformlambda.service.HouseKeeper;
import org.platformlambda.service.AdditionalInfo;
import org.platformlambda.service.KeepAlive;
import org.platformlambda.service.PingScheduler;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static final String MANAGER = HazelcastSetup.MANAGER;
    public static final String PRESENCE_HOUSEKEEPER = "presence.housekeeper";
    public static final String PRESENCE_MONITOR = "presence.monitor";
    public static final String PRESENCE_HANDLER = "presence.service";

    private static final String MONITOR = "monitor";
    private static final String ADDITIONAL_INFO = "additional.info";
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;

    public static void main(String[] args) {
        RestServer.main(args);
    }

    @Override
    public void start(String[] args) throws TimeoutException, IOException, InterruptedException {
        ServerPersonality.getInstance().setType(ServerPersonality.Type.RESOURCES);

        Platform platform = Platform.getInstance();
        platform.connectToCloud();
        platform.waitForProvider(CLOUD_CONNECTOR, 20);
        platform.waitForProvider(MANAGER, 20);
        // start additional info service
        platform.registerPrivate(ADDITIONAL_INFO, new AdditionalInfo(), 1);
        /*
         * setup presence.monitor topic producer and consumer
         */
        HazelcastInstance client = HazelcastSetup.getHazelcastClient();
        String topic = HazelcastSetup.getNamespace()+MONITOR;
        client.getLifecycleService().addLifecycleListener(new TopicLifecycleListener(topic));
        // setup producer
        platform.registerPrivate(PRESENCE_MONITOR, new PresenceProducer(client, topic), 1);
        // setup presence handler
        platform.registerPrivate(PRESENCE_HANDLER, new PresenceHandler(), 1);
        // ping connected application instances periodically
        new PingScheduler().start();
        // broadcast heart beat to presence monitor peers
        new KeepAlive().start();
        // setup presence housekeeper that removes expired hazelcast topics
        platform.registerPrivate(PRESENCE_HOUSEKEEPER, new HouseKeeper(), 1);
        log.info("Started");
    }

}
