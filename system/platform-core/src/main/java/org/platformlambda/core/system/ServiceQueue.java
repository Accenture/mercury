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

package org.platformlambda.core.system;

import io.vertx.core.Handler;
import io.vertx.core.eventbus.EventBus;
import io.vertx.core.eventbus.Message;
import io.vertx.core.eventbus.MessageConsumer;
import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.util.ElasticQueue;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;

public class ServiceQueue {
    private static final Logger log = LoggerFactory.getLogger(ServiceQueue.class);
    private static final String READY = "ready";
    private static final String HASH = "#";
    private static final String AS_COROUTINE = "as coroutine";
    private static final String WORKER_POOL = "in worker pool";
    private static final String STREAM = "STREAM";
    private static final String PUBLIC = "PUBLIC";
    private static final String PRIVATE = "PRIVATE";
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
            boolean coroutine = service.getStreamFunction().getClass().getAnnotation(CoroutineRunner.class) != null;
            log.info("{} {} started {}", STREAM, route, coroutine ? AS_COROUTINE : "");
        } else {
            streamRoute = null;
            int instances = service.getConcurrency();
            for (int i = 0; i < instances; i++) {
                int n = i + 1;
                WorkerQueue worker = new WorkerQueue(service, route + HASH + n, n);
                workers.add(worker);
            }
            if (service.isKotlin()) {
                if (instances == 1) {
                    log.info("{} {} started as suspend function", service.isPrivate() ? PRIVATE : PUBLIC, route);
                } else {
                    log.info("{} {} with {} instances started as suspend function",
                            service.isPrivate() ? PRIVATE : PUBLIC, route, instances);
                }
            } else {
                boolean coroutine = service.getFunction().getClass().getAnnotation(CoroutineRunner.class) != null;
                if (instances == 1) {
                    log.info("{} {} started {}", service.isPrivate() ? PRIVATE : PUBLIC,
                            route, coroutine? AS_COROUTINE : WORKER_POOL);
                } else {
                    log.info("{} {} with {} instances started {}", service.isPrivate() ? PRIVATE : PUBLIC,
                            route, instances, coroutine? AS_COROUTINE : WORKER_POOL);
                }
            }
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
                String worker = getWorker((String) body);
                if (worker != null && !stopped) {
                    // Just for the safe side, this guarantees that a unique worker is inserted
                    if (!pool.contains(worker)) {
                        pool.offer(worker);
                    }
                    if (buffering) {
                        byte[] event = elasticQueue.read();
                        if (event == null) {
                            // Close elastic queue when all messages are cleared
                            buffering = false;
                            elasticQueue.close();
                        } else {
                            // Guarantees that there is an available worker
                            String nextWorker = pool.poll();
                            if (nextWorker != null) {
                                system.send(nextWorker, event);
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
                        // Check if a next worker is available
                        String nextWorker = pool.peek();
                        if (nextWorker == null) {
                            // Start persistent queue when no workers are available
                            buffering = true;
                            elasticQueue.write(event);
                        } else {
                            // Deliver event to the next worker
                            nextWorker = pool.poll();
                            if (nextWorker != null) {
                                system.send(nextWorker, event);
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

}
