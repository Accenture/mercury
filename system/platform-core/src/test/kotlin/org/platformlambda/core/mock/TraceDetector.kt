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

package org.platformlambda.core.mock

import org.platformlambda.core.annotations.PreLoad
import org.platformlambda.core.models.KotlinLambdaFunction
import org.slf4j.LoggerFactory

@PreLoad(route = "coroutine.trace.detector")
class TraceDetector : KotlinLambdaFunction<Any?, Boolean> {
    override suspend fun handleEvent(headers: Map<String, String>, input: Any?, instance: Int): Boolean {
        log.info("Trace detector got {}", headers)
        return  headers.containsKey("my_route") &&
                headers.containsKey("my_trace_id") && headers.containsKey("my_trace_path")
    }

    companion object {
        private val log = LoggerFactory.getLogger(TraceDetector::class.java)
    }

}