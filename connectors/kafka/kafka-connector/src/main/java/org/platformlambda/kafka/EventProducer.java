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

package org.platformlambda.kafka;

import org.apache.kafka.clients.producer.KafkaProducer;
import org.apache.kafka.clients.producer.ProducerConfig;
import org.apache.kafka.clients.producer.ProducerRecord;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

public class EventProducer implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(EventProducer.class);

    private static final CryptoApi crypto = new CryptoApi();
    private static final String MANAGER = KafkaSetup.MANAGER;
    private static final String SERVICE_MONITOR = "service.monitor";
    private static final String PRESENCE_MONITOR = "presence.monitor";
    private static final String TO = "to";
    private static final String BROADCAST = "broadcast";
    private static final String SERVICE_REGISTRY = ServiceDiscovery.SERVICE_REGISTRY;
    private static final String TRUE = "true";
    private static final String FALSE = "false";
    private static final String TYPE = "type";
    private static final String INIT = "init";
    private static final String LOOP_BACK = "loopback";
    private static final String STOP = "stop";
    private static final String REPLY_TO = "reply_to";
    private static final String PING = "ping";
    private static final String PONG = "pong";
    private static final String ORIGIN = "origin";
    private static final String TARGET = "target";
    private static final String EXISTS = "exists";
    // static because this is a shared lambda function
    private static long lastStarted = 0;
    private static long lastActive = System.currentTimeMillis();
    private static KafkaProducer<String, byte[]> producer;
    private static boolean isServiceMonitor, ready = false, abort = false;
    private static long seq = 0, totalEvents = 0;
    private static Properties properties;

    public EventProducer(Properties base) {
        final ProducerWatcher monitor = new ProducerWatcher();
        monitor.start();
        Properties prop = new Properties();
        prop.putAll(base);
        prop.put(ProducerConfig.ACKS_CONFIG, "1"); // Setting to "1" ensures that the message is received by the leader
        prop.put(ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.StringSerializer.class);
        prop.put(ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.ByteArraySerializer.class);
        prop.put(ProducerConfig.MAX_BLOCK_MS_CONFIG, 15000);
        EventProducer.properties = prop;
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = TRUE.equals(reader.getProperty(SERVICE_MONITOR, FALSE));
        Runtime.getRuntime().addShutdownHook(new Thread(()->{
            closeProducer();
            monitor.shutdown();
        }));
    }

    public static long getLastStarted() {
        return lastStarted == 0? System.currentTimeMillis() : lastStarted;
    }

    public static long getLastActive() {
        return lastActive;
    }

    private void startProducer() {
        if (producer == null) {
            // create unique ID from origin ID by dropping date prefix and adding a sequence suffix
            String id = Platform.getInstance().getOrigin()+"p"+(++seq);
            properties.put(ProducerConfig.CLIENT_ID_CONFIG, id.substring(8));
            producer = new KafkaProducer<>(properties);
            lastStarted = System.currentTimeMillis();
            log.info("Producer {} ready", properties.getProperty(ProducerConfig.CLIENT_ID_CONFIG));
        }
    }

    private void closeProducer() {
        if (producer != null) {
            try {
                producer.close();
                log.info("Producer {} released, delivered: {}", properties.getProperty(ProducerConfig.CLIENT_ID_CONFIG), totalEvents);
            } catch (Exception e) {
                // ok to ignore
            }
            producer = null;
            lastStarted = 0;
            totalEvents = 0;
        }
        lastActive = System.currentTimeMillis();
    }

    private boolean validRegistry() {
        if (ready) {
            return true;
        }
        if (!abort) {
            try {
                Platform.getInstance().waitForProvider(SERVICE_REGISTRY, 60);
                ready = true;
                return true;
            } catch (TimeoutException e) {
                abort = true;
            }
        }
        return false;
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        // check for critical resource
        if (!validRegistry()) {
            log.error("abort because {} is not available", SERVICE_REGISTRY);
            return false;
        }
        PostOffice po = PostOffice.getInstance();
        String type = headers.get(TYPE);
        if (type != null) {
            if (INIT.equals(type) && !isServiceMonitor) {
                initializeConsumer();
            }
            if (LOOP_BACK.equals(type) && headers.containsKey(REPLY_TO) && headers.containsKey(ORIGIN)) {
                sendLoopback(headers.get(REPLY_TO), headers.get(ORIGIN));
            }
            if (PING.equals(type) && headers.containsKey(REPLY_TO) && headers.containsKey(TARGET) && headers.containsKey(ORIGIN)) {
                sendPing(headers.get(REPLY_TO), headers.get(TARGET), headers.get(ORIGIN));
            }
            if (STOP.equals(type)) {
                closeProducer();
            }
            return true;
        }
        if (headers.containsKey(TO) && body instanceof byte[]) {
            List<String> undelivered = new ArrayList<>();
            List<String> destinations = getDestinations(headers.get(TO), headers.containsKey(BROADCAST));
            if (destinations != null) {
                String uuid = Utility.getInstance().getUuid();
                byte[] payload = (byte[]) body;
                for (String dest : destinations) {
                    startProducer();
                    try {
                        /*
                         * Automatic segmentation happens at the PostOffice level
                         * so this outgoing payload may be a whole event or a block of it.
                         *
                         * The EventConsumer at the receiving side will reconstruct the payload if needed.
                         */
                        long t1 = System.currentTimeMillis();
                        producer.send(new ProducerRecord<>(dest, uuid, payload)).get(20, TimeUnit.SECONDS);
                        long diff = System.currentTimeMillis() - t1;
                        if (diff > 5000) {
                            log.warn("Kafka is slow in sending event - took {} ms", diff);
                        }
                        totalEvents++;
                        lastActive = System.currentTimeMillis();
                    } catch (InterruptedException | ExecutionException | TimeoutException e) {
                        log.error("Unable to send message to {} - {}", dest, e.getMessage());
                        undelivered.add(dest);
                        closeProducer();
                    }
                }
            }
            if (!undelivered.isEmpty()) {
                // decode the message to see if there is a reply address
                String targets = trimBrackets(undelivered);
                EventEnvelope event = new EventEnvelope();
                event.load((byte[]) body);
                if (event.getReplyTo() != null) {
                    EventEnvelope error = new EventEnvelope();
                    error.setTo(event.getReplyTo()).setStatus(404).setBody("Route " + targets + " not found");
                    po.send(error);
                } else {
                    log.warn("Event dropped because route {} not found", targets);
                }
            }
        }
        return true;
    }

    private String trimBrackets(List<String> routes) {
        if (routes.size() ==1) {
            return routes.get(0);
        } else {
            return routes.toString().replace("[", "").replace("]", "");
        }
    }

    private List<String> getDestinations(String to, boolean broadcast) {
        // broadcast to all presence monitor instances?
        if (to.equals("*")) {
            String namespace = Platform.getInstance().getNamespace();
            return Collections.singletonList(namespace == null? PRESENCE_MONITOR : PRESENCE_MONITOR + "." + namespace);
        }
        // normal message
        Platform platform = Platform.getInstance();
        if (to.contains("@")) {
            // direct addressing
            String target = to.substring(to.indexOf('@') + 1);
            if (isServiceMonitor || target.equals(platform.getOrigin())) {
                return Collections.singletonList(target);
            } else if (ServiceRegistry.destinationExists(target)) {
                return Collections.singletonList(target);
            }
        } else {
            if (!broadcast && platform.hasRoute(to)) {
                // use local routing
                return Collections.singletonList(platform.getOrigin());
            }
            Map<String, String> targets = ServiceRegistry.getDestinations(to);
            if (targets != null) {
                List<String> available = new ArrayList<>(targets.keySet());
                if (!available.isEmpty()) {
                    if (broadcast) {
                        return available;
                    } else {
                        String target = getNextAvailable(available);
                        if (target != null) {
                            return Collections.singletonList(target);
                        }
                    }
                }
            }
        }
        return null;
    }

    private String getNextAvailable(List<String> available) {
        if (isServiceMonitor) {
            // skip validation if it is a service monitor
            return available.get(available.size() > 1? crypto.nextInt(available.size()) : 0);
        }
        if (available.size() == 1) {
            return ServiceRegistry.destinationExists(available.get(0))? available.get(0) : null;
        } else {
            Collections.shuffle(available);
            for (String target: available) {
                if (ServiceRegistry.destinationExists(target)) {
                    return target;
                }
            }
            return null;
        }
    }

    private void initializeConsumer() {
        startProducer();
        try {
            String origin = Platform.getInstance().getOrigin();
            String uuid = Utility.getInstance().getUuid();
            EventEnvelope direct = new EventEnvelope().setTo(ServiceDiscovery.SERVICE_REGISTRY).setHeader(TYPE, INIT);
            producer.send(new ProducerRecord<>(origin, uuid, direct.toBytes())).get(10000, TimeUnit.MILLISECONDS);
            totalEvents++;
            lastActive = System.currentTimeMillis();
            log.info("Tell event consumer to start with {}", origin);

        } catch (IOException | InterruptedException | ExecutionException | TimeoutException e) {
            log.warn("Unable to initialize consumer - {}", e.getMessage());
            closeProducer();
        }
    }

    private void sendLoopback(String replyTo, String origin) {
        startProducer();
        try {
            String uuid = Utility.getInstance().getUuid();
            EventEnvelope direct = new EventEnvelope().setTo(replyTo).setHeader(TYPE, PONG).setBody(true);
            producer.send(new ProducerRecord<>(origin, uuid, direct.toBytes())).get(10000, TimeUnit.MILLISECONDS);
            totalEvents++;
            lastActive = System.currentTimeMillis();

        } catch (IOException | InterruptedException | ExecutionException | TimeoutException e) {
            log.warn("Unable to send direct message to {} - {}", origin, e.getMessage());
            closeProducer();
        }
    }

    private void sendPing(String replyTo, String target, String origin) throws TimeoutException, IOException, AppException {
        // guarantee that target topic exists
        EventEnvelope response = PostOffice.getInstance().request(MANAGER, 10000, new Kv(TYPE, EXISTS), new Kv(ORIGIN, target));
        if (response.getBody() instanceof Boolean) {
            if (((Boolean) response.getBody())) {
                startProducer();
                try {
                    String uuid = Utility.getInstance().getUuid();
                    EventEnvelope direct = new EventEnvelope().setTo(PostOffice.CLOUD_CONNECTOR)
                            .setHeader(TYPE, LOOP_BACK).setHeader(ORIGIN, origin).setHeader(REPLY_TO, replyTo);
                    producer.send(new ProducerRecord<>(target, uuid, direct.toBytes())).get(10000, TimeUnit.MILLISECONDS);
                    totalEvents++;
                    lastActive = System.currentTimeMillis();
                    return;

                } catch (IOException | InterruptedException | ExecutionException | TimeoutException e) {
                    log.warn("Unable to send direct message to {} - {}", target, e.getMessage());
                    closeProducer();
                }
            }
        }
        log.error("Target topic {} not found", target);
    }

}
