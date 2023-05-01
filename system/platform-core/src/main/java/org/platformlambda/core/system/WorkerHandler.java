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

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.*;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Date;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class WorkerHandler {
    private static final Logger log = LoggerFactory.getLogger(WorkerHandler.class);
    private static final Utility util = Utility.getInstance();
    private static final String TYPE = "type";
    private static final String ID = "id";
    private static final String PATH = "path";
    private static final String SUCCESS = "success";
    private static final String FROM = "from";
    private static final String UNKNOWN = "unknown";
    private static final String EXEC_TIME = "exec_time";
    private static final String TIME = "time";
    private static final String APP = "app";
    private static final String PONG = "pong";
    private static final String REASON = "reason";
    private static final String MESSAGE = "message";
    private static final String ORIGIN = "origin";
    private static final String SERVICE = "service";
    private static final String START = "start";
    private static final String TRACE = "trace";
    private static final String INPUT = "input";
    private static final String OUTPUT = "output";
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String STATUS = "status";
    private static final String EXCEPTION = "exception";
    private static final String ASYNC = "async";
    private static final String ANNOTATIONS = "annotations";
    private static final String REMARK = "remark";
    private static final String JOURNAL = "journal";
    private static final String RPC = "rpc";
    private static final String DELIVERED = "delivered";
    private static final String MY_ROUTE = "my_route";
    private static final String MY_TRACE_ID = "my_trace_id";
    private static final String MY_TRACE_PATH = "my_trace_path";
    private static final String READY = "ready:";
    private static final String HASH = "#";
    private final boolean tracing;
    private final ServiceDef def;
    private final String route;
    private final String parentRoute;
    private final int instance;
    private final String myOrigin;
    private final boolean interceptor;
    private final boolean useEnvelope;

    public WorkerHandler(ServiceDef def, String route, int instance,
                         boolean tracing, boolean interceptor, boolean useEnvelope) {
        this.route = route;
        this.parentRoute = route.contains("#")? route.substring(0, route.lastIndexOf('#')) : route;
        this.instance = instance;
        this.def = def;
        this.tracing = tracing;
        this.interceptor = interceptor;
        this.useEnvelope = useEnvelope;
        this.myOrigin = Platform.getInstance().getOrigin();
    }

    public void executeFunction(EventEnvelope event) {
        String rpc = event.getTag(EventEmitter.RPC);
        EventEmitter po = EventEmitter.getInstance();
        String ref = tracing? po.startTracing(parentRoute, event.getTraceId(), event.getTracePath(), instance) : "?";
        ProcessStatus ps = processEvent(event);
        TraceInfo trace = po.stopTracing(ref);
        if (tracing && trace != null && trace.id != null && trace.path != null) {
            try {
                boolean journaled = po.isJournaled(def.getRoute());
                if (journaled || rpc == null || !ps.isDelivered()) {
                    // Send tracing information to distributed trace logger
                    EventEnvelope dt = new EventEnvelope().setTo(EventEmitter.DISTRIBUTED_TRACING);
                    Map<String, Object> payload = new HashMap<>();
                    payload.put(ANNOTATIONS, trace.annotations);
                    // send input/output dataset to journal if configured in journal.yaml
                    if (journaled) {
                        payload.put(JOURNAL, ps.getInputOutput());
                    }
                    Map<String, Object> metrics = new HashMap<>();
                    metrics.put(ORIGIN, myOrigin);
                    metrics.put(ID, trace.id);
                    metrics.put(PATH, trace.path);
                    metrics.put(SERVICE, def.getRoute());
                    metrics.put(START, trace.startTime);
                    metrics.put(SUCCESS, ps.isSuccess());
                    metrics.put(FROM, event.getFrom() == null ? UNKNOWN : event.getFrom());
                    metrics.put(EXEC_TIME, ps.getExecutionTime());
                    if (!ps.isSuccess()) {
                        metrics.put(STATUS, ps.getStatus());
                        metrics.put(EXCEPTION, ps.getException());
                    }
                    if (!ps.isDelivered()) {
                        metrics.put(REMARK, "Response not delivered - "+ps.getDeliveryError());
                    }
                    payload.put(TRACE, metrics);
                    dt.setHeader(DELIVERED, ps.isDelivered());
                    dt.setHeader(RPC, rpc != null);
                    dt.setHeader(JOURNAL, journaled);
                    po.send(dt.setBody(payload));
                }
            } catch (Exception e) {
                log.error("Unable to send to " + EventEmitter.DISTRIBUTED_TRACING, e);
            }
        } else {
            if (!ps.isDelivered()) {
                log.error("Delivery error - {}, from={}, to={}, type={}, exec_time={}",
                        ps.getDeliveryError(),
                        event.getFrom() == null? UNKNOWN : event.getFrom(), event.getTo(),
                        ps.isSuccess()? "response" : "exception("+ps.getStatus()+", "+ps.getException()+")",
                        ps.getExecutionTime());
            }
        }
        /*
         * Send a ready signal to inform the system this worker is ready for next event.
         * This guarantee that this future task is executed orderly
         */
        Platform.getInstance().getEventSystem().send(def.getRoute(), READY+route);
    }

    @SuppressWarnings({"rawtypes", "unchecked"})
    private ProcessStatus processEvent(EventEnvelope event) {
        Map<String, String> eventHeaders = event.getHeaders();
        ProcessStatus ps = new ProcessStatus();
        EventEmitter po = EventEmitter.getInstance();
        Map<String, Object> inputOutput = new HashMap<>();
        Map<String, Object> input = new HashMap<>();
        input.put(HEADERS, eventHeaders);
        input.put(BODY, event.getRawBody());
        inputOutput.put(INPUT, input);
        TypedLambdaFunction f = def.getFunction();
        long begin = System.nanoTime();
        try {
            /*
             * Interceptor can read any input (i.e. including case for empty headers and null body).
             * The system therefore disables ping when the target function is an interceptor.
             */
            boolean ping = !interceptor && !event.isOptional() && event.getRawBody() == null && eventHeaders.isEmpty();
            /*
             * If the service is an interceptor or the input argument is EventEnvelope,
             * we will pass the original event envelope instead of the message body.
             */
            final Object inputBody;
            if (useEnvelope || (interceptor && def.getInputClass() == null)) {
                inputBody = event;
            } else {
                if (event.getRawBody() instanceof Map && def.getInputClass() != null) {
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
            // Insert READ only metadata into function input headers
            Map<String, String> parameters = new HashMap<>(eventHeaders);
            parameters.put(MY_ROUTE, parentRoute);
            if (event.getTraceId() != null) {
                parameters.put(MY_TRACE_ID, event.getTraceId());
            }
            if (event.getTracePath() != null) {
                parameters.put(MY_TRACE_PATH, event.getTracePath());
            }
            Object result = ping? null : f.handleEvent(parameters, inputBody, instance);
            float delta = ping? 0 : (float) (System.nanoTime() - begin) / EventEmitter.ONE_MILLISECOND;
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
                            encodeTraceAnnotations(response);
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
            float delta = (float) (System.nanoTime() - begin) / EventEmitter.ONE_MILLISECOND;
            float diff = Float.parseFloat(String.format("%.3f", Math.max(0.0f, delta)));
            ps.setExecutionTime(diff);
            final String replyTo = event.getReplyTo();
            final int status;
            if (e instanceof AppException) {
                status = ((AppException) e).getStatus();
            } else if (e instanceof TimeoutException) {
                status = 408;
            } else if (e instanceof IllegalArgumentException) {
                status = 400;
            } else {
                status = 500;
            }
            Throwable ex = util.getRootCause(e);
            if (f instanceof PoJoMappingExceptionHandler) {
                String error = simplifyCastError(ex.getMessage());
                PoJoMappingExceptionHandler handler = (PoJoMappingExceptionHandler) f;
                try {
                    handler.onError(parentRoute, new AppException(status, error), event, instance);
                } catch (Exception e3) {
                    ps.setUnDelivery(e3.getMessage());
                }
                Map<String, Object> output = new HashMap<>();
                output.put(STATUS, status);
                output.put(EXCEPTION, error);
                inputOutput.put(OUTPUT, output);
                return ps.setException(status, error).setInputOutput(inputOutput);
            }
            Map<String, Object> output = new HashMap<>();
            if (replyTo != null) {
                EventEnvelope response = new EventEnvelope();
                response.setTo(replyTo).setStatus(status).setBody(ex.getMessage());
                response.setException(e);
                response.setExecutionTime(diff);
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
                encodeTraceAnnotations(response);
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

    private String simplifyCastError(String error) {
        if (error == null) {
            return "null";
        } else {
            int sep = error.lastIndexOf(" (");
            return sep > 0 ? error.substring(0, sep) : error;
        }
    }

    private void encodeTraceAnnotations(EventEnvelope response) {
        EventEmitter po = EventEmitter.getInstance();
        Map<String, String> headers = response.getHeaders();
        TraceInfo trace = po.getTrace(parentRoute, instance);
        if (trace != null) {
            Map<String, String> annotations = trace.annotations;
            if (!annotations.isEmpty()) {
                int n = 0;
                for (Map.Entry<String, String> kv : annotations.entrySet()) {
                    n++;
                    headers.put("_" + n, kv.getKey() + "=" + kv.getValue());
                }
                headers.put("_", String.valueOf(n));
            }
        }
    }

}
