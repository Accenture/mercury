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
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.lang.websocket.server.LanguageConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@EventInterceptor
public class LanguageRelay implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(LanguageRelay.class);
    private static final MsgPack msgPack = new MsgPack();
    private static final CryptoApi crypto = new CryptoApi();

    private static final String TYPE = LanguageConnector.TYPE;
    private static final String EVENT = LanguageConnector.EVENT;

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (body instanceof EventEnvelope) {
            EventEnvelope event = (EventEnvelope) body;
            String to = event.getTo();
            if (to != null) {
                List<String> targets = LanguageConnector.getDestinations(to);
                if (!targets.isEmpty()) {
                    if (event.getBroadcastLevel() == 2) {
                        // level-2 broadcast - send it to all connected language pack clients
                        for (String token: targets) {
                            send(token, event);
                        }
                    } else {
                        // select the next available client
                        String token = getNextAvailable(targets);
                        send(token, event);
                    }

                } else {
                    log.warn("Event dropped because {} does not have any connections", to);
                }
            }

        }
        return null;
    }

    private void send(String token, EventEnvelope event) throws IOException {
        String txPath = LanguageConnector.getTxPathFromToken(token);
        if (txPath != null) {
            if (event.getBroadcastLevel() == 2) {
                event.setBroadcastLevel(3);
            }
            // relay the event
            Map<String, Object> response = new HashMap<>();
            response.put(TYPE, EVENT);
            response.put(EVENT, LanguageConnector.mapFromEvent(event));
            PostOffice.getInstance().send(txPath, msgPack.pack(response));
        } else {
            log.warn("Event dropped because {} not present", token);
        }
    }

    private String getNextAvailable(List<String> targets) {
        int i = targets.size() == 1? 0 : crypto.nextInt(targets.size());
        return targets.get(i);
    }

}
