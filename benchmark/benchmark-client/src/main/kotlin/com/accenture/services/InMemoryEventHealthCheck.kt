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

package com.accenture.services

import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.KotlinLambdaFunction
import org.platformlambda.core.system.FastRPC
import org.platformlambda.core.util.Utility

class InMemoryEventHealthCheck: KotlinLambdaFunction<EventEnvelope, Any> {

    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, instance: Int): Any {
        val fastRPC = FastRPC(headers)
        /*
         * The interface contract for a health check service includes both INFO and HEALTH responses
         */
        if (INFO == headers[TYPE]) {
            val result: MutableMap<String, Any> = HashMap()
            result["service"] = "in-memory-event-system"
            result["href"] = "http://127.0.0.1"
            return result
        }
        if (HEALTH == headers[TYPE]) {
            val start = System.currentTimeMillis()
            val payload: MutableMap<String, Any> = HashMap();
            payload["hello"] = "world"
            val req = EventEnvelope().setTo("network.one.way").setBody(payload)
            val response = fastRPC.awaitRequest(req, 5000)
            if (response.body is Map<*, *>) {
                val timestamp = ""+(response.body as Map<*, *>)["time"]
                val oneTrip = Utility.getInstance().str2date(timestamp)
                return "test event delivered in ${oneTrip.time - start} ms - "+timestamp
            }
        }
        throw IllegalArgumentException("type must be info or health")
    }

    companion object {
        private const val TYPE = "type"
        private const val INFO = "info"
        private const val HEALTH = "health"
    }

}