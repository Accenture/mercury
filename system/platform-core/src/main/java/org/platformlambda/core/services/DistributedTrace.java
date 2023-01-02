/*

    Copyright 2018-2023 Accenture Technology

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
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;

@EventInterceptor
@ZeroTracing
@PreLoad(route="distributed.tracing")
public class DistributedTrace implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(DistributedTrace.class);

    private static final String DISTRIBUTED_TRACE_PROCESSOR = "distributed.trace.processor";
    private static final String ANNOTATIONS = "annotations";
    private static final String PAYLOAD = "payload";
    private final boolean aggregation;

    public DistributedTrace() {
        AppConfigReader config = AppConfigReader.getInstance();
        aggregation = "true".equalsIgnoreCase(config.getProperty("distributed.trace.aggregation", "true"));
    }

    @SuppressWarnings("unchecked")
    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        EventEnvelope trace = (EventEnvelope) body;
        if (trace.getBody() instanceof Map) {
            PostOffice po = PostOffice.getInstance();
            Map<String, Object> data = (Map<String, Object>) trace.getBody();
            log.info("trace={}, annotations={}", trace.getHeaders(), data.get(ANNOTATIONS));
            /*
             * When deployed, distributed trace aggregator will receive all trace metrics and
             * optionally annotations and payload
             *
             * If distributed.trace.aggregation=false, trace metrics without transaction payloads will be ignored.
             * Therefore, this allows us to do journaling without trace aggregation.
             * You can rely on distributed trace logging to inspect the trace metrics from a centralized logging system.
             */
            if (po.exists(DISTRIBUTED_TRACE_PROCESSOR)) {
                EventEnvelope event = new EventEnvelope();
                event.setTo(DISTRIBUTED_TRACE_PROCESSOR).setHeaders(trace.getHeaders());
                if (aggregation || data.containsKey(PAYLOAD)) {
                    try {
                        po.send(event.setBody(data));
                    } catch (IOException e) {
                        log.warn("Unable to relay trace to {} - {}", DISTRIBUTED_TRACE_PROCESSOR, e.getMessage());
                    }
                }
            }
        }
        return null;
    }

}
