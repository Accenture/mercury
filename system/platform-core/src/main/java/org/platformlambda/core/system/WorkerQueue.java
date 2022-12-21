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
import org.apache.logging.log4j.ThreadContext;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.*;
import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;

public class WorkerQueue extends WorkerQueues {
    private static final Logger log = LoggerFactory.getLogger(WorkerQueue.class);
    private static final Utility util = Utility.getInstance();
    private static final String TYPE = "type";
    private static final String TIME = "time";
    private static final String APP = "app";
    private static final String PONG = "pong";
    private static final String REASON = "reason";
    private static final String MESSAGE = "message";
    private static final String ORIGIN = "origin";
    private static final String SERVICE = "service";
    private static final String INPUT = "input";
    private static final String OUTPUT = "output";
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String STATUS = "status";
    private static final String EXCEPTION = "exception";
    private static final String ASYNC = "async";
    private static final String ANNOTATIONS = "annotations";
    private static final String UNDELIVERED = "undelivered";
    private static final String PAYLOAD = "payload";
    private final String myOrigin;
    private final boolean interceptor;
    private final boolean useEnvelope;
    private final boolean tracing;
    private final int instance;

    public WorkerQueue(ServiceDef def, String route, int instance) {
        super(def, route);
        this.instance = instance;
        EventBus system = Platform.getInstance().getEventSystem();
        this.consumer = system.localConsumer(route, new WorkerHandler());
        this.interceptor = def.getFunction().getClass().getAnnotation(EventInterceptor.class) != null;
        this.useEnvelope = def.inputIsEnvelope();
        this.tracing = def.getFunction().getClass().getAnnotation(ZeroTracing.class) == null;
        this.myOrigin = Platform.getInstance().getOrigin();
        // tell manager that this worker is ready to process a new event
        system.send(def.getRoute(), READY+route);
        this.started();
    }

    private class WorkerHandler implements Handler<Message<byte[]>> {

        @Override
        public void handle(Message<byte[]> message) {
            if (!stopped) {
                final EventEnvelope event = new EventEnvelope();
                try {
                    event.load(message.body());
                } catch (IOException e) {
                    log.error("Unable to decode event - {}", e.getMessage());
                    return;
                }
                // execute function as a future task
                executor.submit(()->{
                    PostOffice po = PostOffice.getInstance();
                    String traceLogHeader = po.getTraceLogHeader();
                    po.startTracing(def.getRoute(), event.getTraceId(), event.getTracePath());
                    if (event.getTraceId() != null) {
                        ThreadContext.put(traceLogHeader, event.getTraceId());
                    }
                    ProcessStatus ps = processEvent(event);
                    TraceInfo trace = po.stopTracing();
                    ThreadContext.remove(traceLogHeader);
                    if (tracing && trace != null && trace.id != null && trace.path != null) {
                        try {
                            if (!ps.isDelivered()) {
                                trace.annotate(UNDELIVERED, ps.getDeliveryError());
                            }
                            // Send tracing information to distributed trace logger
                            EventEnvelope dt = new EventEnvelope();
                            Map<String, Object> payload = new HashMap<>();
                            payload.put(ANNOTATIONS, trace.annotations);
                            // send input/output dataset to journal if configured in journal.yaml
                            if (po.isJournaled(def.getRoute())) {
                                payload.put(PAYLOAD, ps.getInputOutput());
                            }
                            dt.setTo(PostOffice.DISTRIBUTED_TRACING).setBody(payload);
                            dt.setHeader(ORIGIN, myOrigin);
                            dt.setHeader("id", trace.id).setHeader("path", trace.path);
                            dt.setHeader(SERVICE, def.getRoute()).setHeader("start", trace.startTime);
                            dt.setHeader("success", ps.isSuccess());
                            if (event.getFrom() != null) {
                                dt.setHeader("from", event.getFrom());
                            }
                            dt.setHeader("exec_time", ps.getExecutionTime());
                            if (!ps.isSuccess()) {
                                dt.setHeader(STATUS, ps.getStatus()).setHeader(EXCEPTION, ps.getException());
                            }
                            po.send(dt);
                        } catch (Exception e) {
                            log.error("Unable to send to "+PostOffice.DISTRIBUTED_TRACING, e);
                        }
                    } else {
                        if (!ps.isDelivered()) {
                            log.error("Delivery error - {}, from={}, to={}, type={}, exec_time={}",
                                    ps.getDeliveryError(),
                                    event.getFrom(), event.getTo(),
                                    ps.isSuccess()? "response" : "exception("+ps.getStatus()+", "+ps.getException()+")",
                                    ps.getExecutionTime());
                        }
                    }
                    /*
                     * Send a ready signal to inform the system this worker is ready for next event.
                     * This guarantee that this future task is executed orderly
                     */
                    Platform.getInstance().getEventSystem().send(def.getRoute(), READY+route);
                });
            }
        }

