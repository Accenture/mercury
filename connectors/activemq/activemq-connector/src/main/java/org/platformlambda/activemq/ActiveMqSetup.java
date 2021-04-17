package org.platformlambda.activemq;

import org.apache.activemq.artemis.jms.client.ActiveMQConnectionFactory;
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
import org.platformlambda.activemq.pubsub.EventProducer;
import org.platformlambda.activemq.pubsub.ActiveMqPubSub;
import org.platformlambda.activemq.reporter.PresenceConnector;
import org.platformlambda.activemq.services.ActiveMqHealthCheck;
import org.platformlambda.activemq.services.ServiceQuery;
import org.platformlambda.activemq.services.ServiceRegistry;
import org.platformlambda.activemq.util.ConfigUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.jms.Connection;
import javax.jms.ConnectionFactory;
import javax.jms.JMSException;
import java.io.IOException;
import java.util.List;

@CloudConnector(name="activemq")
public class ActiveMqSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(ActiveMqSetup.class);

    public static final String APP_GROUP = "@monitor-";
    public static final String MONITOR_PARTITION = APP_GROUP +"0";
    public static final String MANAGER = "activemq.manager";
    private static final String CLOUD_CHECK = "cloud.connector.health";
    private static final String ACTIVEMQ_CLUSTER = "activemq.cluster";
    private static final String USER_ID = "activemq.user.id";
    private static final String USER_PWD = "activemq.user.password";
    private static Connection connection;
    private static String displayUrl = "unknown";

    public static String getDisplayUrl() {
        return displayUrl;
    }

    public static Connection getConnection() throws JMSException {
        if (connection == null) {
            AppConfigReader reader = AppConfigReader.getInstance();
            // ActiveMQ cluster is a comma separated list of domains or IP addresses
            String cluster = reader.getProperty(ACTIVEMQ_CLUSTER, "tcp://127.0.0.1:61616");
            String userId = reader.getProperty(USER_ID, "");
            String password = reader.getProperty(USER_PWD, "");
            ConnectionFactory factory = new ActiveMQConnectionFactory(cluster);
            connection = factory.createConnection(userId, password);
            connection.start();
        }
        return connection;
    }

    @Override
    public void initialize() {
        Utility util = Utility.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        List<String> cluster = util.split(config.getProperty(ACTIVEMQ_CLUSTER,
                "tcp://127.0.0.1:7222"), ", ");
        displayUrl = cluster.toString();
        boolean reachable = false;
        for (String address : cluster) {
            int start = address.lastIndexOf('/');
            int colon = address.lastIndexOf(':');
            if (colon > 1 && colon > start) {
                String host = address.substring(start+1, colon);
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
            log.error("ActiveMQ cluster {} is not reachable", cluster);
            System.exit(-1);
        }
        try {
            Platform platform = Platform.getInstance();
            PubSub ps = PubSub.getInstance();
            ps.enableFeature(new ActiveMqPubSub());
            if (!"true".equals(config.getProperty("service.monitor", "false"))) {
                // start presence connector
                ConfigReader monitorConfig = ConfigUtil.getConfig("presence.properties",
                        "file:/tmp/config/presence.properties,classpath:/presence.properties");
                List<String> monitors = Utility.getInstance().split(monitorConfig.getProperty("url"), ", ");
                PersistentWsClient ws = new PersistentWsClient(PresenceConnector.getInstance(), monitors);
                ws.start();
                // enable keep alive
                AppAlive alive = new AppAlive();
                alive.start();
            }
            platform.registerPrivate(PostOffice.CLOUD_CONNECTOR, new EventProducer(), 1);
            // enable service discovery
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, new ServiceRegistry(), 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
            platform.registerPrivate(CLOUD_CHECK, new ActiveMqHealthCheck(), 2);
            platform.startCloudServices();
        } catch (IOException | JMSException e) {
            log.error("Unable to setup ActiveMQ connection - {}", e.getMessage());
            System.exit(-1);
        }
    }

}
