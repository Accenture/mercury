package org.platformlambda.tibco;

import com.tibco.tibjms.TibjmsConnectionFactory;
import com.tibco.tibjms.admin.TibjmsAdmin;
import com.tibco.tibjms.admin.TibjmsAdminException;
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
import org.platformlambda.tibco.pubsub.EventProducer;
import org.platformlambda.tibco.pubsub.TibcoPubSub;
import org.platformlambda.tibco.reporter.PresenceConnector;
import org.platformlambda.tibco.services.TibcoHealthCheck;
import org.platformlambda.tibco.services.ServiceQuery;
import org.platformlambda.tibco.services.ServiceRegistry;
import org.platformlambda.tibco.util.ConfigUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.jms.Connection;
import javax.jms.ConnectionFactory;
import javax.jms.JMSException;
import javax.jms.Session;
import java.io.IOException;
import java.util.List;

@CloudConnector(name="tibco")
public class TibcoSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(TibcoSetup.class);

    public static final String APP_GROUP = "@monitor-";
    public static final String MONITOR_PARTITION = APP_GROUP +"0";
    public static final String MANAGER = "tibco.manager";
    private static final String CLOUD_CHECK = "cloud.connector.health";
    private static final String TIBCO_CLUSTER = "tibco.cluster";
    private static final String USER_ID = "tibco.user.id";
    private static final String USER_PWD = "tibco.user.password";
    private static final String ADMIN_ID = "tibco.admin.id";
    private static final String ADMIN_PWD = "tibco.admin.password";
    private static Connection connection;
    private static TibjmsAdmin adminClient;
    private static String displayUrl = "unknown";

    public static String getDisplayUrl() {
        return displayUrl;
    }

    public static Connection getConnection() throws JMSException {
        if (connection == null) {
            AppConfigReader reader = AppConfigReader.getInstance();
            // Tibco EMS cluster is a comma separated list of domains or IP addresses
            String cluster = reader.getProperty(TIBCO_CLUSTER, "tcp://127.0.0.1:7222");
            String userId = reader.getProperty(USER_ID, "");
            String password = reader.getProperty(USER_PWD, "");
            ConnectionFactory factory = new TibjmsConnectionFactory(cluster);
            connection = factory.createConnection(userId, password);
            connection.start();
        }
        return connection;
    }

    public static TibjmsAdmin getAdminClient() throws TibjmsAdminException {
        if (adminClient == null) {
            AppConfigReader reader = AppConfigReader.getInstance();
            // Tibco EMS cluster is a comma separated list of domains or IP addresses
            String cluster = reader.getProperty(TIBCO_CLUSTER, "tcp://127.0.0.1:7222");
            String userId = reader.getProperty(ADMIN_ID, "");
            String password = reader.getProperty(ADMIN_PWD, "");
            adminClient = new TibjmsAdmin(cluster, userId, password);
        }
        return adminClient;
    }

    @Override
    public void initialize() {
        Utility util = Utility.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        List<String> cluster = util.split(config.getProperty(TIBCO_CLUSTER,
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
            log.error("Tibco EMS cluster {} is not reachable", cluster);
            System.exit(-1);
        }
        try {
            Platform platform = Platform.getInstance();
            PubSub ps = PubSub.getInstance();
            ps.enableFeature(new TibcoPubSub());
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
            platform.registerPrivate(CLOUD_CHECK, new TibcoHealthCheck(), 2);
            platform.startCloudServices();
        } catch (IOException | JMSException e) {
            log.error("Unable to setup Tibco connection - {}", e.getMessage());
            System.exit(-1);
        }
    }

}
