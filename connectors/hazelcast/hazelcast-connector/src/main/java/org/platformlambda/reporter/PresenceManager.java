/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.reporter;

import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.WsClientEndpoint;
import org.platformlambda.core.websocket.common.WsConfigurator;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.ClientEndpointConfig;
import javax.websocket.ContainerProvider;
import javax.websocket.DeploymentException;
import javax.websocket.WebSocketContainer;
import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.TimeoutException;

public class PresenceManager extends Thread {
    private static final Logger log = LoggerFactory.getLogger(PresenceManager.class);

    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String SERVICE_REGISTRY = ServiceDiscovery.SERVICE_REGISTRY;
    private static boolean normal = true;
    private long timer = 0;
    private static final long WAIT_INTERVAL = 5000;
    private static List<String> platformPaths = new ArrayList<>();
    private String url;
    
    public PresenceManager(String url) {
        this.url = url;
    }

    @Override
    public void run() {
        // make sure all dependencies are ready
        Platform platform = Platform.getInstance();
        try {
            platform.waitForProvider(CLOUD_CONNECTOR, 30);
            if (!platform.hasRoute(CLOUD_CONNECTOR)) {
                throw new IOException(CLOUD_CONNECTOR + " not available");
            }
            platform.waitForProvider(SERVICE_REGISTRY, 30);
            if (!platform.hasRoute(SERVICE_REGISTRY)) {
                throw new IOException(SERVICE_REGISTRY + " not available");
            }
        } catch (TimeoutException | IOException e) {
            log.error("Unable to start", e);
            System.exit(-1);
        }
        // start reporting to presence monitor
        long lastActivity = 0;
        PresenceConnector connector = PresenceConnector.getInstance();
        if (connector.isUnassigned()) {
            lastActivity = System.currentTimeMillis();
            long idleTimeout = WsConfigurator.getInstance().getIdleTimeout() * 1000;
            if (url.contains(",")) {
                platformPaths = Utility.getInstance().split(url, ", ");
            } else {
                platformPaths.add(url);
            }
            /*
             * Immediate connect when running for the first time.
             * Thereafter, wait for 5 seconds and try again until it is connected.
             */
            long interval = idleTimeout / 3;
            log.info("Reporting to {} with keep-alive interval {} seconds ", platformPaths, interval / 1000);
            while (normal) {
                long now = System.currentTimeMillis();
                if (connector.isConnected()) {
                    connector.checkActivation();
                    timer = now;
                    // keep-alive
                    if (now - lastActivity > interval) {
                        connector.keepAlive();
                        lastActivity = System.currentTimeMillis();
                    }

                } else {
                    if (now - timer >= WAIT_INTERVAL) {
                        timer = System.currentTimeMillis();
                        try {
                            connect();
                        } catch (Exception e) {
                            // this should never happen
                            log.error("Unexpected error when trying to reconnect - {}", e.getMessage());
                        }
                    }
                }
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e1) {
                    // ok to ignore
                }
            }
            log.info("Stopped");
        } else {
            log.warn("Presence connector already started");
        }

    }

    private void connect() {
        Platform platform = Platform.getInstance();
        PresenceConnector connector = PresenceConnector.getInstance();
        connector.setState(PresenceConnector.State.CONNECTING);
        for (String path : platformPaths) {
            try {
                URI uri = new URI((path.endsWith("/") ? path : path + "/") + platform.getOrigin());
                try {
                    WsClientEndpoint endpoint = new WsClientEndpoint(PresenceConnector.getInstance(), uri);
                    WebSocketContainer container = ContainerProvider.getWebSocketContainer();
                    ClientEndpointConfig config = ClientEndpointConfig.Builder.create().build();
                    container.connectToServer(endpoint, config, uri);
                    log.info("Reporting to {}", uri);
                    break; // exit after successful connection
                } catch (DeploymentException | IOException e) {
                    log.warn("{} {}", simpleError(e.getMessage()), uri);
                    try {
                        Thread.sleep(1000);
                    } catch (InterruptedException e1) {
                        // ok to ignore
                    }
                }
            } catch (URISyntaxException e) {
                log.error("Invalid service monitor URL "+path, e);
                System.exit(-1);
            }
        }

    }

    private String simpleError(String error) {
        if (error == null) {
            return "Unreachable";
        }
        String message = error.contains(":") ? error.substring(error.lastIndexOf(':')+1).trim() : error;
        return message.equals("no further information") || message.contains("null")
                || message.contains("connection fail")? "Unreachable" : message;
    }

    public static void shutdown() {
        normal = false;
    }


}
