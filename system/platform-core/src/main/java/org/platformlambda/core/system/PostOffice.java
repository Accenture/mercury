package org.platformlambda.core.system;

import io.vertx.core.Future;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.TraceInfo;

import java.io.IOException;
import java.util.Date;
import java.util.List;
import java.util.Map;

public class PostOffice {

    private static final String MY_ROUTE = "my_route";
    private static final String MY_TRACE_ID = "my_trace_id";
    private static final String MY_TRACE_PATH = "my_trace_path";

    private final String myRoute;
    private final String myTraceId;
    private final String myTracePath;
    private final int instance;

    private static final EventEmitter po = EventEmitter.getInstance();

    /**
     * Create a PostOffice instance
     *
     * @param headers in the input arguments to a user function
     * @param instance for the worker serving the current transaction
     */
    public PostOffice(Map<String, String> headers, int instance) {
        myRoute = headers.get(MY_ROUTE);
        myTraceId = headers.get(MY_TRACE_ID);
        myTracePath = headers.get(MY_TRACE_PATH);
        this.instance = instance;
    }

    public PostOffice(String myRoute, String myTraceId, String myTracePath) {
        this.myRoute = myRoute;
        this.myTraceId = myTraceId;
        this.myTracePath = myTracePath;
        this.instance = 0;
    }

    /**
     * Get my route name for the currently running service.
     * This is typically used in Role Based Access Control (RBAC) to restrict certain user roles to execute the service.
     * RBAC is a user application's responsibility.
     *
     * @return route name
     */
    public String getRoute() {
        return myRoute;
    }

    /**
     * User application may obtain the trace ID of the current transaction
     *
     * @return trace ID of the current transaction
     */
    public String getTraceId() {
        return myTraceId;
    }

    /**
     * User application may obtain the trace path of the current transaction
     *
     * @return trace path of the current transaction
     */
    public String getTracePath() {
        return myTracePath;
    }

    /**
     * User application may obtain the complete trace object for trace ID, path, start time and annotations.
     *
     * @return trace info
     */
    public TraceInfo getTrace() {
        return po.getTrace(myRoute, instance);
    }

    /**
     * Check if a function with the named route exists
     *
     * @param route name
     * @return true or false
     */
    public boolean exists(String... route) {
        return po.exists(route);
    }

    /**
     * User application may add key-values using this method
     * <p>
     * Please note that trace annotation feature is available inside a user function
     * that implements LambdaFunction, TypedLambdaFunction or KotlinLambdaFunction
     * <p>
     * Trace annotation is disabled when called directly. e.g. in a unit test.
     *
     * @param key of the annotation
     * @param value of the annotation
     * @return this PostOffice instance
     */
    public PostOffice annotateTrace(String key, String value) {
        TraceInfo trace = getTrace();
        if (trace != null) {
            trace.annotate(key, value);
        }
        return this;
    }

    /**
     * Broadcast an event to multiple servers holding the target route
     * <p>
     * This method is only relevant when running in a minimalist service mesh
     *
     * @param to target route
     * @param parameters for the event
     * @throws IOException in case of invalid route
     */
    public void broadcast(String to, Kv... parameters) throws IOException {
        send(touch(po.asEnvelope(to, null, parameters)).setBroadcastLevel(1));
    }

    /**
     * Broadcast an event to multiple servers holding the target route
     * <p>
     * This method is only relevant when running in a minimalist service mesh
     *
     * @param to target route
     * @param body message payload
     * @throws IOException in case of invalid route
     */
    public void broadcast(String to, Object body) throws IOException {
        if (body instanceof Kv) {
            // in case if a single KV is sent
            Kv[] keyValue = new Kv[1];
            keyValue[0] = (Kv) body;
            send(touch(po.asEnvelope(to, null, keyValue)).setBroadcastLevel(1));
        } else {
            send(touch(po.asEnvelope(to, body)).setBroadcastLevel(1));
        }
    }

    /**
     * Broadcast an event to multiple servers holding the target route
     * <p>
     * This method is only relevant when running in a minimalist service mesh
     *
     * @param to target route
     * @param body message payload
     * @param parameters for the event
     * @throws IOException in case of invalid route
     */
    public void broadcast(String to, Object body, Kv... parameters) throws IOException {
        send(touch(po.asEnvelope(to, body, parameters)).setBroadcastLevel(1));
    }

    /**
     * Send an event to a target service in multiple servers
     *
     * @param to target route
     * @param parameters for the event
     * @throws IOException in case of invalid route
     */
    public void send(String to, Kv... parameters) throws IOException {
        send(touch(po.asEnvelope(to, null, parameters)));
    }

    /**
     * Send an event to a target service
     *
     * @param to target route
     * @param body message payload
     * @throws IOException in case of invalid route
     */
    public void send(String to, Object body) throws IOException {
        if (body instanceof Kv) {
            // in case if a single KV is sent
            Kv[] keyValue = new Kv[1];
            keyValue[0] = (Kv) body;
            send(touch(po.asEnvelope(to, null, keyValue)));
        } else {
            send(touch(po.asEnvelope(to, body)));
        }
    }

