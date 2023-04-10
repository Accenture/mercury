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
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

@EventInterceptor
@ZeroTracing
@PreLoad(route="distributed.tracing")
public class DistributedTrace implements TypedLambdaFunction<EventEnvelope, Void> {
    private static final Logger log = LoggerFactory.getLogger(DistributedTrace.class);

    private static final String DISTRIBUTED_TRACE_FORWARDER = "distributed.trace.forwarder";
    private static final String TRANSACTION_JOURNAL_RECORDER = "transaction.journal.recorder";
    private static final String TRACE = "trace";
    private static final String ANNOTATIONS = "annotations";
    private static final String JOURNAL = "journal";
    private static final String FROM = "from";
    private static final String SERVICE = "service";
    private static final String ORIGIN_SUFFIX = "@" + Platform.getInstance().getOrigin();

    @SuppressWarnings("unchecked")
    @Override
    public Void handleEvent(Map<String, String> headers, EventEnvelope input, int instance) {
        if (input.getRawBody() instanceof Map) {
            Platform platform = Platform.getInstance();
            EventEmitter po = EventEmitter.getInstance();
            Map<String, Object> body = (Map<String, Object>) input.getRawBody();
            Map<String, String> metrics = input.getHeaders();
            String service = metrics.get(SERVICE);
            String from = metrics.get(FROM);
            if (service != null && service.contains("@")) {
                metrics.put(SERVICE, trimOrigin(service));
            }
            if (from != null && from.contains("@")) {
                metrics.put(FROM, trimOrigin(from));
            }
            Map<String, Object> kv = (Map<String, Object>) body.getOrDefault(ANNOTATIONS, Collections.emptyMap());
            if (kv.isEmpty()) {
                log.info("trace={}", metrics);
            } else {
                log.info("trace={}, annotations={}", metrics, kv);
            }
            /*
             *
             * Optionally, forward the perf metrics to a telemetry system.
             * You may implement a function with the "distributed.trace.forwarder" route name.
             *
             * If you have turned on request/response journaling for some services,
             * you may implement a function with the "transaction.journal.recorder" route name.
             *
             * IMPORTANT
             * ---------
             * 1. journal data contains request and response payloads and may contain
             *    sensitive personal information (e.g. PII, PHI, PCI), please adhere to
             *    your organization security policy in saving and accessing the journal
             *    in a database.
             *
             * 2. distributed.trace.forwarder and/or transaction.journal.recorder must be bundled
             *    in the same application executable.
             */
            if (body.containsKey(JOURNAL) && platform.hasRoute(TRANSACTION_JOURNAL_RECORDER)) {
                EventEnvelope event = new EventEnvelope().setTo(TRANSACTION_JOURNAL_RECORDER);
                Map<String, Object> data = new HashMap<>();
                data.put(JOURNAL, body.get(JOURNAL));
                data.put(TRACE, metrics);
                if (!kv.isEmpty()) {
                    data.put(ANNOTATIONS, kv);
                }
                try {
                    po.send(event.setBody(data));
                } catch (IOException e) {
                    log.warn("Unable to relay journal to {} - {}", TRANSACTION_JOURNAL_RECORDER, e.getMessage());
                }
            }
            if (platform.hasRoute(DISTRIBUTED_TRACE_FORWARDER)) {
                EventEnvelope event = new EventEnvelope().setTo(DISTRIBUTED_TRACE_FORWARDER);
                Map<String, Object> data = new HashMap<>();
                data.put(TRACE, metrics);
                if (!kv.isEmpty()) {
                    data.put(ANNOTATIONS, kv);
                }
                try {
                    po.send(event.setBody(data));
                } catch (IOException e) {
                    log.warn("Unable to relay trace metrics to {} - {}", DISTRIBUTED_TRACE_FORWARDER, e.getMessage());
                }
            }
        }
        return null;
    }

    private String trimOrigin(String route) {
        return route.endsWith(ORIGIN_SUFFIX)? route.substring(0, route.indexOf('@')) : route;
    }

}
