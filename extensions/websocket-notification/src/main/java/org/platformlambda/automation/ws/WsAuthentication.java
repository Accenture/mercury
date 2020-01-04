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

package org.platformlambda.automation.ws;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;

public class WsAuthentication implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(WsAuthentication.class);

    private static final String X_USER = "x-user"; // this must be lower case
    private static final String TYPE = WsEnvelope.TYPE;
    private static final String ROUTE = WsEnvelope.ROUTE;
    private static final String TX_PATH = WsEnvelope.TX_PATH;
    private static final String AUTHENTICATION  = "authentication";
    private static final String TOKEN = WsEnvelope.TOKEN;
    private static final String APPLICATION = "application";
    private static final String IP = WsEnvelope.IP;
    private static final String QUERY = WsEnvelope.QUERY;
    private static final String ID = "id";

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws AppException {
        /*
         * Sample input:
         * headers = {route=api.in.50344.2, tx_path=api.out.50344.2, type=authentication}
         * body = {application=notification, query={id=some_user}, ip=127.0.0.1, token=some_value}
         */
        log.info("Authenticate {} {}", headers, body);
        if (body instanceof Map && AUTHENTICATION.equals(headers.get(TYPE)) && headers.containsKey(ROUTE) &&
                headers.containsKey(TX_PATH)) {
            Map<String, Object> data = (Map<String, Object>) body;
            if (data.containsKey(APPLICATION) && data.containsKey(IP) &&
                    data.containsKey(QUERY) && data.containsKey(TOKEN)) {
                String user = getAuthenticatedUser((String) data.get(TOKEN), (Map<String, String>) data.get(QUERY));
                if (user == null) {
                    // this will reject the request without a reason
                    return false;
                } else {
                    return new EventEnvelope().setHeader(X_USER, user).setBody(true);
                }
            }
        }
        return false;
    }

    private String getAuthenticatedUser(String token, Map<String, String> query) throws AppException {
        /*
         * TODO: insert your authentication logic here
         */
        if ("demo-token".equals(token)) {
            if (query.containsKey(ID)) {
                return query.get(ID);
            } else {
                throw new AppException(401, "Missing id in query parameters");
            }

        } else {
            return null;
        }
    }

}
