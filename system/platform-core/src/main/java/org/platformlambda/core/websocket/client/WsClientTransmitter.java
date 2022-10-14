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

package org.platformlambda.core.websocket.client;

import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.WebSocket;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;

import java.util.Map;

public class WsClientTransmitter implements LambdaFunction {

    private final WebSocket ws;
    private boolean connected = true;

    public WsClientTransmitter(WebSocket ws) {
        this.ws = ws;
    }

    public void close() {
        connected = false;
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (connected && !ws.isClosed()) {
            if (body instanceof byte[]) {
                ws.writeBinaryMessage(Buffer.buffer((byte[]) body));
            }
            if (body instanceof String) {
                ws.writeTextMessage((String) body);
            }
            if (body instanceof Map) {
                ws.writeTextMessage(SimpleMapper.getInstance().getMapper().writeValueAsString(body));
            }
            return true;
        } else {
            return false;
        }
    }
}
