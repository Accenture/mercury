/*

    Copyright 2018-2023 Accenture Technology

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

package org.platformlambda.core.services;

import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;

@ZeroTracing
@EventInterceptor
@PreLoad(route="event.api.service", instances=50)
public class EventApiService implements TypedLambdaFunction<EventEnvelope, Void> {
    private static final Logger log = LoggerFactory.getLogger(EventApiService.class);

    private static final String EVENT_API_SERVICE = "event.api.service";
    private static final String STREAM_TO_BYTES = "stream.to.bytes";
    private static final String TYPE = "type";
    private static final String ASYNC = "async";
    private static final String DELIVERED = "delivered";
    private static final String TIME = "time";
    private static final String STREAM = "stream";
    private static final String TIMEOUT = "timeout";
    private static final String CONTENT_TYPE = "content-type";
    private static final String OCTET_STREAM = "application/octet-stream";
    private static final String PLAIN_TEXT = "text/plain";
    private static final String X_TIMEOUT = "X-Timeout";
    private static final String X_ASYNC = "X-Async";
    private static final String MISSING_ROUTING_PATH = "Missing routing path";
    private static final String PRIVATE_FUNCTION = " is private";
    private static final String ROUTE = "Route ";
    private static final String NOT_FOUND = " not found";

    @Override
    public Void handleEvent(Map<String, String> headers, EventEnvelope input, int instance) throws IOException {
        if (input.getRawBody() instanceof Map && input.getReplyTo() != null) {
            Utility util = Utility.getInstance();
            AsyncHttpRequest httpRequest = new AsyncHttpRequest(input.getRawBody());
            long timeout = Math.max(100, util.str2long(httpRequest.getHeader(X_TIMEOUT)));
            boolean async = "true".equals(httpRequest.getHeader(X_ASYNC));
            String streamId = httpRequest.getStreamRoute();
            if (streamId != null) {
                // read the input stream into a byte array using the "stream.to.bytes" function
                EventEmitter po = EventEmitter.getInstance();
                EventEnvelope req = new EventEnvelope().setTo(STREAM_TO_BYTES)
                                                        .setHeader(STREAM, streamId).setHeader(TIMEOUT, timeout);
                po.asyncRequest(req, timeout)
                    .onSuccess(result -> {
                        if (result.getRawBody() instanceof byte[]) {
                            try {
                                handleRequest(headers, instance, (byte[]) result.getRawBody(), input, timeout, async);
                            } catch (Exception e) {
                                sendError(input, 400, e.getMessage());
                            }
                        } else {
                            sendError(input, 500, "Corrupted input stream");
                        }
                    })
                    .onFailure(e -> sendError(input, 408, e.getMessage()));

            } else if (httpRequest.getBody() instanceof byte[]) {
                try {
                    handleRequest(headers, instance, (byte[]) httpRequest.getBody(), input, timeout, async);
                } catch (Exception e) {
                    sendError(input, 400, e.getMessage());
                }
            }
        }
        return null;
    }

    private void handleRequest(Map<String, String> headers, int instance,
                               byte[] requestBody, EventEnvelope input,
                               long timeout, boolean async) throws IOException {
        EventEnvelope request = new EventEnvelope(requestBody);
        PostOffice po = new PostOffice(headers, instance);
        if (request.getTo() != null) {
            if (po.exists(request.getTo())) {
                if (Platform.getInstance().isPrivate(request.getTo())) {
                    sendError(input, 403, request.getTo() + PRIVATE_FUNCTION);
                } else {
                    if (async) {
                        // Drop-n-forget
                        po.send(request);
                        Map<String, Object> ackBody = new HashMap<>();
                        ackBody.put(TYPE, ASYNC);
                        ackBody.put(DELIVERED, true);
                        ackBody.put(TIME, new Date());
                        EventEnvelope pending = new EventEnvelope().setStatus(202).setBody(ackBody);
                        sendResponse(input, pending);
                    } else {
                        // RPC
                        po.asyncRequest(request, timeout)
                                .onSuccess(result -> sendResponse(input, result))
                                .onFailure(e -> sendError(input, 408, e.getMessage()));
                    }
                }
            } else {
                sendError(input, 404, ROUTE + request.getTo() + NOT_FOUND);
            }
        } else {
            sendError(input, 400, MISSING_ROUTING_PATH);
        }
    }

    private void sendResponse(EventEnvelope input, EventEnvelope result) {
        try {
            EventEnvelope response = new EventEnvelope().setTo(input.getReplyTo())
                    .setFrom(EVENT_API_SERVICE)
                    .setCorrelationId(input.getCorrelationId())
                    // binary payload
                    .setHeader(CONTENT_TYPE, OCTET_STREAM)
                    .setBody(result.toBytes())
                    .setTrace(input.getTraceId(), input.getTracePath());
            EventEmitter.getInstance().send(response);
        } catch (IOException e) {
            log.error("Unable to send response {} -> {} - {}", EVENT_API_SERVICE, input.getReplyTo(), e.getMessage());
        }
    }

    private void sendError(EventEnvelope input, int status, String error) {
        try {
            EventEnvelope response = new EventEnvelope().setTo(input.getReplyTo())
                    .setFrom(EVENT_API_SERVICE)
                    .setCorrelationId(input.getCorrelationId())
                    // text payload
                    .setHeader(CONTENT_TYPE, PLAIN_TEXT)
                    .setStatus(status)
                    .setBody(error)
                    .setTrace(input.getTraceId(), input.getTracePath());
            EventEmitter.getInstance().send(response);
        } catch (IOException e) {
            log.error("Unable to send error {} -> {} - {}", EVENT_API_SERVICE, input.getReplyTo(), e.getMessage());
        }
    }

}
