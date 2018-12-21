/*

    Copyright 2018 Accenture Technology

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

import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.WsClientEndpoint;
import org.platformlambda.core.websocket.common.WsConfigurator;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.*;
import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.util.ArrayList;
import java.util.List;

public class EventNodeManager extends Thread {
    private static final Logger log = LoggerFactory.getLogger(EventNodeManager.class);

    private static final String PLATFORM_PATH = "event.node.path";
    private static final String LOCAL_PLATFORM = "ws://127.0.0.1:8080/ws/events/";
    private static final long WAIT_INTERVAL = 5000;
    private static final long MAX_HANDSHAKE_WAIT = 8000;
    private static final int IDLE_THRESHOLD = 10;
    private static List<String> platformPaths = new ArrayList<>();

    private static long lastActivity;
    private static boolean normal = true;
    private long timer = 0;

    @Override
    public void run() {
        EventNodeConnector connector = EventNodeConnector.getInstance();
        if (connector.isUnassigned()) {
            lastActivity = System.currentTimeMillis();
            long idleSeconds = WsConfigurator.getInstance().getIdleTimeout() - IDLE_THRESHOLD;
            long idleTimeout = (idleSeconds < IDLE_THRESHOLD? IDLE_THRESHOLD : idleSeconds) * 1000;
            if (platformPaths.isEmpty()) {
                String paths = AppConfigReader.getInstance().getProperty(PLATFORM_PATH, LOCAL_PLATFORM);
                if (paths.contains(",")) {
                    platformPaths = Utility.getInstance().split(paths, ", ");
                } else {
                    platformPaths.add(paths);
                }
            }
            /*
             * Immediate connect when running for the first time.
             * Thereafter, wait for 5 seconds and try again until
             * it is connected.
             */
            log.info("{} = {}", PLATFORM_PATH, platformPaths);
            while (normal) {
                long now = System.currentTimeMillis();
                if (connector.isConnected()) {
                    timer = now;
                    if (connector.isReady()) {
                        // keep-alive when idle
                        if (now - lastActivity > idleTimeout) {
                            connector.keepAlive();
                            lastActivity = System.currentTimeMillis();
                        } else {
                            connector.isAlive();
                        }
                    } else {
                        if (now - connector.getHandshakeStartTime() > MAX_HANDSHAKE_WAIT) {
                            String message = "Event node failed to handshake in "+(MAX_HANDSHAKE_WAIT / 1000)+" seconds";
                            log.error(message);
                            try {
                                Utility.getInstance().closeConnection(connector.getTxPath(), CloseReason.CloseCodes.GOING_AWAY, message);
                            } catch (IOException e) {
                                // ok to ignore
                            }
                        }
                    }

                } else {
                    if (now - timer >= WAIT_INTERVAL) {
                        timer = System.currentTimeMillis();
                        try {
                            connect();
                        } catch (Exception e) {
                            // this should never occur but just in case...
                            log.error("Unexpected error when reconnect - {}", e.getMessage());
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
        }
    }

    public static void touch() {
        lastActivity = System.currentTimeMillis();
    }

    public static void close() {
        normal = false;
    }

    private void connect() {
        EventNodeConnector connector = EventNodeConnector.getInstance();
        connector.setState(EventNodeConnector.State.CONNECTING);
        Platform platform = Platform.getInstance();
        for (String path: platformPaths) {
            try {
                URI uri = new URI((path.endsWith("/")? path : path+"/") + platform.getOrigin());
                try {
                    WsClientEndpoint endpoint = new WsClientEndpoint(EventNodeConnector.getInstance(), uri);
                    WebSocketContainer container = ContainerProvider.getWebSocketContainer();
                    // use default client endpoint configuration
                    container.connectToServer(endpoint, ClientEndpointConfig.Builder.create().build(), uri);
                    break; // exit after successful connection
                } catch (DeploymentException | IOException e) {
                    log.warn("{} {}", simplifiedError(e.getMessage()), uri);
                    try {
                        Thread.sleep(1000);
                    } catch (InterruptedException e1) {
                        // ok to ignore
                    }
                }
            } catch (URISyntaxException e) {
                log.error("Invalid event node URL {}", path);
            }
        }
    }

    private String simplifiedError(String error) {
        String message = error.contains(":") ? error.substring(error.lastIndexOf(':')+1).trim() : error;
        return message.equals("no further information") || message.contains("null")
                || message.contains("connection fail")? "Unreachable" : message;
    }


}
