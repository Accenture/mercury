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

package org.platformlambda.core.actuator;

import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;

import java.io.IOException;
import java.util.Map;

public class SuspendResume implements LambdaFunction {
    private static final String TYPE = "type";
    private static final String USER = "user";
    private static final String WHEN = "when";

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        if (headers.containsKey(USER) && headers.containsKey(TYPE) && headers.containsKey(WHEN)) {
            String type = headers.get(TYPE);
            String when = headers.get(WHEN);
            String user = headers.get(USER);
            PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY,
                    new Kv(TYPE, type), new Kv(WHEN, when), new Kv(USER, user));
        }
        return false;
    }
}