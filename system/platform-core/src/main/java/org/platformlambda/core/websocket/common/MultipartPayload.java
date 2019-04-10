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

package org.platformlambda.core.websocket.common;

import akka.actor.ActorRef;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.Arrays;
import java.util.Map;

public class MultipartPayload {
    private static final Logger log = LoggerFactory.getLogger(MultipartPayload.class);

    private static MultipartPayload instance = new MultipartPayload();
    private static final ManagedCache cache = ManagedCache.createCache("LargePayloads", 30000);
    private static final String ID = "id";
    private static final String TO = "to";
    private static final String COUNT = "count";
    private static final String TOTAL = "total";
    private static final String BROADCAST = "broadcast";

    private MultipartPayload() {
        // singleton
    }

    public static MultipartPayload getInstance() {
        return instance;
    }

    public void incoming(EventEnvelope message) throws IOException {
        Map<String, String> control = message.getHeaders();
        if (control.size() == 3 && control.containsKey(ID)
                && control.containsKey(COUNT) && control.containsKey(TOTAL)) {
            Utility util = Utility.getInstance();
            String id = control.get(ID);
            int count = util.str2int(control.get(COUNT));
            int total = util.str2int(control.get(TOTAL));
            byte[] data = (byte[]) message.getBody();
            if (data != null && count != -1 && total != -1) {
                ByteArrayOutputStream buffer = (ByteArrayOutputStream) cache.get(id);
                if (count == 1 || buffer == null) {
                    buffer = new ByteArrayOutputStream();
                    cache.put(id, buffer);
                }
                buffer.write(data);
                if (count == total) {
                    EventEnvelope reconstructed = new EventEnvelope();
                    reconstructed.load(buffer.toByteArray());
                    cache.remove(id);
                    PostOffice.getInstance().send(reconstructed);
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
            int maxPayload = WsConfigurator.getInstance().getMaxBinaryPayload();
            byte[] payload = event.toBytes();
            if (payload.length > maxPayload) {
                int total = (payload.length / maxPayload) + (payload.length % maxPayload == 0 ? 0 : 1);
                ByteArrayInputStream in = new ByteArrayInputStream(payload);
                for (int i = 0; i < total; i++) {
                    // To distinguish from a normal payload, the segmented block MUST not have a "TO" value.
                    EventEnvelope block = new EventEnvelope();
                    block.setHeader(MultipartPayload.ID, event.getId());
                    block.setHeader(MultipartPayload.COUNT, String.valueOf(i + 1));
                    block.setHeader(MultipartPayload.TOTAL, String.valueOf(total));
                    byte[] segment = new byte[maxPayload];
                    int size = in.read(segment);
                    block.setBody(size == maxPayload ? segment : Arrays.copyOfRange(segment, 0, size));
                    EventEnvelope out = new EventEnvelope().setHeader(TO, event.getTo()).setBody(block.toBytes());
                    if (event.getBroadcastLevel() == 1) {
                        event.setBroadcastLevel(2);
                        out.setHeader(BROADCAST, "1");
                    }
                    dest.tell(out, ActorRef.noSender());
                    log.debug("Sending block {} of {} to {} via {}", i + 1, total, event.getTo(), dest.path().name());
                }

            } else {
                EventEnvelope out = new EventEnvelope().setHeader(TO, event.getTo()).setBody(payload);
                if (event.getBroadcastLevel() == 1) {
                    event.setBroadcastLevel(2);
                    out.setHeader(BROADCAST, "1");
                }
                dest.tell(out, ActorRef.noSender());
            }
        }
    }

}
