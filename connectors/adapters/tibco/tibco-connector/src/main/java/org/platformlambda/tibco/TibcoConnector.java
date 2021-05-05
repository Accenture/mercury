package org.platformlambda.tibco;

import com.tibco.tibjms.TibjmsConnectionFactory;
import com.tibco.tibjms.admin.TibjmsAdmin;
import com.tibco.tibjms.admin.TibjmsAdminException;
import org.platformlambda.cloud.ConfigUtil;
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
import org.platformlambda.tibco.services.TibcoPubSub;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.jms.Connection;
import javax.jms.ConnectionFactory;
import javax.jms.JMSException;
import java.io.IOException;
import java.util.List;
import java.util.Properties;

@CloudConnector(name="tibco")
public class TibcoConnector implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(TibcoConnector.class);

    public static final String BROKER_URL = "bootstrap.servers";
    private static final String CLOUD_CONNECTOR_HEALTH = "cloud.connector.health";
    private static final String USER_ID = "user.id";
    private static final String USER_PWD = "user.password";
    private static Properties properties;
    private static Connection connection;
    private static TibjmsAdmin adminClient;
    private static String displayUrl = "unknown";

    public static String getDisplayUrl() {
        return displayUrl;
    }

    public static synchronized Connection getConnection() throws JMSException {
        if (connection == null) {
            String cluster = properties.getProperty(BROKER_URL, "tcp://127.0.0.1:7222");
            String userId = properties.getProperty(USER_ID, "");
            String password = properties.getProperty(USER_PWD, "");
            ConnectionFactory factory = new TibjmsConnectionFactory(cluster);
            connection = factory.createConnection(userId, password);
            connection.setExceptionListener((e) -> {
                String error = e.getMessage();
                log.error("Tibco cluster exception - {}", error);
                if (error != null && (error.contains("terminated") || error.contains("disconnect"))) {
                    TibcoConnector.stopConnection();
                    System.exit(10);
                }
            });
            connection.start();
            log.info("Connection started - {}", cluster);
        }
        return connection;
    }

    public static synchronized void stopConnection() {
        if (connection != null) {
            try {
                connection.stop();
                connection = null;
                log.info("Connection stopped");
            } catch (JMSException e) {
                // ok to ignore
            }
        }
    }

    public static synchronized TibjmsAdmin getAdminClient() throws TibjmsAdminException {
        if (adminClient == null) {
            String cluster = properties.getProperty(BROKER_URL, "tcp://127.0.0.1:7222");
            String userId = properties.getProperty(USER_ID, "");
            String password = properties.getProperty(USER_PWD, "");
            adminClient = new TibjmsAdmin(cluster, userId, password);
        }
        return adminClient;
    }

    @Override
    public void initialize() {
        Utility util = Utility.getInstance();
        ConfigReader clusterConfig = null;
        try {
            clusterConfig = ConfigUtil.getConfig("cloud.client.properties",
                    "file:/tmp/config/tibco.properties,classpath:/tibco.properties");
        } catch (IOException e) {
            log.error("Unable to find tibco.properties - {}", e.getMessage());
            System.exit(-1);
        }
        properties = new Properties();
        for (String k : clusterConfig.getMap().keySet()) {
            properties.setProperty(k, clusterConfig.getProperty(k));
        }
        displayUrl = properties.getProperty(BROKER_URL);
        List<String> cluster = util.split(displayUrl, ", ");
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
            log.error("Tibco EMS cluster {} is not reachable", cluster);
            System.exit(-1);
        }
        try {
            Platform platform = Platform.getInstance();
            PubSub ps = PubSub.getInstance();
            ps.enableFeature(new TibcoPubSub());
            AppConfigReader config = AppConfigReader.getInstance();
            if (!"true".equals(config.getProperty("service.monitor", "false"))) {
                // start presence connector
                ConfigReader monitorConfig = ConfigUtil.getConfig("presence.properties",
                        "file:/tmp/config/presence.properties,classpath:/presence.properties");
                List<String> monitors = Utility.getInstance().split(monitorConfig.getProperty("url"), ", ");
                PersistentWsClient ws = new PersistentWsClient(PresenceConnector.getInstance(), monitors);
                ws.start();
            }
            platform.registerPrivate(PostOffice.CLOUD_CONNECTOR, new EventProducer(), 1);
            // enable service discovery
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, new ServiceRegistry(), 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
            platform.registerPrivate(CLOUD_CONNECTOR_HEALTH, new CloudHealthCheck(), 2);
            platform.startCloudServices();
        } catch (IOException | JMSException e) {
            log.error("Unable to setup Tibco connection - {}", e.getMessage());
            System.exit(-1);
        }
    }

}