        @SuppressWarnings({"rawtypes", "unchecked"})
        private ProcessStatus processEvent(EventEnvelope event) {
            ProcessStatus ps = new ProcessStatus();
            PostOffice po = PostOffice.getInstance();
            Map<String, Object> inputOutput = new HashMap<>();
            Map<String, Object> input = new HashMap<>();
            input.put(HEADERS, event.getHeaders());
            input.put(BODY, event.getRawBody());
            inputOutput.put(INPUT, input);
            TypedLambdaFunction f = def.getFunction();
            if (event.hasError() && f instanceof ServiceExceptionHandler) {
                ServiceExceptionHandler handler = (ServiceExceptionHandler) f;
                try {
                    handler.onError(new AppException(event.getStatus(), event.getError()), event);
                } catch (Exception e1) {
                    ps.setUnDelivery(e1.getMessage());
                }
                Map<String, Object> output = new HashMap<>();
                output.put(STATUS, event.getStatus());
                output.put(EXCEPTION, event.getError());
                inputOutput.put(OUTPUT, output);
                return ps.setException(event.getStatus(), event.getError()).setInputOutput(inputOutput);
            }
            long begin = System.nanoTime();
            try {
                /*
                 * Interceptor can read any input (i.e. including case for empty headers and null body).
                 * The system therefore disables ping when the target function is an interceptor.
                 */
                boolean ping = !interceptor && event.getHeaders().isEmpty() &&
                                !event.isOptional() && event.getRawBody() == null;
                /*
                 * If the service is an interceptor or the input argument is EventEnvelope,
                 * we will pass the original event envelope instead of the message body.
                 */
                final Object inputBody;
                if (interceptor || useEnvelope) {
                    inputBody = event;
                } else {
                    if (PayloadMapper.MAP.equals(event.getType()) && def.getInputClass() != null) {
                        if (def.getInputClass() == AsyncHttpRequest.class) {
                            // handle special case
                            event.setType(null);
                            inputBody = new AsyncHttpRequest(event.getRawBody());
                        } else {
                            // automatically convert Map to PoJo
                            event.setType(def.getInputClass().getName());
                            inputBody = event.getBody();
                        }
                    } else {
                        inputBody = event.getBody();
                    }
                }
                Object result = ping? null : f.handleEvent(event.getHeaders(), inputBody, instance);
                float delta = ping? 0 : (float) (System.nanoTime() - begin) / PostOffice.ONE_MILLISECOND;
                // adjust precision to 3 decimal points
                float diff = Float.parseFloat(String.format("%.3f", Math.max(0.0f, delta)));
                Map<String, Object> output = new HashMap<>();
                String replyTo = event.getReplyTo();
                if (replyTo != null) {
                    boolean serviceTimeout = false;
                    EventEnvelope response = new EventEnvelope();
                    response.setTo(replyTo);
                    response.setFrom(def.getRoute());
                    /*
                     * Preserve correlation ID and notes
                     *
                     * "Notes" is usually used by event interceptors. The system does not restrict the content of the notes.
                     * For example, to save some metadata from the original sender.
                     */
                    if (event.getCorrelationId() != null) {
                        response.setCorrelationId(event.getCorrelationId());
                    }
                    if (event.getExtra() != null) {
                        response.setExtra(event.getExtra());
                    }
                    // propagate the trace to the next service if any
                    if (event.getTraceId() != null) {
                        response.setTrace(event.getTraceId(), event.getTracePath());
                    }
                    if (result instanceof EventEnvelope) {
                        EventEnvelope resultEvent = (EventEnvelope) result;
                        Map<String, String> headers = resultEvent.getHeaders();
                        if (headers.isEmpty() && resultEvent.getStatus() == 408 && resultEvent.getBody() == null) {
                            /*
                             * An empty event envelope with timeout status
                             * is used by the ObjectStreamService to simulate a READ timeout.
                             */
                            serviceTimeout = true;
                        } else {
                            /*
                             * When EventEnvelope is used as a return type, the system will transport
                             * 1. payload
                             * 2. key-values (as headers)
                             * 3. optional parametric types for Java class that uses generic types
                             */
                            response.setBody(resultEvent.getBody());
                            for (Map.Entry<String, String> kv: headers.entrySet()) {
                                response.setHeader(kv.getKey(), kv.getValue());
                            }
                            response.setStatus(resultEvent.getStatus());
                            if (resultEvent.getParametricType() != null) {
                                response.setParametricType(resultEvent.getParametricType());
                            }
                        }
                        if (!response.getHeaders().isEmpty()) {
                            output.put(HEADERS, response.getHeaders());
                        }
                    } else {
                        response.setBody(result);
                    }
                    output.put(BODY, response.getRawBody() == null? "null" : response.getRawBody());
                    output.put(STATUS, response.getStatus());
                    inputOutput.put(OUTPUT, output);
                    try {
                        if (ping) {
                            String parent = route.contains(HASH) ? route.substring(0, route.lastIndexOf(HASH)) : route;
                            Platform platform = Platform.getInstance();
                            // execution time is not set because there is no need to execute the lambda function
                            Map<String, Object> pong = new HashMap<>();
                            pong.put(TYPE, PONG);
                            pong.put(TIME, new Date());
                            pong.put(APP, platform.getName());
                            pong.put(ORIGIN, platform.getOrigin());
                            pong.put(SERVICE, parent);
                            pong.put(REASON, "This response is generated when you send an event without headers and body");
                            pong.put(MESSAGE, "you have reached " + parent);
                            response.setBody(pong);
                            po.send(response);
                        } else {
                            if (!interceptor && !serviceTimeout) {
                                response.setExecutionTime(diff);
                                po.send(response);
                            }
                        }
                    } catch (Exception e2) {
                        ps.setUnDelivery(e2.getMessage());
                    }
                } else {
                    EventEnvelope response = new EventEnvelope().setBody(result);
                    output.put(BODY, response.getRawBody() == null? "null" : response.getRawBody());
                    output.put(STATUS, response.getStatus());
                    output.put(ASYNC, true);
                    inputOutput.put(OUTPUT, output);
                }
                return ps.setExecutionTime(diff).setInputOutput(inputOutput);

            } catch (Exception e) {
                float delta = (float) (System.nanoTime() - begin) / PostOffice.ONE_MILLISECOND;
                float diff = Float.parseFloat(String.format("%.3f", Math.max(0.0f, delta)));
                ps.setExecutionTime(diff);
                final String replyTo = event.getReplyTo();
                final int status;
                Throwable ex = util.getRootCause(e);
                if (ex instanceof AppException) {
                    status = ((AppException) ex).getStatus();
                } else if (ex instanceof IllegalArgumentException || ex instanceof IOException) {
                    status = 400;
                } else {
                    status = 500;
                }
                if (f instanceof ServiceExceptionHandler) {
                    ServiceExceptionHandler handler = (ServiceExceptionHandler) f;
                    try {
                        handler.onError(new AppException(status, ex.getMessage()), event);
                    } catch (Exception e3) {
                        ps.setUnDelivery(e3.getMessage());
                    }
                    Map<String, Object> output = new HashMap<>();
                    output.put(STATUS, status);
                    output.put(EXCEPTION, ex.getMessage());
                    inputOutput.put(OUTPUT, output);
                    return ps.setException(status, ex.getMessage()).setInputOutput(inputOutput);
                }
                Map<String, Object> output = new HashMap<>();
                if (replyTo != null) {
                    EventEnvelope response = new EventEnvelope();
                    response.setTo(replyTo).setStatus(status).setBody(ex.getMessage());
                    response.setException(e);
                    response.setFrom(def.getRoute());
                    if (event.getCorrelationId() != null) {
                        response.setCorrelationId(event.getCorrelationId());
                    }
                    if (event.getExtra() != null) {
                        response.setExtra(event.getExtra());
                    }
                    // propagate the trace to the next service if any
                    if (event.getTraceId() != null) {
                        response.setTrace(event.getTraceId(), event.getTracePath());
                    }
                    try {
                        po.send(response);
                    } catch (Exception e4) {
                        ps.setUnDelivery(e4.getMessage());
                    }
                } else {
                    output.put(ASYNC, true);
                    if (status >= 500) {
                        log.error("Unhandled exception for "+route, ex);
                    } else {
                        log.warn("Unhandled exception for {} - {}", route, ex.getMessage());
                    }
                }
                output.put(STATUS, status);
                output.put(EXCEPTION, ex.getMessage());
                inputOutput.put(OUTPUT, output);
                return ps.setException(status, ex.getMessage()).setInputOutput(inputOutput);
            }
        }
    }

}
