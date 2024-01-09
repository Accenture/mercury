/*

    Copyright 2018-2024 Accenture Technology

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
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

/**
 * The Mercury platform provides abstraction of the underlying event stream system
 * <p>
 * <i>Real-time inter-service communication</i>
 * <p>
 * Mercury supports both enterprise messaging systems and publish/subscribe style event stream system.
 * <p>
 * Your application can test if streaming "pub/sub" is supported with the "isStreamingPubSub()" method.
 */
public class PubSub {
    private static final Logger log = LoggerFactory.getLogger(PubSub.class);
    private static final String SYSTEM = "system";
    private static final ConcurrentMap<String, PubSub> instances = new ConcurrentHashMap<>();
    private final ConcurrentMap<String, SubscriberDetails> currentSubscribers = new ConcurrentHashMap<>();
    private final ConcurrentMap<String, SubscriberDetails> suspendedSubscribers = new ConcurrentHashMap<>();

    private PubSubProvider provider;

    private String instanceName;

    /**
     * Instances can only be created using the getInstance methods
     *
     * The getSystemInstance() is reserved for use by the system itself
     * The getInstance() can be used by the user application
     *
     * In the rare case that more than 2 pub/sub clusters are needed,
     * use the getInstance(clusterName) method.
     */
    private PubSub() {
        // protected constructor
    }

    /**
     * Retrieve all PubSub instances
     *
     * @return map of instances
     */
    public static Map<String, PubSub> getInstances() {
        return instances;
    }

    /**
     * Obtain the pub/sub cluster handler instance
     *
     * @param clusterName for referring to an event stream cluster
     * @return PubSub handler instance
     */
    public static synchronized PubSub getInstance(String clusterName) {
        if (!instances.containsKey(clusterName)) {
            PubSub ps = new PubSub();
            ps.instanceName = clusterName;
            instances.put(clusterName, ps);
            log.info("Created new PubSub instance ({})", clusterName);
            return ps;
        } else {
            return instances.get(clusterName);
        }
    }

    /**
     * Obtain the default pub/sub cluster handler instance ('system')
     * @return PubSub handler instance
     */
    public static synchronized PubSub getInstance() {
        // the default instance is 'system'
        return getInstance(SYSTEM);
    }

