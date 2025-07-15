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

package org.platformlambda.kafka.adapter;

import org.apache.kafka.clients.producer.KafkaProducer;
import org.apache.kafka.clients.producer.ProducerRecord;
import org.apache.kafka.common.header.Header;
import org.apache.kafka.common.header.internals.RecordHeader;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

@PreLoad(route="kafka.notification", instances=10)
public class KafkaNotification implements TypedLambdaFunction<Map<String, Object>, Object> {
    private static final Logger log = LoggerFactory.getLogger(KafkaNotification.class);
    private static final String JSON = "json";
    private static final String EVENT = "event";
    private static final String TOPIC = "topic";
    private static final String X_TRACE_ID = "x-trace-id";
    private static final String X_CONTENT_TYPE = "x-content-type";
    private static final String CONTENT = "content";

    @Override
    public Object handleEvent(Map<String, String> headers, Map<String, Object> input, int instance)
                                throws ExecutionException, InterruptedException, TimeoutException {
        PostOffice po = new PostOffice(headers, instance);
        if (headers.containsKey(TOPIC) && input.containsKey(CONTENT)) {
            Utility util = Utility.getInstance();
            String id = util.getUuid();
            String topic = headers.get(TOPIC);
            String contentType = headers.getOrDefault(X_CONTENT_TYPE, JSON);
            String traceId = headers.getOrDefault(X_TRACE_ID, po.getTraceId());
            List<Header> headerList = new ArrayList<>();
            headerList.add(new RecordHeader(X_CONTENT_TYPE, util.getUTF(contentType)));
            if (traceId != null) {
                headerList.add(new RecordHeader(X_TRACE_ID, util.getUTF(traceId)));
            }
            Object o = input.get(CONTENT);
            if (EVENT.equals(contentType) && o instanceof byte[]) {
                KafkaProducer<String, byte[]> producer = waitForProducer();
                producer.send(new ProducerRecord<>(topic, null, id, (byte[]) o, headerList))
                        .get(20, TimeUnit.SECONDS);
                return true;
            } else if (JSON.equals(contentType)) {
                KafkaProducer<String, byte[]> producer = waitForProducer();
                final byte[] data;
                if (o instanceof String) {
                    data = util.getUTF((String) o);
                } else if (o instanceof Map) {
                    data = SimpleMapper.getInstance().getMapper().writeValueAsBytes(o);
                } else if (o instanceof byte[]){
                    data = (byte[]) o;
                } else {
                    data = util.getUTF(String.valueOf(o));
                }
                producer.send(new ProducerRecord<>(topic, null, id, data, headerList))
                        .get(20, TimeUnit.SECONDS);
                return true;
            } else {
                throw new IllegalArgumentException("content must be JSON string or bytes (EventEnvelope)");
            }
        }
        throw new IllegalArgumentException("Missing topic in headers or content in body");
    }

    private KafkaProducer<String, byte[]> waitForProducer() throws InterruptedException {
        for (int i=0; i < 10; i++) {
            KafkaProducer<String, byte[]> producer = KafkaAdapter.getProducer();
            if (producer == null) {
                Thread.sleep(1000);
                log.info("Waiting for Kafka producer to get ready...{}", i+1);
            } else {
                return producer;
            }
        }
        throw new IllegalArgumentException("Producer not ready");
    }
}
