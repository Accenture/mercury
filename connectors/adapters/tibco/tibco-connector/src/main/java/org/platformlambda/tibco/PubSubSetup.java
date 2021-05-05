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

package org.platformlambda.tibco;

import org.platformlambda.core.annotations.CloudService;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.tibco.services.TibcoPubSub;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.jms.JMSException;

/**
 * This cloud service provides native pub/sub service only.
 * This allows the user application to run as a standalone executable and
 * communicate with each other purely using native pub/sub.
 *
 * For real-time and pub/sub use cases, please use the "tibco" connector.
 * It offers native pub/sub service and also uses Tibco EMS as a service mesh
 * to support topic virtualization and closed user groups.
 */
@CloudService(name="tibco.pubsub")
public class PubSubSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(PubSubSetup.class);

    @Override
    public void initialize() {
        if (!PubSub.getInstance().featureEnabled()) {
            try {
                PubSub.getInstance().enableFeature(new TibcoPubSub());
                log.info("Tibco cluster = {}",  TibcoConnector.getDisplayUrl());
            } catch (JMSException e) {
                log.error("Unable to connect to Tibco cluster {} - {}", TibcoConnector.getDisplayUrl(), e.getMessage());
                System.exit(10);
            }
        }
    }
    
}
