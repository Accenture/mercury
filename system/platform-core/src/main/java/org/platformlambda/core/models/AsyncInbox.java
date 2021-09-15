/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.core.models;

import io.vertx.core.Future;
import io.vertx.core.Handler;
import io.vertx.core.Promise;
import io.vertx.core.eventbus.Message;
import io.vertx.core.eventbus.MessageConsumer;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeoutException;

public class AsyncInbox extends InboxBase {
    private static final Logger log = LoggerFactory.getLogger(AsyncInbox.class);

    private static final ExecutorService executor = Executors.newWorkStealingPool();
    private final long begin = System.nanoTime();
    private final Future<EventEnvelope> future;
    private final long timeout;
    private final String to;
    private long timer;
    private MessageConsumer<byte[]> listener;
    private Promise<EventEnvelope> promise;

    public AsyncInbox(String to, long timeout) {
        final Platform platform = Platform.getInstance();
        this.to = to;
        this.timeout = Math.max(100, timeout);
        this.future = Future.future(promise -> {
            this.promise = promise;
            this.id = "r."+ Utility.getInstance().getUuid();
            this.listener = platform.getEventSystem().localConsumer(this.id, new InboxHandler());
            inboxes.put(id, this);
            timer = platform.getVertx().setTimer(timeout, t -> {
                abort(id);
            });
        });
    }

    public Future<EventEnvelope> getFuture() {
        return future;
    }

    private void abort(String inboxId) {
        AsyncInbox holder = (AsyncInbox) inboxes.get(inboxId);
        if (holder != null) {
            holder.close();
            executor.submit(() -> holder.promise.fail(new TimeoutException(to+" timeout for "+holder.timeout+" ms")));
        }
    }

    private void saveResponse(String inboxId, EventEnvelope reply) {
        AsyncInbox holder = (AsyncInbox) inboxes.get(inboxId);
        if (holder != null) {
            holder.close();
            Platform.getInstance().getVertx().cancelTimer(timer);
            float diff = System.nanoTime() - holder.begin;
            reply.setRoundTrip(diff / PostOffice.ONE_MILLISECOND);
            executor.submit(() -> holder.promise.complete(reply));
        }
    }

    private void close() {
        inboxes.remove(id);
        if (listener.isRegistered()) {
            listener.unregister();
        }
    }

    private class InboxHandler implements Handler<Message<byte[]>> {

        @Override
        public void handle(Message<byte[]> message) {
            try {
                EventEnvelope event = new EventEnvelope(message.body());
                String inboxId = event.getReplyTo();
                if (inboxId != null) {
                    saveResponse(inboxId, event);
                }
            } catch (IOException e) {
                log.error("Unable to decode event - {}", e.getMessage());
            }
        }
    }
}
