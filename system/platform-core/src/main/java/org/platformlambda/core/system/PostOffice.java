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

import akka.actor.ActorRef;
import akka.actor.ActorSystem;
import akka.actor.Cancellable;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import scala.concurrent.duration.Duration;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

public class PostOffice {
    private static final Logger log = LoggerFactory.getLogger(PostOffice.class);

    public static final int ONE_MILLISECOND = 1000000;
    public static final String CLOUD_CONNECTOR = "cloud.connector";
    public static final String CLOUD_SERVICES = "cloud.services";
    public static final String EVENT_NODE = "event.node";
    private static final String ROUTE_SUBSTITUTION = "route.substitution";
    private static final String ROUTE_SUBSTITUTION_FEATURE = "application.feature.route.substitution";
    private static final CryptoApi crypto = new CryptoApi();
    private static final ConcurrentMap<String, FutureEvent> futureEvents = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, String> reRoutes = new ConcurrentHashMap<>();
    private static final ConcurrentMap<Long, TraceInfo> traces = new ConcurrentHashMap<>();
    private boolean eventNode, substituteRoutes;
    private static final PostOffice instance = new PostOffice();

    private PostOffice() {
        AppConfigReader config = AppConfigReader.getInstance();
        eventNode = EVENT_NODE.equals(config.getProperty(CLOUD_CONNECTOR, EVENT_NODE));
        substituteRoutes = config.getProperty(ROUTE_SUBSTITUTION_FEATURE, "false").equals("true");
        if (substituteRoutes) {
            // load route substitution list from application.properties
            List<String> substitutionList = Utility.getInstance().split(config.getProperty(ROUTE_SUBSTITUTION, ""), ", ");
            for (String entry : substitutionList) {
                int colon = entry.indexOf(':');
                String route = entry.substring(0, colon).trim();
                String replacement = entry.substring(colon + 1).trim();
                try {
                    addRouteSubstitution(route, replacement);
                } catch (IllegalArgumentException e) {
                    log.error("Unable to add route substitution {} - {}", entry, e.getMessage());
                }
            }
        }
    }

    public static PostOffice getInstance() {
        return instance;
    }

    /**
     * User application may obtain the trace ID of the current transaction so it can use for logging
     * or passing to an external system.
     *
     * @return trace ID of the current transaction
     */
    public String getTraceId() {
        TraceInfo info = traces.get(Thread.currentThread().getId());
        return info != null? info.id : null;
    }

    /**
     * User application may obtain the complete trace object for trace ID, path, start time and annotations.
     *
     * @return trace info
     */
    public TraceInfo getTrace() {
        return traces.get(Thread.currentThread().getId());
    }

    /**
     * Get my route name for the currently running service.
     * This is typically used in Role Based Access Control (RBAC) to restrict certain user roles to execute the service.
     * RBAC is a user application's responsibility.
     *
     * @return route name
     */
    public String getRoute() {
        TraceInfo trace = getTrace();
        return trace != null? trace.route : "?";
    }

    /**
     * User application may add key-values using this method so they are shown as annotations
     * in the trace report.
     *
     * @param key of the annotation
     * @param value of the annotation
     */
    public void annotateTrace(String key, String value) {
        TraceInfo info = traces.get(Thread.currentThread().getId());
        if (info != null) {
            info.annotate(key, value);
        }
    }

    /**
     * IMPORTANT: This method is reserved by the system. User application MUST NOT access this.
     * @param route name
     * @param traceId to identify a transaction
     * @param tracePath for the transaction
     */
    public void startTracing(String route, String traceId, String tracePath) {
        traces.put(Thread.currentThread().getId(), new TraceInfo(route, traceId, tracePath));
    }

    /**
     * IMPORTANT: This method is reserved by the system. User application MUST NOT access this.
     * @return current trace info before it is stopped
     */
    public TraceInfo stopTracing() {
        long threadId = Thread.currentThread().getId();
        TraceInfo trace = traces.get(threadId);
        if (trace != null) {
            traces.remove(threadId);
            return trace;
        } else {
            return null;
        }
    }

