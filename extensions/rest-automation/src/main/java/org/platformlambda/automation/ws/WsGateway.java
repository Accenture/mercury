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

import org.platformlambda.automation.config.WsEntry;
import org.platformlambda.automation.models.WsInfo;
import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.io.IOException;
import java.io.UnsupportedEncodingException;
import java.net.URLDecoder;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

@WebSocketService("api")
public class WsGateway implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(WsGateway.class);

    private static final String TYPE = WsEnvelope.TYPE;
    private static final String OPEN = WsEnvelope.OPEN;
    private static final String CLOSE = WsEnvelope.CLOSE;
    private static final String ROUTE = WsEnvelope.ROUTE;
    private static final String IP = WsEnvelope.IP;
    private static final String TX_PATH = WsEnvelope.TX_PATH;
    private static final String QUERY = WsEnvelope.QUERY;
    private static final String BYTES = WsEnvelope.BYTES;
    private static final String STRING = WsEnvelope.STRING;
    private static final String TEXT = "text";
    private static final String TOKEN = WsEnvelope.TOKEN;
    private static final String AUTHENTICATION = "authentication";
    private static final String APPLICATION = "application";
    // route -> info
    private static final ConcurrentMap<String, WsInfo> route2WsInfo = new ConcurrentHashMap<>();

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (headers.containsKey(TYPE)) {
            String type = headers.get(TYPE);
            if (OPEN.equals(type)) {
                handleOpen(headers);
            }
            if (CLOSE.equals(type)) {
                handleClose(headers);
            }
            if (STRING.equals(type) || BYTES.equals(type)) {
                handleMessage(headers, body);
            }
        }
        // nothing to return because this is asynchronous
        return null;
    }

    private void handleOpen(Map<String, String> headers) throws IOException {
        String origin = Platform.getInstance().getOrigin();
        Utility util = Utility.getInstance();
        PostOffice po = PostOffice.getInstance();
        WsEntry wsEntry = WsEntry.getInstance();
        if (wsEntry.isEmpty()) {
            util.closeConnection(headers.get(TX_PATH), CloseReason.CloseCodes.CANNOT_ACCEPT,
                    "WebSocket service not enabled");
            return;
        }
        // the open event contains route, txPath, ip, path, query and token
        String ip = headers.get(IP);
        String identifier = headers.get(TOKEN);
        int colon = identifier.indexOf(':');
        if (colon == -1) {
            util.closeConnection(headers.get(TX_PATH), CloseReason.CloseCodes.CANNOT_ACCEPT,
                    "URL must end with {application}:{session_id}");
            return;
        }
        String app = identifier.substring(0, colon);
        WsInfo wsInfo = wsEntry.getRoute(app);
        if (wsInfo == null) {
            util.closeConnection(headers.get(TX_PATH), CloseReason.CloseCodes.CANNOT_ACCEPT,
                    "WebSocket application "+app+" not found");
            return;
        }
        String token = identifier.substring(colon+1);
        Map<String, Object> request = new HashMap<>();
        Map<String, String> parameters = getQueryParameters(headers.get(QUERY));
        request.put(QUERY, parameters);
        request.put(APPLICATION, app);
        request.put(TOKEN, token);
        request.put(IP, ip);
        if (!po.exists(wsInfo.authService, wsInfo.userService)) {
            util.closeConnection(headers.get(TX_PATH), CloseReason.CloseCodes.CANNOT_ACCEPT,
                    "Service "+wsInfo.authService+" or "+wsInfo.userService+" not reachable");
            return;
        }
        String authError = null;
        Map<String, String> authResponseHeaders = new HashMap<>();
        try {
            EventEnvelope authResult = po.request(wsInfo.authService, 5000, request,
                    new Kv(TYPE, AUTHENTICATION),
                    new Kv(ROUTE, headers.get(ROUTE)), new Kv(TX_PATH, headers.get(TX_PATH)));
            if (Boolean.TRUE.equals(authResult.getBody())) {
                authResponseHeaders = authResult.getHeaders();
            } else {
                authError = "Unauthorized";
            }
        } catch (IOException | TimeoutException e) {
            authError = e.getMessage();
            log.error("WebSocket authentication - {}", authError);
        } catch (AppException e) {
            authError = e.getMessage();
        }
        if (authError != null) {
            util.closeConnection(headers.get(TX_PATH), CloseReason.CloseCodes.CANNOT_ACCEPT, authError);
            return;
        }
        // remove credentials from request parameters before sending to the user service
        request.remove(TOKEN);
        // save routing information
        route2WsInfo.put(headers.get(ROUTE), wsInfo);
        EventEnvelope forward = new EventEnvelope();
        forward.setTo(wsInfo.userService).setBody(request).setHeader(TYPE, OPEN);
        forward.setHeader(ROUTE, headers.get(ROUTE)+"@"+origin);
        forward.setHeader(TX_PATH, headers.get(TX_PATH)+"@"+origin);
        /*
         * Upon successful authentication, the user-defined authentication service should insert
         * relevant session metadata. e.g. user ID.
         *
         * The user service must understand the session metadata that the user-defined authentication
         * service provides. e.g. authenticated user ID may be provided in the "X-User" header.
         */
        for (String h: authResponseHeaders.keySet()) {
            // for consistency with HTTP headers, these user defined headers are also case insensitive.
            forward.setHeader(h.toLowerCase(), authResponseHeaders.get(h));
        }
        po.send(forward);
        log.info("Started {}, {}, {}", headers.get(ROUTE), ip, app);
    }

    private void handleClose(Map<String, String> headers) throws IOException {
        forwardEvent(headers, null);
        route2WsInfo.remove(headers.get(ROUTE));
        log.info("Stopping {}", headers.get(ROUTE));
    }

    private void handleMessage(Map<String, String> headers, Object body) throws IOException {
        Utility util = Utility.getInstance();
        if (!forwardEvent(headers, body)) {
            WsInfo service = route2WsInfo.get(headers.get(ROUTE));
            if (service != null) {
                util.closeConnection(headers.get(TX_PATH),
                        CloseReason.CloseCodes.GOING_AWAY, service.userService+" offline");
            }
        }
    }

    private boolean forwardEvent(Map<String, String> headers, Object body) throws IOException {
        String origin = Platform.getInstance().getOrigin();
        WsInfo forward = route2WsInfo.get(headers.get(ROUTE));
        if (forward != null) {
            String fullRoute = headers.get(ROUTE)+"@"+origin;
            String fullTxPath = headers.get(TX_PATH)+"@"+origin;
            PostOffice po = PostOffice.getInstance();
            // check if the target service is available
            if (po.exists(forward.userService)) {
                if (CLOSE.equals(headers.get(TYPE))) {
                    PostOffice.getInstance().send(forward.userService,
                            new Kv(TYPE, headers.get(TYPE)), new Kv(ROUTE, fullRoute));
                } else if (body != null) {
                    PostOffice.getInstance().send(forward.userService, body,
                            new Kv(TYPE, BYTES.equals(headers.get(TYPE)) ? BYTES : TEXT),
                            new Kv(ROUTE, fullRoute), new Kv(TX_PATH, fullTxPath));
                }
                return true;
            }
        }
        return false;
    }

    private Map<String, String> getQueryParameters(String query) throws UnsupportedEncodingException {
        Map<String, String> result = new HashMap<>();
        Utility util = Utility.getInstance();
        List<String> parts = util.split(query, "&");
        for (String p: parts) {
            int sep = p.indexOf('=');
            if (sep > 0) {
                String key = URLDecoder.decode(p.substring(0, sep), "UTF-8");
                String value = URLDecoder.decode(p.substring(sep+1), "UTF-8");
                result.put(key, value);
            } else {
                result.put(URLDecoder.decode(p, "UTF-8"), "");
            }
        }
        return result;
    }

}
