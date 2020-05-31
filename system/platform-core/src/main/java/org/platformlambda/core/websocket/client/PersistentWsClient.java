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

package org.platformlambda.core.websocket.client;

import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.websocket.common.WsConfigurator;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import javax.websocket.ContainerProvider;
import javax.websocket.WebSocketContainer;
import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.util.*;

public class PersistentWsClient extends Thread implements AutoCloseable {
    private static final Logger log = LoggerFactory.getLogger(PersistentWsClient.class);

    private static final long WAIT_INTERVAL = 10000;
    private static final int IDLE_THRESHOLD = 10;
    private static final String TYPE = "type";
    private static final String ALIVE = "keep-alive";
    private static final String SEQ = "seq";
    final private List<String> urls;
    final private LambdaFunction connector;
    private ConnectorReady condition = () -> true;
    private SimpleClientEndpoint client = null;
    private long aliveTime = 0, aliveSeq = 1;
    private long timer = 0;
    private boolean normal = true;

    public PersistentWsClient(LambdaFunction connector, List<String> urls) {
        this.connector = connector;
        this.urls = urls;
    }

    public void setCondition(ConnectorReady condition) {
        if (condition != null) {
            this.condition = condition;
        }
    }

    @Override
    public void run() {
        Runtime.getRuntime().addShutdownHook(new Thread(this::close));
        log.info("Started");
        long idleSeconds = WsConfigurator.getInstance().getIdleTimeout() - IDLE_THRESHOLD;
        long idleTimeout = (idleSeconds < IDLE_THRESHOLD? IDLE_THRESHOLD : idleSeconds) * 1000;
        /*
         * Immediate connect when running for the first time.
         * Thereafter, wait for 5 seconds and try again until it is connected.
         */
        log.info("Connection list {}", urls);
        while (normal) {
            try {
                manageConnection(idleTimeout / 2);
            } catch (Exception e) {
                log.error("Unexpected connectivity issue - {}", e.getMessage());
                // guaranteed persistent connection
                if (client != null && client.isConnected()) {
                    try {
                        client.close(new CloseReason(CloseReason.CloseCodes.GOING_AWAY, e.getMessage()));
                    } catch (IOException ex) {
                        // ok to ignore
                    }
                }
                client = null;
            }
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e1) {
                // ok to ignore
            }
        }
        log.info("Stopped");
    }

    private void manageConnection(long keepAliveInterval) {
        long now = System.currentTimeMillis();
        if (client != null && client.isConnected()) {
            if (condition.isReady()) {
                if (now - aliveTime > keepAliveInterval) {
                    keepAlive();
                }
            } else {
                disconnect("Disconnect due to external condition");
            }
        } else {
            if (client != null && client.justDisconnected()) {
                client = null;
                // when it is just disconnected, we want to wait one cycle before reconnect
                timer = now;
                log.info("Just disconnected");
            } else if (now - timer >= WAIT_INTERVAL) {
                try {
                    if (condition.isReady()) {
                        timer = now;
                        connect();
                    }
                } catch (Exception e) {
                    log.error("Unexpected error when reconnect - {}", e.getMessage());
                }
            }
        }
    }

    private void connect() {
        Platform platform = Platform.getInstance();
        List<String> urlList = new ArrayList<>(this.urls);
        if (urlList.size() > 1) {
            Collections.shuffle(urlList);
        }
        for (String path : urlList) {
            if (path.startsWith("ws://") || path.startsWith("wss://")) {
                try {
                    URI uri = new URI((path.endsWith("/") ? path : path + "/") + platform.getOrigin());
                    try {
                        SimpleClientEndpoint client = new SimpleClientEndpoint(connector, uri);
                        WebSocketContainer container = ContainerProvider.getWebSocketContainer();
                        container.connectToServer(client, uri);
                        this.client = client;
                        return; // exit after successful connection
                    } catch (Exception e) {
                        log.warn("{} {}", simplifiedError(e.getMessage()), uri);
                    }
                } catch (URISyntaxException e) {
                    log.error("Invalid event node URL {}", path);
                }
            } else {
                log.error("Invalid websocket URL {} ignored", path);
            }
        }
    }

    private void keepAlive() {
        if (client != null && client.isConnected() && client.getTxPath() != null) {
            PostOffice po = PostOffice.getInstance();
            aliveTime = System.currentTimeMillis();
            try {
                Map<String, Object> alive = new HashMap<>();
                alive.put(TYPE, ALIVE);
                alive.put(SEQ, aliveSeq++);
                /*
                 * tell primary handler so the client can take action to handle keep-alive
                 */
                po.send(client.getRoute(), alive, new Kv(WsEnvelope.TYPE, WsEnvelope.MAP),
                        new Kv(WsEnvelope.ROUTE, client.getRoute()),
                        new Kv(WsEnvelope.TX_PATH, client.getTxPath()));
                /*
                 * send keep-alive to the remote websocket server so the socket will not timeout
                 */
                po.send(client.getTxPath(), alive);
            } catch (IOException e) {
                log.error("Unable to send keep alive to {} - {}", client.getUri(), e.getMessage());
            }
        }
    }

    private String simplifiedError(String error) {
        String message = error.contains(":") ? error.substring(error.lastIndexOf(':')+1).trim() : error;
        return message.equals("no further information") || message.contains("null")
                || message.contains("connection fail")? "Unreachable" : message;
    }

    public void disconnect(String reason) {
        if (client != null && client.isConnected()) {
            try {
                client.close(new CloseReason(CloseReason.CloseCodes.GOING_AWAY, reason));
            } catch (IOException e) {
                // ok to ignore
            }
            client = null;
        }
    }

    @Override
    public void close() {
        disconnect("Shutdown");
        normal = false;
    }

}
