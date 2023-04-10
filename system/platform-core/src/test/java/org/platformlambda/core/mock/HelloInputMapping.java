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

package org.platformlambda.core.mock;

import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.PoJo;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.Platform;

import java.util.Map;

@CoroutineRunner
@PreLoad(route="hello.input.mapping", instances = 10)
public class HelloInputMapping implements TypedLambdaFunction<PoJo, EventEnvelope> {

    private static final String ROUTE = "hello.input.mapping";

    private static final String MY_ROUTE = "my_route";
    private static final String MY_TRACE_ID = "my_trace_id";
    private static final String MY_TRACE_PATH = "my_trace_path";
    private static final String COROUTINE = "coroutine";
    private static final String SUSPEND = "suspend";
    private static final String INTERCEPTOR = "interceptor";
    private static final String TRACING = "tracing";

    @Override
    public EventEnvelope handleEvent(Map<String, String> headers, PoJo input, int instance) {
        Platform platform = Platform.getInstance();
        boolean isCoroutine = platform.isCoroutine(ROUTE);
        boolean isKotlin = platform.isSuspendFunction(ROUTE);
        boolean isInterceptor = platform.isInterceptor(ROUTE);
        boolean isTrackable = platform.isTrackable(ROUTE);
        /*
         * trace_id and trace_path are READ only metadata
         */
        return new EventEnvelope().setBody(input)
                .setHeader(MY_ROUTE, headers.get(MY_ROUTE))
                .setHeader(MY_TRACE_ID, headers.get(MY_TRACE_ID))
                .setHeader(MY_TRACE_PATH, headers.get(MY_TRACE_PATH))
                .setHeader(COROUTINE, isCoroutine)
                .setHeader(SUSPEND, isKotlin)
                .setHeader(INTERCEPTOR, isInterceptor)
                .setHeader(TRACING, isTrackable);
    }

}
