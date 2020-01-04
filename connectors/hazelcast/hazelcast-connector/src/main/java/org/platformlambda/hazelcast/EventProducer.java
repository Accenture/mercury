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

package org.platformlambda.hazelcast;

import com.hazelcast.core.HazelcastInstance;
import com.hazelcast.core.ITopic;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class EventProducer implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(EventProducer.class);

    private static final CryptoApi crypto = new CryptoApi();
    private static final String TO = "to";
    private static final String BROADCAST = "broadcast";
    private static final String SERVICE_REGISTRY = ServiceDiscovery.SERVICE_REGISTRY;
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String LOOP_BACK = "loopback";
    private static final String PONG = "pong";
    private static final String REPLY_TO = "reply_to";
    private static final String ORIGIN = "origin";
    // static because this is a shared lambda function
    private static HazelcastInstance client;
    private static boolean isServiceMonitor, ready = false, abort = false;
    private static String forMe;

    public EventProducer(HazelcastInstance client) {
        this.client = client;
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
        forMe = "@"+Platform.getInstance().getOrigin();
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
            if (LOOP_BACK.equals(type) && headers.containsKey(REPLY_TO) && headers.containsKey(ORIGIN)) {
                sendLoopback(headers.get(REPLY_TO), headers.get(ORIGIN));
            }
        }
        if (headers.containsKey(TO) && body instanceof byte[]) {
            List<String> destinations = getDestinations(headers.get(TO), headers.containsKey(BROADCAST));
            if (destinations != null) {
                byte[] payload = (byte[]) body;
                if (destinations.isEmpty()) {
                    // decode the message to see if there is a reply address
                    EventEnvelope event = new EventEnvelope();
                    event.load(payload);
                    if (event.getReplyTo() != null) {
                        EventEnvelope error = new EventEnvelope();
                        error.setTo(event.getReplyTo()).setStatus(404).setBody("Route " + headers.get(TO) + " not found");
                        po.send(error);
                    } else {
                        log.warn("Event dropped because route {} not found", headers.get(TO));
                    }
                } else {
                    for (String dest : destinations) {
                        /*
                         * Automatic segmentation happens at the PostOffice level
                         * so this outgoing payload may be a whole event or a block of it.
                         *
                         * The EventConsumer at the receiving side will reconstruct the payload if needed.
                         */
                        String realTopic = HazelcastSetup.getNamespace() + dest;
                        ITopic<byte[]> iTopic = client.getReliableTopic(realTopic);
                        iTopic.publish(payload);
                    }
                }

            }
        }
        return true;
    }

    private void sendLoopback(String replyTo, String origin) throws IOException {
        /*
         * Since this is a loopback message, the reply-to address is a temporary INBOX with origin as the suffix.
         * Send loopback signal only it is received from the same application instance.
         */
        if (replyTo.endsWith(forMe)) {
            EventEnvelope direct = new EventEnvelope().setTo(replyTo).setHeader(TYPE, PONG).setBody(true);
            String realTopic = HazelcastSetup.getNamespace() + origin;
            ITopic<byte[]> iTopic = client.getReliableTopic(realTopic);
            iTopic.publish(direct.toBytes());
        }
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


    private List<String> getDestinations(String to, boolean broadcast) {
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
}
