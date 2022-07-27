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

import io.vertx.core.Future;
import io.vertx.core.Vertx;
import io.vertx.core.eventbus.EventBus;
import org.platformlambda.core.actuator.ActuatorServices;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.*;
import org.platformlambda.core.services.DistributedTrace;
import org.platformlambda.core.services.Multicaster;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.*;

public class PostOffice {
    private static final Logger log = LoggerFactory.getLogger(PostOffice.class);

    public static final int ONE_MILLISECOND = 1000000;
    public static final String CLOUD_CONNECTOR = "cloud.connector";
    public static final String CLOUD_SERVICES = "cloud.services";
    public static final String DISTRIBUTED_TRACING = "distributed.tracing";
    public static final String ACTUATOR_SERVICES = "actuator.services";
    private static final String[] BUILT_IN = {DISTRIBUTED_TRACING, ACTUATOR_SERVICES};
    private static final String ROUTE_SUBSTITUTION = "route.substitution";
    private static final String ROUTE_SUBSTITUTION_FILE = "route.substitution.file";
    private static final String ROUTE_SUBSTITUTION_FEATURE = "application.feature.route.substitution";
    private static final String MULTICAST_YAML = "multicast.yaml";
    private static final String JOURNAL_YAML = "journal.yaml";
    private static final String APP_GROUP_PREFIX = "monitor-";
    private static final ConcurrentMap<String, FutureEvent> futureEvents = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, String> reRoutes = new ConcurrentHashMap<>();
    private static final ConcurrentMap<Long, TraceInfo> traces = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, ConcurrentMap<String, String>> cloudRoutes = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, String> cloudOrigins = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, Boolean> journaledRoutes = new ConcurrentHashMap<>();
    private final String traceLogHeader;
    private static final PostOffice INSTANCE = new PostOffice();

    private PostOffice() {
        AppConfigReader config = AppConfigReader.getInstance();
        traceLogHeader = config.getProperty("trace.log.header", "X-Trace-Id");
        Platform platform = Platform.getInstance();
        try {
            // start built-in services
            platform.registerPrivate(DISTRIBUTED_TRACING, new DistributedTrace(), 1);
            platform.registerPrivate(ACTUATOR_SERVICES, new ActuatorServices(), 10);
            log.info("Includes {}", Arrays.asList(BUILT_IN));
            // load route substitution table if any
            if (config.getProperty(ROUTE_SUBSTITUTION_FEATURE, "false").equals("true")) {
                loadRouteSubstitution();
            }
            String multicast = config.getProperty(MULTICAST_YAML);
            if (multicast != null) {
                log.info("Loading multicast config from {}", multicast);
                ConfigReader reader = new ConfigReader();
                reader.load(multicast);
                loadMulticast(reader.getMap());
            }
            String journal = config.getProperty(JOURNAL_YAML);
            if (journal != null) {
                log.info("Loading journal config from {}", journal);
                ConfigReader reader = new ConfigReader();
                reader.load(journal);
                loadJournalRoutes(reader.getMap());
            }

        } catch (IOException e) {
            log.error("Unable to start - {}", e.getMessage());
            System.exit(-1);
        }
    }

    public static PostOffice getInstance() {
        return INSTANCE;
    }

    public ConcurrentMap<String, ConcurrentMap<String, String>> getCloudRoutes() {
        return cloudRoutes;
    }

    public ConcurrentMap<String, String> getCloudOrigins() {
        return cloudOrigins;
    }

    public boolean isJournaled(String route) {
        return journaledRoutes.getOrDefault(route, false);
    }

    public List<String> getJournaledRoutes() {
        return new ArrayList<>(journaledRoutes.keySet());
    }

    public String getTraceLogHeader() {
        return traceLogHeader;
    }

    @SuppressWarnings("unchecked")
    private void loadJournalRoutes(Map<String, Object> map) {
        Object o = map.get("journal");
        if (o == null) {
            log.warn("Missing 'journal' section");
        } else if (o instanceof List) {
            List<Object> entries = (List<Object>) o;
            for (Object item: entries) {
                String route = item.toString();
                journaledRoutes.put(route, true);
            }
            log.info("Total {} route{} will be recorded in journal", journaledRoutes.size(),
                    journaledRoutes.size() == 1? "" : "s");
        } else {
            log.warn("Invalid 'journal' section - it must be a list of route names");
        }
    }

