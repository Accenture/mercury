/*

    Copyright 2018-2022 Accenture Technology

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

package org.platformlambda.activemq;

import org.platformlambda.activemq.services.PubSubManager;
import org.platformlambda.core.annotations.CloudService;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.util.AppConfigReader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.jms.JMSException;
import java.io.IOException;
import java.util.Properties;

/**
 * This cloud service provides pub/sub service.
 * This allows the user application to run as a standalone executable and
 * communicate with each other purely using pub/sub.
 *
 * For real-time and pub/sub use cases, please use the "activemq" connector.
 * It offers pub/sub service and also uses ActiveMQ cluster as a service mesh
 * to support topic virtualization and closed user groups.
 */
@CloudService(name="activemq.pubsub")
public class PubSubSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(PubSubSetup.class);

    private static final String USER = "user";
    private static final String USER_CLOUD_CLIENT_PROPERTIES = "user.clouod.client.properties";
    private static final String CUSTOM_CLOUD_MANAGER_CONFIG = "user.cloud.manager";
    private static final String CUSTOM_CLOUD_MANAGER_ROUTE = "user.cloud.manager";

    @Override
    public void initialize() {
        if (!PubSub.getInstance(USER).featureEnabled()) {
            try {
                AppConfigReader config = AppConfigReader.getInstance();
                String cloudManager = config.getProperty(CUSTOM_CLOUD_MANAGER_CONFIG);
                if (cloudManager == null) {
                    log.warn("Config parameter {} not defined - using default cloud manager: {}",
                            CUSTOM_CLOUD_MANAGER_CONFIG, CUSTOM_CLOUD_MANAGER_ROUTE);
                    cloudManager = CUSTOM_CLOUD_MANAGER_ROUTE;
                }
                Properties properties = ArtemisConnector.getClusterProperties(USER_CLOUD_CLIENT_PROPERTIES);
                PubSub.getInstance(USER).enableFeature(new PubSubManager(USER, properties, cloudManager));
                log.info("Started");
            } catch (JMSException | IOException e) {
                log.error("Unable to connect to ActiveMQ cluster", e);
                System.exit(10);
            }
        }
    }
    
}
