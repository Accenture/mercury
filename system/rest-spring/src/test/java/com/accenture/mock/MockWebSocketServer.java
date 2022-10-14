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

package com.accenture.mock;

import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.websocket.server.WsEnvelope;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;

@WebSocketService("mock")
public class MockWebSocketServer implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(MockWebSocketServer.class);


    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        PostOffice po = PostOffice.getInstance();
        String route, token, txPath;

        if (headers.containsKey(WsEnvelope.TYPE)) {
            switch (headers.get(WsEnvelope.TYPE)) {
                case WsEnvelope.OPEN:
                    // the open event contains route, txPath, ip, path, query and token
                    route = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    token = headers.get(WsEnvelope.TOKEN);
                    String ip = headers.get(WsEnvelope.IP);
                    String path = headers.get(WsEnvelope.PATH);
                    String query = headers.get(WsEnvelope.QUERY);
                    log.info("Started {}, {}, ip={}, path={}, query={}, token={}", route, txPath, ip, path, query, token);
                    break;
                case WsEnvelope.CLOSE:
                    // the close event contains route and token for this websocket
                    route = headers.get(WsEnvelope.ROUTE);
                    token = headers.get(WsEnvelope.TOKEN);
                    log.info("Stopped {}, token={}", route, token);
                    break;
                case WsEnvelope.BYTES:
                    // the data event for byteArray payload contains route and txPath
                    route = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    byte[] payload = (byte[]) body;
                    // echo the byte payload
                    po.send(txPath, payload);
                    log.info("{} got {} bytes", route, payload.length);
                    break;
                case WsEnvelope.STRING:
                    // the data event for string payload contains route and txPath
                    route = headers.get(WsEnvelope.ROUTE);
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    String message = (String) body;
                    // just echo the message
                    po.send(txPath, message);
                    log.info("{} received: {}", route, message);
                    break;
                default:
                    // this should not happen
                    log.error("Invalid event {} {}", headers, body);
                    break;
            }
        }
        // nothing to return because this is asynchronous
        return null;
    }
}
