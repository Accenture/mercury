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

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.PubSubProvider;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

/**
 * The Mercury platform provides abstraction of the underlying event stream system
 * to provide real-time inter-service communication.
 *
 * Some event stream systems are purely real-time.
 * e.g. Hazelcast, a distributed memory grid for event streaming and caching.
 * As a result, "store-n-forward" style of pub/sub is not supported for real-time event streaming systems.
 *
 * However, "store-n-forward" addresses a use case when the producer can send events ahead of the consumer.
 * For example, Kafka is a high performance "store-n-forward" event stream system.
 *
 * "Store-n-forward" event streaming is optional.
 * The pub/sub feature depends on the underlying event stream system.
 *
 * For example, it is supported in the Mercury kafka-connector and not the hazelcast-connector.
 */
public class PubSub {
    private static final Logger log = LoggerFactory.getLogger(PubSub.class);

    private PubSubProvider provider;
    private boolean verified = false;
    private static final PubSub instance = new PubSub();

    private PubSub() {
        // singleton
    }

    public static PubSub getInstance() {
        return instance;
    }

    /**
     * This method is reserved for cloud connector.
     * You should not call this method unless you are writing your own cloud connector.
     *
     * @param pubSub provider
     */
    public void enableFeature(PubSubProvider pubSub) {
        this.provider = pubSub;
    }

    /**
     * Check if pub/sub feature is enabled.
     *
     * @return true or false
     */
    public boolean featureEnabled() {
        if (!verified) {
            try {
                Platform.getInstance().waitForProvider(PostOffice.CLOUD_CONNECTOR, 30);
                verified = true;
            } catch (TimeoutException e) {
                log.warn("{} not ready", PostOffice.CLOUD_CONNECTOR);
                return false;
            }
        }
        return provider != null;
    }

    private void checkFeature() throws IOException {
        if (!featureEnabled()) {
            throw new IOException("Pub/sub feature not implemented in cloud connector");
        }
    }

    /**
     * Create a topic before publishing
     *
     * @param topic for a store-n-forward pub/sub channel
     * @return true when topic is successfully created
     * @throws IOException in case the topic cannot be created
     */
    public boolean createTopic(String topic) throws IOException {
        checkFeature();
        return provider.createTopic(topic);
    }

    /**
     * Delete a topic
     *
     * @param topic for a store-n-forward pub/sub channel
     * @throws IOException in case the topic cannot be deleted
     */
    public void deleteTopic(String topic) throws IOException {
        checkFeature();
        provider.deleteTopic(topic);
    }

    /**
     * Publish an event to a topic
     *
     * @param topic for a store-n-forward pub/sub channel
     * @param headers key-value pairs
     * @param body PoJo, Java primitive (Boolean, Integer, Long, String), Map, List of Strings,
     * @throws IOException in case the event cannot be published or the topic is not found
     */
    public void publish(String topic, Map<String, String> headers, Object body) throws IOException {
        checkFeature();
        provider.publish(topic, headers, body);
    }

    /**
     * Subscribe to a topic
     *
     * @param topic for a store-n-forward pub/sub channel
     * @param listener function to collect event events
     * @param parameters optional parameters that are cloud connector specific
     * @throws IOException in case topic is not yet created
     */
    public void subscribe(String topic, LambdaFunction listener, String... parameters) throws IOException {
        checkFeature();
        provider.subscribe(topic, listener, parameters);
    }

    /**
     * Unsubscribe from a topic. This will detach the registered lambda function.
     *
     * @param topic for a store-n-forward pub/sub channel
     * @throws IOException in case topic was not subscribed
     */
    public void unsubscribe(String topic) throws IOException {
        checkFeature();
        provider.unsubscribe(topic);
    }

    /**
     * Check if a topic exists
     *
     * @param topic name
     * @return true if topic exists
     * @throws IOException in case feature is not enabled
     */
    public boolean exists(String topic) throws IOException {
        checkFeature();
        return provider.exists(topic);
    }

    /**
     * Obtain list of all pub/sub topics
     *
     * @return list of topics
     * @throws IOException in case feature is not enabled
     */
    public List<String> list() throws IOException {
        checkFeature();
        return provider.list();
    }

}
