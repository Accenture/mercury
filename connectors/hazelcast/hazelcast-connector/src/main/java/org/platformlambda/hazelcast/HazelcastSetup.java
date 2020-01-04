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
import java.util.concurrent.TimeoutException;

@CloudConnector(name="hazelcast")
public class HazelcastSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(HazelcastSetup.class);

    public static final String MANAGER = "hazelcast.manager";
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String CLOUD_CHECK = "cloud.connector.health";

    private static HazelcastInstance client;
    private static String namespace;
    private static List<String> cluster;
    private boolean isServiceMonitor;

    public static HazelcastInstance getHazelcastClient() {
        return client;
    }

    public HazelcastSetup() {
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
    }

    public static String getNamespace() {
        return namespace;
    }

    public static List<String> getClusterList() {
        return cluster;
    }

    @Override
    public void initialize() {
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        // Since more than one application may use the same Hazelcast cluster, we use a namespace to prefix all queues and maps
        namespace = reader.getProperty("hazelcast.namespace", "connector") + "-";
        // Hazelcast cluster is a list of domains or IP addresses
        cluster = util.split(reader.getProperty("hazelcast.cluster", "127.0.0.1:5701"), ", ");
        String[] addrs = new String[cluster.size()];
        boolean reachable = false;
        for (int i=0; i < cluster.size(); i++) {
            String address = cluster.get(i);
            addrs[i] = address;
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
        ClientConnectionStrategyConfig connectionStrategy = new ClientConnectionStrategyConfig();
        connectionStrategy.setReconnectMode(ClientConnectionStrategyConfig.ReconnectMode.ASYNC);

        ConnectionRetryConfig retryConfig = new ConnectionRetryConfig();
        retryConfig.setEnabled(true);
        connectionStrategy.setConnectionRetryConfig(retryConfig);

        ClientConfig config = new ClientConfig();
        config.getNetworkConfig().addAddress(addrs);
        config.setConnectionStrategyConfig(connectionStrategy);
        client = HazelcastClient.newHazelcastClient(config);

        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        String origin = platform.getOrigin();
        try {
            platform.registerPrivate(MANAGER, new TopicManager(), 1);
            if (!isServiceMonitor) {
                po.request(MANAGER, 10000, new Kv(TYPE, TopicManager.CREATE_TOPIC));
                String realTopic = HazelcastSetup.getNamespace()+origin;
                client.getLifecycleService().addLifecycleListener(new TopicLifecycleListener(realTopic));
                // start topic keep alive
                TopicKeepAlive alive = new TopicKeepAlive();
                alive.start();
            }
            platform.registerPrivate(PostOffice.CLOUD_CONNECTOR, new EventProducer(HazelcastSetup.getHazelcastClient()), 1);
            // enable service discovery
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, new ServiceRegistry(), 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
            platform.registerPrivate(CLOUD_CHECK, new HazelcastHealthCheck(), 2);
            platform.startCloudServices();

        } catch (IOException | TimeoutException | AppException e) {
            log.error("Unable to setup Hazelcast connection", e);
            System.exit(-1);
        }

    }

}
