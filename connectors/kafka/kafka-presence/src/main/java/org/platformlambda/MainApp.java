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

package org.platformlambda;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.kafka.InitialLoad;
import org.platformlambda.kafka.KafkaSetup;
import org.platformlambda.kafka.PresenceHandler;
import org.platformlambda.rest.RestServer;
import org.platformlambda.services.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeoutException;
import java.util.concurrent.atomic.AtomicBoolean;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static final String MANAGER = KafkaSetup.MANAGER;
    public static final String PRESENCE_HANDLER = "presence.service";
    public static final String PRESENCE_HOUSEKEEPER = "presence.housekeeper";
    public static final String TOPIC_CONTROLLER = "topic.controller";
    public static final String MONITOR_ALIVE = "monitor_alive";
    private static final String ADDITIONAL_INFO = "additional.info";
    private static final String INIT = InitialLoad.INIT;
    private static final String TYPE = "type";
    private static final String ORIGIN = "origin";

    public static void main(String[] args) {
        RestServer.main(args);
    }

    @Override
    public void start(String[] args) {
        try {
            setup();
        } catch (Exception e) {
            log.error("Unable to start - {}", e.getMessage());
            System.exit(-1);
        }
    }

    private void setup() throws TimeoutException, IOException {
        ServerPersonality.getInstance().setType(ServerPersonality.Type.RESOURCES);
        Utility util = Utility.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        Platform platform = Platform.getInstance();
        PubSub ps = PubSub.getInstance();
        PostOffice po = PostOffice.getInstance();
        platform.connectToCloud();
        platform.waitForProvider(MANAGER, 20);
        platform.waitForProvider(ServiceDiscovery.SERVICE_REGISTRY, 20);
        // start additional info service
        platform.registerPrivate(ADDITIONAL_INFO, new AdditionalInfo(), 3);
        // broadcast heart beat to presence monitor peers
        new MonitorAlive().start();
        platform.registerPrivate(TOPIC_CONTROLLER, new TopicController(), 1);
        // setup presence housekeeper that removes expired Kafka topics
        platform.registerPrivate(PRESENCE_HOUSEKEEPER, new HouseKeeper(), 1);
        // setup presence handler
        platform.registerPrivate(PRESENCE_HANDLER, new PresenceHandler(), 1);
        // start consumer
        String monitorTopic = config.getProperty("monitor.topic", "service.monitor");
        // max.closed.user.groups (3 to 30)
        int maxGroups = Math.min(30,
                Math.max(3, util.str2int(config.getProperty("max.closed.user.groups", "30"))));
        if (!ps.exists(monitorTopic)) {
            // one partition for presence monitor and one for routing table distribution
            ps.createTopic(monitorTopic, maxGroups+1);
        }
        String clientId = platform.getOrigin();
        final AtomicBoolean pending = new AtomicBoolean(true);
        LambdaFunction service = (headers, body, instance) -> {
            // handle initialization request when app starts
            if (INIT.equals(body) && INIT.equals(headers.get(TYPE)) && pending.get()) {
                pending.set(false);
                po.send(PRESENCE_HANDLER, new Kv(TYPE, INIT), new Kv(ORIGIN, platform.getOrigin()));
            }
            return true;
        };
        String groupId = config.getProperty("default.monitor.group.id", "monitorGroup");
        ps.subscribe(monitorTopic, 0, service, clientId, groupId, String.valueOf(InitialLoad.INITIALIZE));
        log.info("Started");
    }

}