    /**
     * Service discovery
     *
     * @param to destination
     * @param endOfRoute if no relay is needed
     * @return actor, websocket txPaths or null
     * @throws IOException in case route is not found
     */
    public TargetRoute discover(String to, boolean endOfRoute) throws IOException {
        Platform platform = Platform.getInstance();
        boolean eventNode = ServerPersonality.getInstance().getType() == ServerPersonality.Type.PLATFORM;
        if (to.contains("@")) {
            int at = to.indexOf('@');
            if (at > 0) {
                String origin = to.substring(at+1);
                if (origin.equals(platform.getOrigin())) {
                    String target = to.substring(0, at);
                    if (platform.hasRoute(target)) {
                        return new TargetRoute(platform.getManager(target), false);
                    }
                } else {
                    // resolve from lambda routing table if it is the event node itself
                    if (eventNode) {
                        String txPath = ServiceDiscovery.getTxPath(origin);
                        if (platform.hasRoute(txPath)) {
                            return new TargetRoute(Collections.singletonList(txPath));
                        }
                    }
                }
            }

        } else {
            if (platform.hasRoute(to)) {
                return new TargetRoute(platform.getManager(to), false);
            } else {
                // resolve from lambda routing table if it is the event node itself
                if (eventNode) {
                    List<String> txPaths = ServiceDiscovery.getAllPaths(to);
                    if (txPaths != null) {
                        return new TargetRoute(txPaths);
                    }
                }
            }
        }
        /*
         * route is not found in-memory and local lambda routing table.
         * Try cloud routing if not end of route or not "cloud.connector" itself.
         */
        if (!endOfRoute && !to.equals(CLOUD_CONNECTOR)) {
            TargetRoute cloud = getCloudRoute();
            if (cloud != null) {
                return cloud;
            }
        }
        throw new IOException("Route "+to+" not found");
    }

    public TargetRoute getCloudRoute() {
        if (eventNode) {
            // avoid loopback if it is the event node itself
            if (ServerPersonality.getInstance().getType() != ServerPersonality.Type.PLATFORM) {
                EventNodeConnector connector = EventNodeConnector.getInstance();
                if (connector.isConnected() && connector.isReady()) {
                    return new TargetRoute(Collections.singletonList(connector.getTxPath()));
                }
            }
        } else {
            // cloud connector is not an event node
            Platform platform = Platform.getInstance();
            if (platform.hasRoute(CLOUD_CONNECTOR)) {
                return new TargetRoute(platform.getManager(CLOUD_CONNECTOR), true);
            }
        }
        return null;
    }

    /**
     * Broadcast an event to a target service that may exist in multiple servers
     *
     * @param to target route
     * @param parameters for the event
     * @throws IOException in case of invalid route
     */
    public void broadcast(String to, Kv... parameters) throws IOException {
        deliver(true, to, parameters);
    }

    /**
     * Broadcast an event to a target service that may exist in multiple servers
     *
     * @param to target route
     * @param body message payload
     * @throws IOException in case of invalid route
     */
    public void broadcast(String to, Object body) throws IOException {
        deliver(true, to, body);
    }

    /**
     * Broadcast an event to a target service that may exist in multiple servers
     *
     * @param to target route
     * @param body message payload
     * @param parameters for the event
     * @throws IOException in case of invalid route
     */
    public void broadcast(String to, Object body, Kv... parameters) throws IOException {
        deliver(true, to, body, parameters);
    }

    /**
     * Send an event to a target service in multiple servers
     *
     * @param to target route
     * @param parameters for the event
     * @throws IOException in case of invalid route
     */
    public void send(String to, Kv... parameters) throws IOException {
        deliver(false, to, parameters);
    }

