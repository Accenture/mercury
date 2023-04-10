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
import org.platformlambda.core.system.Platform
import org.platformlambda.core.system.PostOffice

@PreLoad(route="hello.level.1", instances=10)
class MultiLevelTrace: KotlinLambdaFunction<EventEnvelope, Any> {
    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, instance: Int): Any {
        val fastRPC = FastRPC(headers)
        val po = PostOffice(headers, instance)
        val result: MutableMap<String, Any> = HashMap()
        result["headers"] = headers
        result["body"] = input.body
        result["instance"] = instance
        result["origin"] = Platform.getInstance().origin
        result["route_one"] = po.route
        result["trace_id"] = po.traceId
        result["trace_path"] = po.tracePath
        // annotate trace
        po.annotateTrace("some_key", "some value")
        po.annotateTrace("another_key", "another_value")
        // send to level-2 service
        val request = EventEnvelope().setTo("hello.level.2").setBody("test")
        val response = fastRPC.awaitRequest(request, 5000)
        result["level-2"] = response.body
        return result
    }
}