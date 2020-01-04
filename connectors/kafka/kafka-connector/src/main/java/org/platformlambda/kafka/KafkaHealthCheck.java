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

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Inbox;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.kafka.reporter.PresenceConnector;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class KafkaHealthCheck implements LambdaFunction {

    private static final String MANAGER = KafkaSetup.MANAGER;
    private static final String TYPE = "type";
    private static final String HEALTH = "health";
    private static final String INFO = "info";
    private static final String LOOP_BACK = "loopback";
    private static final String REPLY_TO = "reply_to";
    private static final String ORIGIN = "origin";
    private static final String LIST = "list";
    private static final long TIMEOUT = 5000;
    // static because this is a shared lambda function
    private static boolean isServiceMonitor;
    private static String statusMessage = "OK";

    public KafkaHealthCheck() {
        AppConfigReader reader = AppConfigReader.getInstance();
        isServiceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {

        if (INFO.equals(headers.get(TYPE))) {
            Map<String, Object> result = new HashMap<>();
            result.put("service", "kafka");
            result.put("href", KafkaSetup.getDisplayUrl());
            return result;
        }

        if (HEALTH.equals(headers.get(TYPE))) {
            if (isServiceMonitor) {
                // Since service monitor does not have a topic, we can just list all topics
                PostOffice po = PostOffice.getInstance();
                EventEnvelope response = po.request(MANAGER, TIMEOUT, new Kv(TYPE, LIST), new Kv(ORIGIN, Platform.getInstance().getOrigin()));
                if (response.getBody() instanceof List) {
                    List<String> list = (List<String>) response.getBody();
                    if (list.isEmpty()) {
                        statusMessage =  "Kafka is healthy but it does not have any topics";
                    } else {
                        statusMessage =  "kafka is healthy and it contains " + list.size() + " " + (list.size() == 1 ? "topic" : "topics");
                    }
                } else {
                    throw new AppException(500, "Unable to list kafka topics");
                }
                return statusMessage;

            } else {
                long begin = System.currentTimeMillis();
                // wait for reply
                String me = Platform.getInstance().getOrigin();
                Inbox inbox = new Inbox(1);
                PostOffice.getInstance().send(PostOffice.CLOUD_CONNECTOR, new Kv(REPLY_TO, inbox.getId()+"@"+me), new Kv(ORIGIN, me), new Kv(TYPE, LOOP_BACK));
                inbox.waitForResponse(TIMEOUT);
                EventEnvelope pong = inbox.getReply();
                inbox.close();
                if (pong == null) {
                    throw new AppException(408, "Loopback test timeout for " + TIMEOUT + " ms");
                }
                if (pong.hasError()) {
                    throw new AppException(pong.getStatus(), pong.getError());
                }
                if (pong.getBody() instanceof Boolean) {
                    Boolean pingOk = (Boolean) pong.getBody();
                    if (pingOk) {
                        long diff = System.currentTimeMillis() - begin;
                        String loopback = "Loopback test took " + diff + " ms";
                        PresenceConnector connector = PresenceConnector.getInstance();
                        boolean ready = connector.isConnected() && connector.isReady();
                        return loopback+"; presence-monitor "+(ready? "connected" : "offline");
                    }
                }
                throw new AppException(500, "Loopback test failed");
            }
        } else {
            throw new IllegalArgumentException("Usage: type=health");
        }

    }

}