    /**
     * Send an event to a target service
     *
     * @param to target route
     * @param body message payload
     * @throws IOException in case of invalid route
     */
    public void send(String to, Object body) throws IOException {
        deliver(false, to, body);
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
        deliver(false, to, body, parameters);
    }

    private void deliver(boolean bc, String to, Kv... parameters) throws IOException {
        EventEnvelope event = new EventEnvelope().setTo(to);
        if (parameters != null) {
            for (Kv kv: parameters) {
                if (kv.key != null && kv.value != null) {
                    event.setHeader(kv.key, kv.value);
                }
            }
        }
        send(bc? event.setBroadcastLevel(1) : event);
    }

    private void deliver(boolean bc, String to, Object body) throws IOException {
        if (body instanceof Kv) {
            // in case if a single KV is sent
            Kv[] keyValue = new Kv[1];
            keyValue[0] = (Kv) body;
            deliver(bc, to, keyValue);
        } else {
            EventEnvelope event = new EventEnvelope();
            event.setTo(to).setBody(body);
            send(bc? event.setBroadcastLevel(1) : event);
        }
    }

    private void deliver(boolean bc, String to, Object body, Kv... parameters) throws IOException {
        EventEnvelope event = new EventEnvelope().setTo(to).setBody(body);
        if (parameters != null) {
            for (Kv kv: parameters) {
                if (kv.key != null && kv.value != null) {
                    event.setHeader(kv.key, kv.value);
                }
            }
        }
        send(bc? event.setBroadcastLevel(1) : event);
    }

    /**
     * Schedule a future event
     *
     * @param event envelope
     * @param future time
     * @return eventId of the scheduled delivery
     * @throws IOException if route not found or invalid parameters
     */
    public String sendLater(final EventEnvelope event, Date future) throws IOException {
        String dest = event.getTo();
        if (dest == null) {
            throw new IOException("Missing routing path");
        }
        String to = substituteRouteIfAny(dest);
        event.setTo(to);
        // propagate trace info
        TraceInfo trace = getTrace();
        if (trace != null) {
            if (trace.route != null && event.getFrom() == null) {
                event.setFrom(trace.route);
            }
            if (trace.id != null && trace.path != null) {
                event.setTrace(trace.id, trace.path);
            }
        }
        long now = System.currentTimeMillis();
        long futureMs = future.getTime();
        long interval = futureMs - now;
        if (interval < 1) {
            throw new IOException("Future milliseconds must be later than present time");
        }
        // best effort to check if the target can be discovered
        discover(to, event.isEndOfRoute());
        log.debug("Future event to {} in {} ms", to, interval);
        // schedule the event delivery
        ActorSystem system = Platform.getInstance().getEventSystem();
        Cancellable task = system.scheduler().scheduleOnce(Duration.create(interval, TimeUnit.MILLISECONDS), () -> {
            try {
                futureEvents.remove(event.getId());
                send(event);
            } catch (IOException e) {
                log.error("Deferred delivery to {} failed - {}", event.getTo(), e.getMessage());
            }
        }, system.dispatcher());
        futureEvents.put(event.getId(), new FutureEvent(to, task, future));
        return event.getId();
    }

    /**
     * Cancel a future event
     *
     * @param id of the scheduled event
     */
    public void cancelFutureEvent(String id) {
        FutureEvent event = futureEvents.get(id);
        if (event != null) {
            event.task.cancel();
            futureEvents.remove(id);
            log.info("Future event {} to {} at {} canceled", id, event.to, Utility.getInstance().date2str(event.time));
        }
    }

    /**
     * Find all future events
     *
     * @return a list of targets that have scheduled events
     */
    public List<String> getAllFutureEvents() {
        List<String> result = new ArrayList<>();
        for (String id: futureEvents.keySet()) {
            String to = futureEvents.get(id).to;
            if (!result.contains(to)) {
                result.add(to);
            }
        }
        return result;
    }

