/*

    Copyright 2018-2022 Accenture Technology

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

import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;

import java.util.HashMap;
import java.util.Map;

@PreLoad(route="hello.world, hello.alias", instances = 10, isPrivate = false, envInstances = "instances.hello.world")
public class HelloWorld implements LambdaFunction {

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        int c = body instanceof Integer? (int) body : 2;
        if (c % 2 == 0) {
            // simulate slow response
            Thread.sleep(1000);
        }
        if (headers.containsKey("exception")) {
            throw new AppException(400, "just a test");
        }
        Map<String, Object> result = new HashMap<>();
        result.put("headers", headers);
        result.put("body", body);
        result.put("instance", instance);
        result.put("counter", c);
        result.put("origin", Platform.getInstance().getOrigin());
        return result;
    }
}
