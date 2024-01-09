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

package org.platformlambda.core.mock;

import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.PostOffice;

import java.util.HashMap;
import java.util.Map;

@CoroutineRunner
@PreLoad(route="rpc.timeout.check", instances = 50)
public class RpcTimeoutCheck implements TypedLambdaFunction<EventEnvelope, Map<String, Object>> {

    private static final String RPC = EventEmitter.RPC;
    private static final String BODY = "body";

    @Override
    public Map<String, Object> handleEvent(Map<String, String> headers, EventEnvelope event, int instance) {
        Map<String, Object> result = new HashMap<>();
        result.put(RPC, event.getTag(RPC));
        if (event.getBody() instanceof Integer) {
            PostOffice po = new PostOffice(headers, instance);
            result.put(BODY, event.getBody());
            po.annotateTrace(BODY, String.valueOf(event.getBody()));
        }
        return result;
    }
}
