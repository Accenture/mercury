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

package org.platformlambda.core.services;

import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.WsRegistry;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;

import javax.websocket.CloseReason;
import javax.websocket.Session;
import java.nio.ByteBuffer;
import java.util.Map;

@ZeroTracing
public class WsTransmitter implements LambdaFunction {

    private static final CryptoApi crypto = new CryptoApi();
    private static final Utility util = Utility.getInstance();
    public static final String STATUS = "status";
    public static final String MESSAGE = "message";
    private Session session;
    private byte[] sessionKey;

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
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
                sessionKey = null;
            }
            if (WsEnvelope.OPEN.equals(headers.get(WsEnvelope.TYPE))
                    && headers.containsKey(WsEnvelope.ROUTE)) {
                WsEnvelope envelope = WsRegistry.getInstance().get(headers.get(WsEnvelope.ROUTE));
                if (envelope != null) {
                    session = WsRegistry.getInstance().getSession(envelope.route);
                }
            }
            if (WsEnvelope.ENCRYPT.equals(headers.get(WsEnvelope.TYPE)) && session != null
                    && headers.containsKey(WsEnvelope.ENCRYPT)) {
                sessionKey = util.base64ToBytes(headers.get(WsEnvelope.ENCRYPT));
            }

        } else if (body instanceof byte[]) {
            if (session != null && session.isOpen()) {
                byte[] bytes = sessionKey != null ? crypto.aesEncrypt((byte[]) body, sessionKey) : (byte[]) body;
                session.getBasicRemote().sendBinary(ByteBuffer.wrap(bytes));
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
        return null;
    }

}