package org.platformlambda.hazelcast;

import com.hazelcast.client.HazelcastClient;
import com.hazelcast.client.config.ClientConfig;
import com.hazelcast.client.config.ClientConnectionStrategyConfig;
import com.hazelcast.client.config.ConnectionRetryConfig;
import com.hazelcast.core.HazelcastInstance;
import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.cloud.EventProducer;
import org.platformlambda.cloud.reporter.PresenceConnector;
import org.platformlambda.cloud.services.CloudHealthCheck;
import org.platformlambda.cloud.services.ServiceQuery;
import org.platformlambda.cloud.services.ServiceRegistry;
import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.PersistentWsClient;
import org.platformlambda.hazelcast.services.PubSubManager;
import org.platformlambda.hazelcast.services.TopicLifecycleListener;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.List;
import java.util.Properties;

@CloudConnector(name="hazelcast")
public class HazelcastConnector implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(HazelcastConnector.class);

    public static final String BROKER_URL = "bootstrap.servers";
    private static final String CLOUD_CHECK = "cloud.connector.health";
    private static final long MAX_CLUSTER_WAIT = 5 * 60 * 1000;
    private static Properties properties;
    private static HazelcastInstance client;

    public static synchronized HazelcastInstance getClient() {
        if (client == null) {
            Utility util = Utility.getInstance();
            String url = properties.getProperty(BROKER_URL);
            List<String> cluster = util.split(url, ", ");
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
            /*
             * When hazelcast is offline, this application instance will stop.
             * In cloud native deployment, the application instance will be restarted
             * automatically by Kubernetes or similar container manager.
             *
             * For more advanced recovery, please update the TopicLifecycleListener class.
             */
            client.getLifecycleService().addLifecycleListener(new TopicLifecycleListener());
            // use the first broker URL as the display URL in the info endpoint
            ConnectorConfig.setServiceName("hazelcast");
            ConnectorConfig.setDisplayUrl(url);
        }
        return client;
    }

    @Override
    public void initialize() {
        Utility util = Utility.getInstance();
        ConfigReader clusterConfig = null;
        try {
            clusterConfig = ConnectorConfig.getConfig("cloud.client.properties",
                    "file:/tmp/config/hazelcast.properties,classpath:/hazelcast.properties");
        } catch (IOException e) {
            log.error("Unable to find hazelcast.properties - {}", e.getMessage());
            System.exit(-1);
        }
        properties = new Properties();
        for (String k : clusterConfig.getMap().keySet()) {
            properties.setProperty(k, clusterConfig.getProperty(k));
        }
        String url = properties.getProperty(BROKER_URL);
        List<String> cluster = util.split(url, ", ");
        boolean reachable = false;
        for (String address : cluster) {
            int colon = address.lastIndexOf(':');
            if (colon > 1) {
                String host = address.substring(0, colon);
                int port = util.str2int(address.substring(colon + 1));
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
        try {
            Platform platform = Platform.getInstance();
            PubSub ps = PubSub.getInstance();
            ps.enableFeature(new PubSubManager());
            AppConfigReader config = AppConfigReader.getInstance();
            if (!"true".equals(config.getProperty("service.monitor", "false"))) {
                // start presence connector
                ConfigReader monitorConfig = ConnectorConfig.getConfig("presence.properties",
                        "file:/tmp/config/presence.properties,classpath:/presence.properties");
                List<String> monitors = Utility.getInstance().split(monitorConfig.getProperty("url"), ", ");
                PersistentWsClient ws = new PersistentWsClient(PresenceConnector.getInstance(), monitors);
                ws.start();
            }
            platform.registerPrivate(PostOffice.CLOUD_CONNECTOR, new EventProducer(), 1);
            // enable service discovery
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, new ServiceRegistry(), 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
            platform.registerPrivate(CLOUD_CHECK, new CloudHealthCheck(), 2);
            platform.startCloudServices();
        } catch (IOException e) {
            log.error("Unable to setup Hazelcast connection - {}", e.getMessage());
            System.exit(-1);
        }
    }

}
