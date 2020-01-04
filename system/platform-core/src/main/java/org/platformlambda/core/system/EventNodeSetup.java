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

package org.platformlambda.core.system;

import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.models.CloudSetup;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

@CloudConnector(name="event.node")
public class EventNodeSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(EventNodeSetup.class);

    @Override
    public void initialize() {
        try {
            log.info("Connecting to event node");
            // connect to event node
            ServerPersonality personality = ServerPersonality.getInstance();
            // set personality to APP automatically
            if (personality.getType() == ServerPersonality.Type.UNDEFINED) {
                personality.setType(ServerPersonality.Type.APP);
            } else if (personality.getType() == ServerPersonality.Type.PLATFORM) {
                throw new IOException("Unable to connect because you are already an event node");
            }
            new EventNodeManager().start();
            /*
             * When event node is used, it will resolve websocket txPaths dynamically.
             * For other cloud connectors, they will simply return "cloud.connector".
             */
            Platform.getInstance().registerPrivate(PostOffice.CLOUD_CONNECTOR,
                    (headers, body, instance) -> PostOffice.EVENT_NODE, 1);

        } catch (IOException e) {
            log.error("Cloud setup exception - {}", e.getMessage());
        }
    }

}
