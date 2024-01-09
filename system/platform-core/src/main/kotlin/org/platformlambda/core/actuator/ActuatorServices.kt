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

package org.platformlambda.core.actuator

import org.platformlambda.core.annotations.PreLoad
import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.KotlinLambdaFunction
import org.platformlambda.core.models.Kv
import org.platformlambda.core.system.EventEmitter
import org.platformlambda.core.system.ServiceDiscovery
import org.slf4j.LoggerFactory
import java.io.IOException
import java.util.concurrent.atomic.AtomicBoolean
import kotlin.system.exitProcess

@PreLoad(route = "actuator.services", instances = 10)
class ActuatorServices : KotlinLambdaFunction<EventEnvelope, Any> {

    companion object {
        private val log = LoggerFactory.getLogger(ActuatorServices::class.java)
        private const val TYPE = "type"
        private const val INFO = "info"
        private const val ROUTES = "routes"
        private const val LIB = "lib"
        private const val ENV = "env"
        private const val HEALTH = "health"
        private const val HEALTH_STATUS = "health_status"
        private const val SHUTDOWN = "shutdown"
        private const val SUSPEND = "suspend"
        private const val RESUME = "resume"
        private const val LIVENESS_PROBE = "livenessprobe"
        private const val USER = "user"
        private const val WHEN = "when"
        private val infoFunction = InfoService()
        private val healthFunction = HealthService()
        private val healthStatus = AtomicBoolean(true)
    }

    @Throws(Exception::class)
    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, instance: Int): Any {
        if (headers.containsKey(TYPE)) {
            val type = headers[TYPE]
            if (HEALTH_STATUS == type && input.body is Boolean) {
                healthStatus.set(input.body as Boolean)
                return true
            }
            if (LIVENESS_PROBE == type) {
                return if (healthStatus.get()) {
                    EventEnvelope().setBody("OK").setHeader("content-type", "text/plain")
                } else {
                    EventEnvelope().setBody("Unhealthy. Please check '/health' endpoint.")
                        .setStatus(400).setHeader("content-type", "text/plain")
                }
            }
            if (HEALTH == type) {
                return healthFunction.handleEvent(headers, input, instance)
            }
            if (INFO == type || LIB == type || ROUTES == type || ENV == type) {
                return infoFunction.handleEvent(headers, input, instance)
            }
            if (headers.containsKey(USER)) {
                if (SUSPEND == type || RESUME == type) {
                    if (headers.containsKey(USER) && headers.containsKey(TYPE) && headers.containsKey(WHEN)) {
                        try {
                            EventEmitter.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY,
                                Kv(TYPE, headers[TYPE]), Kv(WHEN, headers[WHEN]), Kv(USER, headers[USER]))
                        } catch (e: IOException) {
                            log.error("Unable to perform {} - {}", type, e.message)
                        }
                    }
                    return false
                }
                if (SHUTDOWN == type && headers.containsKey(USER)) {
                    log.info("Shutdown requested by {}", headers[USER])
                    exitProcess(-2)
                }
            }
        }
        return false;
    }

}