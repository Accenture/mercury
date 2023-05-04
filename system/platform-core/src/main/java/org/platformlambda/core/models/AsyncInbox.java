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
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class AsyncInbox extends InboxBase {
    private static final Logger log = LoggerFactory.getLogger(AsyncInbox.class);

    private final String start = Utility.getInstance().date2str(new Date());
    private final long begin = System.nanoTime();
    private final Future<EventEnvelope> future;
    private final String traceId;
    private final String tracePath;
    private final String from;
    private final String to;
    private final long timeout;
    private final boolean timeoutException;
    private long timer;
    private MessageConsumer<byte[]> listener;
    private Promise<EventEnvelope> promise;

    public AsyncInbox(String from, String to, String traceId, String tracePath, long timeout,
                      boolean timeoutException) {
        final Platform platform = Platform.getInstance();
        this.timeoutException = timeoutException;
        this.from = from == null? "unknown" : from;
        this.to = to;
        this.traceId = traceId;
        this.tracePath = tracePath;
        this.timeout = Math.max(100, timeout);
        this.future = Future.future(p -> {
            this.promise = p;
            this.id = "r."+ Utility.getInstance().getUuid();
            this.listener = platform.getEventSystem().localConsumer(this.id, new InboxHandler());
            inboxes.put(id, this);
            timer = platform.getVertx().setTimer(timeout, t -> abort(this.id));
        });
    }

    public Future<EventEnvelope> getFuture() {
        return future;
    }

    private void abort(String inboxId) {
        AsyncInbox holder = (AsyncInbox) inboxes.get(inboxId);
        if (holder != null) {
            holder.close();
            String error = "Timeout for " + holder.timeout + " ms";
            executor.submit(() -> {
                if (timeoutException) {
                    holder.promise.fail(new TimeoutException(error));
                } else {
                    holder.promise.complete(new EventEnvelope().setStatus(408).setBody(error));
                }
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

        private static final String RPC = "rpc";
        private static final String UNDERSCORE = "_";
        private static final String ANNOTATIONS = "annotations";

        @Override
        public void handle(Message<byte[]> message) {
            try {
                EventEnvelope event = new EventEnvelope(message.body());
                String inboxId = event.getReplyTo();
                if (inboxId != null) {
                    saveResponse(inboxId, event.setReplyTo(null));
                }
            } catch (IOException e) {
                log.error("Unable to decode event - {}", e.getMessage());
            }
        }

        private void saveResponse(String inboxId, EventEnvelope reply) {
            AsyncInbox holder = (AsyncInbox) inboxes.get(inboxId);
            if (holder != null) {
                holder.close();
                Platform.getInstance().getVertx().cancelTimer(timer);
                float diff = (float) (System.nanoTime() - holder.begin) / EventEmitter.ONE_MILLISECOND;
                // adjust precision to 3 decimal points
                float roundTrip = Float.parseFloat(String.format("%.3f", Math.max(0.0f, diff)));
                reply.setRoundTrip(roundTrip);
                // remove some metadata that are not relevant for a RPC response
                reply.removeTag(RPC).setTo(null).setReplyTo(null).setTrace(null, null);
                Map<String, Object> annotations = new HashMap<>();
                // decode trace annotations from reply event
                Map<String, String> headers = reply.getHeaders();
                if (headers.containsKey(UNDERSCORE)) {
                    int count = Utility.getInstance().str2int(headers.get(UNDERSCORE));
                    for (int i=1; i <= count; i++) {
                        String kv = headers.get(UNDERSCORE+i);
                        if (kv != null) {
                            int eq = kv.indexOf('=');
                            if (eq > 0) {
                                annotations.put(kv.substring(0, eq), kv.substring(eq+1));
                            }
                        }
                    }
                    headers.remove(UNDERSCORE);
                    for (int i=1; i <= count; i++) {
                        headers.remove(UNDERSCORE+i);
                    }
                }
                executor.submit(() -> {
                    holder.promise.complete(reply);
                });
                if (to != null && holder.traceId != null && holder.tracePath != null) {
                    try {
                        Map<String, Object> payload = new HashMap<>();
                        Map<String, Object> metrics = new HashMap<>();
                        metrics.put("origin", Platform.getInstance().getOrigin());
                        metrics.put("id", holder.traceId);
                        metrics.put("service", to);
                        metrics.put("from", holder.from);
                        metrics.put("exec_time", reply.getExecutionTime());
                        metrics.put("round_trip", roundTrip);
                        metrics.put("success", true);
                        metrics.put("status", reply.getStatus());
                        metrics.put("start", start);
                        metrics.put("path", holder.tracePath);
                        payload.put("trace", metrics);
                        if (!annotations.isEmpty()) {
                            payload.put(ANNOTATIONS, annotations);
                        }
                        EventEnvelope dt = new EventEnvelope().setTo(EventEmitter.DISTRIBUTED_TRACING);
                        EventEmitter.getInstance().send(dt.setBody(payload));
                    } catch (Exception e) {
                        log.error("Unable to send to " + EventEmitter.DISTRIBUTED_TRACING, e);
                    }
                }
            }
        }
    }
}
