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

import io.vertx.core.Handler;
import io.vertx.core.eventbus.Message;
import io.vertx.core.eventbus.MessageConsumer;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;

public class Inbox {
    private static final Logger log = LoggerFactory.getLogger(Inbox.class);

    private static final ConcurrentMap<String, Inbox> inboxes = new ConcurrentHashMap<>();
    private final MessageConsumer<byte[]> listener;
    private final String id;
    private final int n;
    private final long begin = System.nanoTime();
    private final AtomicInteger total = new AtomicInteger(1);
    private final BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
    private EventEnvelope reply;
    private ConcurrentMap<String, EventEnvelope> replies;

    /**
     * Inbox for one or more requests
     * @param n is the number of parallel requests
     */
    public Inbox(int n) {
        if (n > 1) {
            total.set(n);
            replies = new ConcurrentHashMap<>();
            this.n = n;
        } else {
            this.n = 1;
        }
        this.id = "r."+ Utility.getInstance().getUuid();
        this.listener = Platform.getInstance().getEventSystem().localConsumer(this.id, new InboxHandler());
        Inbox.inboxes.put(id, this);
    }

    public String getId() {
        return id;
    }

    public void waitForResponse(long timeout) {
        try {
            bench.poll(timeout, TimeUnit.MILLISECONDS);
        } catch (InterruptedException e) {
            // ok to ignore
        }
    }

    public EventEnvelope getReply() {
        return reply;
    }

    public List<EventEnvelope> getReplies() {
        List<EventEnvelope> results = new ArrayList<>();
        if (n > 1) {
            for (String k: replies.keySet()) {
                results.add(replies.get(k));
            }
        } else if (reply != null) {
            results.add(reply);
        }
        return results;
    }

    private void setReply(EventEnvelope reply) {
        this.reply = reply;
    }

    private void addReply(EventEnvelope reply) {
        if (replies != null) {
            replies.put(reply.getId(), reply);
        }
    }

    public static Inbox getHolder(String inboxId) {
        return Inbox.inboxes.get(inboxId);
    }

    public static void saveResponse(String inboxId, EventEnvelope reply) {
        Inbox holder = Inbox.inboxes.get(inboxId);
        if (holder != null) {
            float diff = System.nanoTime() - holder.begin;
            reply.setRoundTrip(diff / PostOffice.ONE_MILLISECOND);
            if (holder.n > 1) {
                holder.addReply(reply);
                // all parallel responses have arrived
                if (holder.total.decrementAndGet() == 0) {
                    holder.bench.offer(true);
                }
            } else {
                // response has arrived
                holder.setReply(reply);
                holder.bench.offer(true);
            }
        }
    }

    public void close() {
        Inbox.inboxes.remove(id);
        if (listener.isRegistered()) {
            listener.unregister();
        }
    }

    private class InboxHandler implements Handler<Message<byte[]>> {

        @Override
        public void handle(Message<byte[]> message) {
            EventEnvelope event = new EventEnvelope();
            try {
                event.load(message.body());
                String inboxId = event.getReplyTo();
                if (inboxId != null) {
                    Inbox.saveResponse(inboxId, event);
                }
            } catch (IOException e) {
                log.error("Unable to decode event - {}", e.getMessage());
            }
        }
    }

}
