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
package org.platformlambda.core.system

import io.vertx.core.Handler
import io.vertx.core.eventbus.Message
import io.vertx.kotlin.coroutines.dispatcher
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import org.platformlambda.core.annotations.CoroutineRunner
import org.platformlambda.core.models.EventEnvelope
import org.slf4j.LoggerFactory
import java.io.IOException

class StreamQueue(def: ServiceDef, route: String?) : WorkerQueues(def, route) {
    private val coroutine: Boolean

    init {
        coroutine = def.streamFunction.javaClass.getAnnotation(CoroutineRunner::class.java) != null
        val system = Platform.getInstance().eventSystem
        consumer = system.localConsumer(route, StreamHandler())
        def.streamFunction.init(def.route)
        started()
    }

    private inner class StreamHandler : Handler<Message<ByteArray?>> {
        @OptIn(DelicateCoroutinesApi::class)
        override fun handle(message: Message<ByteArray?>) {
            if (!stopped) {
                try {
                    val event = EventEnvelope(message.body())
                    if (coroutine) {
                        // execute function as a coroutine
                        GlobalScope.launch(Platform.getInstance().vertx.dispatcher()) {
                            executeFunction(event)
                        }
                    } else {
                        // execute function as a runnable
                        executor.submit {
                            executeFunction(event)
                        }
                    }
                } catch (e: IOException) {
                    log.error("Unable to decode event - {}", e.message)
                }
            }
        }

        private fun executeFunction(event: EventEnvelope) {
            try {
                def.streamFunction.handleEvent(event.headers, event.body)
            } catch (e: Exception) {
                log.error("Unhandled exception for $route", e)
            }
        }
    }

    companion object {
        private val log = LoggerFactory.getLogger(StreamQueue::class.java)
    }
}