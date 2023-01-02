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

package org.platformlambda.lang.services;

import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.platformlambda.lang.websocket.server.LanguageConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

@EventInterceptor
public class LanguageInbox implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(LanguageInbox.class);
    private static final MsgPack msgPack = new MsgPack();

    private static final String TYPE = LanguageConnector.TYPE;
    private static final String EVENT = LanguageConnector.EVENT;
    private static final String ROUTING = "routing";
    private static final String BLOCK = LanguageConnector.BLOCK;
    private static final String ID = MultipartPayload.ID;
    private static final String COUNT = MultipartPayload.COUNT;
    private static final String TOTAL = MultipartPayload.TOTAL;
    private static final int OVERHEAD = MultipartPayload.OVERHEAD;
    private static final int MAX_PAYLOAD_SIZE = 64 * 1024 - OVERHEAD;

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        try {
            process((EventEnvelope) body);
        } catch (Exception e) {
            log.warn("Unable to delivery event to language pack - {}", e.getMessage());
        }
        return null;
    }

    public void process(EventEnvelope event) throws IOException {
        /*
         * The original sender is encoded in the extra field as a routing tag.
         * It's value is in the format: "token_id->reply_to".
         *
         * This extra routing tag can be removed after processing.
         */
        String sender = event.getTag(ROUTING);
        if (sender != null && sender.contains("->")) {
            int sep = sender.indexOf("->");
            String token = sender.substring(0, sep);
            String replyTo = sender.substring(sep+2);
            String txPath = LanguageConnector.getTxPathFromToken(token);
            if (txPath != null) {
                EventEnvelope relay = new EventEnvelope();
                relay.setTo(replyTo);
                if (event.hasError()) {
                    relay.setStatus(event.getStatus());
                }
                relay.setBody(event.getBody());
                relay.setHeaders(event.getHeaders());
                if (event.getExecutionTime() > -1) {
                    relay.setExecutionTime(event.getExecutionTime());
                }
                if (event.getCorrelationId() != null) {
                    relay.setCorrelationId(event.getCorrelationId());
                }
                if (event.getTraceId() != null) {
                    relay.setTrace(event.getCorrelationId(), event.getTracePath());
                }
                // remove routing tag and forward remaining tags if any
                event.removeTag(ROUTING);
                if (event.getExtra() != null) {
                    relay.setExtra(event.getExtra());
                }
                byte[] payload = msgPack.pack(LanguageConnector.mapFromEvent(relay));
                if (payload.length > MAX_PAYLOAD_SIZE) {
                    int total = (payload.length / MAX_PAYLOAD_SIZE) + (payload.length % MAX_PAYLOAD_SIZE == 0 ? 0 : 1);
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
                        byte[] segment = new byte[MAX_PAYLOAD_SIZE];
                        int size = in.read(segment);
                        inner.setBody(size == MAX_PAYLOAD_SIZE ? segment : Arrays.copyOfRange(segment, 0, size));
                        block.put(BLOCK, LanguageConnector.mapFromEvent(inner));
                        PostOffice.getInstance().send(txPath, msgPack.pack(block));
                        log.debug("Sending block {} of {} to {} via {}", i + 1, total, event.getTo(), txPath);
                    }
                } else {
                    Map<String, Object> response = new HashMap<>();
                    response.put(TYPE, EVENT);
                    response.put(EVENT, payload);
                    PostOffice.getInstance().send(txPath, msgPack.pack(response));
                }

            } else {
                log.warn("Event dropped because {} not present", token);
            }
        }
    }

}
