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

package org.platformlambda.node.system;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.services.WsTransmitter;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.node.models.ConnectionStatus;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.io.IOException;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class ConnectionMonitor extends Thread {
    private static final Logger log = LoggerFactory.getLogger(ConnectionMonitor.class);

    private static final ConcurrentMap<String, ConnectionStatus> connections = new ConcurrentHashMap<>();
    private static final long TIMEOUT = 8000;

    @Override
    public void run() {

        long timeoutSeconds = TIMEOUT / 1000;

        while (true) {
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                // ok to ignore
            }
            long now = System.currentTimeMillis();
            for (String c: connections.keySet()) {
                ConnectionStatus status = connections.get(c);
                if (!status.authenticated) {
                    if (now - status.connectTime > TIMEOUT) {
                        log.error("{} does not handshake in {} seconds", status.route, timeoutSeconds);
                        EventEnvelope error = new EventEnvelope();
                        error.setTo(status.txPath);
                        error.setHeader(WsTransmitter.STATUS, String.valueOf(CloseReason.CloseCodes.PROTOCOL_ERROR.getCode()));
                        error.setHeader(WsTransmitter.MESSAGE, "Authentication does not complete in "+timeoutSeconds+" seconds");
                        error.setHeader(WsEnvelope.TYPE, WsEnvelope.CLOSE);
                        try {
                            PostOffice.getInstance().send(error);
                        } catch (IOException e) {
                            log.error("Unable to close connection {} - {}", status.txPath, e.getMessage());
                        }
                    }
                }
            }

        }
    }

    public static void addConnection(String route, String txPath) {
        connections.put(route, new ConnectionStatus(route, txPath));
    }

    public static void acceptConnection(String route) {
        ConnectionStatus status = connections.get(route);
        if (status != null) {
            status.accept();
        }
    }

    public static void removeConnection(String route) {
        connections.remove(route);
    }

}
