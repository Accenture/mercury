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
import org.platformlambda.lang.websocket.server.LanguageConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;

@EventInterceptor
public class LanguageInbox implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(LanguageInbox.class);
    private static final MsgPack msgPack = new MsgPack();

    private static final String TYPE = LanguageConnector.TYPE;
    private static final String EVENT = LanguageConnector.EVENT;

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (body instanceof EventEnvelope) {
           /*
             * The original sender is encoded in the notes field
             * It is in the format: "token_id->reply_to"
             */
            EventEnvelope event = (EventEnvelope) body;
            String sender = event.getNotes();
            if (sender.contains("->")) {
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
                    if (event.getCorrelationId() != null) {
                        relay.setCorrelationId(event.getCorrelationId());
                    }
                    response.put(EVENT, LanguageConnector.mapFromEvent(relay));
                    PostOffice.getInstance().send(txPath, msgPack.pack(response));
                } else {
                    log.warn("Event dropped because {} not present", token);
                }
            }
        }
        return null;
    }

}
