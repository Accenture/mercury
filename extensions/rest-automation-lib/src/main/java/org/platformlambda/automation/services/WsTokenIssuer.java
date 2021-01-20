package org.platformlambda.automation.services;

import org.platformlambda.automation.MainModule;
import org.platformlambda.automation.config.WsEntry;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
/*

    Copyright 2018-2021 Accenture Technology

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

import org.platformlambda.core.util.Utility;

import java.util.Date;
import java.util.HashMap;
import java.util.Map;

public class WsTokenIssuer implements LambdaFunction {

    private static final long EXPIRY_TIMER = 30000;
    private static final String TYPE = "type";
    private static final String TOKEN = "token";
    private static final String EXPIRY = "expiry";
    private static final String TIME = "time";
    private static final String APPLICATION = "application";

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (body instanceof Map) {
            AsyncHttpRequest request = new AsyncHttpRequest(body);
            if (request.getUrl() != null) {
                String app = request.getPathParameter(APPLICATION);
                if (app == null) {
                    throw new IllegalArgumentException("Missing path parameter '"+APPLICATION+"'");
                }
                WsEntry wsEntry = WsEntry.getInstance();
                if (wsEntry.getInfo(app) == null) {
                    throw new IllegalArgumentException("Websocket application ("+app+") not found");
                }
                PostOffice po = PostOffice.getInstance();
                Utility util = Utility.getInstance();
                Map<String, Object> result = new HashMap<>();
                String token = util.getUuid();
                result.put(TOKEN, token);
                result.put(APPLICATION, app);
                result.put(TIME, new Date());
                result.put(EXPIRY, new Date(System.currentTimeMillis() + EXPIRY_TIMER));
                po.broadcast(MainModule.NOTIFICATION_MANAGER, new Kv(TYPE, TOKEN),
                        new Kv(TOKEN, token), new Kv(APPLICATION, app));
                return result;
            }
        }
        throw new IllegalArgumentException("Invalid HTTP request");
    }

}
