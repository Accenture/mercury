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

package org.platformlambda.kafka;

import org.platformlambda.core.annotations.CloudService;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.kafka.services.PubSubManager;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Properties;

/**
 * This cloud service provides streaming pub/sub service only.
 * This allows the user application to run as a standalone executable and
 * communicate with each other purely using streaming pub/sub.
 *
 * For real-time and pub/sub use cases, please use the "kafka" connector.
 * It offers native pub/sub service and also uses Kafka as a service mesh
 * to support topic virtualization and closed user groups.
 *
 * This user pub/sub system can be used alone or to be deployed
 * with the system pub/sub where the latter is used as a service mesh.
 */
@CloudService(name="kafka.pubsub")
public class PubSubSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(PubSubSetup.class);

    private static final String USER = "user";
    private static final String USER_CLOUD_CLIENT_PROPERTIES = "user.cloud.client.properties";
    private static final String CUSTOM_CLOUD_MANAGER_CONFIG = "user.cloud.manager";
    private static final String CUSTOM_CLOUD_MANAGER_ROUTE = "user.cloud.manager";

    @Override
    public void initialize() {
        if (!PubSub.getInstance(USER).featureEnabled()) {
            AppConfigReader config = AppConfigReader.getInstance();
            String cloudManager = config.getProperty(CUSTOM_CLOUD_MANAGER_CONFIG);
            if (cloudManager == null) {
                log.warn("Config parameter {} not defined - using default cloud manager: {}",
                        CUSTOM_CLOUD_MANAGER_CONFIG, CUSTOM_CLOUD_MANAGER_ROUTE);
                cloudManager = CUSTOM_CLOUD_MANAGER_ROUTE;
            }
            Properties properties = KafkaConnector.getKafkaProperties(USER_CLOUD_CLIENT_PROPERTIES);
            String brokerUrls = properties.getProperty(KafkaConnector.BROKER_URL);
            List<String> brokers = Utility.getInstance().split(brokerUrls, ",");
            log.info("{} = {}", KafkaConnector.BROKER_URL, brokers);
            PubSub.getInstance(USER).enableFeature(new PubSubManager(USER, properties, cloudManager));
        }
    }
    
}
