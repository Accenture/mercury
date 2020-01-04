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

package org.platformlambda.core.services;

import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;

@EventInterceptor
@ZeroTracing
public class DistributedTrace implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(DistributedTrace.class);

    private static final String DISTRIBUTED_TRACING = "distributed.trace.processor";
    private static final long INTERVAL = 5000;
    private boolean found = false;
    private Long lastCheck = null;

    private String processor;

    public DistributedTrace() {
        AppConfigReader config = AppConfigReader.getInstance();
        this.processor = config.getProperty(DISTRIBUTED_TRACING, DISTRIBUTED_TRACING);
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        long now = System.currentTimeMillis();
        if (lastCheck == null || now - lastCheck > INTERVAL) {
            lastCheck = now;
            found = PostOffice.getInstance().exists(processor);
        }
        if (body instanceof EventEnvelope) {
            EventEnvelope trace = (EventEnvelope) body;
            log.info("trace={}, annotations={}", trace.getHeaders(), trace.getBody());
            if (found) {
                EventEnvelope event = new EventEnvelope();
                event.setTo(processor).setBody(trace.getBody());
                Map<String, String> map = trace.getHeaders();
                for (String h: map.keySet()) {
                    event.setHeader(h, map.get(h));
                }
                try {
                    PostOffice.getInstance().send(event);
                } catch (Exception e) {
                    log.warn("Unable to relay trace to {} - {}", processor, e.getMessage());
                }
            }
        }
        return null;
    }
}
