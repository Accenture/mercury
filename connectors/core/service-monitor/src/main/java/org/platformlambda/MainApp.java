/*

    Copyright 2018-2023 Accenture Technology

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

import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.cloud.PresenceHandler;
import org.platformlambda.cloud.ServiceLifeCycle;
import org.platformlambda.cloud.services.ServiceRegistry;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.services.AdditionalInfo;
import org.platformlambda.services.HouseKeeper;
import org.platformlambda.services.MonitorAlive;
import org.platformlambda.services.TopicController;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static final String MONITOR_PARTITION = "@monitor-0";
    public static final String PRESENCE_HANDLER = "presence.service";
    public static final String PRESENCE_HOUSEKEEPER = "presence.housekeeper";
    public static final String TOPIC_CONTROLLER = "topic.controller";
    public static final String MONITOR_ALIVE = "monitor_alive";
    private static final String ADDITIONAL_INFO = "additional.info";
    private static final String LOOP_BACK = "loopback";
    private static final String REPLY_TO = "reply_to";
    private static final String INIT = ServiceLifeCycle.INIT;
    private static final String DONE = "done";
    private static final String TYPE = "type";
    private static final String ORIGIN = "origin";

    public static void main(String[] args) {
        AppStarter.main(args);
    }

    @Override
    public void start(String[] args) {
        ServerPersonality.getInstance().setType(ServerPersonality.Type.RESOURCES);
        Platform platform = Platform.getInstance();
        platform.connectToCloud();
        List<String> providers = new ArrayList<>();
        providers.add(ServiceRegistry.CLOUD_MANAGER);
        providers.add(ServiceDiscovery.SERVICE_REGISTRY);
        platform.waitForProviders(providers, 20).onSuccess(done -> {
            if (Boolean.TRUE.equals(done)) {
                try {
                    setup();
                } catch (Exception e) {
                    log.error("Unable to start", e);
                    System.exit(-1);
                }
            } else {
                log.error("Cloud connector not ready");
                System.exit(-2);
            }
        });

    }

    private void setup() throws IOException {
        Utility util = Utility.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        PubSub ps = PubSub.getInstance();
        Platform platform = Platform.getInstance();
        EventEmitter po = EventEmitter.getInstance();
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
        // range: 3 - 30
        int maxGroups = Math.min(30,
                Math.max(3, util.str2int(config.getProperty("max.closed.user.groups", "10"))));
        int requiredPartitions = maxGroups + 1;
        if (!ConnectorConfig.topicSubstitutionEnabled()) {
            // automatically create topic if not exist
            if (ps.exists(monitorTopic)) {
                int actualPartitions = ps.partitionCount(monitorTopic);
                if (actualPartitions < requiredPartitions) {
                    log.error("Insufficient partitions in {}, Expected: {}, Actual: {}",
                            monitorTopic, requiredPartitions, actualPartitions);
                    log.error("SYSTEM NOT OPERATIONAL. Please check kafka cluster health.");
                    return;
                }

            } else {
                // one partition for presence monitor and one for routing table distribution
                ps.createTopic(monitorTopic, requiredPartitions);
            }
        }
        String clientId = platform.getOrigin();
        final AtomicBoolean pending = new AtomicBoolean(true);
        LambdaFunction service = (headers, input, instance) -> {
            if (LOOP_BACK.equals(input) && headers.containsKey(REPLY_TO) && clientId.equals(headers.get(ORIGIN))) {
                po.send(headers.get(REPLY_TO), true);
            }
            if (INIT.equals(input) && INIT.equals(headers.get(TYPE))) {
                if (pending.get()) {
                    pending.set(false);
                    po.send(PRESENCE_HANDLER, new Kv(TYPE, INIT), new Kv(ORIGIN, platform.getOrigin()));
                }
                String INIT_HANDLER = INIT + "." + monitorTopic + ".0";
                if (platform.hasRoute(INIT_HANDLER)) {
                    po.send(INIT_HANDLER, DONE);
                }
            }
            return true;
        };
        String groupId = config.getProperty("default.monitor.group.id", "monitorGroup");
        ps.subscribe(monitorTopic, 0, service, clientId, groupId, String.valueOf(ServiceLifeCycle.INITIALIZE));
    }

}
