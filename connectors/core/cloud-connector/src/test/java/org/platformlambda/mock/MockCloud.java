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

package org.platformlambda.mock;

import org.platformlambda.cloud.EventProducer;
import org.platformlambda.cloud.reporter.PresenceConnector;
import org.platformlambda.cloud.services.CloudHealthCheck;
import org.platformlambda.cloud.services.ServiceQuery;
import org.platformlambda.cloud.services.ServiceRegistry;
import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.PersistentWsClient;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

@CloudConnector(name="mock.cloud")
public class MockCloud implements CloudSetup {
    private static final String CLOUD_CONNECTOR_HEALTH = "cloud.connector.health";

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
            platform.registerPrivate(EventEmitter.CLOUD_CONNECTOR, new EventProducer(), 1);
            platform.registerPrivate(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
            platform.registerPrivate(ServiceDiscovery.SERVICE_REGISTRY, new ServiceRegistry(), 10);
            platform.registerPrivate(CLOUD_CONNECTOR_HEALTH, new CloudHealthCheck(), 2);
            platform.startCloudServices();
        } catch (IOException e) {
            // nothing to worry
        }

    }

}