    /**
     * Get future events for a target
     *
     * @param to target
     * @return event ID list
     */
    public List<String> getFutureEvents(String to) {
        if (to == null || to.length() == 0) {
            throw new IllegalArgumentException("Missing 'to'");
        }
        List<String> result = new ArrayList<>();
        for (String id: futureEvents.keySet()) {
            FutureEvent event = futureEvents.get(id);
            if (event.to.equals(to)) {
                result.add(id);
            }
        }
        return result;
    }

    /**
     * Given an event ID, return the scheduled time
     *
     * @param id of the future event
     * @return scheduled time
     */
    public Date getFutureEventTime(String id) {
        FutureEvent event = futureEvents.get(id);
        return event != null? event.time : null;
    }

    /**
     * Broadcast to multiple target services
     *
     * @param event to the target
     * @throws IOException if invalid route or missing parameters
     */
    public void broadcast(EventEnvelope event) throws IOException {
        send(event.setBroadcastLevel(1));
    }

    public void addRouteSubstitution(String original, String replacement) {
        if (substituteRoutes) {
            Utility util = Utility.getInstance();
            if (util.validServiceName(original) && util.validServiceName(replacement)
                    && original.contains(".") && replacement.contains(".")) {
                if (!original.equals(replacement) && !replacement.equals(reRoutes.get(original))) {
                    if (reRoutes.containsKey(replacement)) {
                        throw new IllegalArgumentException("Nested route substitution not supported");
                    } else {
                        reRoutes.put(original, replacement);
                        log.info("Route substitution: {} -> {}", original, replacement);
                    }
                }
            } else {
                throw new IllegalArgumentException("Invalid route names");
            }
        } else {
            throw new IllegalArgumentException("application.feature.route.substitution is not enabled");
        }
    }

    public void removeRouteSubstitution(String original) {
        if (substituteRoutes) {
            if (reRoutes.containsKey(original)) {
                log.info("Route substitution {} cleared", original);
                reRoutes.remove(original);
            }
        } else {
            throw new IllegalArgumentException("application.feature.route.substitution is not enabled");
        }
    }

    public Map<String, String> getRouteSubstitutionList() {
        return reRoutes;
    }

    public String substituteRouteIfAny(String to) {
        if (to != null) {
            int slash = to.indexOf('@');
            if (slash > 0) {
                String replacement = reRoutes.get(to.substring(0, slash));
                return replacement != null? replacement+to.substring(slash) : to;
            } else {
                String replacement = reRoutes.get(to);
                return replacement != null? replacement : to;
            }
        } else {
            throw new IllegalArgumentException("Missing to");
        }
    }

