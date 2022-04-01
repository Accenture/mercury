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
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;
import java.util.concurrent.atomic.AtomicInteger;

public class AsyncMultiInbox extends InboxBase {
    private static final Logger log = LoggerFactory.getLogger(AsyncMultiInbox.class);

    private final int n;
    private final AtomicInteger total = new AtomicInteger(1);
    private final long begin = System.nanoTime();
    private final Future<List<EventEnvelope>> future;
    private final long timeout;
    private long timer;
    private MessageConsumer<byte[]> listener;
    private Promise<List<EventEnvelope>> promise;
    private ConcurrentMap<String, EventEnvelope> replies = new ConcurrentHashMap<>();

    public AsyncMultiInbox(int n, String from, String traceId, String tracePath, long timeout) {
        final Platform platform = Platform.getInstance();
        this.n = Math.max(1, n);
        this.total.set(this.n);
        this.timeout = Math.max(100, timeout);
        this.future = Future.future(promise -> {
            this.promise = promise;
            this.id = "r."+ Utility.getInstance().getUuid();
            String sender = from == null? ASYNC_INBOX : from;
            this.listener = platform.getEventSystem().localConsumer(this.id, new AsyncMultiInbox.InboxHandler(sender));
            inboxes.put(id, this);
            timer = platform.getVertx().setTimer(timeout, t -> {
                abort(id, sender, traceId, tracePath);
            });
        });
    }

    public Future<List<EventEnvelope>> getFuture() {
        return future;
    }

    private void abort(String inboxId, String from, String traceId, String tracePath) {
        AsyncMultiInbox holder = (AsyncMultiInbox) inboxes.get(inboxId);
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

    private void saveResponse(String sender, String inboxId, EventEnvelope reply) {
        AsyncMultiInbox holder = (AsyncMultiInbox) inboxes.get(inboxId);
        if (holder != null) {
            float diff = System.nanoTime() - holder.begin;
            reply.setRoundTrip(diff / PostOffice.ONE_MILLISECOND);
            replies.put(reply.getId(), reply);
            if (holder.total.decrementAndGet() == 0) {
                List<EventEnvelope> result = new ArrayList<>();
                for (String k: replies.keySet()) {
                    result.add(replies.get(k));
                }
                holder.close();
                Platform.getInstance().getVertx().cancelTimer(timer);
                executor.submit(() -> {
                    PostOffice po = PostOffice.getInstance();
                    String traceLogHeader = po.getTraceLogHeader();
                    po.startTracing(sender, reply.getTraceId(), reply.getTracePath());
                    if (reply.getTraceId() != null) {
                        ThreadContext.put(traceLogHeader, reply.getTraceId());
                    }
                    holder.promise.complete(result);
                    po.stopTracing();
                    ThreadContext.remove(traceLogHeader);
                });
            }
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
                    saveResponse(sender, inboxId, event);
                }
            } catch (IOException e) {
                log.error("Unable to decode event - {}", e.getMessage());
            }
        }
    }
}
