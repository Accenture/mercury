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

package org.platformlambda.core.websocket.client;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.models.WsRouteSet;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.WsRegistry;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.*;
import java.io.IOException;
import java.net.URI;
import java.util.concurrent.TimeoutException;

public class WsClientEndpoint extends Endpoint {
    private static final Logger log = LoggerFactory.getLogger(WsClientEndpoint.class);

    private static final WsRegistry registry = WsRegistry.getInstance();

    private LambdaFunction service;
    private URI uri;
    private String route;
    private Session session;
    private boolean open = false;

    public WsClientEndpoint(LambdaFunction service, URI uri) {
        this.service = service;
        this.uri = uri;
    }

    @Override
    public void onOpen(Session session, EndpointConfig config) {
        this.session = session;
        open = true;
        // create websocket routing metadata
        WsRouteSet rs = new WsRouteSet("ws.client");
        route = rs.getRoute();
        try {
            WsEnvelope envelope = new WsEnvelope(rs.getRoute(), rs.getTxPath(), uri.getHost(), uri.getPath(), uri.getQuery());
            // setup listener and transmitter
            registry.createHandler(service, session, envelope);
            // send open event to the newly created websocket transmitter and wait for completion
            PostOffice.getInstance().request(envelope.txPath, 5000,
                    new Kv(WsEnvelope.ROUTE, route), new Kv(WsEnvelope.TYPE, WsEnvelope.OPEN));
            // then inform the sender function
            PostOffice.getInstance().send(route, new Kv(WsEnvelope.TYPE, WsEnvelope.OPEN),
                    new Kv(WsEnvelope.ROUTE, route), new Kv(WsEnvelope.TX_PATH, envelope.txPath),
                    new Kv(WsEnvelope.IP, envelope.ip), new Kv(WsEnvelope.PATH, envelope.path),
                    new Kv(WsEnvelope.QUERY, envelope.query == null? "" : envelope.query),
                    new Kv(WsEnvelope.TOKEN, envelope.origin));

            session.addMessageHandler(new TextHandler());
            session.addMessageHandler(new BytesHandler());

            if (envelope.query == null) {
                log.info("{} {} connected to {} {}, {}", service.getClass().getSimpleName(), session.getId(),
                        route, envelope.ip, envelope.path);
            } else {
                log.info("{} {} connected to {} {}, {}, {}", service.getClass().getSimpleName(), session.getId(),
                        route, envelope.ip, envelope.path, envelope.query);
            }
        } catch (IOException | TimeoutException e) {
            log.error("Unable to connect to {}, {}", uri, e.getMessage());
        } catch (AppException e) {
            log.error("Unable to connect to {}, status={}, error={}", uri, e.getStatus(), e.getMessage());
        }


    }

    @Override
    public void onClose(Session session, CloseReason reason) {
        open = false;
        String route = registry.getRoute(session.getId());
        if (route != null) {
            WsEnvelope envelope = registry.get(route);
            if (envelope != null) {
                log.info("{} {} closed ({}, {})", session.getId(), route,
                        reason.getCloseCode().getCode(), reason.getReasonPhrase());
                try {
                    /*
                     * Send close event to the handler to release resources.
                     * txPath is not provided because it would have been closed at the time when the handler receives the close event.
                     */
                    PostOffice.getInstance().send(route, new Kv(WsEnvelope.ROUTE, route),
                            new Kv(WsEnvelope.TOKEN, envelope.origin),
                            new Kv(WsEnvelope.TYPE, WsEnvelope.CLOSE));
                    // release websocket registry resources
                    registry.release(route);
                } catch (IOException e) {
                    log.error("Unable to close {} due to {}", route, e.getMessage());
                }
            }
        }
    }

    @Override
    public void onError(Session session, Throwable error) {
        // log all errors except connection failure
        if (open) {
            log.warn("#{} {} exception {}", session.getId(), route, error.getMessage());
        }
    }

    private class TextHandler implements MessageHandler.Whole<String> {

        @Override
        public void onMessage(String message) {

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
    }

    private class BytesHandler implements MessageHandler.Whole<byte[]> {

        @Override
        public void onMessage(byte[] payload) {

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
    }

}
