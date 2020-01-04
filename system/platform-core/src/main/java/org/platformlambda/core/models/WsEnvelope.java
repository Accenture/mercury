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

package org.platformlambda.core.models;

import org.platformlambda.core.util.Utility;

import javax.websocket.Session;
import java.util.List;

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
    public static final String ENCRYPT = "encrypt";
    public static final String CLOSE_CODE = "close_code";
    public static final String CLOSE_REASON = "close_reason";

    public String route, txPath, ip, path, query, origin;
    public Session session;
    public byte[] sessionKey, publicKey;
    public boolean encrypt = false;

    public WsEnvelope(String route, String txPath, String ip, String path, String query) {
        this.route = route;
        this.txPath = txPath;
        this.ip = ip;
        this.path = path;
        this.query = query;
        // this assumes origin is the last element in the URL path
        List<String> elements = Utility.getInstance().split(path, "/");
        /*
         * For event node, the last segment is the origin of the calling lambda application.
         * For user websockets, this is usually used as the access token.
         */
        this.origin = elements.get(elements.size()-1);
    }

}
