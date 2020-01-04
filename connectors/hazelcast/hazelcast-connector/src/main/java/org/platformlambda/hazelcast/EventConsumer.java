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

package org.platformlambda.hazelcast;

import com.hazelcast.core.Message;
import com.hazelcast.core.MessageListener;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class EventConsumer implements MessageListener<byte[]> {
    private static final Logger log = LoggerFactory.getLogger(EventConsumer.class);

    @Override
    public void onMessage(Message<byte[]> event) {
        EventEnvelope message = new EventEnvelope();
        try {
            message.load(event.getMessageObject());
            message.setEndOfRoute();
            if (message.getTo() != null) {
                String to = message.getTo();
                /*
                 * Since this is the final destination, we will drop all events with
                 * direct addressing not targeting to this application instance.
                 */
                if (to.contains("@")) {
                    if (to.endsWith(Platform.getInstance().getOrigin())) {
                        PostOffice.getInstance().send(message);
                    }
                } else {
                    PostOffice.getInstance().send(message);
                }

            } else {
                // reconstruct large payload larger than 64MB
                MultipartPayload.getInstance().incoming(message);
            }

        } catch (IOException e) {
            log.error("Unable to process incoming event - {}", e.getMessage());
        }
    }
}
