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

import io.vertx.core.http.ServerWebSocket;

/**
 * This is reserved for system use.
 * DO NOT use this directly in your application code.
 */
public class WsEnvelope {
    public static final String TYPE = "type";
    public static final String OPEN = "open";
    public static final String CLOSE = "close";
    public static final String BYTES = "bytes";
    public static final String STRING = "string";
    public static final String ROUTE = "route";
    public static final String TX_PATH = "tx_path";
    public static final String IP = "ip";
    public static final String PATH = "path";
    public static final String QUERY = "query";
    public static final String TOKEN = "token";
    public static final String CLOSE_CODE = "close_code";
    public static final String CLOSE_REASON = "close_reason";
    private final String uriPath;
    private final String rxPath;
    private final String txPath;
    private final ServerWebSocket ws;
    private long lastAccess = System.currentTimeMillis();

    public WsEnvelope(ServerWebSocket ws, String uriPath, String rxPath, String txPath) {
        this.ws = ws;
        this.uriPath = uriPath;
        this.rxPath = rxPath;
        this.txPath = txPath;
    }

    public void touch() {
        lastAccess = System.currentTimeMillis();
    }

    public long getLastAccess() {
        return lastAccess;
    }

    public String getUriPath() {
        return uriPath;
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
