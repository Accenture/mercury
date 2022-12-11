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

package org.platformlambda.automation.services;

import org.platformlambda.automation.models.HttpRequestEvent;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;

@ZeroTracing
@EventInterceptor
public class AuthInterceptor implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(AuthInterceptor.class);

    private static final String HTTP_REQUEST = "http.request";
    private static final String ASYNC_HTTP_RESPONSE = AppStarter.ASYNC_HTTP_RESPONSE;

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        if (body instanceof EventEnvelope) {
            EventEnvelope input = (EventEnvelope) body;
            PostOffice po = PostOffice.getInstance();
            HttpRequestEvent evt = new HttpRequestEvent(input.getBody());
            if (evt.authService != null && evt.requestId != null && !evt.httpRequest.isEmpty()) {
                AsyncHttpRequest req = new AsyncHttpRequest(evt.httpRequest);
                EventEnvelope authRequest = new EventEnvelope();
                // the AsyncHttpRequest is sent as a map
                authRequest.setTo(evt.authService).setBody(evt.httpRequest);
                // distributed tracing required?
                if (evt.tracing) {
                    authRequest.setFrom(HTTP_REQUEST);
                    authRequest.setTrace(evt.traceId, evt.tracePath);
                }
                po.asyncRequest(authRequest, evt.timeout)
                        .onSuccess(response -> {
                            if (Boolean.TRUE.equals(response.getBody())) {
                                /*
                                 * Upon successful authentication,
                                 * the authentication service may save session information as headers
                                 * (auth headers are converted to lower case for case insensitivity)
                                 */
                                Map<String, String> authResHeaders = response.getHeaders();
                                for (Map.Entry<String, String> entry : authResHeaders.entrySet()) {
                                    req.setSessionInfo(entry.getKey(), entry.getValue());
                                }
                                // forward request to target service(s)
                                EventEnvelope event = new EventEnvelope();
                                event.setTo(evt.primary).setBody(req)
                                        .setCorrelationId(evt.requestId)
                                        .setReplyTo(ASYNC_HTTP_RESPONSE + "@" + Platform.getInstance().getOrigin());
                                // enable distributed tracing if needed
                                if (evt.tracing) {
                                    event.setFrom(evt.authService);
                                    event.setTrace(evt.traceId, evt.tracePath);
                                }
                                try {
                                    po.send(event);
                                    // copying to secondary services if any
                                    if (evt.services.size() > 1) {
                                        for (String secondary : evt.services) {
                                            if (!secondary.equals(evt.primary)) {
                                                EventEnvelope copy = new EventEnvelope()
                                                                            .setTo(secondary).setBody(evt.httpRequest);
                                                if (evt.tracing) {
                                                    copy.setFrom(HTTP_REQUEST);
                                                    copy.setTrace(evt.traceId, evt.tracePath);
                                                }
                                                sendToSecondaryTarget(copy);
                                            }
                                        }
                                    }
                                } catch (IOException e) {
                                    sendError(evt, 400, e.getMessage());
                                }
                            } else {
                                sendError(evt, 401, "Unauthorized");
                            }
                        })
                        .onFailure(e -> sendError(evt, 408, e.getMessage()));
            }
        }
        return null;
    }

    private void sendError(HttpRequestEvent evt, int status, String message) {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope event = new EventEnvelope();
        event.setTo(ASYNC_HTTP_RESPONSE).setCorrelationId(evt.requestId).setStatus(status).setBody(message);
        // enable distributed tracing if needed
        if (evt.tracing) {
            event.setFrom(evt.authService);
            event.setTrace(evt.traceId, evt.tracePath);
        }
        try {
            po.send(event);
        } catch (IOException e) {
            log.error("Unable to send error to {} - {}", ASYNC_HTTP_RESPONSE, e.getMessage());
        }
    }

    private void sendToSecondaryTarget(EventEnvelope event) {
        try {
            PostOffice.getInstance().send(event);
        } catch (Exception e) {
            log.warn("Unable to copy event to {} - {}", event.getTo(), e.getMessage());
        }
    }

}