    /**
     * Send an event to a target service
     *
     * @param event to the target
     * @throws IOException if invalid route or missing parameters
     */
    public void send(final EventEnvelope event) throws IOException {
        String dest = event.getTo();
        if (dest == null) {
            throw new IOException("Missing routing path");
        }
        String to = substituteRouteIfAny(dest);
        event.setTo(to);
        // propagate trace info
        TraceInfo trace = getTrace();
        if (trace != null) {
            if (trace.route != null && event.getFrom() == null) {
                event.setFrom(trace.route);
            }
            if (trace.id != null && trace.path != null) {
                event.setTrace(trace.id, trace.path);
            }
        }
        // is this a reply message?
        int slash = to.indexOf('@');
        if (slash > 0) {
            String origin = to.substring(slash+1);
            if (origin.equals(Platform.getInstance().getOrigin())) {
                String cid = to.substring(0, slash);
                Inbox inbox = Inbox.getHolder(cid);
                if (inbox != null) {
                    event.setReplyTo(cid);
                    ActorRef listener = inbox.getListener();
                    // It is a reply message to an inbox and broadcast level should be cleared
                    listener.tell(event.setBroadcastLevel(0), ActorRef.noSender());
                    return;
                }
            }
        }
        TargetRoute target = discover(to, event.isEndOfRoute());
        if (target.isEventNode()) {
            if (!target.getTxPaths().isEmpty()) {
                /*
                 * Case 1 - An application instance sends a broadcast event thru the Event Node (broadcast level 1)
                 * Case 2 - Event Node sends a broadcast event to target application instances (broadcast level 2)
                 */
                if (event.getBroadcastLevel() > 0) {
                    event.setBroadcastLevel(event.getBroadcastLevel()+1);
                    for (String p: target.getTxPaths()) {
                        MultipartPayload.getInstance().outgoing(p, event);
                    }
                } else {
                    int selected = target.getTxPaths().size() > 1? crypto.nextInt(target.getTxPaths().size()) : 0;
                    MultipartPayload.getInstance().outgoing(target.getTxPaths().get(selected), event);
                }
            }
        } else if (target.isCloud()) {
            /*
             * If broadcast, set broadcast level to 3 because the event will be sent
             * to the target by the cloud connector directly
             */
            MultipartPayload.getInstance().outgoing(target.getActor(),
                    event.getBroadcastLevel() >  0? event.setBroadcastLevel(3) : event);
        } else {
            /*
             * The target is the same memory space. We will route it to the event node or cloud connector if broadcast.
             */
            if (Platform.isCloudSelected() && event.getBroadcastLevel() == 1 && !to.equals(CLOUD_CONNECTOR) &&
                    ServerPersonality.getInstance().getType() != ServerPersonality.Type.PLATFORM) {
                TargetRoute cloud = getCloudRoute();
                if (cloud != null) {
                    if (cloud.isCloud()) {
                        /*
                         * If broadcast, set broadcast level to 3 because the event will be sent
                         * to the target by the cloud connector directly
                         */
                        MultipartPayload.getInstance().outgoing(cloud.getActor(), event.setBroadcastLevel(3));
                    } else if (cloud.isEventNode() && cloud.getTxPaths().size() == 1) {
                        /*
                         * If broadcast, set broadcast level to 2 because the event will be sent
                         * to an event node for further processing
                         */
                        MultipartPayload.getInstance().outgoing(cloud.getTxPaths().get(0), event.setBroadcastLevel(2));
                    }
                } else {
                    // set broadcast level to 3 for language pack clients if any
                    target.getActor().tell(event.setBroadcastLevel(3), ActorRef.noSender());
                }
            } else {
                // set broadcast level to 3 for language pack clients if any
                target.getActor().tell(event.getBroadcastLevel() > 0? event.setBroadcastLevel(3) : event, ActorRef.noSender());
            }
        }
    }

    /**
     * Ping a target service to check for availability and network latency
     *
     * @param to target route
     * @param timeout in milliseconds
     * @return response in an EventEnvelope
     * @throws IOException if invalid route or missing parameters
     * @throws TimeoutException if target does not respond in time
     * @throws AppException if target throws exception
     */
    public EventEnvelope ping(String to, long timeout) throws IOException, TimeoutException, AppException {
        return request(new EventEnvelope().setTo(to), timeout);
    }

    /**
     * Make a request to a target service
     *
     * @param to target route
     * @param timeout in milliseconds
     * @param parameters for the request
     * @return response in an EventEnvelope
     * @throws IOException if invalid route or missing parameters
     * @throws TimeoutException if target does not respond in time
     * @throws AppException if target throws exception
     */
    public EventEnvelope request(String to, long timeout, Kv... parameters) throws IOException, TimeoutException, AppException {
        EventEnvelope event = new EventEnvelope().setTo(to);
        if (parameters != null) {
            for (Kv kv: parameters) {
                if (kv.key != null && kv.value != null) {
                    event.setHeader(kv.key, kv.value);
                }
            }
        }
        return request(event, timeout);
    }

