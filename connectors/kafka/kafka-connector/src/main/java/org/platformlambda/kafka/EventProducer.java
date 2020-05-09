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
import org.platformlambda.core.util.SimpleCache;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

public class EventProducer implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(EventProducer.class);

    private static final CryptoApi crypto = new CryptoApi();
    private static final SimpleCache cache = SimpleCache.createCache("sticky.destinations", 60000);
    private static final String MANAGER = KafkaSetup.MANAGER;
    private static final String SERVICE_MONITOR = "service.monitor";
    private static final String PRESENCE_MONITOR = "presence.monitor";
    private static final String ID = MultipartPayload.ID;
    private static final String COUNT = MultipartPayload.COUNT;
    private static final String TOTAL = MultipartPayload.TOTAL;
    private static final String TO = MultipartPayload.TO;
    private static final String BROADCAST = MultipartPayload.BROADCAST;
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
    private static String producerId;
    private static boolean isServiceMonitor, ready = false, abort = false;
    private static long seq = 0, totalEvents = 0;

    public EventProducer() {
        final ProducerWatcher monitor = new ProducerWatcher();
        monitor.start();
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

    private Properties getProperties() {
        Properties properties = new Properties();
        properties.putAll(KafkaSetup.getKafkaProperties());
        properties.put(ProducerConfig.ACKS_CONFIG, "1"); // Setting to "1" ensures that the message is received by the leader
        properties.put(ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.StringSerializer.class);
        properties.put(ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG, org.apache.kafka.common.serialization.ByteArraySerializer.class);
        properties.put(ProducerConfig.MAX_BLOCK_MS_CONFIG, 15000);
        return properties;
    }

    private void startProducer() {
        if (producer == null) {
            // create unique ID from origin ID by dropping date prefix and adding a sequence suffix
            String id = (Platform.getInstance().getOrigin()+"p"+(++seq)).substring(8);
            Properties properties = getProperties();
            properties.put(ProducerConfig.CLIENT_ID_CONFIG, id);
            producer = new KafkaProducer<>(properties);
            lastStarted = System.currentTimeMillis();
            producerId = properties.getProperty(ProducerConfig.CLIENT_ID_CONFIG);
            log.info("Producer {} ready", producerId);
        }
    }

    private void closeProducer() {
        if (producer != null) {
            try {
                producer.close();
                log.info("Producer {} released, delivered: {}", producerId, totalEvents);
            } catch (Exception e) {
                // ok to ignore
            }
            producer = null;
            producerId = null;
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
            List<String> destinations = getDestinations(headers);
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
                            log.error("Kafka is slow - took {} ms to send to {}", diff, dest);
                        }
                        totalEvents++;
                        lastActive = System.currentTimeMillis();
                    } catch (Exception e) {
                        /*
                         * this is unrecoverable so the app must shutdown
                         * such that it will be restarted by the infrastructure
                         */
                        log.error("Unable to send message to {} - {}", destinations, e.getMessage());
                        System.exit(21);
                    }
                }
            }
        }
        return true;
    }

    @SuppressWarnings("unchecked")
    private List<String> getDestinations(Map<String, String> headers) {
        String to = headers.get(TO);
        boolean broadcast = headers.containsKey(BROADCAST);
        // broadcast to all presence monitor instances?
        if (to.equals("*")) {
            String namespace = Platform.getInstance().getNamespace();
            return Collections.singletonList(namespace == null? PRESENCE_MONITOR : PRESENCE_MONITOR + "." + namespace);
        }
        String id = headers.get(ID);
        String count = headers.get(COUNT);
        String total = headers.get(TOTAL);
        boolean isSegmented = id != null && count != null && total != null;
        if (isSegmented) {
            Object cached = cache.get(id);
            if (cached instanceof List) {
                // clear cache because this is the last block
                if (count.equals(total)) {
                    cache.remove(id);
                }
                log.debug("cached target {} for {} {} {}", cached, id, count, total);
                return (List<String>) cached;
            }
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
                        if (isSegmented) {
                            cache.put(id, available);
                        }
                        return available;
                    } else {
                        String target = getNextAvailable(available);
                        if (target != null) {
                            List<String> result = Collections.singletonList(target);
                            if (isSegmented) {
                                cache.put(id, result);
                            }
                            return result;
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
            producer.send(new ProducerRecord<>(origin, uuid, direct.toBytes())).get(20, TimeUnit.SECONDS);
            totalEvents++;
            lastActive = System.currentTimeMillis();
            log.info("Tell event consumer to start with {}", origin);
        } catch (Exception e) {
            /*
             * Unrecoverable error. Shutdown and let infrastructure to restart this app instance.
             */
            log.error("Unable to initialize consumer - {}", e.getMessage());
            System.exit(20);
        }
    }

    private void sendLoopback(String replyTo, String origin) {
        startProducer();
        try {
            String uuid = Utility.getInstance().getUuid();
            EventEnvelope direct = new EventEnvelope().setTo(replyTo).setHeader(TYPE, PONG).setBody(true);
            producer.send(new ProducerRecord<>(origin, uuid, direct.toBytes())).get(20, TimeUnit.SECONDS);
            totalEvents++;
            lastActive = System.currentTimeMillis();
        } catch (Exception e) {
            /*
             * Unrecoverable error. Shutdown and let infrastructure to restart this app instance.
             */
            log.error("Unable to send direct message to {} - {}", origin, e.getMessage());
            System.exit(22);
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
                    producer.send(new ProducerRecord<>(target, uuid, direct.toBytes())).get(20, TimeUnit.SECONDS);
                    totalEvents++;
                    lastActive = System.currentTimeMillis();
                } catch (Exception e) {
                    /*
                     * Unrecoverable error. Shutdown and let infrastructure to restart this app instance.
                     */
                    log.error("Unable to send direct message to {} - {}", target, e.getMessage());
                    System.exit(23);
                }
            } else {
                log.error("Target topic {} not found", target);
            }
        } else {
            log.error("Unable to ping {}, Expected response from {} is Boolean, Actual: {}, ", target, MANAGER,
                        response.getBody() == null? "null" : response.getBody().getClass());
        }
    }

}
