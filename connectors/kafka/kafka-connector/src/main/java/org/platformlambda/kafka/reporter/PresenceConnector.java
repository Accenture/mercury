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

import org.platformlambda.core.models.*;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.io.IOException;
import java.util.Map;

public class PresenceConnector implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(PresenceConnector.class);

    private static final String TYPE = "type";
    private static final String JOIN = "join";
    private static final String LEAVE = "leave";
    private static final String ORIGIN = "origin";
    private static final String INFO = "info";
    private static final String READY = "ready";
    private static final String SEQ = "seq";
    private static final String ALIVE = "keep-alive";
    private static final long MAX_WAIT = 8 * 1000;

    private static PresenceConnector instance = new PresenceConnector();
    public enum State {
        UNASSIGNED, CONNECTING, CONNECTED, DISCONNECTED, ERROR
    }
    private State state = State.UNASSIGNED;
    private String monitor;
    private long connectTime = 0;
    private long aliveTime = 0, aliveSeq = 0;
    private boolean ready = false, checkingAlive = false;

    private PresenceConnector() {
        // singleton
    }

    public static PresenceConnector getInstance() {
        return instance;
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {

        Platform platform  = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        String route;

        if (headers.containsKey(WsEnvelope.TYPE)) {
            switch (headers.get(WsEnvelope.TYPE)) {
                case WsEnvelope.OPEN:
                    // the open event contains route, txPath, ip, path, query and token
                    route = headers.get(WsEnvelope.ROUTE);
                    String ip = headers.get(WsEnvelope.IP);
                    String path = headers.get(WsEnvelope.PATH);
                    monitor = headers.get(WsEnvelope.TX_PATH);
                    aliveSeq = 0;
                    connectTime = System.currentTimeMillis();
                    ready = false;
                    checkingAlive = false;
                    setState(State.CONNECTED);
                    sendAppInfo();
                    log.info("Connected {}, {}, {}", route, ip, path);
                    break;
                case WsEnvelope.CLOSE:
                    // the close event contains route and token for this websocket
                    route = headers.get(WsEnvelope.ROUTE);
                    monitor = null;
                    connectTime = 0;
                    ready = false;
                    checkingAlive = false;
                    setState(State.DISCONNECTED);
                    log.info("Disconnected {}", route);
                    // tell service registry to clear routing table
                    po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, LEAVE), new Kv(ORIGIN, platform.getOrigin()));
                    break;
                case WsEnvelope.BYTES:
                    route = headers.get(WsEnvelope.ROUTE);
                    EventEnvelope event = new EventEnvelope();
                    event.load((byte[]) body);
                    if (READY.equals(event.getTo())) {
                        ready = true;
                        log.info("Activated {}", route);
                        // tell peers that this server has joined
                        po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, JOIN), new Kv(ORIGIN, platform.getOrigin()));
                    } else {
                        po.send(event);
                    }
                    break;
                case WsEnvelope.STRING:
                    String message = (String) body;
                    log.debug("{}", body);
                    if (message.contains(ALIVE)) {
                        aliveTime = System.currentTimeMillis();
                        checkingAlive = false;
                        PresenceManager.touch();
                    }
                    break;
                default:
                    // this should not happen
                    if (body instanceof byte[]) {
                        // case WsEnvelope.BYTES
                        log.error("Invalid event {} with {} bytes", headers, ((byte[]) body).length);
                    } else {
                        log.error("Invalid event {} with body={}", headers, body);
                    }
                    break;
            }
        }
        return null;
    }

    public void setState(State state) {
        this.state = state;
    }

    public boolean isUnassigned() {
        return state == State.UNASSIGNED;
    }

    public boolean isConnected() {
        return state == State.CONNECTED;
    }

    public boolean isReady() {
        return ready;
    }

    public void checkActivation() {
        if (!ready && monitor != null && connectTime > 0 && System.currentTimeMillis() - connectTime > MAX_WAIT) {
            String message = "Connection not activated within "+(MAX_WAIT / 1000)+" seconds";
            connectTime = 0;
            log.error(message);
            try {
                Utility.getInstance().closeConnection(monitor, CloseReason.CloseCodes.GOING_AWAY, message);
            } catch (IOException e) {
                // ok to ignore
            }
        }
    }

    public void isAlive() {
        if (checkingAlive && monitor != null && (System.currentTimeMillis() - aliveTime > MAX_WAIT)) {
            String message = "Presence monitor failed to keep alive in "+(MAX_WAIT / 1000)+" seconds";
            log.error(message);
            try {
                Utility.getInstance().closeConnection(monitor, CloseReason.CloseCodes.GOING_AWAY, message);
            } catch (IOException e) {
                // ok to ignore
            }
        }
    }

    public void keepAlive() {
        if (monitor != null) {
            checkingAlive = true;
            aliveTime = System.currentTimeMillis();
            aliveSeq++;
            try {
                PostOffice.getInstance().send(monitor, new EventEnvelope().setTo(ALIVE).setHeader(SEQ, aliveSeq).toBytes());

            } catch (IOException e) {
                log.error("Unable to send keep alive - {}", e.getMessage());
            }
        }
    }

    public void sendAppInfo() {
        if (monitor != null) {
            try {
                VersionInfo app = Utility.getInstance().getVersionInfo();
                EventEnvelope info = new EventEnvelope();
                info.setTo(INFO);
                info.setHeader("name", Platform.getInstance().getName());
                info.setHeader("version", app.getVersion());
                info.setHeader("type", ServerPersonality.getInstance().getType().toString());
                PostOffice.getInstance().send(monitor, info.toBytes());

            } catch (IOException e) {
                log.error("Unable to send application info to presence monitor - {}", e.getMessage());
            }
        }
    }


}
