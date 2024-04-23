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

import org.platformlambda.core.annotations.KernelThreadRunner;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.PoJo;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.Platform;

import java.util.Map;

/**
 * Kernel Thread should be reserved for legacy functions that are blocking.
 * <p>
 * It is used here for unit test only.
 */
@KernelThreadRunner
@PreLoad(route="hello.input.mapping", instances = 10)
public class HelloInputMapping implements TypedLambdaFunction<PoJo, EventEnvelope> {

    private static final String SERVICE_NAME = "hello.input.mapping";

    private static final String MY_ROUTE = "my_route";
    private static final String MY_TRACE_ID = "my_trace_id";
    private static final String MY_TRACE_PATH = "my_trace_path";
    private static final String ROUTE = "route";
    private static final String TRACE_ID = "trace_id";
    private static final String TRACE_PATH = "trace_path";
    private static final String COROUTINE = "coroutine";
    private static final String SUSPEND = "suspend";
    private static final String INTERCEPTOR = "interceptor";
    private static final String TRACING = "tracing";

    @Override
    public EventEnvelope handleEvent(Map<String, String> headers, PoJo input, int instance) {
        Platform platform = Platform.getInstance();
        boolean isCoroutine = platform.isCoroutine(SERVICE_NAME);
        boolean isKotlin = platform.isSuspendFunction(SERVICE_NAME);
        boolean isInterceptor = platform.isInterceptor(SERVICE_NAME);
        boolean isTrackable = platform.isTrackable(SERVICE_NAME);
        /*
         * trace_id and trace_path are READ only metadata
         */
        return new EventEnvelope().setBody(input)
                // the system will filter out reserved metadata
                .setHeader(MY_ROUTE, headers.get(MY_ROUTE))
                .setHeader(MY_TRACE_ID, headers.get(MY_TRACE_ID))
                .setHeader(MY_TRACE_PATH, headers.get(MY_TRACE_PATH))
                // therefore, we keep them in different key-values
                .setHeader(ROUTE, headers.get(MY_ROUTE))
                .setHeader(TRACE_ID, headers.get(MY_TRACE_ID))
                .setHeader(TRACE_PATH, headers.get(MY_TRACE_PATH))
                // return the service attributes for unit test to validate
                .setHeader(COROUTINE, isCoroutine)
                .setHeader(SUSPEND, isKotlin)
                .setHeader(INTERCEPTOR, isInterceptor)
                .setHeader(TRACING, isTrackable);
    }

}
