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

package org.platformlambda.rest.core.websocket;

import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.WsRegistry;
import org.platformlambda.core.util.SimpleClassScanner;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.*;
import javax.websocket.server.PathParam;
import javax.websocket.server.ServerEndpoint;
import java.io.IOException;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

@ServerEndpoint(value = "/ws/{name}/{handle}")
public class WsServer {
    private static final Logger log = LoggerFactory.getLogger(WsServer.class);
    private static final WsRegistry registry = WsRegistry.getInstance();
    private static final Utility util = Utility.getInstance();
    private static boolean ready = false;
    private static boolean open = false;

    private static final ConcurrentMap<String, Class<LambdaFunction>> lambdas = new ConcurrentHashMap<>();

    public static Set<String> getUserPaths() {
        return ready? lambdas.keySet() : new HashSet<>();
    }

    @SuppressWarnings("unchecked")
    public static void begin() {
        // scan for user web socket service
        SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        Set<String> packages = scanner.getPackages(true);
        for (String p : packages) {
            List<Class<?>> services = scanner.getAnnotatedClasses(p, WebSocketService.class);
            for (Class<?> cls : services) {
                WebSocketService annotation = cls.getAnnotation(WebSocketService.class);
                if (annotation.value().length() > 0) {
                    if (!Utility.getInstance().validServiceName(annotation.value())) {
                        log.error("Unable to load {} ({}) because the path is not a valid service name",
                                cls.getName(), annotation.value());
                    }
                    String wsEndpoint = "/ws/"+annotation.value()+"/{handle}";
                    try {
                        Object o = cls.newInstance();
                        if (o instanceof LambdaFunction) {
                            lambdas.put(annotation.value(), (Class<LambdaFunction>) cls);
                            log.info("{} loaded as WEBSOCKET SERVER endpoint {}", cls.getName(), wsEndpoint);
                        } else {
                            log.error("Unable to load {} ({}) because it is not an instance of {}",
                                    cls.getName(), wsEndpoint, LambdaFunction.class.getName());
                        }

                    } catch (InstantiationException  | IllegalAccessException e) {
                        log.error("Unable to load {} ({}) - {}", cls.getName(), wsEndpoint, e.getMessage());
                    }
                }
            }
        }
        // initialized
        ready = true;
    }

    @OnOpen
    public void onOpen(Session session, @PathParam("name") final String name, @PathParam("handle") final String handle) throws IOException {
        open = true;
        if (!ready) {
            session.close(new CloseReason(CloseReason.CloseCodes.SERVICE_RESTART, "Starting up. Please try again."));
            return;
        }
        if (!lambdas.containsKey(name)) {
            session.close(new CloseReason(CloseReason.CloseCodes.CANNOT_ACCEPT, "Path /"+name+"/"+handle+" not available"));
            return;
        }

        Class<LambdaFunction> cls = lambdas.get(name);
        try {
            registry.createHandler(cls.newInstance(), session);
            String route = registry.getRoute(session.getId());
            WsEnvelope envelope = registry.get(route);
            // send open event to the newly created websocket transmitter and wait for completion
            PostOffice.getInstance().request(envelope.txPath, 5000,
                    new Kv(WsEnvelope.ROUTE, route), new Kv(WsEnvelope.TYPE, WsEnvelope.OPEN));
            // then inform the sender function
            PostOffice.getInstance().send(route, new Kv(WsEnvelope.TYPE, WsEnvelope.OPEN),
                    new Kv(WsEnvelope.ROUTE, route), new Kv(WsEnvelope.TX_PATH, envelope.txPath),
                    new Kv(WsEnvelope.IP, envelope.ip), new Kv(WsEnvelope.PATH, envelope.path),
                    new Kv(WsEnvelope.QUERY, normalizeQuery(envelope.query, envelope.ip)),
                    new Kv(WsEnvelope.TOKEN, envelope.origin));
            log.info("Session-{} {} {} connected to {} {}", session.getId(), route,
                    cls.getSimpleName(), envelope.ip, envelope.path);
        } catch (InstantiationException | IllegalAccessException | AppException | IOException | TimeoutException e) {
            log.error("Unable to start {} ({}), {}", cls.getName(), name, e.getMessage());
            session.close(new CloseReason(CloseReason.CloseCodes.CANNOT_ACCEPT, "Path /"+name+"/"+handle+" not available"));
        }
    }

