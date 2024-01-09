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

package org.platformlambda.services;

import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;

import java.util.Map;

@PreLoad(route="v1.api.auth", instances=10)
public class AuthDemo implements TypedLambdaFunction<AsyncHttpRequest, EventEnvelope> {
    @Override
    public EventEnvelope handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) {
        // when you set headers here, it is assumed to be some session information to pass to the next function
        EventEnvelope result = new EventEnvelope();
        result.setHeader("user", "demo");
        /*
         * Since this is a demo, it just accepts the request.
         * In a real application, you should implement logic to authenticate.
         */
        result.setBody(true);
        return result;
    }
}