    /**
     * Make a request to a target service
     *
     * @param to target route
     * @param timeout in milliseconds
     * @param body message payload
     * @return response in an EventEnvelope
     * @throws IOException if invalid route or missing parameters
     * @throws TimeoutException if target does not respond in time
     * @throws AppException if target throws exception
     */
    public EventEnvelope request(String to, long timeout, Object body) throws IOException, TimeoutException, AppException {
        if (body instanceof Kv) {
            // in case if a single KV is sent
            Kv kv = (Kv) body;
            if (kv.key != null && kv.value != null) {
                return request(new EventEnvelope().setTo(to).setHeader(kv.key, kv.value), timeout);
            } else {
                return request(new EventEnvelope().setTo(to), timeout);
            }
        } else {
            return request(new EventEnvelope().setTo(to).setBody(body), timeout);
        }
    }

    /**
     * Make a request to a target service
     *
     * @param to target route
     * @param timeout in milliseconds
     * @param body message payload
     * @param parameters for the request
     * @return response in an EventEnvelope
     * @throws IOException if invalid route or missing parameters
     * @throws TimeoutException if target does not respond in time
     * @throws AppException if target throws exception
     */
    public EventEnvelope request(String to, long timeout, Object body, Kv... parameters) throws IOException, TimeoutException, AppException {
        EventEnvelope event = new EventEnvelope().setTo(to).setBody(body);
        if (parameters != null) {
            for (Kv kv: parameters) {
                if (kv.key != null && kv.value != null) {
                    event.setHeader(kv.key, kv.value);
                }
            }
        }
        return request(event, timeout);
    }

    /**
     * Make a request to a target service
     *
     * @param event to the target
     * @param timeout in milliseconds
     * @return response in an EventEnvelope
     * @throws IOException if invalid route or missing parameters
     * @throws TimeoutException if target does not respond in time
     * @throws AppException if target throws exception
     */
    public EventEnvelope request(final EventEnvelope event, long timeout) throws IOException, TimeoutException, AppException {
        if (event == null) {
            throw new IOException("Missing outgoing event");
        }
        String dest = event.getTo();
        if (dest == null) {
            throw new IOException("Missing routing path");
        }
        String to = substituteRouteIfAny(dest);
        event.setTo(to);
        // propagate trace info
        TraceInfo trace = getTrace();
        if (trace != null) {
            if (trace.route != null && event.getFrom() == null) {
                event.setFrom(trace.route);
            }
            if (trace.id != null && trace.path != null) {
                event.setTrace(trace.id, trace.path);
            }
        }
        Platform platform = Platform.getInstance();
        TargetRoute target = discover(to, event.isEndOfRoute());
        Inbox inbox = new Inbox(1);
        event.setReplyTo(inbox.getId()+"@"+platform.getOrigin());
        // broadcast is not possible with RPC call
        event.setBroadcastLevel(0);
        if (target.isEventNode()) {
            if (!target.getTxPaths().isEmpty()) {
                int selected = target.getTxPaths().size() > 1? crypto.nextInt(target.getTxPaths().size()) : 0;
                MultipartPayload.getInstance().outgoing(target.getTxPaths().get(selected), event);
            }
        } else if (target.isCloud()) {
            MultipartPayload.getInstance().outgoing(target.getActor(), event);
        } else {
            target.getActor().tell(event, ActorRef.noSender());
        }
        // wait for response
        inbox.waitForResponse(timeout < 10? 10 : timeout);
        EventEnvelope result = inbox.getReply();
        inbox.close();
        if (result == null) {
            throw new TimeoutException(to+" timeout for "+timeout+" ms");
        }
        if (result.hasError()) {
            throw new AppException(result.getStatus(), result.getError());
        }
        return result;
    }

    /////////////////////////////////////////
    // parallel requests (aka "fork-n-join")
    /////////////////////////////////////////

