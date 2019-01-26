/*

    Copyright 2018-2019 Accenture Technology

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
import akka.actor.ActorRef;
import akka.actor.PoisonPill;
import akka.actor.Props;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class WorkerQueue extends AbstractActor {
    private static final Logger log = LoggerFactory.getLogger(WorkerQueue.class);
    private static final ExecutorService executor = Executors.newCachedThreadPool();
    private static final ReadySignal READY = new ReadySignal();
    private static final Utility util = Utility.getInstance();
    private static final String ORIGIN = "origin";

    private PostOffice po = PostOffice.getInstance();
    private ServiceDef def;
    private int instance;
    private ActorRef manager;
    private boolean stopped = false;

    public static Props props(ServiceDef def, ActorRef manager, int instance) {
        return Props.create(WorkerQueue.class, () -> new WorkerQueue(def, manager, instance));
    }

    public WorkerQueue(ServiceDef def, ActorRef manager, int instance) {
        this.def = def;
        this.instance = instance;
        this.manager = manager;
        // tell manager that this worker is ready to process a new event
        manager.tell(READY, getSelf());
        log.debug("{} started", getSelf().path().name());
    }

    @Override
    public Receive createReceive() {
        return receiveBuilder().match(EventEnvelope.class, event -> {
            if (!stopped) {
                final ActorRef self = getSelf();
                executor.submit(()->{
                    // execute function as a future task
                    processEvent(event);
                    /*
                     * Send a ready signal to inform the system this worker is ready for next event.
                     * This guarantee that this future task is executed orderly
                     */
                    manager.tell(READY, self);
                });
            }

        }).match(StopSignal.class, signal -> {
            stopped = true;
            getSelf().tell(PoisonPill.getInstance(), ActorRef.noSender());

        }).build();
    }

    private void processEvent(EventEnvelope event) {
        LambdaFunction f = def.getFunction();
        try {
            boolean ping = event.getHeaders().isEmpty() && event.getBody() == null;
            long begin = System.nanoTime();
            Object result = ping? null : f.handleEvent(event.getHeaders(), event.getBody(), instance);
            float diff = ping? 0 : System.nanoTime() - begin;
            String replyTo = event.getReplyTo();
            if (replyTo != null) {
                boolean reply = true;
                EventEnvelope response = new EventEnvelope();
                response.setTo(replyTo);
                if (event.getCorrelationId() != null) {
                    response.setCorrelationId(event.getCorrelationId());
                }
                if (result instanceof EventEnvelope) {
                    EventEnvelope resultEnvelope = (EventEnvelope) result;
                    Map<String, String> headers = resultEnvelope.getHeaders();
                    // is this a no-reply object?
                    if (headers.isEmpty() && resultEnvelope.getBody() == null) {
                        reply = false;
                    } else {
                        /*
                         * When EventEnvelope is used as a return type, the system will transport
                         * 1. payload
                         * 2. key-values (headers or parameters)
                         * 3. optional parametric types for Java class that uses generic types
                         */
                        response.setBody(resultEnvelope.getBody());
                        for (String h : headers.keySet()) {
                            response.setHeader(h, headers.get(h));
                        }
                        response.setStatus(resultEnvelope.getStatus());
                        if (resultEnvelope.getParametricType() != null) {
                            response.setParametricType(resultEnvelope.getParametricType());
                        }
                    }
                } else {
                    response.setStatus(200).setBody(result);
                }
                if (ping) {
                    response.setHeader(ORIGIN, Platform.getInstance().getOrigin());
                } else {
                    response.setExecutionTime(diff / PostOffice.ONE_MILLISECOND);
                }
                if (reply) {
                    po.send(response);
                }
            }

        } catch (Exception e) {
            int status;
            Throwable ex = util.getRootCause(e);
            if (ex instanceof AppException) {
                status = ((AppException) ex).getStatus();
            } else if (ex instanceof IllegalArgumentException) {
                status = 400;
            } else if (ex instanceof IOException) {
                status = 400;
            } else {
                status = 500;
            }
            String replyTo = event.getReplyTo();
            if (replyTo != null) {
                EventEnvelope response = new EventEnvelope();
                response.setTo(replyTo).setStatus(status).setBody(ex.getMessage());
                if (event.getCorrelationId() != null) {
                    response.setCorrelationId(event.getCorrelationId());
                }
                try {
                    po.send(response);
                } catch (Exception nested) {
                    log.warn("Unhandled exception for {} - {}", getSelf().path().name(), nested.getMessage());
                }
            } else {
                if (status >= 500) {
                    log.error("Unhandled exception for "+getSelf().path().name(), ex);
                } else {
                    log.warn("Unhandled exception for {} - {}", getSelf().path().name(), ex.getMessage());
                }
            }
        }
    }

    @Override
    public void postStop() {
        log.debug("{} stopped", getSelf().path().name());
    }

}
