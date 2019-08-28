/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.lang.services;

import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.platformlambda.core.websocket.common.WsConfigurator;
import org.platformlambda.lang.websocket.server.LanguageConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.ByteArrayInputStream;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

@EventInterceptor
public class LanguageInbox implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(LanguageInbox.class);
    private static final MsgPack msgPack = new MsgPack();

    private static final String TYPE = LanguageConnector.TYPE;
    private static final String EVENT = LanguageConnector.EVENT;
    private static final String BLOCK = LanguageConnector.BLOCK;
    private static final String ID = MultipartPayload.ID;
    private static final String COUNT = MultipartPayload.COUNT;
    private static final String TOTAL = MultipartPayload.TOTAL;
    private static final int OVERHEAD = MultipartPayload.OVERHEAD;

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (body instanceof EventEnvelope) {
           /*
             * The original sender is encoded in the notes field
             * It is in the format: "token_id->reply_to"
             */
            EventEnvelope event = (EventEnvelope) body;
            String sender = event.getExtra();
            if (sender != null && sender.contains("->")) {
                int sep = sender.indexOf("->");
                String token = sender.substring(0, sep);
                String replyTo = sender.substring(sep+2);
                String txPath = LanguageConnector.getTxPathFromToken(token);
                if (txPath != null) {
                    Map<String, Object> response = new HashMap<>();
                    response.put(TYPE, EVENT);
                    EventEnvelope relay = new EventEnvelope();
                    relay.setTo(replyTo);
                    if (event.hasError()) {
                        relay.setStatus(event.getStatus());
                    }
                    relay.setBody(event.getBody());
                    Map<String, String> eventHeaders = event.getHeaders();
                    for (String h: eventHeaders.keySet()) {
                        relay.setHeader(h, eventHeaders.get(h));
                    }
                    if (event.getExecutionTime() > -1) {
                        relay.setExecutionTime(event.getExecutionTime());
                    }
                    if (event.getCorrelationId() != null) {
                        relay.setCorrelationId(event.getCorrelationId());
                    }
                    if (event.getTraceId() != null) {
                        relay.setTrace(event.getCorrelationId(), event.getTracePath());
                    }
                    response.put(EVENT, LanguageConnector.mapFromEvent(relay));

                    byte[] payload = msgPack.pack(response);
                    int maxPayload = WsConfigurator.getInstance().getMaxBinaryPayload() - OVERHEAD;
                    if (payload.length > maxPayload) {
                        int total = (payload.length / maxPayload) + (payload.length % maxPayload == 0 ? 0 : 1);
                        ByteArrayInputStream in = new ByteArrayInputStream(payload);
                        for (int i = 0; i < total; i++) {
                            // To distinguish from a normal payload, the segmented block MUST not have a "TO" value.
                            Map<String, Object> block = new HashMap<>();
                            block.put(TYPE, BLOCK);
                            EventEnvelope inner = new EventEnvelope();
                            inner.setId(event.getId());
                            inner.setHeader(ID, event.getId());
                            inner.setHeader(COUNT, i + 1);
                            inner.setHeader(TOTAL, total);
                            byte[] segment = new byte[maxPayload];
                            int size = in.read(segment);
                            inner.setBody(size == maxPayload ? segment : Arrays.copyOfRange(segment, 0, size));
                            block.put(BLOCK, LanguageConnector.mapFromEvent(inner));
                            PostOffice.getInstance().send(txPath, msgPack.pack(block));
                            log.debug("Sending block {} of {} to {} via {}", i + 1, total, event.getTo(), txPath);
                        }
                    } else {
                        PostOffice.getInstance().send(txPath, payload);
                    }

                } else {
                    log.warn("Event dropped because {} not present", token);
                }
            }
        }
        return null;
    }

}
