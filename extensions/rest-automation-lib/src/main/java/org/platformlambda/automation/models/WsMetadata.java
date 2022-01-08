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

package org.platformlambda.automation.models;

import io.vertx.core.http.ServerWebSocket;

public class WsMetadata {

    public String application, recipient, session;
    public Boolean publish, subscribe;
    public ServerWebSocket ws;
    public long lastAccess = System.currentTimeMillis();

    public WsMetadata(ServerWebSocket ws,
                      String application, String recipient, String session, boolean publish, boolean subscribe) {
        this.ws = ws;
        this.application = application;
        this.recipient = recipient;
        this.session = session;
        this.publish = publish;
        this.subscribe = subscribe;
    }

    public void touch() {
        lastAccess = System.currentTimeMillis();
    }
}
