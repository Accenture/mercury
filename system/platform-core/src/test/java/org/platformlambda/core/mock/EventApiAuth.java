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
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;

@CoroutineRunner
@PreLoad(route="event.api.auth")
public class EventApiAuth implements TypedLambdaFunction<AsyncHttpRequest, Object> {
    private static final Logger log = LoggerFactory.getLogger(EventApiAuth.class);
    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) throws Exception {
        boolean authorized = "demo".equals(input.getHeader("authorization"));
        log.info("Event API authorization {} {} = {}", input.getMethod(), input.getUrl(), authorized? "PASS" : "FAIL");
        return new EventEnvelope().setBody(authorized).setHeader("user", "demo");
    }
}
