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
 * <p>
 * <i>Real-time inter-service communication</i>
 * <p>
 * Mercury supports both enterprise messaging systems and publish/subscribe style event stream system.
 * <p>
 * Your application can test if native "pub/sub" is supported with the "isNativePubSub()" method.
 */
public class PubSub {
    private static final Logger log = LoggerFactory.getLogger(PubSub.class);

    public static final String PUBLISHER = "system.pubsub.producer";
    private static PubSubProvider provider;
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
        if (pubSub == null) {
            throw new IllegalArgumentException("Missing provider");
        }
        if (PubSub.provider == null) {
            PubSub.provider = pubSub;
            log.info("Provider {} loaded", PubSub.provider);
        } else {
            log.warn("Provider {} is already loaded", PubSub.provider);
        }
    }

    /**
     * Check if pub/sub feature is enabled.
     *
     * @return true or false
     */
    public boolean featureEnabled() {
        return provider != null;
    }

    private void checkFeature() throws IOException {
        if (!featureEnabled()) {
            throw new IOException("Pub/sub feature not implemented in cloud connector");
        }
    }

    public void waitForProvider(int seconds) throws TimeoutException {
        Platform.getInstance().waitForProvider(PUBLISHER, seconds);
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
     * Create a topic before publishing
     *
     * @param topic for a store-n-forward pub/sub channel
     * @param partitions to be created for this topic
     * @return true when topic is successfully created
     * @throws IOException in case the topic cannot be created
     */
    public boolean createTopic(String topic, int partitions) throws IOException {
        checkFeature();
        return provider.createTopic(topic, partitions);
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
     * Publish an event to a topic
     *
     * @param topic for a store-n-forward pub/sub channel
     * @param partition to publish
     * @param headers key-value pairs
     * @param body PoJo, Java primitive (Boolean, Integer, Long, String), Map, List of Strings,
     * @throws IOException in case the event cannot be published or the topic is not found
     */
    public void publish(String topic, int partition, Map<String, String> headers, Object body) throws IOException {
        checkFeature();
        provider.publish(topic, partition, headers, body);
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
     * Subscribe to a topic
     *
     * @param topic for a store-n-forward pub/sub channel
     * @param partition to be subscribed
     * @param listener function to collect event events
     * @param parameters optional parameters that are cloud connector specific
     * @throws IOException in case topic is not yet created
     */
    public void subscribe(String topic, int partition, LambdaFunction listener, String... parameters) throws IOException {
        checkFeature();
        provider.subscribe(topic, partition, listener, parameters);
    }

    /**
     * Unsubscribe from a topic. This will detach the registered lambda function
     *
     * @param topic for a store-n-forward pub/sub channel
     * @throws IOException in case topic was not subscribed
     */
    public void unsubscribe(String topic) throws IOException {
        checkFeature();
        provider.unsubscribe(topic);
    }

    /**
     * Unsubscribe from a topic. This will detach the registered lambda function
     * @param topic for a store-n-forward pub/sub channel
     * @param partition to be unsubscribed
     * @throws IOException in case topic was not subscribed
     */
    public void unsubscribe(String topic, int partition) throws IOException {
        checkFeature();
        provider.unsubscribe(topic, partition);
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

    public int partitionCount(String topic) throws IOException {
        checkFeature();
        return provider.partitionCount(topic);
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

    public boolean isNativePubSub() {
        return provider.isNativePubSub();
    }

}
