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
import java.util.ArrayList;
import java.util.Collections;
import java.util.Date;
import java.util.List;
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
    private static final PostOffice instance = new PostOffice();
    private static final CryptoApi crypto = new CryptoApi();
    private static final ConcurrentMap<String, FutureEvent> futureEvents = new ConcurrentHashMap<>();
    private boolean isEventNode;

    private PostOffice() {
        AppConfigReader config = AppConfigReader.getInstance();
        isEventNode = config.getProperty(CLOUD_CONNECTOR, "event.node").equals("event.node");
    }

    public static PostOffice getInstance() {
        return instance;
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

    public TargetRoute getCloudRoute() throws IOException {
        if (isEventNode) {
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
        send(bc? event.setBroadcast() : event);
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
            send(bc? event.setBroadcast() : event);
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
        send(bc? event.setBroadcast() : event);
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
        String to = event.getTo();
        if (to == null) {
            throw new IOException("Missing routing path");
        }
        long now = System.currentTimeMillis();
        long futureMs = future.getTime();
        long interval = futureMs - now;
        if (interval < 1) {
            throw new IOException("Future milliseconds must be later than present time");
        }
        // best effort to check if the target can be discovered
        discover(to, event.isEndOfRoute());
        log.info("Future event to {} in {} ms", to, interval);
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
        send(event.setBroadcast());
    }

    /**
     * Send an event to a target service
     *
     * @param event to the target
     * @throws IOException if invalid route or missing parameters
     */
    public void send(final EventEnvelope event) throws IOException {

        String to = event.getTo();
        if (to == null) {
            throw new IOException("Missing routing path");
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
                    listener.tell(event.stopBroadcast(), ActorRef.noSender());
                    return;
                }
            }
        }
        TargetRoute target = discover(to, event.isEndOfRoute());
        if (target.isWebsocket()) {
            /*
             * The target is for an event node
             */
            if (!target.paths.isEmpty()) {
                if (event.isBroadcast()) {
                    // clear the broadcast flag because this is the event node
                    for (String p: target.paths) {
                        MultipartPayload.getInstance().outgoing(p, event.stopBroadcast().setEndOfRoute(), true);
                    }
                } else {
                    int selected = target.paths.size() > 1? crypto.nextInt(target.paths.size()) : 0;
                    MultipartPayload.getInstance().outgoing(target.paths.get(selected), event.stopBroadcast().setEndOfRoute(), true);
                }
            }
        } else if (target.isRemote()) {
            /*
             * This target is for cloud connector
             */
            MultipartPayload.getInstance().outgoing(target.actor, event, true);
        } else {
            /*
             * The target is the same memory space. We will route it to the event node or cloud connector if broadcast.
             */
            if (Platform.isCloudSelected() && event.isBroadcast() && !to.equals(CLOUD_CONNECTOR) &&
                    ServerPersonality.getInstance().getType() != ServerPersonality.Type.PLATFORM) {
                TargetRoute cloud = getCloudRoute();
                if (cloud != null) {
                    if (cloud.isRemote()) {
                        // this is a cloud connector so tell it to broadcast only once to avoid loopback
                        MultipartPayload.getInstance().outgoing(cloud.actor, event, true);
                    } else if (cloud.isWebsocket() && cloud.paths.size() == 1) {
                        // routing to an event node so we keep the broadcast flag in the event
                        MultipartPayload.getInstance().outgoing(cloud.paths.get(0), event, false);
                    }
                } else {
                    log.error("Event to {} dropped because cloud connection cannot be resolved", event.getTo());
                }
            } else {
                target.actor.tell(event.stopBroadcast(), ActorRef.noSender());
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
    public EventEnvelope request(EventEnvelope event, long timeout) throws IOException, TimeoutException, AppException {
        if (event == null) {
            throw new IOException("Missing outgoing event");
        }
        String to = event.getTo();
        if (to == null) {
            throw new IOException("Missing routing path");
        }
        Platform platform = Platform.getInstance();
        TargetRoute target = discover(to, event.isEndOfRoute());
        Inbox inbox = new Inbox(1);
        event.setReplyTo(inbox.getId()+"@"+platform.getOrigin());
        event.stopBroadcast();
        if (target.isWebsocket()) {
            if (!target.paths.isEmpty()) {
                int selected = target.paths.size() > 1? crypto.nextInt(target.paths.size()) : 0;
                MultipartPayload.getInstance().outgoing(target.paths.get(selected), event, true);
            }
        } else if (target.isRemote()) {
            // set end of route to prevent loop-back
            MultipartPayload.getInstance().outgoing(target.actor, event, true);
        } else {
            target.actor.tell(event, ActorRef.noSender());
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
            String to = event.getTo();
            if (to == null) {
                throw new IOException("Missing routing path");
            }
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
            event.stopBroadcast();
            event.setReplyTo(replyTo);
            if (target.isWebsocket()) {
                if (!target.paths.isEmpty()) {
                    int selected = target.paths.size() > 1? crypto.nextInt(target.paths.size()) : 0;
                    MultipartPayload.getInstance().outgoing(target.paths.get(selected), event, true);
                }
            } else if (target.isRemote()) {
                MultipartPayload.getInstance().outgoing(target.actor, event, true);
            } else {
                target.actor.tell(event, ActorRef.noSender());
            }
            n++;
        }
        // wait for response
        inbox.waitForResponse(timeout < 10? 10 : timeout);
        List<EventEnvelope> results = inbox.getReplies();
        inbox.close();
        return results;
    }

}
