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

package org.platformlambda.cloud.services;

import io.vertx.core.Future;
import org.platformlambda.cloud.ConnectorConfig;
import org.platformlambda.cloud.reporter.PresenceConnector;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.*;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@EventInterceptor
@ZeroTracing
public class CloudHealthCheck implements TypedLambdaFunction<EventEnvelope, Void> {
    private static final Logger log = LoggerFactory.getLogger(CloudHealthCheck.class);

    private static final String CLOUD_MANAGER = ServiceRegistry.CLOUD_MANAGER;
    private static final String TYPE = "type";
    private static final String HEALTH = "health";
    private static final String INFO = "info";
    private static final String LOOP_BACK = "loopback";
    private static final String REPLY_TO = "reply_to";
    private static final String ORIGIN = "origin";
    private static final String LIST = "list";
    private static final long TIMEOUT = 5000;
    private final boolean presenceMonitor;
    private final String monitorTopicPartition;

    public CloudHealthCheck() {
        AppConfigReader config = AppConfigReader.getInstance();
        presenceMonitor = "true".equals(config.getProperty("service.monitor", "false"));
        monitorTopicPartition = config.getProperty("monitor.topic", "service.monitor") + "-0";
    }

    @Override
    @SuppressWarnings("unchecked")
    public Void handleEvent(Map<String, String> headers, EventEnvelope input, int instance) throws Exception {
        if (INFO.equals(headers.get(TYPE))) {
            Map<String, Object> result = new HashMap<>();
            result.put("service", ConnectorConfig.getServiceName());
            result.put("href", ConnectorConfig.getDisplayUrl());
            result.put("topics", ConnectorConfig.topicSubstitutionEnabled()? "pre-allocated" : "on-demand");
            sendResponse(input, result);
        }
        if (HEALTH.equals(headers.get(TYPE))) {
            if (presenceMonitor) {
                EventEmitter po = EventEmitter.getInstance();
                EventEnvelope req = new EventEnvelope().setTo(CLOUD_MANAGER).setHeader(TYPE, LIST)
                                            .setHeader(ORIGIN, Platform.getInstance().getOrigin());
                Future<EventEnvelope> res = po.asyncRequest(req, TIMEOUT);
                res.onSuccess(evt -> {
                    if (evt.getBody() instanceof List) {
                        List<String> topicList = (List<String>) evt.getBody();
                        String message;
                        if (topicList.isEmpty()) {
                            message = "System does not have any topics";
                        } else {
                            message = "System contains " +
                                    topicList.size() + " " + (topicList.size() == 1 ? "topic" : "topics");
                        }
                        doLoopbackTest(input, monitorTopicPartition, message);

                    } else {
                        sendError(input, 500, "Unable to list topics");
                    }
                });


            } else {
                PresenceConnector connector = PresenceConnector.getInstance();
                boolean ready = connector.isConnected() && connector.isReady();
                boolean offline = false;
                if (ready) {
                    String topicPartition = PresenceConnector.getInstance().getTopic();
                    if (topicPartition != null) {
                        doLoopbackTest(input, topicPartition, null);
                    } else {
                        offline = true;
                    }
                } else {
                    offline = true;
                }
                if (offline) {
                    sendResponse(input, "offline");
                }
            }
        } else {
            sendError(input, 400, "Usage: type=health");
        }
        return null;
    }

    private void doLoopbackTest(EventEnvelope input, String topicPartition, String message) {
        String topic = getTopic(topicPartition);
        int partition = getPartition(topicPartition);
        long begin = System.currentTimeMillis();
        // wait for reply
        PubSub ps = PubSub.getInstance();
        String me = Platform.getInstance().getOrigin();
        String from = input.getFrom();
        String traceId = input.getTraceId();
        String tracePath = input.getTracePath();
        AsyncInbox inbox = new AsyncInbox(from, "cloud.connector", traceId, tracePath, TIMEOUT, true);
        Map<String, String> parameters = new HashMap<>();
        parameters.put(REPLY_TO, inbox.getId()+"@"+me);
        parameters.put(ORIGIN, me);
        try {
            ps.publish(topic, partition, parameters, LOOP_BACK);
            Future<EventEnvelope> res = inbox.getFuture();
            res.onSuccess(pong -> {
                if (pong.getBody() instanceof Boolean && Boolean.TRUE.equals(pong.getBody())) {
                    long diff = System.currentTimeMillis() - begin;
                    String text = "Loopback test took " + diff + " ms" + (message == null? "" : "; "+message);
                    sendResponse(input, text);
                } else {
                    sendError(input, 500, "Loopback failed");
                }
            });
            res.onFailure(ex -> sendError(input, 408, ex.getMessage()));

        } catch (IOException e) {
            sendError(input, 500, e.getMessage());
        }
    }

    private String getTopic(String topicPartition) {
        return topicPartition.contains("-") ?
                topicPartition.substring(0, topicPartition.lastIndexOf('-')) : topicPartition;
    }

    private int getPartition(String topicPartition) {
        int hyphen = topicPartition.lastIndexOf('-');
        if (hyphen == -1) {
            return -1;
        } else {
            return Utility.getInstance().str2int(topicPartition.substring(topicPartition.lastIndexOf('-')+1));
        }
    }

    private void sendResponse(EventEnvelope input, Object result) {
        EventEnvelope response = new EventEnvelope().setTo(input.getReplyTo())
                .setBody(result)
                .setCorrelationId(input.getCorrelationId())
                .setTrace(input.getTraceId(), input.getTracePath());
        try {
            EventEmitter.getInstance().send(response);
        } catch (IOException e) {
            log.error("Unable to deliver response to {} - {}", input.getReplyTo(), e.getMessage());
        }
    }

    private void sendError(EventEnvelope input, int status, String message) {
        EventEnvelope response = new EventEnvelope().setTo(input.getReplyTo())
                .setBody(message).setStatus(status)
                .setCorrelationId(input.getCorrelationId())
                .setTrace(input.getTraceId(), input.getTracePath());
        try {
            EventEmitter.getInstance().send(response);
        } catch (IOException e) {
            log.error("Unable to deliver response to {} - {}", input.getReplyTo(), e.getMessage());
        }
    }

}
