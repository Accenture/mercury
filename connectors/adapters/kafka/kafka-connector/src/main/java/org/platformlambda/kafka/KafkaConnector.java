/*

    Copyright 2018-2024 Accenture Technology

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
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.PersistentWsClient;
import org.platformlambda.cloud.EventProducer;
import org.platformlambda.kafka.services.PubSubManager;
import org.platformlambda.cloud.reporter.PresenceConnector;
import org.platformlambda.cloud.services.CloudHealthCheck;
import org.platformlambda.cloud.services.ServiceQuery;
import org.platformlambda.cloud.services.ServiceRegistry;
import org.platformlambda.cloud.ConnectorConfig;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Collections;
import java.util.List;
import java.util.Properties;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

@CloudConnector(name="kafka")
public class KafkaConnector implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(KafkaConnector.class);

    private static final String SYSTEM = "system";
    private static final String CLOUD_CLIENT_PROPERTIES = "cloud.client.properties";
    public static final String BROKER_URL = "bootstrap.servers";
    private static final String CLOUD_CONNECTOR_HEALTH = "cloud.connector.health";

    private static final ConcurrentMap<String, Properties> allProperties = new ConcurrentHashMap<>();

    public static synchronized Properties getKafkaProperties(String location) {
        // default location is cloud.client.properties
        Properties properties = allProperties.get(location);
        if (properties == null) {
            properties = new Properties();
            /*
             * Retrieve kafka.properties from the local file system.
             * This assumes the configuration is created by CI/CD or a wrapper for kafka connector.
             */
            ConfigReader config = null;
            try {
                config = ConnectorConfig.getConfig(location,
                        "file:/tmp/config/kafka.properties,classpath:/kafka.properties");
            } catch (IOException e) {
                log.error("Unable to find kafka properties - {}", e.getMessage());
                System.exit(-1);
            }
            for (String k : config.getMap().keySet()) {
                properties.setProperty(k, config.getProperty(k));
            }
            String brokerUrls = properties.getProperty(BROKER_URL);
            List<String> brokers = Utility.getInstance().split(brokerUrls, ",");
            if (brokers.isEmpty()) {
                log.error("Unable to setup kafka - missing {}", BROKER_URL);
                System.exit(-1);
            }
            if (brokers.size() > 1) {
                Collections.sort(brokers);
            }
            /*
             * Ping Kafka cluster when the application starts up.
             * This assumes the broker list is constant over the lifetime of the application.
             */
            try {
                // try 2 times to check if kafka cluster is available
                if (!kafkaReachable(brokers, 2)) {
                    throw new IOException("Unreachable");
                }
            } catch (IOException e) {
                log.error("Kafka cluster failure {} - {}", brokers, e.getMessage());
                System.exit(-1);
            }
            ConnectorConfig.setServiceName("kafka");
            ConnectorConfig.setDisplayUrl(brokers.get(0));
            allProperties.put(location, properties);
        }
        return properties;
    }

    @Override
    public void initialize() {
        try {
            AppConfigReader config = AppConfigReader.getInstance();
            Platform platform = Platform.getInstance();
            PubSub ps = PubSub.getInstance(SYSTEM);
            Properties properties = getKafkaProperties(CLOUD_CLIENT_PROPERTIES);
            ps.enableFeature(new PubSubManager(SYSTEM, properties, ServiceRegistry.CLOUD_MANAGER));
            // is this a regular application?
            if (!"true".equals(config.getProperty("service.monitor", "false"))) {
                // start presence connector
                ConfigReader monitorConfig = ConnectorConfig.getConfig("presence.properties",
                        "file:/tmp/config/presence.properties,classpath:/presence.properties");
                List<String> monitors = Utility.getInstance().split(monitorConfig.getProperty("url"), ", ");
                PersistentWsClient ws = new PersistentWsClient(PresenceConnector.getInstance(), monitors);
                ws.start();
            }
            // setup producer
            platform.registerPrivate(EventEmitter.CLOUD_CONNECTOR, new EventProducer(), 1);
            // enable service discovery
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, new ServiceRegistry(), 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
            platform.registerPrivate(CLOUD_CONNECTOR_HEALTH, new CloudHealthCheck(), 2);
            platform.startCloudServices();

        } catch (Exception e) {
            log.error("Unable to setup kafka client", e);
            System.exit(-1);
        }
    }

    private static boolean kafkaReachable(List<String> brokers, int tries) throws IOException {
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