    @SuppressWarnings("unchecked")
    private void loadMulticast(Map<String, Object> map) {
        Set<String> list = new HashSet<>();
        Platform platform = Platform.getInstance();
        MultiLevelMap multi = new MultiLevelMap(map);
        int n = 0;
        while (true) {
            String source = (String) multi.getElement("multicast["+n+"].source");
            List<String> targets = (List<String>) multi.getElement("multicast["+n+"].targets");
            if (source == null || targets == null) {
                break;
            }
            n++;
            if (targets.contains(source)) {
                log.error("Cyclic multicast ignored ({} exists in {})", source, targets);
                continue;
            }
            if (list.contains(source)) {
                log.error("Duplicated multicast ignored ({} already defined)", source);
                continue;
            }
            list.add(source);
            try {
                platform.registerPrivate(source, new Multicaster(source, targets), 1);
            } catch (IOException e) {
                log.error("Unable to register multicast {} - {}", source, e.getMessage());
            }
        }
    }

    @SuppressWarnings("unchecked")
    private void loadRouteSubstitution() {
        AppConfigReader config = AppConfigReader.getInstance();
        String location = config.getProperty(ROUTE_SUBSTITUTION_FILE);
        if (location != null) {
            try {
                // try loading from location(s) given in route.substitution.file
                ConfigReader reader = getRouteSubstitutionConfig(location);
                Object o = reader.get(ROUTE_SUBSTITUTION);
                if (o instanceof List) {
                    List<Object> list = (List<Object>) o;
                    for (Object entry: list) {
                        String e = entry instanceof String? (String) entry : entry.toString();
                        int sep = e.indexOf("->");
                        if (sep == -1) {
                            log.error("Invalid route substitution entry {}", e);
                        } else {
                            String route = e.substring(0, sep).trim();
                            String replacement = e.substring(sep+2).trim();
                            addRouteSubstitution(e, route, replacement);
                        }
                    }
                }
            } catch (IOException e) {
                log.error("Unable to load route substitution entries - {}", e.getMessage());
            }
        } else {
            // load route substitution list from application.properties
            Utility util = Utility.getInstance();
            List<String> substitutionList = util.split(config.getProperty(ROUTE_SUBSTITUTION, ""), ", ");
            for (String entry : substitutionList) {
                int colon = entry.indexOf(':');
                String route = entry.substring(0, colon).trim();
                String replacement = entry.substring(colon + 1).trim();
                addRouteSubstitution(entry, route, replacement);
            }
        }
    }

    private void addRouteSubstitution(String entry, String original, String replacement) {
        try {
            addRouteSubstitution(original, replacement);
        } catch (IllegalArgumentException e) {
            log.error("Unable to add route substitution {} - {}", entry, e.getMessage());
        }
    }

    private void addRouteSubstitution(String original, String replacement) {
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
    }

    private ConfigReader getRouteSubstitutionConfig(String location) throws IOException {
        List<String> paths = Utility.getInstance().split(location, ", ");
        for (String p: paths) {
            ConfigReader config = new ConfigReader();
            try {
                config.load(p);
                log.info("Loading route substitutions from {}", p);
                return config;
            } catch (IOException e) {
                log.warn("Skipping {} - {}", p, e.getMessage());
            }
        }
        throw new IOException("Route substitutions not found in "+paths);
    }

