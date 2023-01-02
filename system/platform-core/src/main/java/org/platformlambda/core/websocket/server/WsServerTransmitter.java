/*

    Copyright 2018-2023 Accenture Technology

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

package org.platformlambda.core.websocket.server;

import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.ServerWebSocket;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;

import java.util.Map;

/**
 * Outgoing message handler
 */
@ZeroTracing
public class WsServerTransmitter implements LambdaFunction {

    private final ServerWebSocket ws;
    private static final String STATUS = "status";
    private static final String MESSAGE = "message";
    private boolean connected = true;

    public WsServerTransmitter(ServerWebSocket ws) {
        this.ws = ws;
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (connected && !ws.isClosed()) {
            if (WsEnvelope.CLOSE.equals(headers.get(WsEnvelope.TYPE))) {
                connected = false;
                int status = Utility.getInstance().str2int(headers.get(STATUS));
                String message = headers.get(MESSAGE) == null? "bye" : headers.get(MESSAGE);
                ws.close((short) (status >= 0? status : 1000), message);
            } else {
                if (body instanceof byte[]) {
                    ws.writeBinaryMessage(Buffer.buffer((byte[]) body));
                }
                if (body instanceof String) {
                    ws.writeTextMessage((String) body);
                }
                if (body instanceof Map) {
                    ws.writeTextMessage(SimpleMapper.getInstance().getMapper().writeValueAsString(body));
                }
            }
            return true;
        } else {
            return false;
        }
    }
}
