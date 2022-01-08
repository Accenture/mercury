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

package org.platformlambda.core.actuator;

import org.platformlambda.core.models.LambdaFunction;

import java.util.Map;

public class ActuatorServices implements LambdaFunction {
    private static final String TYPE = "type";
    private static final String INFO = "info";
    private static final String ROUTES = "routes";
    private static final String LIB = "lib";
    private static final String ENV = "env";
    private static final String HEALTH = "health";
    private static final String SHUTDOWN = "shutdown";
    private static final String SUSPEND = "suspend";
    private static final String RESUME = "resume";
    private static final String LIVENESSPROBE = "livenessprobe";
    private static final String USER = "user";

    private static final InfoService info = new InfoService();
    private static final HealthService health = new HealthService();
    private static final LivenessProbe livenessProbe = new LivenessProbe();
    private static final ShutdownService shutdown = new ShutdownService();
    private static final SuspendResume suspendResume = new SuspendResume();

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (headers.containsKey(TYPE)) {
            String type = headers.get(TYPE);
            if (LIVENESSPROBE.equals(type)) {
                return livenessProbe.handleEvent(headers, body, instance);
            }
            if (HEALTH.equals(type)) {
                return health.handleEvent(headers, body, instance);
            }
            if (INFO.equals(type) || LIB.equals(type) || ROUTES.equals(type) || ENV.equals(type)) {
                return info.handleEvent(headers, body, instance);
            }
            if (headers.containsKey(USER)) {
                if (SUSPEND.equals(type) || RESUME.equals(type)) {
                    return suspendResume.handleEvent(headers, body, instance);
                }
                if (SHUTDOWN.equals(type)) {
                    return shutdown.handleEvent(headers, body, instance);
                }
            }
        }
        return false;
    }

}
