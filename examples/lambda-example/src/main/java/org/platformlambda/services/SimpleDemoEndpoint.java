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

package org.platformlambda.services;

import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.TypedLambdaFunction;

import java.util.HashMap;
import java.util.Map;

/**
 * This function illustrates parsing an incoming HTTP request for path and query parameters
 * <p>
 * Please refer to the "hello.simple" demo endpoint in rest.yaml config file for details.
 */
@CoroutineRunner
@PreLoad(route="hello.simple", instances=10)
public class SimpleDemoEndpoint implements TypedLambdaFunction<AsyncHttpRequest, Object> {
    private static final String TASK = "task";
    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) throws Exception {
        if (input.getUrl() == null) {
            throw new IllegalArgumentException("The input does not appear to be a HTTP request. " +
                    "Please route the request through REST automation");
        }
        // demonstrate simple input validation
        String a = input.getQueryParameter("a");
        if (a == null) {
            throw new IllegalArgumentException("Missing query parameter 'a'");
        }
        String b = input.getQueryParameter("b");
        if (b == null) {
            throw new IllegalArgumentException("Missing query parameter 'b'");
        }
        String uri = input.getUrl();
        // get a "named" path parameter
        String task = input.getPathParameter(TASK);
        // get a wildcard path parameter (i.e. unnamed)
        int slash = uri.lastIndexOf('/');
        String lastPathElement = uri.substring(slash+1);
        // echo the request
        Map<String, Object> result = new HashMap<>();
        result.put(TASK, task);
        result.put("uri", uri);
        result.put("last_path_parameter", lastPathElement);
        result.put("query", input.getQueryParameters());
        return result;
    }
}
