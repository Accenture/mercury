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

import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.kafka.pubsub.KafkaPubSub;
import org.platformlambda.kafka.util.SetupUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Collections;
import java.util.List;
import java.util.Properties;
import java.util.concurrent.TimeoutException;

@CloudConnector(name="kafka")
public class KafkaSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(KafkaSetup.class);

    public static final String MANAGER = "kafka.manager";
    public static final String PRESENCE_MONITOR = "presence.monitor";
    private static final String CLOUD_CHECK = "cloud.connector.health";
    private static final String ORIGIN = "origin";
    private static final String BROKER_URL = "bootstrap.servers";
    private static final String TYPE = "type";
    private static final String CREATE_TOPIC = "create_topic";

    private static String displayUrl = "unknown";
    private boolean isServiceMonitor;


    public KafkaSetup() {
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
    }

    public static String getDisplayUrl() {
        return displayUrl;
    }

    @Override
    public void initialize() {
        try {
            AppConfigReader reader = AppConfigReader.getInstance();
            // to find kafka.properties, try file first, then classpath
            String pathList = reader.getProperty("kafka.client.properties",
                                          "file:/tmp/config/kafka.properties,classpath:/kafka.properties");
            ConfigReader config = SetupUtil.getConfig(pathList);
            if (config == null) {
                log.error("Unable to find kafka properties from {}", pathList);
                System.exit(-1);
            }
            String brokerUrls = config.getProperty(BROKER_URL);
            List<String> brokers = Utility.getInstance().split(brokerUrls, ",");
            if (brokers.isEmpty()) {
                log.error("Unable to setup kafka from {} - missing {}", pathList, BROKER_URL);
                System.exit(-1);
            }
            if (brokers.size() > 1) {
                Collections.sort(brokers);
            }
            // try 2 times to check if kafka cluster is available
            if (!kafkaReachable(brokers, 2)) {
                log.error("Kafka cluster is not reachable {}", brokers);
                System.exit(-1);
            }
            // use the first broker URL as the display URL in the info endpoint
            displayUrl = brokers.get(0);
            // build kafka base properties
            Properties baseProp = new Properties();
            for (String k: config.getMap().keySet()) {
                baseProp.put(k, config.getProperty(k));
            }
            /*
             * Enable pub/sub feature in case the user application wants to do pub/sub manually
             * (it must be enabled before starting cloud connector
             *  because pub/sub will check the dependency of the connector)
             */
            PubSub.getInstance().enableFeature(new KafkaPubSub(baseProp));
            // setup producer
            Platform platform = Platform.getInstance();
            log.info("Starting kafka producer module");
            platform.registerPrivate(PostOffice.CLOUD_CONNECTOR, new EventProducer(baseProp), 1);
            platform.registerPrivate(MANAGER, new TopicManager(baseProp), 1);
            String origin = platform.getOrigin();
            log.info("Starting kafka consumer module");
            String namespace = Platform.getInstance().getNamespace();
            String presenceMonitor = namespace == null? PRESENCE_MONITOR : PRESENCE_MONITOR + "." + namespace;
            PostOffice po = PostOffice.getInstance();
            String topic = isServiceMonitor? presenceMonitor : origin;
            EventEnvelope init = po.request(MANAGER, 20000, new Kv(TYPE, CREATE_TOPIC), new Kv(ORIGIN, topic));
            if (init.getBody() instanceof Boolean) {
                if (!((Boolean) init.getBody())) {
                    log.error("Unable to start because topic {} cannot be created", topic);
                    System.exit(-1);
                }
            }
            // setup consumer and connect to Kafka
            EventConsumer consumer = new EventConsumer(baseProp, topic);
            consumer.start();
            Runtime.getRuntime().addShutdownHook(new Thread(consumer::shutdown));
            // enable service discovery
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, new ServiceRegistry(), 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
            platform.registerPrivate(CLOUD_CHECK, new KafkaHealthCheck(), 2);
            platform.startCloudServices();

            if (!isServiceMonitor) {
                AppAlive alive = new AppAlive();
                alive.start();
            }

        } catch (IOException | TimeoutException | AppException e) {
            log.error("Unable to setup kafka client", e);
            System.exit(-1);
        }
    }

    private boolean kafkaReachable(List<String> brokers, int tries) throws IOException {
        for (String dest: brokers) {
            if (dest.contains(":")) {
                Utility util = Utility.getInstance();
                int colon = dest.indexOf(':');
                String host = dest.substring(0, colon);
                int port = util.str2int(dest.substring(colon+1));
                if (port == -1) {
                    throw new IOException("Invalid configuration for "+BROKER_URL);
                }
                if (util.portReady(host, port, 8000)) {
                    return true;
                }
            } else {
                throw new IOException("Invalid configuration for "+BROKER_URL);
            }
        }
        int n = tries - 1;
        if (n > 0) {
            log.info("Retrying... Kafka cluster {} is not reachable", brokers);
            return kafkaReachable(brokers, n);
        }
        return false;
    }

}
