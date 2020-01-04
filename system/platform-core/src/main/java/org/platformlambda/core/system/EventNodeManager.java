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

import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.SimpleClientEndpoint;
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

    private static long lastSeen = System.currentTimeMillis();

    private List<String> platformPaths = new ArrayList<>();
    private Session session;
    private long lastPending = 0, timer = 0;
    private boolean normal = true;

    public EventNodeManager() {
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
    }

    @Override
    public void run() {
        EventNodeConnector connector = EventNodeConnector.getInstance();
        if (connector.isUnassigned()) {
            log.info("Started");
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
                try {
                    manageConnection(idleTimeout);
                } catch (Exception e) {
                    log.error("Unexpected connectivity issue - {}", e.getMessage());
                    // guaranteed persistent connection
                    if (session != null && session.isOpen()) {
                        try {
                            session.close(new CloseReason(CloseReason.CloseCodes.GOING_AWAY, e.getMessage()));
                        } catch (IOException ex) {
                            // ok to ignore
                        }
                    }
                    session = null;
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

    private void manageConnection(long idleTimeout) {
        EventNodeConnector connector = EventNodeConnector.getInstance();
        long now = System.currentTimeMillis();
        if (session != null && session.isOpen() && connector.isConnected()) {
            timer = now;
            lastPending = 0;
            if (connector.isReady()) {
                // keep-alive when idle
                if (now - lastSeen > idleTimeout) {
                    connector.keepAlive();
                    lastSeen = System.currentTimeMillis();
                } else {
                    connector.isAlive();
                }
            } else {
                if (now - connector.getHandshakeStartTime() > MAX_HANDSHAKE_WAIT) {
                    try {
                        connector.setState(EventNodeConnector.State.ERROR);
                        String message = "Event node does not handshake in "+(MAX_HANDSHAKE_WAIT / 1000)+" seconds";
                        log.error(message);
                        session.close(new CloseReason(CloseReason.CloseCodes.GOING_AWAY, message));
                    } catch (IOException e) {
                        log.error("Unable to close session - {}", e.getMessage());
                    }
                    session = null;
                } else {
                    log.warn("Waiting for event node to handshake");
                }
            }

        } else {
            if (now - timer >= WAIT_INTERVAL) {
                timer = System.currentTimeMillis();
                try {
                    connect();
                } catch (Exception e) {
                    log.error("Unexpected error when reconnect - {}", e.getMessage());
                }
            }
        }
    }

    public static void touch() {
        lastSeen = System.currentTimeMillis();
    }

    private void connect() throws IOException {
        Platform platform = Platform.getInstance();
        EventNodeConnector connector = EventNodeConnector.getInstance();
        if (lastPending == 0) {
            connector.setState(EventNodeConnector.State.CONNECTING);
            for (String path : platformPaths) {
                try {
                    URI uri = new URI((path.endsWith("/") ? path : path + "/") + platform.getOrigin());
                    try {
                        WebSocketContainer container = ContainerProvider.getWebSocketContainer();
                        session = container.connectToServer(new SimpleClientEndpoint(connector, uri), uri);
                        lastPending = System.currentTimeMillis();
                        return; // exit after successful connection
                    } catch (Exception e) {
                        log.warn("{} {}", simplifiedError(e.getMessage()), uri);
                    }
                } catch (URISyntaxException e) {
                    log.error("Invalid event node URL {}", path);
                }
            }
        } else {
            if (System.currentTimeMillis() - lastPending > MAX_HANDSHAKE_WAIT) {
                if (session != null && session.isOpen()) {
                    connector.setState(EventNodeConnector.State.ERROR);
                    String message = "Event node does not respond in "+(MAX_HANDSHAKE_WAIT / 1000)+" seconds";
                    log.error(message);
                    session.close(new CloseReason(CloseReason.CloseCodes.GOING_AWAY, message));
                }
            } else {
                // give remote end a few seconds to respond
                if (session != null && session.isOpen()) {
                    log.warn("Waiting for event node to respond");
                    return;
                }
            }
        }
        session = null;
        lastPending = 0;
    }

    private String simplifiedError(String error) {
        String message = error.contains(":") ? error.substring(error.lastIndexOf(':')+1).trim() : error;
        return message.equals("no further information") || message.contains("null")
                || message.contains("connection fail")? "Unreachable" : message;
    }

    private void shutdown() {
        normal = false;
    }

}
