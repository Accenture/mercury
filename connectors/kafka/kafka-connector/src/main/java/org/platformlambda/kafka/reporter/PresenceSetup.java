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

package org.platformlambda.kafka.reporter;

import org.platformlambda.core.annotations.CloudService;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.kafka.util.SetupUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@CloudService(name="kafka.reporter")
public class PresenceSetup implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(PresenceSetup.class);

    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String EVENT_NODE = PostOffice.EVENT_NODE;
    private static final String PRESENCE_MONITOR = "presence.monitor";

    @Override
    public void initialize() {
        AppConfigReader reader = AppConfigReader.getInstance();
        boolean serviceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
        if (serviceMonitor) {
            log.error("Presence reporter is not required for presence monitor");
        } else {
            boolean eventNode = EVENT_NODE.equals(reader.getProperty(CLOUD_CONNECTOR, EVENT_NODE));
            if (eventNode) {
                log.error("Presence reporter is not supported when Event Node is used");
            } else {
                try {
                    String url = reader.getProperty(PRESENCE_MONITOR, "ws://127.0.0.1:8080/ws/presence");
                    // to find kafka.properties, try file first, then classpath
                    String pathList = reader.getProperty("presence.properties","file:/tmp/config/presence.properties");
                    ConfigReader config = SetupUtil.getConfig(pathList);
                    if (config != null) {
                        String urlFromProperties = config.getProperty("url");
                        if (urlFromProperties != null) {
                            url = urlFromProperties;
                            log.info("Setting presence URL to {}", urlFromProperties);
                        } else {
                            log.error("Missing url parameter in presence.properties. Fall back to using application.properties");
                        }
                    }
                    PresenceManager connection = new PresenceManager(url);
                    connection.start();

                } catch (Exception e) {
                    PresenceManager.shutdown();
                    log.error("Unable to start", e);
                    System.exit(-1);
                }
            }
        }
    }

}

