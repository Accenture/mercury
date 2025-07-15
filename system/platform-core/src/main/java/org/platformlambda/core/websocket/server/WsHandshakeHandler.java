/*

    Copyright 2018-2025 Accenture Technology

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

import io.vertx.core.Handler;
import io.vertx.core.http.ServerWebSocketHandshake;

import java.util.ArrayList;
import java.util.List;

/**
 * This is reserved for system use.
 * DO NOT use this directly in your application code.
 */
public class WsHandshakeHandler implements Handler<ServerWebSocketHandshake> {

    private final List<String> wsPaths;

    public WsHandshakeHandler(List<String> wsPaths) {
        this.wsPaths = new ArrayList<>(wsPaths);
    }

    @Override
    public void handle(ServerWebSocketHandshake ws) {
        String uri = ws.path().trim();
        String path = findPath(uri);
        if (path != null) {
            ws.accept();
        } else {
            ws.reject();
        }
    }

    private String findPath(String path) {
        for (String u: wsPaths) {
            String prefix = u + "/";
            if (path.startsWith(prefix) && !path.equals(prefix)) {
                return u;
            }
        }
        return null;
    }
}
