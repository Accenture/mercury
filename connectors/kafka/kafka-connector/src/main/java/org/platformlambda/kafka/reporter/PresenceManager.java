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

import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
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
import java.util.concurrent.TimeoutException;

public class PresenceManager extends Thread {
    private static final Logger log = LoggerFactory.getLogger(PresenceManager.class);

    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;
    private static final String SERVICE_REGISTRY = ServiceDiscovery.SERVICE_REGISTRY;
    private static final long WAIT_INTERVAL = 5000;
    private static final long MAX_HANDSHAKE_WAIT = 10000;

    private static long lastSeen = System.currentTimeMillis();
    private static boolean normal = true;

    private List<String> platformPaths = new ArrayList<>();
    private Session session;
    private String url;
    private long lastPending = 0, timer = 0;
    
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
        PresenceConnector connector = PresenceConnector.getInstance();
        if (connector.isUnassigned()) {
            lastSeen = System.currentTimeMillis();
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
            log.info("Reporting to {} with keep-alive {} seconds ", platformPaths, interval / 1000);
            while (normal) {
                try {
                    manageConnection(interval);
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
                    Thread.sleep(2000);
                } catch (InterruptedException e1) {
                    // ok to ignore
                }
            }
            log.info("Stopped");
        }
    }

    private void manageConnection(long interval) {
        PresenceConnector connector = PresenceConnector.getInstance();
        long now = System.currentTimeMillis();
        if (session != null && session.isOpen() && connector.isConnected()) {
            if (connector.isReady()) {
                connector.checkActivation();
                timer = now;
                // keep-alive
                if (now - lastSeen > interval) {
                    connector.keepAlive();
                    lastSeen = System.currentTimeMillis();
                } else {
                    connector.isAlive();
                }
            } else {
                if (now - lastPending > MAX_HANDSHAKE_WAIT) {
                    try {
                        connector.setState(PresenceConnector.State.ERROR);
                        String message = "Presence monitor does not respond in "+(MAX_HANDSHAKE_WAIT / 1000)+" seconds";
                        log.error(message);
                        session.close(new CloseReason(CloseReason.CloseCodes.GOING_AWAY, message));
                    } catch (IOException e) {
                        log.error("Unable to close session - {}", e.getMessage());
                    }
                    session = null;
                } else {
                    log.warn("Waiting for presence monitor to respond");
                    connector.sendAppInfo();
                }
            }

        } else {
            if (now - timer >= WAIT_INTERVAL) {
                timer = System.currentTimeMillis();
                try {
                    connect();
                } catch (Exception e) {
                    log.error("Unexpected error when trying to connect {} - {}", platformPaths, e.getMessage());
                }
            }
        }
    }

    public static void touch() {
        lastSeen = System.currentTimeMillis();
    }

    private void connect() throws IOException {
        Platform platform = Platform.getInstance();
        PresenceConnector connector = PresenceConnector.getInstance();
        if (lastPending == 0) {
            connector.setState(PresenceConnector.State.CONNECTING);
            for (String path : platformPaths) {
                try {
                    URI uri = new URI((path.endsWith("/") ? path : path + "/") + platform.getOrigin());
                    try {
                        WebSocketContainer container = ContainerProvider.getWebSocketContainer();
                        session = container.connectToServer(new SimpleClientEndpoint(connector, uri), uri);
                        lastPending = System.currentTimeMillis();
                        log.info("Reporting to {}", uri);
                        return; // exit after successful connection
                    } catch (Exception e) {
                        log.warn("{} {}", simpleError(e.getMessage()), uri);
                    }
                } catch (URISyntaxException e) {
                    log.error("Invalid service monitor URL " + path, e);
                    System.exit(-1);
                }
            }
        } else {
            if (System.currentTimeMillis() - lastPending > MAX_HANDSHAKE_WAIT) {
                if (session != null && session.isOpen()) {
                    connector.setState(PresenceConnector.State.ERROR);
                    String message = "Presence monitor does not respond in "+(MAX_HANDSHAKE_WAIT / 1000)+" seconds";
                    log.error(message);
                    session.close(new CloseReason(CloseReason.CloseCodes.GOING_AWAY, message));
                }
            } else {
                // give remote end a few seconds to respond
                if (session != null && session.isOpen()) {
                    log.warn("Waiting for presence monitor to respond");
                    connector.sendAppInfo();
                    return;
                }
            }
        }
        session = null;
        lastPending = 0;
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
