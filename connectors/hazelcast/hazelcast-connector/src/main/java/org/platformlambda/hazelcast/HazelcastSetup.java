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

package org.platformlambda.hazelcast;

import com.hazelcast.client.HazelcastClient;
import com.hazelcast.client.config.ClientConfig;
import com.hazelcast.client.config.ClientConnectionStrategyConfig;
import com.hazelcast.client.config.ConnectionRetryConfig;
import com.hazelcast.core.HazelcastInstance;
import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.List;
import java.util.UUID;
import java.util.concurrent.TimeoutException;

@CloudConnector(name="hazelcast")
public class HazelcastSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(HazelcastSetup.class);

    public static final String NAMESPACE = "ms-";
    public static final String MANAGER = "hazelcast.manager";
    public static final String SETUP_CONSUMER = "hazelcast.connection.monitor";
    private static final String MONITOR = "monitor";
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String START = "start";
    private static final String CLOUD_CHECK = "cloud.connector.health";
    private static final long MAX_CLUSTER_WAIT = 5 * 60 * 1000;

    private static HazelcastInstance client;
    private static UUID listenerId;
    private static String realTopic;
    private static List<String> cluster;
    private static boolean isServiceMonitor;

    public static HazelcastInstance getHazelcastClient() {
        return client;
    }

    public HazelcastSetup() {
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
    }

    public static List<String> getClusterList() {
        return cluster;
    }

    @Override
    public void initialize() {
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        // Hazelcast cluster is a list of domains or IP addresses
        cluster = util.split(reader.getProperty("hazelcast.cluster", "127.0.0.1:5701"), ", ");
        boolean reachable = false;
        for (int i=0; i < cluster.size(); i++) {
            String address = cluster.get(i);
            int colon = address.indexOf(':');
            if (colon > 1) {
                String host = address.substring(0, colon);
                int port = util.str2int(address.substring(colon+1));
                if (port > 0) {
                    // ping the address to confirm it is reachable before making a client connection
                    if (util.portReady(host, port, 10000)) {
                        reachable = true;
                    }
                }
            }
        }
        if (!reachable) {
            log.error("Hazelcast cluster {} is not reachable", cluster);
            System.exit(-1);
        }
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        connectToHazelcast();
        try {
            platform.registerPrivate(MANAGER, new TopicManager(), 1);
            if (!isServiceMonitor) {
                // create a new topic and start keep alive
                po.request(MANAGER, 10000, new Kv(TYPE, TopicManager.CREATE_TOPIC));
                TopicKeepAlive alive = new TopicKeepAlive();
                alive.start();
            }
            platform.registerPrivate(PostOffice.CLOUD_CONNECTOR, new EventProducer(), 1);
            // enable service discovery
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, new ServiceRegistry(), 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
            platform.registerPrivate(CLOUD_CHECK, new HazelcastHealthCheck(), 2);
            platform.startCloudServices();
            po.send(SETUP_CONSUMER, new Kv(TYPE, START));

        } catch (IOException | TimeoutException | AppException e) {
            log.error("Unable to setup Hazelcast connection", e);
            System.exit(-1);
        }
    }

    public static void connectToHazelcast() {
        String topic = getRealTopic();
        if (client != null) {
            client.getLifecycleService().removeLifecycleListener(listenerId);
            client.shutdown();
        }
        String[] address = new String[cluster.size()];
        for (int i=0; i < cluster.size(); i++) {
            address[i] = cluster.get(i);
        }
        ClientConnectionStrategyConfig connectionStrategy = new ClientConnectionStrategyConfig();
        connectionStrategy.setReconnectMode(ClientConnectionStrategyConfig.ReconnectMode.ASYNC);
        ConnectionRetryConfig retry = new ConnectionRetryConfig();
        retry.setClusterConnectTimeoutMillis(MAX_CLUSTER_WAIT);
        connectionStrategy.setConnectionRetryConfig(retry);
        ClientConfig config = new ClientConfig();
        config.getNetworkConfig().addAddress(address);
        config.setConnectionStrategyConfig(connectionStrategy);
        client = HazelcastClient.newHazelcastClient(config);
        client.getCluster().addMembershipListener(new ClusterListener());
        listenerId = client.getLifecycleService().addLifecycleListener(new TopicLifecycleListener(topic));
        if (!isServiceMonitor) {
            try {
                // recover the topic
                PostOffice.getInstance().request(MANAGER, 10000, new Kv(TYPE, TopicManager.CREATE_TOPIC));
            } catch (IOException | TimeoutException | AppException e) {
                log.error("Unable to create topic {} - {}", topic, e.getMessage());
            }
        }
        log.info("Connected to hazelcast cluster and listening to {} ", topic);
    }

    public static String getRealTopic() {
        if (realTopic == null) {
            if (isServiceMonitor) {
                realTopic = NAMESPACE + MONITOR;
                String namespace = Platform.getInstance().getNamespace();
                if (namespace != null) {
                    realTopic += "." + namespace;
                }
            } else {
                realTopic = NAMESPACE + Platform.getInstance().getOrigin();
            }
        }
        return realTopic;
    }

}
