/*

    Copyright 2018-2024 Accenture Technology

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

import io.vertx.core.eventbus.EventBus;
import org.platformlambda.core.models.EventBlocks;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.ServiceQueue;
import org.platformlambda.core.util.AppConfigReader;
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
    private static final String DEFAULT_PAYLOAD_SIZE = "65536";
    private static final String BINARY_SIZE = "websocket.binary.size";
    public static final String ID = "_id_";
    public static final String COUNT = "_blk_";
    public static final String TOTAL = "_max_";
    public static final String TO = "to";
    public static final String BROADCAST = "broadcast";
    public static final int OVERHEAD = 256;
    private static final String TO_MONITOR = "@monitor";
    private static final SimpleCache cache = SimpleCache.createCache("payload.segmentation", 60000);
    private final int maxPayload;
    private static final MultipartPayload instance = new MultipartPayload();

    private MultipartPayload() {
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        maxPayload = util.str2int(reader.getProperty(BINARY_SIZE, DEFAULT_PAYLOAD_SIZE)) - OVERHEAD;
        String threshold = NumberFormat.getInstance().format(maxPayload);
        log.info("Automatic segmentation when event payload exceeds {}", threshold);
    }

    public static MultipartPayload getInstance() {
        return instance;
    }

    public void incoming(EventEnvelope message) throws IOException {
        EventEmitter po = EventEmitter.getInstance();
        Map<String, String> control = message.getHeaders();
        if (message.getTo() != null) {
            String to = message.getTo();
            // remove special routing qualifier for presence monitor events
            if (to.contains(TO_MONITOR)) {
                message.setTo(to.substring(0, to.indexOf(TO_MONITOR)));
            }
            po.send(message);
        } else if (isDataBlock(control) && control.containsKey(ID) &&
                control.containsKey(COUNT) && control.containsKey(TOTAL)) {
            // segmented incoming event
            Utility util = Utility.getInstance();
            String id = control.get(ID);
            int count = util.str2int(control.get(COUNT));
            int total = util.str2int(control.get(TOTAL));
            if (message.getBody() instanceof byte[] && count != -1 && total != -1 && count <= total) {
                byte[] data = (byte[]) message.getBody();
                log.debug("Receiving block {} of {} as {} - {} bytes", count, total, id, data.length);
                Object o = cache.get(id);
                EventBlocks segments = o instanceof EventBlocks ? (EventBlocks) o : new EventBlocks(id);
                if (segments.exists(count)) {
                    log.error("Duplicated block {} for event {} dropped", count, id);
                } else {
                    segments.put(count, data);
                    if (total == segments.size()) {
                        EventEnvelope reconstructed = new EventEnvelope();
                        reconstructed.load(segments.toBytes());
                        cache.remove(id);
                        po.send(reconstructed);
                    } else {
                        cache.put(id, segments);
                    }
                }
            }
        }
    }

    private boolean isDataBlock(Map<String, String> control) {
        int n = 0;
        for (String k: control.keySet()) {
            if (k.startsWith("_") && k.endsWith("_") && k.length() > 2) {
                n++;
            }
        }
        return control.size() == n;
    }

    public void outgoing(String dest, EventEnvelope event) throws IOException {
        Platform platform = Platform.getInstance();
        if (platform.hasRoute(dest)) {
            outgoing(platform.getManager(dest), event);
        }
    }

    public void outgoing(ServiceQueue dest, EventEnvelope event) throws IOException {
        if (dest != null && event != null) {
            event.setEndOfRoute();
            byte[] payload = event.toBytes();
            EventBus system = Platform.getInstance().getEventSystem();
            if (payload.length > maxPayload) {
                int total = (payload.length / maxPayload) + (payload.length % maxPayload == 0 ? 0 : 1);
                ByteArrayInputStream in = new ByteArrayInputStream(payload);
                for (int i = 0; i < total; i++) {
                    // To distinguish from a normal payload, the segmented block MUST not have a "TO" value.
                    int count = i + 1;
                    EventEnvelope blk = new EventEnvelope()
                                            .setHeader(MultipartPayload.ID, event.getId())
                                            .setHeader(MultipartPayload.COUNT, count)
                                            .setHeader(MultipartPayload.TOTAL, total);
                    byte[] segment = new byte[maxPayload];
                    int size = in.read(segment);
                    blk.setBody(size == maxPayload ? segment : Arrays.copyOfRange(segment, 0, size));
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
                    system.send(dest.getRoute(), out.toBytes());
                    log.debug("Sending block {} of {} to {} as {} - {} bytes", i + 1, total, event.getTo(),
                            event.getId(), size);
                }

            } else {
                EventEnvelope out = new EventEnvelope().setHeader(TO, event.getTo()).setBody(payload);
                if (event.getBroadcastLevel() > 1) {
                    // tell a cloud connector that this event should be broadcast
                    out.setHeader(BROADCAST, "1");
                }
                system.send(dest.getRoute(), out.toBytes());
            }
        }
    }

}
