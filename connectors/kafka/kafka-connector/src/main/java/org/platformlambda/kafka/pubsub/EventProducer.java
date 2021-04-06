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

package org.platformlambda.kafka.pubsub;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.util.SimpleCache;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.platformlambda.kafka.services.ServiceRegistry;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class EventProducer implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(EventProducer.class);

    public static final String EMBED_EVENT = "_evt";
    public static final String RECIPIENT = "_rx";
    private static final SimpleCache cache = SimpleCache.createCache("sticky.destinations", 60000);
    private static final String ID = MultipartPayload.ID;
    private static final String COUNT = MultipartPayload.COUNT;
    private static final String TOTAL = MultipartPayload.TOTAL;
    private static final String TO = MultipartPayload.TO;
    private static final String BROADCAST = MultipartPayload.BROADCAST;
 
    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (headers.containsKey(TO) && body instanceof byte[]) {
            List<String> destinations = getDestinations(headers);
            if (destinations != null) {
                PubSub ps = PubSub.getInstance();
                Utility util = Utility.getInstance();
                byte[] payload = (byte[]) body;
                for (String dest : destinations) {
                    String topicPartition = ServiceRegistry.getTopic(dest);
                    if (topicPartition != null) {
                        final String topic;
                        int partition = -1;
                        if (topicPartition.contains("-")) {
                            int separator = topicPartition.lastIndexOf('-');
                            topic = topicPartition.substring(0, separator);
                            partition = util.str2int(topicPartition.substring(separator + 1));
                        } else {
                            topic = topicPartition;
                        }
                        Map<String, String> parameters = new HashMap<>();
                        parameters.put(EMBED_EVENT, "1");
                        parameters.put(RECIPIENT, dest);
                        ps.publish(topic, partition, parameters, payload);
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
                } else {
                    // reset expiry timer
                    cache.put(id, cached);
                }
                log.debug("cached target {} for {} {} {}", cached, id, count, total);
                return (List<String>) cached;
            }
        }
        // normal message
        Platform platform = Platform.getInstance();
        if (to.contains("@")) {
            String target = to.substring(to.indexOf('@') + 1);
            if (ServiceRegistry.destinationExists(target)) {
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
