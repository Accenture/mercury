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
import io.vertx.core.eventbus.MessageConsumer;
import org.platformlambda.core.util.ElasticQueue;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.ConcurrentLinkedQueue;

public class ServiceQueue {
    private static final Logger log = LoggerFactory.getLogger(ServiceQueue.class);
    private static final String INIT = "init:";
    private static final String READY = "ready";
    private static final String HASH = "#";
    private final ElasticQueue elasticQueue;
    private final String route;
    private final String readyPrefix;
    private final String streamRoute;
    private final EventBus system;
    private final ConcurrentLinkedQueue<String> pool = new ConcurrentLinkedQueue<>();
    private final List<WorkerQueues> workers = new ArrayList<>();
    private MessageConsumer<Object> consumer;
    private boolean buffering = true;
    private boolean stopped = false;

    public ServiceQueue(ServiceDef service) {
        this.route = service.getRoute();
        this.readyPrefix = READY+":" + service.getRoute() + HASH;
        this.elasticQueue = new ElasticQueue(route);
        // create consumer
        system = Platform.getInstance().getEventSystem();
        consumer = system.localConsumer(service.getRoute(), new ServiceHandler());
        if (service.isStream()) {
            streamRoute = route + HASH + 1;
            StreamQueue worker = new StreamQueue(service, streamRoute);
            workers.add(worker);
            log.info("{} {} started", "PRIVATE", route);
        } else {
            // create workers
            streamRoute = null;
            int instances = service.getConcurrency();
            for (int i = 0; i < instances; i++) {
                int n = i + 1;
                WorkerQueue worker = new WorkerQueue(service, route + HASH + n, n);
                workers.add(worker);
            }
            log.info("{} {} with {} instance{} started", service.isPrivate() ? "PRIVATE" : "PUBLIC",
                    route, instances, instances == 1 ? "" : "s");
        }
    }

    public String getRoute() {
        return route;
    }

    public int getFreeWorkers() {
        return pool.size();
    }

    public long getReadCounter() {
        return elasticQueue.getReadCounter();
    }

    public long getWriteCounter() {
        return elasticQueue.getWriteCounter();
    }

    public void stop() {
        if (consumer != null && consumer.isRegistered()) {
            // closing consumer
            consumer.unregister();
            // stopping worker
            for (WorkerQueues w: workers) {
                w.stop();
            }
            // completely close the associated elastic queue
            elasticQueue.destroy();
            consumer = null;
            stopped = true;
            log.info("{} stopped", route);
        }
    }

    private class ServiceHandler implements Handler<Message<Object>> {

        @Override
        public void handle(Message<Object> message) {
            Object body = message.body();
            if (body instanceof String) {
                String text = (String) body;
                if (text.startsWith(INIT)) {
                    String uuid = text.substring(INIT.length());
                    BlockingQueue<Boolean> signal = Platform.getInstance().getServiceToken(uuid);
                    if (signal != null) {
                        signal.offer(true);
                    }
                } else {
                    String sender = getWorker(text);
                    if (sender != null && !stopped) {
                        pool.offer(sender);
                        if (buffering) {
                            byte[] event = elasticQueue.read();
                            if (event == null) {
                                // Close elastic queue when all messages are cleared
                                buffering = false;
                                elasticQueue.close();
                            } else {
                                // Guarantees that there is an available worker
                                String next = pool.poll();
                                if (next != null) {
                                    system.send(next, event);
                                }
                            }
                        }
                    }
                }
            }
            if (body instanceof byte[]) {
                byte[] event = (byte[]) body;
                if (!stopped) {
                    if (buffering) {
                        // Once elastic queue is started, we will continue buffering.
                        elasticQueue.write(event);
                    } else {
                        String next = pool.peek();
                        if (next == null) {
                            // Start persistent queue when no workers are available
                            buffering = true;
                            elasticQueue.write(event);
                        } else {
                            // Guarantees that there is an available worker
                            next = pool.poll();
                            if (next != null) {
                                system.send(next, event);
                            }
                        }
                    }
                }
            }
        }
    }

    private String getWorker(String input) {
        if (input.startsWith(readyPrefix)) {
            return input.substring(READY.length()+1);
        } else if (READY.equals(input) && streamRoute != null) {
            return streamRoute;
        }
        return null;
    }

}
