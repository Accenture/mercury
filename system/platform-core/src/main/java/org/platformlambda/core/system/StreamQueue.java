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

package org.platformlambda.core.system;

import io.vertx.core.Handler;
import io.vertx.core.eventbus.EventBus;
import io.vertx.core.eventbus.Message;
import org.platformlambda.core.models.EventEnvelope;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class StreamQueue extends WorkerQueues {
    private static final Logger log = LoggerFactory.getLogger(StreamQueue.class);

    public StreamQueue(ServiceDef def, String route) {
        super(def, route);
        EventBus system = Platform.getInstance().getEventSystem();
        this.consumer = system.localConsumer(route, new StreamHandler());
        def.getStreamFunction().init(def.getRoute());
        this.started();
    }

    private class StreamHandler implements Handler<Message<byte[]>> {

        @Override
        public void handle(Message<byte[]> message) {
            if (!stopped) {
                try {
                    EventEnvelope event = new EventEnvelope(message.body());
                    executor.submit(()-> processEvent(event));

                } catch (IOException e) {
                    log.error("Unable to decode event - {}", e.getMessage());
                }

            }
        }

        private void processEvent(EventEnvelope event) {
            try {
                def.getStreamFunction().handleEvent(event.getHeaders(), event.getBody());
            } catch (Exception e) {
                log.error("Unhandled exception for "+route, e);
            }
        }
        
    }
}
