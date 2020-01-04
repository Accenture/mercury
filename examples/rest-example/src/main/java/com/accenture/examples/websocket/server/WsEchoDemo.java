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

package com.accenture.examples.websocket.server;

import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.system.PostOffice;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;

@WebSocketService("hello")
public class WsEchoDemo implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(WsEchoDemo.class);

    /**
     * This is an example for a websocket lambda function
     *
     * If you want to close the websocket connection for whatever reason, you may issue:
     * Utility.getInstance().closeConnection(txPath, reasonCode, message);
     *
     * @param headers contains routing information of a websocket connection
     * @param body can be string or byte array as per websocket specification
     * @param instance is always 1 because the service is a singleton for each websocket connection
     * @return null because there is nothing to return
     * @throws IOException in case there are errors in routing
     */
    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {

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
                    // just tell the browser that I have received the bytes
                    po.send(txPath, "received " + payload.length + " bytes");
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