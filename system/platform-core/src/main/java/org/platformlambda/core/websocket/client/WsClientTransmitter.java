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

package org.platformlambda.core.websocket.client;

import io.vertx.core.buffer.Buffer;
import io.vertx.core.http.WebSocket;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;

import java.util.Map;

@ZeroTracing
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
    public Object handleEvent(Map<String, String> headers, Object input, int instance) throws Exception {
        if (connected && !ws.isClosed()) {
            if (input instanceof byte[]) {
                ws.writeBinaryMessage(Buffer.buffer((byte[]) input));
            }
            if (input instanceof String) {
                ws.writeTextMessage((String) input);
            }
            if (input instanceof Map) {
                ws.writeTextMessage(SimpleMapper.getInstance().getMapper().writeValueAsString(input));
            }
            return true;
        } else {
            return false;
        }
    }
}
