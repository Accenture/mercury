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

import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.models.WsRouteSet;
import org.platformlambda.core.services.WsTransmitter;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.WsConfigurator;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.Session;
import java.io.IOException;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class WsRegistry {
    private static final Logger log = LoggerFactory.getLogger(WsRegistry.class);

    // JSR-356 websocket sessionId to route mapping
    private static final ConcurrentMap<String, String> sessionToRoute = new ConcurrentHashMap<>();
    // route to WsEnvelope mapping
    private static final ConcurrentMap<String, WsEnvelope> registry = new ConcurrentHashMap<>();
    private static final String IP = "ip";
    private static final WsRegistry instance = new WsRegistry();

    private WsRegistry() {
        // singleton
    }

    public static WsRegistry getInstance() {
        return instance;
    }

    public void createHandler(LambdaFunction service, Session session) throws IOException {
        WebSocketService annotation = service.getClass().getAnnotation(WebSocketService.class);
        String name = annotation != null && annotation.value().length() > 0?
                Utility.getInstance().filteredServiceName(annotation.value()) : "websocket";
        // for websocket server
        String ip = getRemoteIp(session);
        String uri = session.getRequestURI().getPath();
        String path = uri.contains("?") ? uri.substring(0, uri.indexOf('?')) : uri;
        WsRouteSet rs = new WsRouteSet(name);
        createHandler(service, session, new WsEnvelope(rs.getRoute(), rs.getTxPath(), ip, path, session.getQueryString()));
    }

    public void createHandler(LambdaFunction service, Session session, WsEnvelope connection) throws IOException {
        // update websocket configuration
        WsConfigurator.getInstance().update(session);
        connection.session = session;
        registry.put(connection.route, connection);
        sessionToRoute.put(session.getId(), connection.route);
        Platform platform = Platform.getInstance();
        platform.registerPrivate(connection.route, service,1);
        platform.registerPrivate(connection.txPath, new WsTransmitter(),1);
    }

    public String getRoute(String sessionId) {
        return sessionToRoute.get(sessionId);
    }

    public String getTxPath(String sessionId) {
        String route = getRoute(sessionId);
        if (route != null) {
            WsEnvelope envelope = get(route);
            if (envelope != null) {
                return envelope.txPath;
            }
        }
        return null;
    }

    public Session getSession(String route) {
        WsEnvelope connection = registry.get(route);
        return connection == null? null : connection.session;
    }

    public WsEnvelope get(String route) {
        return registry.get(route);
    }

    public boolean exists(String route) {
        return registry.get(route) != null;
    }

    public void release(String route) {
        WsEnvelope connection = registry.get(route);
        if (connection != null) {
            Platform platform = Platform.getInstance();
            try {
                platform.release(connection.txPath);
            } catch (Exception e) {
                log.error("Unable to release {} - {}", connection.txPath, e.getMessage());
            }
            try {
                platform.release(connection.route);
            } catch (Exception e) {
                log.error("Unable to release {} - {}", connection.route, e.getMessage());
            }
            // clean up websocket registry
            if (registry.containsKey(route)) {
                WsEnvelope envelope = registry.get(route);
                registry.remove(route);
                if (envelope.session != null) {
                    String sessionId = envelope.session.getId();
                    sessionToRoute.remove(sessionId);
                    // remove references
                    envelope.session = null;
                    envelope.sessionKey = null;
                    envelope.publicKey = null;
                    log.info("Session-{} ({}, {}) released", sessionId, envelope.route, envelope.txPath);
                }
            }
        }
    }

    public int size() {
        return registry.size();
    }

    private String getRemoteIp(Session session) {
        // the remote IP address is encoded as a parameter in the query string by the WsRequestWrapper HTTP filter
        String query = session.getQueryString();
        if (query != null) {
            Utility util = Utility.getInstance();
            List<String> parameters = util.split(query, "&");
            for (String p : parameters) {
                int eq = p.indexOf('=');
                if (eq > 1) {
                    if (IP.equals(p.substring(0, eq))) {
                        return p.substring(eq+1);
                    }
                }
            }
        }
        return "unknown";
    }
}