    /**
     * Retrieve the pub/sub handler instance name
     *
     * @return instance name
     */
    public String getInstanceName() {
        return instanceName;
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
        if (this.provider == null) {
            this.provider = pubSub;
            log.info("Provider {} ({}) loaded", instanceName, this.provider);
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

    private void checkFeature() {
        if (!featureEnabled()) {
            throw new IllegalArgumentException("Pub/sub feature not enabled");
        }
    }

    private String getTopicRef(String topic, int partition) {
        return topic+'#'+partition;
    }

    public void resumeSubscription() {
        Utility util = Utility.getInstance();
        List<String> subscribers = new ArrayList<>(suspendedSubscribers.keySet());
        for (String subscriber: subscribers) {
            SubscriberDetails details = suspendedSubscribers.get(subscriber);
            int hash = subscriber.indexOf('#');
            if (hash > 0) {
                String topic = subscriber.substring(0, hash);
                int partition = util.str2int(subscriber.substring(hash+1));
                try {
                    if (partition < 0) {
                        subscribe(topic, details.listener, details.parameters);
                    } else {
                        subscribe(topic, partition, details.listener, details.parameters);
                    }
                } catch (Exception e) {
                    log.error("Unable to suspend topic {} - {}", subscriber, e.getMessage());
                }
            }
        }
        suspendedSubscribers.clear();
    }

    public void suspendSubscription() {
        Utility util = Utility.getInstance();
        List<String> subscribers = new ArrayList<>(currentSubscribers.keySet());
        for (String subscriber: subscribers) {
            suspendedSubscribers.put(subscriber, currentSubscribers.get(subscriber));
        }
        for (String subscriber: subscribers) {
            int hash = subscriber.indexOf('#');
            if (hash > 0) {
                String topic = subscriber.substring(0, hash);
                int partition = util.str2int(subscriber.substring(hash+1));
                try {
                    if (partition < 0) {
                        unsubscribe(topic);
                    } else {
                        unsubscribe(topic, partition);
                    }
                } catch (Exception e) {
                    log.error("Unable to suspend topic {} - {}", subscriber, e.getMessage());
                }
            }
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
     * Create a queue before publishing
     *
     * @param queue in case the messaging system is an enterprise service bus
     * @return true when queue is successfully created
     * @throws IOException in case the queue cannot be created
     */
    public boolean createQueue(String queue) throws IOException {
        checkFeature();
        return provider.createQueue(queue);
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
     * Delete a queue
     *
     * @param queue in case the messaging system is an enterprise service bus
     * @throws IOException in case the topic cannot be deleted
     */
    public void deleteQueue(String queue) throws IOException {
        checkFeature();
        provider.deleteQueue(queue);
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
     * For Kafka, the parameters must include a client ID, a group ID and an optional READ offset.
     * If read offset is not given, the listener will receive the latest events.
     * Specifying an offset allows the application to rewind time to earlier events.
     *
     * e,g, "client-101", "group-101", "0"
     * where "0" means rewinding the offset to the beginning of the event stream.
     *
     * Metadata is available as special headers for each incoming event like this:
     *  `{_timestamp_=1655234495875, _key_=19706bcf5ea54e1bb8a1d8845946e662, _partition_=0, _offset_=1013, _data_=text}`
     *
     * It includes kafka event timestamp, event key, partition number, read offset and whether the event payload
     * is bytes, map or text.
     *
     * Note that metadata tags use the underscore prefix and suffix.
     *
     * @param topic for a store-n-forward pub/sub channel
     * @param listener function to collect event events
     * @param parameters optional parameters that are cloud connector specific
     * @throws IOException in case topic is not yet created
     */
    public void subscribe(String topic, LambdaFunction listener, String... parameters) throws IOException {
        checkFeature();
        provider.subscribe(topic, listener, parameters);
        int partition = -1;
        currentSubscribers.put(getTopicRef(topic, partition), new SubscriberDetails(listener, parameters));
    }

    /**
     * Subscribe to a topic
     *
     * For Kafka, the parameters must include a client ID, a group ID and an optional READ offset.
     * If read offset is not given, the listener will receive the latest events.
     * Specifying an offset allows the application to rewind time to earlier events.
     *
     * e,g, "client-101", "group-101", "0"
     * where "0" means rewinding the offset to the beginning of the event stream.
     *
     * Metadata is available as special headers for each incoming event like this:
     *  `{_timestamp_=1655234495875, _key_=19706bcf5ea54e1bb8a1d8845946e662, _partition_=0, _offset_=1013, _data_=text}`
     *
     * It includes kafka event timestamp, event key, partition number, read offset and whether the event payload
     * is bytes, map or text.
     *
     * Note that metadata tags use the underscore prefix and suffix.
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
        currentSubscribers.put(getTopicRef(topic, partition), new SubscriberDetails(listener, parameters));
    }

    /**
     * Send an event to a queue in case of enterprise service bus (ESB)
     * @param queue name
     * @param headers are optional
     * @param body is the message payload, most likely in text
     * @throws IOException in case the queue is not available
     */
    public void send(String queue, Map<String, String> headers, Object body) throws IOException {
        checkFeature();
        provider.send(queue, headers, body);
    }

    /**
     * Listen to a queue in case of enterprise service bus (ESB)
     * @param queue name
     * @param listener function to receive messages from the queue
     * @param parameters are optional as per ESB implementation
     * @throws IOException in case the queue is not available
     */
    public void listen(String queue, LambdaFunction listener, String... parameters) throws IOException {
        checkFeature();
        provider.listen(queue, listener, parameters);
    }

    /**
     * Unsubscribe from a topic (or queue). This will detach the registered lambda function
     *
     * @param topic for a store-n-forward pub/sub channel
     * @throws IOException in case topic was not subscribed
     */
    public void unsubscribe(String topic) throws IOException {
        checkFeature();
        provider.unsubscribe(topic);
        currentSubscribers.remove(getTopicRef(topic, -1));
    }

    /**
     * Unsubscribe from a topic (or queue). This will detach the registered lambda function
     * @param topic for a store-n-forward pub/sub channel
     * @param partition to be unsubscribed
     * @throws IOException in case topic was not subscribed
     */
    public void unsubscribe(String topic, int partition) throws IOException {
        checkFeature();
        provider.unsubscribe(topic, partition);
        currentSubscribers.remove(getTopicRef(topic, partition));
    }

    /**
     * Check if a topic (or queue) exists
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

    /**
     * Check if the underlying event stream system support journal streaming
     *
     * @return true or false
     */
    public boolean isStreamingPubSub() {
        return provider.isStreamingPubSub();
    }

    /**
     * Tell the pub/sub provider to perform clean up.
     * e.g. closing connection, etc.
     */
    public void cleanup() {
        provider.cleanup();
    }

    private static class SubscriberDetails {

        public final LambdaFunction listener;
        public final String[] parameters;

        public SubscriberDetails(LambdaFunction listener, String... parameters) {
            this.listener = listener;
            if (parameters.length > 2) {
                // drop offset parameter because it should only be used once
                String[] trimmed = new String[2];
                trimmed[0] = parameters[0];
                trimmed[1] = parameters[1];
                this.parameters = trimmed;
            } else {
                this.parameters = parameters;
            }
        }

    }

}
