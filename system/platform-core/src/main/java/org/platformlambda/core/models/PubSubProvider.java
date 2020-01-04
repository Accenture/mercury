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

package org.platformlambda.core.models;

import java.io.IOException;
import java.util.List;
import java.util.Map;

/**
 * This interface is reserved for cloud connector.
 * You should not implement this method unless you are writing your own cloud connector.
 */
public interface PubSubProvider {

    boolean createTopic(String topic) throws IOException;

    void deleteTopic(String topic) throws IOException;

    void publish(String topic, Map<String, String> headers, Object body) throws IOException;

    void subscribe(String topic, LambdaFunction listener, String... parameters) throws IOException;

    void unsubscribe(String topic) throws IOException;

    boolean exists(String topic) throws IOException;

    List<String> list() throws IOException;
}
