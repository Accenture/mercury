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

import akka.actor.*;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.util.ElasticQueue;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import scala.concurrent.duration.Duration;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.TimeUnit;

public class ServiceQueue extends AbstractActor {
    private static final Logger log = LoggerFactory.getLogger(ServiceQueue.class);
    private static final String QUEUES = "queues";
    private static final StopSignal STOP = new StopSignal();
    private static final long SCHEDULED_STOP = 500;

    private Platform platform = Platform.getInstance();
    private ConcurrentLinkedQueue<ActorRef> pool = new ConcurrentLinkedQueue<>();
    private List<ActorRef> workers = new ArrayList<>();
    private ElasticQueue elasticQueue;
    private String route;
    private boolean buffering = true;
    private boolean started = false, stopped = false;

    public static Props props(String route) {
        return Props.create(ServiceQueue.class, () -> new ServiceQueue(route));
    }

    public ServiceQueue(String route) {
        this.route = route;
        String origin = Platform.getInstance().getOrigin();
        this.elasticQueue = new ElasticQueue(new File(Utility.getInstance().getWorkFolder(), QUEUES), route+"-"+origin);
    }

    @Override
    public Receive createReceive() {
        return receiveBuilder().match(ServiceDef.class, def -> {
            if (!started) {
                started = true;
                ActorSystem system = platform.getEventSystem();
                int instances = def.getConcurrency();
                for (int i=0; i < instances; i++) {
                    int n = i + 1;
                    ActorRef worker = system.actorOf(WorkerQueue.props(def, getSelf(), n), getSelf().path().name()+"@"+n);
                    workers.add(worker);
                }
                log.info("{} {} with {} instance{} started", def.isPrivate()? "PRIVATE" : "PUBLIC",
                        route, instances, instances == 1 ? "" : "s");
            }

        }).match(ReadySignal.class, signal -> {
            if (!stopped) {
                log.debug(getSender().path().name() + " is ready");
                pool.offer(getSender());
                if (buffering) {
                    try {
                        EventEnvelope event = elasticQueue.read();
                        if (event == null) {
                            // Close elastic queue when all messages are cleared
                            buffering = false;
                            elasticQueue.close();
                        } else {
                            // Guarantees that there is an available worker
                            ActorRef next = pool.poll();
                            if (next != null) {
                                next.tell(event, getSelf());
                            }
                        }
                    } catch (IOException e) {
                        // this should not happen
                        log.error("Unable to read elastic queue " + elasticQueue.getId(), e);
                    }
                }
            }

        }).match(EventEnvelope.class, event -> {
            if (!stopped) {
                if (buffering) {
                    // Once elastic queue is started, we will continue buffering.
                    elasticQueue.write(event);
                } else {
                    ActorRef next = pool.peek();
                    if (next == null) {
                        // Start persistent queue when no workers are available
                        buffering = true;
                        elasticQueue.write(event);
                    } else {
                        // Guarantees that there is an available worker
                        next = pool.poll();
                        if (next != null) {
                            next.tell(event, getSelf());
                        }
                    }
                }
            }

        }).match(StopSignal.class, signal -> {
            if (!stopped) {
                // stop processing events
                stopped = true;
                log.debug("Stopping {} with {} of {} workers in pool", getSelf().path().name(), pool.size(), workers.size());
                // stop workers
                for (ActorRef w: workers) {
                    w.tell(STOP, ActorRef.noSender());
                }
                pool.clear();
                workers = new ArrayList<>();
                // give a little bit of time to ignore incoming messages that are already queued
                final ActorSystem system = Platform.getInstance().getEventSystem();
                system.scheduler().scheduleOnce(Duration.create(SCHEDULED_STOP, TimeUnit.MILLISECONDS), () -> {
                    getSelf().tell(PoisonPill.getInstance(), ActorRef.noSender());
                }, system.dispatcher());
            }

        }).build();
    }

    @Override
    public void postStop() {
        // close elastic queue
        if (!elasticQueue.isClosed()) {
            elasticQueue.close();
        }
        // remove elastic queue folder
        elasticQueue.destroy();
        log.info("{} stopped", getSelf().path().name());
    }

}