    private String normalizeQuery(String query, String ip) {
        if (query.contains(WsEnvelope.IP)) {
            StringBuilder sb = new StringBuilder();
            List<String> parameters = util.split(query, "&");
            for (String p : parameters) {
                int eq = p.indexOf('=');
                if (eq > 0) {
                    String key = p.substring(0, eq);
                    String value = p.substring(eq + 1);
                    if (key.equals(WsEnvelope.IP) && ip.equals(value)) {
                        // skip IP address that was inserted by the InfoFilter
                        continue;
                    }
                }
                sb.append(p);
                sb.append('&');
            }
            return sb.length() == 0 ? "" : sb.substring(0, sb.length() - 1);
        } else {
            return query;
        }
    }

    @OnMessage
    public void onText(String message, Session session) {
        String route = registry.getRoute(session.getId());
        if (route != null) {
            WsEnvelope envelope = registry.get(route);
            if (envelope != null) {
                try {
                    PostOffice.getInstance().send(route, message, new Kv(WsEnvelope.TYPE, WsEnvelope.STRING),
                            new Kv(WsEnvelope.ROUTE, route), new Kv(WsEnvelope.TX_PATH, envelope.txPath));
                } catch (IOException e) {
                    log.error("Unable to route websocket message to {}, {}", route, e.getMessage());
                }
            }
        }
    }

    @OnMessage
    public void onBinary(byte[] payload, Session session) {
        String route = registry.getRoute(session.getId());
        if (route != null) {
            WsEnvelope envelope = registry.get(route);
            if (envelope != null) {
                try {
                    PostOffice.getInstance().send(route, payload, new Kv(WsEnvelope.TYPE, WsEnvelope.BYTES),
                            new Kv(WsEnvelope.ROUTE, route), new Kv(WsEnvelope.TX_PATH, envelope.txPath));
                } catch (IOException e) {
                    log.error("Unable to route websocket payload to {}, {}", route, e.getMessage());
                }
            }
        }
    }

    @OnClose
    public void onClose(Session session, CloseReason reason) {
        open = false;
        String route = registry.getRoute(session.getId());
        if (route != null) {
            WsEnvelope envelope = registry.get(route);
            if (envelope != null) {
                log.info("Session-{} {} closed ({}, {})", session.getId(), route,
                        reason.getCloseCode().getCode(), reason.getReasonPhrase());
                try {
                    /*
                     * Send close event to the handler to release resources.
                     * txPath is not provided because it would have been closed at the time when the handler receives the close event.
                     */
                    PostOffice.getInstance().send(route, new Kv(WsEnvelope.ROUTE, route),
                            new Kv(WsEnvelope.TOKEN, envelope.origin),
                            new Kv(WsEnvelope.CLOSE_CODE, reason.getCloseCode().getCode()),
                            new Kv(WsEnvelope.CLOSE_REASON, reason.getReasonPhrase()),
                            new Kv(WsEnvelope.TYPE, WsEnvelope.CLOSE));
                    // release websocket registry resources
                    registry.release(route);
                } catch (IOException e) {
                    log.error("Unable to close {} due to {}", route, e.getMessage());
                }
            }
        }
    }

    @OnError
    public void onError(Session session, Throwable error) {
        if (open) {
            String route = registry.getRoute(session.getId());
            log.warn("Session-{} {} exception {}", session.getId(), route, error.getMessage());
        }
    }


}