    /**
     * Make a request to multiple target services in parallel.
     * It does not throw TimeoutException so the caller can receive partial results.
     *
     * @param events of multiple requests
     * @param timeout in milliseconds
     * @return list of EventEnvelope, empty or partial results in case of timeouts
     * @throws IOException in case of error
     */
    public List<EventEnvelope> request(List<EventEnvelope> events, long timeout) throws IOException {
        if (events == null || events.isEmpty()) {
            throw new IOException("Missing outgoing events");
        }
        List<TargetRoute> destinations = new ArrayList<>();
        int seq = 0;
        for (EventEnvelope event: events) {
            seq++;
            String dest = event.getTo();
            if (dest == null) {
                throw new IOException("Missing routing path");
            }
            String to = substituteRouteIfAny(dest);
            event.setTo(to);
            // insert sequence number as correlation ID if not present
            // so that the caller can correlate the service responses
            if (event.getCorrelationId() == null) {
                event.setCorrelationId(String.valueOf(seq));
            }
            destinations.add(discover(to, event.isEndOfRoute()));
        }
        Platform platform = Platform.getInstance();
        Inbox inbox = new Inbox(events.size());
        String replyTo = inbox.getId()+"@"+platform.getOrigin();
        int n = 0;
        for (EventEnvelope event: events) {
            TargetRoute target = destinations.get(n);
            event.setBroadcastLevel(0);
            event.setReplyTo(replyTo);
            if (target.isEventNode()) {
                if (!target.getTxPaths().isEmpty()) {
                    int selected = target.getTxPaths().size() > 1? crypto.nextInt(target.getTxPaths().size()) : 0;
                    MultipartPayload.getInstance().outgoing(target.getTxPaths().get(selected), event);
                }
            } else if (target.isCloud()) {
                MultipartPayload.getInstance().outgoing(target.getActor(), event);
            } else {
                target.getActor().tell(event, ActorRef.noSender());
            }
            n++;
        }
        // wait for response
        inbox.waitForResponse(timeout < 10? 10 : timeout);
        List<EventEnvelope> results = inbox.getReplies();
        inbox.close();
        return results;
    }

    /**
     * Check if a route exists.
     *
     * Normally, the response should be instantaneous because the cloud connector
     * is designed to maintain a distributed routing table.
     *
     * However, when using Event Node as the cloud emulator, it would make a network call
     * to discover the route.
     *
     * @param route name of the target service
     * @return true or false
     */
    public boolean exists(String... route) {
        if (route.length == 0) {
            return false;
        }
        int local = 0;
        Platform platform = Platform.getInstance();
        List<String> remoteServices = new ArrayList<>();
        for (String r: route) {
            String actualRoute = substituteRouteIfAny(r);
            if (platform.hasRoute(actualRoute)) {
                local++;
            } else {
                remoteServices.add(actualRoute);
            }
        }
        // all routes are local
        if (local == route.length && remoteServices.isEmpty()) {
            return true;
        }
        // check if the remote services are reachable
        try {
            if (platform.hasRoute(ServiceDiscovery.SERVICE_QUERY) || platform.hasRoute(CLOUD_CONNECTOR)) {
                if (remoteServices.size() == 1) {
                    EventEnvelope response = request(ServiceDiscovery.SERVICE_QUERY, 3000,
                            new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.FIND),
                            new Kv(ServiceDiscovery.ROUTE, remoteServices.get(0)));
                    if (response.getBody() instanceof Boolean) {
                        return (Boolean) response.getBody();
                    }
                } else {
                    EventEnvelope response = request(ServiceDiscovery.SERVICE_QUERY, 3000,
                            remoteServices,
                            new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.FIND),
                            new Kv(ServiceDiscovery.ROUTE, "*"));
                    if (response.getBody() instanceof Boolean) {
                        return (Boolean) response.getBody();
                    }
                }
            }
        } catch (IOException | TimeoutException e) {
            /*
             * This happens when the network connection is not ready.
             * i.e. cloud.connector is available but system.service.query is not ready.
             */
        } catch (AppException e) {
            // this should not occur
            log.error("Unable to check route {} - ({}) {}", route, e.getStatus(), e.getMessage());
        }
        return false;
    }

}
