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

package org.platformlambda.core.mock

import kotlinx.coroutines.delay
import org.platformlambda.core.annotations.PreLoad
import org.platformlambda.core.exception.AppException
import org.platformlambda.core.models.KotlinLambdaFunction
import org.platformlambda.core.system.Platform
import org.platformlambda.core.util.Utility
import org.slf4j.LoggerFactory
import java.sql.SQLException

@PreLoad(route = "hello.world, hello.alias", instances = 10, isPrivate = false, envInstances = "instances.hello.world")
class HelloWorld : KotlinLambdaFunction<Any?, Map<String, Any>> {

    @Throws(Exception::class)
    override suspend fun handleEvent(headers: Map<String, String>, input: Any?, instance: Int): Map<String, Any> {
        val body = input?.toString() ?: EMPTY;
        val c = Utility.getInstance().str2int(body)
        if (c % 2 == 0) {
            log.info("{} for {} ms", SIMULATE_DELAY, TIMEOUT)
            // Simulate slow response. Unlike Thread.sleep in Java, Kotlin's delay API is non-blocking
            delay(TIMEOUT)
        }
        if (headers.containsKey(EXCEPTION)) {
            log.info(SIMULATE_EXCEPTION)
            throw AppException(400, JUST_A_TEST)
        }
        if (headers.containsKey(NEST_EXCEPTION)) {
            log.info(SIMULATE_NESTED)
            throw AppException(400, JUST_A_TEST, SQLException(SQL_ERROR))
        }
        val result: MutableMap<String, Any> = HashMap()
        result["headers"] = headers
        if (input != null) {
            result["body"] = input
        }
        result["instance"] = instance
        result["counter"] = c
        result["origin"] = Platform.getInstance().origin
        return result
    }

    companion object {
        private val log = LoggerFactory.getLogger(HelloWorld::class.java)
        private const val EXCEPTION = "exception"
        private const val NEST_EXCEPTION = "nested_exception"
        private const val SIMULATE_DELAY = "Simulate delay"
        private const val SIMULATE_NESTED = "Simulate nested SQL exception"
        private const val SIMULATE_EXCEPTION = "Simulate exception"
        private const val JUST_A_TEST = "just a test"
        private const val TIMEOUT = 1000L;
        private const val EMPTY = "empty";
        private const val SQL_ERROR = "sql error"
    }
}