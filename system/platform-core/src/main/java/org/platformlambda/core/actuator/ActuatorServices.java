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

import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import java.util.concurrent.atomic.AtomicBoolean;

import java.util.Map;

@PreLoad(route="actuator.services", instances=10)
public class ActuatorServices implements LambdaFunction {
    private static final String TYPE = "type";
    private static final String INFO = "info";
    private static final String ROUTES = "routes";
    private static final String LIB = "lib";
    private static final String ENV = "env";
    private static final String HEALTH = "health";
    private static final String HEALTH_STATUS = "health_status";
    private static final String SHUTDOWN = "shutdown";
    private static final String SUSPEND = "suspend";
    private static final String RESUME = "resume";
    private static final String LIVENESS_PROBE = "livenessprobe";
    private static final String USER = "user";

    private static final InfoService infoFunction = new InfoService();
    private static final HealthService healthFunction = new HealthService();
    private static final ShutdownService shutdownFunction = new ShutdownService();
    private static final SuspendResume suspendResume = new SuspendResume();
    private static final AtomicBoolean healthStatus = new AtomicBoolean(true);

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        if (headers.containsKey(TYPE)) {
            String type = headers.get(TYPE);
            if (HEALTH_STATUS.equals(type) && body instanceof Boolean) {
                healthStatus.set((Boolean) body);
                return true;
            }
            if (LIVENESS_PROBE.equals(type)) {
                if (healthStatus.get()) {
                    return new EventEnvelope().setBody("OK").setHeader("content-type", "text/plain");
                } else {
                    return new EventEnvelope().setBody("Unhealthy. Please check '/health' endpoint.")
                                    .setStatus(400).setHeader("content-type", "text/plain");
                }
            }
            if (HEALTH.equals(type)) {
                return healthFunction.handleEvent(headers, body, instance);
            }
            if (INFO.equals(type) || LIB.equals(type) || ROUTES.equals(type) || ENV.equals(type)) {
                return infoFunction.handleEvent(headers, body, instance);
            }
            if (headers.containsKey(USER)) {
                if (SUSPEND.equals(type) || RESUME.equals(type)) {
                    return suspendResume.handleEvent(headers, body, instance);
                }
                if (SHUTDOWN.equals(type)) {
                    return shutdownFunction.handleEvent(headers, body, instance);
                }
            }
        }
        return false;
    }

}
