package org.platformlambda.mock;

import org.platformlambda.cloud.EventProducer;
import org.platformlambda.cloud.reporter.PresenceConnector;
import org.platformlambda.cloud.services.CloudHealthCheck;
import org.platformlambda.cloud.services.ServiceQuery;
import org.platformlambda.cloud.services.ServiceRegistry;
import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.PersistentWsClient;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@CloudConnector(name="mock.cloud")
public class MockCloud implements CloudSetup {
    private static final String CLOUD_CONNECTOR_HEALTH = "cloud.connector.health";
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String INFO = "info";
    private static final String HEALTH = "health";

    @Override
    public void initialize() {
        Utility util = Utility.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        Platform platform = Platform.getInstance();
        PubSub ps = PubSub.getInstance();
        ps.enableFeature(new MockPubSub());
        int port = util.str2int(config.getProperty("server.port", "8085"));
        String url1 = "ws://127.0.0.1:"+port+"/ws/presence";
        String url2 = "ws://localhost:"+port+"/ws/presence";
        List<String> monitors = new ArrayList<>();
        monitors.add(url1);
        monitors.add(url2);
        PersistentWsClient ws = new PersistentWsClient(PresenceConnector.getInstance(), monitors);
        ws.start();
        try {
            platform.registerPrivate(PostOffice.CLOUD_CONNECTOR, new EventProducer(), 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, new ServiceRegistry(), 10);
            platform.registerPrivate(CLOUD_CONNECTOR_HEALTH, new CloudHealthCheck(), 2);
            platform.startCloudServices();
        } catch (IOException e) {
            // nothing to worry
        }

    }

}
