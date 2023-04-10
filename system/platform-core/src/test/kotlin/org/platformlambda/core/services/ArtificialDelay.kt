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

import kotlinx.coroutines.delay
import org.platformlambda.core.annotations.PreLoad
import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.KotlinLambdaFunction
import org.slf4j.LoggerFactory

@PreLoad(route="artificial.delay", instances=200)
class ArtificialDelay: KotlinLambdaFunction<EventEnvelope, Any> {
    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, instance: Int): Any {
        log.info("#{}. simulate delay of {} ms", instance, TIMEOUT)
        delay(TIMEOUT)
        return input
    }

    companion object {
        private val log = LoggerFactory.getLogger(ArtificialDelay::class.java)
        private const val TIMEOUT = 500L;
    }
}