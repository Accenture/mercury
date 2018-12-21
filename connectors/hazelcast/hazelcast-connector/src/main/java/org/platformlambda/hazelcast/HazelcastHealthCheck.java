/*

    Copyright 2018 Accenture Technology

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

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Inbox;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.AppConfigReader;

import java.util.HashMap;
import java.util.Map;

public class HazelcastHealthCheck implements LambdaFunction {

    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String HEALTH = "health";
    private static final String INFO = "info";
    private static final String LOOP_BACK = "loopback";
    private static final String REPLY_TO = "reply_to";
    private static final String ORIGIN = "origin";
    private static final long TIMEOUT = 5000;

    private boolean isServiceMonitor = false;

    public HazelcastHealthCheck() {
        AppConfigReader reader = AppConfigReader.getInstance();
        if ("true".equals(reader.getProperty("service.monitor", "false"))) {
            isServiceMonitor = true;
        }
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (INFO.equals(headers.get(TYPE))) {
            String namespace = HazelcastSetup.getNamespace();
            if (namespace.endsWith("-")) {
                namespace = namespace.substring(0, namespace.length()-1);
            }
            Map<String, Object> result = new HashMap<>();
            result.put("service", "hazelcast");
            result.put("namespace", namespace);
            result.put("cluster", HazelcastSetup.getClusterList());
            return result;
        }

        if (HEALTH.equals(headers.get(TYPE))) {
            long begin = System.currentTimeMillis();
            // wait for reply
            String me = Platform.getInstance().getOrigin();
            Inbox inbox = new Inbox(1);
            // in case of service monitor, the origin of "monitor" is predefined
            String origin = isServiceMonitor? "monitor" : me;
            PostOffice.getInstance().send(PostOffice.CLOUD_CONNECTOR, new Kv(REPLY_TO, inbox.getId() + "@" + me), new Kv(ORIGIN, origin), new Kv(TYPE, LOOP_BACK));
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
                    return "Loopback test took " + diff + " ms";
                }
            }
            throw new AppException(500, "Loopback test failed");

        } else {
            throw new IllegalArgumentException("Usage: type=health");
        }
    }
}
