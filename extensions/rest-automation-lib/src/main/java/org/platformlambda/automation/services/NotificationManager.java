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

package org.platformlambda.automation.services;

import org.platformlambda.automation.MainModule;
import org.platformlambda.automation.ws.WsGateway;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.ManagedCache;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;

public class NotificationManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(NotificationManager.class);

    public static final String NOTIFICATION_INTERNAL = "notification.manager.internal";
    private static final String APPLICATION = "application";
    private static final String TYPE = "type";
    private static final String TOKEN = "token";
    private static final String LOAD = "load";
    private static final String JOIN = "join";
    private static final String LEAVE = "leave";
    private static final String CLOSE = WsEnvelope.CLOSE;
    private static final String TOPIC = "topic";
    private static final String LIST = "list";
    private static final String PUBLISH = "publish";
    private static final String SUBSCRIBE = "subscribe";
    private static final String UNSUBSCRIBE = "unsubscribe";
    private static final String FINAL = "final";
    private static final String STEP = "step";
    private static final String ORIGIN = "origin";
    private static final String TX_PATH = WsEnvelope.TX_PATH;
    private static final String CLEAR = "clear";

    private static final ManagedCache tokenCache = ManagedCache.createCache("ws.tokens", 30000);
    private static final ManagedCache eventCache = ManagedCache.createCache("ws.conn.events", 2000);
    // topic -> list of websocket return path (aka TX_PATH)
    private static final Map<String, List<String>> subscription = new HashMap<>();
    private static boolean ready = false;

    public NotificationManager() throws IOException {
        final Platform platform = Platform.getInstance();
        final PostOffice po = PostOffice.getInstance();
        final String origin = platform.getOrigin();
        if (!platform.hasRoute(NOTIFICATION_INTERNAL)) {
            LambdaFunction f = (headers, body, instance) -> {
                String type = headers.get(TYPE);
                if (SUBSCRIBE.equals(type) && headers.containsKey(TOPIC) && headers.containsKey(ORIGIN)
                        && headers.containsKey(TX_PATH)) {
                    String topic = headers.get(TOPIC);
                    String target = headers.get(TX_PATH) + "@" + headers.get(ORIGIN);
                    List<String> list = subscription.getOrDefault(topic, new ArrayList<>());
                    list.add(target);
                    subscription.put(topic, list);
                    log.info("{} subscribed to {}", target, topic);
                }
                if (UNSUBSCRIBE.equals(type) && headers.containsKey(TOPIC) && headers.containsKey(ORIGIN)
                        && headers.containsKey(TX_PATH)) {
                    String topic = headers.get(TOPIC);
                    String target = headers.get(TX_PATH) + "@" + headers.get(ORIGIN);
                    List<String> list = subscription.getOrDefault(topic, new ArrayList<>());
                    if (list.contains(target)) {
                        list.remove(target);
                        log.info("{} unsubscribed from {}", target, topic);
                        if (list.isEmpty()) {
                            subscription.remove(topic);
                            log.info("Notification topic {} cleared", topic);
                        } else {
                            subscription.put(topic, list);
                        }
                    }
                }
                if (JOIN.equals(type) && headers.containsKey(ORIGIN)) {
                    if (origin.equals(headers.get(ORIGIN))) {
                        String step = headers.getOrDefault(STEP, "0");
                        if ("0".equals(step)) {
                            // give a moment for newly connected event stream to get ready
                            po.sendLater(new EventEnvelope().setTo(NOTIFICATION_INTERNAL)
                                    .setHeader(TYPE, JOIN).setHeader(ORIGIN, origin).setHeader(STEP, 1),
                                    new Date(System.currentTimeMillis()+5000));
                        }
                        if ("1".equals(step)) {
                            ready = true;
                            log.info("Online");
                            po.broadcast(MainModule.NOTIFICATION_MANAGER, new Kv(TYPE, JOIN), new Kv(STEP, 2),
                                    new Kv(ORIGIN, origin));
                        }

                    } else {
                        String peer = NOTIFICATION_INTERNAL + "@" + headers.get(ORIGIN);
                        String atOrigin = "@" + origin;
                        Map<String, List<String>> loadList = new HashMap<>();
                        for (String t : subscription.keySet()) {
                            List<String> list = subscription.get(t);
                            List<String> partialList = new ArrayList<>();
                            for (String target : list) {
                                if (target.endsWith(atOrigin)) {
                                    partialList.add(target);
                                }
                            }
                            if (!partialList.isEmpty()) {
                                loadList.put(t, partialList);
                            }
                        }
                        if (!loadList.isEmpty()) {
                            po.send(peer, loadList, new Kv(ORIGIN, origin), new Kv(TYPE, LOAD));
                        }
                    }
                }
                if (LOAD.equals(type) && headers.containsKey(ORIGIN) && body instanceof Map) {
                    loadRoutesFromPeer(headers.get(ORIGIN), body);
                }
                if (LEAVE.equals(type) && headers.containsKey(ORIGIN)) {
                    String peer = headers.get(ORIGIN);
                    String key = LEAVE+"/"+peer;
                    if (!eventCache.exists(key)) {
                        eventCache.put(key, true);
                        if (origin.equals(peer)) {
                            ready = false;
                            WsGateway.closeAllConnections();
                            subscription.clear();
                            log.info("Offline");
                        } else {
                            // clear all entries from the ORIGIN
                            clearEntries("@" + peer);
                        }
                    }
                }
                if (CLOSE.equals(type) && headers.containsKey(TX_PATH)) {
                    // clear all entries with the TX_PATH
                    clearEntries(headers.get(TX_PATH));
                }
                return null;
            };
            // create singleton function to serialize updates
            platform.registerPrivate(NOTIFICATION_INTERNAL, f, 1);
        }
    }

    private void clearEntries(String path) {
        boolean atPath = path.startsWith("@");
        List<String> topicToDelete = new ArrayList<>();
        for (String topic: subscription.keySet()) {
            List<String> list = subscription.get(topic);
            List<String> filtered = new ArrayList<>();
            for (String target: list) {
                boolean found = false;
                if (atPath) {
                    if (target.endsWith(path)) {
                        found = true;
                    }
                } else {
                    if (target.equals(path)) {
                        found = true;
                    }
                }
                if (found) {
                    log.info("{} unsubscribed from {}", target, topic);
                } else {
                    filtered.add(target);
                }
            }
            if (filtered.size() < list.size()) {
                if (filtered.isEmpty()) {
                    topicToDelete.add(topic);
                } else {
                    subscription.put(topic, filtered);
                }
            }
        }
        for (String topic : topicToDelete) {
            subscription.remove(topic);
            log.info("Notification topic {} cleared", topic);
        }
    }

    @SuppressWarnings("unchecked")
    private void loadRoutesFromPeer(String peer, Object body) {
        log.info("Loading entries from {}", peer);
        Map<String, List<String>> loadList = (Map<String, List<String>>) body;
        for (String topic: loadList.keySet()) {
            List<String> list = subscription.getOrDefault(topic, new ArrayList<>());
            List<String> additions = loadList.get(topic);
            int n = 0;
            for (String target: additions) {
                if (!list.contains(target)) {
                    list.add(target);
                    log.info("{} subscribed to {}", target, topic);
                    n++;
                }
            }
            if (n > 0) {
                subscription.put(topic, list);
            }
        }
    }

    public static void clear(String token) {
        if (tokenCache.exists(token)) {
            log.debug("Clearing token {}", token);
        }
        tokenCache.remove(token);
    }

    public static String getApplication(String token) {
        Object app = tokenCache.get(token);
        return app instanceof String? (String) app : null;
    }

    public static boolean isReady() {
        return ready;
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        String type = headers.get(TYPE);
        String origin = Platform.getInstance().getOrigin();
        PostOffice po = PostOffice.getInstance();
        if (TOKEN.equals(type) && headers.containsKey(TOKEN) && headers.containsKey(APPLICATION)) {
            tokenCache.put(headers.get(TOKEN), headers.get(APPLICATION));
            return true;
        }
        if (CLEAR.equals(type) && headers.containsKey(TOKEN)) {
            clear(headers.get(TOKEN));
            return true;
        }
        if (LIST.equals(type)) {
            if (headers.containsKey(TOPIC)) {
                String topic = headers.get(TOPIC);
                List<String> list = subscription.get(topic);
                if (list == null) {
                    throw new AppException(404, "Notification topic "+topic+" not found");
                }
                Map<String, List<String>> result = new HashMap<>();
                for (String entry: list) {
                    int at = entry.indexOf('@');
                    if (at > 0) {
                        String path = entry.substring(0, at);
                        String source = entry.substring(at + 1);
                        List<String> pathList = result.getOrDefault(source, new ArrayList<>());
                        pathList.add(path);
                        result.put(source, pathList);
                    }
                }
                return result;

            } else {
                List<String> topics = new ArrayList<>(subscription.keySet());
                if (topics.size() > 1) {
                    Collections.sort(topics);
                }
                return topics;
            }
        }
        if (SUBSCRIBE.equals(type) || UNSUBSCRIBE.equals(type)) {
            po.send(new EventEnvelope().setTo(NOTIFICATION_INTERNAL).setHeaders(headers).setBody(body));
            return true;
        }
        if (PUBLISH.equals(type) && headers.containsKey(TOPIC) && body instanceof String) {
            boolean isFinal = headers.containsKey(FINAL);
            String topic = headers.get(TOPIC);
            Set<String> peers = new HashSet<>();
            List<String> target = new ArrayList<>(subscription.getOrDefault(topic, new ArrayList<>()));
            for (String t : target) {
                int at = t.indexOf('@');
                if (at > 0) {
                    String txPath = t.substring(0, at);
                    String peerOrigin = t.substring(at + 1);
                    if (origin.equals(peerOrigin)) {
                        po.send(txPath, body);
                    } else {
                        if (!isFinal) {
                            peers.add(peerOrigin);
                        }
                    }
                }
            }
            for (String p : peers) {
                po.send(MainModule.NOTIFICATION_MANAGER + "@" + p, body, new Kv(FINAL, true),
                        new Kv(TYPE, PUBLISH), new Kv(TOPIC, topic));
            }
            return true;
        }
        if (CLOSE.equals(type) && headers.containsKey(TX_PATH)) {
            po.send(NOTIFICATION_INTERNAL, new Kv(TYPE, CLOSE), new Kv(TX_PATH, headers.get(TX_PATH)));
            return true;
        }
        if (JOIN.equals(type)) {
            po.send(new EventEnvelope().setHeaders(headers).setTo(NOTIFICATION_INTERNAL));
            return true;
        }
        if (LEAVE.equals(type) && headers.containsKey(ORIGIN)) {
            po.send(NOTIFICATION_INTERNAL, new Kv(TYPE, LEAVE), new Kv(ORIGIN, headers.get(ORIGIN)));
            return true;
        }
        log.warn("Unknown event dropped - {}", headers);
        return false;
    }

}
