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

import org.platformlambda.common.JacksonSerializer
import org.platformlambda.common.SimplePoJo
import org.platformlambda.core.annotations.PreLoad
import org.platformlambda.core.models.KotlinLambdaFunction

@PreLoad(route = "custom.serializer.service.kotlin", customSerializer = JacksonSerializer::class)
class CustomSerializerKotlinService : KotlinLambdaFunction<SimplePoJo, SimplePoJo> {
    override suspend fun handleEvent(headers: Map<String, String>, input: SimplePoJo, instance: Int): SimplePoJo {
        return input
    }
}