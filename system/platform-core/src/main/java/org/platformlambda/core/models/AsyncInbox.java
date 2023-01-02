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

package org.platformlambda.core.models;

import io.vertx.core.Future;
import io.vertx.core.Handler;
import io.vertx.core.Promise;
import io.vertx.core.eventbus.Message;
import io.vertx.core.eventbus.MessageConsumer;
import org.apache.logging.log4j.ThreadContext;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

public class AsyncInbox extends InboxBase {
    private static final Logger log = LoggerFactory.getLogger(AsyncInbox.class);

    private final long begin = System.nanoTime();
    private final Future<EventEnvelope> future;
    private final long timeout;
    private long timer;
    private MessageConsumer<byte[]> listener;
    private Promise<EventEnvelope> promise;

    public AsyncInbox(String from, String traceId, String tracePath, long timeout) {
        final Platform platform = Platform.getInstance();
        this.timeout = Math.max(100, timeout);
        this.future = Future.future(p -> {
            this.promise = p;
            this.id = "r."+ Utility.getInstance().getUuid();
            String sender = from == null? ASYNC_INBOX : from;
            this.listener = platform.getEventSystem().localConsumer(this.id, new InboxHandler(sender));
            inboxes.put(id, this);
            timer = platform.getVertx().setTimer(timeout, t -> abort(id, sender, traceId, tracePath));
        });
    }

    public Future<EventEnvelope> getFuture() {
        return future;
    }

    private void abort(String inboxId, String from, String traceId, String tracePath) {
        AsyncInbox holder = (AsyncInbox) inboxes.get(inboxId);
        if (holder != null) {
            holder.close();
            executor.submit(() -> {
                PostOffice po = PostOffice.getInstance();
                String traceLogHeader = po.getTraceLogHeader();
                po.startTracing(from, traceId, tracePath);
                if (traceId != null) {
                    ThreadContext.put(traceLogHeader, traceId);
                }
                holder.promise.fail(new TimeoutException("Timeout for "+holder.timeout+" ms"));
                po.stopTracing();
                ThreadContext.remove(traceLogHeader);
            });
        }
    }

    private void close() {
        inboxes.remove(id);
        if (listener.isRegistered()) {
            listener.unregister();
        }
    }

    private class InboxHandler implements Handler<Message<byte[]>> {

        final String sender;

        public InboxHandler(String sender) {
            this.sender = sender == null? ASYNC_INBOX : sender;
        }

        @Override
        public void handle(Message<byte[]> message) {
            try {
                EventEnvelope event = new EventEnvelope(message.body());
                String inboxId = event.getReplyTo();
                if (inboxId != null) {
                    saveResponse(sender, inboxId, event.setReplyTo(null));
                }
            } catch (IOException e) {
                log.error("Unable to decode event - {}", e.getMessage());
            }
        }

        private void saveResponse(String sender, String inboxId, EventEnvelope reply) {
            AsyncInbox holder = (AsyncInbox) inboxes.get(inboxId);
            if (holder != null) {
                holder.close();
                Platform.getInstance().getVertx().cancelTimer(timer);
                float diff = (float) (System.nanoTime() - holder.begin) / PostOffice.ONE_MILLISECOND;
                // adjust precision to 3 decimal points
                reply.setRoundTrip(Float.parseFloat(String.format("%.3f", Math.max(0.0f, diff))));
                executor.submit(() -> {
                    PostOffice po = PostOffice.getInstance();
                    String traceLogHeader = po.getTraceLogHeader();
                    po.startTracing(sender, reply.getTraceId(), reply.getTracePath());
                    if (reply.getTraceId() != null) {
                        ThreadContext.put(traceLogHeader, reply.getTraceId());
                    }
                    holder.promise.complete(reply);
                    po.stopTracing();
                    ThreadContext.remove(traceLogHeader);
                });
            }
        }
    }
}
