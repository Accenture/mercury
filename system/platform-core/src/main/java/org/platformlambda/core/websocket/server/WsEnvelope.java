/*

    Copyright 2018-2024 Accenture Technology

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

import io.vertx.core.http.ServerWebSocket;

public class WsEnvelope {

    public static final String TYPE = "type";
    public static final String OPEN = "open";
    public static final String CLOSE = "close";
    public static final String BYTES = "bytes";
    public static final String STRING = "string";
    public static final String MAP = "map";
    public static final String ROUTE = "route";
    public static final String TX_PATH = "tx_path";
    public static final String IP = "ip";
    public static final String PATH = "path";
    public static final String QUERY = "query";
    public static final String TOKEN = "token";
    public static final String CLOSE_CODE = "close_code";
    public static final String CLOSE_REASON = "close_reason";
    private final String path, rxPath, txPath;
    private final ServerWebSocket ws;
    private long lastAccess = System.currentTimeMillis();

    public WsEnvelope(ServerWebSocket ws, String path, String rxPath, String txPath) {
        this.ws = ws;
        this.path = path;
        this.rxPath = rxPath;
        this.txPath = txPath;
    }

    public void touch() {
        lastAccess = System.currentTimeMillis();
    }

    public long getLastAccess() {
        return lastAccess;
    }

    public String getPath() {
        return path;
    }

    public String getRxPath() {
        return rxPath;
    }

    public String getTxPath() {
        return txPath;
    }

    public ServerWebSocket getWebSocket() {
        return ws;
    }

}
