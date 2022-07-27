/*

    Copyright 2018-2022 Accenture Technology

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

package org.platformlambda.websocket;

import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import javax.websocket.Session;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.util.Map;

@ZeroTracing
public class WsTransmitter implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(WsTransmitter.class);

    public static final String STATUS = "status";
    public static final String MESSAGE = "message";
    private Session session;

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        try {
            send(headers, body);
        } catch (RuntimeException e) {
            /*
             * Let the underlying websocket system handles the socket clean up.
             * It is likely that the connection is closed before we can detect it.
             * e.g. "IllegalStateException: Message will not be sent because the WebSocket session has been closed"
             */
            log.error("Unable to send websocket message - {}, {}", e.getClass().getSimpleName(), e.getMessage());
        }
        return null;
    }

    public void send(Map<String, String> headers, Object body) throws IOException {
        if (body == null && headers.containsKey(WsEnvelope.TYPE)) {
            if (WsEnvelope.CLOSE.equals(headers.get(WsEnvelope.TYPE))) {
                if (session != null && session.isOpen()) {
                    int status = Utility.getInstance().str2int(headers.get(STATUS));
                    String message = headers.get(MESSAGE) == null? "" : headers.get(MESSAGE);
                    if (status >= 0) {
                        session.close(new CloseReason(CloseReason.CloseCodes.getCloseCode(status), message));
                    } else {
                        session.close();
                    }
                }
                // remove references
                session = null;
            }
            if (WsEnvelope.OPEN.equals(headers.get(WsEnvelope.TYPE))
                    && headers.containsKey(WsEnvelope.ROUTE)) {
                WsEnvelope envelope = WsRegistry.getInstance().get(headers.get(WsEnvelope.ROUTE));
                if (envelope != null) {
                    session = WsRegistry.getInstance().getSession(envelope.route);
                }
            }

        } else if (body instanceof byte[]) {
            if (session != null && session.isOpen()) {
                session.getBasicRemote().sendBinary(ByteBuffer.wrap((byte[]) body));
            }

        } else if (body instanceof String) {
            if (session != null && session.isOpen()) {
                session.getBasicRemote().sendText((String) body);
            }
        } else {
            if (session != null && session.isOpen()) {
                session.getBasicRemote().sendText(SimpleMapper.getInstance().getMapper().writeValueAsString(body));
            }
        }
    }

}