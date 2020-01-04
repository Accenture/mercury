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
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServiceDiscovery;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;
import java.util.concurrent.TimeoutException;

public class PresenceProducer implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(PresenceProducer.class);

    private HazelcastInstance client;
    private boolean ready = false, abort = false;
    private String topic;

    public PresenceProducer(HazelcastInstance client, String topic) {
        this.client = client;
        this.topic = topic;
    }

    private boolean validRegistry() {
        if (ready) {
            return true;
        }
        if (!abort) {
            try {
                Platform.getInstance().waitForProvider(ServiceDiscovery.SERVICE_REGISTRY, 60);
                ready = true;
                return true;
            } catch (TimeoutException e) {
                abort = true;
            }
        }
        return false;
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        // check for critical resource
        if (!validRegistry()) {
            log.error("abort because {} is not available", ServiceDiscovery.SERVICE_REGISTRY);
            return false;
        }
        if (body instanceof byte[]) {
            byte[] payload = (byte[]) body;
            ITopic<byte[]> iTopic = client.getReliableTopic(topic);
            iTopic.publish(payload);
        }
        return true;
    }

}