    /**
     * Send an event to a target service
     *
     * @param to target route
     * @param body message payload
     * @param parameters for the event
     * @throws IOException in case of invalid route
     */
    public void send(String to, Object body, Kv... parameters) throws IOException {
        send(touch(po.asEnvelope(to, body, parameters)));
    }

    /**
     * Send an event to a target service
     *
     * @param event to the target
     * @throws IOException if invalid route or missing parameters
     */
    public void send(final EventEnvelope event) throws IOException {
        po.send(touch(event));
    }

    /**
     * Schedule a future event
     *
     * @param event envelope
     * @param future time
     * @return eventId of the scheduled delivery
     */
    public String sendLater(final EventEnvelope event, Date future) {
        return po.sendLater(touch(event), future);
    }

    /**
     * Broadcast to multiple target services
     *
     * @param event to the target
     * @throws IOException if invalid route or missing parameters
     */
    public void broadcast(final EventEnvelope event) throws IOException {
        po.broadcast(touch(event));
    }

    /**
     * This method allows your app to send an async or RPC request to another application instance
     * <p>
     * You can retrieve result from the future's
     * onSuccess(EventEnvelope event)
     * <p>
     *     Note that onFailure is not required because exceptions are returned as regular event.
     *
     * @param event to be sent to a peer application instance
     * @param timeout to abort the request
     * @param headers optional security headers such as "Authorization"
     * @param eventEndpoint fully qualified URL such as http://domain:port/api/event
     * @param rpc if true, the target service will return a response.
     *            Otherwise, a response with status=202 will be returned to indicate that the event will be delivered.
     * @return response event
     * @throws IOException in case of routing error
     */
    public Future<EventEnvelope> asyncRequest(final EventEnvelope event, long timeout,
                                              Map<String, String> headers,
                                              String eventEndpoint, boolean rpc) throws IOException {
        return po.asyncRequest(touch(event), timeout, headers, eventEndpoint, rpc);
    }

    /**
     * Send a request asynchronously with a future result
     * <p>
     * You can retrieve result from the future's
     * onSuccess(EventEnvelope event)
     * onFailure(Throwable timeoutException)
     * <p>
     * IMPORTANT: This is an asynchronous RPC using Future.
     * You should NOT use this API in your function that will be run as a coroutine.
     * <p>
     * If you annotate your LambdaFunction using "CoroutineRunner", please use "CoroutineBridge" to bridge your Java
     * code to a kotlin suspend function that calls "awaitRequest" in "FastRPC.kt".
     *
     * @param event to the target
     * @param timeout in milliseconds
     * @return future results
     * @throws IOException in case of routing error
     */
    public Future<EventEnvelope> asyncRequest(final EventEnvelope event, long timeout) throws IOException {
        return po.asyncRequest(touch(event), timeout);
    }

    /**
     * Send a request asynchronously with a future result
     * <p>
     * You can retrieve result from the future's
     * onSuccess(EventEnvelope event)
     * onFailure(Throwable timeoutException)
     * <p>
     * IMPORTANT: This is an asynchronous RPC using Future.
     * You should NOT use this API in your function that will be run as a coroutine.
     * <p>
     * If you annotate your LambdaFunction using "CoroutineRunner", please use "CoroutineBridge" to bridge your Java
     * code to a kotlin suspend function that calls "awaitRequest" in "FastRPC.kt".
     *
     * @param event to the target
     * @param timeout in milliseconds
     * @param timeoutException if true, return TimeoutException in onFailure method. Otherwise, return timeout result.
     * @return future result
     * @throws IOException in case of routing error
     */
    public Future<EventEnvelope> asyncRequest(final EventEnvelope event, long timeout, boolean timeoutException)
            throws IOException {
        return po.asyncRequest(touch(event), timeout, timeoutException);
    }

    /**
     * Send parallel requests asynchronously with a future result
     * <p>
     * You can retrieve result from the future's
     * onSuccess(EventEnvelope event)
     * onFailure(Throwable timeoutException)
     * <p>
     * IMPORTANT: This is an asynchronous RPC using Future.
     * If you want sequential non-blocking RPC, please implement KotlinLambdaFunction in your function
     * and use the awaitRequest API in FastRPC.kt
     *
     * @param events list of envelopes
     * @param timeout in milliseconds
     * @return future list of results
     * @throws IOException in case of error
     */
    public Future<List<EventEnvelope>> asyncRequest(final List<EventEnvelope> events, long timeout) throws IOException {
        events.forEach(this::touch);
        return po.asyncRequest(events, timeout, true);
    }

    private EventEnvelope touch(final EventEnvelope event) {
        if (event.getFrom() == null) {
            event.setFrom(myRoute);
        }
        if (event.getTraceId() == null) {
            event.setTraceId(myTraceId);
        }
        if (event.getTracePath() == null) {
            event.setTracePath(myTracePath);
        }
        return event;
    }

}
