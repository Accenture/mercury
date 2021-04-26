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

package org.platformlambda.core.system;

import io.vertx.core.Handler;
import io.vertx.core.eventbus.EventBus;
import io.vertx.core.eventbus.Message;
import io.vertx.core.eventbus.MessageConsumer;
import org.platformlambda.core.util.ElasticQueue;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.ConcurrentLinkedQueue;

public class ServiceQueue {
    private static final Logger log = LoggerFactory.getLogger(ServiceQueue.class);
    private static final String QUEUES = "queues";
    private static final String INIT = "init:";
    private static final String READY = "ready:";
    private final ElasticQueue elasticQueue;
    private final String route;
    private final EventBus system;
    private final ConcurrentLinkedQueue<String> pool = new ConcurrentLinkedQueue<>();
    private final List<WorkerQueue> workers = new ArrayList<>();
    private MessageConsumer<Object> consumer;
    private boolean buffering = true;
    private boolean stopped = false;

    public ServiceQueue(ServiceDef service) {
        this.route = service.getRoute();
        String origin = Platform.getInstance().getOrigin();
        File workerFolder = Utility.getInstance().getWorkFolder();
        this.elasticQueue = new ElasticQueue(new File(workerFolder, QUEUES),this.route+"-"+origin);
        // create consumer
        system = Platform.getInstance().getEventSystem();
        consumer = system.localConsumer(service.getRoute(), new ServiceHandler());
        // create workers
        int instances = service.getConcurrency();
        for (int i=0; i < instances; i++) {
            int n = i + 1;
            WorkerQueue worker = new WorkerQueue(service, route+"@"+n, n);
            workers.add(worker);
        }
        log.info("{} {} with {} instance{} started", service.isPrivate()? "PRIVATE" : "PUBLIC",
                route, instances, instances == 1 ? "" : "s");
    }

    public String getRoute() {
        return route;
    }

    public void stop() {
        if (consumer != null && consumer.isRegistered()) {
            // stopping worker
            for (WorkerQueue worker: workers) {
                worker.stop();
            }
            // closing elastic queue
            if (!elasticQueue.isClosed()) {
                elasticQueue.close();
            }
            // remove elastic queue folder
            elasticQueue.destroy();
            // closing consumer
            consumer.unregister();
            consumer = null;
            stopped = true;
            log.info("{} stopped", route);
        }
    }

    @SuppressWarnings("unchecked")
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
                }
                if (text.startsWith(READY)) {
                    String sender = text.substring(READY.length());
                    if (!stopped) {
                        pool.offer(sender);
                        if (buffering) {
                            try {
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
                            } catch (IOException e) {
                                // this should not happen
                                log.error("Unable to read elastic queue " + elasticQueue.getId(), e);
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

}
