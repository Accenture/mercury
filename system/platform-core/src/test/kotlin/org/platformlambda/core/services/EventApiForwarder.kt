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

package org.platformlambda.core.services

import org.platformlambda.core.annotations.PreLoad
import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.KotlinLambdaFunction
import org.platformlambda.core.system.FastRPC
import org.platformlambda.core.util.Utility
import java.util.HashMap

@PreLoad(route="event.api.forwarder", instances=10)
class EventApiForwarder: KotlinLambdaFunction<EventEnvelope, Any> {
    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, instance: Int): Any {
        val fastRPC = FastRPC(headers)
        val util = Utility.getInstance()
        val timeout = util.str2long(headers["timeout"])
        val rpc = "true" == headers["rpc"]
        val endpoint = headers["endpoint"]
        if (endpoint != null && input.body is ByteArray) {
            val securityHeaders: MutableMap<String, String> = HashMap()
            securityHeaders["Authorization"] = "demo"
            return fastRPC.awaitRequest(EventEnvelope(input.body as ByteArray), timeout, securityHeaders, endpoint, rpc)
        } else {
            throw IllegalArgumentException("Invalid request")
        }
    }
}