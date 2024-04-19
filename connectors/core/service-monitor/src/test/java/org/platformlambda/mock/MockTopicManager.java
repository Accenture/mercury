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

package org.platformlambda.mock;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.util.List;
import java.util.Map;

public class MockTopicManager implements LambdaFunction {

    private static final String TYPE = "type";
    private static final String PARTITIONS = "partitions";
    private static final String TOPIC = "topic";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private final PubSub ps;

    public MockTopicManager() {
        this.ps = PubSub.getInstance();
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object input, int instance) throws IOException {
        if (headers.containsKey(TYPE)) {
            if (LIST.equals(headers.get(TYPE))) {
                return listTopics();
            }
            if (EXISTS.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                String origin = headers.get(TOPIC);
                return topicExists(origin);
            }
            if (PARTITIONS.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                String origin = headers.get(TOPIC);
                return topicPartitions(origin);
            }
            // if origin is not specified, it will create the dedicated topic for a new application that is starting up
            if (CREATE.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                int partitions = headers.containsKey(PARTITIONS)?
                        Math.max(1, Utility.getInstance().str2int(headers.get(PARTITIONS))) : 1;
                createTopic(headers.get(TOPIC), partitions);
                return true;
            }
            // delete topic when an application instance expires
            if (DELETE.equals(headers.get(TYPE)) && headers.containsKey(TOPIC)) {
                String origin = headers.get(TOPIC);
                if (topicExists(origin)) {
                    deleteTopic(origin);
                }
                return true;
            }
        }
        return false;
    }

    private boolean topicExists(String topic) throws IOException {
        return ps.exists(topic);
    }

    private int topicPartitions(String topic) throws IOException {
        return ps.partitionCount(topic);
    }

    private void createTopic(String topic, int partitions) throws IOException {
        ps.createTopic(topic, partitions);
    }

    private void deleteTopic(String topic) throws IOException {
        ps.deleteTopic(topic);
    }

    private List<String> listTopics() throws IOException {
        return ps.list();
    }
}
