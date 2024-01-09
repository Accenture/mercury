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

package org.platformlambda.core.services

import org.platformlambda.core.annotations.PreLoad
import org.platformlambda.core.annotations.ZeroTracing
import org.platformlambda.core.exception.AppException
import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.KotlinLambdaFunction
import org.platformlambda.core.models.Kv
import org.platformlambda.core.system.EventEmitter
import org.platformlambda.core.system.FastRPC
import org.platformlambda.core.util.Utility
import java.io.ByteArrayOutputStream

@ZeroTracing
@PreLoad(route = "stream.to.bytes", instances = 50)
class StreamToBytes : KotlinLambdaFunction<EventEnvelope, ByteArray> {
    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, instance: Int): ByteArray {
        val fastRPC = FastRPC(headers)
        val util = Utility.getInstance()
        val streamId = headers[STREAM]
        val timeout = headers[TIMEOUT]
        val out = ByteArrayOutputStream()

        var n = 0
        if (streamId != null && timeout != null) {
            val req = EventEnvelope().setTo(streamId).setHeader(TYPE, READ)
            while (true) {
                val event = fastRPC.awaitRequest(req, 100L.coerceAtLeast(util.str2long(timeout)))
                if (event.status == 408) {
                    throw AppException(408, "timeout for $timeout ms")
                }
                if (EOF == event.headers[TYPE]) {
                    EventEmitter.getInstance().send(streamId, Kv(TYPE, CLOSE))
                    break
                }
                if (DATA == event.headers[TYPE]) {
                    val block = event.body
                    if (block is ByteArray) {
                        n++
                        out.write(block)
                    }
                }
            }
        }
        return out.toByteArray()
    }

    companion object {
        private const val TYPE = "type"
        private const val READ = "read"
        private const val DATA = "data"
        private const val EOF = "eof"
        private const val CLOSE = "close"
        private const val STREAM = "stream"
        private const val TIMEOUT = "timeout"
    }
}