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

package org.platformlambda.core.system;

import akka.actor.AbstractActor;
import akka.actor.Props;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Inbox;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class InboxListener extends AbstractActor {
    private static final Logger log = LoggerFactory.getLogger(InboxListener.class);

    public static Props props() {
        return Props.create(InboxListener.class, () -> new InboxListener());
    }

    @Override
    public Receive createReceive() {
        return receiveBuilder().match(EventEnvelope.class, reply -> {
            String inboxId = reply.getReplyTo();
            if (inboxId != null) {
                Inbox.saveResponse(inboxId, reply);
            }
        }).build();
    }

    @Override
    public void postStop() {
        log.debug("Temporary inbox {} closed", getSelf().path().name());
    }
}
