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
package org.platformlambda.core.websocket.common;

import akka.actor.ActorRef;
import org.platformlambda.core.models.EventBlocks;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.SimpleCache;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.text.NumberFormat;
import java.util.Arrays;
import java.util.Map;

public class MultipartPayload {
    private static final Logger log = LoggerFactory.getLogger(MultipartPayload.class);
    public static final String ID = "id";
    public static final String COUNT = "count";
    public static final String TOTAL = "total";
    public static final String TO = "to";
    public static final String BROADCAST = "broadcast";
    public static final int OVERHEAD = 256;
    private static final SimpleCache cache = SimpleCache.createCache("payload.segmentation", 60000);
    private static final int MAX_PAYLOAD = WsConfigurator.getInstance().getMaxBinaryPayload() - OVERHEAD;
    private static final MultipartPayload instance = new MultipartPayload();

    private MultipartPayload() {
        log.info("Automatic segmentation when event payload exceeds {}", NumberFormat.getInstance().format(MAX_PAYLOAD));
    }

    public static MultipartPayload getInstance() {
        return instance;
    }

    public void incoming(EventEnvelope message) throws IOException {
        Map<String, String> control = message.getHeaders();
        if (message.getTo() != null) {
            // normal event
            PostOffice.getInstance().send(message);
        } else if (control.size() == 3 && control.containsKey(ID)
                && control.containsKey(COUNT) && control.containsKey(TOTAL)) {
            // segmented incoming event
            Utility util = Utility.getInstance();
            String id = control.get(ID);
            int count = util.str2int(control.get(COUNT));
            int total = util.str2int(control.get(TOTAL));
            if (message.getBody() instanceof byte[] && count != -1 && total != -1 && count <= total) {
                byte[] data = (byte[]) message.getBody();
                log.debug("Receiving block {} of {} as {} - {} bytes", count, total, id, data.length);
                Object o = cache.get(id);
                EventBlocks segments = o instanceof EventBlocks ? (EventBlocks) o : new EventBlocks(id, total);
                if (segments.exists(count)) {
                    log.error("Duplicated block {} for event {} dropped", count, id);
                } else {
                    segments.put(count, data);
                    if (total == segments.size()) {
                        EventEnvelope reconstructed = new EventEnvelope();
                        reconstructed.load(segments.toBytes());
                        cache.remove(id);
                        PostOffice.getInstance().send(reconstructed);
                    } else {
                        cache.put(id, segments);
                    }
                }
            }
        }
    }

    public void outgoing(String dest, EventEnvelope event) throws IOException {
        Platform platform = Platform.getInstance();
        if (platform.hasRoute(dest)) {
            outgoing(platform.getManager(dest), event);
        }
    }

    public void outgoing(ActorRef dest, EventEnvelope event) throws IOException {
        if (dest != null && event != null) {
            event.setEndOfRoute();
            byte[] payload = event.toBytes();
            if (payload.length > MAX_PAYLOAD) {
                int total = (payload.length / MAX_PAYLOAD) + (payload.length % MAX_PAYLOAD == 0 ? 0 : 1);
                ByteArrayInputStream in = new ByteArrayInputStream(payload);
                for (int i = 0; i < total; i++) {
                    // To distinguish from a normal payload, the segmented block MUST not have a "TO" value.
                    int count = i + 1;
                    EventEnvelope blk = new EventEnvelope()
                                            .setHeader(MultipartPayload.ID, event.getId())
                                            .setHeader(MultipartPayload.COUNT, count)
                                            .setHeader(MultipartPayload.TOTAL, total);
                    byte[] segment = new byte[MAX_PAYLOAD];
                    int size = in.read(segment);
                    blk.setBody(size == MAX_PAYLOAD ? segment : Arrays.copyOfRange(segment, 0, size));
                    /*
                     * To guarantee that the cloud connector can deliver blocks of the same event
                     * to the same destination, we pass id, count and total as the headers
                     */
                    EventEnvelope out = new EventEnvelope()
                                            .setHeader(TO, event.getTo())
                                            .setHeader(MultipartPayload.ID, event.getId())
                                            .setHeader(MultipartPayload.COUNT, count)
                                            .setHeader(MultipartPayload.TOTAL, total)
                                            .setBody(blk.toBytes());

                    if (event.getBroadcastLevel() > 1) {
                        // tell a cloud connector that this event should be broadcast
                        out.setHeader(BROADCAST, "1");
                    }
                    dest.tell(out, ActorRef.noSender());
                    log.debug("Sending block {} of {} to {} - {} bytes", i + 1, total, event.getTo(), size);
                }

            } else {
                EventEnvelope out = new EventEnvelope().setHeader(TO, event.getTo()).setBody(payload);
                if (event.getBroadcastLevel() > 1) {
                    // tell a cloud connector that this event should be broadcast
                    out.setHeader(BROADCAST, "1");
                }
                dest.tell(out, ActorRef.noSender());
            }
        }
    }

}