    /**
     * User application may obtain the trace ID of the current transaction
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
     * User application may add key-values using this method
     *
     * @param key of the annotation
     * @param value of the annotation
     * @return post office instance
     */
    public PostOffice annotateTrace(String key, String value) {
        TraceInfo info = traces.get(Thread.currentThread().getId());
        if (info != null) {
            info.annotate(key, value);
        }
        return this;
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
     * Start tracing from current route
     *
     * @param traceId to identify a transaction
     * @param tracePath for the transaction
     */
    public void startTracing(String traceId, String tracePath) {
        traces.put(Thread.currentThread().getId(), new TraceInfo(getRoute(), traceId, tracePath));
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
        boolean checkCloud = !endOfRoute && !to.equals(CLOUD_CONNECTOR);
        Platform platform = Platform.getInstance();
        if (to.contains("@")) {
            int at = to.indexOf('@');
            String origin = to.substring(at+1);
            String target = to.substring(0, at);
            if (origin.equals(platform.getOrigin())) {
                if (platform.hasRoute(target)) {
                    return new TargetRoute(platform.getManager(target), false);
                }
            } else if (checkCloud) {
                TargetRoute cloud = getCloudRoute();
                if (cloud != null && (origin.startsWith(APP_GROUP_PREFIX) || cloudOrigins.containsKey(origin))) {
                    return cloud;
                }
            }

        } else {
            if (platform.hasRoute(to)) {
                return new TargetRoute(platform.getManager(to), false);
            } else if (checkCloud) {
                TargetRoute cloud = getCloudRoute();
                if (cloud != null && exists(to)) {
                    return cloud;
                }
            }
        }
        throw new IOException("Route "+to+" not found");
    }

    public TargetRoute getCloudRoute() {
        Platform platform = Platform.getInstance();
        if (platform.hasRoute(CLOUD_CONNECTOR)) {
            return new TargetRoute(platform.getManager(CLOUD_CONNECTOR), true);
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
            throw new IllegalArgumentException("Missing routing path");
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
        long interval = Math.max(1, futureMs - now);
        log.debug("Future event to {} in {} ms", to, interval);
        // schedule the event delivery
        Vertx vertx = Platform.getInstance().getVertx();
        long taskId = vertx.setTimer(interval, (id) -> {
            try {
                PostOffice.getInstance().send(event);
            } catch (IOException e) {
                log.error("Deferred delivery to {} failed - {}", event.getTo(), e.getMessage());
            }
        });
        futureEvents.put(event.getId(), new FutureEvent(event.getTo(), taskId, future));
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
            Utility util = Utility.getInstance();
            Vertx vertx = Platform.getInstance().getVertx();
            vertx.cancelTimer(event.taskId);
            futureEvents.remove(id);
            log.info("Cancel future event {}, {}", event.to, util.date2str(event.time, true));
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
            throw new IllegalArgumentException("Missing routing path");
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
                InboxBase inbox = Inbox.getHolder(cid);
                if (inbox != null) {
                    // Clear broadcast indicator because this is a reply message to an inbox
                    event.setReplyTo(cid).setBroadcastLevel(0);
                    Platform.getInstance().getEventSystem().send(inbox.getId(), event.toBytes());
                    return;
                }
            }
        }
        TargetRoute target = discover(to, event.isEndOfRoute());
        if (target.isCloud()) {
            /*
             * If broadcast, set broadcast level to 3 because the event will be sent
             * to the target by the cloud connector directly
             */
            MultipartPayload.getInstance().outgoing(target.getManager(),
                    event.getBroadcastLevel() > 0? event.setBroadcastLevel(3) : event);
        } else {
            EventBus system = Platform.getInstance().getEventSystem();
            /*
             * The target is the same memory space. We will route it to the cloud connector if broadcast.
             */
            if (Platform.isCloudSelected() && event.getBroadcastLevel() == 1 && !to.equals(CLOUD_CONNECTOR)) {
                TargetRoute cloud = getCloudRoute();
                if (cloud != null) {
                    if (cloud.isCloud()) {
                        /*
                         * If broadcast, set broadcast level to 3 because the event will be sent
                         * to the target by the cloud connector directly
                         */
                        MultipartPayload.getInstance().outgoing(cloud.getManager(), event.setBroadcastLevel(3));
                    }
                } else {
                    // set broadcast level to 3 for language pack clients if any
                    system.send(target.getManager().getRoute(), event.setBroadcastLevel(3).toBytes());
                }
            } else {
                // set broadcast level to 3 for language pack clients if any
                EventEnvelope out = event.getBroadcastLevel() > 0? event.setBroadcastLevel(3) : event;
                system.send(target.getManager().getRoute(), out.toBytes());
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
            throw new IllegalArgumentException("Missing outgoing event");
        }
        String dest = event.getTo();
        if (dest == null) {
            throw new IllegalArgumentException("Missing routing path");
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
        try (Inbox inbox = new Inbox(1)) {
            event.setReplyTo(inbox.getId() + "@" + platform.getOrigin());
            // broadcast is not possible with RPC call
            event.setBroadcastLevel(0);
            if (target.isCloud()) {
                MultipartPayload.getInstance().outgoing(target.getManager(), event);
            } else {
                platform.getEventSystem().send(target.getManager().getRoute(), event.toBytes());
            }
            // wait for response
            inbox.waitForResponse(Math.max(10, timeout));
            EventEnvelope result = inbox.getReply();
            if (result == null) {
                throw new TimeoutException(to + " timeout for " + timeout + " ms");
            }
            if (result.hasError()) {
                Throwable ex = result.getException();
                if (ex != null) {
                    throw new AppException(result.getStatus(), result.getError(), result.getException());
                } else {
                    throw new AppException(result.getStatus(), result.getError());
                }
            }
            return result;
        }
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
    public List<EventEnvelope> request(final List<EventEnvelope> events, long timeout) throws IOException {
        if (events == null || events.isEmpty()) {
            throw new IllegalArgumentException("Missing outgoing events");
        }
        List<TargetRoute> destinations = new ArrayList<>();
        int seq = 0;
        for (EventEnvelope event: events) {
            seq++;
            String dest = event.getTo();
            if (dest == null) {
                throw new IllegalArgumentException("Missing routing path");
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
            // insert sequence number when correlation ID if not present
            // so that the caller can correlate the service responses
            if (event.getCorrelationId() == null) {
                event.setCorrelationId(String.valueOf(seq));
            }
            destinations.add(discover(to, event.isEndOfRoute()));
        }
        Platform platform = Platform.getInstance();
        EventBus system = platform.getEventSystem();
        try (Inbox inbox = new Inbox(events.size())) {
            String replyTo = inbox.getId() + "@" + platform.getOrigin();
            int n = 0;
            for (EventEnvelope event : events) {
                TargetRoute target = destinations.get(n++);
                event.setBroadcastLevel(0);
                event.setReplyTo(replyTo);
                if (target.isCloud()) {
                    MultipartPayload.getInstance().outgoing(target.getManager(), event);
                } else {
                    system.send(target.getManager().getRoute(), event.toBytes());
                }
            }
            // wait for response
            inbox.waitForResponse(Math.max(10, timeout));
            return inbox.getReplies();
        }
    }

    /**
     * Send a request asynchronously with a future result
     * <p>
     * You can retrieve result from the future's
     * onSuccess(EventEnvelope event)
     * onFailure(Throwable timeoutException)
     * <p>
     * You may check return result EventEnvelope's status to handle error cases.
     *
     * @param event to the target
     * @param timeout in milliseconds
     * @return future result
     * @throws IOException in case of error
     */
    public Future<EventEnvelope> asyncRequest(final EventEnvelope event, long timeout) throws IOException {
        if (event == null) {
            throw new IllegalArgumentException("Missing outgoing event");
        }
        String dest = event.getTo();
        if (dest == null) {
            throw new IllegalArgumentException("Missing routing path");
        }
        String to = substituteRouteIfAny(dest);
        event.setTo(to);
        // propagate trace info
        TraceInfo trace = getTrace();
        if (trace != null) {
            if (trace.id != null && trace.path != null) {
                event.setTrace(trace.id, trace.path);
            }
            if (trace.route != null && event.getFrom() == null) {
                event.setFrom(trace.route);
            }
        }
        Platform platform = Platform.getInstance();
        TargetRoute target = discover(to, event.isEndOfRoute());
        AsyncInbox inbox = new AsyncInbox(event.getFrom(), event.getTraceId(), event.getTracePath(), timeout);
        event.setReplyTo(inbox.getId() + "@" + platform.getOrigin());
        event.setBroadcastLevel(0);
        if (target.isCloud()) {
            MultipartPayload.getInstance().outgoing(target.getManager(), event);
        } else {
            platform.getEventSystem().send(target.getManager().getRoute(), event.toBytes());
        }
        return inbox.getFuture();
    }

    /**
     * Send parallel requests asynchronously with a future result
     * <p>
     * You can retrieve result from the future's
     * onSuccess(EventEnvelope event)
     * onFailure(Throwable timeoutException)
     * <p>
     * You may check return result EventEnvelope's status to handle error cases.
     *
     * @param events list of envelopes
     * @param timeout in milliseconds
     * @return future list of result
     * @throws IOException in case of error
     */
    public Future<List<EventEnvelope>> asyncRequest(final List<EventEnvelope> events, long timeout) throws IOException {
        if (events == null || events.isEmpty()) {
            throw new IllegalArgumentException("Missing outgoing event");
        }
        List<TargetRoute> destinations = new ArrayList<>();
        int seq = 0;
        for (EventEnvelope event: events) {
            seq++;
            String dest = event.getTo();
            if (dest == null) {
                throw new IllegalArgumentException("Missing routing path");
            }
            String to = substituteRouteIfAny(dest);
            event.setTo(to);
            // propagate trace info
            TraceInfo trace = getTrace();
            if (trace != null) {
                if (trace.id != null && trace.path != null) {
                    event.setTrace(trace.id, trace.path);
                }
                if (trace.route != null && event.getFrom() == null) {
                    event.setFrom(trace.route);
                }
            }
            // insert sequence number when correlation ID if not present
            // so that the caller can correlate the service responses
            if (event.getCorrelationId() == null) {
                event.setCorrelationId(String.valueOf(seq));
            }
            destinations.add(discover(to, event.isEndOfRoute()));
        }
        Platform platform = Platform.getInstance();
        EventBus system = platform.getEventSystem();
        EventEnvelope first = events.get(0);
        AsyncMultiInbox inbox = new AsyncMultiInbox(events.size(),
                first.getFrom(), first.getTraceId(), first.getTracePath(), timeout);
        String replyTo = inbox.getId() + "@" + platform.getOrigin();
        int n = 0;
        for (EventEnvelope event : events) {
            TargetRoute target = destinations.get(n++);
            event.setBroadcastLevel(0);
            event.setReplyTo(replyTo);
            if (target.isCloud()) {
                MultipartPayload.getInstance().outgoing(target.getManager(), event);
            } else {
                system.send(target.getManager().getRoute(), event.toBytes());
            }
        }
        return inbox.getFuture();
    }

    /**
     * Check if all routes in a list exist
     *
     * @param routes list of service route names
     * @return true or false
     */
    public boolean exists(String... routes) {
        if (routes == null || routes.length == 0) {
            return false;
        }
        if (routes.length == 1) {
            return routeExists(routes[0]);
        }
        for (String r: routes) {
            if (!routeExists(r)) {
                return false;
            }
        }
        return true;
    }
    /**
     * Check if a route or origin-ID exists.
     *
     * The response should be instantaneous using a distributed routing table.
     *
     * @param route name of the target service
     * @return true or false
     */
    private boolean routeExists(String route) {
        if (route == null) {
            return false;
        }
        Platform platform = Platform.getInstance();
        String dest = substituteRouteIfAny(route);
        if (platform.hasRoute(dest)) {
            return true;
        }
        // check if the remote service is reachable
        if (Platform.isCloudSelected() &&
                (platform.hasRoute(ServiceDiscovery.SERVICE_QUERY) || platform.hasRoute(CLOUD_CONNECTOR))) {
            if (dest.contains(".")) {
                ConcurrentMap<String, String> targets = cloudRoutes.get(dest);
                return targets != null && !targets.isEmpty();
            } else {
                return cloudOrigins.containsKey(dest);
            }
        }
        return false;
    }

    /**
     * Search for all application instances that contain the service route
     *
     * @param route for a service
     * @return list of application instance IDs (aka originId)
     */
    public List<String> search(String route) {
        return search(route, false);
    }

    /**
     * Search for all application instances that contain the service route
     *
     * @param route for a service
     * @param remoteOnly if true, search only from service registry. Otherwise, check local registry too.
     * @return list of application instance IDs (aka originId)
     */
    @SuppressWarnings("unchecked")
    public List<String> search(String route, boolean remoteOnly) {
        Platform platform = Platform.getInstance();
        String actualRoute = substituteRouteIfAny(route);
        if (!remoteOnly && platform.hasRoute(actualRoute)) {
            return Collections.singletonList(platform.getOrigin());
        }
        if (Platform.isCloudSelected()) {
            try {
                if (platform.hasRoute(ServiceDiscovery.SERVICE_QUERY) || platform.hasRoute(CLOUD_CONNECTOR)) {
                    EventEnvelope response = request(ServiceDiscovery.SERVICE_QUERY, 3000,
                            new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.SEARCH),
                            new Kv(ServiceDiscovery.ROUTE, actualRoute));
                    if (response.getBody() instanceof List) {
                        return (List<String>) response.getBody();
                    }
                }
            } catch (IOException | TimeoutException e) {
                log.warn("Unable to search route {} - {}", route, e.getMessage());
            } catch (AppException e) {
                // this should not occur
                log.error("Unable to search route {} - ({}) {}", route, e.getStatus(), e.getMessage());
            }
        }
        return Collections.emptyList();
    }

}
