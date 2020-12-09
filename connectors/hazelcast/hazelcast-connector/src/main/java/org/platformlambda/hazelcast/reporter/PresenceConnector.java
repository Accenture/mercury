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

package org.platformlambda.hazelcast.reporter;

import org.platformlambda.core.models.*;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Date;
import java.util.Map;

public class PresenceConnector implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(PresenceConnector.class);

    private static final String TYPE = "type";
    private static final String INSTANCE = "instance";
    private static final String ALIVE = "keep-alive";
    private static final String JOIN = "join";
    private static final String LEAVE = "leave";
    private static final String ORIGIN = "origin";
    private static final String INFO = "info";
    private static final String READY = "ready";
    private static final String SEQ = "seq";
    private static final String CREATED = "created";
    private static final String NAME = "name";
    private static final String VERSION = "version";
    final private String created;
    public enum State {
        UNASSIGNED, CONNECTED, DISCONNECTED
    }
    private State state = State.UNASSIGNED;
    private String monitor;
    private long seq = 1;
    private boolean ready = false;
    private static final PresenceConnector instance = new PresenceConnector();

    private PresenceConnector() {
        created = Utility.getInstance().date2str(new Date(), true);
    }

    public static PresenceConnector getInstance() {
        return instance;
    }

    @SuppressWarnings("unchecked")
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
                    ready = false;
                    state = State.CONNECTED;
                    // register basic application info
                    sendAppInfo(seq++, false);
                    log.info("Connected {}, {}, {}", route, ip, path);
                    break;
                case WsEnvelope.CLOSE:
                    // the close event contains route and token for this websocket
                    route = headers.get(WsEnvelope.ROUTE);
                    monitor = null;
                    ready = false;
                    state = State.DISCONNECTED;
                    log.info("Disconnected {}", route);
                    // tell service registry to clear routing table
                    PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, LEAVE),
                            new Kv(ORIGIN, Platform.getInstance().getOrigin()));
                    break;
                case WsEnvelope.BYTES:
                    route = headers.get(WsEnvelope.ROUTE);
                    EventEnvelope event = new EventEnvelope();
                    event.load((byte[]) body);
                    if (READY.equals(event.getTo())) {
                        ready = true;
                        log.info("Activated {}", route);
                        String platformVersion = event.getHeaders().getOrDefault(VERSION, "1.0");
                        // tell peers that this server has joined
                        po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, JOIN),
                                new Kv(VERSION, platformVersion),
                                new Kv(ORIGIN, platform.getOrigin()));
                    } else {
                        po.send(event);
                    }
                    break;
                case WsEnvelope.MAP:
                    if (body instanceof Map) {
                        Map<String, Object> data = (Map<String, Object>) body;
                        if (ALIVE.equals(data.get(TYPE))) {
                            sendAppInfo(seq++, true);
                        }
                    }
                    break;
                case WsEnvelope.STRING:
                    log.debug("{}", body);
                    break;
                default:
                    break;
            }
        }
        return null;
    }

    public boolean isConnected() {
        return state == State.CONNECTED;
    }

    public boolean isReady() {
        return ready;
    }

    private void sendAppInfo(long n, boolean alive) {
        if (monitor != null) {
            try {
                String instance = Platform.getInstance().getConsistentAppId();
                VersionInfo app = Utility.getInstance().getVersionInfo();
                EventEnvelope info = new EventEnvelope();
                info.setTo(alive? ALIVE : INFO).setHeader(CREATED, created).setHeader(SEQ, n)
                        .setHeader(NAME, Platform.getInstance().getName())
                        .setHeader(VERSION, app.getVersion())
                        .setHeader(TYPE, ServerPersonality.getInstance().getType());
                if (instance != null) {
                    info.setHeader(INSTANCE, Platform.getInstance().getConsistentAppId());
                }
                PostOffice.getInstance().send(monitor, info.toBytes());

            } catch (IOException e) {
                log.error("Unable to send application info to presence monitor - {}", e.getMessage());
            }
        }
    }

}
